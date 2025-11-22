pub fn sub_8265D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D0E0 size=116
    let mut pc: u32 = 0x8265D0E0;
    'dispatch: loop {
        match pc {
            0x8265D0E0 => {
    //   block [0x8265D0E0..0x8265D154)
	// 8265D0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D0EC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265D0F0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8265D0F4: 390AF468  addi r8, r10, -0xb98
	ctx.r[8].s64 = ctx.r[10].s64 + -2968;
	// 8265D0F8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D0FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265D100: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D104: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D108: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265D10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D114: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 8265D118: 396BDC54  addi r11, r11, -0x23ac
	ctx.r[11].s64 = ctx.r[11].s64 + -9132;
	// 8265D11C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D120: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D124: 386A9808  addi r3, r10, -0x67f8
	ctx.r[3].s64 = ctx.r[10].s64 + -26616;
	// 8265D128: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265D12C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D130: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265D134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D13C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D140: 4BE09CE1  bl 0x82466e20
	ctx.lr = 0x8265D144;
	sub_82466E20(ctx, base);
	// 8265D144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265D158 size=48
    let mut pc: u32 = 0x8265D158;
    'dispatch: loop {
        match pc {
            0x8265D158 => {
    //   block [0x8265D158..0x8265D188)
	// 8265D158: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D15C: 814BF518  lwz r10, -0xae8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2792 as u32) ) } as u64;
	// 8265D160: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D164: 396B26E8  addi r11, r11, 0x26e8
	ctx.r[11].s64 = ctx.r[11].s64 + 9960;
	// 8265D168: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8265D16C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265D170: 814AF514  lwz r10, -0xaec(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2796 as u32) ) } as u64;
	// 8265D174: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 8265D178: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265D17C: 814AF510  lwz r10, -0xaf0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2800 as u32) ) } as u64;
	// 8265D180: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 8265D184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D188 size=116
    let mut pc: u32 = 0x8265D188;
    'dispatch: loop {
        match pc {
            0x8265D188 => {
    //   block [0x8265D188..0x8265D1FC)
	// 8265D188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D194: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265D198: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D19C: 392BDD28  addi r9, r11, -0x22d8
	ctx.r[9].s64 = ctx.r[11].s64 + -8920;
	// 8265D1A0: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D1A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D1A8: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8265D1AC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8265D1B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D1B4: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 8265D1B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D1BC: 396B26E8  addi r11, r11, 0x26e8
	ctx.r[11].s64 = ctx.r[11].s64 + 9960;
	// 8265D1C0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8265D1C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D1C8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8265D1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D1D0: 386A9838  addi r3, r10, -0x67c8
	ctx.r[3].s64 = ctx.r[10].s64 + -26568;
	// 8265D1D4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8265D1D8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8265D1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D1E0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8265D1E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265D1E8: 4BE09C39  bl 0x82466e20
	ctx.lr = 0x8265D1EC;
	sub_82466E20(ctx, base);
	// 8265D1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D200 size=116
    let mut pc: u32 = 0x8265D200;
    'dispatch: loop {
        match pc {
            0x8265D200 => {
    //   block [0x8265D200..0x8265D274)
	// 8265D200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D20C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D210: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265D214: 390BF528  addi r8, r11, -0xad8
	ctx.r[8].s64 = ctx.r[11].s64 + -2776;
	// 8265D218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D21C: 392ADEC8  addi r9, r10, -0x2138
	ctx.r[9].s64 = ctx.r[10].s64 + -8504;
	// 8265D220: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D224: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8265D228: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D22C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D234: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D244: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265D248: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8265D24C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265D250: 386B9868  addi r3, r11, -0x6798
	ctx.r[3].s64 = ctx.r[11].s64 + -26520;
	// 8265D254: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265D258: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D260: 4BE09BC1  bl 0x82466e20
	ctx.lr = 0x8265D264;
	sub_82466E20(ctx, base);
	// 8265D264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D278 size=112
    let mut pc: u32 = 0x8265D278;
    'dispatch: loop {
        match pc {
            0x8265D278 => {
    //   block [0x8265D278..0x8265D2E8)
	// 8265D278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D284: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D288: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D28C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D294: 390BF5B8  addi r8, r11, -0xa48
	ctx.r[8].s64 = ctx.r[11].s64 + -2632;
	// 8265D298: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265D29C: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 8265D2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D2A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D2A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D2B0: 386A9898  addi r3, r10, -0x6768
	ctx.r[3].s64 = ctx.r[10].s64 + -26472;
	// 8265D2B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D2B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D2C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D2C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D2D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D2D4: 4BE09B4D  bl 0x82466e20
	ctx.lr = 0x8265D2D8;
	sub_82466E20(ctx, base);
	// 8265D2D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D2E8 size=108
    let mut pc: u32 = 0x8265D2E8;
    'dispatch: loop {
        match pc {
            0x8265D2E8 => {
    //   block [0x8265D2E8..0x8265D354)
	// 8265D2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D2F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D2F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265D2FC: 38EBF5D0  addi r7, r11, -0xa30
	ctx.r[7].s64 = ctx.r[11].s64 + -2608;
	// 8265D300: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8265D304: 388A0E48  addi r4, r10, 0xe48
	ctx.r[4].s64 = ctx.r[10].s64 + 3656;
	// 8265D308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D30C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D310: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265D314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D318: 386A98C8  addi r3, r10, -0x6738
	ctx.r[3].s64 = ctx.r[10].s64 + -26424;
	// 8265D31C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265D320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D33C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D340: 4BE09AE1  bl 0x82466e20
	ctx.lr = 0x8265D344;
	sub_82466E20(ctx, base);
	// 8265D344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D358 size=112
    let mut pc: u32 = 0x8265D358;
    'dispatch: loop {
        match pc {
            0x8265D358 => {
    //   block [0x8265D358..0x8265D3C8)
	// 8265D358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D364: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D368: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D36C: 38AA7828  addi r5, r10, 0x7828
	ctx.r[5].s64 = ctx.r[10].s64 + 30760;
	// 8265D370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D374: 390BF648  addi r8, r11, -0x9b8
	ctx.r[8].s64 = ctx.r[11].s64 + -2488;
	// 8265D378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265D37C: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 8265D380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D384: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D390: 386A98F8  addi r3, r10, -0x6708
	ctx.r[3].s64 = ctx.r[10].s64 + -26376;
	// 8265D394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D3B4: 4BE09A6D  bl 0x82466e20
	ctx.lr = 0x8265D3B8;
	sub_82466E20(ctx, base);
	// 8265D3B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D3C8 size=108
    let mut pc: u32 = 0x8265D3C8;
    'dispatch: loop {
        match pc {
            0x8265D3C8 => {
    //   block [0x8265D3C8..0x8265D434)
	// 8265D3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D3D4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D3DC: 38EBF660  addi r7, r11, -0x9a0
	ctx.r[7].s64 = ctx.r[11].s64 + -2464;
	// 8265D3E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265D3E4: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 8265D3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D3EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D3F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265D3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D3F8: 386A9928  addi r3, r10, -0x66d8
	ctx.r[3].s64 = ctx.r[10].s64 + -26328;
	// 8265D3FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265D400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D41C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D420: 4BE09A01  bl 0x82466e20
	ctx.lr = 0x8265D424;
	sub_82466E20(ctx, base);
	// 8265D424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D438 size=112
    let mut pc: u32 = 0x8265D438;
    'dispatch: loop {
        match pc {
            0x8265D438 => {
    //   block [0x8265D438..0x8265D4A8)
	// 8265D438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D444: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D448: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D44C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D454: 390BF678  addi r8, r11, -0x988
	ctx.r[8].s64 = ctx.r[11].s64 + -2440;
	// 8265D458: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265D45C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 8265D460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D464: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D46C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D470: 386A9958  addi r3, r10, -0x66a8
	ctx.r[3].s64 = ctx.r[10].s64 + -26280;
	// 8265D474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D47C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D48C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D494: 4BE0998D  bl 0x82466e20
	ctx.lr = 0x8265D498;
	sub_82466E20(ctx, base);
	// 8265D498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D4A8 size=108
    let mut pc: u32 = 0x8265D4A8;
    'dispatch: loop {
        match pc {
            0x8265D4A8 => {
    //   block [0x8265D4A8..0x8265D514)
	// 8265D4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D4B4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D4B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D4BC: 38EBF6C0  addi r7, r11, -0x940
	ctx.r[7].s64 = ctx.r[11].s64 + -2368;
	// 8265D4C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265D4C4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 8265D4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D4CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D4D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265D4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D4D8: 386A9988  addi r3, r10, -0x6678
	ctx.r[3].s64 = ctx.r[10].s64 + -26232;
	// 8265D4DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265D4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D4FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D500: 4BE09921  bl 0x82466e20
	ctx.lr = 0x8265D504;
	sub_82466E20(ctx, base);
	// 8265D504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D518 size=108
    let mut pc: u32 = 0x8265D518;
    'dispatch: loop {
        match pc {
            0x8265D518 => {
    //   block [0x8265D518..0x8265D584)
	// 8265D518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D524: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D52C: 38EBF6F0  addi r7, r11, -0x910
	ctx.r[7].s64 = ctx.r[11].s64 + -2320;
	// 8265D530: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265D534: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 8265D538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D53C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D540: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265D544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D548: 386A99B8  addi r3, r10, -0x6648
	ctx.r[3].s64 = ctx.r[10].s64 + -26184;
	// 8265D54C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265D550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D55C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D56C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D570: 4BE098B1  bl 0x82466e20
	ctx.lr = 0x8265D574;
	sub_82466E20(ctx, base);
	// 8265D574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D588 size=112
    let mut pc: u32 = 0x8265D588;
    'dispatch: loop {
        match pc {
            0x8265D588 => {
    //   block [0x8265D588..0x8265D5F8)
	// 8265D588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D594: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D598: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D59C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D5A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D5A4: 390BF708  addi r8, r11, -0x8f8
	ctx.r[8].s64 = ctx.r[11].s64 + -2296;
	// 8265D5A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265D5AC: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 8265D5B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D5B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D5B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D5BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D5C0: 386A99E8  addi r3, r10, -0x6618
	ctx.r[3].s64 = ctx.r[10].s64 + -26136;
	// 8265D5C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D5DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D5E4: 4BE0983D  bl 0x82466e20
	ctx.lr = 0x8265D5E8;
	sub_82466E20(ctx, base);
	// 8265D5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D5F8 size=112
    let mut pc: u32 = 0x8265D5F8;
    'dispatch: loop {
        match pc {
            0x8265D5F8 => {
    //   block [0x8265D5F8..0x8265D668)
	// 8265D5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D604: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265D608: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D60C: 392ADF20  addi r9, r10, -0x20e0
	ctx.r[9].s64 = ctx.r[10].s64 + -8416;
	// 8265D610: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D614: 390BF740  addi r8, r11, -0x8c0
	ctx.r[8].s64 = ctx.r[11].s64 + -2240;
	// 8265D618: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8265D61C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 8265D620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D624: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D630: 386A9A18  addi r3, r10, -0x65e8
	ctx.r[3].s64 = ctx.r[10].s64 + -26088;
	// 8265D634: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265D638: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265D63C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D64C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D654: 4BE097CD  bl 0x82466e20
	ctx.lr = 0x8265D658;
	sub_82466E20(ctx, base);
	// 8265D658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D65C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D668 size=116
    let mut pc: u32 = 0x8265D668;
    'dispatch: loop {
        match pc {
            0x8265D668 => {
    //   block [0x8265D668..0x8265D6DC)
	// 8265D668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D674: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D678: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265D67C: 390BF7E8  addi r8, r11, -0x818
	ctx.r[8].s64 = ctx.r[11].s64 + -2072;
	// 8265D680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D684: 392ADEF4  addi r9, r10, -0x210c
	ctx.r[9].s64 = ctx.r[10].s64 + -8460;
	// 8265D688: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D68C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8265D690: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265D694: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D69C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D6AC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265D6B0: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 8265D6B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265D6B8: 386B9A48  addi r3, r11, -0x65b8
	ctx.r[3].s64 = ctx.r[11].s64 + -26040;
	// 8265D6BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265D6C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D6C8: 4BE09759  bl 0x82466e20
	ctx.lr = 0x8265D6CC;
	sub_82466E20(ctx, base);
	// 8265D6CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D6D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D6D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D6E0 size=112
    let mut pc: u32 = 0x8265D6E0;
    'dispatch: loop {
        match pc {
            0x8265D6E0 => {
    //   block [0x8265D6E0..0x8265D750)
	// 8265D6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D6EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265D6F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D6F4: 392ADF4C  addi r9, r10, -0x20b4
	ctx.r[9].s64 = ctx.r[10].s64 + -8372;
	// 8265D6F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D6FC: 390BF800  addi r8, r11, -0x800
	ctx.r[8].s64 = ctx.r[11].s64 + -2048;
	// 8265D700: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8265D704: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 8265D708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D70C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D718: 386A9A78  addi r3, r10, -0x6588
	ctx.r[3].s64 = ctx.r[10].s64 + -25992;
	// 8265D71C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265D720: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265D724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D73C: 4BE096E5  bl 0x82466e20
	ctx.lr = 0x8265D740;
	sub_82466E20(ctx, base);
	// 8265D740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D750 size=112
    let mut pc: u32 = 0x8265D750;
    'dispatch: loop {
        match pc {
            0x8265D750 => {
    //   block [0x8265D750..0x8265D7C0)
	// 8265D750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D75C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D760: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D764: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265D768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D76C: 390BF860  addi r8, r11, -0x7a0
	ctx.r[8].s64 = ctx.r[11].s64 + -1952;
	// 8265D770: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265D774: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 8265D778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D77C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D788: 386A9AA8  addi r3, r10, -0x6558
	ctx.r[3].s64 = ctx.r[10].s64 + -25944;
	// 8265D78C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D7AC: 4BE09675  bl 0x82466e20
	ctx.lr = 0x8265D7B0;
	sub_82466E20(ctx, base);
	// 8265D7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D7C0 size=112
    let mut pc: u32 = 0x8265D7C0;
    'dispatch: loop {
        match pc {
            0x8265D7C0 => {
    //   block [0x8265D7C0..0x8265D830)
	// 8265D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D7CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D7D0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D7D4: 38AA8AB8  addi r5, r10, -0x7548
	ctx.r[5].s64 = ctx.r[10].s64 + -30024;
	// 8265D7D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D7DC: 390BF878  addi r8, r11, -0x788
	ctx.r[8].s64 = ctx.r[11].s64 + -1928;
	// 8265D7E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265D7E4: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 8265D7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D7EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D7F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D7F8: 386A9AD8  addi r3, r10, -0x6528
	ctx.r[3].s64 = ctx.r[10].s64 + -25896;
	// 8265D7FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D81C: 4BE09605  bl 0x82466e20
	ctx.lr = 0x8265D820;
	sub_82466E20(ctx, base);
	// 8265D820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D830 size=112
    let mut pc: u32 = 0x8265D830;
    'dispatch: loop {
        match pc {
            0x8265D830 => {
    //   block [0x8265D830..0x8265D8A0)
	// 8265D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D83C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D840: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D844: 38AA8AB8  addi r5, r10, -0x7548
	ctx.r[5].s64 = ctx.r[10].s64 + -30024;
	// 8265D848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D84C: 390BF8C0  addi r8, r11, -0x740
	ctx.r[8].s64 = ctx.r[11].s64 + -1856;
	// 8265D850: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265D854: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 8265D858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D85C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D868: 386A9B08  addi r3, r10, -0x64f8
	ctx.r[3].s64 = ctx.r[10].s64 + -25848;
	// 8265D86C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D88C: 4BE09595  bl 0x82466e20
	ctx.lr = 0x8265D890;
	sub_82466E20(ctx, base);
	// 8265D890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D8A0 size=112
    let mut pc: u32 = 0x8265D8A0;
    'dispatch: loop {
        match pc {
            0x8265D8A0 => {
    //   block [0x8265D8A0..0x8265D910)
	// 8265D8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D8AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D8B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D8B4: 38AA8AE8  addi r5, r10, -0x7518
	ctx.r[5].s64 = ctx.r[10].s64 + -29976;
	// 8265D8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D8BC: 390BF920  addi r8, r11, -0x6e0
	ctx.r[8].s64 = ctx.r[11].s64 + -1760;
	// 8265D8C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265D8C4: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 8265D8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D8CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D8D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D8D8: 386A9B38  addi r3, r10, -0x64c8
	ctx.r[3].s64 = ctx.r[10].s64 + -25800;
	// 8265D8DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D8FC: 4BE09525  bl 0x82466e20
	ctx.lr = 0x8265D900;
	sub_82466E20(ctx, base);
	// 8265D900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D910 size=112
    let mut pc: u32 = 0x8265D910;
    'dispatch: loop {
        match pc {
            0x8265D910 => {
    //   block [0x8265D910..0x8265D980)
	// 8265D910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D91C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D920: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D924: 38AA8AE8  addi r5, r10, -0x7518
	ctx.r[5].s64 = ctx.r[10].s64 + -29976;
	// 8265D928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D92C: 390BF980  addi r8, r11, -0x680
	ctx.r[8].s64 = ctx.r[11].s64 + -1664;
	// 8265D930: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265D934: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 8265D938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D93C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D948: 386A9B68  addi r3, r10, -0x6498
	ctx.r[3].s64 = ctx.r[10].s64 + -25752;
	// 8265D94C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D95C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D96C: 4BE094B5  bl 0x82466e20
	ctx.lr = 0x8265D970;
	sub_82466E20(ctx, base);
	// 8265D970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D980 size=112
    let mut pc: u32 = 0x8265D980;
    'dispatch: loop {
        match pc {
            0x8265D980 => {
    //   block [0x8265D980..0x8265D9F0)
	// 8265D980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D98C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D990: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D994: 38AA8AB8  addi r5, r10, -0x7548
	ctx.r[5].s64 = ctx.r[10].s64 + -30024;
	// 8265D998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D99C: 390BF9E0  addi r8, r11, -0x620
	ctx.r[8].s64 = ctx.r[11].s64 + -1568;
	// 8265D9A0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8265D9A4: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 8265D9A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D9AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D9B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D9B8: 386A9B98  addi r3, r10, -0x6468
	ctx.r[3].s64 = ctx.r[10].s64 + -25704;
	// 8265D9BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D9C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D9C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D9CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D9D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D9D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D9DC: 4BE09445  bl 0x82466e20
	ctx.lr = 0x8265D9E0;
	sub_82466E20(ctx, base);
	// 8265D9E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D9F0 size=112
    let mut pc: u32 = 0x8265D9F0;
    'dispatch: loop {
        match pc {
            0x8265D9F0 => {
    //   block [0x8265D9F0..0x8265DA60)
	// 8265D9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D9FC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265DA00: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8265DA04: 38EAFAA0  addi r7, r10, -0x560
	ctx.r[7].s64 = ctx.r[10].s64 + -1376;
	// 8265DA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DA0C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265DA10: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8265DA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DA18: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DA1C: 396BDF60  addi r11, r11, -0x20a0
	ctx.r[11].s64 = ctx.r[11].s64 + -8352;
	// 8265DA20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DA24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DA28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DA2C: 386A9BC8  addi r3, r10, -0x6438
	ctx.r[3].s64 = ctx.r[10].s64 + -25656;
	// 8265DA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DA34: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265DA38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DA3C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265DA40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DA44: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DA48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DA4C: 4BE093D5  bl 0x82466e20
	ctx.lr = 0x8265DA50;
	sub_82466E20(ctx, base);
	// 8265DA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DA60 size=112
    let mut pc: u32 = 0x8265DA60;
    'dispatch: loop {
        match pc {
            0x8265DA60 => {
    //   block [0x8265DA60..0x8265DAD0)
	// 8265DA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DA6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DA70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DA74: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 8265DA78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DA7C: 390BFC68  addi r8, r11, -0x398
	ctx.r[8].s64 = ctx.r[11].s64 + -920;
	// 8265DA80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265DA84: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 8265DA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DA8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DA90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DA98: 386A9BF8  addi r3, r10, -0x6408
	ctx.r[3].s64 = ctx.r[10].s64 + -25608;
	// 8265DA9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DAA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DAA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DAA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DAAC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265DAB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DAB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DAB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DABC: 4BE09365  bl 0x82466e20
	ctx.lr = 0x8265DAC0;
	sub_82466E20(ctx, base);
	// 8265DAC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DAD0 size=112
    let mut pc: u32 = 0x8265DAD0;
    'dispatch: loop {
        match pc {
            0x8265DAD0 => {
    //   block [0x8265DAD0..0x8265DB40)
	// 8265DAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DAD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DADC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DAE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DAE4: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 8265DAE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DAEC: 390BFC80  addi r8, r11, -0x380
	ctx.r[8].s64 = ctx.r[11].s64 + -896;
	// 8265DAF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265DAF4: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8265DAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DAFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DB00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DB08: 386A9C28  addi r3, r10, -0x63d8
	ctx.r[3].s64 = ctx.r[10].s64 + -25560;
	// 8265DB0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DB10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DB18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DB1C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265DB20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DB24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DB28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DB2C: 4BE092F5  bl 0x82466e20
	ctx.lr = 0x8265DB30;
	sub_82466E20(ctx, base);
	// 8265DB30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DB40 size=112
    let mut pc: u32 = 0x8265DB40;
    'dispatch: loop {
        match pc {
            0x8265DB40 => {
    //   block [0x8265DB40..0x8265DBB0)
	// 8265DB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DB48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DB4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DB50: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DB54: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 8265DB58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DB5C: 390BFC98  addi r8, r11, -0x368
	ctx.r[8].s64 = ctx.r[11].s64 + -872;
	// 8265DB60: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265DB64: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8265DB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DB6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DB70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DB74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DB78: 386A9C58  addi r3, r10, -0x63a8
	ctx.r[3].s64 = ctx.r[10].s64 + -25512;
	// 8265DB7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DB80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DB84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DB8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DB9C: 4BE09285  bl 0x82466e20
	ctx.lr = 0x8265DBA0;
	sub_82466E20(ctx, base);
	// 8265DBA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DBB0 size=108
    let mut pc: u32 = 0x8265DBB0;
    'dispatch: loop {
        match pc {
            0x8265DBB0 => {
    //   block [0x8265DBB0..0x8265DC1C)
	// 8265DBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DBB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DBBC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DBC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DBC4: 38EBFCC8  addi r7, r11, -0x338
	ctx.r[7].s64 = ctx.r[11].s64 + -824;
	// 8265DBC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265DBCC: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8265DBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DBD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DBD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DBE0: 386A9C88  addi r3, r10, -0x6378
	ctx.r[3].s64 = ctx.r[10].s64 + -25464;
	// 8265DBE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DBE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DBF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DBF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DBFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DC00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DC04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DC08: 4BE09219  bl 0x82466e20
	ctx.lr = 0x8265DC0C;
	sub_82466E20(ctx, base);
	// 8265DC0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DC10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DC14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DC20 size=112
    let mut pc: u32 = 0x8265DC20;
    'dispatch: loop {
        match pc {
            0x8265DC20 => {
    //   block [0x8265DC20..0x8265DC90)
	// 8265DC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DC2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DC30: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DC34: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 8265DC38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DC3C: 390BFCF8  addi r8, r11, -0x308
	ctx.r[8].s64 = ctx.r[11].s64 + -776;
	// 8265DC40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265DC44: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8265DC48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DC4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DC50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DC58: 386A9CB8  addi r3, r10, -0x6348
	ctx.r[3].s64 = ctx.r[10].s64 + -25416;
	// 8265DC5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DC60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DC64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DC68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DC6C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265DC70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DC78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DC7C: 4BE091A5  bl 0x82466e20
	ctx.lr = 0x8265DC80;
	sub_82466E20(ctx, base);
	// 8265DC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DC90 size=108
    let mut pc: u32 = 0x8265DC90;
    'dispatch: loop {
        match pc {
            0x8265DC90 => {
    //   block [0x8265DC90..0x8265DCFC)
	// 8265DC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DC9C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DCA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DCA4: 38EBFD10  addi r7, r11, -0x2f0
	ctx.r[7].s64 = ctx.r[11].s64 + -752;
	// 8265DCA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265DCAC: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8265DCB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DCB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DCB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DCBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DCC0: 386A9CE8  addi r3, r10, -0x6318
	ctx.r[3].s64 = ctx.r[10].s64 + -25368;
	// 8265DCC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DCC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DCCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DCD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DCD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DCDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DCE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DCE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DCE8: 4BE09139  bl 0x82466e20
	ctx.lr = 0x8265DCEC;
	sub_82466E20(ctx, base);
	// 8265DCEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DCF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DCF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DCF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DD00 size=108
    let mut pc: u32 = 0x8265DD00;
    'dispatch: loop {
        match pc {
            0x8265DD00 => {
    //   block [0x8265DD00..0x8265DD6C)
	// 8265DD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DD0C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DD10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DD14: 38EBFD40  addi r7, r11, -0x2c0
	ctx.r[7].s64 = ctx.r[11].s64 + -704;
	// 8265DD18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265DD1C: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8265DD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DD24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DD28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DD30: 386A9D18  addi r3, r10, -0x62e8
	ctx.r[3].s64 = ctx.r[10].s64 + -25320;
	// 8265DD34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DD38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DD40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DD44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DD48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DD4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DD50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DD54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DD58: 4BE090C9  bl 0x82466e20
	ctx.lr = 0x8265DD5C;
	sub_82466E20(ctx, base);
	// 8265DD5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DD60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DD64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DD68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DD70 size=112
    let mut pc: u32 = 0x8265DD70;
    'dispatch: loop {
        match pc {
            0x8265DD70 => {
    //   block [0x8265DD70..0x8265DDE0)
	// 8265DD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DD7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DD80: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DD84: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265DD88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DD8C: 390BFD88  addi r8, r11, -0x278
	ctx.r[8].s64 = ctx.r[11].s64 + -632;
	// 8265DD90: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265DD94: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8265DD98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DD9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DDA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DDA8: 386A9D48  addi r3, r10, -0x62b8
	ctx.r[3].s64 = ctx.r[10].s64 + -25272;
	// 8265DDAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DDB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DDB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DDC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DDCC: 4BE09055  bl 0x82466e20
	ctx.lr = 0x8265DDD0;
	sub_82466E20(ctx, base);
	// 8265DDD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DDE0 size=112
    let mut pc: u32 = 0x8265DDE0;
    'dispatch: loop {
        match pc {
            0x8265DDE0 => {
    //   block [0x8265DDE0..0x8265DE50)
	// 8265DDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DDEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DDF0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DDF4: 38AA9808  addi r5, r10, -0x67f8
	ctx.r[5].s64 = ctx.r[10].s64 + -26616;
	// 8265DDF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265DDFC: 390BFDD0  addi r8, r11, -0x230
	ctx.r[8].s64 = ctx.r[11].s64 + -560;
	// 8265DE00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265DE04: 388A0E58  addi r4, r10, 0xe58
	ctx.r[4].s64 = ctx.r[10].s64 + 3672;
	// 8265DE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DE0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DE10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DE18: 386A9D78  addi r3, r10, -0x6288
	ctx.r[3].s64 = ctx.r[10].s64 + -25224;
	// 8265DE1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DE20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DE24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DE28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DE30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DE34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DE38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DE3C: 4BE08FE5  bl 0x82466e20
	ctx.lr = 0x8265DE40;
	sub_82466E20(ctx, base);
	// 8265DE40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DE50 size=108
    let mut pc: u32 = 0x8265DE50;
    'dispatch: loop {
        match pc {
            0x8265DE50 => {
    //   block [0x8265DE50..0x8265DEBC)
	// 8265DE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DE58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DE5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DE60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265DE64: 38EBFDE8  addi r7, r11, -0x218
	ctx.r[7].s64 = ctx.r[11].s64 + -536;
	// 8265DE68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265DE6C: 388A0E74  addi r4, r10, 0xe74
	ctx.r[4].s64 = ctx.r[10].s64 + 3700;
	// 8265DE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DE74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DE78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DE80: 386A9DA8  addi r3, r10, -0x6258
	ctx.r[3].s64 = ctx.r[10].s64 + -25176;
	// 8265DE84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DE94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DEA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DEA8: 4BE08F79  bl 0x82466e20
	ctx.lr = 0x8265DEAC;
	sub_82466E20(ctx, base);
	// 8265DEAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DEC0 size=108
    let mut pc: u32 = 0x8265DEC0;
    'dispatch: loop {
        match pc {
            0x8265DEC0 => {
    //   block [0x8265DEC0..0x8265DF2C)
	// 8265DEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DECC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DED0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265DED4: 38EBFE00  addi r7, r11, -0x200
	ctx.r[7].s64 = ctx.r[11].s64 + -512;
	// 8265DED8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265DEDC: 388A0E9C  addi r4, r10, 0xe9c
	ctx.r[4].s64 = ctx.r[10].s64 + 3740;
	// 8265DEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DEE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DEE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DEF0: 386A9DD8  addi r3, r10, -0x6228
	ctx.r[3].s64 = ctx.r[10].s64 + -25128;
	// 8265DEF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DEF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DF14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DF18: 4BE08F09  bl 0x82466e20
	ctx.lr = 0x8265DF1C;
	sub_82466E20(ctx, base);
	// 8265DF1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DF30 size=112
    let mut pc: u32 = 0x8265DF30;
    'dispatch: loop {
        match pc {
            0x8265DF30 => {
    //   block [0x8265DF30..0x8265DFA0)
	// 8265DF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DF3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DF40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DF44: 38AA9DD8  addi r5, r10, -0x6228
	ctx.r[5].s64 = ctx.r[10].s64 + -25128;
	// 8265DF48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265DF4C: 390BFE30  addi r8, r11, -0x1d0
	ctx.r[8].s64 = ctx.r[11].s64 + -464;
	// 8265DF50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265DF54: 388A0EB4  addi r4, r10, 0xeb4
	ctx.r[4].s64 = ctx.r[10].s64 + 3764;
	// 8265DF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DF5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DF60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DF68: 386A9E08  addi r3, r10, -0x61f8
	ctx.r[3].s64 = ctx.r[10].s64 + -25080;
	// 8265DF6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DF70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DF74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DF78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DF80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DF88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DF8C: 4BE08E95  bl 0x82466e20
	ctx.lr = 0x8265DF90;
	sub_82466E20(ctx, base);
	// 8265DF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265DFA0 size=24
    let mut pc: u32 = 0x8265DFA0;
    'dispatch: loop {
        match pc {
            0x8265DFA0 => {
    //   block [0x8265DFA0..0x8265DFB8)
	// 8265DFA0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DFA4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265DFA8: 394A2AA8  addi r10, r10, 0x2aa8
	ctx.r[10].s64 = ctx.r[10].s64 + 10920;
	// 8265DFAC: 816BFE60  lwz r11, -0x1a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-416 as u32) ) } as u64;
	// 8265DFB0: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8265DFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DFB8 size=116
    let mut pc: u32 = 0x8265DFB8;
    'dispatch: loop {
        match pc {
            0x8265DFB8 => {
    //   block [0x8265DFB8..0x8265E02C)
	// 8265DFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DFC4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DFC8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265DFCC: 390B2AA8  addi r8, r11, 0x2aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 10920;
	// 8265DFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DFD4: 392ADFF8  addi r9, r10, -0x2008
	ctx.r[9].s64 = ctx.r[10].s64 + -8200;
	// 8265DFD8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DFDC: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8265DFE0: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265DFE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DFE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DFEC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265DFF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DFF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DFFC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265E000: 388A0EDC  addi r4, r10, 0xedc
	ctx.r[4].s64 = ctx.r[10].s64 + 3804;
	// 8265E004: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265E008: 386B9E38  addi r3, r11, -0x61c8
	ctx.r[3].s64 = ctx.r[11].s64 + -25032;
	// 8265E00C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265E010: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E018: 4BE08E09  bl 0x82466e20
	ctx.lr = 0x8265E01C;
	sub_82466E20(ctx, base);
	// 8265E01C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E030 size=112
    let mut pc: u32 = 0x8265E030;
    'dispatch: loop {
        match pc {
            0x8265E030 => {
    //   block [0x8265E030..0x8265E0A0)
	// 8265E030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E03C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E040: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E044: 38AA8AE8  addi r5, r10, -0x7518
	ctx.r[5].s64 = ctx.r[10].s64 + -29976;
	// 8265E048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E04C: 390BFE68  addi r8, r11, -0x198
	ctx.r[8].s64 = ctx.r[11].s64 + -408;
	// 8265E050: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8265E054: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8265E058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E05C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E068: 386A9E68  addi r3, r10, -0x6198
	ctx.r[3].s64 = ctx.r[10].s64 + -24984;
	// 8265E06C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E08C: 4BE08D95  bl 0x82466e20
	ctx.lr = 0x8265E090;
	sub_82466E20(ctx, base);
	// 8265E090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E0A0 size=108
    let mut pc: u32 = 0x8265E0A0;
    'dispatch: loop {
        match pc {
            0x8265E0A0 => {
    //   block [0x8265E0A0..0x8265E10C)
	// 8265E0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E0AC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E0B4: 38EBFEF8  addi r7, r11, -0x108
	ctx.r[7].s64 = ctx.r[11].s64 + -264;
	// 8265E0B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265E0BC: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8265E0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E0C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E0C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E0D0: 386A9E98  addi r3, r10, -0x6168
	ctx.r[3].s64 = ctx.r[10].s64 + -24936;
	// 8265E0D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E0E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E0F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E0F8: 4BE08D29  bl 0x82466e20
	ctx.lr = 0x8265E0FC;
	sub_82466E20(ctx, base);
	// 8265E0FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E110 size=108
    let mut pc: u32 = 0x8265E110;
    'dispatch: loop {
        match pc {
            0x8265E110 => {
    //   block [0x8265E110..0x8265E17C)
	// 8265E110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E11C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E124: 38EBFF40  addi r7, r11, -0xc0
	ctx.r[7].s64 = ctx.r[11].s64 + -192;
	// 8265E128: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265E12C: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8265E130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E140: 386A9EC8  addi r3, r10, -0x6138
	ctx.r[3].s64 = ctx.r[10].s64 + -24888;
	// 8265E144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E168: 4BE08CB9  bl 0x82466e20
	ctx.lr = 0x8265E16C;
	sub_82466E20(ctx, base);
	// 8265E16C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E180 size=108
    let mut pc: u32 = 0x8265E180;
    'dispatch: loop {
        match pc {
            0x8265E180 => {
    //   block [0x8265E180..0x8265E1EC)
	// 8265E180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E18C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E194: 38EBFF70  addi r7, r11, -0x90
	ctx.r[7].s64 = ctx.r[11].s64 + -144;
	// 8265E198: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265E19C: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8265E1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E1A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E1B0: 386A9EF8  addi r3, r10, -0x6108
	ctx.r[3].s64 = ctx.r[10].s64 + -24840;
	// 8265E1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E1D8: 4BE08C49  bl 0x82466e20
	ctx.lr = 0x8265E1DC;
	sub_82466E20(ctx, base);
	// 8265E1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E1F0 size=112
    let mut pc: u32 = 0x8265E1F0;
    'dispatch: loop {
        match pc {
            0x8265E1F0 => {
    //   block [0x8265E1F0..0x8265E260)
	// 8265E1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E1FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E200: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E204: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E20C: 390BFFA0  addi r8, r11, -0x60
	ctx.r[8].s64 = ctx.r[11].s64 + -96;
	// 8265E210: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265E214: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8265E218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E21C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E228: 386A9F28  addi r3, r10, -0x60d8
	ctx.r[3].s64 = ctx.r[10].s64 + -24792;
	// 8265E22C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E24C: 4BE08BD5  bl 0x82466e20
	ctx.lr = 0x8265E250;
	sub_82466E20(ctx, base);
	// 8265E250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E260 size=112
    let mut pc: u32 = 0x8265E260;
    'dispatch: loop {
        match pc {
            0x8265E260 => {
    //   block [0x8265E260..0x8265E2D0)
	// 8265E260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E26C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E270: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E274: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E27C: 390BFFD0  addi r8, r11, -0x30
	ctx.r[8].s64 = ctx.r[11].s64 + -48;
	// 8265E280: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265E284: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8265E288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E28C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E298: 386A9F58  addi r3, r10, -0x60a8
	ctx.r[3].s64 = ctx.r[10].s64 + -24744;
	// 8265E29C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E2A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E2A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E2BC: 4BE08B65  bl 0x82466e20
	ctx.lr = 0x8265E2C0;
	sub_82466E20(ctx, base);
	// 8265E2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E2D0 size=112
    let mut pc: u32 = 0x8265E2D0;
    'dispatch: loop {
        match pc {
            0x8265E2D0 => {
    //   block [0x8265E2D0..0x8265E340)
	// 8265E2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E2DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E2E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E2E4: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E2EC: 390BFFE8  addi r8, r11, -0x18
	ctx.r[8].s64 = ctx.r[11].s64 + -24;
	// 8265E2F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265E2F4: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8265E2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E2FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E308: 386A9F88  addi r3, r10, -0x6078
	ctx.r[3].s64 = ctx.r[10].s64 + -24696;
	// 8265E30C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E32C: 4BE08AF5  bl 0x82466e20
	ctx.lr = 0x8265E330;
	sub_82466E20(ctx, base);
	// 8265E330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E340 size=108
    let mut pc: u32 = 0x8265E340;
    'dispatch: loop {
        match pc {
            0x8265E340 => {
    //   block [0x8265E340..0x8265E3AC)
	// 8265E340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E34C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E354: 38EB0000  addi r7, r11, 0
	ctx.r[7].s64 = ctx.r[11].s64 + 0;
	// 8265E358: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265E35C: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8265E360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E370: 386A9FB8  addi r3, r10, -0x6048
	ctx.r[3].s64 = ctx.r[10].s64 + -24648;
	// 8265E374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E398: 4BE08A89  bl 0x82466e20
	ctx.lr = 0x8265E39C;
	sub_82466E20(ctx, base);
	// 8265E39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E3B0 size=112
    let mut pc: u32 = 0x8265E3B0;
    'dispatch: loop {
        match pc {
            0x8265E3B0 => {
    //   block [0x8265E3B0..0x8265E420)
	// 8265E3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E3BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E3C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E3C4: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E3C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E3CC: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 8265E3D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265E3D4: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8265E3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E3DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E3E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E3E8: 386A9FE8  addi r3, r10, -0x6018
	ctx.r[3].s64 = ctx.r[10].s64 + -24600;
	// 8265E3EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E3F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E40C: 4BE08A15  bl 0x82466e20
	ctx.lr = 0x8265E410;
	sub_82466E20(ctx, base);
	// 8265E410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E420 size=108
    let mut pc: u32 = 0x8265E420;
    'dispatch: loop {
        match pc {
            0x8265E420 => {
    //   block [0x8265E420..0x8265E48C)
	// 8265E420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E42C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E434: 38EB0048  addi r7, r11, 0x48
	ctx.r[7].s64 = ctx.r[11].s64 + 72;
	// 8265E438: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8265E43C: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8265E440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E444: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E450: 386AA018  addi r3, r10, -0x5fe8
	ctx.r[3].s64 = ctx.r[10].s64 + -24552;
	// 8265E454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E478: 4BE089A9  bl 0x82466e20
	ctx.lr = 0x8265E47C;
	sub_82466E20(ctx, base);
	// 8265E47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E490 size=116
    let mut pc: u32 = 0x8265E490;
    'dispatch: loop {
        match pc {
            0x8265E490 => {
    //   block [0x8265E490..0x8265E504)
	// 8265E490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E49C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265E4A0: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 8265E4A4: 390A0138  addi r8, r10, 0x138
	ctx.r[8].s64 = ctx.r[10].s64 + 312;
	// 8265E4A8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E4AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265E4B0: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E4B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E4B8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265E4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E4C4: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8265E4C8: 396BE010  addi r11, r11, -0x1ff0
	ctx.r[11].s64 = ctx.r[11].s64 + -8176;
	// 8265E4CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E4D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E4D4: 386AA048  addi r3, r10, -0x5fb8
	ctx.r[3].s64 = ctx.r[10].s64 + -24504;
	// 8265E4D8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265E4DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E4E0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265E4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E4F0: 4BE08931  bl 0x82466e20
	ctx.lr = 0x8265E4F4;
	sub_82466E20(ctx, base);
	// 8265E4F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E4F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E4FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E508 size=108
    let mut pc: u32 = 0x8265E508;
    'dispatch: loop {
        match pc {
            0x8265E508 => {
    //   block [0x8265E508..0x8265E574)
	// 8265E508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E514: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E51C: 38EB0300  addi r7, r11, 0x300
	ctx.r[7].s64 = ctx.r[11].s64 + 768;
	// 8265E520: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8265E524: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8265E528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E52C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E538: 386AA078  addi r3, r10, -0x5f88
	ctx.r[3].s64 = ctx.r[10].s64 + -24456;
	// 8265E53C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E55C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E560: 4BE088C1  bl 0x82466e20
	ctx.lr = 0x8265E564;
	sub_82466E20(ctx, base);
	// 8265E564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E578 size=112
    let mut pc: u32 = 0x8265E578;
    'dispatch: loop {
        match pc {
            0x8265E578 => {
    //   block [0x8265E578..0x8265E5E8)
	// 8265E578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E584: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E588: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E58C: 38AA8AE8  addi r5, r10, -0x7518
	ctx.r[5].s64 = ctx.r[10].s64 + -29976;
	// 8265E590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E594: 390B0498  addi r8, r11, 0x498
	ctx.r[8].s64 = ctx.r[11].s64 + 1176;
	// 8265E598: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8265E59C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8265E5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E5A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E5AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E5B0: 386AA0A8  addi r3, r10, -0x5f58
	ctx.r[3].s64 = ctx.r[10].s64 + -24408;
	// 8265E5B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E5BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E5C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E5D4: 4BE0884D  bl 0x82466e20
	ctx.lr = 0x8265E5D8;
	sub_82466E20(ctx, base);
	// 8265E5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E5E8 size=100
    let mut pc: u32 = 0x8265E5E8;
    'dispatch: loop {
        match pc {
            0x8265E5E8 => {
    //   block [0x8265E5E8..0x8265E64C)
	// 8265E5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E5F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E5FC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E608: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8265E60C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E61C: 386AA0D8  addi r3, r10, -0x5f28
	ctx.r[3].s64 = ctx.r[10].s64 + -24360;
	// 8265E620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E624: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E628: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265E62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E630: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265E634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E638: 4BE087E9  bl 0x82466e20
	ctx.lr = 0x8265E63C;
	sub_82466E20(ctx, base);
	// 8265E63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E650 size=112
    let mut pc: u32 = 0x8265E650;
    'dispatch: loop {
        match pc {
            0x8265E650 => {
    //   block [0x8265E650..0x8265E6C0)
	// 8265E650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E65C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E660: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E664: 38AAA0D8  addi r5, r10, -0x5f28
	ctx.r[5].s64 = ctx.r[10].s64 + -24360;
	// 8265E668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E66C: 390B06F0  addi r8, r11, 0x6f0
	ctx.r[8].s64 = ctx.r[11].s64 + 1776;
	// 8265E670: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8265E674: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8265E678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E67C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E688: 386AA108  addi r3, r10, -0x5ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -24312;
	// 8265E68C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E6AC: 4BE08775  bl 0x82466e20
	ctx.lr = 0x8265E6B0;
	sub_82466E20(ctx, base);
	// 8265E6B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E6B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E6B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E6C0 size=100
    let mut pc: u32 = 0x8265E6C0;
    'dispatch: loop {
        match pc {
            0x8265E6C0 => {
    //   block [0x8265E6C0..0x8265E724)
	// 8265E6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E6CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E6D4: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E6E0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8265E6E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E6F4: 386AA138  addi r3, r10, -0x5ec8
	ctx.r[3].s64 = ctx.r[10].s64 + -24264;
	// 8265E6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E6FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E700: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265E704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E708: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265E70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E710: 4BE08711  bl 0x82466e20
	ctx.lr = 0x8265E714;
	sub_82466E20(ctx, base);
	// 8265E714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E728 size=108
    let mut pc: u32 = 0x8265E728;
    'dispatch: loop {
        match pc {
            0x8265E728 => {
    //   block [0x8265E728..0x8265E794)
	// 8265E728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E734: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E73C: 38EB0768  addi r7, r11, 0x768
	ctx.r[7].s64 = ctx.r[11].s64 + 1896;
	// 8265E740: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265E744: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8265E748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E74C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E758: 386AA168  addi r3, r10, -0x5e98
	ctx.r[3].s64 = ctx.r[10].s64 + -24216;
	// 8265E75C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E77C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E780: 4BE086A1  bl 0x82466e20
	ctx.lr = 0x8265E784;
	sub_82466E20(ctx, base);
	// 8265E784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E798 size=112
    let mut pc: u32 = 0x8265E798;
    'dispatch: loop {
        match pc {
            0x8265E798 => {
    //   block [0x8265E798..0x8265E808)
	// 8265E798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E7A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E7A8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E7AC: 38AAA138  addi r5, r10, -0x5ec8
	ctx.r[5].s64 = ctx.r[10].s64 + -24264;
	// 8265E7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E7B4: 390B07B0  addi r8, r11, 0x7b0
	ctx.r[8].s64 = ctx.r[11].s64 + 1968;
	// 8265E7B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265E7BC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8265E7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E7C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E7C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E7D0: 386AA198  addi r3, r10, -0x5e68
	ctx.r[3].s64 = ctx.r[10].s64 + -24168;
	// 8265E7D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E7D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E7F4: 4BE0862D  bl 0x82466e20
	ctx.lr = 0x8265E7F8;
	sub_82466E20(ctx, base);
	// 8265E7F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E808 size=100
    let mut pc: u32 = 0x8265E808;
    'dispatch: loop {
        match pc {
            0x8265E808 => {
    //   block [0x8265E808..0x8265E86C)
	// 8265E808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E81C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E828: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8265E82C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E83C: 386AA1C8  addi r3, r10, -0x5e38
	ctx.r[3].s64 = ctx.r[10].s64 + -24120;
	// 8265E840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E844: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E848: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265E84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E850: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265E854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E858: 4BE085C9  bl 0x82466e20
	ctx.lr = 0x8265E85C;
	sub_82466E20(ctx, base);
	// 8265E85C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E870 size=100
    let mut pc: u32 = 0x8265E870;
    'dispatch: loop {
        match pc {
            0x8265E870 => {
    //   block [0x8265E870..0x8265E8D4)
	// 8265E870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E87C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E884: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E890: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8265E894: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E8A4: 386AA1F8  addi r3, r10, -0x5e08
	ctx.r[3].s64 = ctx.r[10].s64 + -24072;
	// 8265E8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E8AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E8B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265E8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E8B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265E8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E8C0: 4BE08561  bl 0x82466e20
	ctx.lr = 0x8265E8C4;
	sub_82466E20(ctx, base);
	// 8265E8C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E8C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E8CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E8D8 size=112
    let mut pc: u32 = 0x8265E8D8;
    'dispatch: loop {
        match pc {
            0x8265E8D8 => {
    //   block [0x8265E8D8..0x8265E948)
	// 8265E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E8E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E8E8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E8EC: 38AAA1C8  addi r5, r10, -0x5e38
	ctx.r[5].s64 = ctx.r[10].s64 + -24120;
	// 8265E8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E8F4: 390B07E0  addi r8, r11, 0x7e0
	ctx.r[8].s64 = ctx.r[11].s64 + 2016;
	// 8265E8F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265E8FC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8265E900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E904: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E908: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E910: 386AA228  addi r3, r10, -0x5dd8
	ctx.r[3].s64 = ctx.r[10].s64 + -24024;
	// 8265E914: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E91C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E934: 4BE084ED  bl 0x82466e20
	ctx.lr = 0x8265E938;
	sub_82466E20(ctx, base);
	// 8265E938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E948 size=112
    let mut pc: u32 = 0x8265E948;
    'dispatch: loop {
        match pc {
            0x8265E948 => {
    //   block [0x8265E948..0x8265E9B8)
	// 8265E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E954: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E958: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E95C: 38AAA1F8  addi r5, r10, -0x5e08
	ctx.r[5].s64 = ctx.r[10].s64 + -24072;
	// 8265E960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E964: 390B0840  addi r8, r11, 0x840
	ctx.r[8].s64 = ctx.r[11].s64 + 2112;
	// 8265E968: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265E96C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8265E970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E974: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E978: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E97C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E980: 386AA258  addi r3, r10, -0x5da8
	ctx.r[3].s64 = ctx.r[10].s64 + -23976;
	// 8265E984: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E99C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E9A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E9A4: 4BE0847D  bl 0x82466e20
	ctx.lr = 0x8265E9A8;
	sub_82466E20(ctx, base);
	// 8265E9A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E9B8 size=100
    let mut pc: u32 = 0x8265E9B8;
    'dispatch: loop {
        match pc {
            0x8265E9B8 => {
    //   block [0x8265E9B8..0x8265EA1C)
	// 8265E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E9C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E9CC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E9D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E9D8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8265E9DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E9E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E9E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E9EC: 386AA288  addi r3, r10, -0x5d78
	ctx.r[3].s64 = ctx.r[10].s64 + -23928;
	// 8265E9F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E9F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E9F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265E9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EA00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265EA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EA08: 4BE08419  bl 0x82466e20
	ctx.lr = 0x8265EA0C;
	sub_82466E20(ctx, base);
	// 8265EA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EA20 size=112
    let mut pc: u32 = 0x8265EA20;
    'dispatch: loop {
        match pc {
            0x8265EA20 => {
    //   block [0x8265EA20..0x8265EA90)
	// 8265EA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EA2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EA30: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EA34: 38AAA288  addi r5, r10, -0x5d78
	ctx.r[5].s64 = ctx.r[10].s64 + -23928;
	// 8265EA38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EA3C: 390B08A0  addi r8, r11, 0x8a0
	ctx.r[8].s64 = ctx.r[11].s64 + 2208;
	// 8265EA40: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8265EA44: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8265EA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EA4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EA50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265EA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EA58: 386AA2B8  addi r3, r10, -0x5d48
	ctx.r[3].s64 = ctx.r[10].s64 + -23880;
	// 8265EA5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265EA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EA7C: 4BE083A5  bl 0x82466e20
	ctx.lr = 0x8265EA80;
	sub_82466E20(ctx, base);
	// 8265EA80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EA90 size=108
    let mut pc: u32 = 0x8265EA90;
    'dispatch: loop {
        match pc {
            0x8265EA90 => {
    //   block [0x8265EA90..0x8265EAFC)
	// 8265EA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EA9C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EAA4: 38EB0990  addi r7, r11, 0x990
	ctx.r[7].s64 = ctx.r[11].s64 + 2448;
	// 8265EAA8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8265EAAC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8265EAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EAB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EAB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EAC0: 386AA2E8  addi r3, r10, -0x5d18
	ctx.r[3].s64 = ctx.r[10].s64 + -23832;
	// 8265EAC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EAE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265EAE8: 4BE08339  bl 0x82466e20
	ctx.lr = 0x8265EAEC;
	sub_82466E20(ctx, base);
	// 8265EAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EB00 size=108
    let mut pc: u32 = 0x8265EB00;
    'dispatch: loop {
        match pc {
            0x8265EB00 => {
    //   block [0x8265EB00..0x8265EB6C)
	// 8265EB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EB0C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EB14: 38EB0A80  addi r7, r11, 0xa80
	ctx.r[7].s64 = ctx.r[11].s64 + 2688;
	// 8265EB18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265EB1C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8265EB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EB24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EB28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EB2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EB30: 386AA318  addi r3, r10, -0x5ce8
	ctx.r[3].s64 = ctx.r[10].s64 + -23784;
	// 8265EB34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EB54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265EB58: 4BE082C9  bl 0x82466e20
	ctx.lr = 0x8265EB5C;
	sub_82466E20(ctx, base);
	// 8265EB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EB70 size=108
    let mut pc: u32 = 0x8265EB70;
    'dispatch: loop {
        match pc {
            0x8265EB70 => {
    //   block [0x8265EB70..0x8265EBDC)
	// 8265EB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EB7C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EB84: 38EB0AC8  addi r7, r11, 0xac8
	ctx.r[7].s64 = ctx.r[11].s64 + 2760;
	// 8265EB88: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8265EB8C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8265EB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EB94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EB98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EBA0: 386AA348  addi r3, r10, -0x5cb8
	ctx.r[3].s64 = ctx.r[10].s64 + -23736;
	// 8265EBA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EBA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EBB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EBC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265EBC8: 4BE08259  bl 0x82466e20
	ctx.lr = 0x8265EBCC;
	sub_82466E20(ctx, base);
	// 8265EBCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EBD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EBD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EBE0 size=108
    let mut pc: u32 = 0x8265EBE0;
    'dispatch: loop {
        match pc {
            0x8265EBE0 => {
    //   block [0x8265EBE0..0x8265EC4C)
	// 8265EBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EBEC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EBF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EBF4: 38EB0BA0  addi r7, r11, 0xba0
	ctx.r[7].s64 = ctx.r[11].s64 + 2976;
	// 8265EBF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265EBFC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8265EC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EC04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EC08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EC10: 386AA378  addi r3, r10, -0x5c88
	ctx.r[3].s64 = ctx.r[10].s64 + -23688;
	// 8265EC14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265EC38: 4BE081E9  bl 0x82466e20
	ctx.lr = 0x8265EC3C;
	sub_82466E20(ctx, base);
	// 8265EC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EC50 size=100
    let mut pc: u32 = 0x8265EC50;
    'dispatch: loop {
        match pc {
            0x8265EC50 => {
    //   block [0x8265EC50..0x8265ECB4)
	// 8265EC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EC5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265EC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EC64: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265EC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EC70: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8265EC74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EC84: 386AA3A8  addi r3, r10, -0x5c58
	ctx.r[3].s64 = ctx.r[10].s64 + -23640;
	// 8265EC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EC8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EC90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265EC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EC98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265EC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ECA0: 4BE08181  bl 0x82466e20
	ctx.lr = 0x8265ECA4;
	sub_82466E20(ctx, base);
	// 8265ECA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265ECA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265ECAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265ECB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ECB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ECB8 size=112
    let mut pc: u32 = 0x8265ECB8;
    'dispatch: loop {
        match pc {
            0x8265ECB8 => {
    //   block [0x8265ECB8..0x8265ED28)
	// 8265ECB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ECBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265ECC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265ECC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ECC8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265ECCC: 38AAA3A8  addi r5, r10, -0x5c58
	ctx.r[5].s64 = ctx.r[10].s64 + -23640;
	// 8265ECD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265ECD4: 390B0BB8  addi r8, r11, 0xbb8
	ctx.r[8].s64 = ctx.r[11].s64 + 3000;
	// 8265ECD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265ECDC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8265ECE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265ECE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ECE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265ECEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265ECF0: 386AA3D8  addi r3, r10, -0x5c28
	ctx.r[3].s64 = ctx.r[10].s64 + -23592;
	// 8265ECF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265ECF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265ECFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265ED00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265ED04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265ED08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265ED0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ED10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265ED14: 4BE0810D  bl 0x82466e20
	ctx.lr = 0x8265ED18;
	sub_82466E20(ctx, base);
	// 8265ED18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265ED1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265ED20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265ED24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ED28 size=108
    let mut pc: u32 = 0x8265ED28;
    'dispatch: loop {
        match pc {
            0x8265ED28 => {
    //   block [0x8265ED28..0x8265ED94)
	// 8265ED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ED2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265ED30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265ED34: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265ED38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265ED3C: 38EB0C00  addi r7, r11, 0xc00
	ctx.r[7].s64 = ctx.r[11].s64 + 3072;
	// 8265ED40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265ED44: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8265ED48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265ED4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ED50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265ED54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265ED58: 386AA408  addi r3, r10, -0x5bf8
	ctx.r[3].s64 = ctx.r[10].s64 + -23544;
	// 8265ED5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265ED60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265ED64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265ED68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265ED6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265ED70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265ED74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ED78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265ED7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265ED80: 4BE080A1  bl 0x82466e20
	ctx.lr = 0x8265ED84;
	sub_82466E20(ctx, base);
	// 8265ED84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265ED88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265ED8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265ED90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ED98 size=112
    let mut pc: u32 = 0x8265ED98;
    'dispatch: loop {
        match pc {
            0x8265ED98 => {
    //   block [0x8265ED98..0x8265EE08)
	// 8265ED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EDA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265EDA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EDAC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265EDB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EDB4: 390B0C48  addi r8, r11, 0xc48
	ctx.r[8].s64 = ctx.r[11].s64 + 3144;
	// 8265EDB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265EDBC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8265EDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EDC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EDC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265EDCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EDD0: 386AA438  addi r3, r10, -0x5bc8
	ctx.r[3].s64 = ctx.r[10].s64 + -23496;
	// 8265EDD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265EDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EDDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EDF4: 4BE0802D  bl 0x82466e20
	ctx.lr = 0x8265EDF8;
	sub_82466E20(ctx, base);
	// 8265EDF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EE08 size=108
    let mut pc: u32 = 0x8265EE08;
    'dispatch: loop {
        match pc {
            0x8265EE08 => {
    //   block [0x8265EE08..0x8265EE74)
	// 8265EE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EE14: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EE18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EE1C: 38EB0C60  addi r7, r11, 0xc60
	ctx.r[7].s64 = ctx.r[11].s64 + 3168;
	// 8265EE20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265EE24: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8265EE28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EE2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EE30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EE38: 386AA468  addi r3, r10, -0x5b98
	ctx.r[3].s64 = ctx.r[10].s64 + -23448;
	// 8265EE3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EE40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EE54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EE5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265EE60: 4BE07FC1  bl 0x82466e20
	ctx.lr = 0x8265EE64;
	sub_82466E20(ctx, base);
	// 8265EE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EE78 size=112
    let mut pc: u32 = 0x8265EE78;
    'dispatch: loop {
        match pc {
            0x8265EE78 => {
    //   block [0x8265EE78..0x8265EEE8)
	// 8265EE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EE84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EE88: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EE8C: 38AAA438  addi r5, r10, -0x5bc8
	ctx.r[5].s64 = ctx.r[10].s64 + -23496;
	// 8265EE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EE94: 390B0CA8  addi r8, r11, 0xca8
	ctx.r[8].s64 = ctx.r[11].s64 + 3240;
	// 8265EE98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265EE9C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8265EEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EEA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EEA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265EEAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EEB0: 386AA498  addi r3, r10, -0x5b68
	ctx.r[3].s64 = ctx.r[10].s64 + -23400;
	// 8265EEB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265EEB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EED4: 4BE07F4D  bl 0x82466e20
	ctx.lr = 0x8265EED8;
	sub_82466E20(ctx, base);
	// 8265EED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EEE8 size=100
    let mut pc: u32 = 0x8265EEE8;
    'dispatch: loop {
        match pc {
            0x8265EEE8 => {
    //   block [0x8265EEE8..0x8265EF4C)
	// 8265EEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EEF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265EEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EEFC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265EF00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EF08: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8265EF0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EF1C: 386AA4C8  addi r3, r10, -0x5b38
	ctx.r[3].s64 = ctx.r[10].s64 + -23352;
	// 8265EF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EF24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EF28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265EF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EF30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265EF34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EF38: 4BE07EE9  bl 0x82466e20
	ctx.lr = 0x8265EF3C;
	sub_82466E20(ctx, base);
	// 8265EF3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EF50 size=112
    let mut pc: u32 = 0x8265EF50;
    'dispatch: loop {
        match pc {
            0x8265EF50 => {
    //   block [0x8265EF50..0x8265EFC0)
	// 8265EF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EF58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EF5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EF60: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EF64: 38AAA4C8  addi r5, r10, -0x5b38
	ctx.r[5].s64 = ctx.r[10].s64 + -23352;
	// 8265EF68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EF6C: 390B0CC0  addi r8, r11, 0xcc0
	ctx.r[8].s64 = ctx.r[11].s64 + 3264;
	// 8265EF70: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8265EF74: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 8265EF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EF7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EF80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265EF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EF88: 386AA4F8  addi r3, r10, -0x5b08
	ctx.r[3].s64 = ctx.r[10].s64 + -23304;
	// 8265EF8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265EF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EFAC: 4BE07E75  bl 0x82466e20
	ctx.lr = 0x8265EFB0;
	sub_82466E20(ctx, base);
	// 8265EFB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EFB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EFB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EFC0 size=108
    let mut pc: u32 = 0x8265EFC0;
    'dispatch: loop {
        match pc {
            0x8265EFC0 => {
    //   block [0x8265EFC0..0x8265F02C)
	// 8265EFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EFC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EFCC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EFD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EFD4: 38EB0D68  addi r7, r11, 0xd68
	ctx.r[7].s64 = ctx.r[11].s64 + 3432;
	// 8265EFD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265EFDC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 8265EFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EFE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EFE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EFEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EFF0: 386AA528  addi r3, r10, -0x5ad8
	ctx.r[3].s64 = ctx.r[10].s64 + -23256;
	// 8265EFF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EFFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F00C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F018: 4BE07E09  bl 0x82466e20
	ctx.lr = 0x8265F01C;
	sub_82466E20(ctx, base);
	// 8265F01C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F030 size=112
    let mut pc: u32 = 0x8265F030;
    'dispatch: loop {
        match pc {
            0x8265F030 => {
    //   block [0x8265F030..0x8265F0A0)
	// 8265F030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F03C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265F040: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F044: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265F048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F04C: 390B0D98  addi r8, r11, 0xd98
	ctx.r[8].s64 = ctx.r[11].s64 + 3480;
	// 8265F050: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265F054: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 8265F058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F05C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F068: 386AA558  addi r3, r10, -0x5aa8
	ctx.r[3].s64 = ctx.r[10].s64 + -23208;
	// 8265F06C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F08C: 4BE07D95  bl 0x82466e20
	ctx.lr = 0x8265F090;
	sub_82466E20(ctx, base);
	// 8265F090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F0A0 size=112
    let mut pc: u32 = 0x8265F0A0;
    'dispatch: loop {
        match pc {
            0x8265F0A0 => {
    //   block [0x8265F0A0..0x8265F110)
	// 8265F0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F0AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265F0B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F0B4: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265F0B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F0BC: 390B0DE0  addi r8, r11, 0xde0
	ctx.r[8].s64 = ctx.r[11].s64 + 3552;
	// 8265F0C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265F0C4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 8265F0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F0CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F0D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F0D8: 386AA588  addi r3, r10, -0x5a78
	ctx.r[3].s64 = ctx.r[10].s64 + -23160;
	// 8265F0DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F0FC: 4BE07D25  bl 0x82466e20
	ctx.lr = 0x8265F100;
	sub_82466E20(ctx, base);
	// 8265F100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F110 size=100
    let mut pc: u32 = 0x8265F110;
    'dispatch: loop {
        match pc {
            0x8265F110 => {
    //   block [0x8265F110..0x8265F174)
	// 8265F110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F11C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265F120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F124: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265F128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F130: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8265F134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F144: 386AA5B8  addi r3, r10, -0x5a48
	ctx.r[3].s64 = ctx.r[10].s64 + -23112;
	// 8265F148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F14C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F150: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265F154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F158: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265F15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F160: 4BE07CC1  bl 0x82466e20
	ctx.lr = 0x8265F164;
	sub_82466E20(ctx, base);
	// 8265F164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F178 size=112
    let mut pc: u32 = 0x8265F178;
    'dispatch: loop {
        match pc {
            0x8265F178 => {
    //   block [0x8265F178..0x8265F1E8)
	// 8265F178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F188: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F18C: 38AAA5B8  addi r5, r10, -0x5a48
	ctx.r[5].s64 = ctx.r[10].s64 + -23112;
	// 8265F190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F194: 390B0E28  addi r8, r11, 0xe28
	ctx.r[8].s64 = ctx.r[11].s64 + 3624;
	// 8265F198: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265F19C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 8265F1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F1A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F1A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F1B0: 386AA5E8  addi r3, r10, -0x5a18
	ctx.r[3].s64 = ctx.r[10].s64 + -23064;
	// 8265F1B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F1D4: 4BE07C4D  bl 0x82466e20
	ctx.lr = 0x8265F1D8;
	sub_82466E20(ctx, base);
	// 8265F1D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F1E8 size=112
    let mut pc: u32 = 0x8265F1E8;
    'dispatch: loop {
        match pc {
            0x8265F1E8 => {
    //   block [0x8265F1E8..0x8265F258)
	// 8265F1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F1F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265F1F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F1FC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265F200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F204: 390B0E70  addi r8, r11, 0xe70
	ctx.r[8].s64 = ctx.r[11].s64 + 3696;
	// 8265F208: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265F20C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 8265F210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F220: 386AA618  addi r3, r10, -0x59e8
	ctx.r[3].s64 = ctx.r[10].s64 + -23016;
	// 8265F224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F244: 4BE07BDD  bl 0x82466e20
	ctx.lr = 0x8265F248;
	sub_82466E20(ctx, base);
	// 8265F248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F258 size=112
    let mut pc: u32 = 0x8265F258;
    'dispatch: loop {
        match pc {
            0x8265F258 => {
    //   block [0x8265F258..0x8265F2C8)
	// 8265F258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F264: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265F268: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F26C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265F270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F274: 390B0E88  addi r8, r11, 0xe88
	ctx.r[8].s64 = ctx.r[11].s64 + 3720;
	// 8265F278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265F27C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 8265F280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F290: 386AA648  addi r3, r10, -0x59b8
	ctx.r[3].s64 = ctx.r[10].s64 + -22968;
	// 8265F294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F2A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265F2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F2B4: 4BE07B6D  bl 0x82466e20
	ctx.lr = 0x8265F2B8;
	sub_82466E20(ctx, base);
	// 8265F2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F2C8 size=112
    let mut pc: u32 = 0x8265F2C8;
    'dispatch: loop {
        match pc {
            0x8265F2C8 => {
    //   block [0x8265F2C8..0x8265F338)
	// 8265F2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F2D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F2D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F2DC: 38AAA618  addi r5, r10, -0x59e8
	ctx.r[5].s64 = ctx.r[10].s64 + -23016;
	// 8265F2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F2E4: 390B0EA0  addi r8, r11, 0xea0
	ctx.r[8].s64 = ctx.r[11].s64 + 3744;
	// 8265F2E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265F2EC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 8265F2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F2F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F2F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F300: 386AA678  addi r3, r10, -0x5988
	ctx.r[3].s64 = ctx.r[10].s64 + -22920;
	// 8265F304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F324: 4BE07AFD  bl 0x82466e20
	ctx.lr = 0x8265F328;
	sub_82466E20(ctx, base);
	// 8265F328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F338 size=72
    let mut pc: u32 = 0x8265F338;
    'dispatch: loop {
        match pc {
            0x8265F338 => {
    //   block [0x8265F338..0x8265F380)
	// 8265F338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F344: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265F348: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8265F34C: 38CBC808  addi r6, r11, -0x37f8
	ctx.r[6].s64 = ctx.r[11].s64 + -14328;
	// 8265F350: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265F354: 388BE060  addi r4, r11, -0x1fa0
	ctx.r[4].s64 = ctx.r[11].s64 + -8096;
	// 8265F358: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265F35C: 386BA6A8  addi r3, r11, -0x5958
	ctx.r[3].s64 = ctx.r[11].s64 + -22872;
	// 8265F360: 4BE1C729  bl 0x8247ba88
	ctx.lr = 0x8265F364;
	sub_8247BA88(ctx, base);
	// 8265F364: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8265F368: 386BCDE0  addi r3, r11, -0x3220
	ctx.r[3].s64 = ctx.r[11].s64 + -12832;
	// 8265F36C: 4BED37CD  bl 0x82532b38
	ctx.lr = 0x8265F370;
	sub_82532B38(ctx, base);
	// 8265F370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8265F374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F380 size=108
    let mut pc: u32 = 0x8265F380;
    'dispatch: loop {
        match pc {
            0x8265F380 => {
    //   block [0x8265F380..0x8265F3EC)
	// 8265F380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F38C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F394: 38EB2BE0  addi r7, r11, 0x2be0
	ctx.r[7].s64 = ctx.r[11].s64 + 11232;
	// 8265F398: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8265F39C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 8265F3A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F3A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F3A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F3B0: 386AA6C0  addi r3, r10, -0x5940
	ctx.r[3].s64 = ctx.r[10].s64 + -22848;
	// 8265F3B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F3B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F3C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F3CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F3D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F3D8: 4BE07A49  bl 0x82466e20
	ctx.lr = 0x8265F3DC;
	sub_82466E20(ctx, base);
	// 8265F3DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F3E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F3E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265F3F0 size=24
    let mut pc: u32 = 0x8265F3F0;
    'dispatch: loop {
        match pc {
            0x8265F3F0 => {
    //   block [0x8265F3F0..0x8265F408)
	// 8265F3F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F3F4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8265F3F8: 394AB1B0  addi r10, r10, -0x4e50
	ctx.r[10].s64 = ctx.r[10].s64 + -20048;
	// 8265F3FC: 816B2C58  lwz r11, 0x2c58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11352 as u32) ) } as u64;
	// 8265F400: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8265F404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F408 size=112
    let mut pc: u32 = 0x8265F408;
    'dispatch: loop {
        match pc {
            0x8265F408 => {
    //   block [0x8265F408..0x8265F478)
	// 8265F408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F414: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265F418: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8265F41C: 392AE774  addi r9, r10, -0x188c
	ctx.r[9].s64 = ctx.r[10].s64 + -6284;
	// 8265F420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F424: 390BB1B0  addi r8, r11, -0x4e50
	ctx.r[8].s64 = ctx.r[11].s64 + -20048;
	// 8265F428: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8265F42C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 8265F430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F434: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F440: 386AA6F0  addi r3, r10, -0x5910
	ctx.r[3].s64 = ctx.r[10].s64 + -22800;
	// 8265F444: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265F448: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265F44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F45C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F464: 4BE079BD  bl 0x82466e20
	ctx.lr = 0x8265F468;
	sub_82466E20(ctx, base);
	// 8265F468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F478 size=108
    let mut pc: u32 = 0x8265F478;
    'dispatch: loop {
        match pc {
            0x8265F478 => {
    //   block [0x8265F478..0x8265F4E4)
	// 8265F478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F484: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F48C: 38EB2C5C  addi r7, r11, 0x2c5c
	ctx.r[7].s64 = ctx.r[11].s64 + 11356;
	// 8265F490: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265F494: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 8265F498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F49C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F4A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F4A8: 386AA720  addi r3, r10, -0x58e0
	ctx.r[3].s64 = ctx.r[10].s64 + -22752;
	// 8265F4AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F4CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F4D0: 4BE07951  bl 0x82466e20
	ctx.lr = 0x8265F4D4;
	sub_82466E20(ctx, base);
	// 8265F4D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F4D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F4DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F4E8 size=108
    let mut pc: u32 = 0x8265F4E8;
    'dispatch: loop {
        match pc {
            0x8265F4E8 => {
    //   block [0x8265F4E8..0x8265F554)
	// 8265F4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F4F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F4FC: 38EB2C8C  addi r7, r11, 0x2c8c
	ctx.r[7].s64 = ctx.r[11].s64 + 11404;
	// 8265F500: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265F504: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 8265F508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F50C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F510: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F518: 386AA750  addi r3, r10, -0x58b0
	ctx.r[3].s64 = ctx.r[10].s64 + -22704;
	// 8265F51C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F53C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F540: 4BE078E1  bl 0x82466e20
	ctx.lr = 0x8265F544;
	sub_82466E20(ctx, base);
	// 8265F544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F54C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265F558 size=24
    let mut pc: u32 = 0x8265F558;
    'dispatch: loop {
        match pc {
            0x8265F558 => {
    //   block [0x8265F558..0x8265F570)
	// 8265F558: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F55C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8265F560: 394AB1F8  addi r10, r10, -0x4e08
	ctx.r[10].s64 = ctx.r[10].s64 + -19976;
	// 8265F564: 816B2CBC  lwz r11, 0x2cbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11452 as u32) ) } as u64;
	// 8265F568: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265F56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F570 size=116
    let mut pc: u32 = 0x8265F570;
    'dispatch: loop {
        match pc {
            0x8265F570 => {
    //   block [0x8265F570..0x8265F5E4)
	// 8265F570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F57C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8265F580: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265F584: 390BB1F8  addi r8, r11, -0x4e08
	ctx.r[8].s64 = ctx.r[11].s64 + -19976;
	// 8265F588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F58C: 392AE7A8  addi r9, r10, -0x1858
	ctx.r[9].s64 = ctx.r[10].s64 + -6232;
	// 8265F590: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F594: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8265F598: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265F59C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F5A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F5AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F5B4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265F5B8: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 8265F5BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265F5C0: 386BA780  addi r3, r11, -0x5880
	ctx.r[3].s64 = ctx.r[11].s64 + -22656;
	// 8265F5C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265F5C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F5D0: 4BE07851  bl 0x82466e20
	ctx.lr = 0x8265F5D4;
	sub_82466E20(ctx, base);
	// 8265F5D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F5E8 size=108
    let mut pc: u32 = 0x8265F5E8;
    'dispatch: loop {
        match pc {
            0x8265F5E8 => {
    //   block [0x8265F5E8..0x8265F654)
	// 8265F5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F5F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F5FC: 38EB2CC0  addi r7, r11, 0x2cc0
	ctx.r[7].s64 = ctx.r[11].s64 + 11456;
	// 8265F600: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265F604: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 8265F608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F60C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F618: 386AA7B0  addi r3, r10, -0x5850
	ctx.r[3].s64 = ctx.r[10].s64 + -22608;
	// 8265F61C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F63C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F640: 4BE077E1  bl 0x82466e20
	ctx.lr = 0x8265F644;
	sub_82466E20(ctx, base);
	// 8265F644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F658 size=112
    let mut pc: u32 = 0x8265F658;
    'dispatch: loop {
        match pc {
            0x8265F658 => {
    //   block [0x8265F658..0x8265F6C8)
	// 8265F658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F668: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F66C: 38AAA780  addi r5, r10, -0x5880
	ctx.r[5].s64 = ctx.r[10].s64 + -22656;
	// 8265F670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F674: 390B2D50  addi r8, r11, 0x2d50
	ctx.r[8].s64 = ctx.r[11].s64 + 11600;
	// 8265F678: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8265F67C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 8265F680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F684: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F68C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F690: 386AA7E0  addi r3, r10, -0x5820
	ctx.r[3].s64 = ctx.r[10].s64 + -22560;
	// 8265F694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F69C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F6B4: 4BE0776D  bl 0x82466e20
	ctx.lr = 0x8265F6B8;
	sub_82466E20(ctx, base);
	// 8265F6B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F6BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F6C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F6C8 size=112
    let mut pc: u32 = 0x8265F6C8;
    'dispatch: loop {
        match pc {
            0x8265F6C8 => {
    //   block [0x8265F6C8..0x8265F738)
	// 8265F6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F6D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F6D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F6DC: 38AAA780  addi r5, r10, -0x5880
	ctx.r[5].s64 = ctx.r[10].s64 + -22656;
	// 8265F6E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F6E4: 390B2E70  addi r8, r11, 0x2e70
	ctx.r[8].s64 = ctx.r[11].s64 + 11888;
	// 8265F6E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265F6EC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 8265F6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F6F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F6F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F700: 386AA810  addi r3, r10, -0x57f0
	ctx.r[3].s64 = ctx.r[10].s64 + -22512;
	// 8265F704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F70C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F71C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F724: 4BE076FD  bl 0x82466e20
	ctx.lr = 0x8265F728;
	sub_82466E20(ctx, base);
	// 8265F728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F738 size=108
    let mut pc: u32 = 0x8265F738;
    'dispatch: loop {
        match pc {
            0x8265F738 => {
    //   block [0x8265F738..0x8265F7A4)
	// 8265F738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F744: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F74C: 38EB2E88  addi r7, r11, 0x2e88
	ctx.r[7].s64 = ctx.r[11].s64 + 11912;
	// 8265F750: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265F754: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 8265F758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F75C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F768: 386AA840  addi r3, r10, -0x57c0
	ctx.r[3].s64 = ctx.r[10].s64 + -22464;
	// 8265F76C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F78C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F790: 4BE07691  bl 0x82466e20
	ctx.lr = 0x8265F794;
	sub_82466E20(ctx, base);
	// 8265F794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F7A8 size=112
    let mut pc: u32 = 0x8265F7A8;
    'dispatch: loop {
        match pc {
            0x8265F7A8 => {
    //   block [0x8265F7A8..0x8265F818)
	// 8265F7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F7B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F7B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F7BC: 38AAA780  addi r5, r10, -0x5880
	ctx.r[5].s64 = ctx.r[10].s64 + -22656;
	// 8265F7C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F7C4: 390B2F18  addi r8, r11, 0x2f18
	ctx.r[8].s64 = ctx.r[11].s64 + 12056;
	// 8265F7C8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8265F7CC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 8265F7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F7D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F7D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F7DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F7E0: 386AA870  addi r3, r10, -0x5790
	ctx.r[3].s64 = ctx.r[10].s64 + -22416;
	// 8265F7E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F7E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F7EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F7FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F804: 4BE0761D  bl 0x82466e20
	ctx.lr = 0x8265F808;
	sub_82466E20(ctx, base);
	// 8265F808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F818 size=108
    let mut pc: u32 = 0x8265F818;
    'dispatch: loop {
        match pc {
            0x8265F818 => {
    //   block [0x8265F818..0x8265F884)
	// 8265F818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F824: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F82C: 38EB3008  addi r7, r11, 0x3008
	ctx.r[7].s64 = ctx.r[11].s64 + 12296;
	// 8265F830: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265F834: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 8265F838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F83C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F848: 386AA8A0  addi r3, r10, -0x5760
	ctx.r[3].s64 = ctx.r[10].s64 + -22368;
	// 8265F84C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F86C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F870: 4BE075B1  bl 0x82466e20
	ctx.lr = 0x8265F874;
	sub_82466E20(ctx, base);
	// 8265F874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F888 size=108
    let mut pc: u32 = 0x8265F888;
    'dispatch: loop {
        match pc {
            0x8265F888 => {
    //   block [0x8265F888..0x8265F8F4)
	// 8265F888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F894: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F89C: 38EB3020  addi r7, r11, 0x3020
	ctx.r[7].s64 = ctx.r[11].s64 + 12320;
	// 8265F8A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265F8A4: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 8265F8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F8AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F8B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F8B8: 386AA8D0  addi r3, r10, -0x5730
	ctx.r[3].s64 = ctx.r[10].s64 + -22320;
	// 8265F8BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F8DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F8E0: 4BE07541  bl 0x82466e20
	ctx.lr = 0x8265F8E4;
	sub_82466E20(ctx, base);
	// 8265F8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F8F8 size=116
    let mut pc: u32 = 0x8265F8F8;
    'dispatch: loop {
        match pc {
            0x8265F8F8 => {
    //   block [0x8265F8F8..0x8265F96C)
	// 8265F8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F904: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F908: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265F90C: 390B3084  addi r8, r11, 0x3084
	ctx.r[8].s64 = ctx.r[11].s64 + 12420;
	// 8265F910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F914: 392AE7D4  addi r9, r10, -0x182c
	ctx.r[9].s64 = ctx.r[10].s64 + -6188;
	// 8265F918: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F91C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8265F920: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265F924: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F92C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F93C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265F940: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8265F944: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265F948: 386BA900  addi r3, r11, -0x5700
	ctx.r[3].s64 = ctx.r[11].s64 + -22272;
	// 8265F94C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265F950: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F958: 4BE074C9  bl 0x82466e20
	ctx.lr = 0x8265F95C;
	sub_82466E20(ctx, base);
	// 8265F95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F970 size=108
    let mut pc: u32 = 0x8265F970;
    'dispatch: loop {
        match pc {
            0x8265F970 => {
    //   block [0x8265F970..0x8265F9DC)
	// 8265F970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F97C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F984: 38EB30A0  addi r7, r11, 0x30a0
	ctx.r[7].s64 = ctx.r[11].s64 + 12448;
	// 8265F988: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265F98C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 8265F990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F994: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F9A0: 386AA930  addi r3, r10, -0x56d0
	ctx.r[3].s64 = ctx.r[10].s64 + -22224;
	// 8265F9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F9C8: 4BE07459  bl 0x82466e20
	ctx.lr = 0x8265F9CC;
	sub_82466E20(ctx, base);
	// 8265F9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F9E0 size=108
    let mut pc: u32 = 0x8265F9E0;
    'dispatch: loop {
        match pc {
            0x8265F9E0 => {
    //   block [0x8265F9E0..0x8265FA4C)
	// 8265F9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F9EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F9F4: 38EB30E8  addi r7, r11, 0x30e8
	ctx.r[7].s64 = ctx.r[11].s64 + 12520;
	// 8265F9F8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265F9FC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 8265FA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FA04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FA08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FA10: 386AA960  addi r3, r10, -0x56a0
	ctx.r[3].s64 = ctx.r[10].s64 + -22176;
	// 8265FA14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FA34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FA38: 4BE073E9  bl 0x82466e20
	ctx.lr = 0x8265FA3C;
	sub_82466E20(ctx, base);
	// 8265FA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FA50 size=108
    let mut pc: u32 = 0x8265FA50;
    'dispatch: loop {
        match pc {
            0x8265FA50 => {
    //   block [0x8265FA50..0x8265FABC)
	// 8265FA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FA5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FA64: 38EB3178  addi r7, r11, 0x3178
	ctx.r[7].s64 = ctx.r[11].s64 + 12664;
	// 8265FA68: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265FA6C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 8265FA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FA74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FA80: 386AA990  addi r3, r10, -0x5670
	ctx.r[3].s64 = ctx.r[10].s64 + -22128;
	// 8265FA84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FAA8: 4BE07379  bl 0x82466e20
	ctx.lr = 0x8265FAAC;
	sub_82466E20(ctx, base);
	// 8265FAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FAC0 size=100
    let mut pc: u32 = 0x8265FAC0;
    'dispatch: loop {
        match pc {
            0x8265FAC0 => {
    //   block [0x8265FAC0..0x8265FB24)
	// 8265FAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FACC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FAD4: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265FAD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FAE0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 8265FAE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FAF4: 386AA9C0  addi r3, r10, -0x5640
	ctx.r[3].s64 = ctx.r[10].s64 + -22080;
	// 8265FAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FAFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FB00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265FB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FB08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265FB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FB10: 4BE07311  bl 0x82466e20
	ctx.lr = 0x8265FB14;
	sub_82466E20(ctx, base);
	// 8265FB14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FB28 size=112
    let mut pc: u32 = 0x8265FB28;
    'dispatch: loop {
        match pc {
            0x8265FB28 => {
    //   block [0x8265FB28..0x8265FB98)
	// 8265FB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FB34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FB38: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FB3C: 38AAA9C0  addi r5, r10, -0x5640
	ctx.r[5].s64 = ctx.r[10].s64 + -22080;
	// 8265FB40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FB44: 390B3208  addi r8, r11, 0x3208
	ctx.r[8].s64 = ctx.r[11].s64 + 12808;
	// 8265FB48: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265FB4C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 8265FB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FB54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FB58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265FB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FB60: 386AA9F0  addi r3, r10, -0x5610
	ctx.r[3].s64 = ctx.r[10].s64 + -22032;
	// 8265FB64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265FB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FB84: 4BE0729D  bl 0x82466e20
	ctx.lr = 0x8265FB88;
	sub_82466E20(ctx, base);
	// 8265FB88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FB98 size=108
    let mut pc: u32 = 0x8265FB98;
    'dispatch: loop {
        match pc {
            0x8265FB98 => {
    //   block [0x8265FB98..0x8265FC04)
	// 8265FB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FBA4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FBA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FBAC: 38EB3268  addi r7, r11, 0x3268
	ctx.r[7].s64 = ctx.r[11].s64 + 12904;
	// 8265FBB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265FBB4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 8265FBB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FBBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FBC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FBC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FBC8: 386AAA20  addi r3, r10, -0x55e0
	ctx.r[3].s64 = ctx.r[10].s64 + -21984;
	// 8265FBCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FBD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FBD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FBDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FBE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FBE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FBE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FBEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FBF0: 4BE07231  bl 0x82466e20
	ctx.lr = 0x8265FBF4;
	sub_82466E20(ctx, base);
	// 8265FBF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FBF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FBFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FC08 size=108
    let mut pc: u32 = 0x8265FC08;
    'dispatch: loop {
        match pc {
            0x8265FC08 => {
    //   block [0x8265FC08..0x8265FC74)
	// 8265FC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FC14: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FC18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FC1C: 38EB3298  addi r7, r11, 0x3298
	ctx.r[7].s64 = ctx.r[11].s64 + 12952;
	// 8265FC20: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265FC24: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 8265FC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FC2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FC30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FC38: 386AAA50  addi r3, r10, -0x55b0
	ctx.r[3].s64 = ctx.r[10].s64 + -21936;
	// 8265FC3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FC40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FC4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FC54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FC5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FC60: 4BE071C1  bl 0x82466e20
	ctx.lr = 0x8265FC64;
	sub_82466E20(ctx, base);
	// 8265FC64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FC78 size=108
    let mut pc: u32 = 0x8265FC78;
    'dispatch: loop {
        match pc {
            0x8265FC78 => {
    //   block [0x8265FC78..0x8265FCE4)
	// 8265FC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FC84: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FC88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FC8C: 38EB32F8  addi r7, r11, 0x32f8
	ctx.r[7].s64 = ctx.r[11].s64 + 13048;
	// 8265FC90: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265FC94: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 8265FC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FC9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FCA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FCA8: 386AAA80  addi r3, r10, -0x5580
	ctx.r[3].s64 = ctx.r[10].s64 + -21888;
	// 8265FCAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FCB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FCBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FCCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FCD0: 4BE07151  bl 0x82466e20
	ctx.lr = 0x8265FCD4;
	sub_82466E20(ctx, base);
	// 8265FCD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FCD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FCDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265FCE8 size=24
    let mut pc: u32 = 0x8265FCE8;
    'dispatch: loop {
        match pc {
            0x8265FCE8 => {
    //   block [0x8265FCE8..0x8265FD00)
	// 8265FCE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FCEC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8265FCF0: 394AB270  addi r10, r10, -0x4d90
	ctx.r[10].s64 = ctx.r[10].s64 + -19856;
	// 8265FCF4: 816B309C  lwz r11, 0x309c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12444 as u32) ) } as u64;
	// 8265FCF8: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 8265FCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FD00 size=116
    let mut pc: u32 = 0x8265FD00;
    'dispatch: loop {
        match pc {
            0x8265FD00 => {
    //   block [0x8265FD00..0x8265FD74)
	// 8265FD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FD0C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8265FD10: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265FD14: 390BB270  addi r8, r11, -0x4d90
	ctx.r[8].s64 = ctx.r[11].s64 + -19856;
	// 8265FD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FD1C: 392AE808  addi r9, r10, -0x17f8
	ctx.r[9].s64 = ctx.r[10].s64 + -6136;
	// 8265FD20: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FD24: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8265FD28: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265FD2C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265FD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FD34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FD44: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265FD48: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 8265FD4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265FD50: 386BAAB0  addi r3, r11, -0x5550
	ctx.r[3].s64 = ctx.r[11].s64 + -21840;
	// 8265FD54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265FD58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FD60: 4BE070C1  bl 0x82466e20
	ctx.lr = 0x8265FD64;
	sub_82466E20(ctx, base);
	// 8265FD64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FD78 size=112
    let mut pc: u32 = 0x8265FD78;
    'dispatch: loop {
        match pc {
            0x8265FD78 => {
    //   block [0x8265FD78..0x8265FDE8)
	// 8265FD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FD84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FD88: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FD8C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265FD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FD94: 390B3358  addi r8, r11, 0x3358
	ctx.r[8].s64 = ctx.r[11].s64 + 13144;
	// 8265FD98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265FD9C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 8265FDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FDA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FDA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265FDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FDB0: 386AAAE0  addi r3, r10, -0x5520
	ctx.r[3].s64 = ctx.r[10].s64 + -21792;
	// 8265FDB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265FDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FDD4: 4BE0704D  bl 0x82466e20
	ctx.lr = 0x8265FDD8;
	sub_82466E20(ctx, base);
	// 8265FDD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FDE8 size=112
    let mut pc: u32 = 0x8265FDE8;
    'dispatch: loop {
        match pc {
            0x8265FDE8 => {
    //   block [0x8265FDE8..0x8265FE58)
	// 8265FDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FDF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FDF8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FDFC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265FE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FE04: 390B33A0  addi r8, r11, 0x33a0
	ctx.r[8].s64 = ctx.r[11].s64 + 13216;
	// 8265FE08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265FE0C: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 8265FE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FE14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FE18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265FE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FE20: 386AAB10  addi r3, r10, -0x54f0
	ctx.r[3].s64 = ctx.r[10].s64 + -21744;
	// 8265FE24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265FE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FE34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FE44: 4BE06FDD  bl 0x82466e20
	ctx.lr = 0x8265FE48;
	sub_82466E20(ctx, base);
	// 8265FE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265FE58 size=24
    let mut pc: u32 = 0x8265FE58;
    'dispatch: loop {
        match pc {
            0x8265FE58 => {
    //   block [0x8265FE58..0x8265FE70)
	// 8265FE58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FE5C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8265FE60: 394AB3A8  addi r10, r10, -0x4c58
	ctx.r[10].s64 = ctx.r[10].s64 + -19544;
	// 8265FE64: 816B33E8  lwz r11, 0x33e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13288 as u32) ) } as u64;
	// 8265FE68: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265FE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FE70 size=116
    let mut pc: u32 = 0x8265FE70;
    'dispatch: loop {
        match pc {
            0x8265FE70 => {
    //   block [0x8265FE70..0x8265FEE4)
	// 8265FE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FE7C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8265FE80: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265FE84: 390BB3A8  addi r8, r11, -0x4c58
	ctx.r[8].s64 = ctx.r[11].s64 + -19544;
	// 8265FE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FE8C: 392AE834  addi r9, r10, -0x17cc
	ctx.r[9].s64 = ctx.r[10].s64 + -6092;
	// 8265FE90: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FE94: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8265FE98: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 8265FE9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265FEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FEA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FEB4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265FEB8: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 8265FEBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265FEC0: 386BAB40  addi r3, r11, -0x54c0
	ctx.r[3].s64 = ctx.r[11].s64 + -21696;
	// 8265FEC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265FEC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FED0: 4BE06F51  bl 0x82466e20
	ctx.lr = 0x8265FED4;
	sub_82466E20(ctx, base);
	// 8265FED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FEDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FEE8 size=108
    let mut pc: u32 = 0x8265FEE8;
    'dispatch: loop {
        match pc {
            0x8265FEE8 => {
    //   block [0x8265FEE8..0x8265FF54)
	// 8265FEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FEF4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FEF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FEFC: 38EB33F0  addi r7, r11, 0x33f0
	ctx.r[7].s64 = ctx.r[11].s64 + 13296;
	// 8265FF00: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265FF04: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 8265FF08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FF0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FF10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FF18: 386AAB70  addi r3, r10, -0x5490
	ctx.r[3].s64 = ctx.r[10].s64 + -21648;
	// 8265FF1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FF20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FF28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FF30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FF34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FF38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FF3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FF40: 4BE06EE1  bl 0x82466e20
	ctx.lr = 0x8265FF44;
	sub_82466E20(ctx, base);
	// 8265FF44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FF48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FF4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FF58 size=108
    let mut pc: u32 = 0x8265FF58;
    'dispatch: loop {
        match pc {
            0x8265FF58 => {
    //   block [0x8265FF58..0x8265FFC4)
	// 8265FF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FF64: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FF68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FF6C: 38EB3450  addi r7, r11, 0x3450
	ctx.r[7].s64 = ctx.r[11].s64 + 13392;
	// 8265FF70: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8265FF74: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 8265FF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FF7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FF80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FF88: 386AABA0  addi r3, r10, -0x5460
	ctx.r[3].s64 = ctx.r[10].s64 + -21600;
	// 8265FF8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FFAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FFB0: 4BE06E71  bl 0x82466e20
	ctx.lr = 0x8265FFB4;
	sub_82466E20(ctx, base);
	// 8265FFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FFC8 size=112
    let mut pc: u32 = 0x8265FFC8;
    'dispatch: loop {
        match pc {
            0x8265FFC8 => {
    //   block [0x8265FFC8..0x82660038)
	// 8265FFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FFD4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265FFD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FFDC: 392BE868  addi r9, r11, -0x1798
	ctx.r[9].s64 = ctx.r[11].s64 + -6040;
	// 8265FFE0: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 8265FFE4: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 8265FFE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FFEC: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 8265FFF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FFF4: 396B34F8  addi r11, r11, 0x34f8
	ctx.r[11].s64 = ctx.r[11].s64 + 13560;
	// 8265FFF8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8265FFFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660000: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82660004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660008: 386AABD0  addi r3, r10, -0x5430
	ctx.r[3].s64 = ctx.r[10].s64 + -21552;
	// 8266000C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82660010: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82660014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660018: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8266001C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660020: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82660024: 4BE06DFD  bl 0x82466e20
	ctx.lr = 0x82660028;
	sub_82466E20(ctx, base);
	// 82660028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266002C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660038 size=112
    let mut pc: u32 = 0x82660038;
    'dispatch: loop {
        match pc {
            0x82660038 => {
    //   block [0x82660038..0x826600A8)
	// 82660038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266003C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660044: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660048: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266004C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82660050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660054: 390B3648  addi r8, r11, 0x3648
	ctx.r[8].s64 = ctx.r[11].s64 + 13896;
	// 82660058: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266005C: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 82660060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660064: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266006C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660070: 386AAC00  addi r3, r10, -0x5400
	ctx.r[3].s64 = ctx.r[10].s64 + -21504;
	// 82660074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266007C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266008C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660094: 4BE06D8D  bl 0x82466e20
	ctx.lr = 0x82660098;
	sub_82466E20(ctx, base);
	// 82660098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266009C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826600A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826600A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826600A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826600A8 size=112
    let mut pc: u32 = 0x826600A8;
    'dispatch: loop {
        match pc {
            0x826600A8 => {
    //   block [0x826600A8..0x82660118)
	// 826600A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826600AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826600B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826600B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826600B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826600BC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826600C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826600C4: 390B36F0  addi r8, r11, 0x36f0
	ctx.r[8].s64 = ctx.r[11].s64 + 14064;
	// 826600C8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826600CC: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826600D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826600D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826600D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826600DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826600E0: 386AAC30  addi r3, r10, -0x53d0
	ctx.r[3].s64 = ctx.r[10].s64 + -21456;
	// 826600E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826600E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826600EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826600F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826600F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826600F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826600FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660104: 4BE06D1D  bl 0x82466e20
	ctx.lr = 0x82660108;
	sub_82466E20(ctx, base);
	// 82660108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266010C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660118 size=112
    let mut pc: u32 = 0x82660118;
    'dispatch: loop {
        match pc {
            0x82660118 => {
    //   block [0x82660118..0x82660188)
	// 82660118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266011C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660128: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266012C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82660130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660134: 390B3780  addi r8, r11, 0x3780
	ctx.r[8].s64 = ctx.r[11].s64 + 14208;
	// 82660138: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8266013C: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 82660140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660144: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266014C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660150: 386AAC60  addi r3, r10, -0x53a0
	ctx.r[3].s64 = ctx.r[10].s64 + -21408;
	// 82660154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266015C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266016C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660174: 4BE06CAD  bl 0x82466e20
	ctx.lr = 0x82660178;
	sub_82466E20(ctx, base);
	// 82660178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266017C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660188 size=108
    let mut pc: u32 = 0x82660188;
    'dispatch: loop {
        match pc {
            0x82660188 => {
    //   block [0x82660188..0x826601F4)
	// 82660188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266018C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660194: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266019C: 38EB37F8  addi r7, r11, 0x37f8
	ctx.r[7].s64 = ctx.r[11].s64 + 14328;
	// 826601A0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826601A4: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826601A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826601AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826601B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826601B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826601B8: 386AAC90  addi r3, r10, -0x5370
	ctx.r[3].s64 = ctx.r[10].s64 + -21360;
	// 826601BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826601C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826601C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826601C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826601CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826601D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826601D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826601D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826601DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826601E0: 4BE06C41  bl 0x82466e20
	ctx.lr = 0x826601E4;
	sub_82466E20(ctx, base);
	// 826601E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826601E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826601EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826601F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826601F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826601F8 size=112
    let mut pc: u32 = 0x826601F8;
    'dispatch: loop {
        match pc {
            0x826601F8 => {
    //   block [0x826601F8..0x82660268)
	// 826601F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826601FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660204: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82660208: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266020C: 392AE8C8  addi r9, r10, -0x1738
	ctx.r[9].s64 = ctx.r[10].s64 + -5944;
	// 82660210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660214: 390B38A4  addi r8, r11, 0x38a4
	ctx.r[8].s64 = ctx.r[11].s64 + 14500;
	// 82660218: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8266021C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82660220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660224: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266022C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660230: 386AACC0  addi r3, r10, -0x5340
	ctx.r[3].s64 = ctx.r[10].s64 + -21312;
	// 82660234: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82660238: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266023C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266024C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660254: 4BE06BCD  bl 0x82466e20
	ctx.lr = 0x82660258;
	sub_82466E20(ctx, base);
	// 82660258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266025C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660268 size=100
    let mut pc: u32 = 0x82660268;
    'dispatch: loop {
        match pc {
            0x82660268 => {
    //   block [0x82660268..0x826602CC)
	// 82660268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266026C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660274: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266027C: 38AAB4A0  addi r5, r10, -0x4b60
	ctx.r[5].s64 = ctx.r[10].s64 + -19296;
	// 82660280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660288: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 8266028C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266029C: 386AACF0  addi r3, r10, -0x5310
	ctx.r[3].s64 = ctx.r[10].s64 + -21264;
	// 826602A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826602A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826602A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826602AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826602B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826602B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826602B8: 4BE06B69  bl 0x82466e20
	ctx.lr = 0x826602BC;
	sub_82466E20(ctx, base);
	// 826602BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826602C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826602C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826602C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826602D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826602D0 size=108
    let mut pc: u32 = 0x826602D0;
    'dispatch: loop {
        match pc {
            0x826602D0 => {
    //   block [0x826602D0..0x8266033C)
	// 826602D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826602D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826602D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826602DC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826602E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826602E4: 38EB38D8  addi r7, r11, 0x38d8
	ctx.r[7].s64 = ctx.r[11].s64 + 14552;
	// 826602E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826602EC: 388A2E80  addi r4, r10, 0x2e80
	ctx.r[4].s64 = ctx.r[10].s64 + 11904;
	// 826602F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826602F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826602F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826602FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660300: 386AAD20  addi r3, r10, -0x52e0
	ctx.r[3].s64 = ctx.r[10].s64 + -21216;
	// 82660304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266030C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266031C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660328: 4BE06AF9  bl 0x82466e20
	ctx.lr = 0x8266032C;
	sub_82466E20(ctx, base);
	// 8266032C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660340 size=112
    let mut pc: u32 = 0x82660340;
    'dispatch: loop {
        match pc {
            0x82660340 => {
    //   block [0x82660340..0x826603B0)
	// 82660340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266034C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82660350: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660354: 392AE928  addi r9, r10, -0x16d8
	ctx.r[9].s64 = ctx.r[10].s64 + -5848;
	// 82660358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266035C: 390B3908  addi r8, r11, 0x3908
	ctx.r[8].s64 = ctx.r[11].s64 + 14600;
	// 82660360: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82660364: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 82660368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266036C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660378: 386AAD50  addi r3, r10, -0x52b0
	ctx.r[3].s64 = ctx.r[10].s64 + -21168;
	// 8266037C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82660380: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82660384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266038C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266039C: 4BE06A85  bl 0x82466e20
	ctx.lr = 0x826603A0;
	sub_82466E20(ctx, base);
	// 826603A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826603A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826603A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826603AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826603B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826603B0 size=112
    let mut pc: u32 = 0x826603B0;
    'dispatch: loop {
        match pc {
            0x826603B0 => {
    //   block [0x826603B0..0x82660420)
	// 826603B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826603B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826603B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826603BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826603C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826603C4: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 826603C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826603CC: 390B3980  addi r8, r11, 0x3980
	ctx.r[8].s64 = ctx.r[11].s64 + 14720;
	// 826603D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826603D4: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826603D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826603DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826603E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826603E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826603E8: 386AAD80  addi r3, r10, -0x5280
	ctx.r[3].s64 = ctx.r[10].s64 + -21120;
	// 826603EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826603F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826603F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826603F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826603FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266040C: 4BE06A15  bl 0x82466e20
	ctx.lr = 0x82660410;
	sub_82466E20(ctx, base);
	// 82660410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266041C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660420 size=116
    let mut pc: u32 = 0x82660420;
    'dispatch: loop {
        match pc {
            0x82660420 => {
    //   block [0x82660420..0x82660494)
	// 82660420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266042C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82660430: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82660434: 390A39B0  addi r8, r10, 0x39b0
	ctx.r[8].s64 = ctx.r[10].s64 + 14768;
	// 82660438: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266043C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82660440: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 82660444: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660448: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266044C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660454: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82660458: 396BE93C  addi r11, r11, -0x16c4
	ctx.r[11].s64 = ctx.r[11].s64 + -5828;
	// 8266045C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660460: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660464: 386AADB0  addi r3, r10, -0x5250
	ctx.r[3].s64 = ctx.r[10].s64 + -21072;
	// 82660468: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8266046C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660470: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82660474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266047C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660480: 4BE069A1  bl 0x82466e20
	ctx.lr = 0x82660484;
	sub_82466E20(ctx, base);
	// 82660484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266048C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660498 size=100
    let mut pc: u32 = 0x82660498;
    'dispatch: loop {
        match pc {
            0x82660498 => {
    //   block [0x82660498..0x826604FC)
	// 82660498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266049C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826604A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826604A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826604A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826604AC: 38AAADB0  addi r5, r10, -0x5250
	ctx.r[5].s64 = ctx.r[10].s64 + -21072;
	// 826604B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826604B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826604B8: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826604BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826604C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826604C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826604C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826604CC: 386AADE0  addi r3, r10, -0x5220
	ctx.r[3].s64 = ctx.r[10].s64 + -21024;
	// 826604D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826604D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826604D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826604DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826604E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826604E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826604E8: 4BE06939  bl 0x82466e20
	ctx.lr = 0x826604EC;
	sub_82466E20(ctx, base);
	// 826604EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826604F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826604F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826604F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82660500 size=24
    let mut pc: u32 = 0x82660500;
    'dispatch: loop {
        match pc {
            0x82660500 => {
    //   block [0x82660500..0x82660518)
	// 82660500: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660504: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82660508: 394AB4B0  addi r10, r10, -0x4b50
	ctx.r[10].s64 = ctx.r[10].s64 + -19280;
	// 8266050C: 816B3A58  lwz r11, 0x3a58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14936 as u32) ) } as u64;
	// 82660510: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82660514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660518 size=116
    let mut pc: u32 = 0x82660518;
    'dispatch: loop {
        match pc {
            0x82660518 => {
    //   block [0x82660518..0x8266058C)
	// 82660518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266051C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660524: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82660528: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266052C: 392BE978  addi r9, r11, -0x1688
	ctx.r[9].s64 = ctx.r[11].s64 + -5768;
	// 82660530: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82660534: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660538: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8266053C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82660540: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82660544: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82660548: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266054C: 396BB4B0  addi r11, r11, -0x4b50
	ctx.r[11].s64 = ctx.r[11].s64 + -19280;
	// 82660550: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82660554: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660558: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8266055C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660560: 386AAE10  addi r3, r10, -0x51f0
	ctx.r[3].s64 = ctx.r[10].s64 + -20976;
	// 82660564: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82660568: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8266056C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660570: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82660574: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82660578: 4BE068A9  bl 0x82466e20
	ctx.lr = 0x8266057C;
	sub_82466E20(ctx, base);
	// 8266057C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660590 size=116
    let mut pc: u32 = 0x82660590;
    'dispatch: loop {
        match pc {
            0x82660590 => {
    //   block [0x82660590..0x82660604)
	// 82660590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266059C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826605A0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826605A4: 392BE9CC  addi r9, r11, -0x1634
	ctx.r[9].s64 = ctx.r[11].s64 + -5684;
	// 826605A8: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 826605AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826605B0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826605B4: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 826605B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826605BC: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826605C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826605C4: 396B3A60  addi r11, r11, 0x3a60
	ctx.r[11].s64 = ctx.r[11].s64 + 14944;
	// 826605C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826605CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826605D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826605D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826605D8: 386AAE40  addi r3, r10, -0x51c0
	ctx.r[3].s64 = ctx.r[10].s64 + -20928;
	// 826605DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826605E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826605E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826605E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826605EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826605F0: 4BE06831  bl 0x82466e20
	ctx.lr = 0x826605F4;
	sub_82466E20(ctx, base);
	// 826605F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826605F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826605FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660608 size=108
    let mut pc: u32 = 0x82660608;
    'dispatch: loop {
        match pc {
            0x82660608 => {
    //   block [0x82660608..0x82660674)
	// 82660608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266060C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660614: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266061C: 38EB3B38  addi r7, r11, 0x3b38
	ctx.r[7].s64 = ctx.r[11].s64 + 15160;
	// 82660620: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82660624: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82660628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266062C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660638: 386AAE70  addi r3, r10, -0x5190
	ctx.r[3].s64 = ctx.r[10].s64 + -20880;
	// 8266063C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266064C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266065C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660660: 4BE067C1  bl 0x82466e20
	ctx.lr = 0x82660664;
	sub_82466E20(ctx, base);
	// 82660664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266066C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82660678 size=24
    let mut pc: u32 = 0x82660678;
    'dispatch: loop {
        match pc {
            0x82660678 => {
    //   block [0x82660678..0x82660690)
	// 82660678: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266067C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82660680: 394AB558  addi r10, r10, -0x4aa8
	ctx.r[10].s64 = ctx.r[10].s64 + -19112;
	// 82660684: 816B3B98  lwz r11, 0x3b98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15256 as u32) ) } as u64;
	// 82660688: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8266068C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660690 size=116
    let mut pc: u32 = 0x82660690;
    'dispatch: loop {
        match pc {
            0x82660690 => {
    //   block [0x82660690..0x82660704)
	// 82660690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266069C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826606A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826606A4: 390BB558  addi r8, r11, -0x4aa8
	ctx.r[8].s64 = ctx.r[11].s64 + -19112;
	// 826606A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826606AC: 392AEA40  addi r9, r10, -0x15c0
	ctx.r[9].s64 = ctx.r[10].s64 + -5568;
	// 826606B0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826606B4: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826606B8: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 826606BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826606C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826606C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826606C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826606CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826606D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826606D4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826606D8: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826606DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826606E0: 386BAEA0  addi r3, r11, -0x5160
	ctx.r[3].s64 = ctx.r[11].s64 + -20832;
	// 826606E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826606E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826606EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826606F0: 4BE06731  bl 0x82466e20
	ctx.lr = 0x826606F4;
	sub_82466E20(ctx, base);
	// 826606F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826606F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826606FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660708 size=112
    let mut pc: u32 = 0x82660708;
    'dispatch: loop {
        match pc {
            0x82660708 => {
    //   block [0x82660708..0x82660778)
	// 82660708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266070C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660714: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660718: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266071C: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 82660720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660724: 390B3BA0  addi r8, r11, 0x3ba0
	ctx.r[8].s64 = ctx.r[11].s64 + 15264;
	// 82660728: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266072C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82660730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266073C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660740: 386AAED0  addi r3, r10, -0x5130
	ctx.r[3].s64 = ctx.r[10].s64 + -20784;
	// 82660744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266074C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266075C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660764: 4BE066BD  bl 0x82466e20
	ctx.lr = 0x82660768;
	sub_82466E20(ctx, base);
	// 82660768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266076C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82660778 size=24
    let mut pc: u32 = 0x82660778;
    'dispatch: loop {
        match pc {
            0x82660778 => {
    //   block [0x82660778..0x82660790)
	// 82660778: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266077C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82660780: 394AB6F0  addi r10, r10, -0x4910
	ctx.r[10].s64 = ctx.r[10].s64 + -18704;
	// 82660784: 816B3BD0  lwz r11, 0x3bd0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15312 as u32) ) } as u64;
	// 82660788: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8266078C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660790 size=116
    let mut pc: u32 = 0x82660790;
    'dispatch: loop {
        match pc {
            0x82660790 => {
    //   block [0x82660790..0x82660804)
	// 82660790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266079C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826607A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826607A4: 390BB6F0  addi r8, r11, -0x4910
	ctx.r[8].s64 = ctx.r[11].s64 + -18704;
	// 826607A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826607AC: 392AEA78  addi r9, r10, -0x1588
	ctx.r[9].s64 = ctx.r[10].s64 + -5512;
	// 826607B0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826607B4: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826607B8: 38AAAE40  addi r5, r10, -0x51c0
	ctx.r[5].s64 = ctx.r[10].s64 + -20928;
	// 826607BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826607C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826607C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826607C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826607CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826607D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826607D4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826607D8: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826607DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826607E0: 386BAF00  addi r3, r11, -0x5100
	ctx.r[3].s64 = ctx.r[11].s64 + -20736;
	// 826607E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826607E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826607EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826607F0: 4BE06631  bl 0x82466e20
	ctx.lr = 0x826607F4;
	sub_82466E20(ctx, base);
	// 826607F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826607F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826607FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660808 size=112
    let mut pc: u32 = 0x82660808;
    'dispatch: loop {
        match pc {
            0x82660808 => {
    //   block [0x82660808..0x82660878)
	// 82660808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660818: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266081C: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 82660820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660824: 390B3BD4  addi r8, r11, 0x3bd4
	ctx.r[8].s64 = ctx.r[11].s64 + 15316;
	// 82660828: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266082C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 82660830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660834: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266083C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660840: 386AAF30  addi r3, r10, -0x50d0
	ctx.r[3].s64 = ctx.r[10].s64 + -20688;
	// 82660844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266084C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266085C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660864: 4BE065BD  bl 0x82466e20
	ctx.lr = 0x82660868;
	sub_82466E20(ctx, base);
	// 82660868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266086C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660878 size=100
    let mut pc: u32 = 0x82660878;
    'dispatch: loop {
        match pc {
            0x82660878 => {
    //   block [0x82660878..0x826608DC)
	// 82660878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266087C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660884: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266088C: 38AAB4A0  addi r5, r10, -0x4b60
	ctx.r[5].s64 = ctx.r[10].s64 + -19296;
	// 82660890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660898: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8266089C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826608A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826608A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826608A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826608AC: 386AAF60  addi r3, r10, -0x50a0
	ctx.r[3].s64 = ctx.r[10].s64 + -20640;
	// 826608B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826608B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826608B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826608BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826608C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826608C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826608C8: 4BE06559  bl 0x82466e20
	ctx.lr = 0x826608CC;
	sub_82466E20(ctx, base);
	// 826608CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826608D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826608D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826608D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826608E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826608E0 size=108
    let mut pc: u32 = 0x826608E0;
    'dispatch: loop {
        match pc {
            0x826608E0 => {
    //   block [0x826608E0..0x8266094C)
	// 826608E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826608E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826608E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826608EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826608F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826608F4: 38EB3BF0  addi r7, r11, 0x3bf0
	ctx.r[7].s64 = ctx.r[11].s64 + 15344;
	// 826608F8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826608FC: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 82660900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660904: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660908: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266090C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660910: 386AAF90  addi r3, r10, -0x5070
	ctx.r[3].s64 = ctx.r[10].s64 + -20592;
	// 82660914: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266091C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266092C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660934: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660938: 4BE064E9  bl 0x82466e20
	ctx.lr = 0x8266093C;
	sub_82466E20(ctx, base);
	// 8266093C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660950 size=112
    let mut pc: u32 = 0x82660950;
    'dispatch: loop {
        match pc {
            0x82660950 => {
    //   block [0x82660950..0x826609C0)
	// 82660950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266095C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660960: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660964: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266096C: 390B3CC8  addi r8, r11, 0x3cc8
	ctx.r[8].s64 = ctx.r[11].s64 + 15560;
	// 82660970: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82660974: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 82660978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266097C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660988: 386AAFC0  addi r3, r10, -0x5040
	ctx.r[3].s64 = ctx.r[10].s64 + -20544;
	// 8266098C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266099C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826609A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826609A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826609A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826609AC: 4BE06475  bl 0x82466e20
	ctx.lr = 0x826609B0;
	sub_82466E20(ctx, base);
	// 826609B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826609B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826609B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826609BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826609C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826609C0 size=108
    let mut pc: u32 = 0x826609C0;
    'dispatch: loop {
        match pc {
            0x826609C0 => {
    //   block [0x826609C0..0x82660A2C)
	// 826609C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826609C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826609C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826609CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826609D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826609D4: 38EB3CF8  addi r7, r11, 0x3cf8
	ctx.r[7].s64 = ctx.r[11].s64 + 15608;
	// 826609D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826609DC: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826609E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826609E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826609E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826609EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826609F0: 386AAFF0  addi r3, r10, -0x5010
	ctx.r[3].s64 = ctx.r[10].s64 + -20496;
	// 826609F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826609F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826609FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660A14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660A18: 4BE06409  bl 0x82466e20
	ctx.lr = 0x82660A1C;
	sub_82466E20(ctx, base);
	// 82660A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660A30 size=112
    let mut pc: u32 = 0x82660A30;
    'dispatch: loop {
        match pc {
            0x82660A30 => {
    //   block [0x82660A30..0x82660AA0)
	// 82660A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660A3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660A40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660A44: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660A4C: 390B3D28  addi r8, r11, 0x3d28
	ctx.r[8].s64 = ctx.r[11].s64 + 15656;
	// 82660A50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82660A54: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 82660A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660A5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660A60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660A68: 386AB020  addi r3, r10, -0x4fe0
	ctx.r[3].s64 = ctx.r[10].s64 + -20448;
	// 82660A6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660A8C: 4BE06395  bl 0x82466e20
	ctx.lr = 0x82660A90;
	sub_82466E20(ctx, base);
	// 82660A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660AA0 size=112
    let mut pc: u32 = 0x82660AA0;
    'dispatch: loop {
        match pc {
            0x82660AA0 => {
    //   block [0x82660AA0..0x82660B10)
	// 82660AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660AAC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82660AB0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82660AB4: 38EA3D40  addi r7, r10, 0x3d40
	ctx.r[7].s64 = ctx.r[10].s64 + 15680;
	// 82660AB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660ABC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82660AC0: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 82660AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660AC8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660ACC: 396BEA8C  addi r11, r11, -0x1574
	ctx.r[11].s64 = ctx.r[11].s64 + -5492;
	// 82660AD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660AD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660AD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660ADC: 386AB050  addi r3, r10, -0x4fb0
	ctx.r[3].s64 = ctx.r[10].s64 + -20400;
	// 82660AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660AE4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82660AE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660AEC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82660AF0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660AF4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660AF8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660AFC: 4BE06325  bl 0x82466e20
	ctx.lr = 0x82660B00;
	sub_82466E20(ctx, base);
	// 82660B00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660B10 size=108
    let mut pc: u32 = 0x82660B10;
    'dispatch: loop {
        match pc {
            0x82660B10 => {
    //   block [0x82660B10..0x82660B7C)
	// 82660B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660B1C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660B24: 38EB3E18  addi r7, r11, 0x3e18
	ctx.r[7].s64 = ctx.r[11].s64 + 15896;
	// 82660B28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82660B2C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 82660B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660B34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660B38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660B40: 386AB080  addi r3, r10, -0x4f80
	ctx.r[3].s64 = ctx.r[10].s64 + -20352;
	// 82660B44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660B68: 4BE062B9  bl 0x82466e20
	ctx.lr = 0x82660B6C;
	sub_82466E20(ctx, base);
	// 82660B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660B80 size=108
    let mut pc: u32 = 0x82660B80;
    'dispatch: loop {
        match pc {
            0x82660B80 => {
    //   block [0x82660B80..0x82660BEC)
	// 82660B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660B8C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660B94: 38EB3E30  addi r7, r11, 0x3e30
	ctx.r[7].s64 = ctx.r[11].s64 + 15920;
	// 82660B98: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82660B9C: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 82660BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660BA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660BA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660BB0: 386AB0B0  addi r3, r10, -0x4f50
	ctx.r[3].s64 = ctx.r[10].s64 + -20304;
	// 82660BB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660BD8: 4BE06249  bl 0x82466e20
	ctx.lr = 0x82660BDC;
	sub_82466E20(ctx, base);
	// 82660BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660BF0 size=108
    let mut pc: u32 = 0x82660BF0;
    'dispatch: loop {
        match pc {
            0x82660BF0 => {
    //   block [0x82660BF0..0x82660C5C)
	// 82660BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660BFC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660C04: 38EB3F38  addi r7, r11, 0x3f38
	ctx.r[7].s64 = ctx.r[11].s64 + 16184;
	// 82660C08: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82660C0C: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 82660C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660C14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660C18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660C20: 386AB0E0  addi r3, r10, -0x4f20
	ctx.r[3].s64 = ctx.r[10].s64 + -20256;
	// 82660C24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660C48: 4BE061D9  bl 0x82466e20
	ctx.lr = 0x82660C4C;
	sub_82466E20(ctx, base);
	// 82660C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660C60 size=112
    let mut pc: u32 = 0x82660C60;
    'dispatch: loop {
        match pc {
            0x82660C60 => {
    //   block [0x82660C60..0x82660CD0)
	// 82660C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660C6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660C70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660C74: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660C7C: 390B3F98  addi r8, r11, 0x3f98
	ctx.r[8].s64 = ctx.r[11].s64 + 16280;
	// 82660C80: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82660C84: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82660C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660C8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660C90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660C98: 386AB110  addi r3, r10, -0x4ef0
	ctx.r[3].s64 = ctx.r[10].s64 + -20208;
	// 82660C9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660CBC: 4BE06165  bl 0x82466e20
	ctx.lr = 0x82660CC0;
	sub_82466E20(ctx, base);
	// 82660CC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660CD0 size=112
    let mut pc: u32 = 0x82660CD0;
    'dispatch: loop {
        match pc {
            0x82660CD0 => {
    //   block [0x82660CD0..0x82660D40)
	// 82660CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660CDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660CE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660CE4: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660CE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660CEC: 390B40B8  addi r8, r11, 0x40b8
	ctx.r[8].s64 = ctx.r[11].s64 + 16568;
	// 82660CF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82660CF4: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 82660CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660CFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660D00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660D08: 386AB140  addi r3, r10, -0x4ec0
	ctx.r[3].s64 = ctx.r[10].s64 + -20160;
	// 82660D0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660D2C: 4BE060F5  bl 0x82466e20
	ctx.lr = 0x82660D30;
	sub_82466E20(ctx, base);
	// 82660D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660D40 size=116
    let mut pc: u32 = 0x82660D40;
    'dispatch: loop {
        match pc {
            0x82660D40 => {
    //   block [0x82660D40..0x82660DB4)
	// 82660D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660D4C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82660D50: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82660D54: 390A40D0  addi r8, r10, 0x40d0
	ctx.r[8].s64 = ctx.r[10].s64 + 16592;
	// 82660D58: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660D5C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82660D60: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660D64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660D68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82660D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660D70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660D74: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82660D78: 396BEABC  addi r11, r11, -0x1544
	ctx.r[11].s64 = ctx.r[11].s64 + -5444;
	// 82660D7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660D80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660D84: 386AB170  addi r3, r10, -0x4e90
	ctx.r[3].s64 = ctx.r[10].s64 + -20112;
	// 82660D88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82660D8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660D90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82660D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660DA0: 4BE06081  bl 0x82466e20
	ctx.lr = 0x82660DA4;
	sub_82466E20(ctx, base);
	// 82660DA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660DB8 size=108
    let mut pc: u32 = 0x82660DB8;
    'dispatch: loop {
        match pc {
            0x82660DB8 => {
    //   block [0x82660DB8..0x82660E24)
	// 82660DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660DC4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660DC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82660DCC: 38EB4130  addi r7, r11, 0x4130
	ctx.r[7].s64 = ctx.r[11].s64 + 16688;
	// 82660DD0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82660DD4: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 82660DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660DDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660DE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660DE8: 386AB1A0  addi r3, r10, -0x4e60
	ctx.r[3].s64 = ctx.r[10].s64 + -20064;
	// 82660DEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660E0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660E10: 4BE06011  bl 0x82466e20
	ctx.lr = 0x82660E14;
	sub_82466E20(ctx, base);
	// 82660E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660E28 size=112
    let mut pc: u32 = 0x82660E28;
    'dispatch: loop {
        match pc {
            0x82660E28 => {
    //   block [0x82660E28..0x82660E98)
	// 82660E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660E34: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82660E38: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82660E3C: 38EA41D8  addi r7, r10, 0x41d8
	ctx.r[7].s64 = ctx.r[10].s64 + 16856;
	// 82660E40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82660E44: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82660E48: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 82660E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660E50: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82660E54: 396BEAD0  addi r11, r11, -0x1530
	ctx.r[11].s64 = ctx.r[11].s64 + -5424;
	// 82660E58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82660E5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660E60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660E64: 386AB1D0  addi r3, r10, -0x4e30
	ctx.r[3].s64 = ctx.r[10].s64 + -20016;
	// 82660E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660E6C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82660E70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660E74: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82660E78: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660E7C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660E80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82660E84: 4BE05F9D  bl 0x82466e20
	ctx.lr = 0x82660E88;
	sub_82466E20(ctx, base);
	// 82660E88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660E8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660E90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660E98 size=112
    let mut pc: u32 = 0x82660E98;
    'dispatch: loop {
        match pc {
            0x82660E98 => {
    //   block [0x82660E98..0x82660F08)
	// 82660E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660EA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660EAC: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660EB4: 390B4250  addi r8, r11, 0x4250
	ctx.r[8].s64 = ctx.r[11].s64 + 16976;
	// 82660EB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82660EBC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 82660EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660EC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660EC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660ED0: 386AB200  addi r3, r10, -0x4e00
	ctx.r[3].s64 = ctx.r[10].s64 + -19968;
	// 82660ED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660EF4: 4BE05F2D  bl 0x82466e20
	ctx.lr = 0x82660EF8;
	sub_82466E20(ctx, base);
	// 82660EF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660F04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660F08 size=112
    let mut pc: u32 = 0x82660F08;
    'dispatch: loop {
        match pc {
            0x82660F08 => {
    //   block [0x82660F08..0x82660F78)
	// 82660F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660F14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660F18: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660F1C: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660F24: 390B4298  addi r8, r11, 0x4298
	ctx.r[8].s64 = ctx.r[11].s64 + 17048;
	// 82660F28: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 82660F2C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 82660F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660F34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660F40: 386AB230  addi r3, r10, -0x4dd0
	ctx.r[3].s64 = ctx.r[10].s64 + -19920;
	// 82660F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660F64: 4BE05EBD  bl 0x82466e20
	ctx.lr = 0x82660F68;
	sub_82466E20(ctx, base);
	// 82660F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660F78 size=112
    let mut pc: u32 = 0x82660F78;
    'dispatch: loop {
        match pc {
            0x82660F78 => {
    //   block [0x82660F78..0x82660FE8)
	// 82660F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660F84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660F88: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660F8C: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82660F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82660F94: 390B43A0  addi r8, r11, 0x43a0
	ctx.r[8].s64 = ctx.r[11].s64 + 17312;
	// 82660F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82660F9C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 82660FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82660FA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82660FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82660FB0: 386AB260  addi r3, r10, -0x4da0
	ctx.r[3].s64 = ctx.r[10].s64 + -19872;
	// 82660FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82660FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82660FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82660FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82660FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82660FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82660FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82660FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82660FD4: 4BE05E4D  bl 0x82466e20
	ctx.lr = 0x82660FD8;
	sub_82466E20(ctx, base);
	// 82660FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82660FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82660FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82660FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82660FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82660FE8 size=112
    let mut pc: u32 = 0x82660FE8;
    'dispatch: loop {
        match pc {
            0x82660FE8 => {
    //   block [0x82660FE8..0x82661058)
	// 82660FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82660FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82660FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82660FF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82660FF8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82660FFC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661004: 390B43B8  addi r8, r11, 0x43b8
	ctx.r[8].s64 = ctx.r[11].s64 + 17336;
	// 82661008: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266100C: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 82661010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661014: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266101C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661020: 386AB290  addi r3, r10, -0x4d70
	ctx.r[3].s64 = ctx.r[10].s64 + -19824;
	// 82661024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266102C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266103C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661044: 4BE05DDD  bl 0x82466e20
	ctx.lr = 0x82661048;
	sub_82466E20(ctx, base);
	// 82661048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266104C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661058 size=108
    let mut pc: u32 = 0x82661058;
    'dispatch: loop {
        match pc {
            0x82661058 => {
    //   block [0x82661058..0x826610C4)
	// 82661058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266105C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661064: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266106C: 38EB43E8  addi r7, r11, 0x43e8
	ctx.r[7].s64 = ctx.r[11].s64 + 17384;
	// 82661070: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82661074: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 82661078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266107C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661080: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661088: 386AB2C0  addi r3, r10, -0x4d40
	ctx.r[3].s64 = ctx.r[10].s64 + -19776;
	// 8266108C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266109C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826610A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826610A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826610A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826610AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826610B0: 4BE05D71  bl 0x82466e20
	ctx.lr = 0x826610B4;
	sub_82466E20(ctx, base);
	// 826610B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826610B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826610BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826610C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826610C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826610C8 size=24
    let mut pc: u32 = 0x826610C8;
    'dispatch: loop {
        match pc {
            0x826610C8 => {
    //   block [0x826610C8..0x826610E0)
	// 826610C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826610CC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826610D0: 394AB7E0  addi r10, r10, -0x4820
	ctx.r[10].s64 = ctx.r[10].s64 + -18464;
	// 826610D4: 816B3BEC  lwz r11, 0x3bec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15340 as u32) ) } as u64;
	// 826610D8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826610DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826610E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826610E0 size=116
    let mut pc: u32 = 0x826610E0;
    'dispatch: loop {
        match pc {
            0x826610E0 => {
    //   block [0x826610E0..0x82661154)
	// 826610E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826610E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826610E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826610EC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826610F0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826610F4: 390BB7E0  addi r8, r11, -0x4820
	ctx.r[8].s64 = ctx.r[11].s64 + -18464;
	// 826610F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826610FC: 392AEB08  addi r9, r10, -0x14f8
	ctx.r[9].s64 = ctx.r[10].s64 + -5368;
	// 82661100: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661104: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82661108: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 8266110C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661114: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266111C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661124: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82661128: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 8266112C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82661130: 386BB2F0  addi r3, r11, -0x4d10
	ctx.r[3].s64 = ctx.r[11].s64 + -19728;
	// 82661134: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82661138: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266113C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661140: 4BE05CE1  bl 0x82466e20
	ctx.lr = 0x82661144;
	sub_82466E20(ctx, base);
	// 82661144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266114C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661158 size=112
    let mut pc: u32 = 0x82661158;
    'dispatch: loop {
        match pc {
            0x82661158 => {
    //   block [0x82661158..0x826611C8)
	// 82661158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266115C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661164: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661168: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266116C: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82661170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661174: 390B4460  addi r8, r11, 0x4460
	ctx.r[8].s64 = ctx.r[11].s64 + 17504;
	// 82661178: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266117C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82661180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266118C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661190: 386AB320  addi r3, r10, -0x4ce0
	ctx.r[3].s64 = ctx.r[10].s64 + -19680;
	// 82661194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266119C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826611A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826611A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826611A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826611AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826611B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826611B4: 4BE05C6D  bl 0x82466e20
	ctx.lr = 0x826611B8;
	sub_82466E20(ctx, base);
	// 826611B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826611BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826611C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826611C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826611C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826611C8 size=116
    let mut pc: u32 = 0x826611C8;
    'dispatch: loop {
        match pc {
            0x826611C8 => {
    //   block [0x826611C8..0x8266123C)
	// 826611C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826611CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826611D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826611D4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826611D8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826611DC: 390A4490  addi r8, r10, 0x4490
	ctx.r[8].s64 = ctx.r[10].s64 + 17552;
	// 826611E0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826611E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826611E8: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 826611EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826611F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826611F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826611F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826611FC: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82661200: 396BEB1C  addi r11, r11, -0x14e4
	ctx.r[11].s64 = ctx.r[11].s64 + -5348;
	// 82661204: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661208: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266120C: 386AB350  addi r3, r10, -0x4cb0
	ctx.r[3].s64 = ctx.r[10].s64 + -19632;
	// 82661210: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82661214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661218: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8266121C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661228: 4BE05BF9  bl 0x82466e20
	ctx.lr = 0x8266122C;
	sub_82466E20(ctx, base);
	// 8266122C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661240 size=112
    let mut pc: u32 = 0x82661240;
    'dispatch: loop {
        match pc {
            0x82661240 => {
    //   block [0x82661240..0x826612B0)
	// 82661240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266124C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661250: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661254: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82661258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266125C: 390B4550  addi r8, r11, 0x4550
	ctx.r[8].s64 = ctx.r[11].s64 + 17744;
	// 82661260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82661264: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 82661268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266126C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661278: 386AB380  addi r3, r10, -0x4c80
	ctx.r[3].s64 = ctx.r[10].s64 + -19584;
	// 8266127C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266128C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266129C: 4BE05B85  bl 0x82466e20
	ctx.lr = 0x826612A0;
	sub_82466E20(ctx, base);
	// 826612A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826612A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826612A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826612AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826612B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826612B0 size=108
    let mut pc: u32 = 0x826612B0;
    'dispatch: loop {
        match pc {
            0x826612B0 => {
    //   block [0x826612B0..0x8266131C)
	// 826612B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826612B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826612B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826612BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826612C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826612C4: 38EB4568  addi r7, r11, 0x4568
	ctx.r[7].s64 = ctx.r[11].s64 + 17768;
	// 826612C8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826612CC: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826612D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826612D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826612D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826612DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826612E0: 386AB3B0  addi r3, r10, -0x4c50
	ctx.r[3].s64 = ctx.r[10].s64 + -19536;
	// 826612E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826612E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826612EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826612F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826612F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826612F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826612FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661308: 4BE05B19  bl 0x82466e20
	ctx.lr = 0x8266130C;
	sub_82466E20(ctx, base);
	// 8266130C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661320 size=116
    let mut pc: u32 = 0x82661320;
    'dispatch: loop {
        match pc {
            0x82661320 => {
    //   block [0x82661320..0x82661394)
	// 82661320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266132C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82661330: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82661334: 390A46A0  addi r8, r10, 0x46a0
	ctx.r[8].s64 = ctx.r[10].s64 + 18080;
	// 82661338: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266133C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82661340: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82661344: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661348: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266134C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661354: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82661358: 396BEB40  addi r11, r11, -0x14c0
	ctx.r[11].s64 = ctx.r[11].s64 + -5312;
	// 8266135C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661360: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661364: 386AB3E0  addi r3, r10, -0x4c20
	ctx.r[3].s64 = ctx.r[10].s64 + -19488;
	// 82661368: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8266136C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661370: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82661374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266137C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661380: 4BE05AA1  bl 0x82466e20
	ctx.lr = 0x82661384;
	sub_82466E20(ctx, base);
	// 82661384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266138C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661398 size=112
    let mut pc: u32 = 0x82661398;
    'dispatch: loop {
        match pc {
            0x82661398 => {
    //   block [0x82661398..0x82661408)
	// 82661398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266139C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826613A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826613A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826613A8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826613AC: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 826613B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826613B4: 390B4748  addi r8, r11, 0x4748
	ctx.r[8].s64 = ctx.r[11].s64 + 18248;
	// 826613B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826613BC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826613C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826613C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826613C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826613CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826613D0: 386AB410  addi r3, r10, -0x4bf0
	ctx.r[3].s64 = ctx.r[10].s64 + -19440;
	// 826613D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826613D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826613DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826613E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826613E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826613E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826613EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826613F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826613F4: 4BE05A2D  bl 0x82466e20
	ctx.lr = 0x826613F8;
	sub_82466E20(ctx, base);
	// 826613F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826613FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661408 size=112
    let mut pc: u32 = 0x82661408;
    'dispatch: loop {
        match pc {
            0x82661408 => {
    //   block [0x82661408..0x82661478)
	// 82661408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266140C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661414: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661418: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266141C: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82661420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661424: 390B4760  addi r8, r11, 0x4760
	ctx.r[8].s64 = ctx.r[11].s64 + 18272;
	// 82661428: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 8266142C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 82661430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661434: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266143C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661440: 386AB440  addi r3, r10, -0x4bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -19392;
	// 82661444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266144C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266145C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661464: 4BE059BD  bl 0x82466e20
	ctx.lr = 0x82661468;
	sub_82466E20(ctx, base);
	// 82661468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266146C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661478 size=112
    let mut pc: u32 = 0x82661478;
    'dispatch: loop {
        match pc {
            0x82661478 => {
    //   block [0x82661478..0x826614E8)
	// 82661478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266147C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661484: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661488: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266148C: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 82661490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661494: 390B4898  addi r8, r11, 0x4898
	ctx.r[8].s64 = ctx.r[11].s64 + 18584;
	// 82661498: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266149C: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826614A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826614A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826614A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826614AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826614B0: 386AB470  addi r3, r10, -0x4b90
	ctx.r[3].s64 = ctx.r[10].s64 + -19344;
	// 826614B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826614B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826614BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826614C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826614C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826614C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826614CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826614D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826614D4: 4BE0594D  bl 0x82466e20
	ctx.lr = 0x826614D8;
	sub_82466E20(ctx, base);
	// 826614D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826614DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826614E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826614E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826614E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826614E8 size=116
    let mut pc: u32 = 0x826614E8;
    'dispatch: loop {
        match pc {
            0x826614E8 => {
    //   block [0x826614E8..0x8266155C)
	// 826614E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826614EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826614F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826614F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826614F8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826614FC: 390B48B4  addi r8, r11, 0x48b4
	ctx.r[8].s64 = ctx.r[11].s64 + 18612;
	// 82661500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661504: 392AEB78  addi r9, r10, -0x1488
	ctx.r[9].s64 = ctx.r[10].s64 + -5256;
	// 82661508: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266150C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82661510: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661514: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266151C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266152C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82661530: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 82661534: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82661538: 386BB4A0  addi r3, r11, -0x4b60
	ctx.r[3].s64 = ctx.r[11].s64 + -19296;
	// 8266153C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82661540: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661548: 4BE058D9  bl 0x82466e20
	ctx.lr = 0x8266154C;
	sub_82466E20(ctx, base);
	// 8266154C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661560 size=100
    let mut pc: u32 = 0x82661560;
    'dispatch: loop {
        match pc {
            0x82661560 => {
    //   block [0x82661560..0x826615C4)
	// 82661560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266156C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661574: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266157C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661580: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 82661584: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266158C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661594: 386AB4D0  addi r3, r10, -0x4b30
	ctx.r[3].s64 = ctx.r[10].s64 + -19248;
	// 82661598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266159C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826615A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826615A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826615A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826615AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826615B0: 4BE05871  bl 0x82466e20
	ctx.lr = 0x826615B4;
	sub_82466E20(ctx, base);
	// 826615B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826615B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826615BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826615C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826615C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826615C8 size=112
    let mut pc: u32 = 0x826615C8;
    'dispatch: loop {
        match pc {
            0x826615C8 => {
    //   block [0x826615C8..0x82661638)
	// 826615C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826615CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826615D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826615D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826615D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826615DC: 38AAB4D0  addi r5, r10, -0x4b30
	ctx.r[5].s64 = ctx.r[10].s64 + -19248;
	// 826615E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826615E4: 390B48E4  addi r8, r11, 0x48e4
	ctx.r[8].s64 = ctx.r[11].s64 + 18660;
	// 826615E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826615EC: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826615F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826615F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826615F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826615FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661600: 386AB500  addi r3, r10, -0x4b00
	ctx.r[3].s64 = ctx.r[10].s64 + -19200;
	// 82661604: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266160C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266161C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661624: 4BE057FD  bl 0x82466e20
	ctx.lr = 0x82661628;
	sub_82466E20(ctx, base);
	// 82661628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266162C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661638 size=112
    let mut pc: u32 = 0x82661638;
    'dispatch: loop {
        match pc {
            0x82661638 => {
    //   block [0x82661638..0x826616A8)
	// 82661638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266163C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661644: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661648: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266164C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661654: 390B4900  addi r8, r11, 0x4900
	ctx.r[8].s64 = ctx.r[11].s64 + 18688;
	// 82661658: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266165C: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 82661660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266166C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661670: 386AB530  addi r3, r10, -0x4ad0
	ctx.r[3].s64 = ctx.r[10].s64 + -19152;
	// 82661674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266167C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266168C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661694: 4BE0578D  bl 0x82466e20
	ctx.lr = 0x82661698;
	sub_82466E20(ctx, base);
	// 82661698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266169C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826616A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826616A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826616A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826616A8 size=112
    let mut pc: u32 = 0x826616A8;
    'dispatch: loop {
        match pc {
            0x826616A8 => {
    //   block [0x826616A8..0x82661718)
	// 826616A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826616AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826616B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826616B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826616B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826616BC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826616C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826616C4: 390B49A8  addi r8, r11, 0x49a8
	ctx.r[8].s64 = ctx.r[11].s64 + 18856;
	// 826616C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826616CC: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826616D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826616D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826616D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826616DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826616E0: 386AB560  addi r3, r10, -0x4aa0
	ctx.r[3].s64 = ctx.r[10].s64 + -19104;
	// 826616E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826616E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826616EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826616F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826616F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826616F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826616FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661704: 4BE0571D  bl 0x82466e20
	ctx.lr = 0x82661708;
	sub_82466E20(ctx, base);
	// 82661708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266170C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661718 size=112
    let mut pc: u32 = 0x82661718;
    'dispatch: loop {
        match pc {
            0x82661718 => {
    //   block [0x82661718..0x82661788)
	// 82661718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266171C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661724: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661728: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266172C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661734: 390B49F0  addi r8, r11, 0x49f0
	ctx.r[8].s64 = ctx.r[11].s64 + 18928;
	// 82661738: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266173C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 82661740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661744: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266174C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661750: 386AB590  addi r3, r10, -0x4a70
	ctx.r[3].s64 = ctx.r[10].s64 + -19056;
	// 82661754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266175C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266176C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661774: 4BE056AD  bl 0x82466e20
	ctx.lr = 0x82661778;
	sub_82466E20(ctx, base);
	// 82661778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266177C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661788 size=116
    let mut pc: u32 = 0x82661788;
    'dispatch: loop {
        match pc {
            0x82661788 => {
    //   block [0x82661788..0x826617FC)
	// 82661788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266178C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661794: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82661798: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8266179C: 390A4A20  addi r8, r10, 0x4a20
	ctx.r[8].s64 = ctx.r[10].s64 + 18976;
	// 826617A0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826617A4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826617A8: 38AAAF60  addi r5, r10, -0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + -20640;
	// 826617AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826617B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826617B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826617B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826617BC: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826617C0: 396BEB8C  addi r11, r11, -0x1474
	ctx.r[11].s64 = ctx.r[11].s64 + -5236;
	// 826617C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826617C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826617CC: 386AB5C0  addi r3, r10, -0x4a40
	ctx.r[3].s64 = ctx.r[10].s64 + -19008;
	// 826617D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826617D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826617D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826617DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826617E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826617E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826617E8: 4BE05639  bl 0x82466e20
	ctx.lr = 0x826617EC;
	sub_82466E20(ctx, base);
	// 826617EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826617F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826617F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826617F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661800 size=100
    let mut pc: u32 = 0x82661800;
    'dispatch: loop {
        match pc {
            0x82661800 => {
    //   block [0x82661800..0x82661864)
	// 82661800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266180C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661814: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266181C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661820: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 82661824: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266182C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661834: 386AB5F0  addi r3, r10, -0x4a10
	ctx.r[3].s64 = ctx.r[10].s64 + -18960;
	// 82661838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266183C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661840: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82661844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661848: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266184C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661850: 4BE055D1  bl 0x82466e20
	ctx.lr = 0x82661854;
	sub_82466E20(ctx, base);
	// 82661854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266185C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661868 size=108
    let mut pc: u32 = 0x82661868;
    'dispatch: loop {
        match pc {
            0x82661868 => {
    //   block [0x82661868..0x826618D4)
	// 82661868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266186C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661874: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266187C: 38EB4AE0  addi r7, r11, 0x4ae0
	ctx.r[7].s64 = ctx.r[11].s64 + 19168;
	// 82661880: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82661884: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82661888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266188C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661890: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661898: 386AB620  addi r3, r10, -0x49e0
	ctx.r[3].s64 = ctx.r[10].s64 + -18912;
	// 8266189C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826618A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826618A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826618A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826618AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826618B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826618B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826618B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826618BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826618C0: 4BE05561  bl 0x82466e20
	ctx.lr = 0x826618C4;
	sub_82466E20(ctx, base);
	// 826618C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826618C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826618CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826618D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826618D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826618D8 size=112
    let mut pc: u32 = 0x826618D8;
    'dispatch: loop {
        match pc {
            0x826618D8 => {
    //   block [0x826618D8..0x82661948)
	// 826618D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826618DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826618E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826618E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826618E8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826618EC: 38AAB5F0  addi r5, r10, -0x4a10
	ctx.r[5].s64 = ctx.r[10].s64 + -18960;
	// 826618F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826618F4: 390B4B10  addi r8, r11, 0x4b10
	ctx.r[8].s64 = ctx.r[11].s64 + 19216;
	// 826618F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826618FC: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 82661900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661904: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661908: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266190C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661910: 386AB650  addi r3, r10, -0x49b0
	ctx.r[3].s64 = ctx.r[10].s64 + -18864;
	// 82661914: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266191C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266192C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661934: 4BE054ED  bl 0x82466e20
	ctx.lr = 0x82661938;
	sub_82466E20(ctx, base);
	// 82661938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266193C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661948 size=108
    let mut pc: u32 = 0x82661948;
    'dispatch: loop {
        match pc {
            0x82661948 => {
    //   block [0x82661948..0x826619B4)
	// 82661948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266194C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661954: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266195C: 38EB4B40  addi r7, r11, 0x4b40
	ctx.r[7].s64 = ctx.r[11].s64 + 19264;
	// 82661960: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82661964: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 82661968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266196C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661970: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661978: 386AB680  addi r3, r10, -0x4980
	ctx.r[3].s64 = ctx.r[10].s64 + -18816;
	// 8266197C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266198C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266199C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826619A0: 4BE05481  bl 0x82466e20
	ctx.lr = 0x826619A4;
	sub_82466E20(ctx, base);
	// 826619A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826619A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826619AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826619B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826619B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826619B8 size=112
    let mut pc: u32 = 0x826619B8;
    'dispatch: loop {
        match pc {
            0x826619B8 => {
    //   block [0x826619B8..0x82661A28)
	// 826619B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826619BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826619C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826619C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826619C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826619CC: 38AAB5F0  addi r5, r10, -0x4a10
	ctx.r[5].s64 = ctx.r[10].s64 + -18960;
	// 826619D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826619D4: 390B4B70  addi r8, r11, 0x4b70
	ctx.r[8].s64 = ctx.r[11].s64 + 19312;
	// 826619D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826619DC: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826619E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826619E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826619E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826619EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826619F0: 386AB6B0  addi r3, r10, -0x4950
	ctx.r[3].s64 = ctx.r[10].s64 + -18768;
	// 826619F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826619F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826619FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661A14: 4BE0540D  bl 0x82466e20
	ctx.lr = 0x82661A18;
	sub_82466E20(ctx, base);
	// 82661A18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661A28 size=108
    let mut pc: u32 = 0x82661A28;
    'dispatch: loop {
        match pc {
            0x82661A28 => {
    //   block [0x82661A28..0x82661A94)
	// 82661A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661A34: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661A3C: 38EB4BB8  addi r7, r11, 0x4bb8
	ctx.r[7].s64 = ctx.r[11].s64 + 19384;
	// 82661A40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82661A44: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 82661A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661A4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661A50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661A58: 386AB6E0  addi r3, r10, -0x4920
	ctx.r[3].s64 = ctx.r[10].s64 + -18720;
	// 82661A5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661A80: 4BE053A1  bl 0x82466e20
	ctx.lr = 0x82661A84;
	sub_82466E20(ctx, base);
	// 82661A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661A98 size=112
    let mut pc: u32 = 0x82661A98;
    'dispatch: loop {
        match pc {
            0x82661A98 => {
    //   block [0x82661A98..0x82661B08)
	// 82661A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661AA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661AA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661AAC: 38AAB5F0  addi r5, r10, -0x4a10
	ctx.r[5].s64 = ctx.r[10].s64 + -18960;
	// 82661AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661AB4: 390B4BE8  addi r8, r11, 0x4be8
	ctx.r[8].s64 = ctx.r[11].s64 + 19432;
	// 82661AB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82661ABC: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 82661AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661AC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661AD0: 386AB710  addi r3, r10, -0x48f0
	ctx.r[3].s64 = ctx.r[10].s64 + -18672;
	// 82661AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661AF4: 4BE0532D  bl 0x82466e20
	ctx.lr = 0x82661AF8;
	sub_82466E20(ctx, base);
	// 82661AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661B08 size=108
    let mut pc: u32 = 0x82661B08;
    'dispatch: loop {
        match pc {
            0x82661B08 => {
    //   block [0x82661B08..0x82661B74)
	// 82661B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661B14: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661B1C: 38EB4C30  addi r7, r11, 0x4c30
	ctx.r[7].s64 = ctx.r[11].s64 + 19504;
	// 82661B20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82661B24: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 82661B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661B2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661B30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661B38: 386AB740  addi r3, r10, -0x48c0
	ctx.r[3].s64 = ctx.r[10].s64 + -18624;
	// 82661B3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661B5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661B60: 4BE052C1  bl 0x82466e20
	ctx.lr = 0x82661B64;
	sub_82466E20(ctx, base);
	// 82661B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661B78 size=112
    let mut pc: u32 = 0x82661B78;
    'dispatch: loop {
        match pc {
            0x82661B78 => {
    //   block [0x82661B78..0x82661BE8)
	// 82661B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661B84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661B88: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661B8C: 38AAB5F0  addi r5, r10, -0x4a10
	ctx.r[5].s64 = ctx.r[10].s64 + -18960;
	// 82661B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661B94: 390B4C60  addi r8, r11, 0x4c60
	ctx.r[8].s64 = ctx.r[11].s64 + 19552;
	// 82661B98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82661B9C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82661BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661BA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661BB0: 386AB770  addi r3, r10, -0x4890
	ctx.r[3].s64 = ctx.r[10].s64 + -18576;
	// 82661BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661BD4: 4BE0524D  bl 0x82466e20
	ctx.lr = 0x82661BD8;
	sub_82466E20(ctx, base);
	// 82661BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661BE8 size=108
    let mut pc: u32 = 0x82661BE8;
    'dispatch: loop {
        match pc {
            0x82661BE8 => {
    //   block [0x82661BE8..0x82661C54)
	// 82661BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661BF4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661BFC: 38EB4CA8  addi r7, r11, 0x4ca8
	ctx.r[7].s64 = ctx.r[11].s64 + 19624;
	// 82661C00: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82661C04: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 82661C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661C0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661C10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661C14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661C18: 386AB7A0  addi r3, r10, -0x4860
	ctx.r[3].s64 = ctx.r[10].s64 + -18528;
	// 82661C1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661C20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661C3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661C40: 4BE051E1  bl 0x82466e20
	ctx.lr = 0x82661C44;
	sub_82466E20(ctx, base);
	// 82661C44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661C58 size=112
    let mut pc: u32 = 0x82661C58;
    'dispatch: loop {
        match pc {
            0x82661C58 => {
    //   block [0x82661C58..0x82661CC8)
	// 82661C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661C64: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82661C68: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661C6C: 392AEC48  addi r9, r10, -0x13b8
	ctx.r[9].s64 = ctx.r[10].s64 + -5048;
	// 82661C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661C74: 390B4D10  addi r8, r11, 0x4d10
	ctx.r[8].s64 = ctx.r[11].s64 + 19728;
	// 82661C78: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82661C7C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82661C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661C84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661C88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661C90: 386AB7D0  addi r3, r10, -0x4830
	ctx.r[3].s64 = ctx.r[10].s64 + -18480;
	// 82661C94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82661C98: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82661C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661CB4: 4BE0516D  bl 0x82466e20
	ctx.lr = 0x82661CB8;
	sub_82466E20(ctx, base);
	// 82661CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661CC8 size=108
    let mut pc: u32 = 0x82661CC8;
    'dispatch: loop {
        match pc {
            0x82661CC8 => {
    //   block [0x82661CC8..0x82661D34)
	// 82661CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661CD4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661CDC: 38EB4DD0  addi r7, r11, 0x4dd0
	ctx.r[7].s64 = ctx.r[11].s64 + 19920;
	// 82661CE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82661CE4: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 82661CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661CF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661CF8: 386AB800  addi r3, r10, -0x4800
	ctx.r[3].s64 = ctx.r[10].s64 + -18432;
	// 82661CFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661D20: 4BE05101  bl 0x82466e20
	ctx.lr = 0x82661D24;
	sub_82466E20(ctx, base);
	// 82661D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661D38 size=108
    let mut pc: u32 = 0x82661D38;
    'dispatch: loop {
        match pc {
            0x82661D38 => {
    //   block [0x82661D38..0x82661DA4)
	// 82661D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661D44: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661D4C: 38EB4E30  addi r7, r11, 0x4e30
	ctx.r[7].s64 = ctx.r[11].s64 + 20016;
	// 82661D50: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82661D54: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 82661D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661D5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661D60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82661D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661D68: 386AB830  addi r3, r10, -0x47d0
	ctx.r[3].s64 = ctx.r[10].s64 + -18384;
	// 82661D6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82661D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82661D90: 4BE05091  bl 0x82466e20
	ctx.lr = 0x82661D94;
	sub_82466E20(ctx, base);
	// 82661D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82661DA8 size=24
    let mut pc: u32 = 0x82661DA8;
    'dispatch: loop {
        match pc {
            0x82661DA8 => {
    //   block [0x82661DA8..0x82661DC0)
	// 82661DA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661DAC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82661DB0: 394AB8E8  addi r10, r10, -0x4718
	ctx.r[10].s64 = ctx.r[10].s64 + -18200;
	// 82661DB4: 816B48FC  lwz r11, 0x48fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18684 as u32) ) } as u64;
	// 82661DB8: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82661DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661DC0 size=116
    let mut pc: u32 = 0x82661DC0;
    'dispatch: loop {
        match pc {
            0x82661DC0 => {
    //   block [0x82661DC0..0x82661E34)
	// 82661DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661DCC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82661DD0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661DD4: 392BEBD4  addi r9, r11, -0x142c
	ctx.r[9].s64 = ctx.r[11].s64 + -5164;
	// 82661DD8: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 82661DDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661DE0: 38E9008C  addi r7, r9, 0x8c
	ctx.r[7].s64 = ctx.r[9].s64 + 140;
	// 82661DE4: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 82661DE8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82661DEC: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82661DF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661DF4: 396BB8E8  addi r11, r11, -0x4718
	ctx.r[11].s64 = ctx.r[11].s64 + -18200;
	// 82661DF8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82661DFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661E00: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82661E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661E08: 386AB860  addi r3, r10, -0x47a0
	ctx.r[3].s64 = ctx.r[10].s64 + -18336;
	// 82661E0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82661E10: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82661E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661E18: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82661E1C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82661E20: 4BE05001  bl 0x82466e20
	ctx.lr = 0x82661E24;
	sub_82466E20(ctx, base);
	// 82661E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661E38 size=100
    let mut pc: u32 = 0x82661E38;
    'dispatch: loop {
        match pc {
            0x82661E38 => {
    //   block [0x82661E38..0x82661E9C)
	// 82661E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661E44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661E4C: 38AAACF0  addi r5, r10, -0x5310
	ctx.r[5].s64 = ctx.r[10].s64 + -21264;
	// 82661E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661E58: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 82661E5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661E6C: 386AB890  addi r3, r10, -0x4770
	ctx.r[3].s64 = ctx.r[10].s64 + -18288;
	// 82661E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82661E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82661E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661E88: 4BE04F99  bl 0x82466e20
	ctx.lr = 0x82661E8C;
	sub_82466E20(ctx, base);
	// 82661E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82661EA0 size=24
    let mut pc: u32 = 0x82661EA0;
    'dispatch: loop {
        match pc {
            0x82661EA0 => {
    //   block [0x82661EA0..0x82661EB8)
	// 82661EA0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661EA4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82661EA8: 394ABAB0  addi r10, r10, -0x4550
	ctx.r[10].s64 = ctx.r[10].s64 + -17744;
	// 82661EAC: 816B4ED8  lwz r11, 0x4ed8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20184 as u32) ) } as u64;
	// 82661EB0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82661EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661EB8 size=116
    let mut pc: u32 = 0x82661EB8;
    'dispatch: loop {
        match pc {
            0x82661EB8 => {
    //   block [0x82661EB8..0x82661F2C)
	// 82661EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661EC4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82661EC8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82661ECC: 390BBAB0  addi r8, r11, -0x4550
	ctx.r[8].s64 = ctx.r[11].s64 + -17744;
	// 82661ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661ED4: 392AECE4  addi r9, r10, -0x131c
	ctx.r[9].s64 = ctx.r[10].s64 + -4892;
	// 82661ED8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661EDC: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82661EE0: 38AAB890  addi r5, r10, -0x4770
	ctx.r[5].s64 = ctx.r[10].s64 + -18288;
	// 82661EE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661EEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661EFC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82661F00: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 82661F04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82661F08: 386BB8C0  addi r3, r11, -0x4740
	ctx.r[3].s64 = ctx.r[11].s64 + -18240;
	// 82661F0C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82661F10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661F18: 4BE04F09  bl 0x82466e20
	ctx.lr = 0x82661F1C;
	sub_82466E20(ctx, base);
	// 82661F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661F30 size=112
    let mut pc: u32 = 0x82661F30;
    'dispatch: loop {
        match pc {
            0x82661F30 => {
    //   block [0x82661F30..0x82661FA0)
	// 82661F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661F3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661F40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661F44: 38AAB890  addi r5, r10, -0x4770
	ctx.r[5].s64 = ctx.r[10].s64 + -18288;
	// 82661F48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82661F4C: 390B4EE0  addi r8, r11, 0x4ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 20192;
	// 82661F50: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82661F54: 388AAAE8  addi r4, r10, -0x5518
	ctx.r[4].s64 = ctx.r[10].s64 + -21784;
	// 82661F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661F5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661F68: 386AB8F0  addi r3, r10, -0x4710
	ctx.r[3].s64 = ctx.r[10].s64 + -18192;
	// 82661F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661F8C: 4BE04E95  bl 0x82466e20
	ctx.lr = 0x82661F90;
	sub_82466E20(ctx, base);
	// 82661F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82661F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82661F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82661F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82661FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82661FA0 size=112
    let mut pc: u32 = 0x82661FA0;
    'dispatch: loop {
        match pc {
            0x82661FA0 => {
    //   block [0x82661FA0..0x82662010)
	// 82661FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82661FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82661FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82661FAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661FB0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82661FB4: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82661FB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82661FBC: 390B4FB8  addi r8, r11, 0x4fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 20408;
	// 82661FC0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82661FC4: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 82661FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82661FCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82661FD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82661FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82661FD8: 386AB920  addi r3, r10, -0x46e0
	ctx.r[3].s64 = ctx.r[10].s64 + -18144;
	// 82661FDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82661FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82661FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82661FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82661FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82661FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82661FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82661FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82661FFC: 4BE04E25  bl 0x82466e20
	ctx.lr = 0x82662000;
	sub_82466E20(ctx, base);
	// 82662000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266200C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662010 size=108
    let mut pc: u32 = 0x82662010;
    'dispatch: loop {
        match pc {
            0x82662010 => {
    //   block [0x82662010..0x8266207C)
	// 82662010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266201C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662024: 38EB5090  addi r7, r11, 0x5090
	ctx.r[7].s64 = ctx.r[11].s64 + 20624;
	// 82662028: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8266202C: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 82662030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662034: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266203C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662040: 386AB950  addi r3, r10, -0x46b0
	ctx.r[3].s64 = ctx.r[10].s64 + -18096;
	// 82662044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266204C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266205C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662068: 4BE04DB9  bl 0x82466e20
	ctx.lr = 0x8266206C;
	sub_82466E20(ctx, base);
	// 8266206C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662080 size=108
    let mut pc: u32 = 0x82662080;
    'dispatch: loop {
        match pc {
            0x82662080 => {
    //   block [0x82662080..0x826620EC)
	// 82662080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266208C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662094: 38EB5108  addi r7, r11, 0x5108
	ctx.r[7].s64 = ctx.r[11].s64 + 20744;
	// 82662098: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266209C: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826620A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826620A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826620A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826620AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826620B0: 386AB980  addi r3, r10, -0x4680
	ctx.r[3].s64 = ctx.r[10].s64 + -18048;
	// 826620B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826620B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826620BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826620C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826620C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826620C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826620CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826620D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826620D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826620D8: 4BE04D49  bl 0x82466e20
	ctx.lr = 0x826620DC;
	sub_82466E20(ctx, base);
	// 826620DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826620E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826620E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826620E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826620F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826620F0 size=112
    let mut pc: u32 = 0x826620F0;
    'dispatch: loop {
        match pc {
            0x826620F0 => {
    //   block [0x826620F0..0x82662160)
	// 826620F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826620F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826620F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826620FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662100: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662104: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82662108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266210C: 390B5150  addi r8, r11, 0x5150
	ctx.r[8].s64 = ctx.r[11].s64 + 20816;
	// 82662110: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82662114: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 82662118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266211C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662128: 386AB9B0  addi r3, r10, -0x4650
	ctx.r[3].s64 = ctx.r[10].s64 + -18000;
	// 8266212C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82662130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266213C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266214C: 4BE04CD5  bl 0x82466e20
	ctx.lr = 0x82662150;
	sub_82466E20(ctx, base);
	// 82662150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266215C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662160 size=108
    let mut pc: u32 = 0x82662160;
    'dispatch: loop {
        match pc {
            0x82662160 => {
    //   block [0x82662160..0x826621CC)
	// 82662160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266216C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662174: 38EB5330  addi r7, r11, 0x5330
	ctx.r[7].s64 = ctx.r[11].s64 + 21296;
	// 82662178: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266217C: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 82662180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266218C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662190: 386AB9E0  addi r3, r10, -0x4620
	ctx.r[3].s64 = ctx.r[10].s64 + -17952;
	// 82662194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266219C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826621A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826621A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826621A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826621AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826621B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826621B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826621B8: 4BE04C69  bl 0x82466e20
	ctx.lr = 0x826621BC;
	sub_82466E20(ctx, base);
	// 826621BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826621C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826621C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826621C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826621D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826621D0 size=24
    let mut pc: u32 = 0x826621D0;
    'dispatch: loop {
        match pc {
            0x826621D0 => {
    //   block [0x826621D0..0x826621E8)
	// 826621D0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826621D4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826621D8: 394ABBD0  addi r10, r10, -0x4430
	ctx.r[10].s64 = ctx.r[10].s64 + -17456;
	// 826621DC: 816B4EDC  lwz r11, 0x4edc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20188 as u32) ) } as u64;
	// 826621E0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826621E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826621E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826621E8 size=112
    let mut pc: u32 = 0x826621E8;
    'dispatch: loop {
        match pc {
            0x826621E8 => {
    //   block [0x826621E8..0x82662258)
	// 826621E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826621EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826621F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826621F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826621F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826621FC: 392AED3C  addi r9, r10, -0x12c4
	ctx.r[9].s64 = ctx.r[10].s64 + -4804;
	// 82662200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662204: 390BBBD0  addi r8, r11, -0x4430
	ctx.r[8].s64 = ctx.r[11].s64 + -17456;
	// 82662208: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8266220C: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82662210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266221C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662220: 386ABA10  addi r3, r10, -0x45f0
	ctx.r[3].s64 = ctx.r[10].s64 + -17904;
	// 82662224: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662228: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266222C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266223C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662244: 4BE04BDD  bl 0x82466e20
	ctx.lr = 0x82662248;
	sub_82466E20(ctx, base);
	// 82662248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266224C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662258 size=112
    let mut pc: u32 = 0x82662258;
    'dispatch: loop {
        match pc {
            0x82662258 => {
    //   block [0x82662258..0x826622C8)
	// 82662258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662264: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82662268: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8266226C: 38EA5348  addi r7, r10, 0x5348
	ctx.r[7].s64 = ctx.r[10].s64 + 21320;
	// 82662270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662274: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82662278: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 8266227C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662280: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662284: 396BED50  addi r11, r11, -0x12b0
	ctx.r[11].s64 = ctx.r[11].s64 + -4784;
	// 82662288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266228C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662290: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662294: 386ABA40  addi r3, r10, -0x45c0
	ctx.r[3].s64 = ctx.r[10].s64 + -17856;
	// 82662298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266229C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826622A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826622A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826622A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826622AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826622B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826622B4: 4BE04B6D  bl 0x82466e20
	ctx.lr = 0x826622B8;
	sub_82466E20(ctx, base);
	// 826622B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826622BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826622C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826622C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826622C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826622C8 size=112
    let mut pc: u32 = 0x826622C8;
    'dispatch: loop {
        match pc {
            0x826622C8 => {
    //   block [0x826622C8..0x82662338)
	// 826622C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826622CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826622D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826622D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826622D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826622DC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 826622E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826622E4: 390B53D8  addi r8, r11, 0x53d8
	ctx.r[8].s64 = ctx.r[11].s64 + 21464;
	// 826622E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826622EC: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 826622F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826622F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826622F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826622FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662300: 386ABA70  addi r3, r10, -0x4590
	ctx.r[3].s64 = ctx.r[10].s64 + -17808;
	// 82662304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82662308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266230C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266231C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662324: 4BE04AFD  bl 0x82466e20
	ctx.lr = 0x82662328;
	sub_82466E20(ctx, base);
	// 82662328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266232C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662338 size=108
    let mut pc: u32 = 0x82662338;
    'dispatch: loop {
        match pc {
            0x82662338 => {
    //   block [0x82662338..0x826623A4)
	// 82662338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266233C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662344: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266234C: 38EB53F8  addi r7, r11, 0x53f8
	ctx.r[7].s64 = ctx.r[11].s64 + 21496;
	// 82662350: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82662354: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82662358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266235C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662368: 386ABAA0  addi r3, r10, -0x4560
	ctx.r[3].s64 = ctx.r[10].s64 + -17760;
	// 8266236C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266237C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266238C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662390: 4BE04A91  bl 0x82466e20
	ctx.lr = 0x82662394;
	sub_82466E20(ctx, base);
	// 82662394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266239C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826623A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826623A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826623A8 size=108
    let mut pc: u32 = 0x826623A8;
    'dispatch: loop {
        match pc {
            0x826623A8 => {
    //   block [0x826623A8..0x82662414)
	// 826623A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826623AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826623B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826623B4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826623B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826623BC: 38EB5458  addi r7, r11, 0x5458
	ctx.r[7].s64 = ctx.r[11].s64 + 21592;
	// 826623C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826623C4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826623C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826623CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826623D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826623D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826623D8: 386ABAD0  addi r3, r10, -0x4530
	ctx.r[3].s64 = ctx.r[10].s64 + -17712;
	// 826623DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826623E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826623E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826623E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826623EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826623F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826623F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826623F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826623FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662400: 4BE04A21  bl 0x82466e20
	ctx.lr = 0x82662404;
	sub_82466E20(ctx, base);
	// 82662404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266240C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662418 size=116
    let mut pc: u32 = 0x82662418;
    'dispatch: loop {
        match pc {
            0x82662418 => {
    //   block [0x82662418..0x8266248C)
	// 82662418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266241C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662424: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662428: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266242C: 390B5488  addi r8, r11, 0x5488
	ctx.r[8].s64 = ctx.r[11].s64 + 21640;
	// 82662430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662434: 392AED84  addi r9, r10, -0x127c
	ctx.r[9].s64 = ctx.r[10].s64 + -4732;
	// 82662438: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266243C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82662440: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 82662444: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266244C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266245C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82662460: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82662464: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662468: 386BBB00  addi r3, r11, -0x4500
	ctx.r[3].s64 = ctx.r[11].s64 + -17664;
	// 8266246C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82662470: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662478: 4BE049A9  bl 0x82466e20
	ctx.lr = 0x8266247C;
	sub_82466E20(ctx, base);
	// 8266247C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662490 size=96
    let mut pc: u32 = 0x82662490;
    'dispatch: loop {
        match pc {
            0x82662490 => {
    //   block [0x82662490..0x826624F0)
	// 82662490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266249C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826624A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826624A4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826624A8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826624AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826624B0: 386ABB30  addi r3, r10, -0x44d0
	ctx.r[3].s64 = ctx.r[10].s64 + -17616;
	// 826624B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826624B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826624BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826624C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826624C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826624C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826624CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826624D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826624D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826624D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826624DC: 4BE04945  bl 0x82466e20
	ctx.lr = 0x826624E0;
	sub_82466E20(ctx, base);
	// 826624E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826624E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826624E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826624EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826624F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826624F0 size=112
    let mut pc: u32 = 0x826624F0;
    'dispatch: loop {
        match pc {
            0x826624F0 => {
    //   block [0x826624F0..0x82662560)
	// 826624F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826624F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826624F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826624FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662500: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662504: 38AABB30  addi r5, r10, -0x44d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17616;
	// 82662508: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266250C: 390B54A0  addi r8, r11, 0x54a0
	ctx.r[8].s64 = ctx.r[11].s64 + 21664;
	// 82662510: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82662514: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82662518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266251C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662528: 386ABB60  addi r3, r10, -0x44a0
	ctx.r[3].s64 = ctx.r[10].s64 + -17568;
	// 8266252C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82662530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266253C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266254C: 4BE048D5  bl 0x82466e20
	ctx.lr = 0x82662550;
	sub_82466E20(ctx, base);
	// 82662550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266255C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662560 size=112
    let mut pc: u32 = 0x82662560;
    'dispatch: loop {
        match pc {
            0x82662560 => {
    //   block [0x82662560..0x826625D0)
	// 82662560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266256C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82662570: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662574: 392AEDA0  addi r9, r10, -0x1260
	ctx.r[9].s64 = ctx.r[10].s64 + -4704;
	// 82662578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266257C: 390B54D0  addi r8, r11, 0x54d0
	ctx.r[8].s64 = ctx.r[11].s64 + 21712;
	// 82662580: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82662584: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82662588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266258C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662598: 386ABB90  addi r3, r10, -0x4470
	ctx.r[3].s64 = ctx.r[10].s64 + -17520;
	// 8266259C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826625A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826625A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826625A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826625AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826625B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826625B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826625B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826625BC: 4BE04865  bl 0x82466e20
	ctx.lr = 0x826625C0;
	sub_82466E20(ctx, base);
	// 826625C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826625C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826625C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826625CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826625D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826625D0 size=108
    let mut pc: u32 = 0x826625D0;
    'dispatch: loop {
        match pc {
            0x826625D0 => {
    //   block [0x826625D0..0x8266263C)
	// 826625D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826625D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826625D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826625DC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826625E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826625E4: 38EB5578  addi r7, r11, 0x5578
	ctx.r[7].s64 = ctx.r[11].s64 + 21880;
	// 826625E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826625EC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826625F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826625F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826625F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826625FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662600: 386ABBC0  addi r3, r10, -0x4440
	ctx.r[3].s64 = ctx.r[10].s64 + -17472;
	// 82662604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266260C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266261C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662628: 4BE047F9  bl 0x82466e20
	ctx.lr = 0x8266262C;
	sub_82466E20(ctx, base);
	// 8266262C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662640 size=108
    let mut pc: u32 = 0x82662640;
    'dispatch: loop {
        match pc {
            0x82662640 => {
    //   block [0x82662640..0x826626AC)
	// 82662640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266264C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662650: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662654: 38EB55A8  addi r7, r11, 0x55a8
	ctx.r[7].s64 = ctx.r[11].s64 + 21928;
	// 82662658: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266265C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82662660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266266C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662670: 386ABBF0  addi r3, r10, -0x4410
	ctx.r[3].s64 = ctx.r[10].s64 + -17424;
	// 82662674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266267C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266268C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662698: 4BE04789  bl 0x82466e20
	ctx.lr = 0x8266269C;
	sub_82466E20(ctx, base);
	// 8266269C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826626A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826626A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826626A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826626B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826626B0 size=28
    let mut pc: u32 = 0x826626B0;
    'dispatch: loop {
        match pc {
            0x826626B0 => {
    //   block [0x826626B0..0x826626CC)
	// 826626B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826626B4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826626B8: 394ABC00  addi r10, r10, -0x4400
	ctx.r[10].s64 = ctx.r[10].s64 + -17408;
	// 826626BC: 816B55D8  lwz r11, 0x55d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21976 as u32) ) } as u64;
	// 826626C0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826626C4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826626C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826626D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826626D0 size=112
    let mut pc: u32 = 0x826626D0;
    'dispatch: loop {
        match pc {
            0x826626D0 => {
    //   block [0x826626D0..0x82662740)
	// 826626D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826626D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826626D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826626DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826626E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826626E4: 392AEF50  addi r9, r10, -0x10b0
	ctx.r[9].s64 = ctx.r[10].s64 + -4272;
	// 826626E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826626EC: 390BBC00  addi r8, r11, -0x4400
	ctx.r[8].s64 = ctx.r[11].s64 + -17408;
	// 826626F0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826626F4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826626F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826626FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662708: 386ABC20  addi r3, r10, -0x43e0
	ctx.r[3].s64 = ctx.r[10].s64 + -17376;
	// 8266270C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662710: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82662714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266271C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266272C: 4BE046F5  bl 0x82466e20
	ctx.lr = 0x82662730;
	sub_82466E20(ctx, base);
	// 82662730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266273C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662740 size=108
    let mut pc: u32 = 0x82662740;
    'dispatch: loop {
        match pc {
            0x82662740 => {
    //   block [0x82662740..0x826627AC)
	// 82662740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266274C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662750: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662754: 38EB55E4  addi r7, r11, 0x55e4
	ctx.r[7].s64 = ctx.r[11].s64 + 21988;
	// 82662758: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266275C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82662760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266276C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662770: 386ABC50  addi r3, r10, -0x43b0
	ctx.r[3].s64 = ctx.r[10].s64 + -17328;
	// 82662774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266277C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266278C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662798: 4BE04689  bl 0x82466e20
	ctx.lr = 0x8266279C;
	sub_82466E20(ctx, base);
	// 8266279C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826627A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826627A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826627A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826627B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826627B0 size=108
    let mut pc: u32 = 0x826627B0;
    'dispatch: loop {
        match pc {
            0x826627B0 => {
    //   block [0x826627B0..0x8266281C)
	// 826627B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826627B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826627B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826627BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826627C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826627C4: 38EB5614  addi r7, r11, 0x5614
	ctx.r[7].s64 = ctx.r[11].s64 + 22036;
	// 826627C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826627CC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826627D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826627D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826627D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826627DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826627E0: 386ABC80  addi r3, r10, -0x4380
	ctx.r[3].s64 = ctx.r[10].s64 + -17280;
	// 826627E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826627E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826627EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826627F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826627F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826627F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826627FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662808: 4BE04619  bl 0x82466e20
	ctx.lr = 0x8266280C;
	sub_82466E20(ctx, base);
	// 8266280C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662820 size=108
    let mut pc: u32 = 0x82662820;
    'dispatch: loop {
        match pc {
            0x82662820 => {
    //   block [0x82662820..0x8266288C)
	// 82662820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266282C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662830: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662834: 38EB5644  addi r7, r11, 0x5644
	ctx.r[7].s64 = ctx.r[11].s64 + 22084;
	// 82662838: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266283C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 82662840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662844: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266284C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662850: 386ABCB0  addi r3, r10, -0x4350
	ctx.r[3].s64 = ctx.r[10].s64 + -17232;
	// 82662854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266285C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266286C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662878: 4BE045A9  bl 0x82466e20
	ctx.lr = 0x8266287C;
	sub_82466E20(ctx, base);
	// 8266287C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82662890 size=24
    let mut pc: u32 = 0x82662890;
    'dispatch: loop {
        match pc {
            0x82662890 => {
    //   block [0x82662890..0x826628A8)
	// 82662890: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662894: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82662898: 394ABCC0  addi r10, r10, -0x4340
	ctx.r[10].s64 = ctx.r[10].s64 + -17216;
	// 8266289C: 816B565C  lwz r11, 0x565c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22108 as u32) ) } as u64;
	// 826628A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826628A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826628A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826628A8 size=112
    let mut pc: u32 = 0x826628A8;
    'dispatch: loop {
        match pc {
            0x826628A8 => {
    //   block [0x826628A8..0x82662918)
	// 826628A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826628AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826628B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826628B4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826628B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826628BC: 392AEFA4  addi r9, r10, -0x105c
	ctx.r[9].s64 = ctx.r[10].s64 + -4188;
	// 826628C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826628C4: 390BBCC0  addi r8, r11, -0x4340
	ctx.r[8].s64 = ctx.r[11].s64 + -17216;
	// 826628C8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826628CC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826628D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826628D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826628D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826628DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826628E0: 386ABCE0  addi r3, r10, -0x4320
	ctx.r[3].s64 = ctx.r[10].s64 + -17184;
	// 826628E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826628E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826628EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826628F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826628F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826628F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826628FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662904: 4BE0451D  bl 0x82466e20
	ctx.lr = 0x82662908;
	sub_82466E20(ctx, base);
	// 82662908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266290C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662918 size=112
    let mut pc: u32 = 0x82662918;
    'dispatch: loop {
        match pc {
            0x82662918 => {
    //   block [0x82662918..0x82662988)
	// 82662918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266291C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662924: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82662928: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8266292C: 392AEFE0  addi r9, r10, -0x1020
	ctx.r[9].s64 = ctx.r[10].s64 + -4128;
	// 82662930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662934: 390B5668  addi r8, r11, 0x5668
	ctx.r[8].s64 = ctx.r[11].s64 + 22120;
	// 82662938: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8266293C: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 82662940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662944: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266294C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662950: 386ABD10  addi r3, r10, -0x42f0
	ctx.r[3].s64 = ctx.r[10].s64 + -17136;
	// 82662954: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662958: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8266295C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266296C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662974: 4BE044AD  bl 0x82466e20
	ctx.lr = 0x82662978;
	sub_82466E20(ctx, base);
	// 82662978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266297C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662988 size=108
    let mut pc: u32 = 0x82662988;
    'dispatch: loop {
        match pc {
            0x82662988 => {
    //   block [0x82662988..0x826629F4)
	// 82662988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266298C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662994: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662998: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266299C: 38EB56B0  addi r7, r11, 0x56b0
	ctx.r[7].s64 = ctx.r[11].s64 + 22192;
	// 826629A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826629A4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826629A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826629AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826629B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826629B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826629B8: 386ABD40  addi r3, r10, -0x42c0
	ctx.r[3].s64 = ctx.r[10].s64 + -17088;
	// 826629BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826629C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826629C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826629C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826629CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826629D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826629D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826629D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826629DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826629E0: 4BE04441  bl 0x82466e20
	ctx.lr = 0x826629E4;
	sub_82466E20(ctx, base);
	// 826629E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826629E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826629EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826629F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826629F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826629F8 size=108
    let mut pc: u32 = 0x826629F8;
    'dispatch: loop {
        match pc {
            0x826629F8 => {
    //   block [0x826629F8..0x82662A64)
	// 826629F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826629FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662A04: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662A08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662A0C: 38EB56E0  addi r7, r11, 0x56e0
	ctx.r[7].s64 = ctx.r[11].s64 + 22240;
	// 82662A10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662A14: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82662A18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662A1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662A20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662A28: 386ABD70  addi r3, r10, -0x4290
	ctx.r[3].s64 = ctx.r[10].s64 + -17040;
	// 82662A2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662A30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662A50: 4BE043D1  bl 0x82466e20
	ctx.lr = 0x82662A54;
	sub_82466E20(ctx, base);
	// 82662A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662A60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662A68 size=112
    let mut pc: u32 = 0x82662A68;
    'dispatch: loop {
        match pc {
            0x82662A68 => {
    //   block [0x82662A68..0x82662AD8)
	// 82662A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662A74: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82662A78: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662A7C: 392AF020  addi r9, r10, -0xfe0
	ctx.r[9].s64 = ctx.r[10].s64 + -4064;
	// 82662A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662A84: 390B5718  addi r8, r11, 0x5718
	ctx.r[8].s64 = ctx.r[11].s64 + 22296;
	// 82662A88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82662A8C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 82662A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662A94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662AA0: 386ABDA0  addi r3, r10, -0x4260
	ctx.r[3].s64 = ctx.r[10].s64 + -16992;
	// 82662AA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662AA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82662AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662AC4: 4BE0435D  bl 0x82466e20
	ctx.lr = 0x82662AC8;
	sub_82466E20(ctx, base);
	// 82662AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662AD8 size=108
    let mut pc: u32 = 0x82662AD8;
    'dispatch: loop {
        match pc {
            0x82662AD8 => {
    //   block [0x82662AD8..0x82662B44)
	// 82662AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662AE4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662AE8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662AEC: 38EB5790  addi r7, r11, 0x5790
	ctx.r[7].s64 = ctx.r[11].s64 + 22416;
	// 82662AF0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82662AF4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82662AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662AFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662B08: 386ABDD0  addi r3, r10, -0x4230
	ctx.r[3].s64 = ctx.r[10].s64 + -16944;
	// 82662B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662B30: 4BE042F1  bl 0x82466e20
	ctx.lr = 0x82662B34;
	sub_82466E20(ctx, base);
	// 82662B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662B48 size=108
    let mut pc: u32 = 0x82662B48;
    'dispatch: loop {
        match pc {
            0x82662B48 => {
    //   block [0x82662B48..0x82662BB4)
	// 82662B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662B54: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662B58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82662B5C: 38EB5898  addi r7, r11, 0x5898
	ctx.r[7].s64 = ctx.r[11].s64 + 22680;
	// 82662B60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82662B64: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82662B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662B6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662B70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662B78: 386ABE00  addi r3, r10, -0x4200
	ctx.r[3].s64 = ctx.r[10].s64 + -16896;
	// 82662B7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662B80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662B88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662B90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662B94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662B98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662B9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662BA0: 4BE04281  bl 0x82466e20
	ctx.lr = 0x82662BA4;
	sub_82466E20(ctx, base);
	// 82662BA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662BA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662BAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662BB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662BB8 size=108
    let mut pc: u32 = 0x82662BB8;
    'dispatch: loop {
        match pc {
            0x82662BB8 => {
    //   block [0x82662BB8..0x82662C24)
	// 82662BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662BC4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662BC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662BCC: 38EB58B0  addi r7, r11, 0x58b0
	ctx.r[7].s64 = ctx.r[11].s64 + 22704;
	// 82662BD0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82662BD4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82662BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662BDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662BE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662BE8: 386ABE30  addi r3, r10, -0x41d0
	ctx.r[3].s64 = ctx.r[10].s64 + -16848;
	// 82662BEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662BF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662C10: 4BE04211  bl 0x82466e20
	ctx.lr = 0x82662C14;
	sub_82466E20(ctx, base);
	// 82662C14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82662C28 size=24
    let mut pc: u32 = 0x82662C28;
    'dispatch: loop {
        match pc {
            0x82662C28 => {
    //   block [0x82662C28..0x82662C40)
	// 82662C28: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662C2C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82662C30: 394ABD98  addi r10, r10, -0x4268
	ctx.r[10].s64 = ctx.r[10].s64 + -17000;
	// 82662C34: 816B5714  lwz r11, 0x5714(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22292 as u32) ) } as u64;
	// 82662C38: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82662C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662C40 size=108
    let mut pc: u32 = 0x82662C40;
    'dispatch: loop {
        match pc {
            0x82662C40 => {
    //   block [0x82662C40..0x82662CAC)
	// 82662C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662C4C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82662C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662C54: 38EBBD98  addi r7, r11, -0x4268
	ctx.r[7].s64 = ctx.r[11].s64 + -17000;
	// 82662C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662C5C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82662C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662C64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662C70: 386ABE60  addi r3, r10, -0x41a0
	ctx.r[3].s64 = ctx.r[10].s64 + -16800;
	// 82662C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662C98: 4BE04189  bl 0x82466e20
	ctx.lr = 0x82662C9C;
	sub_82466E20(ctx, base);
	// 82662C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82662CB0 size=24
    let mut pc: u32 = 0x82662CB0;
    'dispatch: loop {
        match pc {
            0x82662CB0 => {
    //   block [0x82662CB0..0x82662CC8)
	// 82662CB0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662CB4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82662CB8: 394ABDC8  addi r10, r10, -0x4238
	ctx.r[10].s64 = ctx.r[10].s64 + -16952;
	// 82662CBC: 816B5714  lwz r11, 0x5714(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22292 as u32) ) } as u64;
	// 82662CC0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82662CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662CC8 size=108
    let mut pc: u32 = 0x82662CC8;
    'dispatch: loop {
        match pc {
            0x82662CC8 => {
    //   block [0x82662CC8..0x82662D34)
	// 82662CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662CD4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82662CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662CDC: 38EBBDC8  addi r7, r11, -0x4238
	ctx.r[7].s64 = ctx.r[11].s64 + -16952;
	// 82662CE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662CE4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82662CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662CF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662CF8: 386ABE90  addi r3, r10, -0x4170
	ctx.r[3].s64 = ctx.r[10].s64 + -16752;
	// 82662CFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662D20: 4BE04101  bl 0x82466e20
	ctx.lr = 0x82662D24;
	sub_82466E20(ctx, base);
	// 82662D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662D38 size=108
    let mut pc: u32 = 0x82662D38;
    'dispatch: loop {
        match pc {
            0x82662D38 => {
    //   block [0x82662D38..0x82662DA4)
	// 82662D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662D44: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662D4C: 38EB5928  addi r7, r11, 0x5928
	ctx.r[7].s64 = ctx.r[11].s64 + 22824;
	// 82662D50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82662D54: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82662D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662D5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662D60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662D68: 386ABEC0  addi r3, r10, -0x4140
	ctx.r[3].s64 = ctx.r[10].s64 + -16704;
	// 82662D6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662D90: 4BE04091  bl 0x82466e20
	ctx.lr = 0x82662D94;
	sub_82466E20(ctx, base);
	// 82662D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82662DA8 size=24
    let mut pc: u32 = 0x82662DA8;
    'dispatch: loop {
        match pc {
            0x82662DA8 => {
    //   block [0x82662DA8..0x82662DC0)
	// 82662DA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662DAC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82662DB0: 394ABDF8  addi r10, r10, -0x4208
	ctx.r[10].s64 = ctx.r[10].s64 + -16904;
	// 82662DB4: 816B5714  lwz r11, 0x5714(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22292 as u32) ) } as u64;
	// 82662DB8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82662DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662DC0 size=108
    let mut pc: u32 = 0x82662DC0;
    'dispatch: loop {
        match pc {
            0x82662DC0 => {
    //   block [0x82662DC0..0x82662E2C)
	// 82662DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662DCC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82662DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662DD4: 38EBBDF8  addi r7, r11, -0x4208
	ctx.r[7].s64 = ctx.r[11].s64 + -16904;
	// 82662DD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662DDC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82662DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662DE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662DF0: 386ABEF0  addi r3, r10, -0x4110
	ctx.r[3].s64 = ctx.r[10].s64 + -16656;
	// 82662DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662E18: 4BE04009  bl 0x82466e20
	ctx.lr = 0x82662E1C;
	sub_82466E20(ctx, base);
	// 82662E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662E30 size=112
    let mut pc: u32 = 0x82662E30;
    'dispatch: loop {
        match pc {
            0x82662E30 => {
    //   block [0x82662E30..0x82662EA0)
	// 82662E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662E3C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82662E40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662E44: 392AF064  addi r9, r10, -0xf9c
	ctx.r[9].s64 = ctx.r[10].s64 + -3996;
	// 82662E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662E4C: 390B5940  addi r8, r11, 0x5940
	ctx.r[8].s64 = ctx.r[11].s64 + 22848;
	// 82662E50: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82662E54: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82662E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662E5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82662E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662E68: 386ABF20  addi r3, r10, -0x40e0
	ctx.r[3].s64 = ctx.r[10].s64 + -16608;
	// 82662E6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82662E70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82662E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662E84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662E8C: 4BE03F95  bl 0x82466e20
	ctx.lr = 0x82662E90;
	sub_82466E20(ctx, base);
	// 82662E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662EA0 size=108
    let mut pc: u32 = 0x82662EA0;
    'dispatch: loop {
        match pc {
            0x82662EA0 => {
    //   block [0x82662EA0..0x82662F0C)
	// 82662EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662EAC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662EB4: 38EB5970  addi r7, r11, 0x5970
	ctx.r[7].s64 = ctx.r[11].s64 + 22896;
	// 82662EB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662EBC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82662EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662EC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662ED0: 386ABF50  addi r3, r10, -0x40b0
	ctx.r[3].s64 = ctx.r[10].s64 + -16560;
	// 82662ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662EF8: 4BE03F29  bl 0x82466e20
	ctx.lr = 0x82662EFC;
	sub_82466E20(ctx, base);
	// 82662EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662F10 size=108
    let mut pc: u32 = 0x82662F10;
    'dispatch: loop {
        match pc {
            0x82662F10 => {
    //   block [0x82662F10..0x82662F7C)
	// 82662F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662F1C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662F24: 38EB59A0  addi r7, r11, 0x59a0
	ctx.r[7].s64 = ctx.r[11].s64 + 22944;
	// 82662F28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82662F2C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 82662F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662F34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662F38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662F40: 386ABF80  addi r3, r10, -0x4080
	ctx.r[3].s64 = ctx.r[10].s64 + -16512;
	// 82662F44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662F68: 4BE03EB9  bl 0x82466e20
	ctx.lr = 0x82662F6C;
	sub_82466E20(ctx, base);
	// 82662F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662F80 size=108
    let mut pc: u32 = 0x82662F80;
    'dispatch: loop {
        match pc {
            0x82662F80 => {
    //   block [0x82662F80..0x82662FEC)
	// 82662F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662F8C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82662F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82662F94: 38EB59B8  addi r7, r11, 0x59b8
	ctx.r[7].s64 = ctx.r[11].s64 + 22968;
	// 82662F98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82662F9C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 82662FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82662FA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82662FA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82662FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82662FB0: 386ABFB0  addi r3, r10, -0x4050
	ctx.r[3].s64 = ctx.r[10].s64 + -16464;
	// 82662FB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82662FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82662FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82662FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82662FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82662FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82662FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82662FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82662FD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82662FD8: 4BE03E49  bl 0x82466e20
	ctx.lr = 0x82662FDC;
	sub_82466E20(ctx, base);
	// 82662FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82662FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82662FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82662FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82662FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82662FF0 size=112
    let mut pc: u32 = 0x82662FF0;
    'dispatch: loop {
        match pc {
            0x82662FF0 => {
    //   block [0x82662FF0..0x82663060)
	// 82662FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82662FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82662FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82662FFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663000: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663004: 38AAC010  addi r5, r10, -0x3ff0
	ctx.r[5].s64 = ctx.r[10].s64 + -16368;
	// 82663008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266300C: 390B59E8  addi r8, r11, 0x59e8
	ctx.r[8].s64 = ctx.r[11].s64 + 23016;
	// 82663010: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82663014: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82663018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266301C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82663024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663028: 386ABFE0  addi r3, r10, -0x4020
	ctx.r[3].s64 = ctx.r[10].s64 + -16416;
	// 8266302C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82663030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266303C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266304C: 4BE03DD5  bl 0x82466e20
	ctx.lr = 0x82663050;
	sub_82466E20(ctx, base);
	// 82663050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266305C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663060 size=108
    let mut pc: u32 = 0x82663060;
    'dispatch: loop {
        match pc {
            0x82663060 => {
    //   block [0x82663060..0x826630CC)
	// 82663060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266306C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663074: 38EB5A00  addi r7, r11, 0x5a00
	ctx.r[7].s64 = ctx.r[11].s64 + 23040;
	// 82663078: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266307C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82663080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663084: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266308C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663090: 386AC010  addi r3, r10, -0x3ff0
	ctx.r[3].s64 = ctx.r[10].s64 + -16368;
	// 82663094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266309C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826630A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826630A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826630A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826630AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826630B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826630B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826630B8: 4BE03D69  bl 0x82466e20
	ctx.lr = 0x826630BC;
	sub_82466E20(ctx, base);
	// 826630BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826630C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826630C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826630C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826630D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826630D0 size=108
    let mut pc: u32 = 0x826630D0;
    'dispatch: loop {
        match pc {
            0x826630D0 => {
    //   block [0x826630D0..0x8266313C)
	// 826630D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826630D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826630D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826630DC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826630E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826630E4: 38EB5A30  addi r7, r11, 0x5a30
	ctx.r[7].s64 = ctx.r[11].s64 + 23088;
	// 826630E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826630EC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826630F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826630F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826630F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826630FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663100: 386AC040  addi r3, r10, -0x3fc0
	ctx.r[3].s64 = ctx.r[10].s64 + -16320;
	// 82663104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266310C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266311C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663128: 4BE03CF9  bl 0x82466e20
	ctx.lr = 0x8266312C;
	sub_82466E20(ctx, base);
	// 8266312C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663140 size=108
    let mut pc: u32 = 0x82663140;
    'dispatch: loop {
        match pc {
            0x82663140 => {
    //   block [0x82663140..0x826631AC)
	// 82663140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266314C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663154: 38EB5A48  addi r7, r11, 0x5a48
	ctx.r[7].s64 = ctx.r[11].s64 + 23112;
	// 82663158: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266315C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82663160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663164: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266316C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663170: 386AC070  addi r3, r10, -0x3f90
	ctx.r[3].s64 = ctx.r[10].s64 + -16272;
	// 82663174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266317C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266318C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663198: 4BE03C89  bl 0x82466e20
	ctx.lr = 0x8266319C;
	sub_82466E20(ctx, base);
	// 8266319C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826631A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826631A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826631A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826631B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826631B0 size=108
    let mut pc: u32 = 0x826631B0;
    'dispatch: loop {
        match pc {
            0x826631B0 => {
    //   block [0x826631B0..0x8266321C)
	// 826631B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826631B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826631B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826631BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826631C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826631C4: 38EB5A78  addi r7, r11, 0x5a78
	ctx.r[7].s64 = ctx.r[11].s64 + 23160;
	// 826631C8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826631CC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826631D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826631D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826631D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826631DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826631E0: 386AC0A0  addi r3, r10, -0x3f60
	ctx.r[3].s64 = ctx.r[10].s64 + -16224;
	// 826631E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826631E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826631EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826631F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826631F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826631F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826631FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663208: 4BE03C19  bl 0x82466e20
	ctx.lr = 0x8266320C;
	sub_82466E20(ctx, base);
	// 8266320C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663220 size=108
    let mut pc: u32 = 0x82663220;
    'dispatch: loop {
        match pc {
            0x82663220 => {
    //   block [0x82663220..0x8266328C)
	// 82663220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266322C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663234: 38EB5B20  addi r7, r11, 0x5b20
	ctx.r[7].s64 = ctx.r[11].s64 + 23328;
	// 82663238: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266323C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82663240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663244: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266324C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663250: 386AC0D0  addi r3, r10, -0x3f30
	ctx.r[3].s64 = ctx.r[10].s64 + -16176;
	// 82663254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266325C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266326C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663278: 4BE03BA9  bl 0x82466e20
	ctx.lr = 0x8266327C;
	sub_82466E20(ctx, base);
	// 8266327C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663290 size=108
    let mut pc: u32 = 0x82663290;
    'dispatch: loop {
        match pc {
            0x82663290 => {
    //   block [0x82663290..0x826632FC)
	// 82663290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266329C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826632A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826632A4: 38EB5B50  addi r7, r11, 0x5b50
	ctx.r[7].s64 = ctx.r[11].s64 + 23376;
	// 826632A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826632AC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826632B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826632B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826632B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826632BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826632C0: 386AC100  addi r3, r10, -0x3f00
	ctx.r[3].s64 = ctx.r[10].s64 + -16128;
	// 826632C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826632C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826632CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826632D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826632D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826632D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826632DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826632E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826632E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826632E8: 4BE03B39  bl 0x82466e20
	ctx.lr = 0x826632EC;
	sub_82466E20(ctx, base);
	// 826632EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826632F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826632F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826632F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663300 size=108
    let mut pc: u32 = 0x82663300;
    'dispatch: loop {
        match pc {
            0x82663300 => {
    //   block [0x82663300..0x8266336C)
	// 82663300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266330C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663314: 38EB5B68  addi r7, r11, 0x5b68
	ctx.r[7].s64 = ctx.r[11].s64 + 23400;
	// 82663318: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266331C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82663320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266332C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663330: 386AC130  addi r3, r10, -0x3ed0
	ctx.r[3].s64 = ctx.r[10].s64 + -16080;
	// 82663334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266333C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266334C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663358: 4BE03AC9  bl 0x82466e20
	ctx.lr = 0x8266335C;
	sub_82466E20(ctx, base);
	// 8266335C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663370 size=112
    let mut pc: u32 = 0x82663370;
    'dispatch: loop {
        match pc {
            0x82663370 => {
    //   block [0x82663370..0x826633E0)
	// 82663370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82663374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266337C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663380: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663384: 38AABF80  addi r5, r10, -0x4080
	ctx.r[5].s64 = ctx.r[10].s64 + -16512;
	// 82663388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266338C: 390B5B98  addi r8, r11, 0x5b98
	ctx.r[8].s64 = ctx.r[11].s64 + 23448;
	// 82663390: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82663394: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82663398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266339C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826633A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826633A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826633A8: 386AC160  addi r3, r10, -0x3ea0
	ctx.r[3].s64 = ctx.r[10].s64 + -16032;
	// 826633AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826633B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826633B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826633B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826633BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826633C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826633C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826633C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826633CC: 4BE03A55  bl 0x82466e20
	ctx.lr = 0x826633D0;
	sub_82466E20(ctx, base);
	// 826633D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826633D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826633D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826633DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826633E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826633E0 size=24
    let mut pc: u32 = 0x826633E0;
    'dispatch: loop {
        match pc {
            0x826633E0 => {
    //   block [0x826633E0..0x826633F8)
	// 826633E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826633E4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826633E8: 394ABE28  addi r10, r10, -0x41d8
	ctx.r[10].s64 = ctx.r[10].s64 + -16856;
	// 826633EC: 816B5C40  lwz r11, 0x5c40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23616 as u32) ) } as u64;
	// 826633F0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826633F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826633F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826633F8 size=112
    let mut pc: u32 = 0x826633F8;
    'dispatch: loop {
        match pc {
            0x826633F8 => {
    //   block [0x826633F8..0x82663468)
	// 826633F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826633FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663404: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82663408: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266340C: 392AF090  addi r9, r10, -0xf70
	ctx.r[9].s64 = ctx.r[10].s64 + -3952;
	// 82663410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663414: 390BBE28  addi r8, r11, -0x41d8
	ctx.r[8].s64 = ctx.r[11].s64 + -16856;
	// 82663418: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8266341C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82663420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663424: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266342C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663430: 386AC190  addi r3, r10, -0x3e70
	ctx.r[3].s64 = ctx.r[10].s64 + -15984;
	// 82663434: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82663438: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266343C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82663444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266344C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82663454: 4BE039CD  bl 0x82466e20
	ctx.lr = 0x82663458;
	sub_82466E20(ctx, base);
	// 82663458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266345C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663468 size=108
    let mut pc: u32 = 0x82663468;
    'dispatch: loop {
        match pc {
            0x82663468 => {
    //   block [0x82663468..0x826634D4)
	// 82663468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266346C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663474: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266347C: 38EB5C48  addi r7, r11, 0x5c48
	ctx.r[7].s64 = ctx.r[11].s64 + 23624;
	// 82663480: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82663484: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82663488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266348C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663490: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663498: 386AC1C0  addi r3, r10, -0x3e40
	ctx.r[3].s64 = ctx.r[10].s64 + -15936;
	// 8266349C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826634A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826634A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826634A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826634AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826634B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826634B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826634B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826634BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826634C0: 4BE03961  bl 0x82466e20
	ctx.lr = 0x826634C4;
	sub_82466E20(ctx, base);
	// 826634C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826634C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826634CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826634D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826634D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826634D8 size=116
    let mut pc: u32 = 0x826634D8;
    'dispatch: loop {
        match pc {
            0x826634D8 => {
    //   block [0x826634D8..0x8266354C)
	// 826634D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826634DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826634E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826634E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826634E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826634EC: 390B5C78  addi r8, r11, 0x5c78
	ctx.r[8].s64 = ctx.r[11].s64 + 23672;
	// 826634F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826634F4: 392AF0D4  addi r9, r10, -0xf2c
	ctx.r[9].s64 = ctx.r[10].s64 + -3884;
	// 826634F8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826634FC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82663500: 38AABF80  addi r5, r10, -0x4080
	ctx.r[5].s64 = ctx.r[10].s64 + -16512;
	// 82663504: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82663508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266350C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266351C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82663520: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82663524: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82663528: 386BC1F0  addi r3, r11, -0x3e10
	ctx.r[3].s64 = ctx.r[11].s64 + -15888;
	// 8266352C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82663530: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663538: 4BE038E9  bl 0x82466e20
	ctx.lr = 0x8266353C;
	sub_82466E20(ctx, base);
	// 8266353C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82663550 size=24
    let mut pc: u32 = 0x82663550;
    'dispatch: loop {
        match pc {
            0x82663550 => {
    //   block [0x82663550..0x82663568)
	// 82663550: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663554: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82663558: 394ABEA0  addi r10, r10, -0x4160
	ctx.r[10].s64 = ctx.r[10].s64 + -16736;
	// 8266355C: 816B5D38  lwz r11, 0x5d38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23864 as u32) ) } as u64;
	// 82663560: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82663564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663568 size=112
    let mut pc: u32 = 0x82663568;
    'dispatch: loop {
        match pc {
            0x82663568 => {
    //   block [0x82663568..0x826635D8)
	// 82663568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266356C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663574: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82663578: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266357C: 392AF110  addi r9, r10, -0xef0
	ctx.r[9].s64 = ctx.r[10].s64 + -3824;
	// 82663580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82663584: 390BBEA0  addi r8, r11, -0x4160
	ctx.r[8].s64 = ctx.r[11].s64 + -16736;
	// 82663588: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8266358C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82663590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82663594: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266359C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826635A0: 386AC220  addi r3, r10, -0x3de0
	ctx.r[3].s64 = ctx.r[10].s64 + -15840;
	// 826635A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826635A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826635AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826635B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826635B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826635B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826635BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826635C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826635C4: 4BE0385D  bl 0x82466e20
	ctx.lr = 0x826635C8;
	sub_82466E20(ctx, base);
	// 826635C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826635CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826635D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826635D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826635D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826635D8 size=108
    let mut pc: u32 = 0x826635D8;
    'dispatch: loop {
        match pc {
            0x826635D8 => {
    //   block [0x826635D8..0x82663644)
	// 826635D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826635DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826635E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826635E4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826635E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826635EC: 38EB5D3C  addi r7, r11, 0x5d3c
	ctx.r[7].s64 = ctx.r[11].s64 + 23868;
	// 826635F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826635F4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826635F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826635FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663608: 386AC250  addi r3, r10, -0x3db0
	ctx.r[3].s64 = ctx.r[10].s64 + -15792;
	// 8266360C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266361C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266362C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663630: 4BE037F1  bl 0x82466e20
	ctx.lr = 0x82663634;
	sub_82466E20(ctx, base);
	// 82663634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266363C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82663640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82663648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82663648 size=108
    let mut pc: u32 = 0x82663648;
    'dispatch: loop {
        match pc {
            0x82663648 => {
    //   block [0x82663648..0x826636B4)
	// 82663648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266364C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82663650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82663654: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82663658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266365C: 38EB5D54  addi r7, r11, 0x5d54
	ctx.r[7].s64 = ctx.r[11].s64 + 23892;
	// 82663660: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82663664: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82663668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266366C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82663674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82663678: 386AC280  addi r3, r10, -0x3d80
	ctx.r[3].s64 = ctx.r[10].s64 + -15744;
	// 8266367C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82663680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82663684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266368C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266369C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826636A0: 4BE03781  bl 0x82466e20
	ctx.lr = 0x826636A4;
	sub_82466E20(ctx, base);
	// 826636A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826636A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826636AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826636B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826636B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826636B8 size=24
    let mut pc: u32 = 0x826636B8;
    'dispatch: loop {
        match pc {
            0x826636B8 => {
    //   block [0x826636B8..0x826636D0)
	// 826636B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826636BC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826636C0: 394ABEE8  addi r10, r10, -0x4118
	ctx.r[10].s64 = ctx.r[10].s64 + -16664;
	// 826636C4: 816B5D84  lwz r11, 0x5d84(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23940 as u32) ) } as u64;
	// 826636C8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826636CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826636D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826636D0 size=112
    let mut pc: u32 = 0x826636D0;
    'dispatch: loop {
        match pc {
            0x826636D0 => {
    //   block [0x826636D0..0x82663740)
	// 826636D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826636D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826636D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826636DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826636E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826636E4: 392AF14C  addi r9, r10, -0xeb4
	ctx.r[9].s64 = ctx.r[10].s64 + -3764;
	// 826636E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826636EC: 390BBEE8  addi r8, r11, -0x4118
	ctx.r[8].s64 = ctx.r[11].s64 + -16664;
	// 826636F0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826636F4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826636F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826636FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82663700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82663704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82663708: 386AC2B0  addi r3, r10, -0x3d50
	ctx.r[3].s64 = ctx.r[10].s64 + -15696;
	// 8266370C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82663710: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82663714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82663718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266371C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82663720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82663724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82663728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266372C: 4BE036F5  bl 0x82466e20
	ctx.lr = 0x82663730;
	sub_82466E20(ctx, base);
	// 82663730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82663734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82663738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266373C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


