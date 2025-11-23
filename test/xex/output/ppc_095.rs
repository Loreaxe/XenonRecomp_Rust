pub fn sub_8266C790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C790 size=108
    let mut pc: u32 = 0x8266C790;
    'dispatch: loop {
        match pc {
            0x8266C790 => {
    //   block [0x8266C790..0x8266C7FC)
	// 8266C790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C79C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C7A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C7A4: 38EBD918  addi r7, r11, -0x26e8
	ctx.r[7].s64 = ctx.r[11].s64 + -9960;
	// 8266C7A8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8266C7AC: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8266C7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C7B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C7B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C7C0: 386AFFE8  addi r3, r10, -0x18
	ctx.r[3].s64 = ctx.r[10].s64 + -24;
	// 8266C7C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C7C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C7D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C7D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C7D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C7DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C7E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C7E8: 4BDFA639  bl 0x82466e20
	ctx.lr = 0x8266C7EC;
	sub_82466E20(ctx, base);
	// 8266C7EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C800 size=108
    let mut pc: u32 = 0x8266C800;
    'dispatch: loop {
        match pc {
            0x8266C800 => {
    //   block [0x8266C800..0x8266C86C)
	// 8266C800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C80C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C810: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C814: 38EBD9D8  addi r7, r11, -0x2628
	ctx.r[7].s64 = ctx.r[11].s64 + -9768;
	// 8266C818: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266C81C: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 8266C820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C824: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C828: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C82C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C830: 386A0018  addi r3, r10, 0x18
	ctx.r[3].s64 = ctx.r[10].s64 + 24;
	// 8266C834: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C83C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C854: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C858: 4BDFA5C9  bl 0x82466e20
	ctx.lr = 0x8266C85C;
	sub_82466E20(ctx, base);
	// 8266C85C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C870 size=108
    let mut pc: u32 = 0x8266C870;
    'dispatch: loop {
        match pc {
            0x8266C870 => {
    //   block [0x8266C870..0x8266C8DC)
	// 8266C870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C87C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C884: 38EBD9F0  addi r7, r11, -0x2610
	ctx.r[7].s64 = ctx.r[11].s64 + -9744;
	// 8266C888: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8266C88C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 8266C890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C894: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C898: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C8A0: 386A0048  addi r3, r10, 0x48
	ctx.r[3].s64 = ctx.r[10].s64 + 72;
	// 8266C8A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C8C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C8C8: 4BDFA559  bl 0x82466e20
	ctx.lr = 0x8266C8CC;
	sub_82466E20(ctx, base);
	// 8266C8CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266C8E0 size=24
    let mut pc: u32 = 0x8266C8E0;
    'dispatch: loop {
        match pc {
            0x8266C8E0 => {
    //   block [0x8266C8E0..0x8266C8F8)
	// 8266C8E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C8E4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266C8E8: 394A2140  addi r10, r10, 0x2140
	ctx.r[10].s64 = ctx.r[10].s64 + 8512;
	// 8266C8EC: 816BD8B4  lwz r11, -0x274c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10060 as u32) ) } as u64;
	// 8266C8F0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8266C8F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C8F8 size=108
    let mut pc: u32 = 0x8266C8F8;
    'dispatch: loop {
        match pc {
            0x8266C8F8 => {
    //   block [0x8266C8F8..0x8266C964)
	// 8266C8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C904: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C90C: 38EB2140  addi r7, r11, 0x2140
	ctx.r[7].s64 = ctx.r[11].s64 + 8512;
	// 8266C910: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C914: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 8266C918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C91C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C928: 386A0078  addi r3, r10, 0x78
	ctx.r[3].s64 = ctx.r[10].s64 + 120;
	// 8266C92C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C94C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C950: 4BDFA4D1  bl 0x82466e20
	ctx.lr = 0x8266C954;
	sub_82466E20(ctx, base);
	// 8266C954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266C968 size=24
    let mut pc: u32 = 0x8266C968;
    'dispatch: loop {
        match pc {
            0x8266C968 => {
    //   block [0x8266C968..0x8266C980)
	// 8266C968: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C96C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266C970: 394A2170  addi r10, r10, 0x2170
	ctx.r[10].s64 = ctx.r[10].s64 + 8560;
	// 8266C974: 816BD8B4  lwz r11, -0x274c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10060 as u32) ) } as u64;
	// 8266C978: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8266C97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C980 size=108
    let mut pc: u32 = 0x8266C980;
    'dispatch: loop {
        match pc {
            0x8266C980 => {
    //   block [0x8266C980..0x8266C9EC)
	// 8266C980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C98C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C994: 38EB2170  addi r7, r11, 0x2170
	ctx.r[7].s64 = ctx.r[11].s64 + 8560;
	// 8266C998: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C99C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 8266C9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C9A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C9A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C9AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C9B0: 386A00A8  addi r3, r10, 0xa8
	ctx.r[3].s64 = ctx.r[10].s64 + 168;
	// 8266C9B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C9C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C9C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C9CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C9D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C9D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C9D8: 4BDFA449  bl 0x82466e20
	ctx.lr = 0x8266C9DC;
	sub_82466E20(ctx, base);
	// 8266C9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C9F0 size=108
    let mut pc: u32 = 0x8266C9F0;
    'dispatch: loop {
        match pc {
            0x8266C9F0 => {
    //   block [0x8266C9F0..0x8266CA5C)
	// 8266C9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C9FC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CA00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CA04: 38EBDA68  addi r7, r11, -0x2598
	ctx.r[7].s64 = ctx.r[11].s64 + -9624;
	// 8266CA08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266CA0C: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 8266CA10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CA14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CA18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CA1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CA20: 386A00D8  addi r3, r10, 0xd8
	ctx.r[3].s64 = ctx.r[10].s64 + 216;
	// 8266CA24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266CA28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CA2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CA30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CA38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CA3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CA40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CA44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CA48: 4BDFA3D9  bl 0x82466e20
	ctx.lr = 0x8266CA4C;
	sub_82466E20(ctx, base);
	// 8266CA4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266CA60 size=24
    let mut pc: u32 = 0x8266CA60;
    'dispatch: loop {
        match pc {
            0x8266CA60 => {
    //   block [0x8266CA60..0x8266CA78)
	// 8266CA60: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CA64: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266CA68: 394A21A0  addi r10, r10, 0x21a0
	ctx.r[10].s64 = ctx.r[10].s64 + 8608;
	// 8266CA6C: 816BD8B4  lwz r11, -0x274c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10060 as u32) ) } as u64;
	// 8266CA70: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8266CA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CA78 size=108
    let mut pc: u32 = 0x8266CA78;
    'dispatch: loop {
        match pc {
            0x8266CA78 => {
    //   block [0x8266CA78..0x8266CAE4)
	// 8266CA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CA84: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CA88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CA8C: 38EB21A0  addi r7, r11, 0x21a0
	ctx.r[7].s64 = ctx.r[11].s64 + 8608;
	// 8266CA90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266CA94: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 8266CA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CA9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CAA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CAA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CAA8: 386A0108  addi r3, r10, 0x108
	ctx.r[3].s64 = ctx.r[10].s64 + 264;
	// 8266CAAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266CAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CAD0: 4BDFA351  bl 0x82466e20
	ctx.lr = 0x8266CAD4;
	sub_82466E20(ctx, base);
	// 8266CAD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CAE8 size=112
    let mut pc: u32 = 0x8266CAE8;
    'dispatch: loop {
        match pc {
            0x8266CAE8 => {
    //   block [0x8266CAE8..0x8266CB58)
	// 8266CAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CAF4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266CAF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CAFC: 392A14CC  addi r9, r10, 0x14cc
	ctx.r[9].s64 = ctx.r[10].s64 + 5324;
	// 8266CB00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CB04: 390BDA80  addi r8, r11, -0x2580
	ctx.r[8].s64 = ctx.r[11].s64 + -9600;
	// 8266CB08: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8266CB0C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 8266CB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CB14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CB18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266CB1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CB20: 386A0138  addi r3, r10, 0x138
	ctx.r[3].s64 = ctx.r[10].s64 + 312;
	// 8266CB24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266CB28: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266CB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CB3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CB44: 4BDFA2DD  bl 0x82466e20
	ctx.lr = 0x8266CB48;
	sub_82466E20(ctx, base);
	// 8266CB48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CB4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CB50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CB58 size=108
    let mut pc: u32 = 0x8266CB58;
    'dispatch: loop {
        match pc {
            0x8266CB58 => {
    //   block [0x8266CB58..0x8266CBC4)
	// 8266CB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CB64: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CB68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CB6C: 38EBDAB0  addi r7, r11, -0x2550
	ctx.r[7].s64 = ctx.r[11].s64 + -9552;
	// 8266CB70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266CB74: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 8266CB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CB7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CB80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CB84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CB88: 386A0168  addi r3, r10, 0x168
	ctx.r[3].s64 = ctx.r[10].s64 + 360;
	// 8266CB8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266CB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CB94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CB9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CBA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CBA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CBAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CBB0: 4BDFA271  bl 0x82466e20
	ctx.lr = 0x8266CBB4;
	sub_82466E20(ctx, base);
	// 8266CBB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CBB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CBBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CBC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CBC8 size=108
    let mut pc: u32 = 0x8266CBC8;
    'dispatch: loop {
        match pc {
            0x8266CBC8 => {
    //   block [0x8266CBC8..0x8266CC34)
	// 8266CBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CBD4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CBDC: 38EBDAE0  addi r7, r11, -0x2520
	ctx.r[7].s64 = ctx.r[11].s64 + -9504;
	// 8266CBE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266CBE4: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 8266CBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CBEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CBF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CBF8: 386A0198  addi r3, r10, 0x198
	ctx.r[3].s64 = ctx.r[10].s64 + 408;
	// 8266CBFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266CC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CC0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CC1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CC20: 4BDFA201  bl 0x82466e20
	ctx.lr = 0x8266CC24;
	sub_82466E20(ctx, base);
	// 8266CC24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CC38 size=112
    let mut pc: u32 = 0x8266CC38;
    'dispatch: loop {
        match pc {
            0x8266CC38 => {
    //   block [0x8266CC38..0x8266CCA8)
	// 8266CC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CC44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CC48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CC4C: 38AA01F8  addi r5, r10, 0x1f8
	ctx.r[5].s64 = ctx.r[10].s64 + 504;
	// 8266CC50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CC54: 390BDB10  addi r8, r11, -0x24f0
	ctx.r[8].s64 = ctx.r[11].s64 + -9456;
	// 8266CC58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266CC5C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 8266CC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CC64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CC68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266CC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CC70: 386A01C8  addi r3, r10, 0x1c8
	ctx.r[3].s64 = ctx.r[10].s64 + 456;
	// 8266CC74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266CC78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CC94: 4BDFA18D  bl 0x82466e20
	ctx.lr = 0x8266CC98;
	sub_82466E20(ctx, base);
	// 8266CC98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CCA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CCA8 size=108
    let mut pc: u32 = 0x8266CCA8;
    'dispatch: loop {
        match pc {
            0x8266CCA8 => {
    //   block [0x8266CCA8..0x8266CD14)
	// 8266CCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CCB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CCB4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CCB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CCBC: 38EBDB28  addi r7, r11, -0x24d8
	ctx.r[7].s64 = ctx.r[11].s64 + -9432;
	// 8266CCC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266CCC4: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 8266CCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CCCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CCD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CCD8: 386A01F8  addi r3, r10, 0x1f8
	ctx.r[3].s64 = ctx.r[10].s64 + 504;
	// 8266CCDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266CCE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CCE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CCE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CCF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CCF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CCFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CD00: 4BDFA121  bl 0x82466e20
	ctx.lr = 0x8266CD04;
	sub_82466E20(ctx, base);
	// 8266CD04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CD18 size=108
    let mut pc: u32 = 0x8266CD18;
    'dispatch: loop {
        match pc {
            0x8266CD18 => {
    //   block [0x8266CD18..0x8266CD84)
	// 8266CD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CD20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CD24: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CD2C: 38EBDB58  addi r7, r11, -0x24a8
	ctx.r[7].s64 = ctx.r[11].s64 + -9384;
	// 8266CD30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266CD34: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 8266CD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CD3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CD40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CD48: 386A0228  addi r3, r10, 0x228
	ctx.r[3].s64 = ctx.r[10].s64 + 552;
	// 8266CD4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266CD50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CD58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CD5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CD60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CD68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CD6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CD70: 4BDFA0B1  bl 0x82466e20
	ctx.lr = 0x8266CD74;
	sub_82466E20(ctx, base);
	// 8266CD74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CD80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CD88 size=108
    let mut pc: u32 = 0x8266CD88;
    'dispatch: loop {
        match pc {
            0x8266CD88 => {
    //   block [0x8266CD88..0x8266CDF4)
	// 8266CD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CD94: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CD98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CD9C: 38EBDB70  addi r7, r11, -0x2490
	ctx.r[7].s64 = ctx.r[11].s64 + -9360;
	// 8266CDA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266CDA4: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 8266CDA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CDAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CDB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CDB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CDB8: 386A0258  addi r3, r10, 0x258
	ctx.r[3].s64 = ctx.r[10].s64 + 600;
	// 8266CDBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266CDC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CDC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CDCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CDD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CDD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CDD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CDDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CDE0: 4BDFA041  bl 0x82466e20
	ctx.lr = 0x8266CDE4;
	sub_82466E20(ctx, base);
	// 8266CDE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CDE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CDEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CDF8 size=108
    let mut pc: u32 = 0x8266CDF8;
    'dispatch: loop {
        match pc {
            0x8266CDF8 => {
    //   block [0x8266CDF8..0x8266CE64)
	// 8266CDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CE04: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CE08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CE0C: 38EBDBA0  addi r7, r11, -0x2460
	ctx.r[7].s64 = ctx.r[11].s64 + -9312;
	// 8266CE10: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8266CE14: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 8266CE18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CE1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CE20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CE24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CE28: 386A0288  addi r3, r10, 0x288
	ctx.r[3].s64 = ctx.r[10].s64 + 648;
	// 8266CE2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266CE30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CE34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CE3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CE44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CE4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CE50: 4BDF9FD1  bl 0x82466e20
	ctx.lr = 0x8266CE54;
	sub_82466E20(ctx, base);
	// 8266CE54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CE68 size=108
    let mut pc: u32 = 0x8266CE68;
    'dispatch: loop {
        match pc {
            0x8266CE68 => {
    //   block [0x8266CE68..0x8266CED4)
	// 8266CE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CE74: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CE78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CE7C: 38EBDC48  addi r7, r11, -0x23b8
	ctx.r[7].s64 = ctx.r[11].s64 + -9144;
	// 8266CE80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266CE84: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 8266CE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CE8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CE90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CE94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CE98: 386A02B8  addi r3, r10, 0x2b8
	ctx.r[3].s64 = ctx.r[10].s64 + 696;
	// 8266CE9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266CEA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CEA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CEAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CEB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CEB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CEB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CEBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CEC0: 4BDF9F61  bl 0x82466e20
	ctx.lr = 0x8266CEC4;
	sub_82466E20(ctx, base);
	// 8266CEC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CED8 size=108
    let mut pc: u32 = 0x8266CED8;
    'dispatch: loop {
        match pc {
            0x8266CED8 => {
    //   block [0x8266CED8..0x8266CF44)
	// 8266CED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CEE4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CEE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CEEC: 38EBDC78  addi r7, r11, -0x2388
	ctx.r[7].s64 = ctx.r[11].s64 + -9096;
	// 8266CEF0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8266CEF4: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 8266CEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CEFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CF00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266CF08: 386A02E8  addi r3, r10, 0x2e8
	ctx.r[3].s64 = ctx.r[10].s64 + 744;
	// 8266CF0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266CF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266CF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CF18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CF20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CF28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CF2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CF30: 4BDF9EF1  bl 0x82466e20
	ctx.lr = 0x8266CF34;
	sub_82466E20(ctx, base);
	// 8266CF34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CF38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CF3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266CF48 size=24
    let mut pc: u32 = 0x8266CF48;
    'dispatch: loop {
        match pc {
            0x8266CF48 => {
    //   block [0x8266CF48..0x8266CF60)
	// 8266CF48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CF4C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266CF50: 394A21D0  addi r10, r10, 0x21d0
	ctx.r[10].s64 = ctx.r[10].s64 + 8656;
	// 8266CF54: 816BDD38  lwz r11, -0x22c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8904 as u32) ) } as u64;
	// 8266CF58: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8266CF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CF60 size=112
    let mut pc: u32 = 0x8266CF60;
    'dispatch: loop {
        match pc {
            0x8266CF60 => {
    //   block [0x8266CF60..0x8266CFD0)
	// 8266CF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CF6C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266CF70: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CF74: 392A14F8  addi r9, r10, 0x14f8
	ctx.r[9].s64 = ctx.r[10].s64 + 5368;
	// 8266CF78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CF7C: 390B21D0  addi r8, r11, 0x21d0
	ctx.r[8].s64 = ctx.r[11].s64 + 8656;
	// 8266CF80: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8266CF84: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 8266CF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CF8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CF90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266CF94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266CF98: 386A0318  addi r3, r10, 0x318
	ctx.r[3].s64 = ctx.r[10].s64 + 792;
	// 8266CF9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266CFA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266CFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266CFA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266CFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266CFB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266CFB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266CFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266CFBC: 4BDF9E65  bl 0x82466e20
	ctx.lr = 0x8266CFC0;
	sub_82466E20(ctx, base);
	// 8266CFC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266CFC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266CFC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266CFCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266CFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266CFD0 size=108
    let mut pc: u32 = 0x8266CFD0;
    'dispatch: loop {
        match pc {
            0x8266CFD0 => {
    //   block [0x8266CFD0..0x8266D03C)
	// 8266CFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266CFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266CFD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266CFDC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266CFE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266CFE4: 38EBDD40  addi r7, r11, -0x22c0
	ctx.r[7].s64 = ctx.r[11].s64 + -8896;
	// 8266CFE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266CFEC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 8266CFF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266CFF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266CFF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266CFFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D000: 386A0348  addi r3, r10, 0x348
	ctx.r[3].s64 = ctx.r[10].s64 + 840;
	// 8266D004: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D00C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D028: 4BDF9DF9  bl 0x82466e20
	ctx.lr = 0x8266D02C;
	sub_82466E20(ctx, base);
	// 8266D02C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D040 size=112
    let mut pc: u32 = 0x8266D040;
    'dispatch: loop {
        match pc {
            0x8266D040 => {
    //   block [0x8266D040..0x8266D0B0)
	// 8266D040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D04C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266D050: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D054: 392A153C  addi r9, r10, 0x153c
	ctx.r[9].s64 = ctx.r[10].s64 + 5436;
	// 8266D058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D05C: 390BDD70  addi r8, r11, -0x2290
	ctx.r[8].s64 = ctx.r[11].s64 + -8848;
	// 8266D060: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8266D064: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 8266D068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D06C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266D074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D078: 386A0378  addi r3, r10, 0x378
	ctx.r[3].s64 = ctx.r[10].s64 + 888;
	// 8266D07C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266D080: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266D084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D09C: 4BDF9D85  bl 0x82466e20
	ctx.lr = 0x8266D0A0;
	sub_82466E20(ctx, base);
	// 8266D0A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266D0B0 size=24
    let mut pc: u32 = 0x8266D0B0;
    'dispatch: loop {
        match pc {
            0x8266D0B0 => {
    //   block [0x8266D0B0..0x8266D0C8)
	// 8266D0B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D0B4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266D0B8: 394A2248  addi r10, r10, 0x2248
	ctx.r[10].s64 = ctx.r[10].s64 + 8776;
	// 8266D0BC: 816BDE30  lwz r11, -0x21d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8656 as u32) ) } as u64;
	// 8266D0C0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8266D0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D0C8 size=112
    let mut pc: u32 = 0x8266D0C8;
    'dispatch: loop {
        match pc {
            0x8266D0C8 => {
    //   block [0x8266D0C8..0x8266D138)
	// 8266D0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D0D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266D0D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D0DC: 392A1578  addi r9, r10, 0x1578
	ctx.r[9].s64 = ctx.r[10].s64 + 5496;
	// 8266D0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D0E4: 390B2248  addi r8, r11, 0x2248
	ctx.r[8].s64 = ctx.r[11].s64 + 8776;
	// 8266D0E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8266D0EC: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 8266D0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D0F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266D0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D100: 386A03A8  addi r3, r10, 0x3a8
	ctx.r[3].s64 = ctx.r[10].s64 + 936;
	// 8266D104: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266D108: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266D10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D11C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D124: 4BDF9CFD  bl 0x82466e20
	ctx.lr = 0x8266D128;
	sub_82466E20(ctx, base);
	// 8266D128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D138 size=108
    let mut pc: u32 = 0x8266D138;
    'dispatch: loop {
        match pc {
            0x8266D138 => {
    //   block [0x8266D138..0x8266D1A4)
	// 8266D138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D144: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D14C: 38EBDE34  addi r7, r11, -0x21cc
	ctx.r[7].s64 = ctx.r[11].s64 + -8652;
	// 8266D150: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266D154: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 8266D158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D15C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D160: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D168: 386A03D8  addi r3, r10, 0x3d8
	ctx.r[3].s64 = ctx.r[10].s64 + 984;
	// 8266D16C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D17C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D18C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D190: 4BDF9C91  bl 0x82466e20
	ctx.lr = 0x8266D194;
	sub_82466E20(ctx, base);
	// 8266D194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D1A8 size=108
    let mut pc: u32 = 0x8266D1A8;
    'dispatch: loop {
        match pc {
            0x8266D1A8 => {
    //   block [0x8266D1A8..0x8266D214)
	// 8266D1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D1B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D1B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D1BC: 38EBDE4C  addi r7, r11, -0x21b4
	ctx.r[7].s64 = ctx.r[11].s64 + -8628;
	// 8266D1C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266D1C4: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8266D1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D1CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D1D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D1D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D1D8: 386A0408  addi r3, r10, 0x408
	ctx.r[3].s64 = ctx.r[10].s64 + 1032;
	// 8266D1DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D1F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D1FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D200: 4BDF9C21  bl 0x82466e20
	ctx.lr = 0x8266D204;
	sub_82466E20(ctx, base);
	// 8266D204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266D218 size=24
    let mut pc: u32 = 0x8266D218;
    'dispatch: loop {
        match pc {
            0x8266D218 => {
    //   block [0x8266D218..0x8266D230)
	// 8266D218: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D21C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266D220: 394A2290  addi r10, r10, 0x2290
	ctx.r[10].s64 = ctx.r[10].s64 + 8848;
	// 8266D224: 816BDE7C  lwz r11, -0x2184(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8580 as u32) ) } as u64;
	// 8266D228: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8266D22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D230 size=112
    let mut pc: u32 = 0x8266D230;
    'dispatch: loop {
        match pc {
            0x8266D230 => {
    //   block [0x8266D230..0x8266D2A0)
	// 8266D230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D23C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266D240: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D244: 392A15B4  addi r9, r10, 0x15b4
	ctx.r[9].s64 = ctx.r[10].s64 + 5556;
	// 8266D248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D24C: 390B2290  addi r8, r11, 0x2290
	ctx.r[8].s64 = ctx.r[11].s64 + 8848;
	// 8266D250: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8266D254: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 8266D258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D25C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266D264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D268: 386A0438  addi r3, r10, 0x438
	ctx.r[3].s64 = ctx.r[10].s64 + 1080;
	// 8266D26C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266D270: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266D274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D28C: 4BDF9B95  bl 0x82466e20
	ctx.lr = 0x8266D290;
	sub_82466E20(ctx, base);
	// 8266D290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D2A0 size=108
    let mut pc: u32 = 0x8266D2A0;
    'dispatch: loop {
        match pc {
            0x8266D2A0 => {
    //   block [0x8266D2A0..0x8266D30C)
	// 8266D2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D2AC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D2B4: 38EBDE80  addi r7, r11, -0x2180
	ctx.r[7].s64 = ctx.r[11].s64 + -8576;
	// 8266D2B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266D2BC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 8266D2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D2C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D2C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D2D0: 386A0468  addi r3, r10, 0x468
	ctx.r[3].s64 = ctx.r[10].s64 + 1128;
	// 8266D2D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D2F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D2F8: 4BDF9B29  bl 0x82466e20
	ctx.lr = 0x8266D2FC;
	sub_82466E20(ctx, base);
	// 8266D2FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D310 size=108
    let mut pc: u32 = 0x8266D310;
    'dispatch: loop {
        match pc {
            0x8266D310 => {
    //   block [0x8266D310..0x8266D37C)
	// 8266D310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D31C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D324: 38EBDE98  addi r7, r11, -0x2168
	ctx.r[7].s64 = ctx.r[11].s64 + -8552;
	// 8266D328: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266D32C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 8266D330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D334: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D340: 386A0498  addi r3, r10, 0x498
	ctx.r[3].s64 = ctx.r[10].s64 + 1176;
	// 8266D344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D34C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D35C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D368: 4BDF9AB9  bl 0x82466e20
	ctx.lr = 0x8266D36C;
	sub_82466E20(ctx, base);
	// 8266D36C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D380 size=108
    let mut pc: u32 = 0x8266D380;
    'dispatch: loop {
        match pc {
            0x8266D380 => {
    //   block [0x8266D380..0x8266D3EC)
	// 8266D380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D38C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D394: 38EBDEE0  addi r7, r11, -0x2120
	ctx.r[7].s64 = ctx.r[11].s64 + -8480;
	// 8266D398: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266D39C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 8266D3A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D3A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D3A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D3B0: 386A04C8  addi r3, r10, 0x4c8
	ctx.r[3].s64 = ctx.r[10].s64 + 1224;
	// 8266D3B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D3B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D3C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D3CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D3D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D3D8: 4BDF9A49  bl 0x82466e20
	ctx.lr = 0x8266D3DC;
	sub_82466E20(ctx, base);
	// 8266D3DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D3E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D3E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D3F0 size=108
    let mut pc: u32 = 0x8266D3F0;
    'dispatch: loop {
        match pc {
            0x8266D3F0 => {
    //   block [0x8266D3F0..0x8266D45C)
	// 8266D3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D3FC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D404: 38EBDF10  addi r7, r11, -0x20f0
	ctx.r[7].s64 = ctx.r[11].s64 + -8432;
	// 8266D408: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8266D40C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 8266D410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D414: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D420: 386A04F8  addi r3, r10, 0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + 1272;
	// 8266D424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D448: 4BDF99D9  bl 0x82466e20
	ctx.lr = 0x8266D44C;
	sub_82466E20(ctx, base);
	// 8266D44C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D460 size=108
    let mut pc: u32 = 0x8266D460;
    'dispatch: loop {
        match pc {
            0x8266D460 => {
    //   block [0x8266D460..0x8266D4CC)
	// 8266D460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D46C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D474: 38EBE030  addi r7, r11, -0x1fd0
	ctx.r[7].s64 = ctx.r[11].s64 + -8144;
	// 8266D478: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8266D47C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8266D480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D484: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D490: 386A0528  addi r3, r10, 0x528
	ctx.r[3].s64 = ctx.r[10].s64 + 1320;
	// 8266D494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D4B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D4B8: 4BDF9969  bl 0x82466e20
	ctx.lr = 0x8266D4BC;
	sub_82466E20(ctx, base);
	// 8266D4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D4D0 size=108
    let mut pc: u32 = 0x8266D4D0;
    'dispatch: loop {
        match pc {
            0x8266D4D0 => {
    //   block [0x8266D4D0..0x8266D53C)
	// 8266D4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D4DC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D4E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D4E4: 38EBE0C0  addi r7, r11, -0x1f40
	ctx.r[7].s64 = ctx.r[11].s64 + -8000;
	// 8266D4E8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8266D4EC: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8266D4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D4F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D4F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D500: 386A0558  addi r3, r10, 0x558
	ctx.r[3].s64 = ctx.r[10].s64 + 1368;
	// 8266D504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D528: 4BDF98F9  bl 0x82466e20
	ctx.lr = 0x8266D52C;
	sub_82466E20(ctx, base);
	// 8266D52C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D540 size=108
    let mut pc: u32 = 0x8266D540;
    'dispatch: loop {
        match pc {
            0x8266D540 => {
    //   block [0x8266D540..0x8266D5AC)
	// 8266D540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D54C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D554: 38EBE180  addi r7, r11, -0x1e80
	ctx.r[7].s64 = ctx.r[11].s64 + -7808;
	// 8266D558: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8266D55C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8266D560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D564: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D570: 386A0588  addi r3, r10, 0x588
	ctx.r[3].s64 = ctx.r[10].s64 + 1416;
	// 8266D574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D598: 4BDF9889  bl 0x82466e20
	ctx.lr = 0x8266D59C;
	sub_82466E20(ctx, base);
	// 8266D59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D5B0 size=108
    let mut pc: u32 = 0x8266D5B0;
    'dispatch: loop {
        match pc {
            0x8266D5B0 => {
    //   block [0x8266D5B0..0x8266D61C)
	// 8266D5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D5BC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D5C4: 38EBE258  addi r7, r11, -0x1da8
	ctx.r[7].s64 = ctx.r[11].s64 + -7592;
	// 8266D5C8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8266D5CC: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 8266D5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D5D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D5D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D5E0: 386A05B8  addi r3, r10, 0x5b8
	ctx.r[3].s64 = ctx.r[10].s64 + 1464;
	// 8266D5E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D608: 4BDF9819  bl 0x82466e20
	ctx.lr = 0x8266D60C;
	sub_82466E20(ctx, base);
	// 8266D60C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D620 size=108
    let mut pc: u32 = 0x8266D620;
    'dispatch: loop {
        match pc {
            0x8266D620 => {
    //   block [0x8266D620..0x8266D68C)
	// 8266D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D62C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D634: 38EBE318  addi r7, r11, -0x1ce8
	ctx.r[7].s64 = ctx.r[11].s64 + -7400;
	// 8266D638: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8266D63C: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 8266D640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D644: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D650: 386A05E8  addi r3, r10, 0x5e8
	ctx.r[3].s64 = ctx.r[10].s64 + 1512;
	// 8266D654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D678: 4BDF97A9  bl 0x82466e20
	ctx.lr = 0x8266D67C;
	sub_82466E20(ctx, base);
	// 8266D67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D690 size=112
    let mut pc: u32 = 0x8266D690;
    'dispatch: loop {
        match pc {
            0x8266D690 => {
    //   block [0x8266D690..0x8266D700)
	// 8266D690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D69C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266D6A0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8266D6A4: 38EAE3C0  addi r7, r10, -0x1c40
	ctx.r[7].s64 = ctx.r[10].s64 + -7232;
	// 8266D6A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D6AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8266D6B0: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8266D6B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D6B8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D6BC: 396B15C8  addi r11, r11, 0x15c8
	ctx.r[11].s64 = ctx.r[11].s64 + 5576;
	// 8266D6C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D6C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D6C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D6CC: 386A0618  addi r3, r10, 0x618
	ctx.r[3].s64 = ctx.r[10].s64 + 1560;
	// 8266D6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D6D4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8266D6D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D6DC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8266D6E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D6E4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D6E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D6EC: 4BDF9735  bl 0x82466e20
	ctx.lr = 0x8266D6F0;
	sub_82466E20(ctx, base);
	// 8266D6F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D700 size=108
    let mut pc: u32 = 0x8266D700;
    'dispatch: loop {
        match pc {
            0x8266D700 => {
    //   block [0x8266D700..0x8266D76C)
	// 8266D700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D70C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D714: 38EBE4E0  addi r7, r11, -0x1b20
	ctx.r[7].s64 = ctx.r[11].s64 + -6944;
	// 8266D718: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266D71C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 8266D720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D724: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D728: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D730: 386A0648  addi r3, r10, 0x648
	ctx.r[3].s64 = ctx.r[10].s64 + 1608;
	// 8266D734: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D754: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D758: 4BDF96C9  bl 0x82466e20
	ctx.lr = 0x8266D75C;
	sub_82466E20(ctx, base);
	// 8266D75C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D770 size=108
    let mut pc: u32 = 0x8266D770;
    'dispatch: loop {
        match pc {
            0x8266D770 => {
    //   block [0x8266D770..0x8266D7DC)
	// 8266D770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D77C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D784: 38EBE540  addi r7, r11, -0x1ac0
	ctx.r[7].s64 = ctx.r[11].s64 + -6848;
	// 8266D788: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8266D78C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 8266D790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D794: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D798: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D7A0: 386A0678  addi r3, r10, 0x678
	ctx.r[3].s64 = ctx.r[10].s64 + 1656;
	// 8266D7A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D7C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D7C8: 4BDF9659  bl 0x82466e20
	ctx.lr = 0x8266D7CC;
	sub_82466E20(ctx, base);
	// 8266D7CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D7E0 size=108
    let mut pc: u32 = 0x8266D7E0;
    'dispatch: loop {
        match pc {
            0x8266D7E0 => {
    //   block [0x8266D7E0..0x8266D84C)
	// 8266D7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D7EC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D7F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D7F4: 38EBE648  addi r7, r11, -0x19b8
	ctx.r[7].s64 = ctx.r[11].s64 + -6584;
	// 8266D7F8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8266D7FC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 8266D800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D804: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D810: 386A06A8  addi r3, r10, 0x6a8
	ctx.r[3].s64 = ctx.r[10].s64 + 1704;
	// 8266D814: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D838: 4BDF95E9  bl 0x82466e20
	ctx.lr = 0x8266D83C;
	sub_82466E20(ctx, base);
	// 8266D83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D850 size=108
    let mut pc: u32 = 0x8266D850;
    'dispatch: loop {
        match pc {
            0x8266D850 => {
    //   block [0x8266D850..0x8266D8BC)
	// 8266D850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D85C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D864: 38EBE720  addi r7, r11, -0x18e0
	ctx.r[7].s64 = ctx.r[11].s64 + -6368;
	// 8266D868: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266D86C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 8266D870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D874: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D880: 386A06D8  addi r3, r10, 0x6d8
	ctx.r[3].s64 = ctx.r[10].s64 + 1752;
	// 8266D884: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D8A8: 4BDF9579  bl 0x82466e20
	ctx.lr = 0x8266D8AC;
	sub_82466E20(ctx, base);
	// 8266D8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D8C0 size=108
    let mut pc: u32 = 0x8266D8C0;
    'dispatch: loop {
        match pc {
            0x8266D8C0 => {
    //   block [0x8266D8C0..0x8266D92C)
	// 8266D8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D8CC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D8D4: 38EBE750  addi r7, r11, -0x18b0
	ctx.r[7].s64 = ctx.r[11].s64 + -6320;
	// 8266D8D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266D8DC: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 8266D8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D8E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D8F0: 386A0708  addi r3, r10, 0x708
	ctx.r[3].s64 = ctx.r[10].s64 + 1800;
	// 8266D8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D918: 4BDF9509  bl 0x82466e20
	ctx.lr = 0x8266D91C;
	sub_82466E20(ctx, base);
	// 8266D91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D930 size=108
    let mut pc: u32 = 0x8266D930;
    'dispatch: loop {
        match pc {
            0x8266D930 => {
    //   block [0x8266D930..0x8266D99C)
	// 8266D930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D93C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D944: 38EBE768  addi r7, r11, -0x1898
	ctx.r[7].s64 = ctx.r[11].s64 + -6296;
	// 8266D948: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266D94C: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 8266D950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D954: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D960: 386A0738  addi r3, r10, 0x738
	ctx.r[3].s64 = ctx.r[10].s64 + 1848;
	// 8266D964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D988: 4BDF9499  bl 0x82466e20
	ctx.lr = 0x8266D98C;
	sub_82466E20(ctx, base);
	// 8266D98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266D990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266D994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266D998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266D9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266D9A0 size=108
    let mut pc: u32 = 0x8266D9A0;
    'dispatch: loop {
        match pc {
            0x8266D9A0 => {
    //   block [0x8266D9A0..0x8266DA0C)
	// 8266D9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266D9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266D9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266D9AC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266D9B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266D9B4: 38EBE7B0  addi r7, r11, -0x1850
	ctx.r[7].s64 = ctx.r[11].s64 + -6224;
	// 8266D9B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266D9BC: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 8266D9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266D9C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266D9C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266D9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266D9D0: 386A0768  addi r3, r10, 0x768
	ctx.r[3].s64 = ctx.r[10].s64 + 1896;
	// 8266D9D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266D9D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266D9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266D9E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266D9E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266D9E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266D9EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266D9F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266D9F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266D9F8: 4BDF9429  bl 0x82466e20
	ctx.lr = 0x8266D9FC;
	sub_82466E20(ctx, base);
	// 8266D9FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DA10 size=112
    let mut pc: u32 = 0x8266DA10;
    'dispatch: loop {
        match pc {
            0x8266DA10 => {
    //   block [0x8266DA10..0x8266DA80)
	// 8266DA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DA1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DA20: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266DA24: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266DA28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DA2C: 390BE7C8  addi r8, r11, -0x1838
	ctx.r[8].s64 = ctx.r[11].s64 + -6200;
	// 8266DA30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266DA34: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 8266DA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DA3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266DA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DA48: 386A0798  addi r3, r10, 0x798
	ctx.r[3].s64 = ctx.r[10].s64 + 1944;
	// 8266DA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266DA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266DA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DA6C: 4BDF93B5  bl 0x82466e20
	ctx.lr = 0x8266DA70;
	sub_82466E20(ctx, base);
	// 8266DA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DA80 size=108
    let mut pc: u32 = 0x8266DA80;
    'dispatch: loop {
        match pc {
            0x8266DA80 => {
    //   block [0x8266DA80..0x8266DAEC)
	// 8266DA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DA8C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266DA90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DA94: 38EBE810  addi r7, r11, -0x17f0
	ctx.r[7].s64 = ctx.r[11].s64 + -6128;
	// 8266DA98: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266DA9C: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 8266DAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DAA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DAA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266DAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DAB0: 386A07C8  addi r3, r10, 0x7c8
	ctx.r[3].s64 = ctx.r[10].s64 + 1992;
	// 8266DAB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266DAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266DAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DAD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266DAD8: 4BDF9349  bl 0x82466e20
	ctx.lr = 0x8266DADC;
	sub_82466E20(ctx, base);
	// 8266DADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DAF0 size=112
    let mut pc: u32 = 0x8266DAF0;
    'dispatch: loop {
        match pc {
            0x8266DAF0 => {
    //   block [0x8266DAF0..0x8266DB60)
	// 8266DAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DAFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DB00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266DB04: 38AA07C8  addi r5, r10, 0x7c8
	ctx.r[5].s64 = ctx.r[10].s64 + 1992;
	// 8266DB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DB0C: 390BE870  addi r8, r11, -0x1790
	ctx.r[8].s64 = ctx.r[11].s64 + -6032;
	// 8266DB10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266DB14: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 8266DB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DB1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266DB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DB28: 386A07F8  addi r3, r10, 0x7f8
	ctx.r[3].s64 = ctx.r[10].s64 + 2040;
	// 8266DB2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266DB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266DB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DB4C: 4BDF92D5  bl 0x82466e20
	ctx.lr = 0x8266DB50;
	sub_82466E20(ctx, base);
	// 8266DB50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DB60 size=96
    let mut pc: u32 = 0x8266DB60;
    'dispatch: loop {
        match pc {
            0x8266DB60 => {
    //   block [0x8266DB60..0x8266DBC0)
	// 8266DB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DB6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DB74: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 8266DB78: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DB80: 386A0828  addi r3, r10, 0x828
	ctx.r[3].s64 = ctx.r[10].s64 + 2088;
	// 8266DB84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DB8C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266DB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DBA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266DBA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266DBA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266DBAC: 4BDF9275  bl 0x82466e20
	ctx.lr = 0x8266DBB0;
	sub_82466E20(ctx, base);
	// 8266DBB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DBC0 size=112
    let mut pc: u32 = 0x8266DBC0;
    'dispatch: loop {
        match pc {
            0x8266DBC0 => {
    //   block [0x8266DBC0..0x8266DC30)
	// 8266DBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DBCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DBD0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266DBD4: 38AA2058  addi r5, r10, 0x2058
	ctx.r[5].s64 = ctx.r[10].s64 + 8280;
	// 8266DBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DBDC: 390BE8B8  addi r8, r11, -0x1748
	ctx.r[8].s64 = ctx.r[11].s64 + -5960;
	// 8266DBE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266DBE4: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 8266DBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DBEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DBF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266DBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DBF8: 386A0858  addi r3, r10, 0x858
	ctx.r[3].s64 = ctx.r[10].s64 + 2136;
	// 8266DBFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266DC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266DC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DC1C: 4BDF9205  bl 0x82466e20
	ctx.lr = 0x8266DC20;
	sub_82466E20(ctx, base);
	// 8266DC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DC30 size=96
    let mut pc: u32 = 0x8266DC30;
    'dispatch: loop {
        match pc {
            0x8266DC30 => {
    //   block [0x8266DC30..0x8266DC90)
	// 8266DC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DC3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DC40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DC44: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 8266DC48: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DC50: 386A0888  addi r3, r10, 0x888
	ctx.r[3].s64 = ctx.r[10].s64 + 2184;
	// 8266DC54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DC58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DC5C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266DC60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DC68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DC6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DC70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266DC74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266DC78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266DC7C: 4BDF91A5  bl 0x82466e20
	ctx.lr = 0x8266DC80;
	sub_82466E20(ctx, base);
	// 8266DC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DC90 size=100
    let mut pc: u32 = 0x8266DC90;
    'dispatch: loop {
        match pc {
            0x8266DC90 => {
    //   block [0x8266DC90..0x8266DCF4)
	// 8266DC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DC9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DCA4: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266DCA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DCB0: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 8266DCB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DCBC: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266DCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DCC4: 386A08B8  addi r3, r10, 0x8b8
	ctx.r[3].s64 = ctx.r[10].s64 + 2232;
	// 8266DCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DCCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DCD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266DCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DCD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266DCDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DCE0: 4BDF9141  bl 0x82466e20
	ctx.lr = 0x8266DCE4;
	sub_82466E20(ctx, base);
	// 8266DCE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DCE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DCEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DCF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DCF8 size=96
    let mut pc: u32 = 0x8266DCF8;
    'dispatch: loop {
        match pc {
            0x8266DCF8 => {
    //   block [0x8266DCF8..0x8266DD58)
	// 8266DCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DD04: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DD0C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 8266DD10: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DD14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DD18: 386A08E8  addi r3, r10, 0x8e8
	ctx.r[3].s64 = ctx.r[10].s64 + 2280;
	// 8266DD1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DD24: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266DD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DD2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DD38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266DD3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266DD40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266DD44: 4BDF90DD  bl 0x82466e20
	ctx.lr = 0x8266DD48;
	sub_82466E20(ctx, base);
	// 8266DD48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DD58 size=112
    let mut pc: u32 = 0x8266DD58;
    'dispatch: loop {
        match pc {
            0x8266DD58 => {
    //   block [0x8266DD58..0x8266DDC8)
	// 8266DD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DD64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DD68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266DD6C: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 8266DD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DD74: 390BE918  addi r8, r11, -0x16e8
	ctx.r[8].s64 = ctx.r[11].s64 + -5864;
	// 8266DD78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266DD7C: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 8266DD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DD84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DD88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266DD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DD90: 386A0918  addi r3, r10, 0x918
	ctx.r[3].s64 = ctx.r[10].s64 + 2328;
	// 8266DD94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266DD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DD9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DDA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266DDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DDB4: 4BDF906D  bl 0x82466e20
	ctx.lr = 0x8266DDB8;
	sub_82466E20(ctx, base);
	// 8266DDB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DDC8 size=112
    let mut pc: u32 = 0x8266DDC8;
    'dispatch: loop {
        match pc {
            0x8266DDC8 => {
    //   block [0x8266DDC8..0x8266DE38)
	// 8266DDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DDD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DDD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DDD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266DDDC: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 8266DDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DDE4: 390BE948  addi r8, r11, -0x16b8
	ctx.r[8].s64 = ctx.r[11].s64 + -5816;
	// 8266DDE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266DDEC: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 8266DDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DDF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DDF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266DDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DE00: 386A0948  addi r3, r10, 0x948
	ctx.r[3].s64 = ctx.r[10].s64 + 2376;
	// 8266DE04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266DE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DE0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266DE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DE24: 4BDF8FFD  bl 0x82466e20
	ctx.lr = 0x8266DE28;
	sub_82466E20(ctx, base);
	// 8266DE28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DE38 size=100
    let mut pc: u32 = 0x8266DE38;
    'dispatch: loop {
        match pc {
            0x8266DE38 => {
    //   block [0x8266DE38..0x8266DE9C)
	// 8266DE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DE44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DE48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DE4C: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 8266DE50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DE58: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8266DE5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DE64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266DE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DE6C: 386A0978  addi r3, r10, 0x978
	ctx.r[3].s64 = ctx.r[10].s64 + 2424;
	// 8266DE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DE74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DE78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266DE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DE80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266DE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DE88: 4BDF8F99  bl 0x82466e20
	ctx.lr = 0x8266DE8C;
	sub_82466E20(ctx, base);
	// 8266DE8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DE90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DE94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DE98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DEA0 size=96
    let mut pc: u32 = 0x8266DEA0;
    'dispatch: loop {
        match pc {
            0x8266DEA0 => {
    //   block [0x8266DEA0..0x8266DF00)
	// 8266DEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DEAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DEB4: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 8266DEB8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DEC0: 386A09A8  addi r3, r10, 0x9a8
	ctx.r[3].s64 = ctx.r[10].s64 + 2472;
	// 8266DEC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DECC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266DED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DEE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266DEE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266DEE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266DEEC: 4BDF8F35  bl 0x82466e20
	ctx.lr = 0x8266DEF0;
	sub_82466E20(ctx, base);
	// 8266DEF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DEF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DEF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DF00 size=112
    let mut pc: u32 = 0x8266DF00;
    'dispatch: loop {
        match pc {
            0x8266DF00 => {
    //   block [0x8266DF00..0x8266DF70)
	// 8266DF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DF0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DF10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266DF14: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266DF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DF1C: 390BE960  addi r8, r11, -0x16a0
	ctx.r[8].s64 = ctx.r[11].s64 + -5792;
	// 8266DF20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266DF24: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 8266DF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DF2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DF30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266DF34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DF38: 386A09D8  addi r3, r10, 0x9d8
	ctx.r[3].s64 = ctx.r[10].s64 + 2520;
	// 8266DF3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266DF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266DF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DF5C: 4BDF8EC5  bl 0x82466e20
	ctx.lr = 0x8266DF60;
	sub_82466E20(ctx, base);
	// 8266DF60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DF70 size=108
    let mut pc: u32 = 0x8266DF70;
    'dispatch: loop {
        match pc {
            0x8266DF70 => {
    //   block [0x8266DF70..0x8266DFDC)
	// 8266DF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DF7C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266DF80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DF84: 38EBE978  addi r7, r11, -0x1688
	ctx.r[7].s64 = ctx.r[11].s64 + -5768;
	// 8266DF88: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266DF8C: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8266DF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266DF94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DF98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266DF9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266DFA0: 386A0A08  addi r3, r10, 0xa08
	ctx.r[3].s64 = ctx.r[10].s64 + 2568;
	// 8266DFA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266DFA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266DFAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266DFB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266DFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266DFB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266DFBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266DFC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266DFC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266DFC8: 4BDF8E59  bl 0x82466e20
	ctx.lr = 0x8266DFCC;
	sub_82466E20(ctx, base);
	// 8266DFCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266DFD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266DFD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266DFD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266DFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266DFE0 size=112
    let mut pc: u32 = 0x8266DFE0;
    'dispatch: loop {
        match pc {
            0x8266DFE0 => {
    //   block [0x8266DFE0..0x8266E050)
	// 8266DFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266DFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266DFE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266DFEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266DFF0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266DFF4: 38AA0B28  addi r5, r10, 0xb28
	ctx.r[5].s64 = ctx.r[10].s64 + 2856;
	// 8266DFF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266DFFC: 390BE9D8  addi r8, r11, -0x1628
	ctx.r[8].s64 = ctx.r[11].s64 + -5672;
	// 8266E000: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266E004: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8266E008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E00C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E010: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E018: 386A0A38  addi r3, r10, 0xa38
	ctx.r[3].s64 = ctx.r[10].s64 + 2616;
	// 8266E01C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E03C: 4BDF8DE5  bl 0x82466e20
	ctx.lr = 0x8266E040;
	sub_82466E20(ctx, base);
	// 8266E040: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E050 size=112
    let mut pc: u32 = 0x8266E050;
    'dispatch: loop {
        match pc {
            0x8266E050 => {
    //   block [0x8266E050..0x8266E0C0)
	// 8266E050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E05C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E060: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E064: 38AA09D8  addi r5, r10, 0x9d8
	ctx.r[5].s64 = ctx.r[10].s64 + 2520;
	// 8266E068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E06C: 390BE9F0  addi r8, r11, -0x1610
	ctx.r[8].s64 = ctx.r[11].s64 + -5648;
	// 8266E070: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266E074: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8266E078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E07C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E088: 386A0A68  addi r3, r10, 0xa68
	ctx.r[3].s64 = ctx.r[10].s64 + 2664;
	// 8266E08C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E09C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E0A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E0AC: 4BDF8D75  bl 0x82466e20
	ctx.lr = 0x8266E0B0;
	sub_82466E20(ctx, base);
	// 8266E0B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E0B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E0B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E0BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E0C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E0C0 size=112
    let mut pc: u32 = 0x8266E0C0;
    'dispatch: loop {
        match pc {
            0x8266E0C0 => {
    //   block [0x8266E0C0..0x8266E130)
	// 8266E0C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E0C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E0C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E0CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E0D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E0D4: 38AA09D8  addi r5, r10, 0x9d8
	ctx.r[5].s64 = ctx.r[10].s64 + 2520;
	// 8266E0D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E0DC: 390BEA20  addi r8, r11, -0x15e0
	ctx.r[8].s64 = ctx.r[11].s64 + -5600;
	// 8266E0E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266E0E4: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8266E0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E0EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E0F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E0F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E0F8: 386A0A98  addi r3, r10, 0xa98
	ctx.r[3].s64 = ctx.r[10].s64 + 2712;
	// 8266E0FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E11C: 4BDF8D05  bl 0x82466e20
	ctx.lr = 0x8266E120;
	sub_82466E20(ctx, base);
	// 8266E120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E130 size=112
    let mut pc: u32 = 0x8266E130;
    'dispatch: loop {
        match pc {
            0x8266E130 => {
    //   block [0x8266E130..0x8266E1A0)
	// 8266E130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E13C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E140: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E144: 38AA0B28  addi r5, r10, 0xb28
	ctx.r[5].s64 = ctx.r[10].s64 + 2856;
	// 8266E148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E14C: 390BEA38  addi r8, r11, -0x15c8
	ctx.r[8].s64 = ctx.r[11].s64 + -5576;
	// 8266E150: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266E154: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8266E158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E15C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E168: 386A0AC8  addi r3, r10, 0xac8
	ctx.r[3].s64 = ctx.r[10].s64 + 2760;
	// 8266E16C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E17C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E18C: 4BDF8C95  bl 0x82466e20
	ctx.lr = 0x8266E190;
	sub_82466E20(ctx, base);
	// 8266E190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E1A0 size=112
    let mut pc: u32 = 0x8266E1A0;
    'dispatch: loop {
        match pc {
            0x8266E1A0 => {
    //   block [0x8266E1A0..0x8266E210)
	// 8266E1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E1A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E1AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E1B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E1B4: 38AA09D8  addi r5, r10, 0x9d8
	ctx.r[5].s64 = ctx.r[10].s64 + 2520;
	// 8266E1B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E1BC: 390BEA68  addi r8, r11, -0x1598
	ctx.r[8].s64 = ctx.r[11].s64 + -5528;
	// 8266E1C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266E1C4: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8266E1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E1CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E1D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E1D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E1D8: 386A0AF8  addi r3, r10, 0xaf8
	ctx.r[3].s64 = ctx.r[10].s64 + 2808;
	// 8266E1DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E1E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E1F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E1FC: 4BDF8C25  bl 0x82466e20
	ctx.lr = 0x8266E200;
	sub_82466E20(ctx, base);
	// 8266E200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E210 size=112
    let mut pc: u32 = 0x8266E210;
    'dispatch: loop {
        match pc {
            0x8266E210 => {
    //   block [0x8266E210..0x8266E280)
	// 8266E210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E21C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E220: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E224: 38AA0FD8  addi r5, r10, 0xfd8
	ctx.r[5].s64 = ctx.r[10].s64 + 4056;
	// 8266E228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E22C: 390BEA80  addi r8, r11, -0x1580
	ctx.r[8].s64 = ctx.r[11].s64 + -5504;
	// 8266E230: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266E234: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8266E238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E23C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E248: 386A0B28  addi r3, r10, 0xb28
	ctx.r[3].s64 = ctx.r[10].s64 + 2856;
	// 8266E24C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E26C: 4BDF8BB5  bl 0x82466e20
	ctx.lr = 0x8266E270;
	sub_82466E20(ctx, base);
	// 8266E270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E27C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E280 size=112
    let mut pc: u32 = 0x8266E280;
    'dispatch: loop {
        match pc {
            0x8266E280 => {
    //   block [0x8266E280..0x8266E2F0)
	// 8266E280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E28C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E290: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E294: 38AA0D38  addi r5, r10, 0xd38
	ctx.r[5].s64 = ctx.r[10].s64 + 3384;
	// 8266E298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E29C: 390BEA98  addi r8, r11, -0x1568
	ctx.r[8].s64 = ctx.r[11].s64 + -5480;
	// 8266E2A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266E2A4: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8266E2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E2AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E2B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E2B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E2B8: 386A0B58  addi r3, r10, 0xb58
	ctx.r[3].s64 = ctx.r[10].s64 + 2904;
	// 8266E2BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E2C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E2C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E2C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E2CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E2D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E2D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E2D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E2DC: 4BDF8B45  bl 0x82466e20
	ctx.lr = 0x8266E2E0;
	sub_82466E20(ctx, base);
	// 8266E2E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E2E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E2E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E2EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E2F0 size=112
    let mut pc: u32 = 0x8266E2F0;
    'dispatch: loop {
        match pc {
            0x8266E2F0 => {
    //   block [0x8266E2F0..0x8266E360)
	// 8266E2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E2F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E2FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E300: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E304: 38AA0AF8  addi r5, r10, 0xaf8
	ctx.r[5].s64 = ctx.r[10].s64 + 2808;
	// 8266E308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E30C: 390BEAB0  addi r8, r11, -0x1550
	ctx.r[8].s64 = ctx.r[11].s64 + -5456;
	// 8266E310: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266E314: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8266E318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E31C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E320: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E328: 386A0B88  addi r3, r10, 0xb88
	ctx.r[3].s64 = ctx.r[10].s64 + 2952;
	// 8266E32C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E34C: 4BDF8AD5  bl 0x82466e20
	ctx.lr = 0x8266E350;
	sub_82466E20(ctx, base);
	// 8266E350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E360 size=112
    let mut pc: u32 = 0x8266E360;
    'dispatch: loop {
        match pc {
            0x8266E360 => {
    //   block [0x8266E360..0x8266E3D0)
	// 8266E360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E36C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E370: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E374: 38AA0B28  addi r5, r10, 0xb28
	ctx.r[5].s64 = ctx.r[10].s64 + 2856;
	// 8266E378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E37C: 390BEAF8  addi r8, r11, -0x1508
	ctx.r[8].s64 = ctx.r[11].s64 + -5384;
	// 8266E380: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266E384: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8266E388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E38C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E398: 386A0BB8  addi r3, r10, 0xbb8
	ctx.r[3].s64 = ctx.r[10].s64 + 3000;
	// 8266E39C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E3AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E3BC: 4BDF8A65  bl 0x82466e20
	ctx.lr = 0x8266E3C0;
	sub_82466E20(ctx, base);
	// 8266E3C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E3D0 size=112
    let mut pc: u32 = 0x8266E3D0;
    'dispatch: loop {
        match pc {
            0x8266E3D0 => {
    //   block [0x8266E3D0..0x8266E440)
	// 8266E3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E3DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E3E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E3E4: 38AA0B28  addi r5, r10, 0xb28
	ctx.r[5].s64 = ctx.r[10].s64 + 2856;
	// 8266E3E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E3EC: 390BEB28  addi r8, r11, -0x14d8
	ctx.r[8].s64 = ctx.r[11].s64 + -5336;
	// 8266E3F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266E3F4: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8266E3F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E3FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E408: 386A0BE8  addi r3, r10, 0xbe8
	ctx.r[3].s64 = ctx.r[10].s64 + 3048;
	// 8266E40C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E42C: 4BDF89F5  bl 0x82466e20
	ctx.lr = 0x8266E430;
	sub_82466E20(ctx, base);
	// 8266E430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E440 size=108
    let mut pc: u32 = 0x8266E440;
    'dispatch: loop {
        match pc {
            0x8266E440 => {
    //   block [0x8266E440..0x8266E4AC)
	// 8266E440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E44C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E454: 38EBEB58  addi r7, r11, -0x14a8
	ctx.r[7].s64 = ctx.r[11].s64 + -5288;
	// 8266E458: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266E45C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8266E460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E464: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266E46C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E470: 386A0C18  addi r3, r10, 0xc18
	ctx.r[3].s64 = ctx.r[10].s64 + 3096;
	// 8266E474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266E478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E48C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266E498: 4BDF8989  bl 0x82466e20
	ctx.lr = 0x8266E49C;
	sub_82466E20(ctx, base);
	// 8266E49C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E4A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E4A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E4A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E4B0 size=112
    let mut pc: u32 = 0x8266E4B0;
    'dispatch: loop {
        match pc {
            0x8266E4B0 => {
    //   block [0x8266E4B0..0x8266E520)
	// 8266E4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E4BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E4C0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E4C4: 38AA0B28  addi r5, r10, 0xb28
	ctx.r[5].s64 = ctx.r[10].s64 + 2856;
	// 8266E4C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E4CC: 390BEBA0  addi r8, r11, -0x1460
	ctx.r[8].s64 = ctx.r[11].s64 + -5216;
	// 8266E4D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8266E4D4: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8266E4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E4DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E4E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E4E8: 386A0C48  addi r3, r10, 0xc48
	ctx.r[3].s64 = ctx.r[10].s64 + 3144;
	// 8266E4EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E50C: 4BDF8915  bl 0x82466e20
	ctx.lr = 0x8266E510;
	sub_82466E20(ctx, base);
	// 8266E510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E520 size=116
    let mut pc: u32 = 0x8266E520;
    'dispatch: loop {
        match pc {
            0x8266E520 => {
    //   block [0x8266E520..0x8266E594)
	// 8266E520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E52C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E530: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266E534: 390BEC20  addi r8, r11, -0x13e0
	ctx.r[8].s64 = ctx.r[11].s64 + -5088;
	// 8266E538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E53C: 392A1650  addi r9, r10, 0x1650
	ctx.r[9].s64 = ctx.r[10].s64 + 5712;
	// 8266E540: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E544: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8266E548: 38AA0B28  addi r5, r10, 0xb28
	ctx.r[5].s64 = ctx.r[10].s64 + 2856;
	// 8266E54C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E554: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E564: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8266E568: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8266E56C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266E570: 386B0C78  addi r3, r11, 0xc78
	ctx.r[3].s64 = ctx.r[11].s64 + 3192;
	// 8266E574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266E578: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E580: 4BDF88A1  bl 0x82466e20
	ctx.lr = 0x8266E584;
	sub_82466E20(ctx, base);
	// 8266E584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E598 size=100
    let mut pc: u32 = 0x8266E598;
    'dispatch: loop {
        match pc {
            0x8266E598 => {
    //   block [0x8266E598..0x8266E5FC)
	// 8266E598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E5A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E5A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E5AC: 38AA0DC8  addi r5, r10, 0xdc8
	ctx.r[5].s64 = ctx.r[10].s64 + 3528;
	// 8266E5B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E5B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E5B8: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8266E5BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E5C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E5CC: 386A0CA8  addi r3, r10, 0xca8
	ctx.r[3].s64 = ctx.r[10].s64 + 3240;
	// 8266E5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E5D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E5D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266E5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E5E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266E5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E5E8: 4BDF8839  bl 0x82466e20
	ctx.lr = 0x8266E5EC;
	sub_82466E20(ctx, base);
	// 8266E5EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E5F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E5F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E5F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E600 size=100
    let mut pc: u32 = 0x8266E600;
    'dispatch: loop {
        match pc {
            0x8266E600 => {
    //   block [0x8266E600..0x8266E664)
	// 8266E600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E60C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E614: 38AA09D8  addi r5, r10, 0x9d8
	ctx.r[5].s64 = ctx.r[10].s64 + 2520;
	// 8266E618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E620: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8266E624: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E634: 386A0CD8  addi r3, r10, 0xcd8
	ctx.r[3].s64 = ctx.r[10].s64 + 3288;
	// 8266E638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E63C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E640: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266E644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E648: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266E64C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E650: 4BDF87D1  bl 0x82466e20
	ctx.lr = 0x8266E654;
	sub_82466E20(ctx, base);
	// 8266E654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E668 size=108
    let mut pc: u32 = 0x8266E668;
    'dispatch: loop {
        match pc {
            0x8266E668 => {
    //   block [0x8266E668..0x8266E6D4)
	// 8266E668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E674: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E67C: 38EBEC98  addi r7, r11, -0x1368
	ctx.r[7].s64 = ctx.r[11].s64 + -4968;
	// 8266E680: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266E684: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8266E688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E68C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266E694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E698: 386A0D08  addi r3, r10, 0xd08
	ctx.r[3].s64 = ctx.r[10].s64 + 3336;
	// 8266E69C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266E6A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E6A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E6B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E6B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E6BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266E6C0: 4BDF8761  bl 0x82466e20
	ctx.lr = 0x8266E6C4;
	sub_82466E20(ctx, base);
	// 8266E6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E6D8 size=112
    let mut pc: u32 = 0x8266E6D8;
    'dispatch: loop {
        match pc {
            0x8266E6D8 => {
    //   block [0x8266E6D8..0x8266E748)
	// 8266E6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E6E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E6E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E6EC: 38AA0AF8  addi r5, r10, 0xaf8
	ctx.r[5].s64 = ctx.r[10].s64 + 2808;
	// 8266E6F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E6F4: 390BECC8  addi r8, r11, -0x1338
	ctx.r[8].s64 = ctx.r[11].s64 + -4920;
	// 8266E6F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266E6FC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8266E700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E704: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E710: 386A0D38  addi r3, r10, 0xd38
	ctx.r[3].s64 = ctx.r[10].s64 + 3384;
	// 8266E714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E71C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E72C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E734: 4BDF86ED  bl 0x82466e20
	ctx.lr = 0x8266E738;
	sub_82466E20(ctx, base);
	// 8266E738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E748 size=108
    let mut pc: u32 = 0x8266E748;
    'dispatch: loop {
        match pc {
            0x8266E748 => {
    //   block [0x8266E748..0x8266E7B4)
	// 8266E748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E754: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E75C: 38EBECE0  addi r7, r11, -0x1320
	ctx.r[7].s64 = ctx.r[11].s64 + -4896;
	// 8266E760: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266E764: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8266E768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E76C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266E774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E778: 386A0D68  addi r3, r10, 0xd68
	ctx.r[3].s64 = ctx.r[10].s64 + 3432;
	// 8266E77C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266E780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E79C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266E7A0: 4BDF8681  bl 0x82466e20
	ctx.lr = 0x8266E7A4;
	sub_82466E20(ctx, base);
	// 8266E7A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266E7B8 size=28
    let mut pc: u32 = 0x8266E7B8;
    'dispatch: loop {
        match pc {
            0x8266E7B8 => {
    //   block [0x8266E7B8..0x8266E7D4)
	// 8266E7B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E7BC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266E7C0: 394A2308  addi r10, r10, 0x2308
	ctx.r[10].s64 = ctx.r[10].s64 + 8968;
	// 8266E7C4: 816BEC1C  lwz r11, -0x13e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5092 as u32) ) } as u64;
	// 8266E7C8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8266E7CC: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8266E7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E7D8 size=108
    let mut pc: u32 = 0x8266E7D8;
    'dispatch: loop {
        match pc {
            0x8266E7D8 => {
    //   block [0x8266E7D8..0x8266E844)
	// 8266E7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E7E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E7E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E7EC: 38EB2308  addi r7, r11, 0x2308
	ctx.r[7].s64 = ctx.r[11].s64 + 8968;
	// 8266E7F0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8266E7F4: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8266E7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E7FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266E804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E808: 386A0D98  addi r3, r10, 0xd98
	ctx.r[3].s64 = ctx.r[10].s64 + 3480;
	// 8266E80C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266E810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E82C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266E830: 4BDF85F1  bl 0x82466e20
	ctx.lr = 0x8266E834;
	sub_82466E20(ctx, base);
	// 8266E834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E848 size=116
    let mut pc: u32 = 0x8266E848;
    'dispatch: loop {
        match pc {
            0x8266E848 => {
    //   block [0x8266E848..0x8266E8BC)
	// 8266E848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E854: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E858: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266E85C: 390BED00  addi r8, r11, -0x1300
	ctx.r[8].s64 = ctx.r[11].s64 + -4864;
	// 8266E860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E864: 392A16A4  addi r9, r10, 0x16a4
	ctx.r[9].s64 = ctx.r[10].s64 + 5796;
	// 8266E868: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E86C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8266E870: 38AA0AF8  addi r5, r10, 0xaf8
	ctx.r[5].s64 = ctx.r[10].s64 + 2808;
	// 8266E874: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E87C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E88C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8266E890: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8266E894: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266E898: 386B0DC8  addi r3, r11, 0xdc8
	ctx.r[3].s64 = ctx.r[11].s64 + 3528;
	// 8266E89C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8266E8A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E8A8: 4BDF8579  bl 0x82466e20
	ctx.lr = 0x8266E8AC;
	sub_82466E20(ctx, base);
	// 8266E8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E8C0 size=112
    let mut pc: u32 = 0x8266E8C0;
    'dispatch: loop {
        match pc {
            0x8266E8C0 => {
    //   block [0x8266E8C0..0x8266E930)
	// 8266E8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E8CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E8D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E8D4: 38AA0A98  addi r5, r10, 0xa98
	ctx.r[5].s64 = ctx.r[10].s64 + 2712;
	// 8266E8D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E8DC: 390BED60  addi r8, r11, -0x12a0
	ctx.r[8].s64 = ctx.r[11].s64 + -4768;
	// 8266E8E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266E8E4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8266E8E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E8EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E8F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E8F8: 386A0DF8  addi r3, r10, 0xdf8
	ctx.r[3].s64 = ctx.r[10].s64 + 3576;
	// 8266E8FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E91C: 4BDF8505  bl 0x82466e20
	ctx.lr = 0x8266E920;
	sub_82466E20(ctx, base);
	// 8266E920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E930 size=108
    let mut pc: u32 = 0x8266E930;
    'dispatch: loop {
        match pc {
            0x8266E930 => {
    //   block [0x8266E930..0x8266E99C)
	// 8266E930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E93C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E944: 38EBED78  addi r7, r11, -0x1288
	ctx.r[7].s64 = ctx.r[11].s64 + -4744;
	// 8266E948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266E94C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8266E950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E954: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266E95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E960: 386A0E28  addi r3, r10, 0xe28
	ctx.r[3].s64 = ctx.r[10].s64 + 3624;
	// 8266E964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266E968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266E988: 4BDF8499  bl 0x82466e20
	ctx.lr = 0x8266E98C;
	sub_82466E20(ctx, base);
	// 8266E98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266E990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266E994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266E998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266E9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266E9A0 size=112
    let mut pc: u32 = 0x8266E9A0;
    'dispatch: loop {
        match pc {
            0x8266E9A0 => {
    //   block [0x8266E9A0..0x8266EA10)
	// 8266E9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266E9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266E9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266E9AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E9B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266E9B4: 38AA09D8  addi r5, r10, 0x9d8
	ctx.r[5].s64 = ctx.r[10].s64 + 2520;
	// 8266E9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266E9BC: 390BEDA8  addi r8, r11, -0x1258
	ctx.r[8].s64 = ctx.r[11].s64 + -4696;
	// 8266E9C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266E9C4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8266E9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266E9CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266E9D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266E9D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266E9D8: 386A0E58  addi r3, r10, 0xe58
	ctx.r[3].s64 = ctx.r[10].s64 + 3672;
	// 8266E9DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266E9E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266E9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266E9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266E9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266E9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266E9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266E9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266E9FC: 4BDF8425  bl 0x82466e20
	ctx.lr = 0x8266EA00;
	sub_82466E20(ctx, base);
	// 8266EA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EA10 size=112
    let mut pc: u32 = 0x8266EA10;
    'dispatch: loop {
        match pc {
            0x8266EA10 => {
    //   block [0x8266EA10..0x8266EA80)
	// 8266EA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EA1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EA20: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266EA24: 38AA0FD8  addi r5, r10, 0xfd8
	ctx.r[5].s64 = ctx.r[10].s64 + 4056;
	// 8266EA28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266EA2C: 390BEDD8  addi r8, r11, -0x1228
	ctx.r[8].s64 = ctx.r[11].s64 + -4648;
	// 8266EA30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266EA34: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8266EA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EA3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266EA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EA48: 386A0E88  addi r3, r10, 0xe88
	ctx.r[3].s64 = ctx.r[10].s64 + 3720;
	// 8266EA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266EA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EA6C: 4BDF83B5  bl 0x82466e20
	ctx.lr = 0x8266EA70;
	sub_82466E20(ctx, base);
	// 8266EA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EA80 size=100
    let mut pc: u32 = 0x8266EA80;
    'dispatch: loop {
        match pc {
            0x8266EA80 => {
    //   block [0x8266EA80..0x8266EAE4)
	// 8266EA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EA8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EA94: 38AA09D8  addi r5, r10, 0x9d8
	ctx.r[5].s64 = ctx.r[10].s64 + 2520;
	// 8266EA98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266EA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EAA0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8266EAA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EAA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EAAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EAB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EAB4: 386A0EB8  addi r3, r10, 0xeb8
	ctx.r[3].s64 = ctx.r[10].s64 + 3768;
	// 8266EAB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EAC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266EAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EAC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266EACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EAD0: 4BDF8351  bl 0x82466e20
	ctx.lr = 0x8266EAD4;
	sub_82466E20(ctx, base);
	// 8266EAD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EAE8 size=112
    let mut pc: u32 = 0x8266EAE8;
    'dispatch: loop {
        match pc {
            0x8266EAE8 => {
    //   block [0x8266EAE8..0x8266EB58)
	// 8266EAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EAF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EAF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266EAFC: 38AA0CD8  addi r5, r10, 0xcd8
	ctx.r[5].s64 = ctx.r[10].s64 + 3288;
	// 8266EB00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266EB04: 390BEE08  addi r8, r11, -0x11f8
	ctx.r[8].s64 = ctx.r[11].s64 + -4600;
	// 8266EB08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266EB0C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8266EB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EB14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EB18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266EB1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EB20: 386A0EE8  addi r3, r10, 0xee8
	ctx.r[3].s64 = ctx.r[10].s64 + 3816;
	// 8266EB24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266EB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EB2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EB34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EB44: 4BDF82DD  bl 0x82466e20
	ctx.lr = 0x8266EB48;
	sub_82466E20(ctx, base);
	// 8266EB48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EB4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EB50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EB54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EB58 size=112
    let mut pc: u32 = 0x8266EB58;
    'dispatch: loop {
        match pc {
            0x8266EB58 => {
    //   block [0x8266EB58..0x8266EBC8)
	// 8266EB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EB64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EB68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266EB6C: 38AA0CD8  addi r5, r10, 0xcd8
	ctx.r[5].s64 = ctx.r[10].s64 + 3288;
	// 8266EB70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266EB74: 390BEE50  addi r8, r11, -0x11b0
	ctx.r[8].s64 = ctx.r[11].s64 + -4528;
	// 8266EB78: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266EB7C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8266EB80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EB84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EB88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266EB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EB90: 386A0F18  addi r3, r10, 0xf18
	ctx.r[3].s64 = ctx.r[10].s64 + 3864;
	// 8266EB94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266EB98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EBA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EBA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EBAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EBB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EBB4: 4BDF826D  bl 0x82466e20
	ctx.lr = 0x8266EBB8;
	sub_82466E20(ctx, base);
	// 8266EBB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EBC8 size=108
    let mut pc: u32 = 0x8266EBC8;
    'dispatch: loop {
        match pc {
            0x8266EBC8 => {
    //   block [0x8266EBC8..0x8266EC34)
	// 8266EBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EBD4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266EBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266EBDC: 38EBEEF8  addi r7, r11, -0x1108
	ctx.r[7].s64 = ctx.r[11].s64 + -4360;
	// 8266EBE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266EBE4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8266EBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EBEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EBF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266EBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EBF8: 386A0F48  addi r3, r10, 0xf48
	ctx.r[3].s64 = ctx.r[10].s64 + 3912;
	// 8266EBFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266EC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EC0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EC1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266EC20: 4BDF8201  bl 0x82466e20
	ctx.lr = 0x8266EC24;
	sub_82466E20(ctx, base);
	// 8266EC24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EC38 size=112
    let mut pc: u32 = 0x8266EC38;
    'dispatch: loop {
        match pc {
            0x8266EC38 => {
    //   block [0x8266EC38..0x8266ECA8)
	// 8266EC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EC44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EC48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266EC4C: 38AA0AF8  addi r5, r10, 0xaf8
	ctx.r[5].s64 = ctx.r[10].s64 + 2808;
	// 8266EC50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266EC54: 390BEF40  addi r8, r11, -0x10c0
	ctx.r[8].s64 = ctx.r[11].s64 + -4288;
	// 8266EC58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266EC5C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8266EC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EC64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EC68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266EC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EC70: 386A0F78  addi r3, r10, 0xf78
	ctx.r[3].s64 = ctx.r[10].s64 + 3960;
	// 8266EC74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266EC78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EC94: 4BDF818D  bl 0x82466e20
	ctx.lr = 0x8266EC98;
	sub_82466E20(ctx, base);
	// 8266EC98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266ECA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266ECA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266ECA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266ECA8 size=100
    let mut pc: u32 = 0x8266ECA8;
    'dispatch: loop {
        match pc {
            0x8266ECA8 => {
    //   block [0x8266ECA8..0x8266ED0C)
	// 8266ECA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266ECAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266ECB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266ECB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ECB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266ECBC: 38AA0B28  addi r5, r10, 0xb28
	ctx.r[5].s64 = ctx.r[10].s64 + 2856;
	// 8266ECC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266ECC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266ECC8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8266ECCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ECD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266ECD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266ECD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266ECDC: 386A0FA8  addi r3, r10, 0xfa8
	ctx.r[3].s64 = ctx.r[10].s64 + 4008;
	// 8266ECE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266ECE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266ECE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266ECEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266ECF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266ECF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266ECF8: 4BDF8129  bl 0x82466e20
	ctx.lr = 0x8266ECFC;
	sub_82466E20(ctx, base);
	// 8266ECFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266ED00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266ED04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266ED08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266ED10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266ED10 size=100
    let mut pc: u32 = 0x8266ED10;
    'dispatch: loop {
        match pc {
            0x8266ED10 => {
    //   block [0x8266ED10..0x8266ED74)
	// 8266ED10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266ED14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266ED18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266ED1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ED20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266ED24: 38AA09D8  addi r5, r10, 0x9d8
	ctx.r[5].s64 = ctx.r[10].s64 + 2520;
	// 8266ED28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266ED2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266ED30: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8266ED34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ED38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266ED3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266ED40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266ED44: 386A0FD8  addi r3, r10, 0xfd8
	ctx.r[3].s64 = ctx.r[10].s64 + 4056;
	// 8266ED48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266ED4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266ED50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266ED54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266ED58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266ED5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266ED60: 4BDF80C1  bl 0x82466e20
	ctx.lr = 0x8266ED64;
	sub_82466E20(ctx, base);
	// 8266ED64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266ED68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266ED6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266ED70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266ED78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266ED78 size=108
    let mut pc: u32 = 0x8266ED78;
    'dispatch: loop {
        match pc {
            0x8266ED78 => {
    //   block [0x8266ED78..0x8266EDE4)
	// 8266ED78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266ED7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266ED80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266ED84: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266ED88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266ED8C: 38EBEFA0  addi r7, r11, -0x1060
	ctx.r[7].s64 = ctx.r[11].s64 + -4192;
	// 8266ED90: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8266ED94: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8266ED98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266ED9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EDA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266EDA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EDA8: 386A1008  addi r3, r10, 0x1008
	ctx.r[3].s64 = ctx.r[10].s64 + 4104;
	// 8266EDAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266EDB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EDB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EDC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EDCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266EDD0: 4BDF8051  bl 0x82466e20
	ctx.lr = 0x8266EDD4;
	sub_82466E20(ctx, base);
	// 8266EDD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EDD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EDDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EDE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EDE8 size=112
    let mut pc: u32 = 0x8266EDE8;
    'dispatch: loop {
        match pc {
            0x8266EDE8 => {
    //   block [0x8266EDE8..0x8266EE58)
	// 8266EDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EDF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EDF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266EDFC: 38AA0DC8  addi r5, r10, 0xdc8
	ctx.r[5].s64 = ctx.r[10].s64 + 3528;
	// 8266EE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266EE04: 390BF030  addi r8, r11, -0xfd0
	ctx.r[8].s64 = ctx.r[11].s64 + -4048;
	// 8266EE08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266EE0C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8266EE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EE14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EE18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266EE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EE20: 386A1038  addi r3, r10, 0x1038
	ctx.r[3].s64 = ctx.r[10].s64 + 4152;
	// 8266EE24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266EE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EE34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EE44: 4BDF7FDD  bl 0x82466e20
	ctx.lr = 0x8266EE48;
	sub_82466E20(ctx, base);
	// 8266EE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EE58 size=112
    let mut pc: u32 = 0x8266EE58;
    'dispatch: loop {
        match pc {
            0x8266EE58 => {
    //   block [0x8266EE58..0x8266EEC8)
	// 8266EE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EE64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EE68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266EE6C: 38AA0F18  addi r5, r10, 0xf18
	ctx.r[5].s64 = ctx.r[10].s64 + 3864;
	// 8266EE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266EE74: 390BF048  addi r8, r11, -0xfb8
	ctx.r[8].s64 = ctx.r[11].s64 + -4024;
	// 8266EE78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266EE7C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8266EE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EE84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EE88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266EE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EE90: 386A1068  addi r3, r10, 0x1068
	ctx.r[3].s64 = ctx.r[10].s64 + 4200;
	// 8266EE94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266EE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EEB4: 4BDF7F6D  bl 0x82466e20
	ctx.lr = 0x8266EEB8;
	sub_82466E20(ctx, base);
	// 8266EEB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EEC8 size=112
    let mut pc: u32 = 0x8266EEC8;
    'dispatch: loop {
        match pc {
            0x8266EEC8 => {
    //   block [0x8266EEC8..0x8266EF38)
	// 8266EEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EED4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EED8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266EEDC: 38AA09D8  addi r5, r10, 0x9d8
	ctx.r[5].s64 = ctx.r[10].s64 + 2520;
	// 8266EEE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266EEE4: 390BF078  addi r8, r11, -0xf88
	ctx.r[8].s64 = ctx.r[11].s64 + -3976;
	// 8266EEE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266EEEC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8266EEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EEF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EEF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266EEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EF00: 386A1098  addi r3, r10, 0x1098
	ctx.r[3].s64 = ctx.r[10].s64 + 4248;
	// 8266EF04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266EF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EF24: 4BDF7EFD  bl 0x82466e20
	ctx.lr = 0x8266EF28;
	sub_82466E20(ctx, base);
	// 8266EF28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EF38 size=112
    let mut pc: u32 = 0x8266EF38;
    'dispatch: loop {
        match pc {
            0x8266EF38 => {
    //   block [0x8266EF38..0x8266EFA8)
	// 8266EF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EF40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EF44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EF48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266EF4C: 38AA0B28  addi r5, r10, 0xb28
	ctx.r[5].s64 = ctx.r[10].s64 + 2856;
	// 8266EF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266EF54: 390BF0C0  addi r8, r11, -0xf40
	ctx.r[8].s64 = ctx.r[11].s64 + -3904;
	// 8266EF58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266EF5C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8266EF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EF64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EF68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266EF6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EF70: 386A10C8  addi r3, r10, 0x10c8
	ctx.r[3].s64 = ctx.r[10].s64 + 4296;
	// 8266EF74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266EF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EF7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EF84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EF94: 4BDF7E8D  bl 0x82466e20
	ctx.lr = 0x8266EF98;
	sub_82466E20(ctx, base);
	// 8266EF98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266EF9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266EFA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266EFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266EFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266EFA8 size=108
    let mut pc: u32 = 0x8266EFA8;
    'dispatch: loop {
        match pc {
            0x8266EFA8 => {
    //   block [0x8266EFA8..0x8266F014)
	// 8266EFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266EFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266EFB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266EFB4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266EFB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266EFBC: 38EBF108  addi r7, r11, -0xef8
	ctx.r[7].s64 = ctx.r[11].s64 + -3832;
	// 8266EFC0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266EFC4: 388A21D8  addi r4, r10, 0x21d8
	ctx.r[4].s64 = ctx.r[10].s64 + 8664;
	// 8266EFC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266EFCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266EFD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266EFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266EFD8: 386A10F8  addi r3, r10, 0x10f8
	ctx.r[3].s64 = ctx.r[10].s64 + 4344;
	// 8266EFDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266EFE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266EFE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266EFE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266EFEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266EFF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266EFF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266EFF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266EFFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266F000: 4BDF7E21  bl 0x82466e20
	ctx.lr = 0x8266F004;
	sub_82466E20(ctx, base);
	// 8266F004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F00C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F018 size=112
    let mut pc: u32 = 0x8266F018;
    'dispatch: loop {
        match pc {
            0x8266F018 => {
    //   block [0x8266F018..0x8266F088)
	// 8266F018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F024: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F028: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F02C: 38AA0A98  addi r5, r10, 0xa98
	ctx.r[5].s64 = ctx.r[10].s64 + 2712;
	// 8266F030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F034: 390BF168  addi r8, r11, -0xe98
	ctx.r[8].s64 = ctx.r[11].s64 + -3736;
	// 8266F038: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266F03C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8266F040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F044: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F048: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F04C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F050: 386A1128  addi r3, r10, 0x1128
	ctx.r[3].s64 = ctx.r[10].s64 + 4392;
	// 8266F054: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266F058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F05C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F074: 4BDF7DAD  bl 0x82466e20
	ctx.lr = 0x8266F078;
	sub_82466E20(ctx, base);
	// 8266F078: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F07C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F088 size=112
    let mut pc: u32 = 0x8266F088;
    'dispatch: loop {
        match pc {
            0x8266F088 => {
    //   block [0x8266F088..0x8266F0F8)
	// 8266F088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F094: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F098: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F09C: 38AA0AF8  addi r5, r10, 0xaf8
	ctx.r[5].s64 = ctx.r[10].s64 + 2808;
	// 8266F0A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F0A4: 390BF180  addi r8, r11, -0xe80
	ctx.r[8].s64 = ctx.r[11].s64 + -3712;
	// 8266F0A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266F0AC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8266F0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F0B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F0B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F0C0: 386A1158  addi r3, r10, 0x1158
	ctx.r[3].s64 = ctx.r[10].s64 + 4440;
	// 8266F0C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266F0C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F0D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F0E4: 4BDF7D3D  bl 0x82466e20
	ctx.lr = 0x8266F0E8;
	sub_82466E20(ctx, base);
	// 8266F0E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F0F8 size=112
    let mut pc: u32 = 0x8266F0F8;
    'dispatch: loop {
        match pc {
            0x8266F0F8 => {
    //   block [0x8266F0F8..0x8266F168)
	// 8266F0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F104: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F108: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F10C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266F110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F114: 390BF1B0  addi r8, r11, -0xe50
	ctx.r[8].s64 = ctx.r[11].s64 + -3664;
	// 8266F118: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266F11C: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8266F120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F12C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F130: 386A1188  addi r3, r10, 0x1188
	ctx.r[3].s64 = ctx.r[10].s64 + 4488;
	// 8266F134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266F138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F154: 4BDF7CCD  bl 0x82466e20
	ctx.lr = 0x8266F158;
	sub_82466E20(ctx, base);
	// 8266F158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F168 size=112
    let mut pc: u32 = 0x8266F168;
    'dispatch: loop {
        match pc {
            0x8266F168 => {
    //   block [0x8266F168..0x8266F1D8)
	// 8266F168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F174: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F178: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F17C: 38AA1188  addi r5, r10, 0x1188
	ctx.r[5].s64 = ctx.r[10].s64 + 4488;
	// 8266F180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F184: 390BF210  addi r8, r11, -0xdf0
	ctx.r[8].s64 = ctx.r[11].s64 + -3568;
	// 8266F188: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266F18C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8266F190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F1A0: 386A11B8  addi r3, r10, 0x11b8
	ctx.r[3].s64 = ctx.r[10].s64 + 4536;
	// 8266F1A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266F1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F1B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F1B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F1BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F1C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F1C4: 4BDF7C5D  bl 0x82466e20
	ctx.lr = 0x8266F1C8;
	sub_82466E20(ctx, base);
	// 8266F1C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F1D8 size=112
    let mut pc: u32 = 0x8266F1D8;
    'dispatch: loop {
        match pc {
            0x8266F1D8 => {
    //   block [0x8266F1D8..0x8266F248)
	// 8266F1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F1E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F1E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F1EC: 38AA1188  addi r5, r10, 0x1188
	ctx.r[5].s64 = ctx.r[10].s64 + 4488;
	// 8266F1F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F1F4: 390BF228  addi r8, r11, -0xdd8
	ctx.r[8].s64 = ctx.r[11].s64 + -3544;
	// 8266F1F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266F1FC: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8266F200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F204: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F208: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F210: 386A11E8  addi r3, r10, 0x11e8
	ctx.r[3].s64 = ctx.r[10].s64 + 4584;
	// 8266F214: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266F218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F22C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F234: 4BDF7BED  bl 0x82466e20
	ctx.lr = 0x8266F238;
	sub_82466E20(ctx, base);
	// 8266F238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F248 size=112
    let mut pc: u32 = 0x8266F248;
    'dispatch: loop {
        match pc {
            0x8266F248 => {
    //   block [0x8266F248..0x8266F2B8)
	// 8266F248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F254: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F258: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F25C: 38AA1188  addi r5, r10, 0x1188
	ctx.r[5].s64 = ctx.r[10].s64 + 4488;
	// 8266F260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F264: 390BF258  addi r8, r11, -0xda8
	ctx.r[8].s64 = ctx.r[11].s64 + -3496;
	// 8266F268: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266F26C: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8266F270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F274: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F280: 386A1218  addi r3, r10, 0x1218
	ctx.r[3].s64 = ctx.r[10].s64 + 4632;
	// 8266F284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266F288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F2A4: 4BDF7B7D  bl 0x82466e20
	ctx.lr = 0x8266F2A8;
	sub_82466E20(ctx, base);
	// 8266F2A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266F2B8 size=24
    let mut pc: u32 = 0x8266F2B8;
    'dispatch: loop {
        match pc {
            0x8266F2B8 => {
    //   block [0x8266F2B8..0x8266F2D0)
	// 8266F2B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F2BC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266F2C0: 394A2440  addi r10, r10, 0x2440
	ctx.r[10].s64 = ctx.r[10].s64 + 9280;
	// 8266F2C4: 816BECFC  lwz r11, -0x1304(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4868 as u32) ) } as u64;
	// 8266F2C8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8266F2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F2D0 size=112
    let mut pc: u32 = 0x8266F2D0;
    'dispatch: loop {
        match pc {
            0x8266F2D0 => {
    //   block [0x8266F2D0..0x8266F340)
	// 8266F2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F2DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266F2E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F2E4: 392A16F4  addi r9, r10, 0x16f4
	ctx.r[9].s64 = ctx.r[10].s64 + 5876;
	// 8266F2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F2EC: 390B2440  addi r8, r11, 0x2440
	ctx.r[8].s64 = ctx.r[11].s64 + 9280;
	// 8266F2F0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8266F2F4: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8266F2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F2FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F308: 386A1248  addi r3, r10, 0x1248
	ctx.r[3].s64 = ctx.r[10].s64 + 4680;
	// 8266F30C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266F310: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266F314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F31C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266F328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F32C: 4BDF7AF5  bl 0x82466e20
	ctx.lr = 0x8266F330;
	sub_82466E20(ctx, base);
	// 8266F330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F340 size=108
    let mut pc: u32 = 0x8266F340;
    'dispatch: loop {
        match pc {
            0x8266F340 => {
    //   block [0x8266F340..0x8266F3AC)
	// 8266F340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F34C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F354: 38EBF270  addi r7, r11, -0xd90
	ctx.r[7].s64 = ctx.r[11].s64 + -3472;
	// 8266F358: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266F35C: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8266F360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266F36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F370: 386A1278  addi r3, r10, 0x1278
	ctx.r[3].s64 = ctx.r[10].s64 + 4728;
	// 8266F374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266F378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266F398: 4BDF7A89  bl 0x82466e20
	ctx.lr = 0x8266F39C;
	sub_82466E20(ctx, base);
	// 8266F39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F3B0 size=108
    let mut pc: u32 = 0x8266F3B0;
    'dispatch: loop {
        match pc {
            0x8266F3B0 => {
    //   block [0x8266F3B0..0x8266F41C)
	// 8266F3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F3BC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F3C4: 38EBF288  addi r7, r11, -0xd78
	ctx.r[7].s64 = ctx.r[11].s64 + -3448;
	// 8266F3C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266F3CC: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8266F3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F3D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F3D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266F3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F3E0: 386A12A8  addi r3, r10, 0x12a8
	ctx.r[3].s64 = ctx.r[10].s64 + 4776;
	// 8266F3E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266F3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266F408: 4BDF7A19  bl 0x82466e20
	ctx.lr = 0x8266F40C;
	sub_82466E20(ctx, base);
	// 8266F40C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F420 size=116
    let mut pc: u32 = 0x8266F420;
    'dispatch: loop {
        match pc {
            0x8266F420 => {
    //   block [0x8266F420..0x8266F494)
	// 8266F420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F42C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F430: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266F434: 390BF2D4  addi r8, r11, -0xd2c
	ctx.r[8].s64 = ctx.r[11].s64 + -3372;
	// 8266F438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F43C: 392A17C0  addi r9, r10, 0x17c0
	ctx.r[9].s64 = ctx.r[10].s64 + 6080;
	// 8266F440: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F444: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8266F448: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266F44C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F454: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F464: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8266F468: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8266F46C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266F470: 386B12D8  addi r3, r11, 0x12d8
	ctx.r[3].s64 = ctx.r[11].s64 + 4824;
	// 8266F474: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266F478: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F47C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F480: 4BDF79A1  bl 0x82466e20
	ctx.lr = 0x8266F484;
	sub_82466E20(ctx, base);
	// 8266F484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F498 size=108
    let mut pc: u32 = 0x8266F498;
    'dispatch: loop {
        match pc {
            0x8266F498 => {
    //   block [0x8266F498..0x8266F504)
	// 8266F498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F4A4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F4A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266F4AC: 38EBF2F0  addi r7, r11, -0xd10
	ctx.r[7].s64 = ctx.r[11].s64 + -3344;
	// 8266F4B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266F4B4: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 8266F4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F4BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F4C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266F4C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F4C8: 386A1308  addi r3, r10, 0x1308
	ctx.r[3].s64 = ctx.r[10].s64 + 4872;
	// 8266F4CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266F4D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F4DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F4E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F4EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266F4F0: 4BDF7931  bl 0x82466e20
	ctx.lr = 0x8266F4F4;
	sub_82466E20(ctx, base);
	// 8266F4F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F4F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F4FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266F508 size=24
    let mut pc: u32 = 0x8266F508;
    'dispatch: loop {
        match pc {
            0x8266F508 => {
    //   block [0x8266F508..0x8266F520)
	// 8266F508: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F50C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266F510: 394A2488  addi r10, r10, 0x2488
	ctx.r[10].s64 = ctx.r[10].s64 + 9352;
	// 8266F514: 816BF2EC  lwz r11, -0xd14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3348 as u32) ) } as u64;
	// 8266F518: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8266F51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F520 size=116
    let mut pc: u32 = 0x8266F520;
    'dispatch: loop {
        match pc {
            0x8266F520 => {
    //   block [0x8266F520..0x8266F594)
	// 8266F520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F52C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F530: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266F534: 390B2488  addi r8, r11, 0x2488
	ctx.r[8].s64 = ctx.r[11].s64 + 9352;
	// 8266F538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F53C: 392A181C  addi r9, r10, 0x181c
	ctx.r[9].s64 = ctx.r[10].s64 + 6172;
	// 8266F540: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F544: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8266F548: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266F54C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F554: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F564: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8266F568: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8266F56C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266F570: 386B1338  addi r3, r11, 0x1338
	ctx.r[3].s64 = ctx.r[11].s64 + 4920;
	// 8266F574: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8266F578: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F580: 4BDF78A1  bl 0x82466e20
	ctx.lr = 0x8266F584;
	sub_82466E20(ctx, base);
	// 8266F584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F598 size=112
    let mut pc: u32 = 0x8266F598;
    'dispatch: loop {
        match pc {
            0x8266F598 => {
    //   block [0x8266F598..0x8266F608)
	// 8266F598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F5A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F5A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F5AC: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266F5B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266F5B4: 390BF358  addi r8, r11, -0xca8
	ctx.r[8].s64 = ctx.r[11].s64 + -3240;
	// 8266F5B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266F5BC: 388A2200  addi r4, r10, 0x2200
	ctx.r[4].s64 = ctx.r[10].s64 + 8704;
	// 8266F5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F5C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F5D0: 386A1368  addi r3, r10, 0x1368
	ctx.r[3].s64 = ctx.r[10].s64 + 4968;
	// 8266F5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266F5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F5F4: 4BDF782D  bl 0x82466e20
	ctx.lr = 0x8266F5F8;
	sub_82466E20(ctx, base);
	// 8266F5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F608 size=112
    let mut pc: u32 = 0x8266F608;
    'dispatch: loop {
        match pc {
            0x8266F608 => {
    //   block [0x8266F608..0x8266F678)
	// 8266F608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F614: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F618: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F61C: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266F620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F624: 390BF370  addi r8, r11, -0xc90
	ctx.r[8].s64 = ctx.r[11].s64 + -3216;
	// 8266F628: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266F62C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 8266F630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F634: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F640: 386A1398  addi r3, r10, 0x1398
	ctx.r[3].s64 = ctx.r[10].s64 + 5016;
	// 8266F644: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266F648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F664: 4BDF77BD  bl 0x82466e20
	ctx.lr = 0x8266F668;
	sub_82466E20(ctx, base);
	// 8266F668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F678 size=108
    let mut pc: u32 = 0x8266F678;
    'dispatch: loop {
        match pc {
            0x8266F678 => {
    //   block [0x8266F678..0x8266F6E4)
	// 8266F678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F684: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F688: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266F68C: 38EBF3A0  addi r7, r11, -0xc60
	ctx.r[7].s64 = ctx.r[11].s64 + -3168;
	// 8266F690: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266F694: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 8266F698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F69C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F6A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266F6A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F6A8: 386A13C8  addi r3, r10, 0x13c8
	ctx.r[3].s64 = ctx.r[10].s64 + 5064;
	// 8266F6AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266F6B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F6B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F6B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F6C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F6C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F6CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266F6D0: 4BDF7751  bl 0x82466e20
	ctx.lr = 0x8266F6D4;
	sub_82466E20(ctx, base);
	// 8266F6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F6E8 size=108
    let mut pc: u32 = 0x8266F6E8;
    'dispatch: loop {
        match pc {
            0x8266F6E8 => {
    //   block [0x8266F6E8..0x8266F754)
	// 8266F6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F6F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F6F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266F6FC: 38EBF3E8  addi r7, r11, -0xc18
	ctx.r[7].s64 = ctx.r[11].s64 + -3096;
	// 8266F700: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266F704: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 8266F708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F70C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266F714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F718: 386A13F8  addi r3, r10, 0x13f8
	ctx.r[3].s64 = ctx.r[10].s64 + 5112;
	// 8266F71C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266F720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F73C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266F740: 4BDF76E1  bl 0x82466e20
	ctx.lr = 0x8266F744;
	sub_82466E20(ctx, base);
	// 8266F744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F758 size=112
    let mut pc: u32 = 0x8266F758;
    'dispatch: loop {
        match pc {
            0x8266F758 => {
    //   block [0x8266F758..0x8266F7C8)
	// 8266F758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F768: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F76C: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266F770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F774: 390BF418  addi r8, r11, -0xbe8
	ctx.r[8].s64 = ctx.r[11].s64 + -3048;
	// 8266F778: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266F77C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 8266F780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F790: 386A1428  addi r3, r10, 0x1428
	ctx.r[3].s64 = ctx.r[10].s64 + 5160;
	// 8266F794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266F798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F7B4: 4BDF766D  bl 0x82466e20
	ctx.lr = 0x8266F7B8;
	sub_82466E20(ctx, base);
	// 8266F7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F7C8 size=108
    let mut pc: u32 = 0x8266F7C8;
    'dispatch: loop {
        match pc {
            0x8266F7C8 => {
    //   block [0x8266F7C8..0x8266F834)
	// 8266F7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F7D4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F7D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266F7DC: 38EBF448  addi r7, r11, -0xbb8
	ctx.r[7].s64 = ctx.r[11].s64 + -3000;
	// 8266F7E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266F7E4: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 8266F7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F7EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F7F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266F7F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F7F8: 386A1458  addi r3, r10, 0x1458
	ctx.r[3].s64 = ctx.r[10].s64 + 5208;
	// 8266F7FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266F800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F81C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266F820: 4BDF7601  bl 0x82466e20
	ctx.lr = 0x8266F824;
	sub_82466E20(ctx, base);
	// 8266F824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F82C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F838 size=108
    let mut pc: u32 = 0x8266F838;
    'dispatch: loop {
        match pc {
            0x8266F838 => {
    //   block [0x8266F838..0x8266F8A4)
	// 8266F838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F844: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F848: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266F84C: 38EBF4A8  addi r7, r11, -0xb58
	ctx.r[7].s64 = ctx.r[11].s64 + -2904;
	// 8266F850: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266F854: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 8266F858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F85C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266F864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F868: 386A1488  addi r3, r10, 0x1488
	ctx.r[3].s64 = ctx.r[10].s64 + 5256;
	// 8266F86C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266F870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F88C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266F890: 4BDF7591  bl 0x82466e20
	ctx.lr = 0x8266F894;
	sub_82466E20(ctx, base);
	// 8266F894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F89C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F8A8 size=116
    let mut pc: u32 = 0x8266F8A8;
    'dispatch: loop {
        match pc {
            0x8266F8A8 => {
    //   block [0x8266F8A8..0x8266F91C)
	// 8266F8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F8B4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266F8B8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8266F8BC: 390AF4F0  addi r8, r10, -0xb10
	ctx.r[8].s64 = ctx.r[10].s64 + -2832;
	// 8266F8C0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F8C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8266F8C8: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266F8CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F8D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266F8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F8D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F8DC: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 8266F8E0: 396B1858  addi r11, r11, 0x1858
	ctx.r[11].s64 = ctx.r[11].s64 + 6232;
	// 8266F8E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F8E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F8EC: 386A14B8  addi r3, r10, 0x14b8
	ctx.r[3].s64 = ctx.r[10].s64 + 5304;
	// 8266F8F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8266F8F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F8F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8266F8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F908: 4BDF7519  bl 0x82466e20
	ctx.lr = 0x8266F90C;
	sub_82466E20(ctx, base);
	// 8266F90C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F920 size=112
    let mut pc: u32 = 0x8266F920;
    'dispatch: loop {
        match pc {
            0x8266F920 => {
    //   block [0x8266F920..0x8266F990)
	// 8266F920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F92C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F930: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F934: 38AA1518  addi r5, r10, 0x1518
	ctx.r[5].s64 = ctx.r[10].s64 + 5400;
	// 8266F938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F93C: 390BF580  addi r8, r11, -0xa80
	ctx.r[8].s64 = ctx.r[11].s64 + -2688;
	// 8266F940: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8266F944: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 8266F948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F94C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266F954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F958: 386A14E8  addi r3, r10, 0x14e8
	ctx.r[3].s64 = ctx.r[10].s64 + 5352;
	// 8266F95C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266F960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F97C: 4BDF74A5  bl 0x82466e20
	ctx.lr = 0x8266F980;
	sub_82466E20(ctx, base);
	// 8266F980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266F990 size=100
    let mut pc: u32 = 0x8266F990;
    'dispatch: loop {
        match pc {
            0x8266F990 => {
    //   block [0x8266F990..0x8266F9F4)
	// 8266F990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266F994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266F998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266F99C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266F9A4: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266F9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266F9AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266F9B0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8266F9B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266F9B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266F9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266F9C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266F9C4: 386A1518  addi r3, r10, 0x1518
	ctx.r[3].s64 = ctx.r[10].s64 + 5400;
	// 8266F9C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266F9CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266F9D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266F9D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266F9D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266F9DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266F9E0: 4BDF7441  bl 0x82466e20
	ctx.lr = 0x8266F9E4;
	sub_82466E20(ctx, base);
	// 8266F9E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266F9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266F9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266F9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266F9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266F9F8 size=24
    let mut pc: u32 = 0x8266F9F8;
    'dispatch: loop {
        match pc {
            0x8266F9F8 => {
    //   block [0x8266F9F8..0x8266FA10)
	// 8266F9F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266F9FC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266FA00: 394A2548  addi r10, r10, 0x2548
	ctx.r[10].s64 = ctx.r[10].s64 + 9544;
	// 8266FA04: 816BF5F8  lwz r11, -0xa08(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2568 as u32) ) } as u64;
	// 8266FA08: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8266FA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FA10 size=116
    let mut pc: u32 = 0x8266FA10;
    'dispatch: loop {
        match pc {
            0x8266FA10 => {
    //   block [0x8266FA10..0x8266FA84)
	// 8266FA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FA1C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FA20: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266FA24: 390B2548  addi r8, r11, 0x2548
	ctx.r[8].s64 = ctx.r[11].s64 + 9544;
	// 8266FA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FA2C: 392A189C  addi r9, r10, 0x189c
	ctx.r[9].s64 = ctx.r[10].s64 + 6300;
	// 8266FA30: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FA34: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8266FA38: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266FA3C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266FA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FA44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266FA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FA54: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8266FA58: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8266FA5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266FA60: 386B1548  addi r3, r11, 0x1548
	ctx.r[3].s64 = ctx.r[11].s64 + 5448;
	// 8266FA64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266FA68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FA70: 4BDF73B1  bl 0x82466e20
	ctx.lr = 0x8266FA74;
	sub_82466E20(ctx, base);
	// 8266FA74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FA88 size=112
    let mut pc: u32 = 0x8266FA88;
    'dispatch: loop {
        match pc {
            0x8266FA88 => {
    //   block [0x8266FA88..0x8266FAF8)
	// 8266FA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FA94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FA98: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FA9C: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266FAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266FAA4: 390BF600  addi r8, r11, -0xa00
	ctx.r[8].s64 = ctx.r[11].s64 + -2560;
	// 8266FAA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266FAAC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 8266FAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FAB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FAB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266FABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FAC0: 386A1578  addi r3, r10, 0x1578
	ctx.r[3].s64 = ctx.r[10].s64 + 5496;
	// 8266FAC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266FAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FAE4: 4BDF733D  bl 0x82466e20
	ctx.lr = 0x8266FAE8;
	sub_82466E20(ctx, base);
	// 8266FAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FAF8 size=112
    let mut pc: u32 = 0x8266FAF8;
    'dispatch: loop {
        match pc {
            0x8266FAF8 => {
    //   block [0x8266FAF8..0x8266FB68)
	// 8266FAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FB04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FB08: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FB0C: 38AA14B8  addi r5, r10, 0x14b8
	ctx.r[5].s64 = ctx.r[10].s64 + 5304;
	// 8266FB10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266FB14: 390BF648  addi r8, r11, -0x9b8
	ctx.r[8].s64 = ctx.r[11].s64 + -2488;
	// 8266FB18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266FB1C: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 8266FB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FB24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FB28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266FB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FB30: 386A15A8  addi r3, r10, 0x15a8
	ctx.r[3].s64 = ctx.r[10].s64 + 5544;
	// 8266FB34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266FB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FB3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FB54: 4BDF72CD  bl 0x82466e20
	ctx.lr = 0x8266FB58;
	sub_82466E20(ctx, base);
	// 8266FB58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FB68 size=108
    let mut pc: u32 = 0x8266FB68;
    'dispatch: loop {
        match pc {
            0x8266FB68 => {
    //   block [0x8266FB68..0x8266FBD4)
	// 8266FB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FB74: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FB78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266FB7C: 38EBF6A8  addi r7, r11, -0x958
	ctx.r[7].s64 = ctx.r[11].s64 + -2392;
	// 8266FB80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266FB84: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 8266FB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FB8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FB90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266FB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FB98: 386A15D8  addi r3, r10, 0x15d8
	ctx.r[3].s64 = ctx.r[10].s64 + 5592;
	// 8266FB9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266FBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FBBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266FBC0: 4BDF7261  bl 0x82466e20
	ctx.lr = 0x8266FBC4;
	sub_82466E20(ctx, base);
	// 8266FBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FBD8 size=108
    let mut pc: u32 = 0x8266FBD8;
    'dispatch: loop {
        match pc {
            0x8266FBD8 => {
    //   block [0x8266FBD8..0x8266FC44)
	// 8266FBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FBE4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FBE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266FBEC: 38EBF6F0  addi r7, r11, -0x910
	ctx.r[7].s64 = ctx.r[11].s64 + -2320;
	// 8266FBF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266FBF4: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 8266FBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FBFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FC00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266FC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FC08: 386A1608  addi r3, r10, 0x1608
	ctx.r[3].s64 = ctx.r[10].s64 + 5640;
	// 8266FC0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266FC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FC2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266FC30: 4BDF71F1  bl 0x82466e20
	ctx.lr = 0x8266FC34;
	sub_82466E20(ctx, base);
	// 8266FC34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FC38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FC3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FC48 size=112
    let mut pc: u32 = 0x8266FC48;
    'dispatch: loop {
        match pc {
            0x8266FC48 => {
    //   block [0x8266FC48..0x8266FCB8)
	// 8266FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FC54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FC58: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FC5C: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266FC60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266FC64: 390BF738  addi r8, r11, -0x8c8
	ctx.r[8].s64 = ctx.r[11].s64 + -2248;
	// 8266FC68: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266FC6C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 8266FC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FC74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FC78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266FC7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FC80: 386A1638  addi r3, r10, 0x1638
	ctx.r[3].s64 = ctx.r[10].s64 + 5688;
	// 8266FC84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266FC88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FC90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FC94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FC98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FCA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FCA4: 4BDF717D  bl 0x82466e20
	ctx.lr = 0x8266FCA8;
	sub_82466E20(ctx, base);
	// 8266FCA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FCAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FCB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FCB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FCB8 size=112
    let mut pc: u32 = 0x8266FCB8;
    'dispatch: loop {
        match pc {
            0x8266FCB8 => {
    //   block [0x8266FCB8..0x8266FD28)
	// 8266FCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FCC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FCC8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FCCC: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266FCD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266FCD4: 390BF7E0  addi r8, r11, -0x820
	ctx.r[8].s64 = ctx.r[11].s64 + -2080;
	// 8266FCD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266FCDC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 8266FCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FCE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FCE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266FCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FCF0: 386A1668  addi r3, r10, 0x1668
	ctx.r[3].s64 = ctx.r[10].s64 + 5736;
	// 8266FCF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266FCF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FCFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FD14: 4BDF710D  bl 0x82466e20
	ctx.lr = 0x8266FD18;
	sub_82466E20(ctx, base);
	// 8266FD18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FD28 size=108
    let mut pc: u32 = 0x8266FD28;
    'dispatch: loop {
        match pc {
            0x8266FD28 => {
    //   block [0x8266FD28..0x8266FD94)
	// 8266FD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FD34: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FD38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266FD3C: 38EBF828  addi r7, r11, -0x7d8
	ctx.r[7].s64 = ctx.r[11].s64 + -2008;
	// 8266FD40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266FD44: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 8266FD48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FD4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FD50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266FD54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FD58: 386A1698  addi r3, r10, 0x1698
	ctx.r[3].s64 = ctx.r[10].s64 + 5784;
	// 8266FD5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266FD60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FD64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FD68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FD70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FD7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266FD80: 4BDF70A1  bl 0x82466e20
	ctx.lr = 0x8266FD84;
	sub_82466E20(ctx, base);
	// 8266FD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FD98 size=108
    let mut pc: u32 = 0x8266FD98;
    'dispatch: loop {
        match pc {
            0x8266FD98 => {
    //   block [0x8266FD98..0x8266FE04)
	// 8266FD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FDA4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FDA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266FDAC: 38EBF858  addi r7, r11, -0x7a8
	ctx.r[7].s64 = ctx.r[11].s64 + -1960;
	// 8266FDB0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8266FDB4: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 8266FDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FDBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FDC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266FDC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FDC8: 386A16C8  addi r3, r10, 0x16c8
	ctx.r[3].s64 = ctx.r[10].s64 + 5832;
	// 8266FDCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266FDD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FDD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FDE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FDE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FDE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FDEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266FDF0: 4BDF7031  bl 0x82466e20
	ctx.lr = 0x8266FDF4;
	sub_82466E20(ctx, base);
	// 8266FDF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FE08 size=112
    let mut pc: u32 = 0x8266FE08;
    'dispatch: loop {
        match pc {
            0x8266FE08 => {
    //   block [0x8266FE08..0x8266FE78)
	// 8266FE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FE14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FE18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FE1C: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266FE20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266FE24: 390BF8E8  addi r8, r11, -0x718
	ctx.r[8].s64 = ctx.r[11].s64 + -1816;
	// 8266FE28: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8266FE2C: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 8266FE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FE34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FE38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266FE3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FE40: 386A16F8  addi r3, r10, 0x16f8
	ctx.r[3].s64 = ctx.r[10].s64 + 5880;
	// 8266FE44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266FE48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FE64: 4BDF6FBD  bl 0x82466e20
	ctx.lr = 0x8266FE68;
	sub_82466E20(ctx, base);
	// 8266FE68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FE78 size=112
    let mut pc: u32 = 0x8266FE78;
    'dispatch: loop {
        match pc {
            0x8266FE78 => {
    //   block [0x8266FE78..0x8266FEE8)
	// 8266FE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FE84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FE88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FE8C: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266FE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266FE94: 390BF978  addi r8, r11, -0x688
	ctx.r[8].s64 = ctx.r[11].s64 + -1672;
	// 8266FE98: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8266FE9C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 8266FEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FEA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FEA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266FEAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FEB0: 386A1728  addi r3, r10, 0x1728
	ctx.r[3].s64 = ctx.r[10].s64 + 5928;
	// 8266FEB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266FEB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FED4: 4BDF6F4D  bl 0x82466e20
	ctx.lr = 0x8266FED8;
	sub_82466E20(ctx, base);
	// 8266FED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FEE8 size=100
    let mut pc: u32 = 0x8266FEE8;
    'dispatch: loop {
        match pc {
            0x8266FEE8 => {
    //   block [0x8266FEE8..0x8266FF4C)
	// 8266FEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FEF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FEFC: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 8266FF00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266FF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FF08: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8266FF0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FF1C: 386A1758  addi r3, r10, 0x1758
	ctx.r[3].s64 = ctx.r[10].s64 + 5976;
	// 8266FF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FF24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FF28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266FF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FF30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266FF34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FF38: 4BDF6EE9  bl 0x82466e20
	ctx.lr = 0x8266FF3C;
	sub_82466E20(ctx, base);
	// 8266FF3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FF50 size=112
    let mut pc: u32 = 0x8266FF50;
    'dispatch: loop {
        match pc {
            0x8266FF50 => {
    //   block [0x8266FF50..0x8266FFC0)
	// 8266FF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FF58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FF5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FF60: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FF64: 38AA1338  addi r5, r10, 0x1338
	ctx.r[5].s64 = ctx.r[10].s64 + 4920;
	// 8266FF68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266FF6C: 390BFA38  addi r8, r11, -0x5c8
	ctx.r[8].s64 = ctx.r[11].s64 + -1480;
	// 8266FF70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266FF74: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 8266FF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FF7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FF80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266FF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FF88: 386A1788  addi r3, r10, 0x1788
	ctx.r[3].s64 = ctx.r[10].s64 + 6024;
	// 8266FF8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266FF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266FF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266FF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266FF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266FFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266FFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266FFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266FFAC: 4BDF6E75  bl 0x82466e20
	ctx.lr = 0x8266FFB0;
	sub_82466E20(ctx, base);
	// 8266FFB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266FFB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266FFB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266FFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266FFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266FFC0 size=112
    let mut pc: u32 = 0x8266FFC0;
    'dispatch: loop {
        match pc {
            0x8266FFC0 => {
    //   block [0x8266FFC0..0x82670030)
	// 8266FFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266FFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266FFC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266FFCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FFD0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266FFD4: 38AA1188  addi r5, r10, 0x1188
	ctx.r[5].s64 = ctx.r[10].s64 + 4488;
	// 8266FFD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266FFDC: 390BFA68  addi r8, r11, -0x598
	ctx.r[8].s64 = ctx.r[11].s64 + -1432;
	// 8266FFE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266FFE4: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 8266FFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266FFEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266FFF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266FFF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266FFF8: 386A17B8  addi r3, r10, 0x17b8
	ctx.r[3].s64 = ctx.r[10].s64 + 6072;
	// 8266FFFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267000C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267001C: 4BDF6E05  bl 0x82466e20
	ctx.lr = 0x82670020;
	sub_82466E20(ctx, base);
	// 82670020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267002C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670030 size=108
    let mut pc: u32 = 0x82670030;
    'dispatch: loop {
        match pc {
            0x82670030 => {
    //   block [0x82670030..0x8267009C)
	// 82670030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267003C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670044: 38EBFA80  addi r7, r11, -0x580
	ctx.r[7].s64 = ctx.r[11].s64 + -1408;
	// 82670048: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267004C: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82670050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670054: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267005C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670060: 386A17E8  addi r3, r10, 0x17e8
	ctx.r[3].s64 = ctx.r[10].s64 + 6120;
	// 82670064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82670068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267006C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267007C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82670088: 4BDF6D99  bl 0x82466e20
	ctx.lr = 0x8267008C;
	sub_82466E20(ctx, base);
	// 8267008C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826700A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826700A0 size=112
    let mut pc: u32 = 0x826700A0;
    'dispatch: loop {
        match pc {
            0x826700A0 => {
    //   block [0x826700A0..0x82670110)
	// 826700A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826700A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826700A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826700AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826700B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826700B4: 38AA1758  addi r5, r10, 0x1758
	ctx.r[5].s64 = ctx.r[10].s64 + 5976;
	// 826700B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826700BC: 390BFAB0  addi r8, r11, -0x550
	ctx.r[8].s64 = ctx.r[11].s64 + -1360;
	// 826700C0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826700C4: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826700C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826700CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826700D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826700D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826700D8: 386A1818  addi r3, r10, 0x1818
	ctx.r[3].s64 = ctx.r[10].s64 + 6168;
	// 826700DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826700E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826700E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826700E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826700EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826700F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826700F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826700F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826700FC: 4BDF6D25  bl 0x82466e20
	ctx.lr = 0x82670100;
	sub_82466E20(ctx, base);
	// 82670100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267010C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670110 size=108
    let mut pc: u32 = 0x82670110;
    'dispatch: loop {
        match pc {
            0x82670110 => {
    //   block [0x82670110..0x8267017C)
	// 82670110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267011C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670120: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82670124: 38EBFB28  addi r7, r11, -0x4d8
	ctx.r[7].s64 = ctx.r[11].s64 + -1240;
	// 82670128: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267012C: 388A23DC  addi r4, r10, 0x23dc
	ctx.r[4].s64 = ctx.r[10].s64 + 9180;
	// 82670130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267013C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670140: 386A1848  addi r3, r10, 0x1848
	ctx.r[3].s64 = ctx.r[10].s64 + 6216;
	// 82670144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82670148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267014C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267015C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82670168: 4BDF6CB9  bl 0x82466e20
	ctx.lr = 0x8267016C;
	sub_82466E20(ctx, base);
	// 8267016C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670180 size=108
    let mut pc: u32 = 0x82670180;
    'dispatch: loop {
        match pc {
            0x82670180 => {
    //   block [0x82670180..0x826701EC)
	// 82670180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267018C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670190: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82670194: 38EBFB70  addi r7, r11, -0x490
	ctx.r[7].s64 = ctx.r[11].s64 + -1168;
	// 82670198: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267019C: 388A2400  addi r4, r10, 0x2400
	ctx.r[4].s64 = ctx.r[10].s64 + 9216;
	// 826701A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826701A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826701A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826701AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826701B0: 386A1878  addi r3, r10, 0x1878
	ctx.r[3].s64 = ctx.r[10].s64 + 6264;
	// 826701B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826701B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826701BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826701C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826701C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826701C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826701CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826701D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826701D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826701D8: 4BDF6C49  bl 0x82466e20
	ctx.lr = 0x826701DC;
	sub_82466E20(ctx, base);
	// 826701DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826701E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826701E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826701E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826701F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826701F0 size=112
    let mut pc: u32 = 0x826701F0;
    'dispatch: loop {
        match pc {
            0x826701F0 => {
    //   block [0x826701F0..0x82670260)
	// 826701F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826701F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826701F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826701FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670200: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670204: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 82670208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267020C: 390BFBA0  addi r8, r11, -0x460
	ctx.r[8].s64 = ctx.r[11].s64 + -1120;
	// 82670210: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82670214: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 82670218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267021C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670228: 386A18A8  addi r3, r10, 0x18a8
	ctx.r[3].s64 = ctx.r[10].s64 + 6312;
	// 8267022C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267023C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267024C: 4BDF6BD5  bl 0x82466e20
	ctx.lr = 0x82670250;
	sub_82466E20(ctx, base);
	// 82670250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267025C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670260 size=108
    let mut pc: u32 = 0x82670260;
    'dispatch: loop {
        match pc {
            0x82670260 => {
    //   block [0x82670260..0x826702CC)
	// 82670260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267026C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670274: 38EBFC30  addi r7, r11, -0x3d0
	ctx.r[7].s64 = ctx.r[11].s64 + -976;
	// 82670278: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8267027C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82670280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267028C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670290: 386A18D8  addi r3, r10, 0x18d8
	ctx.r[3].s64 = ctx.r[10].s64 + 6360;
	// 82670294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82670298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267029C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826702A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826702A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826702A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826702AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826702B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826702B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826702B8: 4BDF6B69  bl 0x82466e20
	ctx.lr = 0x826702BC;
	sub_82466E20(ctx, base);
	// 826702BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826702C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826702C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826702C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826702D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826702D0 size=112
    let mut pc: u32 = 0x826702D0;
    'dispatch: loop {
        match pc {
            0x826702D0 => {
    //   block [0x826702D0..0x82670340)
	// 826702D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826702D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826702D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826702DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826702E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826702E4: 38AA1758  addi r5, r10, 0x1758
	ctx.r[5].s64 = ctx.r[10].s64 + 5976;
	// 826702E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826702EC: 390BFCC0  addi r8, r11, -0x340
	ctx.r[8].s64 = ctx.r[11].s64 + -832;
	// 826702F0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826702F4: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826702F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826702FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670308: 386A1908  addi r3, r10, 0x1908
	ctx.r[3].s64 = ctx.r[10].s64 + 6408;
	// 8267030C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267031C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267032C: 4BDF6AF5  bl 0x82466e20
	ctx.lr = 0x82670330;
	sub_82466E20(ctx, base);
	// 82670330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267033C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670340 size=108
    let mut pc: u32 = 0x82670340;
    'dispatch: loop {
        match pc {
            0x82670340 => {
    //   block [0x82670340..0x826703AC)
	// 82670340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267034C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670354: 38EBFD80  addi r7, r11, -0x280
	ctx.r[7].s64 = ctx.r[11].s64 + -640;
	// 82670358: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267035C: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 82670360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267036C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670370: 386A1938  addi r3, r10, 0x1938
	ctx.r[3].s64 = ctx.r[10].s64 + 6456;
	// 82670374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82670378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267037C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267038C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82670398: 4BDF6A89  bl 0x82466e20
	ctx.lr = 0x8267039C;
	sub_82466E20(ctx, base);
	// 8267039C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826703A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826703A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826703A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826703B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826703B0 size=112
    let mut pc: u32 = 0x826703B0;
    'dispatch: loop {
        match pc {
            0x826703B0 => {
    //   block [0x826703B0..0x82670420)
	// 826703B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826703B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826703B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826703BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826703C0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826703C4: 38AA1758  addi r5, r10, 0x1758
	ctx.r[5].s64 = ctx.r[10].s64 + 5976;
	// 826703C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826703CC: 390BFDC8  addi r8, r11, -0x238
	ctx.r[8].s64 = ctx.r[11].s64 + -568;
	// 826703D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826703D4: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 826703D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826703DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826703E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826703E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826703E8: 386A1968  addi r3, r10, 0x1968
	ctx.r[3].s64 = ctx.r[10].s64 + 6504;
	// 826703EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826703F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826703F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826703F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826703FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267040C: 4BDF6A15  bl 0x82466e20
	ctx.lr = 0x82670410;
	sub_82466E20(ctx, base);
	// 82670410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267041C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670420 size=112
    let mut pc: u32 = 0x82670420;
    'dispatch: loop {
        match pc {
            0x82670420 => {
    //   block [0x82670420..0x82670490)
	// 82670420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267042C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670430: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670434: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 82670438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267043C: 390BFE28  addi r8, r11, -0x1d8
	ctx.r[8].s64 = ctx.r[11].s64 + -472;
	// 82670440: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82670444: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82670448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267044C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670458: 386A1998  addi r3, r10, 0x1998
	ctx.r[3].s64 = ctx.r[10].s64 + 6552;
	// 8267045C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267046C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267047C: 4BDF69A5  bl 0x82466e20
	ctx.lr = 0x82670480;
	sub_82466E20(ctx, base);
	// 82670480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267048C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670490 size=108
    let mut pc: u32 = 0x82670490;
    'dispatch: loop {
        match pc {
            0x82670490 => {
    //   block [0x82670490..0x826704FC)
	// 82670490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267049C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826704A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826704A4: 38EBFE40  addi r7, r11, -0x1c0
	ctx.r[7].s64 = ctx.r[11].s64 + -448;
	// 826704A8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826704AC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826704B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826704B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826704B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826704BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826704C0: 386A19C8  addi r3, r10, 0x19c8
	ctx.r[3].s64 = ctx.r[10].s64 + 6600;
	// 826704C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826704C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826704CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826704D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826704D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826704D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826704DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826704E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826704E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826704E8: 4BDF6939  bl 0x82466e20
	ctx.lr = 0x826704EC;
	sub_82466E20(ctx, base);
	// 826704EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826704F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826704F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826704F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670500 size=112
    let mut pc: u32 = 0x82670500;
    'dispatch: loop {
        match pc {
            0x82670500 => {
    //   block [0x82670500..0x82670570)
	// 82670500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267050C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670510: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670514: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 82670518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267051C: 390BFEB8  addi r8, r11, -0x148
	ctx.r[8].s64 = ctx.r[11].s64 + -328;
	// 82670520: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82670524: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82670528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267052C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670538: 386A19F8  addi r3, r10, 0x19f8
	ctx.r[3].s64 = ctx.r[10].s64 + 6648;
	// 8267053C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267054C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267055C: 4BDF68C5  bl 0x82466e20
	ctx.lr = 0x82670560;
	sub_82466E20(ctx, base);
	// 82670560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267056C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670570 size=104
    let mut pc: u32 = 0x82670570;
    'dispatch: loop {
        match pc {
            0x82670570 => {
    //   block [0x82670570..0x826705D8)
	// 82670570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267057C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82670580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670584: 392A18D8  addi r9, r10, 0x18d8
	ctx.r[9].s64 = ctx.r[10].s64 + 6360;
	// 82670588: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267058C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670590: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82670594: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267059C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826705A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826705A4: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 826705A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826705AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826705B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826705B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826705B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826705BC: 386A1A28  addi r3, r10, 0x1a28
	ctx.r[3].s64 = ctx.r[10].s64 + 6696;
	// 826705C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826705C4: 4BDF685D  bl 0x82466e20
	ctx.lr = 0x826705C8;
	sub_82466E20(ctx, base);
	// 826705C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826705CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826705D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826705D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826705D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826705D8 size=112
    let mut pc: u32 = 0x826705D8;
    'dispatch: loop {
        match pc {
            0x826705D8 => {
    //   block [0x826705D8..0x82670648)
	// 826705D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826705DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826705E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826705E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826705E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826705EC: 38AA1A28  addi r5, r10, 0x1a28
	ctx.r[5].s64 = ctx.r[10].s64 + 6696;
	// 826705F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826705F4: 390BFEE8  addi r8, r11, -0x118
	ctx.r[8].s64 = ctx.r[11].s64 + -280;
	// 826705F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826705FC: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82670600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670604: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267060C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670610: 386A1A58  addi r3, r10, 0x1a58
	ctx.r[3].s64 = ctx.r[10].s64 + 6744;
	// 82670614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267061C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267062C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670634: 4BDF67ED  bl 0x82466e20
	ctx.lr = 0x82670638;
	sub_82466E20(ctx, base);
	// 82670638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267063C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670648 size=112
    let mut pc: u32 = 0x82670648;
    'dispatch: loop {
        match pc {
            0x82670648 => {
    //   block [0x82670648..0x826706B8)
	// 82670648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267064C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670654: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670658: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267065C: 38AA1A58  addi r5, r10, 0x1a58
	ctx.r[5].s64 = ctx.r[10].s64 + 6744;
	// 82670660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670664: 390BFF18  addi r8, r11, -0xe8
	ctx.r[8].s64 = ctx.r[11].s64 + -232;
	// 82670668: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267066C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82670670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670674: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267067C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670680: 386A1A88  addi r3, r10, 0x1a88
	ctx.r[3].s64 = ctx.r[10].s64 + 6792;
	// 82670684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267068C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267069C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826706A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826706A4: 4BDF677D  bl 0x82466e20
	ctx.lr = 0x826706A8;
	sub_82466E20(ctx, base);
	// 826706A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826706AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826706B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826706B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826706B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826706B8 size=112
    let mut pc: u32 = 0x826706B8;
    'dispatch: loop {
        match pc {
            0x826706B8 => {
    //   block [0x826706B8..0x82670728)
	// 826706B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826706BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826706C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826706C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826706C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826706CC: 38AA1A58  addi r5, r10, 0x1a58
	ctx.r[5].s64 = ctx.r[10].s64 + 6744;
	// 826706D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826706D4: 390BFF78  addi r8, r11, -0x88
	ctx.r[8].s64 = ctx.r[11].s64 + -136;
	// 826706D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826706DC: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826706E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826706E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826706E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826706EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826706F0: 386A1AB8  addi r3, r10, 0x1ab8
	ctx.r[3].s64 = ctx.r[10].s64 + 6840;
	// 826706F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826706F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826706FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267070C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670714: 4BDF670D  bl 0x82466e20
	ctx.lr = 0x82670718;
	sub_82466E20(ctx, base);
	// 82670718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267071C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670728 size=112
    let mut pc: u32 = 0x82670728;
    'dispatch: loop {
        match pc {
            0x82670728 => {
    //   block [0x82670728..0x82670798)
	// 82670728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267072C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670738: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267073C: 38AA1A58  addi r5, r10, 0x1a58
	ctx.r[5].s64 = ctx.r[10].s64 + 6744;
	// 82670740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670744: 390BFFA8  addi r8, r11, -0x58
	ctx.r[8].s64 = ctx.r[11].s64 + -88;
	// 82670748: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267074C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82670750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670754: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267075C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670760: 386A1AE8  addi r3, r10, 0x1ae8
	ctx.r[3].s64 = ctx.r[10].s64 + 6888;
	// 82670764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267076C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267077C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670784: 4BDF669D  bl 0x82466e20
	ctx.lr = 0x82670788;
	sub_82466E20(ctx, base);
	// 82670788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267078C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670798 size=112
    let mut pc: u32 = 0x82670798;
    'dispatch: loop {
        match pc {
            0x82670798 => {
    //   block [0x82670798..0x82670808)
	// 82670798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267079C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826707A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826707A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826707A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826707AC: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 826707B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826707B4: 390BFFF0  addi r8, r11, -0x10
	ctx.r[8].s64 = ctx.r[11].s64 + -16;
	// 826707B8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826707BC: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826707C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826707C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826707C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826707CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826707D0: 386A1B18  addi r3, r10, 0x1b18
	ctx.r[3].s64 = ctx.r[10].s64 + 6936;
	// 826707D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826707D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826707DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826707E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826707E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826707E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826707EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826707F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826707F4: 4BDF662D  bl 0x82466e20
	ctx.lr = 0x826707F8;
	sub_82466E20(ctx, base);
	// 826707F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826707FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670808 size=116
    let mut pc: u32 = 0x82670808;
    'dispatch: loop {
        match pc {
            0x82670808 => {
    //   block [0x82670808..0x8267087C)
	// 82670808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267080C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670814: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82670818: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 8267081C: 390A0080  addi r8, r10, 0x80
	ctx.r[8].s64 = ctx.r[10].s64 + 128;
	// 82670820: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670824: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82670828: 38AA1FC8  addi r5, r10, 0x1fc8
	ctx.r[5].s64 = ctx.r[10].s64 + 8136;
	// 8267082C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670830: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82670834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267083C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82670840: 396B18F0  addi r11, r11, 0x18f0
	ctx.r[11].s64 = ctx.r[11].s64 + 6384;
	// 82670844: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670848: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267084C: 386A1B48  addi r3, r10, 0x1b48
	ctx.r[3].s64 = ctx.r[10].s64 + 6984;
	// 82670850: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82670854: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670858: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267085C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670868: 4BDF65B9  bl 0x82466e20
	ctx.lr = 0x8267086C;
	sub_82466E20(ctx, base);
	// 8267086C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670880 size=100
    let mut pc: u32 = 0x82670880;
    'dispatch: loop {
        match pc {
            0x82670880 => {
    //   block [0x82670880..0x826708E4)
	// 82670880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267088C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670894: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82670898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267089C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826708A0: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826708A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826708A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826708AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826708B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826708B4: 386A1B78  addi r3, r10, 0x1b78
	ctx.r[3].s64 = ctx.r[10].s64 + 7032;
	// 826708B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826708BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826708C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826708C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826708C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826708CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826708D0: 4BDF6551  bl 0x82466e20
	ctx.lr = 0x826708D4;
	sub_82466E20(ctx, base);
	// 826708D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826708D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826708DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826708E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826708E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826708E8 size=100
    let mut pc: u32 = 0x826708E8;
    'dispatch: loop {
        match pc {
            0x826708E8 => {
    //   block [0x826708E8..0x8267094C)
	// 826708E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826708EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826708F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826708F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826708F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826708FC: 38AA1C08  addi r5, r10, 0x1c08
	ctx.r[5].s64 = ctx.r[10].s64 + 7176;
	// 82670900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670908: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8267090C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267091C: 386A1BA8  addi r3, r10, 0x1ba8
	ctx.r[3].s64 = ctx.r[10].s64 + 7080;
	// 82670920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670924: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670928: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267092C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670930: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670938: 4BDF64E9  bl 0x82466e20
	ctx.lr = 0x8267093C;
	sub_82466E20(ctx, base);
	// 8267093C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670950 size=100
    let mut pc: u32 = 0x82670950;
    'dispatch: loop {
        match pc {
            0x82670950 => {
    //   block [0x82670950..0x826709B4)
	// 82670950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267095C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670964: 38AA1B48  addi r5, r10, 0x1b48
	ctx.r[5].s64 = ctx.r[10].s64 + 6984;
	// 82670968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267096C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670970: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82670974: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267097C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670984: 386A1BD8  addi r3, r10, 0x1bd8
	ctx.r[3].s64 = ctx.r[10].s64 + 7128;
	// 82670988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267098C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670990: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670998: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267099C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826709A0: 4BDF6481  bl 0x82466e20
	ctx.lr = 0x826709A4;
	sub_82466E20(ctx, base);
	// 826709A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826709A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826709AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826709B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826709B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826709B8 size=104
    let mut pc: u32 = 0x826709B8;
    'dispatch: loop {
        match pc {
            0x826709B8 => {
    //   block [0x826709B8..0x82670A20)
	// 826709B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826709BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826709C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826709C4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826709C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826709CC: 392A195C  addi r9, r10, 0x195c
	ctx.r[9].s64 = ctx.r[10].s64 + 6492;
	// 826709D0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826709D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826709D8: 38AA1B78  addi r5, r10, 0x1b78
	ctx.r[5].s64 = ctx.r[10].s64 + 7032;
	// 826709DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826709E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826709E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826709E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826709EC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826709F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826709F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826709F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826709FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670A00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670A04: 386A1C08  addi r3, r10, 0x1c08
	ctx.r[3].s64 = ctx.r[10].s64 + 7176;
	// 82670A08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82670A0C: 4BDF6415  bl 0x82466e20
	ctx.lr = 0x82670A10;
	sub_82466E20(ctx, base);
	// 82670A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670A20 size=108
    let mut pc: u32 = 0x82670A20;
    'dispatch: loop {
        match pc {
            0x82670A20 => {
    //   block [0x82670A20..0x82670A8C)
	// 82670A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670A2C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670A34: 38EB0234  addi r7, r11, 0x234
	ctx.r[7].s64 = ctx.r[11].s64 + 564;
	// 82670A38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82670A3C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82670A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670A44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670A48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82670A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670A50: 386A1C38  addi r3, r10, 0x1c38
	ctx.r[3].s64 = ctx.r[10].s64 + 7224;
	// 82670A54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82670A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82670A78: 4BDF63A9  bl 0x82466e20
	ctx.lr = 0x82670A7C;
	sub_82466E20(ctx, base);
	// 82670A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670A90 size=112
    let mut pc: u32 = 0x82670A90;
    'dispatch: loop {
        match pc {
            0x82670A90 => {
    //   block [0x82670A90..0x82670B00)
	// 82670A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670A9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670AA0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670AA4: 38AA1C08  addi r5, r10, 0x1c08
	ctx.r[5].s64 = ctx.r[10].s64 + 7176;
	// 82670AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670AAC: 390B0268  addi r8, r11, 0x268
	ctx.r[8].s64 = ctx.r[11].s64 + 616;
	// 82670AB0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82670AB4: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82670AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670ABC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670AC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670AC8: 386A1C68  addi r3, r10, 0x1c68
	ctx.r[3].s64 = ctx.r[10].s64 + 7272;
	// 82670ACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670AD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670AE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670AEC: 4BDF6335  bl 0x82466e20
	ctx.lr = 0x82670AF0;
	sub_82466E20(ctx, base);
	// 82670AF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670B00 size=116
    let mut pc: u32 = 0x82670B00;
    'dispatch: loop {
        match pc {
            0x82670B00 => {
    //   block [0x82670B00..0x82670B74)
	// 82670B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670B0C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670B10: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82670B14: 390B0310  addi r8, r11, 0x310
	ctx.r[8].s64 = ctx.r[11].s64 + 784;
	// 82670B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670B1C: 392A19C0  addi r9, r10, 0x19c0
	ctx.r[9].s64 = ctx.r[10].s64 + 6592;
	// 82670B20: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670B24: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82670B28: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82670B2C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670B34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670B44: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 82670B48: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82670B4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82670B50: 386B1C98  addi r3, r11, 0x1c98
	ctx.r[3].s64 = ctx.r[11].s64 + 7320;
	// 82670B54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82670B58: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670B60: 4BDF62C1  bl 0x82466e20
	ctx.lr = 0x82670B64;
	sub_82466E20(ctx, base);
	// 82670B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670B78 size=112
    let mut pc: u32 = 0x82670B78;
    'dispatch: loop {
        match pc {
            0x82670B78 => {
    //   block [0x82670B78..0x82670BE8)
	// 82670B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670B84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670B88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670B8C: 38AA1D58  addi r5, r10, 0x1d58
	ctx.r[5].s64 = ctx.r[10].s64 + 7512;
	// 82670B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670B94: 390B0328  addi r8, r11, 0x328
	ctx.r[8].s64 = ctx.r[11].s64 + 808;
	// 82670B98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82670B9C: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82670BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670BA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670BB0: 386A1CC8  addi r3, r10, 0x1cc8
	ctx.r[3].s64 = ctx.r[10].s64 + 7368;
	// 82670BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670BD4: 4BDF624D  bl 0x82466e20
	ctx.lr = 0x82670BD8;
	sub_82466E20(ctx, base);
	// 82670BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670BE8 size=100
    let mut pc: u32 = 0x82670BE8;
    'dispatch: loop {
        match pc {
            0x82670BE8 => {
    //   block [0x82670BE8..0x82670C4C)
	// 82670BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670BF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670BFC: 38AA1D28  addi r5, r10, 0x1d28
	ctx.r[5].s64 = ctx.r[10].s64 + 7464;
	// 82670C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670C08: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82670C0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670C1C: 386A1CF8  addi r3, r10, 0x1cf8
	ctx.r[3].s64 = ctx.r[10].s64 + 7416;
	// 82670C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670C24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670C28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670C30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670C38: 4BDF61E9  bl 0x82466e20
	ctx.lr = 0x82670C3C;
	sub_82466E20(ctx, base);
	// 82670C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670C50 size=112
    let mut pc: u32 = 0x82670C50;
    'dispatch: loop {
        match pc {
            0x82670C50 => {
    //   block [0x82670C50..0x82670CC0)
	// 82670C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670C5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670C60: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670C64: 38AA1D58  addi r5, r10, 0x1d58
	ctx.r[5].s64 = ctx.r[10].s64 + 7512;
	// 82670C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670C6C: 390B0340  addi r8, r11, 0x340
	ctx.r[8].s64 = ctx.r[11].s64 + 832;
	// 82670C70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82670C74: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82670C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670C7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670C88: 386A1D28  addi r3, r10, 0x1d28
	ctx.r[3].s64 = ctx.r[10].s64 + 7464;
	// 82670C8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670CAC: 4BDF6175  bl 0x82466e20
	ctx.lr = 0x82670CB0;
	sub_82466E20(ctx, base);
	// 82670CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670CC0 size=112
    let mut pc: u32 = 0x82670CC0;
    'dispatch: loop {
        match pc {
            0x82670CC0 => {
    //   block [0x82670CC0..0x82670D30)
	// 82670CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670CCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670CD0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670CD4: 38AA1C98  addi r5, r10, 0x1c98
	ctx.r[5].s64 = ctx.r[10].s64 + 7320;
	// 82670CD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82670CDC: 390B0370  addi r8, r11, 0x370
	ctx.r[8].s64 = ctx.r[11].s64 + 880;
	// 82670CE0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82670CE4: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 82670CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670CF8: 386A1D58  addi r3, r10, 0x1d58
	ctx.r[3].s64 = ctx.r[10].s64 + 7512;
	// 82670CFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670D1C: 4BDF6105  bl 0x82466e20
	ctx.lr = 0x82670D20;
	sub_82466E20(ctx, base);
	// 82670D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670D30 size=100
    let mut pc: u32 = 0x82670D30;
    'dispatch: loop {
        match pc {
            0x82670D30 => {
    //   block [0x82670D30..0x82670D94)
	// 82670D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670D3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670D44: 38AA1D58  addi r5, r10, 0x1d58
	ctx.r[5].s64 = ctx.r[10].s64 + 7512;
	// 82670D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670D50: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82670D54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670D64: 386A1D88  addi r3, r10, 0x1d88
	ctx.r[3].s64 = ctx.r[10].s64 + 7560;
	// 82670D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670D6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670D70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670D78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670D80: 4BDF60A1  bl 0x82466e20
	ctx.lr = 0x82670D84;
	sub_82466E20(ctx, base);
	// 82670D84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670D98 size=100
    let mut pc: u32 = 0x82670D98;
    'dispatch: loop {
        match pc {
            0x82670D98 => {
    //   block [0x82670D98..0x82670DFC)
	// 82670D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670DA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670DAC: 38AA1CC8  addi r5, r10, 0x1cc8
	ctx.r[5].s64 = ctx.r[10].s64 + 7368;
	// 82670DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670DB8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82670DBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670DCC: 386A1DB8  addi r3, r10, 0x1db8
	ctx.r[3].s64 = ctx.r[10].s64 + 7608;
	// 82670DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670DD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670DD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670DE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670DE8: 4BDF6039  bl 0x82466e20
	ctx.lr = 0x82670DEC;
	sub_82466E20(ctx, base);
	// 82670DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670E00 size=100
    let mut pc: u32 = 0x82670E00;
    'dispatch: loop {
        match pc {
            0x82670E00 => {
    //   block [0x82670E00..0x82670E64)
	// 82670E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670E0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670E14: 38AA1D88  addi r5, r10, 0x1d88
	ctx.r[5].s64 = ctx.r[10].s64 + 7560;
	// 82670E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670E20: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82670E24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670E34: 386A1DE8  addi r3, r10, 0x1de8
	ctx.r[3].s64 = ctx.r[10].s64 + 7656;
	// 82670E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670E3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670E40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670E48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670E50: 4BDF5FD1  bl 0x82466e20
	ctx.lr = 0x82670E54;
	sub_82466E20(ctx, base);
	// 82670E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670E68 size=100
    let mut pc: u32 = 0x82670E68;
    'dispatch: loop {
        match pc {
            0x82670E68 => {
    //   block [0x82670E68..0x82670ECC)
	// 82670E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670E74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670E7C: 38AA1CC8  addi r5, r10, 0x1cc8
	ctx.r[5].s64 = ctx.r[10].s64 + 7368;
	// 82670E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670E84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670E88: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 82670E8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670E9C: 386A1E18  addi r3, r10, 0x1e18
	ctx.r[3].s64 = ctx.r[10].s64 + 7704;
	// 82670EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670EA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670EA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82670EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670EB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82670EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670EB8: 4BDF5F69  bl 0x82466e20
	ctx.lr = 0x82670EBC;
	sub_82466E20(ctx, base);
	// 82670EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670ED0 size=112
    let mut pc: u32 = 0x82670ED0;
    'dispatch: loop {
        match pc {
            0x82670ED0 => {
    //   block [0x82670ED0..0x82670F40)
	// 82670ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670EDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670EE0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670EE4: 38AA1EA8  addi r5, r10, 0x1ea8
	ctx.r[5].s64 = ctx.r[10].s64 + 7848;
	// 82670EE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670EEC: 390B0418  addi r8, r11, 0x418
	ctx.r[8].s64 = ctx.r[11].s64 + 1048;
	// 82670EF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82670EF4: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82670EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670EFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670F00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670F08: 386A1E48  addi r3, r10, 0x1e48
	ctx.r[3].s64 = ctx.r[10].s64 + 7752;
	// 82670F0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670F2C: 4BDF5EF5  bl 0x82466e20
	ctx.lr = 0x82670F30;
	sub_82466E20(ctx, base);
	// 82670F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670F40 size=112
    let mut pc: u32 = 0x82670F40;
    'dispatch: loop {
        match pc {
            0x82670F40 => {
    //   block [0x82670F40..0x82670FB0)
	// 82670F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670F4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670F50: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670F54: 38AA1ED8  addi r5, r10, 0x1ed8
	ctx.r[5].s64 = ctx.r[10].s64 + 7896;
	// 82670F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670F5C: 390B0448  addi r8, r11, 0x448
	ctx.r[8].s64 = ctx.r[11].s64 + 1096;
	// 82670F60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82670F64: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82670F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670F6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670F70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670F78: 386A1E78  addi r3, r10, 0x1e78
	ctx.r[3].s64 = ctx.r[10].s64 + 7800;
	// 82670F7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82670F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82670F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82670F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82670F9C: 4BDF5E85  bl 0x82466e20
	ctx.lr = 0x82670FA0;
	sub_82466E20(ctx, base);
	// 82670FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82670FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82670FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82670FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82670FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82670FB0 size=112
    let mut pc: u32 = 0x82670FB0;
    'dispatch: loop {
        match pc {
            0x82670FB0 => {
    //   block [0x82670FB0..0x82671020)
	// 82670FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82670FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82670FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82670FBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670FC0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82670FC4: 38AA1FC8  addi r5, r10, 0x1fc8
	ctx.r[5].s64 = ctx.r[10].s64 + 8136;
	// 82670FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82670FCC: 390B0460  addi r8, r11, 0x460
	ctx.r[8].s64 = ctx.r[11].s64 + 1120;
	// 82670FD0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82670FD4: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82670FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82670FDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82670FE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82670FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82670FE8: 386A1EA8  addi r3, r10, 0x1ea8
	ctx.r[3].s64 = ctx.r[10].s64 + 7848;
	// 82670FEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82670FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82670FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82670FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82670FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267100C: 4BDF5E15  bl 0x82466e20
	ctx.lr = 0x82671010;
	sub_82466E20(ctx, base);
	// 82671010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267101C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671020 size=112
    let mut pc: u32 = 0x82671020;
    'dispatch: loop {
        match pc {
            0x82671020 => {
    //   block [0x82671020..0x82671090)
	// 82671020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267102C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671030: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671034: 38AA1EA8  addi r5, r10, 0x1ea8
	ctx.r[5].s64 = ctx.r[10].s64 + 7848;
	// 82671038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267103C: 390B0490  addi r8, r11, 0x490
	ctx.r[8].s64 = ctx.r[11].s64 + 1168;
	// 82671040: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671044: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82671048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267104C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671050: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671058: 386A1ED8  addi r3, r10, 0x1ed8
	ctx.r[3].s64 = ctx.r[10].s64 + 7896;
	// 8267105C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267106C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267107C: 4BDF5DA5  bl 0x82466e20
	ctx.lr = 0x82671080;
	sub_82466E20(ctx, base);
	// 82671080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267108C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671090 size=112
    let mut pc: u32 = 0x82671090;
    'dispatch: loop {
        match pc {
            0x82671090 => {
    //   block [0x82671090..0x82671100)
	// 82671090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267109C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826710A0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826710A4: 38AA1ED8  addi r5, r10, 0x1ed8
	ctx.r[5].s64 = ctx.r[10].s64 + 7896;
	// 826710A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826710AC: 390B04A8  addi r8, r11, 0x4a8
	ctx.r[8].s64 = ctx.r[11].s64 + 1192;
	// 826710B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826710B4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826710B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826710BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826710C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826710C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826710C8: 386A1F08  addi r3, r10, 0x1f08
	ctx.r[3].s64 = ctx.r[10].s64 + 7944;
	// 826710CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826710D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826710D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826710D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826710DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826710E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826710E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826710E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826710EC: 4BDF5D35  bl 0x82466e20
	ctx.lr = 0x826710F0;
	sub_82466E20(ctx, base);
	// 826710F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826710F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826710F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826710FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671100 size=112
    let mut pc: u32 = 0x82671100;
    'dispatch: loop {
        match pc {
            0x82671100 => {
    //   block [0x82671100..0x82671170)
	// 82671100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267110C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82671110: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671114: 392A19EC  addi r9, r10, 0x19ec
	ctx.r[9].s64 = ctx.r[10].s64 + 6636;
	// 82671118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267111C: 390B04C4  addi r8, r11, 0x4c4
	ctx.r[8].s64 = ctx.r[11].s64 + 1220;
	// 82671120: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82671124: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82671128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267112C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671138: 386A1F38  addi r3, r10, 0x1f38
	ctx.r[3].s64 = ctx.r[10].s64 + 7992;
	// 8267113C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82671140: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82671144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267114C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267115C: 4BDF5CC5  bl 0x82466e20
	ctx.lr = 0x82671160;
	sub_82466E20(ctx, base);
	// 82671160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267116C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671170 size=112
    let mut pc: u32 = 0x82671170;
    'dispatch: loop {
        match pc {
            0x82671170 => {
    //   block [0x82671170..0x826711E0)
	// 82671170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267117C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671180: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671184: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82671188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267118C: 390B04F8  addi r8, r11, 0x4f8
	ctx.r[8].s64 = ctx.r[11].s64 + 1272;
	// 82671190: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82671194: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82671198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267119C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826711A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826711A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826711A8: 386A1F68  addi r3, r10, 0x1f68
	ctx.r[3].s64 = ctx.r[10].s64 + 8040;
	// 826711AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826711B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826711B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826711B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826711BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826711C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826711C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826711C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826711CC: 4BDF5C55  bl 0x82466e20
	ctx.lr = 0x826711D0;
	sub_82466E20(ctx, base);
	// 826711D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826711D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826711D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826711DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826711E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826711E0 size=48
    let mut pc: u32 = 0x826711E0;
    'dispatch: loop {
        match pc {
            0x826711E0 => {
    //   block [0x826711E0..0x82671210)
	// 826711E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826711E4: 814B0590  lwz r10, 0x590(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1424 as u32) ) } as u64;
	// 826711E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826711EC: 396B25A8  addi r11, r11, 0x25a8
	ctx.r[11].s64 = ctx.r[11].s64 + 9640;
	// 826711F0: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826711F4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 826711F8: 814A058C  lwz r10, 0x58c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1420 as u32) ) } as u64;
	// 826711FC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82671200: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82671204: 814A0588  lwz r10, 0x588(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1416 as u32) ) } as u64;
	// 82671208: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 8267120C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671210 size=116
    let mut pc: u32 = 0x82671210;
    'dispatch: loop {
        match pc {
            0x82671210 => {
    //   block [0x82671210..0x82671284)
	// 82671210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267121C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82671220: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671224: 392B1AE0  addi r9, r11, 0x1ae0
	ctx.r[9].s64 = ctx.r[11].s64 + 6880;
	// 82671228: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8267122C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671230: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 82671234: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 82671238: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267123C: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82671240: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671244: 396B25A8  addi r11, r11, 0x25a8
	ctx.r[11].s64 = ctx.r[11].s64 + 9640;
	// 82671248: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267124C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671250: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82671254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671258: 386A1F98  addi r3, r10, 0x1f98
	ctx.r[3].s64 = ctx.r[10].s64 + 8088;
	// 8267125C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82671260: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82671264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671268: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267126C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82671270: 4BDF5BB1  bl 0x82466e20
	ctx.lr = 0x82671274;
	sub_82466E20(ctx, base);
	// 82671274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267127C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671288 size=116
    let mut pc: u32 = 0x82671288;
    'dispatch: loop {
        match pc {
            0x82671288 => {
    //   block [0x82671288..0x826712FC)
	// 82671288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267128C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671294: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671298: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267129C: 390B0598  addi r8, r11, 0x598
	ctx.r[8].s64 = ctx.r[11].s64 + 1432;
	// 826712A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826712A4: 392A1BC8  addi r9, r10, 0x1bc8
	ctx.r[9].s64 = ctx.r[10].s64 + 7112;
	// 826712A8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826712AC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826712B0: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 826712B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826712B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826712BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826712C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826712C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826712C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826712CC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 826712D0: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826712D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826712D8: 386B1FC8  addi r3, r11, 0x1fc8
	ctx.r[3].s64 = ctx.r[11].s64 + 8136;
	// 826712DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826712E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826712E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826712E8: 4BDF5B39  bl 0x82466e20
	ctx.lr = 0x826712EC;
	sub_82466E20(ctx, base);
	// 826712EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826712F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826712F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826712F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671300 size=112
    let mut pc: u32 = 0x82671300;
    'dispatch: loop {
        match pc {
            0x82671300 => {
    //   block [0x82671300..0x82671370)
	// 82671300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267130C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671310: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671314: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82671318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267131C: 390B0628  addi r8, r11, 0x628
	ctx.r[8].s64 = ctx.r[11].s64 + 1576;
	// 82671320: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671324: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82671328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267132C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671338: 386A1FF8  addi r3, r10, 0x1ff8
	ctx.r[3].s64 = ctx.r[10].s64 + 8184;
	// 8267133C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267134C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267135C: 4BDF5AC5  bl 0x82466e20
	ctx.lr = 0x82671360;
	sub_82466E20(ctx, base);
	// 82671360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267136C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671370 size=112
    let mut pc: u32 = 0x82671370;
    'dispatch: loop {
        match pc {
            0x82671370 => {
    //   block [0x82671370..0x826713E0)
	// 82671370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267137C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671380: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671384: 38AA07F8  addi r5, r10, 0x7f8
	ctx.r[5].s64 = ctx.r[10].s64 + 2040;
	// 82671388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267138C: 390B0640  addi r8, r11, 0x640
	ctx.r[8].s64 = ctx.r[11].s64 + 1600;
	// 82671390: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671394: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82671398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267139C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826713A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826713A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826713A8: 386A2028  addi r3, r10, 0x2028
	ctx.r[3].s64 = ctx.r[10].s64 + 8232;
	// 826713AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826713B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826713B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826713B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826713BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826713C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826713C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826713C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826713CC: 4BDF5A55  bl 0x82466e20
	ctx.lr = 0x826713D0;
	sub_82466E20(ctx, base);
	// 826713D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826713D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826713D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826713DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826713E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826713E0 size=108
    let mut pc: u32 = 0x826713E0;
    'dispatch: loop {
        match pc {
            0x826713E0 => {
    //   block [0x826713E0..0x8267144C)
	// 826713E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826713E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826713E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826713EC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826713F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826713F4: 38EB0658  addi r7, r11, 0x658
	ctx.r[7].s64 = ctx.r[11].s64 + 1624;
	// 826713F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826713FC: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82671400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671404: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671408: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267140C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671410: 386A2058  addi r3, r10, 0x2058
	ctx.r[3].s64 = ctx.r[10].s64 + 8280;
	// 82671414: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267141C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267142C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671438: 4BDF59E9  bl 0x82466e20
	ctx.lr = 0x8267143C;
	sub_82466E20(ctx, base);
	// 8267143C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671440: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671444: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671450 size=112
    let mut pc: u32 = 0x82671450;
    'dispatch: loop {
        match pc {
            0x82671450 => {
    //   block [0x82671450..0x826714C0)
	// 82671450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267145C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671460: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671464: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82671468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267146C: 390B0670  addi r8, r11, 0x670
	ctx.r[8].s64 = ctx.r[11].s64 + 1648;
	// 82671470: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82671474: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82671478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267147C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671488: 386A2088  addi r3, r10, 0x2088
	ctx.r[3].s64 = ctx.r[10].s64 + 8328;
	// 8267148C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267149C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826714A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826714A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826714A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826714AC: 4BDF5975  bl 0x82466e20
	ctx.lr = 0x826714B0;
	sub_82466E20(ctx, base);
	// 826714B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826714B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826714B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826714BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826714C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826714C0 size=108
    let mut pc: u32 = 0x826714C0;
    'dispatch: loop {
        match pc {
            0x826714C0 => {
    //   block [0x826714C0..0x8267152C)
	// 826714C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826714C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826714C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826714CC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826714D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826714D4: 38EB06B8  addi r7, r11, 0x6b8
	ctx.r[7].s64 = ctx.r[11].s64 + 1720;
	// 826714D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826714DC: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826714E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826714E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826714E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826714EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826714F0: 386A20B8  addi r3, r10, 0x20b8
	ctx.r[3].s64 = ctx.r[10].s64 + 8376;
	// 826714F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826714F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826714FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267150C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671514: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671518: 4BDF5909  bl 0x82466e20
	ctx.lr = 0x8267151C;
	sub_82466E20(ctx, base);
	// 8267151C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671530 size=112
    let mut pc: u32 = 0x82671530;
    'dispatch: loop {
        match pc {
            0x82671530 => {
    //   block [0x82671530..0x826715A0)
	// 82671530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267153C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671540: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671544: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82671548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267154C: 390B06D0  addi r8, r11, 0x6d0
	ctx.r[8].s64 = ctx.r[11].s64 + 1744;
	// 82671550: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82671554: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82671558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267155C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671568: 386A20E8  addi r3, r10, 0x20e8
	ctx.r[3].s64 = ctx.r[10].s64 + 8424;
	// 8267156C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267157C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267158C: 4BDF5895  bl 0x82466e20
	ctx.lr = 0x82671590;
	sub_82466E20(ctx, base);
	// 82671590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267159C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826715A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826715A0 size=112
    let mut pc: u32 = 0x826715A0;
    'dispatch: loop {
        match pc {
            0x826715A0 => {
    //   block [0x826715A0..0x82671610)
	// 826715A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826715A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826715A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826715AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826715B0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826715B4: 38AA21A8  addi r5, r10, 0x21a8
	ctx.r[5].s64 = ctx.r[10].s64 + 8616;
	// 826715B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826715BC: 390B0700  addi r8, r11, 0x700
	ctx.r[8].s64 = ctx.r[11].s64 + 1792;
	// 826715C0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826715C4: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 826715C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826715CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826715D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826715D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826715D8: 386A2118  addi r3, r10, 0x2118
	ctx.r[3].s64 = ctx.r[10].s64 + 8472;
	// 826715DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826715E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826715E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826715E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826715EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826715F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826715F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826715F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826715FC: 4BDF5825  bl 0x82466e20
	ctx.lr = 0x82671600;
	sub_82466E20(ctx, base);
	// 82671600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267160C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671610 size=108
    let mut pc: u32 = 0x82671610;
    'dispatch: loop {
        match pc {
            0x82671610 => {
    //   block [0x82671610..0x8267167C)
	// 82671610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267161C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82671624: 38EB0778  addi r7, r11, 0x778
	ctx.r[7].s64 = ctx.r[11].s64 + 1912;
	// 82671628: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267162C: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 82671630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671634: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671638: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267163C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671640: 386A2148  addi r3, r10, 0x2148
	ctx.r[3].s64 = ctx.r[10].s64 + 8520;
	// 82671644: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267164C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267165C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671668: 4BDF57B9  bl 0x82466e20
	ctx.lr = 0x8267166C;
	sub_82466E20(ctx, base);
	// 8267166C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671680 size=108
    let mut pc: u32 = 0x82671680;
    'dispatch: loop {
        match pc {
            0x82671680 => {
    //   block [0x82671680..0x826716EC)
	// 82671680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267168C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82671694: 38EB07C0  addi r7, r11, 0x7c0
	ctx.r[7].s64 = ctx.r[11].s64 + 1984;
	// 82671698: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267169C: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 826716A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826716A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826716A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826716AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826716B0: 386A2178  addi r3, r10, 0x2178
	ctx.r[3].s64 = ctx.r[10].s64 + 8568;
	// 826716B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826716B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826716BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826716C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826716C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826716C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826716CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826716D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826716D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826716D8: 4BDF5749  bl 0x82466e20
	ctx.lr = 0x826716DC;
	sub_82466E20(ctx, base);
	// 826716DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826716E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826716E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826716E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826716F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826716F0 size=116
    let mut pc: u32 = 0x826716F0;
    'dispatch: loop {
        match pc {
            0x826716F0 => {
    //   block [0x826716F0..0x82671764)
	// 826716F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826716F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826716F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826716FC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82671700: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82671704: 390A0808  addi r8, r10, 0x808
	ctx.r[8].s64 = ctx.r[10].s64 + 2056;
	// 82671708: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267170C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82671710: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 82671714: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671718: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267171C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671724: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82671728: 396B1BDC  addi r11, r11, 0x1bdc
	ctx.r[11].s64 = ctx.r[11].s64 + 7132;
	// 8267172C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671730: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671734: 386A21A8  addi r3, r10, 0x21a8
	ctx.r[3].s64 = ctx.r[10].s64 + 8616;
	// 82671738: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267173C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671740: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82671744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267174C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671750: 4BDF56D1  bl 0x82466e20
	ctx.lr = 0x82671754;
	sub_82466E20(ctx, base);
	// 82671754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267175C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671768 size=108
    let mut pc: u32 = 0x82671768;
    'dispatch: loop {
        match pc {
            0x82671768 => {
    //   block [0x82671768..0x826717D4)
	// 82671768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267176C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671774: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8267177C: 38EB08E0  addi r7, r11, 0x8e0
	ctx.r[7].s64 = ctx.r[11].s64 + 2272;
	// 82671780: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82671784: 388A24D4  addi r4, r10, 0x24d4
	ctx.r[4].s64 = ctx.r[10].s64 + 9428;
	// 82671788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267178C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671790: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671798: 386A21D8  addi r3, r10, 0x21d8
	ctx.r[3].s64 = ctx.r[10].s64 + 8664;
	// 8267179C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826717A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826717A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826717A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826717AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826717B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826717B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826717B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826717BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826717C0: 4BDF5661  bl 0x82466e20
	ctx.lr = 0x826717C4;
	sub_82466E20(ctx, base);
	// 826717C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826717C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826717CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826717D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826717D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826717D8 size=108
    let mut pc: u32 = 0x826717D8;
    'dispatch: loop {
        match pc {
            0x826717D8 => {
    //   block [0x826717D8..0x82671844)
	// 826717D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826717DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826717E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826717E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826717E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826717EC: 38EB0910  addi r7, r11, 0x910
	ctx.r[7].s64 = ctx.r[11].s64 + 2320;
	// 826717F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826717F4: 388A24F8  addi r4, r10, 0x24f8
	ctx.r[4].s64 = ctx.r[10].s64 + 9464;
	// 826717F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826717FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671808: 386A2208  addi r3, r10, 0x2208
	ctx.r[3].s64 = ctx.r[10].s64 + 8712;
	// 8267180C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267181C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267182C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671830: 4BDF55F1  bl 0x82466e20
	ctx.lr = 0x82671834;
	sub_82466E20(ctx, base);
	// 82671834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267183C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671848 size=112
    let mut pc: u32 = 0x82671848;
    'dispatch: loop {
        match pc {
            0x82671848 => {
    //   block [0x82671848..0x826718B8)
	// 82671848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267184C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671854: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671858: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267185C: 38AA12D8  addi r5, r10, 0x12d8
	ctx.r[5].s64 = ctx.r[10].s64 + 4824;
	// 82671860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671864: 390B0940  addi r8, r11, 0x940
	ctx.r[8].s64 = ctx.r[11].s64 + 2368;
	// 82671868: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8267186C: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 82671870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671874: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267187C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671880: 386A2238  addi r3, r10, 0x2238
	ctx.r[3].s64 = ctx.r[10].s64 + 8760;
	// 82671884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267188C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267189C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826718A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826718A4: 4BDF557D  bl 0x82466e20
	ctx.lr = 0x826718A8;
	sub_82466E20(ctx, base);
	// 826718A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826718AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826718B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826718B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826718B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826718B8 size=112
    let mut pc: u32 = 0x826718B8;
    'dispatch: loop {
        match pc {
            0x826718B8 => {
    //   block [0x826718B8..0x82671928)
	// 826718B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826718BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826718C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826718C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826718C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826718CC: 38AA11E8  addi r5, r10, 0x11e8
	ctx.r[5].s64 = ctx.r[10].s64 + 4584;
	// 826718D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826718D4: 390B0A18  addi r8, r11, 0xa18
	ctx.r[8].s64 = ctx.r[11].s64 + 2584;
	// 826718D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826718DC: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826718E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826718E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826718E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826718EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826718F0: 386A2268  addi r3, r10, 0x2268
	ctx.r[3].s64 = ctx.r[10].s64 + 8808;
	// 826718F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826718F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826718FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267190C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671914: 4BDF550D  bl 0x82466e20
	ctx.lr = 0x82671918;
	sub_82466E20(ctx, base);
	// 82671918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267191C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671928 size=112
    let mut pc: u32 = 0x82671928;
    'dispatch: loop {
        match pc {
            0x82671928 => {
    //   block [0x82671928..0x82671998)
	// 82671928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267192C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671934: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671938: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267193C: 38AA11E8  addi r5, r10, 0x11e8
	ctx.r[5].s64 = ctx.r[10].s64 + 4584;
	// 82671940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671944: 390B0A60  addi r8, r11, 0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + 2656;
	// 82671948: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267194C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82671950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671954: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267195C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671960: 386A2298  addi r3, r10, 0x2298
	ctx.r[3].s64 = ctx.r[10].s64 + 8856;
	// 82671964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267196C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267197C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671984: 4BDF549D  bl 0x82466e20
	ctx.lr = 0x82671988;
	sub_82466E20(ctx, base);
	// 82671988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267198C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671998 size=112
    let mut pc: u32 = 0x82671998;
    'dispatch: loop {
        match pc {
            0x82671998 => {
    //   block [0x82671998..0x82671A08)
	// 82671998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267199C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826719A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826719A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826719A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826719AC: 38AA1218  addi r5, r10, 0x1218
	ctx.r[5].s64 = ctx.r[10].s64 + 4632;
	// 826719B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826719B4: 390B0AC0  addi r8, r11, 0xac0
	ctx.r[8].s64 = ctx.r[11].s64 + 2752;
	// 826719B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826719BC: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826719C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826719C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826719C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826719CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826719D0: 386A22C8  addi r3, r10, 0x22c8
	ctx.r[3].s64 = ctx.r[10].s64 + 8904;
	// 826719D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826719D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826719DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826719E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826719E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826719E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826719EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826719F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826719F4: 4BDF542D  bl 0x82466e20
	ctx.lr = 0x826719F8;
	sub_82466E20(ctx, base);
	// 826719F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826719FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671A08 size=112
    let mut pc: u32 = 0x82671A08;
    'dispatch: loop {
        match pc {
            0x82671A08 => {
    //   block [0x82671A08..0x82671A78)
	// 82671A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671A14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671A18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671A1C: 38AA1218  addi r5, r10, 0x1218
	ctx.r[5].s64 = ctx.r[10].s64 + 4632;
	// 82671A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671A24: 390B0B20  addi r8, r11, 0xb20
	ctx.r[8].s64 = ctx.r[11].s64 + 2848;
	// 82671A28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82671A2C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82671A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671A34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671A38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671A40: 386A22F8  addi r3, r10, 0x22f8
	ctx.r[3].s64 = ctx.r[10].s64 + 8952;
	// 82671A44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671A64: 4BDF53BD  bl 0x82466e20
	ctx.lr = 0x82671A68;
	sub_82466E20(ctx, base);
	// 82671A68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671A78 size=112
    let mut pc: u32 = 0x82671A78;
    'dispatch: loop {
        match pc {
            0x82671A78 => {
    //   block [0x82671A78..0x82671AE8)
	// 82671A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671A84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671A88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671A8C: 38AA11E8  addi r5, r10, 0x11e8
	ctx.r[5].s64 = ctx.r[10].s64 + 4584;
	// 82671A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671A94: 390B0B80  addi r8, r11, 0xb80
	ctx.r[8].s64 = ctx.r[11].s64 + 2944;
	// 82671A98: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82671A9C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82671AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671AA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671AB0: 386A2328  addi r3, r10, 0x2328
	ctx.r[3].s64 = ctx.r[10].s64 + 9000;
	// 82671AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671AD4: 4BDF534D  bl 0x82466e20
	ctx.lr = 0x82671AD8;
	sub_82466E20(ctx, base);
	// 82671AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671AE8 size=108
    let mut pc: u32 = 0x82671AE8;
    'dispatch: loop {
        match pc {
            0x82671AE8 => {
    //   block [0x82671AE8..0x82671B54)
	// 82671AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671AF4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671AFC: 38EB0C40  addi r7, r11, 0xc40
	ctx.r[7].s64 = ctx.r[11].s64 + 3136;
	// 82671B00: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82671B04: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82671B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671B0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671B18: 386A2358  addi r3, r10, 0x2358
	ctx.r[3].s64 = ctx.r[10].s64 + 9048;
	// 82671B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671B40: 4BDF52E1  bl 0x82466e20
	ctx.lr = 0x82671B44;
	sub_82466E20(ctx, base);
	// 82671B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671B58 size=112
    let mut pc: u32 = 0x82671B58;
    'dispatch: loop {
        match pc {
            0x82671B58 => {
    //   block [0x82671B58..0x82671BC8)
	// 82671B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671B64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671B68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671B6C: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 82671B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671B74: 390B0DD8  addi r8, r11, 0xdd8
	ctx.r[8].s64 = ctx.r[11].s64 + 3544;
	// 82671B78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671B7C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82671B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671B84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671B90: 386A2388  addi r3, r10, 0x2388
	ctx.r[3].s64 = ctx.r[10].s64 + 9096;
	// 82671B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671BB4: 4BDF526D  bl 0x82466e20
	ctx.lr = 0x82671BB8;
	sub_82466E20(ctx, base);
	// 82671BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671BC8 size=112
    let mut pc: u32 = 0x82671BC8;
    'dispatch: loop {
        match pc {
            0x82671BC8 => {
    //   block [0x82671BC8..0x82671C38)
	// 82671BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671BD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671BD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671BDC: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 82671BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671BE4: 390B0DF0  addi r8, r11, 0xdf0
	ctx.r[8].s64 = ctx.r[11].s64 + 3568;
	// 82671BE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671BEC: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82671BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671BF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671C00: 386A23B8  addi r3, r10, 0x23b8
	ctx.r[3].s64 = ctx.r[10].s64 + 9144;
	// 82671C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671C14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82671C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671C24: 4BDF51FD  bl 0x82466e20
	ctx.lr = 0x82671C28;
	sub_82466E20(ctx, base);
	// 82671C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671C38 size=112
    let mut pc: u32 = 0x82671C38;
    'dispatch: loop {
        match pc {
            0x82671C38 => {
    //   block [0x82671C38..0x82671CA8)
	// 82671C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671C44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671C48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671C4C: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 82671C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671C54: 390B0E08  addi r8, r11, 0xe08
	ctx.r[8].s64 = ctx.r[11].s64 + 3592;
	// 82671C58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82671C5C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82671C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671C64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671C70: 386A23E8  addi r3, r10, 0x23e8
	ctx.r[3].s64 = ctx.r[10].s64 + 9192;
	// 82671C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671C94: 4BDF518D  bl 0x82466e20
	ctx.lr = 0x82671C98;
	sub_82466E20(ctx, base);
	// 82671C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671CA8 size=108
    let mut pc: u32 = 0x82671CA8;
    'dispatch: loop {
        match pc {
            0x82671CA8 => {
    //   block [0x82671CA8..0x82671D14)
	// 82671CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671CB4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671CBC: 38EB0E38  addi r7, r11, 0xe38
	ctx.r[7].s64 = ctx.r[11].s64 + 3640;
	// 82671CC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82671CC4: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82671CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671CCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671CD8: 386A2418  addi r3, r10, 0x2418
	ctx.r[3].s64 = ctx.r[10].s64 + 9240;
	// 82671CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671D00: 4BDF5121  bl 0x82466e20
	ctx.lr = 0x82671D04;
	sub_82466E20(ctx, base);
	// 82671D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671D18 size=112
    let mut pc: u32 = 0x82671D18;
    'dispatch: loop {
        match pc {
            0x82671D18 => {
    //   block [0x82671D18..0x82671D88)
	// 82671D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671D24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671D28: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671D2C: 38AA08B8  addi r5, r10, 0x8b8
	ctx.r[5].s64 = ctx.r[10].s64 + 2232;
	// 82671D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671D34: 390B0E68  addi r8, r11, 0xe68
	ctx.r[8].s64 = ctx.r[11].s64 + 3688;
	// 82671D38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82671D3C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82671D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671D44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671D50: 386A2448  addi r3, r10, 0x2448
	ctx.r[3].s64 = ctx.r[10].s64 + 9288;
	// 82671D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671D64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82671D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671D74: 4BDF50AD  bl 0x82466e20
	ctx.lr = 0x82671D78;
	sub_82466E20(ctx, base);
	// 82671D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671D88 size=108
    let mut pc: u32 = 0x82671D88;
    'dispatch: loop {
        match pc {
            0x82671D88 => {
    //   block [0x82671D88..0x82671DF4)
	// 82671D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671D94: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671D9C: 38EB0E80  addi r7, r11, 0xe80
	ctx.r[7].s64 = ctx.r[11].s64 + 3712;
	// 82671DA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82671DA4: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 82671DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671DAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671DB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671DB8: 386A2478  addi r3, r10, 0x2478
	ctx.r[3].s64 = ctx.r[10].s64 + 9336;
	// 82671DBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671DE0: 4BDF5041  bl 0x82466e20
	ctx.lr = 0x82671DE4;
	sub_82466E20(ctx, base);
	// 82671DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671DF8 size=108
    let mut pc: u32 = 0x82671DF8;
    'dispatch: loop {
        match pc {
            0x82671DF8 => {
    //   block [0x82671DF8..0x82671E64)
	// 82671DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671E04: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671E0C: 38EB0EB0  addi r7, r11, 0xeb0
	ctx.r[7].s64 = ctx.r[11].s64 + 3760;
	// 82671E10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82671E14: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82671E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671E1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671E28: 386A24A8  addi r3, r10, 0x24a8
	ctx.r[3].s64 = ctx.r[10].s64 + 9384;
	// 82671E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671E50: 4BDF4FD1  bl 0x82466e20
	ctx.lr = 0x82671E54;
	sub_82466E20(ctx, base);
	// 82671E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671E68 size=112
    let mut pc: u32 = 0x82671E68;
    'dispatch: loop {
        match pc {
            0x82671E68 => {
    //   block [0x82671E68..0x82671ED8)
	// 82671E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671E74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671E78: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671E7C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82671E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671E84: 390B0EF8  addi r8, r11, 0xef8
	ctx.r[8].s64 = ctx.r[11].s64 + 3832;
	// 82671E88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82671E8C: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 82671E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671E94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671EA0: 386A24D8  addi r3, r10, 0x24d8
	ctx.r[3].s64 = ctx.r[10].s64 + 9432;
	// 82671EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671EC4: 4BDF4F5D  bl 0x82466e20
	ctx.lr = 0x82671EC8;
	sub_82466E20(ctx, base);
	// 82671EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671ED8 size=112
    let mut pc: u32 = 0x82671ED8;
    'dispatch: loop {
        match pc {
            0x82671ED8 => {
    //   block [0x82671ED8..0x82671F48)
	// 82671ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671EE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671EE8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671EEC: 38AA1218  addi r5, r10, 0x1218
	ctx.r[5].s64 = ctx.r[10].s64 + 4632;
	// 82671EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671EF4: 390B0F40  addi r8, r11, 0xf40
	ctx.r[8].s64 = ctx.r[11].s64 + 3904;
	// 82671EF8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82671EFC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82671F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671F04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82671F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671F10: 386A2508  addi r3, r10, 0x2508
	ctx.r[3].s64 = ctx.r[10].s64 + 9480;
	// 82671F14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82671F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671F34: 4BDF4EED  bl 0x82466e20
	ctx.lr = 0x82671F38;
	sub_82466E20(ctx, base);
	// 82671F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671F48 size=108
    let mut pc: u32 = 0x82671F48;
    'dispatch: loop {
        match pc {
            0x82671F48 => {
    //   block [0x82671F48..0x82671FB4)
	// 82671F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671F54: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671F5C: 38EB0FD0  addi r7, r11, 0xfd0
	ctx.r[7].s64 = ctx.r[11].s64 + 4048;
	// 82671F60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82671F64: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 82671F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671F6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671F70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671F78: 386A2538  addi r3, r10, 0x2538
	ctx.r[3].s64 = ctx.r[10].s64 + 9528;
	// 82671F7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82671F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82671F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82671F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82671F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82671FA0: 4BDF4E81  bl 0x82466e20
	ctx.lr = 0x82671FA4;
	sub_82466E20(ctx, base);
	// 82671FA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82671FA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82671FAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82671FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82671FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82671FB8 size=108
    let mut pc: u32 = 0x82671FB8;
    'dispatch: loop {
        match pc {
            0x82671FB8 => {
    //   block [0x82671FB8..0x82672024)
	// 82671FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82671FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82671FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82671FC4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82671FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82671FCC: 38EB1018  addi r7, r11, 0x1018
	ctx.r[7].s64 = ctx.r[11].s64 + 4120;
	// 82671FD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82671FD4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82671FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82671FDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82671FE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82671FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82671FE8: 386A2568  addi r3, r10, 0x2568
	ctx.r[3].s64 = ctx.r[10].s64 + 9576;
	// 82671FEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82671FF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82671FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82671FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82671FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267200C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672010: 4BDF4E11  bl 0x82466e20
	ctx.lr = 0x82672014;
	sub_82466E20(ctx, base);
	// 82672014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267201C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672028 size=108
    let mut pc: u32 = 0x82672028;
    'dispatch: loop {
        match pc {
            0x82672028 => {
    //   block [0x82672028..0x82672094)
	// 82672028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267202C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672034: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267203C: 38EB1048  addi r7, r11, 0x1048
	ctx.r[7].s64 = ctx.r[11].s64 + 4168;
	// 82672040: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82672044: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 82672048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267204C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672058: 386A2598  addi r3, r10, 0x2598
	ctx.r[3].s64 = ctx.r[10].s64 + 9624;
	// 8267205C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267206C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267207C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672080: 4BDF4DA1  bl 0x82466e20
	ctx.lr = 0x82672084;
	sub_82466E20(ctx, base);
	// 82672084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267208C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672098 size=112
    let mut pc: u32 = 0x82672098;
    'dispatch: loop {
        match pc {
            0x82672098 => {
    //   block [0x82672098..0x82672108)
	// 82672098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267209C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826720A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826720A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826720A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826720AC: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 826720B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826720B4: 390B1078  addi r8, r11, 0x1078
	ctx.r[8].s64 = ctx.r[11].s64 + 4216;
	// 826720B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826720BC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826720C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826720C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826720C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826720CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826720D0: 386A25C8  addi r3, r10, 0x25c8
	ctx.r[3].s64 = ctx.r[10].s64 + 9672;
	// 826720D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826720D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826720DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826720E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826720E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826720E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826720EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826720F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826720F4: 4BDF4D2D  bl 0x82466e20
	ctx.lr = 0x826720F8;
	sub_82466E20(ctx, base);
	// 826720F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826720FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672108 size=112
    let mut pc: u32 = 0x82672108;
    'dispatch: loop {
        match pc {
            0x82672108 => {
    //   block [0x82672108..0x82672178)
	// 82672108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267210C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672114: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672118: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267211C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672124: 390B10A8  addi r8, r11, 0x10a8
	ctx.r[8].s64 = ctx.r[11].s64 + 4264;
	// 82672128: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267212C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82672130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267213C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672140: 386A25F8  addi r3, r10, 0x25f8
	ctx.r[3].s64 = ctx.r[10].s64 + 9720;
	// 82672144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267214C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267215C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672164: 4BDF4CBD  bl 0x82466e20
	ctx.lr = 0x82672168;
	sub_82466E20(ctx, base);
	// 82672168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267216C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672178 size=112
    let mut pc: u32 = 0x82672178;
    'dispatch: loop {
        match pc {
            0x82672178 => {
    //   block [0x82672178..0x826721E8)
	// 82672178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267217C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672188: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267218C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672194: 390B10C0  addi r8, r11, 0x10c0
	ctx.r[8].s64 = ctx.r[11].s64 + 4288;
	// 82672198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267219C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826721A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826721A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826721A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826721AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826721B0: 386A2628  addi r3, r10, 0x2628
	ctx.r[3].s64 = ctx.r[10].s64 + 9768;
	// 826721B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826721B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826721BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826721C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826721C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826721C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826721CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826721D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826721D4: 4BDF4C4D  bl 0x82466e20
	ctx.lr = 0x826721D8;
	sub_82466E20(ctx, base);
	// 826721D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826721DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826721E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826721E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826721E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826721E8 size=108
    let mut pc: u32 = 0x826721E8;
    'dispatch: loop {
        match pc {
            0x826721E8 => {
    //   block [0x826721E8..0x82672254)
	// 826721E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826721EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826721F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826721F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826721F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826721FC: 38EB10D8  addi r7, r11, 0x10d8
	ctx.r[7].s64 = ctx.r[11].s64 + 4312;
	// 82672200: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82672204: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82672208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267220C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672218: 386A2658  addi r3, r10, 0x2658
	ctx.r[3].s64 = ctx.r[10].s64 + 9816;
	// 8267221C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267222C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267223C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672240: 4BDF4BE1  bl 0x82466e20
	ctx.lr = 0x82672244;
	sub_82466E20(ctx, base);
	// 82672244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672258 size=112
    let mut pc: u32 = 0x82672258;
    'dispatch: loop {
        match pc {
            0x82672258 => {
    //   block [0x82672258..0x826722C8)
	// 82672258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672264: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672268: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267226C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672274: 390B1108  addi r8, r11, 0x1108
	ctx.r[8].s64 = ctx.r[11].s64 + 4360;
	// 82672278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267227C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82672280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267228C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672290: 386A2688  addi r3, r10, 0x2688
	ctx.r[3].s64 = ctx.r[10].s64 + 9864;
	// 82672294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267229C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826722A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826722A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826722A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826722AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826722B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826722B4: 4BDF4B6D  bl 0x82466e20
	ctx.lr = 0x826722B8;
	sub_82466E20(ctx, base);
	// 826722B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826722BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826722C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826722C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826722C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826722C8 size=108
    let mut pc: u32 = 0x826722C8;
    'dispatch: loop {
        match pc {
            0x826722C8 => {
    //   block [0x826722C8..0x82672334)
	// 826722C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826722CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826722D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826722D4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826722D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826722DC: 38EB1120  addi r7, r11, 0x1120
	ctx.r[7].s64 = ctx.r[11].s64 + 4384;
	// 826722E0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826722E4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826722E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826722EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826722F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826722F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826722F8: 386A26B8  addi r3, r10, 0x26b8
	ctx.r[3].s64 = ctx.r[10].s64 + 9912;
	// 826722FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267230C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267231C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672320: 4BDF4B01  bl 0x82466e20
	ctx.lr = 0x82672324;
	sub_82466E20(ctx, base);
	// 82672324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267232C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672338 size=112
    let mut pc: u32 = 0x82672338;
    'dispatch: loop {
        match pc {
            0x82672338 => {
    //   block [0x82672338..0x826723A8)
	// 82672338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267233C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672344: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672348: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267234C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672354: 390B11F8  addi r8, r11, 0x11f8
	ctx.r[8].s64 = ctx.r[11].s64 + 4600;
	// 82672358: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8267235C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82672360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267236C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672370: 386A26E8  addi r3, r10, 0x26e8
	ctx.r[3].s64 = ctx.r[10].s64 + 9960;
	// 82672374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267237C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267238C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672394: 4BDF4A8D  bl 0x82466e20
	ctx.lr = 0x82672398;
	sub_82466E20(ctx, base);
	// 82672398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267239C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826723A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826723A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826723A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826723A8 size=108
    let mut pc: u32 = 0x826723A8;
    'dispatch: loop {
        match pc {
            0x826723A8 => {
    //   block [0x826723A8..0x82672414)
	// 826723A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826723AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826723B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826723B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826723B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826723BC: 38EB13A8  addi r7, r11, 0x13a8
	ctx.r[7].s64 = ctx.r[11].s64 + 5032;
	// 826723C0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826723C4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826723C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826723CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826723D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826723D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826723D8: 386A2718  addi r3, r10, 0x2718
	ctx.r[3].s64 = ctx.r[10].s64 + 10008;
	// 826723DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826723E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826723E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826723E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826723EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826723F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826723F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826723F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826723FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672400: 4BDF4A21  bl 0x82466e20
	ctx.lr = 0x82672404;
	sub_82466E20(ctx, base);
	// 82672404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267240C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672418 size=112
    let mut pc: u32 = 0x82672418;
    'dispatch: loop {
        match pc {
            0x82672418 => {
    //   block [0x82672418..0x82672488)
	// 82672418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267241C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672424: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672428: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267242C: 38AA1218  addi r5, r10, 0x1218
	ctx.r[5].s64 = ctx.r[10].s64 + 4632;
	// 82672430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672434: 390B1540  addi r8, r11, 0x1540
	ctx.r[8].s64 = ctx.r[11].s64 + 5440;
	// 82672438: 3920001A  li r9, 0x1a
	ctx.r[9].s64 = 26;
	// 8267243C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82672440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672444: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267244C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672450: 386A2748  addi r3, r10, 0x2748
	ctx.r[3].s64 = ctx.r[10].s64 + 10056;
	// 82672454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267245C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267246C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672474: 4BDF49AD  bl 0x82466e20
	ctx.lr = 0x82672478;
	sub_82466E20(ctx, base);
	// 82672478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267247C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672488 size=100
    let mut pc: u32 = 0x82672488;
    'dispatch: loop {
        match pc {
            0x82672488 => {
    //   block [0x82672488..0x826724EC)
	// 82672488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267248C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672494: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267249C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 826724A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826724A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826724A8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826724AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826724B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826724B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826724B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826724BC: 386A2778  addi r3, r10, 0x2778
	ctx.r[3].s64 = ctx.r[10].s64 + 10104;
	// 826724C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826724C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826724C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826724CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826724D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826724D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826724D8: 4BDF4949  bl 0x82466e20
	ctx.lr = 0x826724DC;
	sub_82466E20(ctx, base);
	// 826724DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826724E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826724E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826724E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826724F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826724F0 size=112
    let mut pc: u32 = 0x826724F0;
    'dispatch: loop {
        match pc {
            0x826724F0 => {
    //   block [0x826724F0..0x82672560)
	// 826724F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826724F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826724F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826724FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672500: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672504: 38AA2778  addi r5, r10, 0x2778
	ctx.r[5].s64 = ctx.r[10].s64 + 10104;
	// 82672508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267250C: 390B17B0  addi r8, r11, 0x17b0
	ctx.r[8].s64 = ctx.r[11].s64 + 6064;
	// 82672510: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82672514: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82672518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267251C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672528: 386A27A8  addi r3, r10, 0x27a8
	ctx.r[3].s64 = ctx.r[10].s64 + 10152;
	// 8267252C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267253C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267254C: 4BDF48D5  bl 0x82466e20
	ctx.lr = 0x82672550;
	sub_82466E20(ctx, base);
	// 82672550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267255C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672560 size=100
    let mut pc: u32 = 0x82672560;
    'dispatch: loop {
        match pc {
            0x82672560 => {
    //   block [0x82672560..0x826725C4)
	// 82672560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267256C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672574: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267257C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672580: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82672584: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267258C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672594: 386A27D8  addi r3, r10, 0x27d8
	ctx.r[3].s64 = ctx.r[10].s64 + 10200;
	// 82672598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267259C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826725A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826725A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826725A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826725AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826725B0: 4BDF4871  bl 0x82466e20
	ctx.lr = 0x826725B4;
	sub_82466E20(ctx, base);
	// 826725B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826725B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826725BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826725C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826725C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826725C8 size=108
    let mut pc: u32 = 0x826725C8;
    'dispatch: loop {
        match pc {
            0x826725C8 => {
    //   block [0x826725C8..0x82672634)
	// 826725C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826725CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826725D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826725D4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826725D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826725DC: 38EB1828  addi r7, r11, 0x1828
	ctx.r[7].s64 = ctx.r[11].s64 + 6184;
	// 826725E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826725E4: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826725E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826725EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826725F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826725F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826725F8: 386A2808  addi r3, r10, 0x2808
	ctx.r[3].s64 = ctx.r[10].s64 + 10248;
	// 826725FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267260C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267261C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672620: 4BDF4801  bl 0x82466e20
	ctx.lr = 0x82672624;
	sub_82466E20(ctx, base);
	// 82672624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672628: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267262C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672630: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672638 size=112
    let mut pc: u32 = 0x82672638;
    'dispatch: loop {
        match pc {
            0x82672638 => {
    //   block [0x82672638..0x826726A8)
	// 82672638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267263C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672644: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672648: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267264C: 38AA27D8  addi r5, r10, 0x27d8
	ctx.r[5].s64 = ctx.r[10].s64 + 10200;
	// 82672650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672654: 390B1870  addi r8, r11, 0x1870
	ctx.r[8].s64 = ctx.r[11].s64 + 6256;
	// 82672658: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267265C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82672660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267266C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672670: 386A2838  addi r3, r10, 0x2838
	ctx.r[3].s64 = ctx.r[10].s64 + 10296;
	// 82672674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267267C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267268C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672694: 4BDF478D  bl 0x82466e20
	ctx.lr = 0x82672698;
	sub_82466E20(ctx, base);
	// 82672698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267269C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826726A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826726A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826726A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826726A8 size=100
    let mut pc: u32 = 0x826726A8;
    'dispatch: loop {
        match pc {
            0x826726A8 => {
    //   block [0x826726A8..0x8267270C)
	// 826726A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826726AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826726B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826726B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826726B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826726BC: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 826726C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826726C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826726C8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826726CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826726D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826726D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826726D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826726DC: 386A2868  addi r3, r10, 0x2868
	ctx.r[3].s64 = ctx.r[10].s64 + 10344;
	// 826726E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826726E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826726E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826726EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826726F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826726F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826726F8: 4BDF4729  bl 0x82466e20
	ctx.lr = 0x826726FC;
	sub_82466E20(ctx, base);
	// 826726FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672710 size=100
    let mut pc: u32 = 0x82672710;
    'dispatch: loop {
        match pc {
            0x82672710 => {
    //   block [0x82672710..0x82672774)
	// 82672710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267271C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672724: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267272C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672730: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82672734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267273C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672744: 386A2898  addi r3, r10, 0x2898
	ctx.r[3].s64 = ctx.r[10].s64 + 10392;
	// 82672748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267274C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672750: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82672754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672758: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267275C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672760: 4BDF46C1  bl 0x82466e20
	ctx.lr = 0x82672764;
	sub_82466E20(ctx, base);
	// 82672764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267276C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672778 size=112
    let mut pc: u32 = 0x82672778;
    'dispatch: loop {
        match pc {
            0x82672778 => {
    //   block [0x82672778..0x826727E8)
	// 82672778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267277C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672788: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267278C: 38AA2868  addi r5, r10, 0x2868
	ctx.r[5].s64 = ctx.r[10].s64 + 10344;
	// 82672790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672794: 390B18A0  addi r8, r11, 0x18a0
	ctx.r[8].s64 = ctx.r[11].s64 + 6304;
	// 82672798: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267279C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826727A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826727A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826727A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826727AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826727B0: 386A28C8  addi r3, r10, 0x28c8
	ctx.r[3].s64 = ctx.r[10].s64 + 10440;
	// 826727B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826727B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826727BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826727C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826727C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826727C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826727CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826727D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826727D4: 4BDF464D  bl 0x82466e20
	ctx.lr = 0x826727D8;
	sub_82466E20(ctx, base);
	// 826727D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826727DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826727E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826727E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826727E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826727E8 size=112
    let mut pc: u32 = 0x826727E8;
    'dispatch: loop {
        match pc {
            0x826727E8 => {
    //   block [0x826727E8..0x82672858)
	// 826727E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826727EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826727F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826727F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826727F8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826727FC: 38AA2898  addi r5, r10, 0x2898
	ctx.r[5].s64 = ctx.r[10].s64 + 10392;
	// 82672800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672804: 390B1900  addi r8, r11, 0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + 6400;
	// 82672808: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267280C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82672810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267281C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672820: 386A28F8  addi r3, r10, 0x28f8
	ctx.r[3].s64 = ctx.r[10].s64 + 10488;
	// 82672824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267282C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267283C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672844: 4BDF45DD  bl 0x82466e20
	ctx.lr = 0x82672848;
	sub_82466E20(ctx, base);
	// 82672848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267284C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672858 size=100
    let mut pc: u32 = 0x82672858;
    'dispatch: loop {
        match pc {
            0x82672858 => {
    //   block [0x82672858..0x826728BC)
	// 82672858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267285C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672864: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267286C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672878: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8267287C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267288C: 386A2928  addi r3, r10, 0x2928
	ctx.r[3].s64 = ctx.r[10].s64 + 10536;
	// 82672890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672894: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672898: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267289C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826728A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826728A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826728A8: 4BDF4579  bl 0x82466e20
	ctx.lr = 0x826728AC;
	sub_82466E20(ctx, base);
	// 826728AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826728B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826728B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826728B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826728C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826728C0 size=112
    let mut pc: u32 = 0x826728C0;
    'dispatch: loop {
        match pc {
            0x826728C0 => {
    //   block [0x826728C0..0x82672930)
	// 826728C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826728C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826728C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826728CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826728D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826728D4: 38AA2928  addi r5, r10, 0x2928
	ctx.r[5].s64 = ctx.r[10].s64 + 10536;
	// 826728D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826728DC: 390B1960  addi r8, r11, 0x1960
	ctx.r[8].s64 = ctx.r[11].s64 + 6496;
	// 826728E0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826728E4: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826728E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826728EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826728F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826728F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826728F8: 386A2958  addi r3, r10, 0x2958
	ctx.r[3].s64 = ctx.r[10].s64 + 10584;
	// 826728FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267290C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267291C: 4BDF4505  bl 0x82466e20
	ctx.lr = 0x82672920;
	sub_82466E20(ctx, base);
	// 82672920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267292C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672930 size=108
    let mut pc: u32 = 0x82672930;
    'dispatch: loop {
        match pc {
            0x82672930 => {
    //   block [0x82672930..0x8267299C)
	// 82672930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267293C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672944: 38EB1A50  addi r7, r11, 0x1a50
	ctx.r[7].s64 = ctx.r[11].s64 + 6736;
	// 82672948: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8267294C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82672950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672954: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267295C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672960: 386A2988  addi r3, r10, 0x2988
	ctx.r[3].s64 = ctx.r[10].s64 + 10632;
	// 82672964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267296C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267297C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672988: 4BDF4499  bl 0x82466e20
	ctx.lr = 0x8267298C;
	sub_82466E20(ctx, base);
	// 8267298C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826729A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826729A0 size=108
    let mut pc: u32 = 0x826729A0;
    'dispatch: loop {
        match pc {
            0x826729A0 => {
    //   block [0x826729A0..0x82672A0C)
	// 826729A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826729A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826729A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826729AC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 826729B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826729B4: 38EB1B40  addi r7, r11, 0x1b40
	ctx.r[7].s64 = ctx.r[11].s64 + 6976;
	// 826729B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826729BC: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826729C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826729C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826729C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826729CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826729D0: 386A29B8  addi r3, r10, 0x29b8
	ctx.r[3].s64 = ctx.r[10].s64 + 10680;
	// 826729D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826729D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826729DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826729E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826729E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826729E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826729EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826729F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826729F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826729F8: 4BDF4429  bl 0x82466e20
	ctx.lr = 0x826729FC;
	sub_82466E20(ctx, base);
	// 826729FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672A10 size=108
    let mut pc: u32 = 0x82672A10;
    'dispatch: loop {
        match pc {
            0x82672A10 => {
    //   block [0x82672A10..0x82672A7C)
	// 82672A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672A1C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672A20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672A24: 38EB1B88  addi r7, r11, 0x1b88
	ctx.r[7].s64 = ctx.r[11].s64 + 7048;
	// 82672A28: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82672A2C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82672A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672A34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672A40: 386A29E8  addi r3, r10, 0x29e8
	ctx.r[3].s64 = ctx.r[10].s64 + 10728;
	// 82672A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672A68: 4BDF43B9  bl 0x82466e20
	ctx.lr = 0x82672A6C;
	sub_82466E20(ctx, base);
	// 82672A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672A80 size=108
    let mut pc: u32 = 0x82672A80;
    'dispatch: loop {
        match pc {
            0x82672A80 => {
    //   block [0x82672A80..0x82672AEC)
	// 82672A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672A8C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672A94: 38EB1C60  addi r7, r11, 0x1c60
	ctx.r[7].s64 = ctx.r[11].s64 + 7264;
	// 82672A98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82672A9C: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 82672AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672AA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672AB0: 386A2A18  addi r3, r10, 0x2a18
	ctx.r[3].s64 = ctx.r[10].s64 + 10776;
	// 82672AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672AD8: 4BDF4349  bl 0x82466e20
	ctx.lr = 0x82672ADC;
	sub_82466E20(ctx, base);
	// 82672ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672AF0 size=100
    let mut pc: u32 = 0x82672AF0;
    'dispatch: loop {
        match pc {
            0x82672AF0 => {
    //   block [0x82672AF0..0x82672B54)
	// 82672AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672AFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672B04: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672B10: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82672B14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672B24: 386A2A48  addi r3, r10, 0x2a48
	ctx.r[3].s64 = ctx.r[10].s64 + 10824;
	// 82672B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672B2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672B30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82672B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672B38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82672B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672B40: 4BDF42E1  bl 0x82466e20
	ctx.lr = 0x82672B44;
	sub_82466E20(ctx, base);
	// 82672B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672B58 size=112
    let mut pc: u32 = 0x82672B58;
    'dispatch: loop {
        match pc {
            0x82672B58 => {
    //   block [0x82672B58..0x82672BC8)
	// 82672B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672B64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672B68: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672B6C: 38AA2A48  addi r5, r10, 0x2a48
	ctx.r[5].s64 = ctx.r[10].s64 + 10824;
	// 82672B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672B74: 390B1C78  addi r8, r11, 0x1c78
	ctx.r[8].s64 = ctx.r[11].s64 + 7288;
	// 82672B78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82672B7C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 82672B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672B84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672B90: 386A2A78  addi r3, r10, 0x2a78
	ctx.r[3].s64 = ctx.r[10].s64 + 10872;
	// 82672B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672BB4: 4BDF426D  bl 0x82466e20
	ctx.lr = 0x82672BB8;
	sub_82466E20(ctx, base);
	// 82672BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672BC8 size=108
    let mut pc: u32 = 0x82672BC8;
    'dispatch: loop {
        match pc {
            0x82672BC8 => {
    //   block [0x82672BC8..0x82672C34)
	// 82672BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672BD4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672BD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672BDC: 38EB1CC0  addi r7, r11, 0x1cc0
	ctx.r[7].s64 = ctx.r[11].s64 + 7360;
	// 82672BE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82672BE4: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82672BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672BEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672BF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672BF8: 386A2AA8  addi r3, r10, 0x2aa8
	ctx.r[3].s64 = ctx.r[10].s64 + 10920;
	// 82672BFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672C00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672C08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672C10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672C20: 4BDF4201  bl 0x82466e20
	ctx.lr = 0x82672C24;
	sub_82466E20(ctx, base);
	// 82672C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672C38 size=112
    let mut pc: u32 = 0x82672C38;
    'dispatch: loop {
        match pc {
            0x82672C38 => {
    //   block [0x82672C38..0x82672CA8)
	// 82672C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672C44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672C48: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672C4C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672C54: 390B1D08  addi r8, r11, 0x1d08
	ctx.r[8].s64 = ctx.r[11].s64 + 7432;
	// 82672C58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82672C5C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82672C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672C64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672C70: 386A2AD8  addi r3, r10, 0x2ad8
	ctx.r[3].s64 = ctx.r[10].s64 + 10968;
	// 82672C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672C94: 4BDF418D  bl 0x82466e20
	ctx.lr = 0x82672C98;
	sub_82466E20(ctx, base);
	// 82672C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672CA8 size=108
    let mut pc: u32 = 0x82672CA8;
    'dispatch: loop {
        match pc {
            0x82672CA8 => {
    //   block [0x82672CA8..0x82672D14)
	// 82672CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672CB4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672CBC: 38EB1D20  addi r7, r11, 0x1d20
	ctx.r[7].s64 = ctx.r[11].s64 + 7456;
	// 82672CC0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82672CC4: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82672CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672CCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672CD8: 386A2B08  addi r3, r10, 0x2b08
	ctx.r[3].s64 = ctx.r[10].s64 + 11016;
	// 82672CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672D00: 4BDF4121  bl 0x82466e20
	ctx.lr = 0x82672D04;
	sub_82466E20(ctx, base);
	// 82672D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672D18 size=112
    let mut pc: u32 = 0x82672D18;
    'dispatch: loop {
        match pc {
            0x82672D18 => {
    //   block [0x82672D18..0x82672D88)
	// 82672D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672D24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672D28: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672D2C: 38AA2AD8  addi r5, r10, 0x2ad8
	ctx.r[5].s64 = ctx.r[10].s64 + 10968;
	// 82672D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672D34: 390B1D68  addi r8, r11, 0x1d68
	ctx.r[8].s64 = ctx.r[11].s64 + 7528;
	// 82672D38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82672D3C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82672D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672D44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672D50: 386A2B38  addi r3, r10, 0x2b38
	ctx.r[3].s64 = ctx.r[10].s64 + 11064;
	// 82672D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672D74: 4BDF40AD  bl 0x82466e20
	ctx.lr = 0x82672D78;
	sub_82466E20(ctx, base);
	// 82672D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672D88 size=100
    let mut pc: u32 = 0x82672D88;
    'dispatch: loop {
        match pc {
            0x82672D88 => {
    //   block [0x82672D88..0x82672DEC)
	// 82672D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672D94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672D9C: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672DA8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 82672DAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672DB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672DB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672DBC: 386A2B68  addi r3, r10, 0x2b68
	ctx.r[3].s64 = ctx.r[10].s64 + 11112;
	// 82672DC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672DC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672DC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82672DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672DD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82672DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672DD8: 4BDF4049  bl 0x82466e20
	ctx.lr = 0x82672DDC;
	sub_82466E20(ctx, base);
	// 82672DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672DF0 size=112
    let mut pc: u32 = 0x82672DF0;
    'dispatch: loop {
        match pc {
            0x82672DF0 => {
    //   block [0x82672DF0..0x82672E60)
	// 82672DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672DFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672E00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672E04: 38AA2B68  addi r5, r10, 0x2b68
	ctx.r[5].s64 = ctx.r[10].s64 + 11112;
	// 82672E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672E0C: 390B1D80  addi r8, r11, 0x1d80
	ctx.r[8].s64 = ctx.r[11].s64 + 7552;
	// 82672E10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82672E14: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82672E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672E1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672E28: 386A2B98  addi r3, r10, 0x2b98
	ctx.r[3].s64 = ctx.r[10].s64 + 11160;
	// 82672E2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672E4C: 4BDF3FD5  bl 0x82466e20
	ctx.lr = 0x82672E50;
	sub_82466E20(ctx, base);
	// 82672E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672E60 size=108
    let mut pc: u32 = 0x82672E60;
    'dispatch: loop {
        match pc {
            0x82672E60 => {
    //   block [0x82672E60..0x82672ECC)
	// 82672E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672E6C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672E70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672E74: 38EB1E28  addi r7, r11, 0x1e28
	ctx.r[7].s64 = ctx.r[11].s64 + 7720;
	// 82672E78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82672E7C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 82672E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672E88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82672E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672E90: 386A2BC8  addi r3, r10, 0x2bc8
	ctx.r[3].s64 = ctx.r[10].s64 + 11208;
	// 82672E94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82672E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672EB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82672EB8: 4BDF3F69  bl 0x82466e20
	ctx.lr = 0x82672EBC;
	sub_82466E20(ctx, base);
	// 82672EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672ED0 size=112
    let mut pc: u32 = 0x82672ED0;
    'dispatch: loop {
        match pc {
            0x82672ED0 => {
    //   block [0x82672ED0..0x82672F40)
	// 82672ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672EDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672EE0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672EE4: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672EE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672EEC: 390B1E58  addi r8, r11, 0x1e58
	ctx.r[8].s64 = ctx.r[11].s64 + 7768;
	// 82672EF0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82672EF4: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82672EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672EFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672F00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672F08: 386A2BF8  addi r3, r10, 0x2bf8
	ctx.r[3].s64 = ctx.r[10].s64 + 11256;
	// 82672F0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672F10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672F2C: 4BDF3EF5  bl 0x82466e20
	ctx.lr = 0x82672F30;
	sub_82466E20(ctx, base);
	// 82672F30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672F34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672F38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672F3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672F40 size=112
    let mut pc: u32 = 0x82672F40;
    'dispatch: loop {
        match pc {
            0x82672F40 => {
    //   block [0x82672F40..0x82672FB0)
	// 82672F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672F4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672F50: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82672F54: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672F58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672F5C: 390B1EA0  addi r8, r11, 0x1ea0
	ctx.r[8].s64 = ctx.r[11].s64 + 7840;
	// 82672F60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82672F64: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 82672F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672F6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672F70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82672F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672F78: 386A2C28  addi r3, r10, 0x2c28
	ctx.r[3].s64 = ctx.r[10].s64 + 11304;
	// 82672F7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82672F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82672F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672F9C: 4BDF3E85  bl 0x82466e20
	ctx.lr = 0x82672FA0;
	sub_82466E20(ctx, base);
	// 82672FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82672FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82672FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82672FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82672FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82672FB0 size=100
    let mut pc: u32 = 0x82672FB0;
    'dispatch: loop {
        match pc {
            0x82672FB0 => {
    //   block [0x82672FB0..0x82673014)
	// 82672FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82672FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82672FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82672FBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82672FC4: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 82672FC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82672FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82672FD0: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82672FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82672FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82672FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82672FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82672FE4: 386A2C58  addi r3, r10, 0x2c58
	ctx.r[3].s64 = ctx.r[10].s64 + 11352;
	// 82672FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82672FEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82672FF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82672FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82672FF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82672FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673000: 4BDF3E21  bl 0x82466e20
	ctx.lr = 0x82673004;
	sub_82466E20(ctx, base);
	// 82673004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82673008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267300C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82673018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82673018 size=112
    let mut pc: u32 = 0x82673018;
    'dispatch: loop {
        match pc {
            0x82673018 => {
    //   block [0x82673018..0x82673088)
	// 82673018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82673020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82673024: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673028: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8267302C: 38AA2C58  addi r5, r10, 0x2c58
	ctx.r[5].s64 = ctx.r[10].s64 + 11352;
	// 82673030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82673034: 390B1EE8  addi r8, r11, 0x1ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 7912;
	// 82673038: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267303C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82673040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82673044: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82673048: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267304C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82673050: 386A2C88  addi r3, r10, 0x2c88
	ctx.r[3].s64 = ctx.r[10].s64 + 11400;
	// 82673054: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82673058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267305C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82673060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82673064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82673068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267306C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82673070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82673074: 4BDF3DAD  bl 0x82466e20
	ctx.lr = 0x82673078;
	sub_82466E20(ctx, base);
	// 82673078: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267307C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82673080: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82673084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


