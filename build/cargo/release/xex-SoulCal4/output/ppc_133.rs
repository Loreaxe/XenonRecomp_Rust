pub fn sub_8270AA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AA00 size=108
    let mut pc: u32 = 0x8270AA00;
    'dispatch: loop {
        match pc {
            0x8270AA00 => {
    //   block [0x8270AA00..0x8270AA6C)
	// 8270AA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AA0C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AA10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AA14: 38EB64A8  addi r7, r11, 0x64a8
	ctx.r[7].s64 = ctx.r[11].s64 + 25768;
	// 8270AA18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8270AA1C: 388ACDFC  addi r4, r10, -0x3204
	ctx.r[4].s64 = ctx.r[10].s64 + -12804;
	// 8270AA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AA24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AA28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270AA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AA30: 386A2964  addi r3, r10, 0x2964
	ctx.r[3].s64 = ctx.r[10].s64 + 10596;
	// 8270AA34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270AA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AA3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AA4C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8270AA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270AA54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270AA58: 4BD5C3C9  bl 0x82466e20
	ctx.lr = 0x8270AA5C;
	sub_82466E20(ctx, base);
	// 8270AA5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270AA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AA70 size=112
    let mut pc: u32 = 0x8270AA70;
    'dispatch: loop {
        match pc {
            0x8270AA70 => {
    //   block [0x8270AA70..0x8270AAE0)
	// 8270AA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AA7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AA80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AA84: 38AA2904  addi r5, r10, 0x2904
	ctx.r[5].s64 = ctx.r[10].s64 + 10500;
	// 8270AA88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AA8C: 390B64F0  addi r8, r11, 0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + 25840;
	// 8270AA90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270AA94: 388ACE38  addi r4, r10, -0x31c8
	ctx.r[4].s64 = ctx.r[10].s64 + -12744;
	// 8270AA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AA9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AAA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270AAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AAA8: 386A2994  addi r3, r10, 0x2994
	ctx.r[3].s64 = ctx.r[10].s64 + 10644;
	// 8270AAAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270AAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AAC4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8270AAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270AACC: 4BD5C355  bl 0x82466e20
	ctx.lr = 0x8270AAD0;
	sub_82466E20(ctx, base);
	// 8270AAD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270AAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AAE0 size=108
    let mut pc: u32 = 0x8270AAE0;
    'dispatch: loop {
        match pc {
            0x8270AAE0 => {
    //   block [0x8270AAE0..0x8270AB4C)
	// 8270AAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AAEC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AAF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AAF4: 38EB6520  addi r7, r11, 0x6520
	ctx.r[7].s64 = ctx.r[11].s64 + 25888;
	// 8270AAF8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8270AAFC: 388ACC04  addi r4, r10, -0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + -13308;
	// 8270AB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AB04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AB08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270AB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AB10: 386A29C4  addi r3, r10, 0x29c4
	ctx.r[3].s64 = ctx.r[10].s64 + 10692;
	// 8270AB14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270AB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AB2C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8270AB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270AB34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270AB38: 4BD5C2E9  bl 0x82466e20
	ctx.lr = 0x8270AB3C;
	sub_82466E20(ctx, base);
	// 8270AB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270AB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AB50 size=112
    let mut pc: u32 = 0x8270AB50;
    'dispatch: loop {
        match pc {
            0x8270AB50 => {
    //   block [0x8270AB50..0x8270ABC0)
	// 8270AB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AB5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AB60: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AB64: 38AA2C34  addi r5, r10, 0x2c34
	ctx.r[5].s64 = ctx.r[10].s64 + 11316;
	// 8270AB68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AB6C: 390B6568  addi r8, r11, 0x6568
	ctx.r[8].s64 = ctx.r[11].s64 + 25960;
	// 8270AB70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270AB74: 388ACC34  addi r4, r10, -0x33cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13260;
	// 8270AB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AB7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AB80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270AB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AB88: 386A29F4  addi r3, r10, 0x29f4
	ctx.r[3].s64 = ctx.r[10].s64 + 10740;
	// 8270AB8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270AB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270ABA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270ABA4: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8270ABA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270ABAC: 4BD5C275  bl 0x82466e20
	ctx.lr = 0x8270ABB0;
	sub_82466E20(ctx, base);
	// 8270ABB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270ABB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270ABB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270ABBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270ABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270ABC0 size=108
    let mut pc: u32 = 0x8270ABC0;
    'dispatch: loop {
        match pc {
            0x8270ABC0 => {
    //   block [0x8270ABC0..0x8270AC2C)
	// 8270ABC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270ABC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270ABC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270ABCC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270ABD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270ABD4: 38EB65B0  addi r7, r11, 0x65b0
	ctx.r[7].s64 = ctx.r[11].s64 + 26032;
	// 8270ABD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270ABDC: 388ACE8C  addi r4, r10, -0x3174
	ctx.r[4].s64 = ctx.r[10].s64 + -12660;
	// 8270ABE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270ABE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270ABE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270ABEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270ABF0: 386A2A24  addi r3, r10, 0x2a24
	ctx.r[3].s64 = ctx.r[10].s64 + 10788;
	// 8270ABF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270ABF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270ABFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AC0C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8270AC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270AC14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270AC18: 4BD5C209  bl 0x82466e20
	ctx.lr = 0x8270AC1C;
	sub_82466E20(ctx, base);
	// 8270AC1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AC20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AC24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AC30 size=112
    let mut pc: u32 = 0x8270AC30;
    'dispatch: loop {
        match pc {
            0x8270AC30 => {
    //   block [0x8270AC30..0x8270ACA0)
	// 8270AC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AC3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270AC40: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AC44: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270AC48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AC4C: 390B65E0  addi r8, r11, 0x65e0
	ctx.r[8].s64 = ctx.r[11].s64 + 26080;
	// 8270AC50: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270AC54: 388ACEA0  addi r4, r10, -0x3160
	ctx.r[4].s64 = ctx.r[10].s64 + -12640;
	// 8270AC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AC5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AC60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270AC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AC68: 386A2A54  addi r3, r10, 0x2a54
	ctx.r[3].s64 = ctx.r[10].s64 + 10836;
	// 8270AC6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270AC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AC84: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8270AC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270AC8C: 4BD5C195  bl 0x82466e20
	ctx.lr = 0x8270AC90;
	sub_82466E20(ctx, base);
	// 8270AC90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270ACA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270ACA0 size=112
    let mut pc: u32 = 0x8270ACA0;
    'dispatch: loop {
        match pc {
            0x8270ACA0 => {
    //   block [0x8270ACA0..0x8270AD10)
	// 8270ACA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270ACA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270ACA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270ACAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270ACB0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270ACB4: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270ACB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8270ACBC: 390B6628  addi r8, r11, 0x6628
	ctx.r[8].s64 = ctx.r[11].s64 + 26152;
	// 8270ACC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270ACC4: 388A88D0  addi r4, r10, -0x7730
	ctx.r[4].s64 = ctx.r[10].s64 + -30512;
	// 8270ACC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270ACCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270ACD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270ACD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270ACD8: 386A2A84  addi r3, r10, 0x2a84
	ctx.r[3].s64 = ctx.r[10].s64 + 10884;
	// 8270ACDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270ACE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270ACE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270ACE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270ACEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270ACF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270ACF4: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8270ACF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270ACFC: 4BD5C125  bl 0x82466e20
	ctx.lr = 0x8270AD00;
	sub_82466E20(ctx, base);
	// 8270AD00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AD04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AD08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270AD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AD10 size=112
    let mut pc: u32 = 0x8270AD10;
    'dispatch: loop {
        match pc {
            0x8270AD10 => {
    //   block [0x8270AD10..0x8270AD80)
	// 8270AD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AD1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AD20: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AD24: 38AA2C04  addi r5, r10, 0x2c04
	ctx.r[5].s64 = ctx.r[10].s64 + 11268;
	// 8270AD28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AD2C: 390B6698  addi r8, r11, 0x6698
	ctx.r[8].s64 = ctx.r[11].s64 + 26264;
	// 8270AD30: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8270AD34: 388ACCDC  addi r4, r10, -0x3324
	ctx.r[4].s64 = ctx.r[10].s64 + -13092;
	// 8270AD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AD3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AD40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270AD44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AD48: 386A2AB4  addi r3, r10, 0x2ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 10932;
	// 8270AD4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270AD50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AD54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AD58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AD5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AD60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AD64: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8270AD68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270AD6C: 4BD5C0B5  bl 0x82466e20
	ctx.lr = 0x8270AD70;
	sub_82466E20(ctx, base);
	// 8270AD70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270AD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AD80 size=108
    let mut pc: u32 = 0x8270AD80;
    'dispatch: loop {
        match pc {
            0x8270AD80 => {
    //   block [0x8270AD80..0x8270ADEC)
	// 8270AD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AD8C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AD90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AD94: 38EB67A0  addi r7, r11, 0x67a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26528;
	// 8270AD98: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8270AD9C: 388ACB8C  addi r4, r10, -0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + -13428;
	// 8270ADA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270ADA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270ADA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270ADAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270ADB0: 386A2AE4  addi r3, r10, 0x2ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 10980;
	// 8270ADB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270ADB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270ADBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270ADC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270ADC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270ADC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270ADCC: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 8270ADD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270ADD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270ADD8: 4BD5C049  bl 0x82466e20
	ctx.lr = 0x8270ADDC;
	sub_82466E20(ctx, base);
	// 8270ADDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270ADE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270ADE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270ADE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270ADF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270ADF0 size=112
    let mut pc: u32 = 0x8270ADF0;
    'dispatch: loop {
        match pc {
            0x8270ADF0 => {
    //   block [0x8270ADF0..0x8270AE60)
	// 8270ADF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270ADF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270ADF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270ADFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AE00: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AE04: 38AA06B4  addi r5, r10, 0x6b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1716;
	// 8270AE08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AE0C: 390B6938  addi r8, r11, 0x6938
	ctx.r[8].s64 = ctx.r[11].s64 + 26936;
	// 8270AE10: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8270AE14: 388ACBA8  addi r4, r10, -0x3458
	ctx.r[4].s64 = ctx.r[10].s64 + -13400;
	// 8270AE18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AE1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AE20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270AE24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AE28: 386A2B14  addi r3, r10, 0x2b14
	ctx.r[3].s64 = ctx.r[10].s64 + 11028;
	// 8270AE2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270AE30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AE44: 38C000D4  li r6, 0xd4
	ctx.r[6].s64 = 212;
	// 8270AE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270AE4C: 4BD5BFD5  bl 0x82466e20
	ctx.lr = 0x8270AE50;
	sub_82466E20(ctx, base);
	// 8270AE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270AE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AE60 size=100
    let mut pc: u32 = 0x8270AE60;
    'dispatch: loop {
        match pc {
            0x8270AE60 => {
    //   block [0x8270AE60..0x8270AEC4)
	// 8270AE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AE6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270AE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AE74: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270AE78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AE80: 388ACE54  addi r4, r10, -0x31ac
	ctx.r[4].s64 = ctx.r[10].s64 + -12716;
	// 8270AE84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AE94: 386A2B44  addi r3, r10, 0x2b44
	ctx.r[3].s64 = ctx.r[10].s64 + 11076;
	// 8270AE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270AE9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AEA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270AEA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AEA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270AEAC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270AEB0: 4BD5BF71  bl 0x82466e20
	ctx.lr = 0x8270AEB4;
	sub_82466E20(ctx, base);
	// 8270AEB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AEB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AEBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270AEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AEC8 size=112
    let mut pc: u32 = 0x8270AEC8;
    'dispatch: loop {
        match pc {
            0x8270AEC8 => {
    //   block [0x8270AEC8..0x8270AF38)
	// 8270AEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AED4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AED8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AEDC: 38AA2C94  addi r5, r10, 0x2c94
	ctx.r[5].s64 = ctx.r[10].s64 + 11412;
	// 8270AEE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AEE4: 390B6BD0  addi r8, r11, 0x6bd0
	ctx.r[8].s64 = ctx.r[11].s64 + 27600;
	// 8270AEE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270AEEC: 388ACED0  addi r4, r10, -0x3130
	ctx.r[4].s64 = ctx.r[10].s64 + -12592;
	// 8270AEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AEF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AEF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270AEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AF00: 386A2B74  addi r3, r10, 0x2b74
	ctx.r[3].s64 = ctx.r[10].s64 + 11124;
	// 8270AF04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270AF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AF1C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8270AF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270AF24: 4BD5BEFD  bl 0x82466e20
	ctx.lr = 0x8270AF28;
	sub_82466E20(ctx, base);
	// 8270AF28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270AF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AF38 size=112
    let mut pc: u32 = 0x8270AF38;
    'dispatch: loop {
        match pc {
            0x8270AF38 => {
    //   block [0x8270AF38..0x8270AFA8)
	// 8270AF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AF40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AF44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AF48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AF4C: 38AA2CF4  addi r5, r10, 0x2cf4
	ctx.r[5].s64 = ctx.r[10].s64 + 11508;
	// 8270AF50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AF54: 390B6C30  addi r8, r11, 0x6c30
	ctx.r[8].s64 = ctx.r[11].s64 + 27696;
	// 8270AF58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270AF5C: 388ACD98  addi r4, r10, -0x3268
	ctx.r[4].s64 = ctx.r[10].s64 + -12904;
	// 8270AF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AF64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AF68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270AF6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AF70: 386A2BA4  addi r3, r10, 0x2ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 11172;
	// 8270AF74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270AF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AF7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AF84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AF8C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8270AF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270AF94: 4BD5BE8D  bl 0x82466e20
	ctx.lr = 0x8270AF98;
	sub_82466E20(ctx, base);
	// 8270AF98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270AF9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270AFA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270AFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270AFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270AFA8 size=112
    let mut pc: u32 = 0x8270AFA8;
    'dispatch: loop {
        match pc {
            0x8270AFA8 => {
    //   block [0x8270AFA8..0x8270B018)
	// 8270AFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270AFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270AFB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270AFB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270AFB8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270AFBC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270AFC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270AFC4: 390B6C90  addi r8, r11, 0x6c90
	ctx.r[8].s64 = ctx.r[11].s64 + 27792;
	// 8270AFC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270AFCC: 388ACEF0  addi r4, r10, -0x3110
	ctx.r[4].s64 = ctx.r[10].s64 + -12560;
	// 8270AFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270AFD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270AFD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270AFDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270AFE0: 386A2BD4  addi r3, r10, 0x2bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 11220;
	// 8270AFE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270AFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270AFEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270AFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270AFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270AFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270AFFC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8270B000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B004: 4BD5BE1D  bl 0x82466e20
	ctx.lr = 0x8270B008;
	sub_82466E20(ctx, base);
	// 8270B008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B018 size=100
    let mut pc: u32 = 0x8270B018;
    'dispatch: loop {
        match pc {
            0x8270B018 => {
    //   block [0x8270B018..0x8270B07C)
	// 8270B018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270B028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B02C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270B030: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270B034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B038: 388ACCC8  addi r4, r10, -0x3338
	ctx.r[4].s64 = ctx.r[10].s64 + -13112;
	// 8270B03C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B04C: 386A2C04  addi r3, r10, 0x2c04
	ctx.r[3].s64 = ctx.r[10].s64 + 11268;
	// 8270B050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B054: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B058: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270B05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B060: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270B064: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270B068: 4BD5BDB9  bl 0x82466e20
	ctx.lr = 0x8270B06C;
	sub_82466E20(ctx, base);
	// 8270B06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B080 size=100
    let mut pc: u32 = 0x8270B080;
    'dispatch: loop {
        match pc {
            0x8270B080 => {
    //   block [0x8270B080..0x8270B0E4)
	// 8270B080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B08C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270B090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B094: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270B098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270B09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B0A0: 388ACBF4  addi r4, r10, -0x340c
	ctx.r[4].s64 = ctx.r[10].s64 + -13324;
	// 8270B0A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B0B4: 386A2C34  addi r3, r10, 0x2c34
	ctx.r[3].s64 = ctx.r[10].s64 + 11316;
	// 8270B0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B0BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B0C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270B0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B0C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270B0CC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270B0D0: 4BD5BD51  bl 0x82466e20
	ctx.lr = 0x8270B0D4;
	sub_82466E20(ctx, base);
	// 8270B0D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B0E8 size=112
    let mut pc: u32 = 0x8270B0E8;
    'dispatch: loop {
        match pc {
            0x8270B0E8 => {
    //   block [0x8270B0E8..0x8270B158)
	// 8270B0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B0F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B0F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270B0FC: 38AA2CC4  addi r5, r10, 0x2cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 11460;
	// 8270B100: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270B104: 390B6CA8  addi r8, r11, 0x6ca8
	ctx.r[8].s64 = ctx.r[11].s64 + 27816;
	// 8270B108: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8270B10C: 388ACBD4  addi r4, r10, -0x342c
	ctx.r[4].s64 = ctx.r[10].s64 + -13356;
	// 8270B110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B114: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270B11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B120: 386A2C64  addi r3, r10, 0x2c64
	ctx.r[3].s64 = ctx.r[10].s64 + 11364;
	// 8270B124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270B128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B13C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8270B140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B144: 4BD5BCDD  bl 0x82466e20
	ctx.lr = 0x8270B148;
	sub_82466E20(ctx, base);
	// 8270B148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B158 size=100
    let mut pc: u32 = 0x8270B158;
    'dispatch: loop {
        match pc {
            0x8270B158 => {
    //   block [0x8270B158..0x8270B1BC)
	// 8270B158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B164: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270B168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B16C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270B170: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270B174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B178: 388ACEB4  addi r4, r10, -0x314c
	ctx.r[4].s64 = ctx.r[10].s64 + -12620;
	// 8270B17C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B18C: 386A2C94  addi r3, r10, 0x2c94
	ctx.r[3].s64 = ctx.r[10].s64 + 11412;
	// 8270B190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B194: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B198: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270B19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B1A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270B1A4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270B1A8: 4BD5BC79  bl 0x82466e20
	ctx.lr = 0x8270B1AC;
	sub_82466E20(ctx, base);
	// 8270B1AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B1C0 size=100
    let mut pc: u32 = 0x8270B1C0;
    'dispatch: loop {
        match pc {
            0x8270B1C0 => {
    //   block [0x8270B1C0..0x8270B224)
	// 8270B1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B1CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270B1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B1D4: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270B1D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270B1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B1E0: 388ACBBC  addi r4, r10, -0x3444
	ctx.r[4].s64 = ctx.r[10].s64 + -13380;
	// 8270B1E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B1F4: 386A2CC4  addi r3, r10, 0x2cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 11460;
	// 8270B1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B1FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B200: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270B204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B208: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270B20C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270B210: 4BD5BC11  bl 0x82466e20
	ctx.lr = 0x8270B214;
	sub_82466E20(ctx, base);
	// 8270B214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B228 size=100
    let mut pc: u32 = 0x8270B228;
    'dispatch: loop {
        match pc {
            0x8270B228 => {
    //   block [0x8270B228..0x8270B28C)
	// 8270B228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270B238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B23C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270B240: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270B244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B248: 388ACD84  addi r4, r10, -0x327c
	ctx.r[4].s64 = ctx.r[10].s64 + -12924;
	// 8270B24C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B25C: 386A2CF4  addi r3, r10, 0x2cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 11508;
	// 8270B260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B264: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B268: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270B26C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B270: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270B274: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270B278: 4BD5BBA9  bl 0x82466e20
	ctx.lr = 0x8270B27C;
	sub_82466E20(ctx, base);
	// 8270B27C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B290 size=56
    let mut pc: u32 = 0x8270B290;
    'dispatch: loop {
        match pc {
            0x8270B290 => {
    //   block [0x8270B290..0x8270B2C8)
	// 8270B290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B298: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B29C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 8270B2A0: 396BE660  addi r11, r11, -0x19a0
	ctx.r[11].s64 = ctx.r[11].s64 + -6560;
	// 8270B2A4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8270B2A8: 48001FE5  bl 0x8270d28c
	ctx.lr = 0x8270B2AC;
	// extern call 0x8270D28C → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8270B2AC: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8270B2B0: 386BCFA8  addi r3, r11, -0x3058
	ctx.r[3].s64 = ctx.r[11].s64 + -12376;
	// 8270B2B4: 4BE27885  bl 0x82532b38
	ctx.lr = 0x8270B2B8;
	sub_82532B38(ctx, base);
	// 8270B2B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270B2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270B2C8 size=16
    let mut pc: u32 = 0x8270B2C8;
    'dispatch: loop {
        match pc {
            0x8270B2C8 => {
    //   block [0x8270B2C8..0x8270B2D8)
	// 8270B2C8: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8270B2CC: 38800072  li r4, 0x72
	ctx.r[4].s64 = 114;
	// 8270B2D0: 386B380C  addi r3, r11, 0x380c
	ctx.r[3].s64 = ctx.r[11].s64 + 14348;
	// 8270B2D4: 4BE58CD4  b 0x82563fa8
	sub_82563FA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270B2D8 size=20
    let mut pc: u32 = 0x8270B2D8;
    'dispatch: loop {
        match pc {
            0x8270B2D8 => {
    //   block [0x8270B2D8..0x8270B2EC)
	// 8270B2D8: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8270B2DC: 388B380C  addi r4, r11, 0x380c
	ctx.r[4].s64 = ctx.r[11].s64 + 14348;
	// 8270B2E0: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8270B2E4: 386B3838  addi r3, r11, 0x3838
	ctx.r[3].s64 = ctx.r[11].s64 + 14392;
	// 8270B2E8: 4BE58D20  b 0x82564008
	sub_82564008(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B2F0 size=56
    let mut pc: u32 = 0x8270B2F0;
    'dispatch: loop {
        match pc {
            0x8270B2F0 => {
    //   block [0x8270B2F0..0x8270B328)
	// 8270B2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B2F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B2FC: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8270B300: 396B2348  addi r11, r11, 0x2348
	ctx.r[11].s64 = ctx.r[11].s64 + 9032;
	// 8270B304: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8270B308: 48001F85  bl 0x8270d28c
	ctx.lr = 0x8270B30C;
	// extern call 0x8270D28C → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8270B30C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8270B310: 386BCFC0  addi r3, r11, -0x3040
	ctx.r[3].s64 = ctx.r[11].s64 + -12352;
	// 8270B314: 4BE27825  bl 0x82532b38
	ctx.lr = 0x8270B318;
	sub_82532B38(ctx, base);
	// 8270B318: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270B31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B328 size=56
    let mut pc: u32 = 0x8270B328;
    'dispatch: loop {
        match pc {
            0x8270B328 => {
    //   block [0x8270B328..0x8270B360)
	// 8270B328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B330: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B334: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8270B338: 396B2368  addi r11, r11, 0x2368
	ctx.r[11].s64 = ctx.r[11].s64 + 9064;
	// 8270B33C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8270B340: 48001F4D  bl 0x8270d28c
	ctx.lr = 0x8270B344;
	// extern call 0x8270D28C → crate::xboxkrnl::RtlInitializeCriticalSection
	crate::xboxkrnl::RtlInitializeCriticalSection(ctx, base);
	// 8270B344: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8270B348: 386BCFD8  addi r3, r11, -0x3028
	ctx.r[3].s64 = ctx.r[11].s64 + -12328;
	// 8270B34C: 4BE277ED  bl 0x82532b38
	ctx.lr = 0x8270B350;
	sub_82532B38(ctx, base);
	// 8270B350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270B354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B360 size=108
    let mut pc: u32 = 0x8270B360;
    'dispatch: loop {
        match pc {
            0x8270B360 => {
    //   block [0x8270B360..0x8270B3CC)
	// 8270B360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B36C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8270B370: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8270B374: 38EB8F20  addi r7, r11, -0x70e0
	ctx.r[7].s64 = ctx.r[11].s64 + -28896;
	// 8270B378: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8270B37C: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8270B380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B384: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B388: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270B38C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B390: 386A3A78  addi r3, r10, 0x3a78
	ctx.r[3].s64 = ctx.r[10].s64 + 14968;
	// 8270B394: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270B398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B39C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B3A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B3AC: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8270B3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B3B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270B3B8: 4BD5BA69  bl 0x82466e20
	ctx.lr = 0x8270B3BC;
	sub_82466E20(ctx, base);
	// 8270B3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B3D0 size=108
    let mut pc: u32 = 0x8270B3D0;
    'dispatch: loop {
        match pc {
            0x8270B3D0 => {
    //   block [0x8270B3D0..0x8270B43C)
	// 8270B3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B3DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8270B3E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8270B3E4: 38EB8FC8  addi r7, r11, -0x7038
	ctx.r[7].s64 = ctx.r[11].s64 + -28728;
	// 8270B3E8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8270B3EC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8270B3F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B3F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B3F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270B3FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B400: 386A3AA8  addi r3, r10, 0x3aa8
	ctx.r[3].s64 = ctx.r[10].s64 + 15016;
	// 8270B404: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270B408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B40C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B41C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8270B420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270B428: 4BD5B9F9  bl 0x82466e20
	ctx.lr = 0x8270B42C;
	sub_82466E20(ctx, base);
	// 8270B42C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270B440 size=244
    let mut pc: u32 = 0x8270B440;
    'dispatch: loop {
        match pc {
            0x8270B440 => {
    //   block [0x8270B440..0x8270B534)
	// 8270B440: 3D208283  lis r9, -0x7d7d
	ctx.r[9].s64 = -2105344000;
	// 8270B444: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8270B448: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8270B44C: 396B23D0  addi r11, r11, 0x23d0
	ctx.r[11].s64 = ctx.r[11].s64 + 9168;
	// 8270B450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B454: 812923C0  lwz r9, 0x23c0(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(9152 as u32) ) } as u64;
	// 8270B458: 39087318  addi r8, r8, 0x7318
	ctx.r[8].s64 = ctx.r[8].s64 + 29464;
	// 8270B45C: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 8270B460: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 8270B464: 38E7730C  addi r7, r7, 0x730c
	ctx.r[7].s64 = ctx.r[7].s64 + 29452;
	// 8270B468: 38C67304  addi r6, r6, 0x7304
	ctx.r[6].s64 = ctx.r[6].s64 + 29444;
	// 8270B46C: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8270B470: 3CA0820D  lis r5, -0x7df3
	ctx.r[5].s64 = -2113077248;
	// 8270B474: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8270B478: 910B0060  stw r8, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 8270B47C: 38A5C4B0  addi r5, r5, -0x3b50
	ctx.r[5].s64 = ctx.r[5].s64 + -15184;
	// 8270B480: 914B0064  stw r10, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8270B484: 912B0068  stw r9, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 8270B488: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8270B48C: 992B006C  stb r9, 0x6c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[9].u8 ) };
	// 8270B490: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270B494: 992B006D  stb r9, 0x6d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(109 as u32), ctx.r[9].u8 ) };
	// 8270B498: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 8270B49C: B14B006E  sth r10, 0x6e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(110 as u32), ctx.r[10].u16 ) };
	// 8270B4A0: B14B0070  sth r10, 0x70(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u16 ) };
	// 8270B4A4: B12B0072  sth r9, 0x72(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(114 as u32), ctx.r[9].u16 ) };
	// 8270B4A8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8270B4AC: 914B0074  stw r10, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8270B4B0: 90EB0078  stw r7, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[7].u32 ) };
	// 8270B4B4: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 8270B4B8: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 8270B4BC: 992B0084  stb r9, 0x84(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[9].u8 ) };
	// 8270B4C0: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 8270B4C4: 994B0085  stb r10, 0x85(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(133 as u32), ctx.r[10].u8 ) };
	// 8270B4C8: B14B0086  sth r10, 0x86(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(134 as u32), ctx.r[10].u16 ) };
	// 8270B4CC: B14B0088  sth r10, 0x88(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u16 ) };
	// 8270B4D0: B12B008A  sth r9, 0x8a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(138 as u32), ctx.r[9].u16 ) };
	// 8270B4D4: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8270B4D8: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 8270B4DC: 90CB0090  stw r6, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[6].u32 ) };
	// 8270B4E0: 914B0094  stw r10, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 8270B4E4: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 8270B4E8: 992B009C  stb r9, 0x9c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[9].u8 ) };
	// 8270B4EC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8270B4F0: 994B009D  stb r10, 0x9d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(157 as u32), ctx.r[10].u8 ) };
	// 8270B4F4: B14B009E  sth r10, 0x9e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(158 as u32), ctx.r[10].u16 ) };
	// 8270B4F8: B14B00A0  sth r10, 0xa0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[10].u16 ) };
	// 8270B4FC: B12B00A2  sth r9, 0xa2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(162 as u32), ctx.r[9].u16 ) };
	// 8270B500: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8270B504: 914B00A4  stw r10, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 8270B508: 90AB00A8  stw r5, 0xa8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(168 as u32), ctx.r[5].u32 ) };
	// 8270B50C: 914B00AC  stw r10, 0xac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 8270B510: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 8270B514: 992B00B4  stb r9, 0xb4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(180 as u32), ctx.r[9].u8 ) };
	// 8270B518: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8270B51C: 994B00B5  stb r10, 0xb5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(181 as u32), ctx.r[10].u8 ) };
	// 8270B520: B14B00B6  sth r10, 0xb6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(182 as u32), ctx.r[10].u16 ) };
	// 8270B524: B14B00B8  sth r10, 0xb8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u16 ) };
	// 8270B528: B12B00BA  sth r9, 0xba(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(186 as u32), ctx.r[9].u16 ) };
	// 8270B52C: 914B00BC  stw r10, 0xbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 8270B530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B538 size=112
    let mut pc: u32 = 0x8270B538;
    'dispatch: loop {
        match pc {
            0x8270B538 => {
    //   block [0x8270B538..0x8270B5A8)
	// 8270B538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B544: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8270B548: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8270B54C: 392A91E0  addi r9, r10, -0x6e20
	ctx.r[9].s64 = ctx.r[10].s64 + -28192;
	// 8270B550: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8270B554: 390B23D0  addi r8, r11, 0x23d0
	ctx.r[8].s64 = ctx.r[11].s64 + 9168;
	// 8270B558: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8270B55C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8270B560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B564: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270B56C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8270B570: 386A3AD8  addi r3, r10, 0x3ad8
	ctx.r[3].s64 = ctx.r[10].s64 + 15064;
	// 8270B574: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8270B578: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8270B57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B58C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270B590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B594: 4BD5B88D  bl 0x82466e20
	ctx.lr = 0x8270B598;
	sub_82466E20(ctx, base);
	// 8270B598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B59C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B5A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B5A8 size=108
    let mut pc: u32 = 0x8270B5A8;
    'dispatch: loop {
        match pc {
            0x8270B5A8 => {
    //   block [0x8270B5A8..0x8270B614)
	// 8270B5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B5B4: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8270B5B8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8270B5BC: 38EB921C  addi r7, r11, -0x6de4
	ctx.r[7].s64 = ctx.r[11].s64 + -28132;
	// 8270B5C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270B5C4: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8270B5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B5CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B5D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270B5D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B5D8: 386A3B08  addi r3, r10, 0x3b08
	ctx.r[3].s64 = ctx.r[10].s64 + 15112;
	// 8270B5DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270B5E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B5F4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270B5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B5FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270B600: 4BD5B821  bl 0x82466e20
	ctx.lr = 0x8270B604;
	sub_82466E20(ctx, base);
	// 8270B604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B618 size=108
    let mut pc: u32 = 0x8270B618;
    'dispatch: loop {
        match pc {
            0x8270B618 => {
    //   block [0x8270B618..0x8270B684)
	// 8270B618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B624: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8270B628: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8270B62C: 38EB924C  addi r7, r11, -0x6db4
	ctx.r[7].s64 = ctx.r[11].s64 + -28084;
	// 8270B630: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270B634: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8270B638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B63C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270B644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B648: 386A3B38  addi r3, r10, 0x3b38
	ctx.r[3].s64 = ctx.r[10].s64 + 15160;
	// 8270B64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270B650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B664: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8270B668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270B670: 4BD5B7B1  bl 0x82466e20
	ctx.lr = 0x8270B674;
	sub_82466E20(ctx, base);
	// 8270B674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B688 size=108
    let mut pc: u32 = 0x8270B688;
    'dispatch: loop {
        match pc {
            0x8270B688 => {
    //   block [0x8270B688..0x8270B6F4)
	// 8270B688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B694: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8270B698: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8270B69C: 38EB9290  addi r7, r11, -0x6d70
	ctx.r[7].s64 = ctx.r[11].s64 + -28016;
	// 8270B6A0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8270B6A4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8270B6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B6AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B6B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270B6B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B6B8: 386A3B68  addi r3, r10, 0x3b68
	ctx.r[3].s64 = ctx.r[10].s64 + 15208;
	// 8270B6BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270B6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B6D4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8270B6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B6DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270B6E0: 4BD5B741  bl 0x82466e20
	ctx.lr = 0x8270B6E4;
	sub_82466E20(ctx, base);
	// 8270B6E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B6E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B6EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B6F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B6F8 size=108
    let mut pc: u32 = 0x8270B6F8;
    'dispatch: loop {
        match pc {
            0x8270B6F8 => {
    //   block [0x8270B6F8..0x8270B764)
	// 8270B6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B704: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8270B708: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8270B70C: 38EB9380  addi r7, r11, -0x6c80
	ctx.r[7].s64 = ctx.r[11].s64 + -27776;
	// 8270B710: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8270B714: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8270B718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B71C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B720: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270B724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B728: 386A3B98  addi r3, r10, 0x3b98
	ctx.r[3].s64 = ctx.r[10].s64 + 15256;
	// 8270B72C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270B730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B744: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8270B748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B74C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270B750: 4BD5B6D1  bl 0x82466e20
	ctx.lr = 0x8270B754;
	sub_82466E20(ctx, base);
	// 8270B754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270B768 size=292
    let mut pc: u32 = 0x8270B768;
    'dispatch: loop {
        match pc {
            0x8270B768 => {
    //   block [0x8270B768..0x8270B88C)
	// 8270B768: 3D208283  lis r9, -0x7d7d
	ctx.r[9].s64 = -2105344000;
	// 8270B76C: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8270B770: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8270B774: 396B24A0  addi r11, r11, 0x24a0
	ctx.r[11].s64 = ctx.r[11].s64 + 9376;
	// 8270B778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B77C: 81292490  lwz r9, 0x2490(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(9360 as u32) ) } as u64;
	// 8270B780: 39087318  addi r8, r8, 0x7318
	ctx.r[8].s64 = ctx.r[8].s64 + 29464;
	// 8270B784: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 8270B788: 3CC08201  lis r6, -0x7dff
	ctx.r[6].s64 = -2113863680;
	// 8270B78C: 38E7730C  addi r7, r7, 0x730c
	ctx.r[7].s64 = ctx.r[7].s64 + 29452;
	// 8270B790: 38C67304  addi r6, r6, 0x7304
	ctx.r[6].s64 = ctx.r[6].s64 + 29444;
	// 8270B794: 912B0050  stw r9, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8270B798: 3CA0820D  lis r5, -0x7df3
	ctx.r[5].s64 = -2113077248;
	// 8270B79C: 914B005C  stw r10, 0x5c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8270B7A0: 910B0060  stw r8, 0x60(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 8270B7A4: 38A5C4B0  addi r5, r5, -0x3b50
	ctx.r[5].s64 = ctx.r[5].s64 + -15184;
	// 8270B7A8: 914B0064  stw r10, 0x64(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8270B7AC: 3C80820A  lis r4, -0x7df6
	ctx.r[4].s64 = -2113273856;
	// 8270B7B0: 912B0068  stw r9, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 8270B7B4: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 8270B7B8: 3884927C  addi r4, r4, -0x6d84
	ctx.r[4].s64 = ctx.r[4].s64 + -28036;
	// 8270B7BC: 992B006C  stb r9, 0x6c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(108 as u32), ctx.r[9].u8 ) };
	// 8270B7C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270B7C4: 992B006D  stb r9, 0x6d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(109 as u32), ctx.r[9].u8 ) };
	// 8270B7C8: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 8270B7CC: B14B006E  sth r10, 0x6e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(110 as u32), ctx.r[10].u16 ) };
	// 8270B7D0: B14B0070  sth r10, 0x70(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(112 as u32), ctx.r[10].u16 ) };
	// 8270B7D4: B12B0072  sth r9, 0x72(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(114 as u32), ctx.r[9].u16 ) };
	// 8270B7D8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8270B7DC: 914B0074  stw r10, 0x74(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 8270B7E0: 90EB0078  stw r7, 0x78(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(120 as u32), ctx.r[7].u32 ) };
	// 8270B7E4: 914B007C  stw r10, 0x7c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 8270B7E8: 914B0080  stw r10, 0x80(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 8270B7EC: 992B0084  stb r9, 0x84(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(132 as u32), ctx.r[9].u8 ) };
	// 8270B7F0: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 8270B7F4: 994B0085  stb r10, 0x85(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(133 as u32), ctx.r[10].u8 ) };
	// 8270B7F8: B14B0086  sth r10, 0x86(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(134 as u32), ctx.r[10].u16 ) };
	// 8270B7FC: B14B0088  sth r10, 0x88(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(136 as u32), ctx.r[10].u16 ) };
	// 8270B800: B12B008A  sth r9, 0x8a(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(138 as u32), ctx.r[9].u16 ) };
	// 8270B804: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8270B808: 914B008C  stw r10, 0x8c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 8270B80C: 90CB0090  stw r6, 0x90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(144 as u32), ctx.r[6].u32 ) };
	// 8270B810: 914B0094  stw r10, 0x94(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(148 as u32), ctx.r[10].u32 ) };
	// 8270B814: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 8270B818: 992B009C  stb r9, 0x9c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[9].u8 ) };
	// 8270B81C: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8270B820: 994B009D  stb r10, 0x9d(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(157 as u32), ctx.r[10].u8 ) };
	// 8270B824: B14B009E  sth r10, 0x9e(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(158 as u32), ctx.r[10].u16 ) };
	// 8270B828: B14B00A0  sth r10, 0xa0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(160 as u32), ctx.r[10].u16 ) };
	// 8270B82C: B12B00A2  sth r9, 0xa2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(162 as u32), ctx.r[9].u16 ) };
	// 8270B830: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8270B834: 914B00A4  stw r10, 0xa4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(164 as u32), ctx.r[10].u32 ) };
	// 8270B838: 90AB00A8  stw r5, 0xa8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(168 as u32), ctx.r[5].u32 ) };
	// 8270B83C: 914B00AC  stw r10, 0xac(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(172 as u32), ctx.r[10].u32 ) };
	// 8270B840: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 8270B844: 992B00B4  stb r9, 0xb4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(180 as u32), ctx.r[9].u8 ) };
	// 8270B848: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8270B84C: 994B00B5  stb r10, 0xb5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(181 as u32), ctx.r[10].u8 ) };
	// 8270B850: B14B00B6  sth r10, 0xb6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(182 as u32), ctx.r[10].u16 ) };
	// 8270B854: B14B00B8  sth r10, 0xb8(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(184 as u32), ctx.r[10].u16 ) };
	// 8270B858: B12B00BA  sth r9, 0xba(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(186 as u32), ctx.r[9].u16 ) };
	// 8270B85C: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 8270B860: 914B00BC  stw r10, 0xbc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(188 as u32), ctx.r[10].u32 ) };
	// 8270B864: 908B00C0  stw r4, 0xc0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(192 as u32), ctx.r[4].u32 ) };
	// 8270B868: 914B00C4  stw r10, 0xc4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(196 as u32), ctx.r[10].u32 ) };
	// 8270B86C: 914B00C8  stw r10, 0xc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 8270B870: 992B00CC  stb r9, 0xcc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(204 as u32), ctx.r[9].u8 ) };
	// 8270B874: 994B00CD  stb r10, 0xcd(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(205 as u32), ctx.r[10].u8 ) };
	// 8270B878: B14B00CE  sth r10, 0xce(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(206 as u32), ctx.r[10].u16 ) };
	// 8270B87C: B14B00D0  sth r10, 0xd0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[10].u16 ) };
	// 8270B880: B12B00D2  sth r9, 0xd2(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(210 as u32), ctx.r[9].u16 ) };
	// 8270B884: 914B00D4  stw r10, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[10].u32 ) };
	// 8270B888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B890 size=112
    let mut pc: u32 = 0x8270B890;
    'dispatch: loop {
        match pc {
            0x8270B890 => {
    //   block [0x8270B890..0x8270B900)
	// 8270B890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B89C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 8270B8A0: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8270B8A4: 392A95E0  addi r9, r10, -0x6a20
	ctx.r[9].s64 = ctx.r[10].s64 + -27168;
	// 8270B8A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8270B8AC: 390B24A0  addi r8, r11, 0x24a0
	ctx.r[8].s64 = ctx.r[11].s64 + 9376;
	// 8270B8B0: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8270B8B4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8270B8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B8BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B8C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270B8C4: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8270B8C8: 386A3BC8  addi r3, r10, 0x3bc8
	ctx.r[3].s64 = ctx.r[10].s64 + 15304;
	// 8270B8CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8270B8D0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8270B8D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B8E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270B8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B8EC: 4BD5B535  bl 0x82466e20
	ctx.lr = 0x8270B8F0;
	sub_82466E20(ctx, base);
	// 8270B8F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B900 size=108
    let mut pc: u32 = 0x8270B900;
    'dispatch: loop {
        match pc {
            0x8270B900 => {
    //   block [0x8270B900..0x8270B96C)
	// 8270B900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B90C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8270B910: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8270B914: 38EB961C  addi r7, r11, -0x69e4
	ctx.r[7].s64 = ctx.r[11].s64 + -27108;
	// 8270B918: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270B91C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8270B920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B924: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270B92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B930: 386A3BF8  addi r3, r10, 0x3bf8
	ctx.r[3].s64 = ctx.r[10].s64 + 15352;
	// 8270B934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270B938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B94C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270B950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270B958: 4BD5B4C9  bl 0x82466e20
	ctx.lr = 0x8270B95C;
	sub_82466E20(ctx, base);
	// 8270B95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270B970 size=108
    let mut pc: u32 = 0x8270B970;
    'dispatch: loop {
        match pc {
            0x8270B970 => {
    //   block [0x8270B970..0x8270B9DC)
	// 8270B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270B974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270B978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270B97C: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 8270B980: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8270B984: 38EB9650  addi r7, r11, -0x69b0
	ctx.r[7].s64 = ctx.r[11].s64 + -27056;
	// 8270B988: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8270B98C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8270B990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270B994: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270B998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270B99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270B9A0: 386A3C28  addi r3, r10, 0x3c28
	ctx.r[3].s64 = ctx.r[10].s64 + 15400;
	// 8270B9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270B9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270B9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270B9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270B9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270B9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270B9BC: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8270B9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270B9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270B9C8: 4BD5B459  bl 0x82466e20
	ctx.lr = 0x8270B9CC;
	sub_82466E20(ctx, base);
	// 8270B9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270B9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270B9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270B9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270B9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270B9E0 size=28
    let mut pc: u32 = 0x8270B9E0;
    'dispatch: loop {
        match pc {
            0x8270B9E0 => {
    //   block [0x8270B9E0..0x8270B9FC)
	// 8270B9E0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270B9E4: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8270B9E8: 396B25E0  addi r11, r11, 0x25e0
	ctx.r[11].s64 = ctx.r[11].s64 + 9696;
	// 8270B9EC: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 8270B9F0: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 8270B9F4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8270B9F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270BA00 size=28
    let mut pc: u32 = 0x8270BA00;
    'dispatch: loop {
        match pc {
            0x8270BA00 => {
    //   block [0x8270BA00..0x8270BA1C)
	// 8270BA00: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270BA04: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8270BA08: 396B2644  addi r11, r11, 0x2644
	ctx.r[11].s64 = ctx.r[11].s64 + 9796;
	// 8270BA0C: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 8270BA10: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 8270BA14: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8270BA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8270BA20 size=56
    let mut pc: u32 = 0x8270BA20;
    'dispatch: loop {
        match pc {
            0x8270BA20 => {
    //   block [0x8270BA20..0x8270BA58)
	// 8270BA20: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270BA24: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8270BA28: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8270BA2C: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 8270BA30: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 8270BA34: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 8270BA38: C00B1850  lfs f0, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8270BA3C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8270BA40: D001FFFC  stfs f0, -4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8270BA44: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 8270BA48: 396B3C60  addi r11, r11, 0x3c60
	ctx.r[11].s64 = ctx.r[11].s64 + 15456;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270BA58 size=528
    let mut pc: u32 = 0x8270BA58;
    'dispatch: loop {
        match pc {
            0x8270BA58 => {
    //   block [0x8270BA58..0x8270BC68)
	// 8270BA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270BA5C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8270BA60: 3D20829A  lis r9, -0x7d66
	ctx.r[9].s64 = -2103836672;
	// 8270BA64: 39293C80  addi r9, r9, 0x3c80
	ctx.r[9].s64 = ctx.r[9].s64 + 15488;
	// 8270BA68: 9141FF20  stw r10, -0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), ctx.r[10].u32 ) };
	// 8270BA6C: 9141FF24  stw r10, -0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-220 as u32), ctx.r[10].u32 ) };
	// 8270BA70: 9141FF28  stw r10, -0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-216 as u32), ctx.r[10].u32 ) };
	// 8270BA74: 9141FF2C  stw r10, -0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-212 as u32), ctx.r[10].u32 ) };
	// 8270BA78: 3901FF20  addi r8, r1, -0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + -224;
	// 8270BA7C: 9141FF30  stw r10, -0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), ctx.r[10].u32 ) };
	// 8270BA80: 9141FF34  stw r10, -0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), ctx.r[10].u32 ) };
	// 8270BA84: 9141FF38  stw r10, -0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), ctx.r[10].u32 ) };
	// 8270BA88: 9161FF3C  stw r11, -0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), ctx.r[11].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270BC68 size=528
    let mut pc: u32 = 0x8270BC68;
    'dispatch: loop {
        match pc {
            0x8270BC68 => {
    //   block [0x8270BC68..0x8270BE78)
	// 8270BC68: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8270BC6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270BC70: 3D20829A  lis r9, -0x7d66
	ctx.r[9].s64 = -2103836672;
	// 8270BC74: 39293D80  addi r9, r9, 0x3d80
	ctx.r[9].s64 = ctx.r[9].s64 + 15744;
	// 8270BC78: 9161FF20  stw r11, -0xe0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-224 as u32), ctx.r[11].u32 ) };
	// 8270BC7C: 9161FF24  stw r11, -0xdc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-220 as u32), ctx.r[11].u32 ) };
	// 8270BC80: 9161FF28  stw r11, -0xd8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-216 as u32), ctx.r[11].u32 ) };
	// 8270BC84: 9161FF2C  stw r11, -0xd4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-212 as u32), ctx.r[11].u32 ) };
	// 8270BC88: 3901FF20  addi r8, r1, -0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + -224;
	// 8270BC8C: 9161FF30  stw r11, -0xd0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-208 as u32), ctx.r[11].u32 ) };
	// 8270BC90: 9161FF34  stw r11, -0xcc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-204 as u32), ctx.r[11].u32 ) };
	// 8270BC94: 9161FF38  stw r11, -0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-200 as u32), ctx.r[11].u32 ) };
	// 8270BC98: 9141FF3C  stw r10, -0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-196 as u32), ctx.r[10].u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270BE78 size=28
    let mut pc: u32 = 0x8270BE78;
    'dispatch: loop {
        match pc {
            0x8270BE78 => {
    //   block [0x8270BE78..0x8270BE94)
	// 8270BE78: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270BE7C: 3D608283  lis r11, -0x7d7d
	ctx.r[11].s64 = -2105344000;
	// 8270BE80: 396B2740  addi r11, r11, 0x2740
	ctx.r[11].s64 = ctx.r[11].s64 + 10048;
	// 8270BE84: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 8270BE88: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 8270BE8C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8270BE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270BE98 size=44
    let mut pc: u32 = 0x8270BE98;
    'dispatch: loop {
        match pc {
            0x8270BE98 => {
    //   block [0x8270BE98..0x8270BEC4)
	// 8270BE98: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8270BE9C: 3D20829A  lis r9, -0x7d66
	ctx.r[9].s64 = -2103836672;
	// 8270BEA0: 394BDE3C  addi r10, r11, -0x21c4
	ctx.r[10].s64 = ctx.r[11].s64 + -8644;
	// 8270BEA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270BEA8: 91693E94  stw r11, 0x3e94(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(16020 as u32), ctx.r[11].u32 ) };
	// 8270BEAC: 3D608287  lis r11, -0x7d79
	ctx.r[11].s64 = -2105081856;
	// 8270BEB0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270BEB4: 396BA080  addi r11, r11, -0x5f80
	ctx.r[11].s64 = ctx.r[11].s64 + -24448;
	// 8270BEB8: 912B0024  stw r9, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 8270BEBC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270BEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270BEC8 size=28
    let mut pc: u32 = 0x8270BEC8;
    'dispatch: loop {
        match pc {
            0x8270BEC8 => {
    //   block [0x8270BEC8..0x8270BEE4)
	// 8270BEC8: 3D6082C0  lis r11, -0x7d40
	ctx.r[11].s64 = -2101346304;
	// 8270BECC: 3D400003  lis r10, 3
	ctx.r[10].s64 = 196608;
	// 8270BED0: 396BBFF0  addi r11, r11, -0x4010
	ctx.r[11].s64 = ctx.r[11].s64 + -16400;
	// 8270BED4: 614A943C  ori r10, r10, 0x943c
	ctx.r[10].u64 = ctx.r[10].u64 | 37948;
	// 8270BED8: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8270BEDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270BEE0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BEE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270BEE4 size=8
    let mut pc: u32 = 0x8270BEE4;
    'dispatch: loop {
        match pc {
            0x8270BEE4 => {
    //   block [0x8270BEE4..0x8270BEEC)
	// 8270BEE4: 4BA11174  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270BEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270BEF0 size=68
    let mut pc: u32 = 0x8270BEF0;
    'dispatch: loop {
        match pc {
            0x8270BEF0 => {
    //   block [0x8270BEF0..0x8270BF34)
	// 8270BEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270BEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270BEF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270BEFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270BF00: 3D608287  lis r11, -0x7d79
	ctx.r[11].s64 = -2105081856;
	// 8270BF04: 3BEBA0CC  addi r31, r11, -0x5f34
	ctx.r[31].s64 = ctx.r[11].s64 + -24372;
	// 8270BF08: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 8270BF0C: 4BA248F5  bl 0x82130800
	ctx.lr = 0x8270BF10;
	sub_82130800(ctx, base);
	// 8270BF10: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8270BF14: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270BF18: 419A0008  beq cr6, 0x8270bf20
	if ctx.cr[6].eq {
	pc = 0x8270BF20; continue 'dispatch;
	}
	// 8270BF1C: 4BA1113D  bl 0x8211d058
	ctx.lr = 0x8270BF20;
	sub_8211D058(ctx, base);
	// 8270BF20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270BF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270BF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270BF2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270BF30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270BF38 size=72
    let mut pc: u32 = 0x8270BF38;
    'dispatch: loop {
        match pc {
            0x8270BF38 => {
    //   block [0x8270BF38..0x8270BF80)
	// 8270BF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270BF3C: 4BE29181  bl 0x825350bc
	ctx.lr = 0x8270BF40;
	sub_82535080(ctx, base);
	// 8270BF40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270BF44: 3D6082C3  lis r11, -0x7d3d
	ctx.r[11].s64 = -2101149696;
	// 8270BF48: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8270BF4C: 396B5530  addi r11, r11, 0x5530
	ctx.r[11].s64 = ctx.r[11].s64 + 21808;
	// 8270BF50: 3D6B000B  addis r11, r11, 0xb
	ctx.r[11].s64 = ctx.r[11].s64 + 720896;
	// 8270BF54: 3BEB53A0  addi r31, r11, 0x53a0
	ctx.r[31].s64 = ctx.r[11].s64 + 21408;
	// 8270BF58: 3D600005  lis r11, 5
	ctx.r[11].s64 = 327680;
	// 8270BF5C: 617DA9D0  ori r29, r11, 0xa9d0
	ctx.r[29].u64 = ctx.r[11].u64 | 43472;
	// 8270BF60: 7FFDF850  subf r31, r29, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[29].s64;
	// 8270BF64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8270BF68: 4BA30569  bl 0x8213c4d0
	ctx.lr = 0x8270BF6C;
	sub_8213C4D0(ctx, base);
	// 8270BF6C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8270BF70: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8270BF74: 4098FFEC  bge cr6, 0x8270bf60
	if !ctx.cr[6].lt {
	pc = 0x8270BF60; continue 'dispatch;
	}
	// 8270BF78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8270BF7C: 4BE29190  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270BF80 size=84
    let mut pc: u32 = 0x8270BF80;
    'dispatch: loop {
        match pc {
            0x8270BF80 => {
    //   block [0x8270BF80..0x8270BFD4)
	// 8270BF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270BF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270BF88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8270BF8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270BF90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270BF94: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270BF98: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8270BF9C: 396BB028  addi r11, r11, -0x4fd8
	ctx.r[11].s64 = ctx.r[11].s64 + -20440;
	// 8270BFA0: 3BEB00A8  addi r31, r11, 0xa8
	ctx.r[31].s64 = ctx.r[11].s64 + 168;
	// 8270BFA4: 3BFFFFAC  addi r31, r31, -0x54
	ctx.r[31].s64 = ctx.r[31].s64 + -84;
	// 8270BFA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8270BFAC: 4BA31F45  bl 0x8213def0
	ctx.lr = 0x8270BFB0;
	sub_8213DEF0(ctx, base);
	// 8270BFB0: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8270BFB4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8270BFB8: 4098FFEC  bge cr6, 0x8270bfa4
	if !ctx.cr[6].lt {
	pc = 0x8270BFA4; continue 'dispatch;
	}
	// 8270BFBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8270BFC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270BFC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270BFC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8270BFCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270BFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270BFD8 size=20
    let mut pc: u32 = 0x8270BFD8;
    'dispatch: loop {
        match pc {
            0x8270BFD8 => {
    //   block [0x8270BFD8..0x8270BFEC)
	// 8270BFD8: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270BFDC: 396BB138  addi r11, r11, -0x4ec8
	ctx.r[11].s64 = ctx.r[11].s64 + -20168;
	// 8270BFE0: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270BFE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270BFE8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BFEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270BFEC size=8
    let mut pc: u32 = 0x8270BFEC;
    'dispatch: loop {
        match pc {
            0x8270BFEC => {
    //   block [0x8270BFEC..0x8270BFF4)
	// 8270BFEC: 4BA1106C  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270BFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270BFF8 size=92
    let mut pc: u32 = 0x8270BFF8;
    'dispatch: loop {
        match pc {
            0x8270BFF8 => {
    //   block [0x8270BFF8..0x8270C054)
	// 8270BFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270BFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270C000: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8270C004: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270C008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C00C: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270C010: 3BC00009  li r30, 9
	ctx.r[30].s64 = 9;
	// 8270C014: 396BB1B0  addi r11, r11, -0x4e50
	ctx.r[11].s64 = ctx.r[11].s64 + -20048;
	// 8270C018: 3BEB0374  addi r31, r11, 0x374
	ctx.r[31].s64 = ctx.r[11].s64 + 884;
	// 8270C01C: 3BFFFFE8  addi r31, r31, -0x18
	ctx.r[31].s64 = ctx.r[31].s64 + -24;
	// 8270C020: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8270C024: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C028: 419A0008  beq cr6, 0x8270c030
	if ctx.cr[6].eq {
	pc = 0x8270C030; continue 'dispatch;
	}
	// 8270C02C: 4BA1102D  bl 0x8211d058
	ctx.lr = 0x8270C030;
	sub_8211D058(ctx, base);
	// 8270C030: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8270C034: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8270C038: 4098FFE4  bge cr6, 0x8270c01c
	if !ctx.cr[6].lt {
	pc = 0x8270C01C; continue 'dispatch;
	}
	// 8270C03C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8270C040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270C044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270C048: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8270C04C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270C050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C058 size=20
    let mut pc: u32 = 0x8270C058;
    'dispatch: loop {
        match pc {
            0x8270C058 => {
    //   block [0x8270C058..0x8270C06C)
	// 8270C058: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270C05C: 3D408287  lis r10, -0x7d79
	ctx.r[10].s64 = -2105081856;
	// 8270C060: 396BE1B0  addi r11, r11, -0x1e50
	ctx.r[11].s64 = ctx.r[11].s64 + -7760;
	// 8270C064: 916AA4D8  stw r11, -0x5b28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23336 as u32), ctx.r[11].u32 ) };
	// 8270C068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C070 size=120
    let mut pc: u32 = 0x8270C070;
    'dispatch: loop {
        match pc {
            0x8270C070 => {
    //   block [0x8270C070..0x8270C0E8)
	// 8270C070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C074: 4BE29045  bl 0x825350b8
	ctx.lr = 0x8270C078;
	sub_82535080(ctx, base);
	// 8270C078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C07C: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270C080: 3B80000F  li r28, 0xf
	ctx.r[28].s64 = 15;
	// 8270C084: 3BCBC470  addi r30, r11, -0x3b90
	ctx.r[30].s64 = ctx.r[11].s64 + -15248;
	// 8270C088: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8270C08C: 3BFE0318  addi r31, r30, 0x318
	ctx.r[31].s64 = ctx.r[30].s64 + 792;
	// 8270C090: 3BABE088  addi r29, r11, -0x1f78
	ctx.r[29].s64 = ctx.r[11].s64 + -8056;
	// 8270C094: 3BFFFFF0  addi r31, r31, -0x10
	ctx.r[31].s64 = ctx.r[31].s64 + -16;
	// 8270C098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8270C09C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8270C0A0: 4BAFBD09  bl 0x82207da8
	ctx.lr = 0x8270C0A4;
	sub_82207DA8(ctx, base);
	// 8270C0A4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8270C0A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C0AC: 419A0008  beq cr6, 0x8270c0b4
	if ctx.cr[6].eq {
	pc = 0x8270C0B4; continue 'dispatch;
	}
	// 8270C0B0: 4BA10FA9  bl 0x8211d058
	ctx.lr = 0x8270C0B4;
	sub_8211D058(ctx, base);
	// 8270C0B4: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 8270C0B8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8270C0BC: 4098FFD8  bge cr6, 0x8270c094
	if !ctx.cr[6].lt {
	pc = 0x8270C094; continue 'dispatch;
	}
	// 8270C0C0: E87E0178  ld r3, 0x178(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(376 as u32) ) };
	// 8270C0C4: 4BC70DF5  bl 0x8237ceb8
	ctx.lr = 0x8270C0C8;
	sub_8237CEB8(ctx, base);
	// 8270C0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C0CC: F97E0178  std r11, 0x178(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(376 as u32), ctx.r[11].u64 ) };
	// 8270C0D0: 807E0170  lwz r3, 0x170(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(368 as u32) ) } as u64;
	// 8270C0D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C0D8: 419A0008  beq cr6, 0x8270c0e0
	if ctx.cr[6].eq {
	pc = 0x8270C0E0; continue 'dispatch;
	}
	// 8270C0DC: 4BA10F7D  bl 0x8211d058
	ctx.lr = 0x8270C0E0;
	sub_8211D058(ctx, base);
	// 8270C0E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270C0E4: 4BE29024  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C0E8 size=92
    let mut pc: u32 = 0x8270C0E8;
    'dispatch: loop {
        match pc {
            0x8270C0E8 => {
    //   block [0x8270C0E8..0x8270C144)
	// 8270C0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270C0F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8270C0F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270C0F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C0FC: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270C100: 3BC00005  li r30, 5
	ctx.r[30].s64 = 5;
	// 8270C104: 396BCB28  addi r11, r11, -0x34d8
	ctx.r[11].s64 = ctx.r[11].s64 + -13528;
	// 8270C108: 3BEB0290  addi r31, r11, 0x290
	ctx.r[31].s64 = ctx.r[11].s64 + 656;
	// 8270C10C: 3BFFFFA0  addi r31, r31, -0x60
	ctx.r[31].s64 = ctx.r[31].s64 + -96;
	// 8270C110: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8270C114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C118: 419A0008  beq cr6, 0x8270c120
	if ctx.cr[6].eq {
	pc = 0x8270C120; continue 'dispatch;
	}
	// 8270C11C: 4BA10F3D  bl 0x8211d058
	ctx.lr = 0x8270C120;
	sub_8211D058(ctx, base);
	// 8270C120: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8270C124: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8270C128: 4098FFE4  bge cr6, 0x8270c10c
	if !ctx.cr[6].lt {
	pc = 0x8270C10C; continue 'dispatch;
	}
	// 8270C12C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8270C130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270C134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270C138: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8270C13C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270C140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C148 size=20
    let mut pc: u32 = 0x8270C148;
    'dispatch: loop {
        match pc {
            0x8270C148 => {
    //   block [0x8270C148..0x8270C15C)
	// 8270C148: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270C14C: 396BCD68  addi r11, r11, -0x3298
	ctx.r[11].s64 = ctx.r[11].s64 + -12952;
	// 8270C150: 806B0220  lwz r3, 0x220(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(544 as u32) ) } as u64;
	// 8270C154: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C158: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C15C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C15C size=8
    let mut pc: u32 = 0x8270C15C;
    'dispatch: loop {
        match pc {
            0x8270C15C => {
    //   block [0x8270C15C..0x8270C164)
	// 8270C15C: 4BA10EFC  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C168 size=20
    let mut pc: u32 = 0x8270C168;
    'dispatch: loop {
        match pc {
            0x8270C168 => {
    //   block [0x8270C168..0x8270C17C)
	// 8270C168: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8270C16C: 3D408287  lis r10, -0x7d79
	ctx.r[10].s64 = -2105081856;
	// 8270C170: 396B4634  addi r11, r11, 0x4634
	ctx.r[11].s64 = ctx.r[11].s64 + 17972;
	// 8270C174: 916ABAE0  stw r11, -0x4520(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17696 as u32), ctx.r[11].u32 ) };
	// 8270C178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C180 size=12
    let mut pc: u32 = 0x8270C180;
    'dispatch: loop {
        match pc {
            0x8270C180 => {
    //   block [0x8270C180..0x8270C18C)
	// 8270C180: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270C184: 386BDBE4  addi r3, r11, -0x241c
	ctx.r[3].s64 = ctx.r[11].s64 + -9244;
	// 8270C188: 4BAE84A8  b 0x821f4630
	sub_821F4630(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C190 size=40
    let mut pc: u32 = 0x8270C190;
    'dispatch: loop {
        match pc {
            0x8270C190 => {
    //   block [0x8270C190..0x8270C1B8)
	// 8270C190: 3D4082C0  lis r10, -0x7d40
	ctx.r[10].s64 = -2101346304;
	// 8270C194: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8270C198: 392B3088  addi r9, r11, 0x3088
	ctx.r[9].s64 = ctx.r[11].s64 + 12424;
	// 8270C19C: 816AB9F4  lwz r11, -0x460c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17932 as u32) ) } as u64;
	// 8270C1A0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8270C1A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270C1A8: 916AB9F4  stw r11, -0x460c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17932 as u32), ctx.r[11].u32 ) };
	// 8270C1AC: 3D4082CF  lis r10, -0x7d31
	ctx.r[10].s64 = -2100363264;
	// 8270C1B0: 912ADBE0  stw r9, -0x2420(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-9248 as u32), ctx.r[9].u32 ) };
	// 8270C1B4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C1B8 size=20
    let mut pc: u32 = 0x8270C1B8;
    'dispatch: loop {
        match pc {
            0x8270C1B8 => {
    //   block [0x8270C1B8..0x8270C1CC)
	// 8270C1B8: 3D60829E  lis r11, -0x7d62
	ctx.r[11].s64 = -2103574528;
	// 8270C1BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C1C0: 396B63D0  addi r11, r11, 0x63d0
	ctx.r[11].s64 = ctx.r[11].s64 + 25552;
	// 8270C1C4: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8270C1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C1D0 size=12
    let mut pc: u32 = 0x8270C1D0;
    'dispatch: loop {
        match pc {
            0x8270C1D0 => {
    //   block [0x8270C1D0..0x8270C1DC)
	// 8270C1D0: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270C1D4: 386BE3F8  addi r3, r11, -0x1c08
	ctx.r[3].s64 = ctx.r[11].s64 + -7176;
	// 8270C1D8: 4BAF3220  b 0x821ff3f8
	sub_821FF3F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C1E0 size=120
    let mut pc: u32 = 0x8270C1E0;
    'dispatch: loop {
        match pc {
            0x8270C1E0 => {
    //   block [0x8270C1E0..0x8270C258)
	// 8270C1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270C1E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270C1EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C1F0: 3D408287  lis r10, -0x7d79
	ctx.r[10].s64 = -2105081856;
	// 8270C1F4: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8270C1F8: 3BEAE6A0  addi r31, r10, -0x1960
	ctx.r[31].s64 = ctx.r[10].s64 + -6496;
	// 8270C1FC: 396B461C  addi r11, r11, 0x461c
	ctx.r[11].s64 = ctx.r[11].s64 + 17948;
	// 8270C200: 807F0154  lwz r3, 0x154(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(340 as u32) ) } as u64;
	// 8270C204: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8270C208: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8270C20C: 419A0008  beq cr6, 0x8270c214
	if ctx.cr[6].eq {
	pc = 0x8270C214; continue 'dispatch;
	}
	// 8270C210: 4BCB4749  bl 0x823c0958
	ctx.lr = 0x8270C214;
	sub_823C0958(ctx, base);
	// 8270C214: 807F0158  lwz r3, 0x158(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(344 as u32) ) } as u64;
	// 8270C218: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8270C21C: 419A0008  beq cr6, 0x8270c224
	if ctx.cr[6].eq {
	pc = 0x8270C224; continue 'dispatch;
	}
	// 8270C220: 4BCB4739  bl 0x823c0958
	ctx.lr = 0x8270C224;
	sub_823C0958(ctx, base);
	// 8270C224: 807F015C  lwz r3, 0x15c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(348 as u32) ) } as u64;
	// 8270C228: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8270C22C: 419A0008  beq cr6, 0x8270c234
	if ctx.cr[6].eq {
	pc = 0x8270C234; continue 'dispatch;
	}
	// 8270C230: 4BCB4729  bl 0x823c0958
	ctx.lr = 0x8270C234;
	sub_823C0958(ctx, base);
	// 8270C234: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270C238: 396BE1B0  addi r11, r11, -0x1e50
	ctx.r[11].s64 = ctx.r[11].s64 + -7760;
	// 8270C23C: 917F0130  stw r11, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[11].u32 ) };
	// 8270C240: 917F0110  stw r11, 0x110(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(272 as u32), ctx.r[11].u32 ) };
	// 8270C244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270C248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270C24C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270C250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270C254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C258 size=112
    let mut pc: u32 = 0x8270C258;
    'dispatch: loop {
        match pc {
            0x8270C258 => {
    //   block [0x8270C258..0x8270C2C8)
	// 8270C258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C25C: 4BE28E61  bl 0x825350bc
	ctx.lr = 0x8270C260;
	sub_82535080(ctx, base);
	// 8270C260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C264: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270C268: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8270C26C: 396B3200  addi r11, r11, 0x3200
	ctx.r[11].s64 = ctx.r[11].s64 + 12800;
	// 8270C270: 3BEB2DD0  addi r31, r11, 0x2dd0
	ctx.r[31].s64 = ctx.r[11].s64 + 11728;
	// 8270C274: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8270C278: 3BAB4634  addi r29, r11, 0x4634
	ctx.r[29].s64 = ctx.r[11].s64 + 17972;
	// 8270C27C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8270C280: 3BFFEEC0  addi r31, r31, -0x1140
	ctx.r[31].s64 = ctx.r[31].s64 + -4416;
	// 8270C284: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8270C288: 38CB45E0  addi r6, r11, 0x45e0
	ctx.r[6].s64 = ctx.r[11].s64 + 17888;
	// 8270C28C: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 8270C290: 387F0400  addi r3, r31, 0x400
	ctx.r[3].s64 = ctx.r[31].s64 + 1024;
	// 8270C294: 4BA0400D  bl 0x821102a0
	ctx.lr = 0x8270C298;
	sub_821102A0(ctx, base);
	// 8270C298: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8270C29C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8270C2A0: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8270C2A4: 38CB45E0  addi r6, r11, 0x45e0
	ctx.r[6].s64 = ctx.r[11].s64 + 17888;
	// 8270C2A8: 388000B0  li r4, 0xb0
	ctx.r[4].s64 = 176;
	// 8270C2AC: 387FFC90  addi r3, r31, -0x370
	ctx.r[3].s64 = ctx.r[31].s64 + -880;
	// 8270C2B0: 4BA03FF1  bl 0x821102a0
	ctx.lr = 0x8270C2B4;
	sub_821102A0(ctx, base);
	// 8270C2B4: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8270C2B8: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8270C2BC: 4098FFC0  bge cr6, 0x8270c27c
	if !ctx.cr[6].lt {
	pc = 0x8270C27C; continue 'dispatch;
	}
	// 8270C2C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8270C2C4: 4BE28E48  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C2C8 size=100
    let mut pc: u32 = 0x8270C2C8;
    'dispatch: loop {
        match pc {
            0x8270C2C8 => {
    //   block [0x8270C2C8..0x8270C32C)
	// 8270C2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270C2D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8270C2D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270C2D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C2DC: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270C2E0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8270C2E4: 396BEF20  addi r11, r11, -0x10e0
	ctx.r[11].s64 = ctx.r[11].s64 + -4320;
	// 8270C2E8: 3BEB5FA8  addi r31, r11, 0x5fa8
	ctx.r[31].s64 = ctx.r[11].s64 + 24488;
	// 8270C2EC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 8270C2F0: 3BFFDE90  addi r31, r31, -0x2170
	ctx.r[31].s64 = ctx.r[31].s64 + -8560;
	// 8270C2F4: 38A00007  li r5, 7
	ctx.r[5].s64 = 7;
	// 8270C2F8: 38CB45E0  addi r6, r11, 0x45e0
	ctx.r[6].s64 = ctx.r[11].s64 + 17888;
	// 8270C2FC: 388000A8  li r4, 0xa8
	ctx.r[4].s64 = 168;
	// 8270C300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8270C304: 4BA03F9D  bl 0x821102a0
	ctx.lr = 0x8270C308;
	sub_821102A0(ctx, base);
	// 8270C308: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 8270C30C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8270C310: 4098FFDC  bge cr6, 0x8270c2ec
	if !ctx.cr[6].lt {
	pc = 0x8270C2EC; continue 'dispatch;
	}
	// 8270C314: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8270C318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270C31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270C320: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8270C324: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270C328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C330 size=20
    let mut pc: u32 = 0x8270C330;
    'dispatch: loop {
        match pc {
            0x8270C330 => {
    //   block [0x8270C330..0x8270C344)
	// 8270C330: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8270C334: 3D408287  lis r10, -0x7d79
	ctx.r[10].s64 = -2105081856;
	// 8270C338: 396B46E8  addi r11, r11, 0x46e8
	ctx.r[11].s64 = ctx.r[11].s64 + 18152;
	// 8270C33C: 916AE800  stw r11, -0x1800(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6144 as u32), ctx.r[11].u32 ) };
	// 8270C340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C348 size=20
    let mut pc: u32 = 0x8270C348;
    'dispatch: loop {
        match pc {
            0x8270C348 => {
    //   block [0x8270C348..0x8270C35C)
	// 8270C348: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270C34C: 396B5480  addi r11, r11, 0x5480
	ctx.r[11].s64 = ctx.r[11].s64 + 21632;
	// 8270C350: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C354: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C358: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C35C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C35C size=8
    let mut pc: u32 = 0x8270C35C;
    'dispatch: loop {
        match pc {
            0x8270C35C => {
    //   block [0x8270C35C..0x8270C364)
	// 8270C35C: 4BA10CFC  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C368 size=20
    let mut pc: u32 = 0x8270C368;
    'dispatch: loop {
        match pc {
            0x8270C368 => {
    //   block [0x8270C368..0x8270C37C)
	// 8270C368: 3D6082CF  lis r11, -0x7d31
	ctx.r[11].s64 = -2100363264;
	// 8270C36C: 396B548C  addi r11, r11, 0x548c
	ctx.r[11].s64 = ctx.r[11].s64 + 21644;
	// 8270C370: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C378: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C37C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C37C size=8
    let mut pc: u32 = 0x8270C37C;
    'dispatch: loop {
        match pc {
            0x8270C37C => {
    //   block [0x8270C37C..0x8270C384)
	// 8270C37C: 4BA10CDC  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C388 size=24
    let mut pc: u32 = 0x8270C388;
    'dispatch: loop {
        match pc {
            0x8270C388 => {
    //   block [0x8270C388..0x8270C3A0)
	// 8270C388: 3D4082C0  lis r10, -0x7d40
	ctx.r[10].s64 = -2101346304;
	// 8270C38C: 816ABA2C  lwz r11, -0x45d4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17876 as u32) ) } as u64;
	// 8270C390: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8270C394: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270C398: 916ABA2C  stw r11, -0x45d4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17876 as u32), ctx.r[11].u32 ) };
	// 8270C39C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C3A0 size=32
    let mut pc: u32 = 0x8270C3A0;
    'dispatch: loop {
        match pc {
            0x8270C3A0 => {
    //   block [0x8270C3A0..0x8270C3C0)
	// 8270C3A0: 3D60829E  lis r11, -0x7d62
	ctx.r[11].s64 = -2103574528;
	// 8270C3A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C3A8: 396B6570  addi r11, r11, 0x6570
	ctx.r[11].s64 = ctx.r[11].s64 + 25968;
	// 8270C3AC: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8270C3B0: 3D4082C0  lis r10, -0x7d40
	ctx.r[10].s64 = -2101346304;
	// 8270C3B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C3B8: 916ABA28  stw r11, -0x45d8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-17880 as u32), ctx.r[11].u32 ) };
	// 8270C3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C3C0 size=12
    let mut pc: u32 = 0x8270C3C0;
    'dispatch: loop {
        match pc {
            0x8270C3C0 => {
    //   block [0x8270C3C0..0x8270C3CC)
	// 8270C3C0: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 8270C3C4: 386BE558  addi r3, r11, -0x1aa8
	ctx.r[3].s64 = ctx.r[11].s64 + -6824;
	// 8270C3C8: 4BBF06A0  b 0x822fca68
	sub_822FCA68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C3D0 size=80
    let mut pc: u32 = 0x8270C3D0;
    'dispatch: loop {
        match pc {
            0x8270C3D0 => {
    //   block [0x8270C3D0..0x8270C420)
	// 8270C3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270C3D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270C3DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C3E0: 3D608287  lis r11, -0x7d79
	ctx.r[11].s64 = -2105081856;
	// 8270C3E4: 3BEB7864  addi r31, r11, 0x7864
	ctx.r[31].s64 = ctx.r[11].s64 + 30820;
	// 8270C3E8: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 8270C3EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8270C3F0: 396BE088  addi r11, r11, -0x1f78
	ctx.r[11].s64 = ctx.r[11].s64 + -8056;
	// 8270C3F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8270C3F8: 4BAFB9B1  bl 0x82207da8
	ctx.lr = 0x8270C3FC;
	sub_82207DA8(ctx, base);
	// 8270C3FC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8270C400: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C404: 419A0008  beq cr6, 0x8270c40c
	if ctx.cr[6].eq {
	pc = 0x8270C40C; continue 'dispatch;
	}
	// 8270C408: 4BA10C51  bl 0x8211d058
	ctx.lr = 0x8270C40C;
	sub_8211D058(ctx, base);
	// 8270C40C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270C410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270C414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270C418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270C41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C420 size=20
    let mut pc: u32 = 0x8270C420;
    'dispatch: loop {
        match pc {
            0x8270C420 => {
    //   block [0x8270C420..0x8270C434)
	// 8270C420: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 8270C424: 396BF4C8  addi r11, r11, -0xb38
	ctx.r[11].s64 = ctx.r[11].s64 + -2872;
	// 8270C428: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C42C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C430: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C434(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C434 size=8
    let mut pc: u32 = 0x8270C434;
    'dispatch: loop {
        match pc {
            0x8270C434 => {
    //   block [0x8270C434..0x8270C43C)
	// 8270C434: 4BA10C24  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C440 size=20
    let mut pc: u32 = 0x8270C440;
    'dispatch: loop {
        match pc {
            0x8270C440 => {
    //   block [0x8270C440..0x8270C454)
	// 8270C440: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 8270C444: 396BF4BC  addi r11, r11, -0xb44
	ctx.r[11].s64 = ctx.r[11].s64 + -2884;
	// 8270C448: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C44C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C450: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C454(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C454 size=8
    let mut pc: u32 = 0x8270C454;
    'dispatch: loop {
        match pc {
            0x8270C454 => {
    //   block [0x8270C454..0x8270C45C)
	// 8270C454: 4BA10C04  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C460 size=20
    let mut pc: u32 = 0x8270C460;
    'dispatch: loop {
        match pc {
            0x8270C460 => {
    //   block [0x8270C460..0x8270C474)
	// 8270C460: 3D6082D0  lis r11, -0x7d30
	ctx.r[11].s64 = -2100297728;
	// 8270C464: 396B49C8  addi r11, r11, 0x49c8
	ctx.r[11].s64 = ctx.r[11].s64 + 18888;
	// 8270C468: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C46C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C470: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C474(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C474 size=8
    let mut pc: u32 = 0x8270C474;
    'dispatch: loop {
        match pc {
            0x8270C474 => {
    //   block [0x8270C474..0x8270C47C)
	// 8270C474: 4BA10BE4  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C480 size=24
    let mut pc: u32 = 0x8270C480;
    'dispatch: loop {
        match pc {
            0x8270C480 => {
    //   block [0x8270C480..0x8270C498)
	// 8270C480: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270C484: 3D4082D0  lis r10, -0x7d30
	ctx.r[10].s64 = -2100297728;
	// 8270C488: 396BE1B0  addi r11, r11, -0x1e50
	ctx.r[11].s64 = ctx.r[11].s64 + -7760;
	// 8270C48C: 394A4AE8  addi r10, r10, 0x4ae8
	ctx.r[10].s64 = ctx.r[10].s64 + 19176;
	// 8270C490: 916A0024  stw r11, 0x24(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8270C494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C498 size=4
    let mut pc: u32 = 0x8270C498;
    'dispatch: loop {
        match pc {
            0x8270C498 => {
    //   block [0x8270C498..0x8270C49C)
	// 8270C498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4A0 size=20
    let mut pc: u32 = 0x8270C4A0;
    'dispatch: loop {
        match pc {
            0x8270C4A0 => {
    //   block [0x8270C4A0..0x8270C4B4)
	// 8270C4A0: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8270C4A4: 396BB438  addi r11, r11, -0x4bc8
	ctx.r[11].s64 = ctx.r[11].s64 + -19400;
	// 8270C4A8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C4AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C4B0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4B4 size=8
    let mut pc: u32 = 0x8270C4B4;
    'dispatch: loop {
        match pc {
            0x8270C4B4 => {
    //   block [0x8270C4B4..0x8270C4BC)
	// 8270C4B4: 4BA10BA4  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4C0 size=20
    let mut pc: u32 = 0x8270C4C0;
    'dispatch: loop {
        match pc {
            0x8270C4C0 => {
    //   block [0x8270C4C0..0x8270C4D4)
	// 8270C4C0: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8270C4C4: 396BB444  addi r11, r11, -0x4bbc
	ctx.r[11].s64 = ctx.r[11].s64 + -19388;
	// 8270C4C8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C4CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C4D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4D4 size=8
    let mut pc: u32 = 0x8270C4D4;
    'dispatch: loop {
        match pc {
            0x8270C4D4 => {
    //   block [0x8270C4D4..0x8270C4DC)
	// 8270C4D4: 4BA10B84  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4E0 size=20
    let mut pc: u32 = 0x8270C4E0;
    'dispatch: loop {
        match pc {
            0x8270C4E0 => {
    //   block [0x8270C4E0..0x8270C4F4)
	// 8270C4E0: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8270C4E4: 396BB450  addi r11, r11, -0x4bb0
	ctx.r[11].s64 = ctx.r[11].s64 + -19376;
	// 8270C4E8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C4EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C4F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4F4 size=8
    let mut pc: u32 = 0x8270C4F4;
    'dispatch: loop {
        match pc {
            0x8270C4F4 => {
    //   block [0x8270C4F4..0x8270C4FC)
	// 8270C4F4: 4BA10B64  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C500 size=4
    let mut pc: u32 = 0x8270C500;
    'dispatch: loop {
        match pc {
            0x8270C500 => {
    //   block [0x8270C500..0x8270C504)
	// 8270C500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C508 size=20
    let mut pc: u32 = 0x8270C508;
    'dispatch: loop {
        match pc {
            0x8270C508 => {
    //   block [0x8270C508..0x8270C51C)
	// 8270C508: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8270C50C: 396BF4D4  addi r11, r11, -0xb2c
	ctx.r[11].s64 = ctx.r[11].s64 + -2860;
	// 8270C510: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C514: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C518: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C51C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C51C size=8
    let mut pc: u32 = 0x8270C51C;
    'dispatch: loop {
        match pc {
            0x8270C51C => {
    //   block [0x8270C51C..0x8270C524)
	// 8270C51C: 4BA10B3C  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C528 size=20
    let mut pc: u32 = 0x8270C528;
    'dispatch: loop {
        match pc {
            0x8270C528 => {
    //   block [0x8270C528..0x8270C53C)
	// 8270C528: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8270C52C: 396BF4E0  addi r11, r11, -0xb20
	ctx.r[11].s64 = ctx.r[11].s64 + -2848;
	// 8270C530: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C534: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C538: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C53C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C53C size=8
    let mut pc: u32 = 0x8270C53C;
    'dispatch: loop {
        match pc {
            0x8270C53C => {
    //   block [0x8270C53C..0x8270C544)
	// 8270C53C: 4BA10B1C  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C548 size=20
    let mut pc: u32 = 0x8270C548;
    'dispatch: loop {
        match pc {
            0x8270C548 => {
    //   block [0x8270C548..0x8270C55C)
	// 8270C548: 3D60830F  lis r11, -0x7cf1
	ctx.r[11].s64 = -2096168960;
	// 8270C54C: 396BF7EC  addi r11, r11, -0x814
	ctx.r[11].s64 = ctx.r[11].s64 + -2068;
	// 8270C550: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C554: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C558: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C55C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C55C size=8
    let mut pc: u32 = 0x8270C55C;
    'dispatch: loop {
        match pc {
            0x8270C55C => {
    //   block [0x8270C55C..0x8270C564)
	// 8270C55C: 4BA10AFC  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
	// 8270C560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C568 size=4
    let mut pc: u32 = 0x8270C568;
    'dispatch: loop {
        match pc {
            0x8270C568 => {
    //   block [0x8270C568..0x8270C56C)
	// 8270C568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C570 size=20
    let mut pc: u32 = 0x8270C570;
    'dispatch: loop {
        match pc {
            0x8270C570 => {
    //   block [0x8270C570..0x8270C584)
	// 8270C570: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 8270C574: 396BC9F0  addi r11, r11, -0x3610
	ctx.r[11].s64 = ctx.r[11].s64 + -13840;
	// 8270C578: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8270C57C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C580: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C584(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C584 size=12
    let mut pc: u32 = 0x8270C584;
    'dispatch: loop {
        match pc {
            0x8270C584 => {
    //   block [0x8270C584..0x8270C590)
	// 8270C584: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C588: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8270C58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C590 size=4
    let mut pc: u32 = 0x8270C590;
    'dispatch: loop {
        match pc {
            0x8270C590 => {
    //   block [0x8270C590..0x8270C594)
	// 8270C590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C598 size=92
    let mut pc: u32 = 0x8270C598;
    'dispatch: loop {
        match pc {
            0x8270C598 => {
    //   block [0x8270C598..0x8270C5F4)
	// 8270C598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270C5A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270C5A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C5A8: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 8270C5AC: 3BEBEA00  addi r31, r11, -0x1600
	ctx.r[31].s64 = ctx.r[11].s64 + -5632;
	// 8270C5B0: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 8270C5B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270C5B8: 419A0014  beq cr6, 0x8270c5cc
	if ctx.cr[6].eq {
	pc = 0x8270C5CC; continue 'dispatch;
	}
	// 8270C5BC: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8270C5C0: 4BCB4399  bl 0x823c0958
	ctx.lr = 0x8270C5C4;
	sub_823C0958(ctx, base);
	// 8270C5C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C5C8: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 8270C5CC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8270C5D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270C5D4: 419A000C  beq cr6, 0x8270c5e0
	if ctx.cr[6].eq {
	pc = 0x8270C5E0; continue 'dispatch;
	}
	// 8270C5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C5DC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8270C5E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270C5E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270C5E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270C5EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270C5F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C5F8 size=20
    let mut pc: u32 = 0x8270C5F8;
    'dispatch: loop {
        match pc {
            0x8270C5F8 => {
    //   block [0x8270C5F8..0x8270C60C)
	// 8270C5F8: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 8270C5FC: 396BCA80  addi r11, r11, -0x3580
	ctx.r[11].s64 = ctx.r[11].s64 + -13696;
	// 8270C600: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270C604: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C608: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C60C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C60C size=12
    let mut pc: u32 = 0x8270C60C;
    'dispatch: loop {
        match pc {
            0x8270C60C => {
    //   block [0x8270C60C..0x8270C618)
	// 8270C60C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C610: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8270C614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C618 size=92
    let mut pc: u32 = 0x8270C618;
    'dispatch: loop {
        match pc {
            0x8270C618 => {
    //   block [0x8270C618..0x8270C674)
	// 8270C618: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 8270C61C: 396BF080  addi r11, r11, -0xf80
	ctx.r[11].s64 = ctx.r[11].s64 + -3968;
	// 8270C620: 814B161C  lwz r10, 0x161c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5660 as u32) ) } as u64;
	// 8270C624: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C628: 419A000C  beq cr6, 0x8270c634
	if ctx.cr[6].eq {
	pc = 0x8270C634; continue 'dispatch;
	}
	// 8270C62C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C630: 914B161C  stw r10, 0x161c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5660 as u32), ctx.r[10].u32 ) };
	// 8270C634: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8270C638: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 8270C63C: 394B1600  addi r10, r11, 0x1600
	ctx.r[10].s64 = ctx.r[11].s64 + 5632;
	// 8270C640: 390804A4  addi r8, r8, 0x4a4
	ctx.r[8].s64 = ctx.r[8].s64 + 1188;
	// 8270C644: 394AFEE0  addi r10, r10, -0x120
	ctx.r[10].s64 = ctx.r[10].s64 + -288;
	// 8270C648: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8270C64C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8270C650: 4080FFF4  bge 0x8270c644
	if !ctx.cr[0].lt {
	pc = 0x8270C644; continue 'dispatch;
	}
	// 8270C654: 814B009C  lwz r10, 0x9c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 8270C658: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C65C: 419A000C  beq cr6, 0x8270c668
	if ctx.cr[6].eq {
	pc = 0x8270C668; continue 'dispatch;
	}
	// 8270C660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C664: 914B009C  stw r10, 0x9c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 8270C668: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270C66C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C670: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C674(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C674 size=12
    let mut pc: u32 = 0x8270C674;
    'dispatch: loop {
        match pc {
            0x8270C674 => {
    //   block [0x8270C674..0x8270C680)
	// 8270C674: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C678: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8270C67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C680 size=32
    let mut pc: u32 = 0x8270C680;
    'dispatch: loop {
        match pc {
            0x8270C680 => {
    //   block [0x8270C680..0x8270C6A0)
	// 8270C680: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270C684: 394B04E0  addi r10, r11, 0x4e0
	ctx.r[10].s64 = ctx.r[11].s64 + 1248;
	// 8270C688: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 8270C68C: 396B0A00  addi r11, r11, 0xa00
	ctx.r[11].s64 = ctx.r[11].s64 + 2560;
	// 8270C690: 812B0038  lwz r9, 0x38(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 8270C694: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270C698: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8270C69C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C6A0 size=12
    let mut pc: u32 = 0x8270C6A0;
    'dispatch: loop {
        match pc {
            0x8270C6A0 => {
    //   block [0x8270C6A0..0x8270C6AC)
	// 8270C6A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C6A4: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8270C6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C6B0 size=124
    let mut pc: u32 = 0x8270C6B0;
    'dispatch: loop {
        match pc {
            0x8270C6B0 => {
    //   block [0x8270C6B0..0x8270C72C)
	// 8270C6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270C6B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270C6BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C6C0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C6C4: 3BEBD180  addi r31, r11, -0x2e80
	ctx.r[31].s64 = ctx.r[11].s64 + -11904;
	// 8270C6C8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C6CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C6D0: 419A0008  beq cr6, 0x8270c6d8
	if ctx.cr[6].eq {
	pc = 0x8270C6D8; continue 'dispatch;
	}
	// 8270C6D4: 4BC8E9CD  bl 0x8239b0a0
	ctx.lr = 0x8270C6D8;
	sub_8239B0A0(ctx, base);
	// 8270C6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C6DC: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8270C6E0: 817F0078  lwz r11, 0x78(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 8270C6E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270C6E8: 419A0030  beq cr6, 0x8270c718
	if ctx.cr[6].eq {
	pc = 0x8270C718; continue 'dispatch;
	}
	// 8270C6EC: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 8270C6F0: 4BCB4269  bl 0x823c0958
	ctx.lr = 0x8270C6F4;
	sub_823C0958(ctx, base);
	// 8270C6F4: 807F0070  lwz r3, 0x70(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 8270C6F8: 4BCB4261  bl 0x823c0958
	ctx.lr = 0x8270C6FC;
	sub_823C0958(ctx, base);
	// 8270C6FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C700: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 8270C704: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8270C708: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270C70C: 419A000C  beq cr6, 0x8270c718
	if ctx.cr[6].eq {
	pc = 0x8270C718; continue 'dispatch;
	}
	// 8270C710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C714: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 8270C718: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270C71C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270C720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270C724: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270C728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C730 size=32
    let mut pc: u32 = 0x8270C730;
    'dispatch: loop {
        match pc {
            0x8270C730 => {
    //   block [0x8270C730..0x8270C750)
	// 8270C730: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270C734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270C738: 394B0508  addi r10, r11, 0x508
	ctx.r[10].s64 = ctx.r[11].s64 + 1288;
	// 8270C73C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270C740: 396B9000  addi r11, r11, -0x7000
	ctx.r[11].s64 = ctx.r[11].s64 + -28672;
	// 8270C744: 912B0028  stw r9, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8270C748: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270C74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C750 size=32
    let mut pc: u32 = 0x8270C750;
    'dispatch: loop {
        match pc {
            0x8270C750 => {
    //   block [0x8270C750..0x8270C770)
	// 8270C750: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270C754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270C758: 394B0508  addi r10, r11, 0x508
	ctx.r[10].s64 = ctx.r[11].s64 + 1288;
	// 8270C75C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270C760: 396B9080  addi r11, r11, -0x6f80
	ctx.r[11].s64 = ctx.r[11].s64 + -28544;
	// 8270C764: 912B0028  stw r9, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8270C768: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270C76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C770 size=20
    let mut pc: u32 = 0x8270C770;
    'dispatch: loop {
        match pc {
            0x8270C770 => {
    //   block [0x8270C770..0x8270C784)
	// 8270C770: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C774: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C778: 396B9100  addi r11, r11, -0x6f00
	ctx.r[11].s64 = ctx.r[11].s64 + -28416;
	// 8270C77C: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8270C780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C788 size=32
    let mut pc: u32 = 0x8270C788;
    'dispatch: loop {
        match pc {
            0x8270C788 => {
    //   block [0x8270C788..0x8270C7A8)
	// 8270C788: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270C78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270C790: 394B0508  addi r10, r11, 0x508
	ctx.r[10].s64 = ctx.r[11].s64 + 1288;
	// 8270C794: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270C798: 396B9100  addi r11, r11, -0x6f00
	ctx.r[11].s64 = ctx.r[11].s64 + -28416;
	// 8270C79C: 912B0028  stw r9, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 8270C7A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270C7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C7A8 size=20
    let mut pc: u32 = 0x8270C7A8;
    'dispatch: loop {
        match pc {
            0x8270C7A8 => {
    //   block [0x8270C7A8..0x8270C7BC)
	// 8270C7A8: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 8270C7AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C7B0: 396B1B00  addi r11, r11, 0x1b00
	ctx.r[11].s64 = ctx.r[11].s64 + 6912;
	// 8270C7B4: 914B0310  stw r10, 0x310(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(784 as u32), ctx.r[10].u32 ) };
	// 8270C7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C7C0 size=96
    let mut pc: u32 = 0x8270C7C0;
    'dispatch: loop {
        match pc {
            0x8270C7C0 => {
    //   block [0x8270C7C0..0x8270C820)
	// 8270C7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270C7C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270C7CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C7D0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C7D4: 3BEB9000  addi r31, r11, -0x7000
	ctx.r[31].s64 = ctx.r[11].s64 + -28672;
	// 8270C7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C7DC: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8270C7E0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8270C7E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8270C7E8: 419A0024  beq cr6, 0x8270c80c
	if ctx.cr[6].eq {
	pc = 0x8270C80C; continue 'dispatch;
	}
	// 8270C7EC: 4BC9421D  bl 0x823a0a08
	ctx.lr = 0x8270C7F0;
	sub_823A0A08(ctx, base);
	// 8270C7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C7F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8270C7F8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8270C7FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270C800: 419A000C  beq cr6, 0x8270c80c
	if ctx.cr[6].eq {
	pc = 0x8270C80C; continue 'dispatch;
	}
	// 8270C804: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C808: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8270C80C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270C810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270C814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270C818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270C81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C820 size=12
    let mut pc: u32 = 0x8270C820;
    'dispatch: loop {
        match pc {
            0x8270C820 => {
    //   block [0x8270C820..0x8270C82C)
	// 8270C820: 3D608310  lis r11, -0x7cf0
	ctx.r[11].s64 = -2096103424;
	// 8270C824: 386B1E80  addi r3, r11, 0x1e80
	ctx.r[3].s64 = ctx.r[11].s64 + 7808;
	// 8270C828: 4BC92A50  b 0x8239f278
	sub_8239F278(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C830 size=4
    let mut pc: u32 = 0x8270C830;
    'dispatch: loop {
        match pc {
            0x8270C830 => {
    //   block [0x8270C830..0x8270C834)
	// 8270C830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C838 size=84
    let mut pc: u32 = 0x8270C838;
    'dispatch: loop {
        match pc {
            0x8270C838 => {
    //   block [0x8270C838..0x8270C88C)
	// 8270C838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270C83C: 4BE28881  bl 0x825350bc
	ctx.lr = 0x8270C840;
	sub_82535080(ctx, base);
	// 8270C840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270C844: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C848: 3BCBF480  addi r30, r11, -0xb80
	ctx.r[30].s64 = ctx.r[11].s64 + -2944;
	// 8270C84C: 83BE0004  lwz r29, 4(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270C850: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8270C854: 419A0030  beq cr6, 0x8270c884
	if ctx.cr[6].eq {
	pc = 0x8270C884; continue 'dispatch;
	}
	// 8270C858: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C85C: 3BEB2200  addi r31, r11, 0x2200
	ctx.r[31].s64 = ctx.r[11].s64 + 8704;
	// 8270C860: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8270C864: 480009F9  bl 0x8270d25c
	ctx.lr = 0x8270C868;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 8270C868: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8270C86C: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8270C870: 4BC9E671  bl 0x823aaee0
	ctx.lr = 0x8270C874;
	sub_823AAEE0(ctx, base);
	// 8270C874: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8270C878: 480009F5  bl 0x8270d26c
	ctx.lr = 0x8270C87C;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 8270C87C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270C880: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8270C884: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8270C888: 4BE28884  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C890 size=40
    let mut pc: u32 = 0x8270C890;
    'dispatch: loop {
        match pc {
            0x8270C890 => {
    //   block [0x8270C890..0x8270C8B8)
	// 8270C890: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270C894: 396B9180  addi r11, r11, -0x6e80
	ctx.r[11].s64 = ctx.r[11].s64 + -28288;
	// 8270C898: 814B009C  lwz r10, 0x9c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 8270C89C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C8A0: 419A000C  beq cr6, 0x8270c8ac
	if ctx.cr[6].eq {
	pc = 0x8270C8AC; continue 'dispatch;
	}
	// 8270C8A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C8A8: 914B009C  stw r10, 0x9c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 8270C8AC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270C8B0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C8B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C8B8 size=12
    let mut pc: u32 = 0x8270C8B8;
    'dispatch: loop {
        match pc {
            0x8270C8B8 => {
    //   block [0x8270C8B8..0x8270C8C4)
	// 8270C8B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C8BC: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8270C8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C8C8 size=20
    let mut pc: u32 = 0x8270C8C8;
    'dispatch: loop {
        match pc {
            0x8270C8C8 => {
    //   block [0x8270C8C8..0x8270C8DC)
	// 8270C8C8: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C8CC: 396BF500  addi r11, r11, -0xb00
	ctx.r[11].s64 = ctx.r[11].s64 + -2816;
	// 8270C8D0: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8270C8D4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C8D8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C8DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C8DC size=12
    let mut pc: u32 = 0x8270C8DC;
    'dispatch: loop {
        match pc {
            0x8270C8DC => {
    //   block [0x8270C8DC..0x8270C8E8)
	// 8270C8DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C8E0: 914B0028  stw r10, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8270C8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C8E8 size=44
    let mut pc: u32 = 0x8270C8E8;
    'dispatch: loop {
        match pc {
            0x8270C8E8 => {
    //   block [0x8270C8E8..0x8270C914)
	// 8270C8E8: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C8EC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 8270C8F0: 396BD280  addi r11, r11, -0x2d80
	ctx.r[11].s64 = ctx.r[11].s64 + -11648;
	// 8270C8F4: 3940000F  li r10, 0xf
	ctx.r[10].s64 = 15;
	// 8270C8F8: 396B2180  addi r11, r11, 0x2180
	ctx.r[11].s64 = ctx.r[11].s64 + 8576;
	// 8270C8FC: 3929053C  addi r9, r9, 0x53c
	ctx.r[9].s64 = ctx.r[9].s64 + 1340;
	// 8270C900: 396BFDE8  addi r11, r11, -0x218
	ctx.r[11].s64 = ctx.r[11].s64 + -536;
	// 8270C904: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8270C908: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8270C90C: 4080FFF4  bge 0x8270c900
	if !ctx.cr[0].lt {
	pc = 0x8270C900; continue 'dispatch;
	}
	// 8270C910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C918 size=80
    let mut pc: u32 = 0x8270C918;
    'dispatch: loop {
        match pc {
            0x8270C918 => {
    //   block [0x8270C918..0x8270C968)
	// 8270C918: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270C91C: 396B9580  addi r11, r11, -0x6a80
	ctx.r[11].s64 = ctx.r[11].s64 + -27264;
	// 8270C920: 814B03BC  lwz r10, 0x3bc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(956 as u32) ) } as u64;
	// 8270C924: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C928: 419A000C  beq cr6, 0x8270c934
	if ctx.cr[6].eq {
	pc = 0x8270C934; continue 'dispatch;
	}
	// 8270C92C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C930: 914B03BC  stw r10, 0x3bc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(956 as u32), ctx.r[10].u32 ) };
	// 8270C934: 814B039C  lwz r10, 0x39c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(924 as u32) ) } as u64;
	// 8270C938: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C93C: 419A000C  beq cr6, 0x8270c948
	if ctx.cr[6].eq {
	pc = 0x8270C948; continue 'dispatch;
	}
	// 8270C940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C944: 914B039C  stw r10, 0x39c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(924 as u32), ctx.r[10].u32 ) };
	// 8270C948: 814B009C  lwz r10, 0x9c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 8270C94C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C950: 419A000C  beq cr6, 0x8270c95c
	if ctx.cr[6].eq {
	pc = 0x8270C95C; continue 'dispatch;
	}
	// 8270C954: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C958: 914B009C  stw r10, 0x9c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 8270C95C: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270C960: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C964: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C968 size=12
    let mut pc: u32 = 0x8270C968;
    'dispatch: loop {
        match pc {
            0x8270C968 => {
    //   block [0x8270C968..0x8270C974)
	// 8270C968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C96C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8270C970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C978 size=20
    let mut pc: u32 = 0x8270C978;
    'dispatch: loop {
        match pc {
            0x8270C978 => {
    //   block [0x8270C978..0x8270C98C)
	// 8270C978: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C97C: 396BFE00  addi r11, r11, -0x200
	ctx.r[11].s64 = ctx.r[11].s64 + -512;
	// 8270C980: 814B0068  lwz r10, 0x68(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(104 as u32) ) } as u64;
	// 8270C984: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C988: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C98C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C98C size=12
    let mut pc: u32 = 0x8270C98C;
    'dispatch: loop {
        match pc {
            0x8270C98C => {
    //   block [0x8270C98C..0x8270C998)
	// 8270C98C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C990: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 8270C994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C998 size=4
    let mut pc: u32 = 0x8270C998;
    'dispatch: loop {
        match pc {
            0x8270C998 => {
    //   block [0x8270C998..0x8270C99C)
	// 8270C998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C9A0 size=20
    let mut pc: u32 = 0x8270C9A0;
    'dispatch: loop {
        match pc {
            0x8270C9A0 => {
    //   block [0x8270C9A0..0x8270C9B4)
	// 8270C9A0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C9A4: 396B0080  addi r11, r11, 0x80
	ctx.r[11].s64 = ctx.r[11].s64 + 128;
	// 8270C9A8: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8270C9AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C9B0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C9B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C9B4 size=12
    let mut pc: u32 = 0x8270C9B4;
    'dispatch: loop {
        match pc {
            0x8270C9B4 => {
    //   block [0x8270C9B4..0x8270C9C0)
	// 8270C9B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C9B8: 914B002C  stw r10, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8270C9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C9C0 size=20
    let mut pc: u32 = 0x8270C9C0;
    'dispatch: loop {
        match pc {
            0x8270C9C0 => {
    //   block [0x8270C9C0..0x8270C9D4)
	// 8270C9C0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C9C4: 396B0000  addi r11, r11, 0
	ctx.r[11].s64 = ctx.r[11].s64 + 0;
	// 8270C9C8: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8270C9CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C9D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C9D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C9D4 size=12
    let mut pc: u32 = 0x8270C9D4;
    'dispatch: loop {
        match pc {
            0x8270C9D4 => {
    //   block [0x8270C9D4..0x8270C9E0)
	// 8270C9D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C9D8: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8270C9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C9E0 size=20
    let mut pc: u32 = 0x8270C9E0;
    'dispatch: loop {
        match pc {
            0x8270C9E0 => {
    //   block [0x8270C9E0..0x8270C9F4)
	// 8270C9E0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270C9E4: 396BFF80  addi r11, r11, -0x80
	ctx.r[11].s64 = ctx.r[11].s64 + -128;
	// 8270C9E8: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8270C9EC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C9F0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C9F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C9F4 size=12
    let mut pc: u32 = 0x8270C9F4;
    'dispatch: loop {
        match pc {
            0x8270C9F4 => {
    //   block [0x8270C9F4..0x8270CA00)
	// 8270C9F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270C9F8: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8270C9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CA00 size=20
    let mut pc: u32 = 0x8270CA00;
    'dispatch: loop {
        match pc {
            0x8270CA00 => {
    //   block [0x8270CA00..0x8270CA14)
	// 8270CA00: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CA04: 396BFF00  addi r11, r11, -0x100
	ctx.r[11].s64 = ctx.r[11].s64 + -256;
	// 8270CA08: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8270CA0C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270CA10: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CA14 size=12
    let mut pc: u32 = 0x8270CA14;
    'dispatch: loop {
        match pc {
            0x8270CA14 => {
    //   block [0x8270CA14..0x8270CA20)
	// 8270CA14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270CA18: 914B0028  stw r10, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 8270CA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CA20 size=20
    let mut pc: u32 = 0x8270CA20;
    'dispatch: loop {
        match pc {
            0x8270CA20 => {
    //   block [0x8270CA20..0x8270CA34)
	// 8270CA20: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CA24: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 8270CA28: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8270CA2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270CA30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CA34 size=12
    let mut pc: u32 = 0x8270CA34;
    'dispatch: loop {
        match pc {
            0x8270CA34 => {
    //   block [0x8270CA34..0x8270CA40)
	// 8270CA34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270CA38: 914B002C  stw r10, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8270CA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CA40 size=4
    let mut pc: u32 = 0x8270CA40;
    'dispatch: loop {
        match pc {
            0x8270CA40 => {
    //   block [0x8270CA40..0x8270CA44)
	// 8270CA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CA48 size=12
    let mut pc: u32 = 0x8270CA48;
    'dispatch: loop {
        match pc {
            0x8270CA48 => {
    //   block [0x8270CA48..0x8270CA54)
	// 8270CA48: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270CA4C: 386B9980  addi r3, r11, -0x6680
	ctx.r[3].s64 = ctx.r[11].s64 + -26240;
	// 8270CA50: 4BC97260  b 0x823a3cb0
	sub_823A3CB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270CA58 size=92
    let mut pc: u32 = 0x8270CA58;
    'dispatch: loop {
        match pc {
            0x8270CA58 => {
    //   block [0x8270CA58..0x8270CAB4)
	// 8270CA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270CA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270CA60: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270CA64: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270CA68: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CA6C: 3BEB0700  addi r31, r11, 0x700
	ctx.r[31].s64 = ctx.r[11].s64 + 1792;
	// 8270CA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8270CA74: 4BC97B75  bl 0x823a45e8
	ctx.lr = 0x8270CA78;
	sub_823A45E8(ctx, base);
	// 8270CA78: 817F07C4  lwz r11, 0x7c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1988 as u32) ) } as u64;
	// 8270CA7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270CA80: 419A000C  beq cr6, 0x8270ca8c
	if ctx.cr[6].eq {
	pc = 0x8270CA8C; continue 'dispatch;
	}
	// 8270CA84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270CA88: 917F07C4  stw r11, 0x7c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1988 as u32), ctx.r[11].u32 ) };
	// 8270CA8C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8270CA90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270CA94: 419A000C  beq cr6, 0x8270caa0
	if ctx.cr[6].eq {
	pc = 0x8270CAA0; continue 'dispatch;
	}
	// 8270CA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270CA9C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8270CAA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270CAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270CAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270CAAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270CAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAB8 size=4
    let mut pc: u32 = 0x8270CAB8;
    'dispatch: loop {
        match pc {
            0x8270CAB8 => {
    //   block [0x8270CAB8..0x8270CABC)
	// 8270CAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAC0 size=4
    let mut pc: u32 = 0x8270CAC0;
    'dispatch: loop {
        match pc {
            0x8270CAC0 => {
    //   block [0x8270CAC0..0x8270CAC4)
	// 8270CAC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAC8 size=20
    let mut pc: u32 = 0x8270CAC8;
    'dispatch: loop {
        match pc {
            0x8270CAC8 => {
    //   block [0x8270CAC8..0x8270CADC)
	// 8270CAC8: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CACC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270CAD0: 396B0200  addi r11, r11, 0x200
	ctx.r[11].s64 = ctx.r[11].s64 + 512;
	// 8270CAD4: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 8270CAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAE0 size=4
    let mut pc: u32 = 0x8270CAE0;
    'dispatch: loop {
        match pc {
            0x8270CAE0 => {
    //   block [0x8270CAE0..0x8270CAE4)
	// 8270CAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAE8 size=4
    let mut pc: u32 = 0x8270CAE8;
    'dispatch: loop {
        match pc {
            0x8270CAE8 => {
    //   block [0x8270CAE8..0x8270CAEC)
	// 8270CAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CAF0 size=12
    let mut pc: u32 = 0x8270CAF0;
    'dispatch: loop {
        match pc {
            0x8270CAF0 => {
    //   block [0x8270CAF0..0x8270CAFC)
	// 8270CAF0: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CAF4: 386B1100  addi r3, r11, 0x1100
	ctx.r[3].s64 = ctx.r[11].s64 + 4352;
	// 8270CAF8: 4BC9B530  b 0x823a8028
	sub_823A8028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CB00 size=4
    let mut pc: u32 = 0x8270CB00;
    'dispatch: loop {
        match pc {
            0x8270CB00 => {
    //   block [0x8270CB00..0x8270CB04)
	// 8270CB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270CB08 size=64
    let mut pc: u32 = 0x8270CB08;
    'dispatch: loop {
        match pc {
            0x8270CB08 => {
    //   block [0x8270CB08..0x8270CB48)
	// 8270CB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270CB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270CB10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270CB14: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270CB18: 38CB9B00  addi r6, r11, -0x6500
	ctx.r[6].s64 = ctx.r[11].s64 + -25856;
	// 8270CB1C: 38660024  addi r3, r6, 0x24
	ctx.r[3].s64 = ctx.r[6].s64 + 36;
	// 8270CB20: 4BC9B7C9  bl 0x823a82e8
	ctx.lr = 0x8270CB24;
	sub_823A82E8(ctx, base);
	// 8270CB24: 81660020  lwz r11, 0x20(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270CB28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270CB2C: 419A000C  beq cr6, 0x8270cb38
	if ctx.cr[6].eq {
	pc = 0x8270CB38; continue 'dispatch;
	}
	// 8270CB30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270CB34: 91660020  stw r11, 0x20(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8270CB38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8270CB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270CB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270CB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CB48 size=20
    let mut pc: u32 = 0x8270CB48;
    'dispatch: loop {
        match pc {
            0x8270CB48 => {
    //   block [0x8270CB48..0x8270CB5C)
	// 8270CB48: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CB4C: 396B2200  addi r11, r11, 0x2200
	ctx.r[11].s64 = ctx.r[11].s64 + 8704;
	// 8270CB50: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270CB54: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270CB58: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CB5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CB5C size=12
    let mut pc: u32 = 0x8270CB5C;
    'dispatch: loop {
        match pc {
            0x8270CB5C => {
    //   block [0x8270CB5C..0x8270CB68)
	// 8270CB5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270CB60: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8270CB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270CB68 size=132
    let mut pc: u32 = 0x8270CB68;
    'dispatch: loop {
        match pc {
            0x8270CB68 => {
    //   block [0x8270CB68..0x8270CBEC)
	// 8270CB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270CB6C: 4BE28551  bl 0x825350bc
	ctx.lr = 0x8270CB70;
	sub_82535080(ctx, base);
	// 8270CB70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270CB74: 3D608311  lis r11, -0x7cef
	ctx.r[11].s64 = -2096037888;
	// 8270CB78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8270CB7C: 3BCB1380  addi r30, r11, 0x1380
	ctx.r[30].s64 = ctx.r[11].s64 + 4992;
	// 8270CB80: 83FE0E30  lwz r31, 0xe30(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(3632 as u32) ) } as u64;
	// 8270CB84: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8270CB88: 419A0048  beq cr6, 0x8270cbd0
	if ctx.cr[6].eq {
	pc = 0x8270CBD0; continue 'dispatch;
	}
	// 8270CB8C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270CB90: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8270CB94: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8270CB98: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8270CB9C: 93E10058  stw r31, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[31].u32 ) };
	// 8270CBA0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8270CBA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270CBA8: E8C10050  ld r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8270CBAC: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 8270CBB0: E8A10058  ld r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8270CBB4: 4BC93EAD  bl 0x823a0a60
	ctx.lr = 0x8270CBB8;
	sub_823A0A60(ctx, base);
	// 8270CBB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8270CBBC: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270CBC0: 387F0001  addi r3, r31, 1
	ctx.r[3].s64 = ctx.r[31].s64 + 1;
	// 8270CBC4: 4BC93FD5  bl 0x823a0b98
	ctx.lr = 0x8270CBC8;
	sub_823A0B98(ctx, base);
	// 8270CBC8: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8270CBCC: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8270CBD0: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270CBD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8270CBD8: 419A000C  beq cr6, 0x8270cbe4
	if ctx.cr[6].eq {
	pc = 0x8270CBE4; continue 'dispatch;
	}
	// 8270CBDC: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 8270CBE0: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8270CBE4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8270CBE8: 4BE28524  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CBF0 size=20
    let mut pc: u32 = 0x8270CBF0;
    'dispatch: loop {
        match pc {
            0x8270CBF0 => {
    //   block [0x8270CBF0..0x8270CC04)
	// 8270CBF0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8270CBF4: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 8270CBF8: 396B1638  addi r11, r11, 0x1638
	ctx.r[11].s64 = ctx.r[11].s64 + 5688;
	// 8270CBFC: 916AA200  stw r11, -0x5e00(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24064 as u32), ctx.r[11].u32 ) };
	// 8270CC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC08 size=12
    let mut pc: u32 = 0x8270CC08;
    'dispatch: loop {
        match pc {
            0x8270CC08 => {
    //   block [0x8270CC08..0x8270CC14)
	// 8270CC08: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270CC0C: 386BDC90  addi r3, r11, -0x2370
	ctx.r[3].s64 = ctx.r[11].s64 + -9072;
	// 8270CC10: 4BCC03F0  b 0x823cd000
	sub_823CD000(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC18 size=32
    let mut pc: u32 = 0x8270CC18;
    'dispatch: loop {
        match pc {
            0x8270CC18 => {
    //   block [0x8270CC18..0x8270CC38)
	// 8270CC18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CC1C: 394BBD38  addi r10, r11, -0x42c8
	ctx.r[10].s64 = ctx.r[11].s64 + -17096;
	// 8270CC20: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270CC24: 396B3810  addi r11, r11, 0x3810
	ctx.r[11].s64 = ctx.r[11].s64 + 14352;
	// 8270CC28: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8270CC2C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270CC30: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8270CC34: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC38 size=8
    let mut pc: u32 = 0x8270CC38;
    'dispatch: loop {
        match pc {
            0x8270CC38 => {
    //   block [0x8270CC38..0x8270CC40)
	// 8270CC38: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8270CC3C: 4BE270FC  b 0x82533d38
	sub_82533D38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC40 size=4
    let mut pc: u32 = 0x8270CC40;
    'dispatch: loop {
        match pc {
            0x8270CC40 => {
    //   block [0x8270CC40..0x8270CC44)
	// 8270CC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC48 size=4
    let mut pc: u32 = 0x8270CC48;
    'dispatch: loop {
        match pc {
            0x8270CC48 => {
    //   block [0x8270CC48..0x8270CC4C)
	// 8270CC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CC50 size=4
    let mut pc: u32 = 0x8270CC50;
    'dispatch: loop {
        match pc {
            0x8270CC50 => {
    //   block [0x8270CC50..0x8270CC54)
	// 8270CC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270CC58 size=84
    let mut pc: u32 = 0x8270CC58;
    'dispatch: loop {
        match pc {
            0x8270CC58 => {
    //   block [0x8270CC58..0x8270CCAC)
	// 8270CC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270CC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270CC60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8270CC64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8270CC68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270CC6C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8270CC70: 3BC00007  li r30, 7
	ctx.r[30].s64 = 7;
	// 8270CC74: 396B5870  addi r11, r11, 0x5870
	ctx.r[11].s64 = ctx.r[11].s64 + 22640;
	// 8270CC78: 3D6B0001  addis r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 65536;
	// 8270CC7C: 3BEB9B5C  addi r31, r11, -0x64a4
	ctx.r[31].s64 = ctx.r[11].s64 + -25764;
	// 8270CC80: 3BFFECD4  addi r31, r31, -0x132c
	ctx.r[31].s64 = ctx.r[31].s64 + -4908;
	// 8270CC84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8270CC88: 4BBA62F9  bl 0x822b2f80
	ctx.lr = 0x8270CC8C;
	sub_822B2F80(ctx, base);
	// 8270CC8C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8270CC90: 4080FFF0  bge 0x8270cc80
	if !ctx.cr[0].lt {
	pc = 0x8270CC80; continue 'dispatch;
	}
	// 8270CC94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8270CC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270CC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270CCA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8270CCA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8270CCA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CCB0 size=16
    let mut pc: u32 = 0x8270CCB0;
    'dispatch: loop {
        match pc {
            0x8270CCB0 => {
    //   block [0x8270CCB0..0x8270CCC0)
	// 8270CCB0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8270CCB4: 396B9198  addi r11, r11, -0x6e68
	ctx.r[11].s64 = ctx.r[11].s64 + -28264;
	// 8270CCB8: 386B0028  addi r3, r11, 0x28
	ctx.r[3].s64 = ctx.r[11].s64 + 40;
	// 8270CCBC: 4BD59664  b 0x82466320
	sub_82466320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CCC0 size=20
    let mut pc: u32 = 0x8270CCC0;
    'dispatch: loop {
        match pc {
            0x8270CCC0 => {
    //   block [0x8270CCC0..0x8270CCD4)
	// 8270CCC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CCC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270CCC8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CCCC: 916A91F8  stw r11, -0x6e08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28168 as u32), ctx.r[11].u32 ) };
	// 8270CCD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CCD8 size=20
    let mut pc: u32 = 0x8270CCD8;
    'dispatch: loop {
        match pc {
            0x8270CCD8 => {
    //   block [0x8270CCD8..0x8270CCEC)
	// 8270CCD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CCDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270CCE0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CCE4: 916A92D8  stw r11, -0x6d28(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27944 as u32), ctx.r[11].u32 ) };
	// 8270CCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CCF0 size=20
    let mut pc: u32 = 0x8270CCF0;
    'dispatch: loop {
        match pc {
            0x8270CCF0 => {
    //   block [0x8270CCF0..0x8270CD04)
	// 8270CCF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CCF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270CCF8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CCFC: 916AEC64  stw r11, -0x139c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-5020 as u32), ctx.r[11].u32 ) };
	// 8270CD00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD08 size=20
    let mut pc: u32 = 0x8270CD08;
    'dispatch: loop {
        match pc {
            0x8270CD08 => {
    //   block [0x8270CD08..0x8270CD1C)
	// 8270CD08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270CD10: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD14: 916A452C  stw r11, 0x452c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(17708 as u32), ctx.r[11].u32 ) };
	// 8270CD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD20 size=20
    let mut pc: u32 = 0x8270CD20;
    'dispatch: loop {
        match pc {
            0x8270CD20 => {
    //   block [0x8270CD20..0x8270CD34)
	// 8270CD20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8270CD28: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD2C: 916A90D4  stw r11, -0x6f2c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28460 as u32), ctx.r[11].u32 ) };
	// 8270CD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD38 size=20
    let mut pc: u32 = 0x8270CD38;
    'dispatch: loop {
        match pc {
            0x8270CD38 => {
    //   block [0x8270CD38..0x8270CD4C)
	// 8270CD38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8270CD40: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD44: 916AE3CC  stw r11, -0x1c34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-7220 as u32), ctx.r[11].u32 ) };
	// 8270CD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD50 size=20
    let mut pc: u32 = 0x8270CD50;
    'dispatch: loop {
        match pc {
            0x8270CD50 => {
    //   block [0x8270CD50..0x8270CD64)
	// 8270CD50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8270CD58: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD5C: 916A1714  stw r11, 0x1714(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(5908 as u32), ctx.r[11].u32 ) };
	// 8270CD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD68 size=20
    let mut pc: u32 = 0x8270CD68;
    'dispatch: loop {
        match pc {
            0x8270CD68 => {
    //   block [0x8270CD68..0x8270CD7C)
	// 8270CD68: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8270CD70: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD74: 916A595C  stw r11, 0x595c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(22876 as u32), ctx.r[11].u32 ) };
	// 8270CD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD80 size=20
    let mut pc: u32 = 0x8270CD80;
    'dispatch: loop {
        match pc {
            0x8270CD80 => {
    //   block [0x8270CD80..0x8270CD94)
	// 8270CD80: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8270CD88: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CD8C: 916A94E4  stw r11, -0x6b1c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-27420 as u32), ctx.r[11].u32 ) };
	// 8270CD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CD98 size=20
    let mut pc: u32 = 0x8270CD98;
    'dispatch: loop {
        match pc {
            0x8270CD98 => {
    //   block [0x8270CD98..0x8270CDAC)
	// 8270CD98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CD9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8270CDA0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CDA4: 916AE5CC  stw r11, -0x1a34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6708 as u32), ctx.r[11].u32 ) };
	// 8270CDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CDB0 size=20
    let mut pc: u32 = 0x8270CDB0;
    'dispatch: loop {
        match pc {
            0x8270CDB0 => {
    //   block [0x8270CDB0..0x8270CDC4)
	// 8270CDB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CDB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8270CDB8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CDBC: 916A1164  stw r11, 0x1164(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4452 as u32), ctx.r[11].u32 ) };
	// 8270CDC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CDC8 size=20
    let mut pc: u32 = 0x8270CDC8;
    'dispatch: loop {
        match pc {
            0x8270CDC8 => {
    //   block [0x8270CDC8..0x8270CDDC)
	// 8270CDC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CDCC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8270CDD0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CDD4: 916A53B0  stw r11, 0x53b0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(21424 as u32), ctx.r[11].u32 ) };
	// 8270CDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CDE0 size=20
    let mut pc: u32 = 0x8270CDE0;
    'dispatch: loop {
        match pc {
            0x8270CDE0 => {
    //   block [0x8270CDE0..0x8270CDF4)
	// 8270CDE0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CDE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8270CDE8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CDEC: 916AA6A8  stw r11, -0x5958(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-22872 as u32), ctx.r[11].u32 ) };
	// 8270CDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CDF8 size=20
    let mut pc: u32 = 0x8270CDF8;
    'dispatch: loop {
        match pc {
            0x8270CDF8 => {
    //   block [0x8270CDF8..0x8270CE0C)
	// 8270CDF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CDFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8270CE00: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE04: 916AF9A0  stw r11, -0x660(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-1632 as u32), ctx.r[11].u32 ) };
	// 8270CE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE10 size=20
    let mut pc: u32 = 0x8270CE10;
    'dispatch: loop {
        match pc {
            0x8270CE10 => {
    //   block [0x8270CE10..0x8270CE24)
	// 8270CE10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8270CE18: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE1C: 916A2D48  stw r11, 0x2d48(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(11592 as u32), ctx.r[11].u32 ) };
	// 8270CE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE28 size=20
    let mut pc: u32 = 0x8270CE28;
    'dispatch: loop {
        match pc {
            0x8270CE28 => {
    //   block [0x8270CE28..0x8270CE3C)
	// 8270CE28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8270CE30: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE34: 916A5850  stw r11, 0x5850(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(22608 as u32), ctx.r[11].u32 ) };
	// 8270CE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE40 size=20
    let mut pc: u32 = 0x8270CE40;
    'dispatch: loop {
        match pc {
            0x8270CE40 => {
    //   block [0x8270CE40..0x8270CE54)
	// 8270CE40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8270CE48: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE4C: 916AA3F8  stw r11, -0x5c08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23560 as u32), ctx.r[11].u32 ) };
	// 8270CE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE58 size=20
    let mut pc: u32 = 0x8270CE58;
    'dispatch: loop {
        match pc {
            0x8270CE58 => {
    //   block [0x8270CE58..0x8270CE6C)
	// 8270CE58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8270CE60: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE64: 916AD7A0  stw r11, -0x2860(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-10336 as u32), ctx.r[11].u32 ) };
	// 8270CE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE70 size=20
    let mut pc: u32 = 0x8270CE70;
    'dispatch: loop {
        match pc {
            0x8270CE70 => {
    //   block [0x8270CE70..0x8270CE84)
	// 8270CE70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8270CE78: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE7C: 916A0B18  stw r11, 0xb18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(2840 as u32), ctx.r[11].u32 ) };
	// 8270CE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CE88 size=20
    let mut pc: u32 = 0x8270CE88;
    'dispatch: loop {
        match pc {
            0x8270CE88 => {
    //   block [0x8270CE88..0x8270CE9C)
	// 8270CE88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CE8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8270CE90: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CE94: 916A6620  stw r11, 0x6620(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26144 as u32), ctx.r[11].u32 ) };
	// 8270CE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CEA0 size=20
    let mut pc: u32 = 0x8270CEA0;
    'dispatch: loop {
        match pc {
            0x8270CEA0 => {
    //   block [0x8270CEA0..0x8270CEB4)
	// 8270CEA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CEA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 8270CEA8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CEAC: 916AC0F8  stw r11, -0x3f08(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-16136 as u32), ctx.r[11].u32 ) };
	// 8270CEB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CEB8 size=20
    let mut pc: u32 = 0x8270CEB8;
    'dispatch: loop {
        match pc {
            0x8270CEB8 => {
    //   block [0x8270CEB8..0x8270CECC)
	// 8270CEB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CEBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 8270CEC0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CEC4: 916A00D4  stw r11, 0xd4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 8270CEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CED0 size=20
    let mut pc: u32 = 0x8270CED0;
    'dispatch: loop {
        match pc {
            0x8270CED0 => {
    //   block [0x8270CED0..0x8270CEE4)
	// 8270CED0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CED4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 8270CED8: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CEDC: 916A41CC  stw r11, 0x41cc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16844 as u32), ctx.r[11].u32 ) };
	// 8270CEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CEE8 size=20
    let mut pc: u32 = 0x8270CEE8;
    'dispatch: loop {
        match pc {
            0x8270CEE8 => {
    //   block [0x8270CEE8..0x8270CEFC)
	// 8270CEE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CEEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 8270CEF0: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CEF4: 916A82C4  stw r11, -0x7d3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-32060 as u32), ctx.r[11].u32 ) };
	// 8270CEF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF00 size=20
    let mut pc: u32 = 0x8270CF00;
    'dispatch: loop {
        match pc {
            0x8270CF00 => {
    //   block [0x8270CF00..0x8270CF14)
	// 8270CF00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 8270CF08: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF0C: 916AB27C  stw r11, -0x4d84(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-19844 as u32), ctx.r[11].u32 ) };
	// 8270CF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF18 size=20
    let mut pc: u32 = 0x8270CF18;
    'dispatch: loop {
        match pc {
            0x8270CF18 => {
    //   block [0x8270CF18..0x8270CF2C)
	// 8270CF18: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 8270CF20: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF24: 916A0B44  stw r11, 0xb44(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(2884 as u32), ctx.r[11].u32 ) };
	// 8270CF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF30 size=20
    let mut pc: u32 = 0x8270CF30;
    'dispatch: loop {
        match pc {
            0x8270CF30 => {
    //   block [0x8270CF30..0x8270CF44)
	// 8270CF30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 8270CF38: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF3C: 916A649C  stw r11, 0x649c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(25756 as u32), ctx.r[11].u32 ) };
	// 8270CF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF48 size=20
    let mut pc: u32 = 0x8270CF48;
    'dispatch: loop {
        match pc {
            0x8270CF48 => {
    //   block [0x8270CF48..0x8270CF5C)
	// 8270CF48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270CF50: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF54: 916AB794  stw r11, -0x486c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18540 as u32), ctx.r[11].u32 ) };
	// 8270CF58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF60 size=20
    let mut pc: u32 = 0x8270CF60;
    'dispatch: loop {
        match pc {
            0x8270CF60 => {
    //   block [0x8270CF60..0x8270CF74)
	// 8270CF60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270CF68: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF6C: 916AF79C  stw r11, -0x864(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-2148 as u32), ctx.r[11].u32 ) };
	// 8270CF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF78 size=20
    let mut pc: u32 = 0x8270CF78;
    'dispatch: loop {
        match pc {
            0x8270CF78 => {
    //   block [0x8270CF78..0x8270CF8C)
	// 8270CF78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF7C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8270CF80: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF84: 916AB67C  stw r11, -0x4984(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-18820 as u32), ctx.r[11].u32 ) };
	// 8270CF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CF90 size=20
    let mut pc: u32 = 0x8270CF90;
    'dispatch: loop {
        match pc {
            0x8270CF90 => {
    //   block [0x8270CF90..0x8270CFA4)
	// 8270CF90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8270CF94: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8270CF98: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8270CF9C: 916AD24C  stw r11, -0x2db4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-11700 as u32), ctx.r[11].u32 ) };
	// 8270CFA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CFA8 size=20
    let mut pc: u32 = 0x8270CFA8;
    'dispatch: loop {
        match pc {
            0x8270CFA8 => {
    //   block [0x8270CFA8..0x8270CFBC)
	// 8270CFA8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270CFAC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 8270CFB0: 396B6DF8  addi r11, r11, 0x6df8
	ctx.r[11].s64 = ctx.r[11].s64 + 28152;
	// 8270CFB4: 916AE660  stw r11, -0x19a0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-6560 as u32), ctx.r[11].u32 ) };
	// 8270CFB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CFC0 size=20
    let mut pc: u32 = 0x8270CFC0;
    'dispatch: loop {
        match pc {
            0x8270CFC0 => {
    //   block [0x8270CFC0..0x8270CFD4)
	// 8270CFC0: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8270CFC4: 3D408283  lis r10, -0x7d7d
	ctx.r[10].s64 = -2105344000;
	// 8270CFC8: 396B68EC  addi r11, r11, 0x68ec
	ctx.r[11].s64 = ctx.r[11].s64 + 26860;
	// 8270CFCC: 916A2348  stw r11, 0x2348(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(9032 as u32), ctx.r[11].u32 ) };
	// 8270CFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CFD8 size=20
    let mut pc: u32 = 0x8270CFD8;
    'dispatch: loop {
        match pc {
            0x8270CFD8 => {
    //   block [0x8270CFD8..0x8270CFEC)
	// 8270CFD8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 8270CFDC: 3D408283  lis r10, -0x7d7d
	ctx.r[10].s64 = -2105344000;
	// 8270CFE0: 396B68EC  addi r11, r11, 0x68ec
	ctx.r[11].s64 = ctx.r[11].s64 + 26860;
	// 8270CFE4: 916A2368  stw r11, 0x2368(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(9064 as u32), ctx.r[11].u32 ) };
	// 8270CFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CFEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CFEC size=16
    let mut pc: u32 = 0x8270CFEC;
    'dispatch: loop {
        match pc {
            0x8270CFEC => {
    //   block [0x8270CFEC..0x8270CFFC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270CFFC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270CFFC size=16
    let mut pc: u32 = 0x8270CFFC;
    'dispatch: loop {
        match pc {
            0x8270CFFC => {
    //   block [0x8270CFFC..0x8270D00C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D00C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D00C size=16
    let mut pc: u32 = 0x8270D00C;
    'dispatch: loop {
        match pc {
            0x8270D00C => {
    //   block [0x8270D00C..0x8270D01C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D01C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D01C size=16
    let mut pc: u32 = 0x8270D01C;
    'dispatch: loop {
        match pc {
            0x8270D01C => {
    //   block [0x8270D01C..0x8270D02C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D02C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D02C size=16
    let mut pc: u32 = 0x8270D02C;
    'dispatch: loop {
        match pc {
            0x8270D02C => {
    //   block [0x8270D02C..0x8270D03C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D03C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D03C size=16
    let mut pc: u32 = 0x8270D03C;
    'dispatch: loop {
        match pc {
            0x8270D03C => {
    //   block [0x8270D03C..0x8270D04C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D04C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D04C size=16
    let mut pc: u32 = 0x8270D04C;
    'dispatch: loop {
        match pc {
            0x8270D04C => {
    //   block [0x8270D04C..0x8270D05C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D05C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D05C size=16
    let mut pc: u32 = 0x8270D05C;
    'dispatch: loop {
        match pc {
            0x8270D05C => {
    //   block [0x8270D05C..0x8270D06C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D06C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D06C size=16
    let mut pc: u32 = 0x8270D06C;
    'dispatch: loop {
        match pc {
            0x8270D06C => {
    //   block [0x8270D06C..0x8270D07C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D07C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D07C size=16
    let mut pc: u32 = 0x8270D07C;
    'dispatch: loop {
        match pc {
            0x8270D07C => {
    //   block [0x8270D07C..0x8270D08C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D08C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D08C size=16
    let mut pc: u32 = 0x8270D08C;
    'dispatch: loop {
        match pc {
            0x8270D08C => {
    //   block [0x8270D08C..0x8270D09C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D09C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D09C size=16
    let mut pc: u32 = 0x8270D09C;
    'dispatch: loop {
        match pc {
            0x8270D09C => {
    //   block [0x8270D09C..0x8270D0AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0AC size=16
    let mut pc: u32 = 0x8270D0AC;
    'dispatch: loop {
        match pc {
            0x8270D0AC => {
    //   block [0x8270D0AC..0x8270D0BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0BC size=16
    let mut pc: u32 = 0x8270D0BC;
    'dispatch: loop {
        match pc {
            0x8270D0BC => {
    //   block [0x8270D0BC..0x8270D0CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0CC size=16
    let mut pc: u32 = 0x8270D0CC;
    'dispatch: loop {
        match pc {
            0x8270D0CC => {
    //   block [0x8270D0CC..0x8270D0DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0DC size=16
    let mut pc: u32 = 0x8270D0DC;
    'dispatch: loop {
        match pc {
            0x8270D0DC => {
    //   block [0x8270D0DC..0x8270D0EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0EC size=16
    let mut pc: u32 = 0x8270D0EC;
    'dispatch: loop {
        match pc {
            0x8270D0EC => {
    //   block [0x8270D0EC..0x8270D0FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D0FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D0FC size=16
    let mut pc: u32 = 0x8270D0FC;
    'dispatch: loop {
        match pc {
            0x8270D0FC => {
    //   block [0x8270D0FC..0x8270D10C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D10C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D10C size=16
    let mut pc: u32 = 0x8270D10C;
    'dispatch: loop {
        match pc {
            0x8270D10C => {
    //   block [0x8270D10C..0x8270D11C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D11C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D11C size=16
    let mut pc: u32 = 0x8270D11C;
    'dispatch: loop {
        match pc {
            0x8270D11C => {
    //   block [0x8270D11C..0x8270D12C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D12C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D12C size=16
    let mut pc: u32 = 0x8270D12C;
    'dispatch: loop {
        match pc {
            0x8270D12C => {
    //   block [0x8270D12C..0x8270D13C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D13C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D13C size=16
    let mut pc: u32 = 0x8270D13C;
    'dispatch: loop {
        match pc {
            0x8270D13C => {
    //   block [0x8270D13C..0x8270D14C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D14C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D14C size=16
    let mut pc: u32 = 0x8270D14C;
    'dispatch: loop {
        match pc {
            0x8270D14C => {
    //   block [0x8270D14C..0x8270D15C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D15C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D15C size=16
    let mut pc: u32 = 0x8270D15C;
    'dispatch: loop {
        match pc {
            0x8270D15C => {
    //   block [0x8270D15C..0x8270D16C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D16C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D16C size=16
    let mut pc: u32 = 0x8270D16C;
    'dispatch: loop {
        match pc {
            0x8270D16C => {
    //   block [0x8270D16C..0x8270D17C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D17C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D17C size=16
    let mut pc: u32 = 0x8270D17C;
    'dispatch: loop {
        match pc {
            0x8270D17C => {
    //   block [0x8270D17C..0x8270D18C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D18C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D18C size=16
    let mut pc: u32 = 0x8270D18C;
    'dispatch: loop {
        match pc {
            0x8270D18C => {
    //   block [0x8270D18C..0x8270D19C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D19C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D19C size=16
    let mut pc: u32 = 0x8270D19C;
    'dispatch: loop {
        match pc {
            0x8270D19C => {
    //   block [0x8270D19C..0x8270D1AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1AC size=16
    let mut pc: u32 = 0x8270D1AC;
    'dispatch: loop {
        match pc {
            0x8270D1AC => {
    //   block [0x8270D1AC..0x8270D1BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1BC size=16
    let mut pc: u32 = 0x8270D1BC;
    'dispatch: loop {
        match pc {
            0x8270D1BC => {
    //   block [0x8270D1BC..0x8270D1CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1CC size=16
    let mut pc: u32 = 0x8270D1CC;
    'dispatch: loop {
        match pc {
            0x8270D1CC => {
    //   block [0x8270D1CC..0x8270D1DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1DC size=16
    let mut pc: u32 = 0x8270D1DC;
    'dispatch: loop {
        match pc {
            0x8270D1DC => {
    //   block [0x8270D1DC..0x8270D1EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1EC size=16
    let mut pc: u32 = 0x8270D1EC;
    'dispatch: loop {
        match pc {
            0x8270D1EC => {
    //   block [0x8270D1EC..0x8270D1FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D1FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D1FC size=16
    let mut pc: u32 = 0x8270D1FC;
    'dispatch: loop {
        match pc {
            0x8270D1FC => {
    //   block [0x8270D1FC..0x8270D20C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D20C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D20C size=16
    let mut pc: u32 = 0x8270D20C;
    'dispatch: loop {
        match pc {
            0x8270D20C => {
    //   block [0x8270D20C..0x8270D21C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D21C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D21C size=16
    let mut pc: u32 = 0x8270D21C;
    'dispatch: loop {
        match pc {
            0x8270D21C => {
    //   block [0x8270D21C..0x8270D22C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D22C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D22C size=16
    let mut pc: u32 = 0x8270D22C;
    'dispatch: loop {
        match pc {
            0x8270D22C => {
    //   block [0x8270D22C..0x8270D23C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D23C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D23C size=16
    let mut pc: u32 = 0x8270D23C;
    'dispatch: loop {
        match pc {
            0x8270D23C => {
    //   block [0x8270D23C..0x8270D24C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D24C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D24C size=16
    let mut pc: u32 = 0x8270D24C;
    'dispatch: loop {
        match pc {
            0x8270D24C => {
    //   block [0x8270D24C..0x8270D25C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D25C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D25C size=16
    let mut pc: u32 = 0x8270D25C;
    'dispatch: loop {
        match pc {
            0x8270D25C => {
    //   block [0x8270D25C..0x8270D26C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D26C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D26C size=16
    let mut pc: u32 = 0x8270D26C;
    'dispatch: loop {
        match pc {
            0x8270D26C => {
    //   block [0x8270D26C..0x8270D27C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D27C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D27C size=16
    let mut pc: u32 = 0x8270D27C;
    'dispatch: loop {
        match pc {
            0x8270D27C => {
    //   block [0x8270D27C..0x8270D28C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D28C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D28C size=16
    let mut pc: u32 = 0x8270D28C;
    'dispatch: loop {
        match pc {
            0x8270D28C => {
    //   block [0x8270D28C..0x8270D29C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D29C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D29C size=16
    let mut pc: u32 = 0x8270D29C;
    'dispatch: loop {
        match pc {
            0x8270D29C => {
    //   block [0x8270D29C..0x8270D2AC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2AC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2AC size=16
    let mut pc: u32 = 0x8270D2AC;
    'dispatch: loop {
        match pc {
            0x8270D2AC => {
    //   block [0x8270D2AC..0x8270D2BC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2BC size=16
    let mut pc: u32 = 0x8270D2BC;
    'dispatch: loop {
        match pc {
            0x8270D2BC => {
    //   block [0x8270D2BC..0x8270D2CC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2CC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2CC size=16
    let mut pc: u32 = 0x8270D2CC;
    'dispatch: loop {
        match pc {
            0x8270D2CC => {
    //   block [0x8270D2CC..0x8270D2DC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2DC size=16
    let mut pc: u32 = 0x8270D2DC;
    'dispatch: loop {
        match pc {
            0x8270D2DC => {
    //   block [0x8270D2DC..0x8270D2EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2EC size=16
    let mut pc: u32 = 0x8270D2EC;
    'dispatch: loop {
        match pc {
            0x8270D2EC => {
    //   block [0x8270D2EC..0x8270D2FC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D2FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D2FC size=16
    let mut pc: u32 = 0x8270D2FC;
    'dispatch: loop {
        match pc {
            0x8270D2FC => {
    //   block [0x8270D2FC..0x8270D30C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D30C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D30C size=16
    let mut pc: u32 = 0x8270D30C;
    'dispatch: loop {
        match pc {
            0x8270D30C => {
    //   block [0x8270D30C..0x8270D31C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D31C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D31C size=16
    let mut pc: u32 = 0x8270D31C;
    'dispatch: loop {
        match pc {
            0x8270D31C => {
    //   block [0x8270D31C..0x8270D32C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D32C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D32C size=16
    let mut pc: u32 = 0x8270D32C;
    'dispatch: loop {
        match pc {
            0x8270D32C => {
    //   block [0x8270D32C..0x8270D33C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D33C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D33C size=16
    let mut pc: u32 = 0x8270D33C;
    'dispatch: loop {
        match pc {
            0x8270D33C => {
    //   block [0x8270D33C..0x8270D34C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D34C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D34C size=16
    let mut pc: u32 = 0x8270D34C;
    'dispatch: loop {
        match pc {
            0x8270D34C => {
    //   block [0x8270D34C..0x8270D35C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D35C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D35C size=16
    let mut pc: u32 = 0x8270D35C;
    'dispatch: loop {
        match pc {
            0x8270D35C => {
    //   block [0x8270D35C..0x8270D36C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270D36C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270D36C size=16
    let mut pc: u32 = 0x8270D36C;
    'dispatch: loop {
        match pc {
            0x8270D36C => {
    //   block [0x8270D36C..0x8270D37C)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


