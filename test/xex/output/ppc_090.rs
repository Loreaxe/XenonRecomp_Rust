pub fn sub_8264C4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C4E8 size=108
    let mut pc: u32 = 0x8264C4E8;
    'dispatch: loop {
        match pc {
            0x8264C4E8 => {
    //   block [0x8264C4E8..0x8264C554)
	// 8264C4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C4F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C4FC: 38EB36A8  addi r7, r11, 0x36a8
	ctx.r[7].s64 = ctx.r[11].s64 + 13992;
	// 8264C500: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8264C504: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 8264C508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C50C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C510: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C518: 386A26E0  addi r3, r10, 0x26e0
	ctx.r[3].s64 = ctx.r[10].s64 + 9952;
	// 8264C51C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C53C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C540: 4BE1A8E1  bl 0x82466e20
	ctx.lr = 0x8264C544;
	sub_82466E20(ctx, base);
	// 8264C544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C54C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C558 size=108
    let mut pc: u32 = 0x8264C558;
    'dispatch: loop {
        match pc {
            0x8264C558 => {
    //   block [0x8264C558..0x8264C5C4)
	// 8264C558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C564: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C56C: 38EB3768  addi r7, r11, 0x3768
	ctx.r[7].s64 = ctx.r[11].s64 + 14184;
	// 8264C570: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8264C574: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 8264C578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C57C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C580: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C588: 386A2710  addi r3, r10, 0x2710
	ctx.r[3].s64 = ctx.r[10].s64 + 10000;
	// 8264C58C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C5A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C5AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C5B0: 4BE1A871  bl 0x82466e20
	ctx.lr = 0x8264C5B4;
	sub_82466E20(ctx, base);
	// 8264C5B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C5B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C5BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C5C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C5C8 size=112
    let mut pc: u32 = 0x8264C5C8;
    'dispatch: loop {
        match pc {
            0x8264C5C8 => {
    //   block [0x8264C5C8..0x8264C638)
	// 8264C5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C5D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C5D4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264C5D8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8264C5DC: 38EA3810  addi r7, r10, 0x3810
	ctx.r[7].s64 = ctx.r[10].s64 + 14352;
	// 8264C5E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C5E4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264C5E8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8264C5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C5F0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C5F4: 396B9DF8  addi r11, r11, -0x6208
	ctx.r[11].s64 = ctx.r[11].s64 + -25096;
	// 8264C5F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C5FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C600: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C604: 386A2740  addi r3, r10, 0x2740
	ctx.r[3].s64 = ctx.r[10].s64 + 10048;
	// 8264C608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C60C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264C610: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C614: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264C618: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C61C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C620: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C624: 4BE1A7FD  bl 0x82466e20
	ctx.lr = 0x8264C628;
	sub_82466E20(ctx, base);
	// 8264C628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C638 size=108
    let mut pc: u32 = 0x8264C638;
    'dispatch: loop {
        match pc {
            0x8264C638 => {
    //   block [0x8264C638..0x8264C6A4)
	// 8264C638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C644: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C64C: 38EB3930  addi r7, r11, 0x3930
	ctx.r[7].s64 = ctx.r[11].s64 + 14640;
	// 8264C650: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264C654: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 8264C658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C65C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C660: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C668: 386A2770  addi r3, r10, 0x2770
	ctx.r[3].s64 = ctx.r[10].s64 + 10096;
	// 8264C66C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C67C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C68C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C690: 4BE1A791  bl 0x82466e20
	ctx.lr = 0x8264C694;
	sub_82466E20(ctx, base);
	// 8264C694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C69C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C6A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C6A8 size=108
    let mut pc: u32 = 0x8264C6A8;
    'dispatch: loop {
        match pc {
            0x8264C6A8 => {
    //   block [0x8264C6A8..0x8264C714)
	// 8264C6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C6B4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C6B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C6BC: 38EB3990  addi r7, r11, 0x3990
	ctx.r[7].s64 = ctx.r[11].s64 + 14736;
	// 8264C6C0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8264C6C4: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 8264C6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C6CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C6D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C6D8: 386A27A0  addi r3, r10, 0x27a0
	ctx.r[3].s64 = ctx.r[10].s64 + 10144;
	// 8264C6DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C6E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C6EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C6F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C6FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C700: 4BE1A721  bl 0x82466e20
	ctx.lr = 0x8264C704;
	sub_82466E20(ctx, base);
	// 8264C704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C718 size=108
    let mut pc: u32 = 0x8264C718;
    'dispatch: loop {
        match pc {
            0x8264C718 => {
    //   block [0x8264C718..0x8264C784)
	// 8264C718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C724: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C72C: 38EB3A98  addi r7, r11, 0x3a98
	ctx.r[7].s64 = ctx.r[11].s64 + 15000;
	// 8264C730: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8264C734: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 8264C738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C73C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C740: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C748: 386A27D0  addi r3, r10, 0x27d0
	ctx.r[3].s64 = ctx.r[10].s64 + 10192;
	// 8264C74C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C76C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C770: 4BE1A6B1  bl 0x82466e20
	ctx.lr = 0x8264C774;
	sub_82466E20(ctx, base);
	// 8264C774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C77C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C788 size=108
    let mut pc: u32 = 0x8264C788;
    'dispatch: loop {
        match pc {
            0x8264C788 => {
    //   block [0x8264C788..0x8264C7F4)
	// 8264C788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C794: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C798: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C79C: 38EB3B70  addi r7, r11, 0x3b70
	ctx.r[7].s64 = ctx.r[11].s64 + 15216;
	// 8264C7A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264C7A4: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 8264C7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C7AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C7B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C7B8: 386A2800  addi r3, r10, 0x2800
	ctx.r[3].s64 = ctx.r[10].s64 + 10240;
	// 8264C7BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C7C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C7DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C7E0: 4BE1A641  bl 0x82466e20
	ctx.lr = 0x8264C7E4;
	sub_82466E20(ctx, base);
	// 8264C7E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C7E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C7EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C7F8 size=108
    let mut pc: u32 = 0x8264C7F8;
    'dispatch: loop {
        match pc {
            0x8264C7F8 => {
    //   block [0x8264C7F8..0x8264C864)
	// 8264C7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C804: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C80C: 38EB3BB8  addi r7, r11, 0x3bb8
	ctx.r[7].s64 = ctx.r[11].s64 + 15288;
	// 8264C810: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264C814: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 8264C818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C81C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C820: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C828: 386A2830  addi r3, r10, 0x2830
	ctx.r[3].s64 = ctx.r[10].s64 + 10288;
	// 8264C82C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C83C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C84C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C850: 4BE1A5D1  bl 0x82466e20
	ctx.lr = 0x8264C854;
	sub_82466E20(ctx, base);
	// 8264C854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C868 size=108
    let mut pc: u32 = 0x8264C868;
    'dispatch: loop {
        match pc {
            0x8264C868 => {
    //   block [0x8264C868..0x8264C8D4)
	// 8264C868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C874: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C87C: 38EB3BD0  addi r7, r11, 0x3bd0
	ctx.r[7].s64 = ctx.r[11].s64 + 15312;
	// 8264C880: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264C884: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 8264C888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C88C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C890: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C898: 386A2860  addi r3, r10, 0x2860
	ctx.r[3].s64 = ctx.r[10].s64 + 10336;
	// 8264C89C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C8A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C8A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C8AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C8B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C8BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C8C0: 4BE1A561  bl 0x82466e20
	ctx.lr = 0x8264C8C4;
	sub_82466E20(ctx, base);
	// 8264C8C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C8C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C8CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C8D8 size=108
    let mut pc: u32 = 0x8264C8D8;
    'dispatch: loop {
        match pc {
            0x8264C8D8 => {
    //   block [0x8264C8D8..0x8264C944)
	// 8264C8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C8E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C8E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C8EC: 38EB3C18  addi r7, r11, 0x3c18
	ctx.r[7].s64 = ctx.r[11].s64 + 15384;
	// 8264C8F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264C8F4: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 8264C8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C8FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C908: 386A2890  addi r3, r10, 0x2890
	ctx.r[3].s64 = ctx.r[10].s64 + 10384;
	// 8264C90C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C92C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C930: 4BE1A4F1  bl 0x82466e20
	ctx.lr = 0x8264C934;
	sub_82466E20(ctx, base);
	// 8264C934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C948 size=112
    let mut pc: u32 = 0x8264C948;
    'dispatch: loop {
        match pc {
            0x8264C948 => {
    //   block [0x8264C948..0x8264C9B8)
	// 8264C948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C954: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C958: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C95C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264C960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C964: 390B3C30  addi r8, r11, 0x3c30
	ctx.r[8].s64 = ctx.r[11].s64 + 15408;
	// 8264C968: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264C96C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 8264C970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C974: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C978: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264C97C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C980: 386A28C0  addi r3, r10, 0x28c0
	ctx.r[3].s64 = ctx.r[10].s64 + 10432;
	// 8264C984: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264C988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C99C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C9A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C9A4: 4BE1A47D  bl 0x82466e20
	ctx.lr = 0x8264C9A8;
	sub_82466E20(ctx, base);
	// 8264C9A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C9B8 size=108
    let mut pc: u32 = 0x8264C9B8;
    'dispatch: loop {
        match pc {
            0x8264C9B8 => {
    //   block [0x8264C9B8..0x8264CA24)
	// 8264C9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C9C4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C9C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C9CC: 38EB3C78  addi r7, r11, 0x3c78
	ctx.r[7].s64 = ctx.r[11].s64 + 15480;
	// 8264C9D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264C9D4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 8264C9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C9DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C9E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C9E8: 386A28F0  addi r3, r10, 0x28f0
	ctx.r[3].s64 = ctx.r[10].s64 + 10480;
	// 8264C9EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C9F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CA00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CA0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264CA10: 4BE1A411  bl 0x82466e20
	ctx.lr = 0x8264CA14;
	sub_82466E20(ctx, base);
	// 8264CA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CA28 size=112
    let mut pc: u32 = 0x8264CA28;
    'dispatch: loop {
        match pc {
            0x8264CA28 => {
    //   block [0x8264CA28..0x8264CA98)
	// 8264CA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CA34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CA38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264CA3C: 38AA28F0  addi r5, r10, 0x28f0
	ctx.r[5].s64 = ctx.r[10].s64 + 10480;
	// 8264CA40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CA44: 390B3CD8  addi r8, r11, 0x3cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 15576;
	// 8264CA48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264CA4C: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 8264CA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CA54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CA58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264CA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CA60: 386A2920  addi r3, r10, 0x2920
	ctx.r[3].s64 = ctx.r[10].s64 + 10528;
	// 8264CA64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264CA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264CA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CA84: 4BE1A39D  bl 0x82466e20
	ctx.lr = 0x8264CA88;
	sub_82466E20(ctx, base);
	// 8264CA88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CA98 size=96
    let mut pc: u32 = 0x8264CA98;
    'dispatch: loop {
        match pc {
            0x8264CA98 => {
    //   block [0x8264CA98..0x8264CAF8)
	// 8264CA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CAA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CAAC: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 8264CAB0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CAB8: 386A2950  addi r3, r10, 0x2950
	ctx.r[3].s64 = ctx.r[10].s64 + 10576;
	// 8264CABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CAC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264CAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CAD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264CADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264CAE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264CAE4: 4BE1A33D  bl 0x82466e20
	ctx.lr = 0x8264CAE8;
	sub_82466E20(ctx, base);
	// 8264CAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CAF8 size=112
    let mut pc: u32 = 0x8264CAF8;
    'dispatch: loop {
        match pc {
            0x8264CAF8 => {
    //   block [0x8264CAF8..0x8264CB68)
	// 8264CAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CB04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CB08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264CB0C: 38AA4750  addi r5, r10, 0x4750
	ctx.r[5].s64 = ctx.r[10].s64 + 18256;
	// 8264CB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CB14: 390B3D20  addi r8, r11, 0x3d20
	ctx.r[8].s64 = ctx.r[11].s64 + 15648;
	// 8264CB18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264CB1C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 8264CB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CB24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CB28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264CB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CB30: 386A2980  addi r3, r10, 0x2980
	ctx.r[3].s64 = ctx.r[10].s64 + 10624;
	// 8264CB34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264CB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CB3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264CB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CB54: 4BE1A2CD  bl 0x82466e20
	ctx.lr = 0x8264CB58;
	sub_82466E20(ctx, base);
	// 8264CB58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CB68 size=96
    let mut pc: u32 = 0x8264CB68;
    'dispatch: loop {
        match pc {
            0x8264CB68 => {
    //   block [0x8264CB68..0x8264CBC8)
	// 8264CB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CB74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CB7C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 8264CB80: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CB84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CB88: 386A29B0  addi r3, r10, 0x29b0
	ctx.r[3].s64 = ctx.r[10].s64 + 10672;
	// 8264CB8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CB90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CB94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264CB98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CB9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CBA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CBA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264CBAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264CBB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264CBB4: 4BE1A26D  bl 0x82466e20
	ctx.lr = 0x8264CBB8;
	sub_82466E20(ctx, base);
	// 8264CBB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CBBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CBC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CBC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CBC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CBC8 size=100
    let mut pc: u32 = 0x8264CBC8;
    'dispatch: loop {
        match pc {
            0x8264CBC8 => {
    //   block [0x8264CBC8..0x8264CC2C)
	// 8264CBC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CBCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CBD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CBD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CBD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CBDC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264CBE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CBE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CBE8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 8264CBEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CBF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CBF4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264CBF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CBFC: 386A29E0  addi r3, r10, 0x29e0
	ctx.r[3].s64 = ctx.r[10].s64 + 10720;
	// 8264CC00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CC04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CC08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264CC0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CC10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264CC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CC18: 4BE1A209  bl 0x82466e20
	ctx.lr = 0x8264CC1C;
	sub_82466E20(ctx, base);
	// 8264CC1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CC20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CC24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CC30 size=104
    let mut pc: u32 = 0x8264CC30;
    'dispatch: loop {
        match pc {
            0x8264CC30 => {
    //   block [0x8264CC30..0x8264CC98)
	// 8264CC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CC3C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264CC40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CC44: 392A9E70  addi r9, r10, -0x6190
	ctx.r[9].s64 = ctx.r[10].s64 + -24976;
	// 8264CC48: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264CC50: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264CC54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CC58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CC5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CC60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CC64: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 8264CC68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CC6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CC70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264CC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CC78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264CC7C: 386A2A10  addi r3, r10, 0x2a10
	ctx.r[3].s64 = ctx.r[10].s64 + 10768;
	// 8264CC80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264CC84: 4BE1A19D  bl 0x82466e20
	ctx.lr = 0x8264CC88;
	sub_82466E20(ctx, base);
	// 8264CC88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CC98 size=96
    let mut pc: u32 = 0x8264CC98;
    'dispatch: loop {
        match pc {
            0x8264CC98 => {
    //   block [0x8264CC98..0x8264CCF8)
	// 8264CC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CCA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CCA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CCA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CCAC: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 8264CCB0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CCB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CCB8: 386A2A40  addi r3, r10, 0x2a40
	ctx.r[3].s64 = ctx.r[10].s64 + 10816;
	// 8264CCBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CCC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CCC4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264CCC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CCCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CCD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CCD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CCD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264CCDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264CCE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264CCE4: 4BE1A13D  bl 0x82466e20
	ctx.lr = 0x8264CCE8;
	sub_82466E20(ctx, base);
	// 8264CCE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CCEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CCF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CCF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CCF8 size=100
    let mut pc: u32 = 0x8264CCF8;
    'dispatch: loop {
        match pc {
            0x8264CCF8 => {
    //   block [0x8264CCF8..0x8264CD5C)
	// 8264CCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CCFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CD04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CD0C: 38AA2A10  addi r5, r10, 0x2a10
	ctx.r[5].s64 = ctx.r[10].s64 + 10768;
	// 8264CD10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CD14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CD18: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 8264CD1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CD24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264CD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CD2C: 386A2A70  addi r3, r10, 0x2a70
	ctx.r[3].s64 = ctx.r[10].s64 + 10864;
	// 8264CD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CD34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CD38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264CD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CD40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264CD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CD48: 4BE1A0D9  bl 0x82466e20
	ctx.lr = 0x8264CD4C;
	sub_82466E20(ctx, base);
	// 8264CD4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CD60 size=112
    let mut pc: u32 = 0x8264CD60;
    'dispatch: loop {
        match pc {
            0x8264CD60 => {
    //   block [0x8264CD60..0x8264CDD0)
	// 8264CD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CD6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CD70: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264CD74: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 8264CD78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CD7C: 390B3D84  addi r8, r11, 0x3d84
	ctx.r[8].s64 = ctx.r[11].s64 + 15748;
	// 8264CD80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264CD84: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 8264CD88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CD8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CD90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264CD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CD98: 386A2AA0  addi r3, r10, 0x2aa0
	ctx.r[3].s64 = ctx.r[10].s64 + 10912;
	// 8264CD9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264CDA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CDA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CDA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CDAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264CDB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CDB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CDB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CDBC: 4BE1A065  bl 0x82466e20
	ctx.lr = 0x8264CDC0;
	sub_82466E20(ctx, base);
	// 8264CDC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CDD0 size=112
    let mut pc: u32 = 0x8264CDD0;
    'dispatch: loop {
        match pc {
            0x8264CDD0 => {
    //   block [0x8264CDD0..0x8264CE40)
	// 8264CDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CDDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CDE0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264CDE4: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 8264CDE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CDEC: 390B3DB4  addi r8, r11, 0x3db4
	ctx.r[8].s64 = ctx.r[11].s64 + 15796;
	// 8264CDF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264CDF4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 8264CDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CDFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CE00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264CE04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CE08: 386A2AD0  addi r3, r10, 0x2ad0
	ctx.r[3].s64 = ctx.r[10].s64 + 10960;
	// 8264CE0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264CE10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CE14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CE18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CE1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264CE20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CE28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CE2C: 4BE19FF5  bl 0x82466e20
	ctx.lr = 0x8264CE30;
	sub_82466E20(ctx, base);
	// 8264CE30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CE40 size=100
    let mut pc: u32 = 0x8264CE40;
    'dispatch: loop {
        match pc {
            0x8264CE40 => {
    //   block [0x8264CE40..0x8264CEA4)
	// 8264CE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CE4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CE54: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 8264CE58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CE5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CE60: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8264CE64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CE68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264CE70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CE74: 386A2B00  addi r3, r10, 0x2b00
	ctx.r[3].s64 = ctx.r[10].s64 + 11008;
	// 8264CE78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CE7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CE80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264CE84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CE88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264CE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CE90: 4BE19F91  bl 0x82466e20
	ctx.lr = 0x8264CE94;
	sub_82466E20(ctx, base);
	// 8264CE94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CEA8 size=96
    let mut pc: u32 = 0x8264CEA8;
    'dispatch: loop {
        match pc {
            0x8264CEA8 => {
    //   block [0x8264CEA8..0x8264CF08)
	// 8264CEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CEB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CEBC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 8264CEC0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CEC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CEC8: 386A2B30  addi r3, r10, 0x2b30
	ctx.r[3].s64 = ctx.r[10].s64 + 11056;
	// 8264CECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CED4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264CED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CEE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264CEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264CEF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264CEF4: 4BE19F2D  bl 0x82466e20
	ctx.lr = 0x8264CEF8;
	sub_82466E20(ctx, base);
	// 8264CEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CF08 size=112
    let mut pc: u32 = 0x8264CF08;
    'dispatch: loop {
        match pc {
            0x8264CF08 => {
    //   block [0x8264CF08..0x8264CF78)
	// 8264CF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CF14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CF18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264CF1C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264CF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CF24: 390B3DCC  addi r8, r11, 0x3dcc
	ctx.r[8].s64 = ctx.r[11].s64 + 15820;
	// 8264CF28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264CF2C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 8264CF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CF34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CF38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264CF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CF40: 386A2B60  addi r3, r10, 0x2b60
	ctx.r[3].s64 = ctx.r[10].s64 + 11104;
	// 8264CF44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264CF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264CF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CF64: 4BE19EBD  bl 0x82466e20
	ctx.lr = 0x8264CF68;
	sub_82466E20(ctx, base);
	// 8264CF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CF78 size=96
    let mut pc: u32 = 0x8264CF78;
    'dispatch: loop {
        match pc {
            0x8264CF78 => {
    //   block [0x8264CF78..0x8264CFD8)
	// 8264CF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CF84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264CF8C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 8264CF90: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264CF98: 386A2B90  addi r3, r10, 0x2b90
	ctx.r[3].s64 = ctx.r[10].s64 + 11152;
	// 8264CF9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264CFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264CFA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264CFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264CFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264CFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264CFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264CFB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264CFBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264CFC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264CFC4: 4BE19E5D  bl 0x82466e20
	ctx.lr = 0x8264CFC8;
	sub_82466E20(ctx, base);
	// 8264CFC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264CFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264CFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264CFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264CFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264CFD8 size=112
    let mut pc: u32 = 0x8264CFD8;
    'dispatch: loop {
        match pc {
            0x8264CFD8 => {
    //   block [0x8264CFD8..0x8264D048)
	// 8264CFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264CFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264CFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264CFE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264CFE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264CFEC: 38AA2B90  addi r5, r10, 0x2b90
	ctx.r[5].s64 = ctx.r[10].s64 + 11152;
	// 8264CFF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264CFF4: 390B3DE4  addi r8, r11, 0x3de4
	ctx.r[8].s64 = ctx.r[11].s64 + 15844;
	// 8264CFF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264CFFC: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 8264D000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D004: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D010: 386A2BC0  addi r3, r10, 0x2bc0
	ctx.r[3].s64 = ctx.r[10].s64 + 11200;
	// 8264D014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D034: 4BE19DED  bl 0x82466e20
	ctx.lr = 0x8264D038;
	sub_82466E20(ctx, base);
	// 8264D038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D048 size=108
    let mut pc: u32 = 0x8264D048;
    'dispatch: loop {
        match pc {
            0x8264D048 => {
    //   block [0x8264D048..0x8264D0B4)
	// 8264D048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D054: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D05C: 38EB3E00  addi r7, r11, 0x3e00
	ctx.r[7].s64 = ctx.r[11].s64 + 15872;
	// 8264D060: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264D064: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8264D068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D06C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264D074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D078: 386A2BF0  addi r3, r10, 0x2bf0
	ctx.r[3].s64 = ctx.r[10].s64 + 11248;
	// 8264D07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264D080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264D0A0: 4BE19D81  bl 0x82466e20
	ctx.lr = 0x8264D0A4;
	sub_82466E20(ctx, base);
	// 8264D0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D0B8 size=112
    let mut pc: u32 = 0x8264D0B8;
    'dispatch: loop {
        match pc {
            0x8264D0B8 => {
    //   block [0x8264D0B8..0x8264D128)
	// 8264D0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D0C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D0C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D0CC: 38AA2D10  addi r5, r10, 0x2d10
	ctx.r[5].s64 = ctx.r[10].s64 + 11536;
	// 8264D0D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D0D4: 390B3E60  addi r8, r11, 0x3e60
	ctx.r[8].s64 = ctx.r[11].s64 + 15968;
	// 8264D0D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264D0DC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8264D0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D0E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D0E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D0F0: 386A2C20  addi r3, r10, 0x2c20
	ctx.r[3].s64 = ctx.r[10].s64 + 11296;
	// 8264D0F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D0F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D10C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D114: 4BE19D0D  bl 0x82466e20
	ctx.lr = 0x8264D118;
	sub_82466E20(ctx, base);
	// 8264D118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D128 size=112
    let mut pc: u32 = 0x8264D128;
    'dispatch: loop {
        match pc {
            0x8264D128 => {
    //   block [0x8264D128..0x8264D198)
	// 8264D128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D134: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D138: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D13C: 38AA2B60  addi r5, r10, 0x2b60
	ctx.r[5].s64 = ctx.r[10].s64 + 11104;
	// 8264D140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D144: 390B3E78  addi r8, r11, 0x3e78
	ctx.r[8].s64 = ctx.r[11].s64 + 15992;
	// 8264D148: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264D14C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8264D150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D154: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D160: 386A2C50  addi r3, r10, 0x2c50
	ctx.r[3].s64 = ctx.r[10].s64 + 11344;
	// 8264D164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D16C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D17C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D184: 4BE19C9D  bl 0x82466e20
	ctx.lr = 0x8264D188;
	sub_82466E20(ctx, base);
	// 8264D188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D18C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D198 size=112
    let mut pc: u32 = 0x8264D198;
    'dispatch: loop {
        match pc {
            0x8264D198 => {
    //   block [0x8264D198..0x8264D208)
	// 8264D198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D1A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D1A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D1AC: 38AA2B60  addi r5, r10, 0x2b60
	ctx.r[5].s64 = ctx.r[10].s64 + 11104;
	// 8264D1B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D1B4: 390B3EA8  addi r8, r11, 0x3ea8
	ctx.r[8].s64 = ctx.r[11].s64 + 16040;
	// 8264D1B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264D1BC: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8264D1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D1C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D1C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D1D0: 386A2C80  addi r3, r10, 0x2c80
	ctx.r[3].s64 = ctx.r[10].s64 + 11392;
	// 8264D1D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D1D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D1F4: 4BE19C2D  bl 0x82466e20
	ctx.lr = 0x8264D1F8;
	sub_82466E20(ctx, base);
	// 8264D1F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D208 size=116
    let mut pc: u32 = 0x8264D208;
    'dispatch: loop {
        match pc {
            0x8264D208 => {
    //   block [0x8264D208..0x8264D27C)
	// 8264D208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D214: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D218: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264D21C: 390B3EC0  addi r8, r11, 0x3ec0
	ctx.r[8].s64 = ctx.r[11].s64 + 16064;
	// 8264D220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D224: 392A9E9C  addi r9, r10, -0x6164
	ctx.r[9].s64 = ctx.r[10].s64 + -24932;
	// 8264D228: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D22C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8264D230: 38AA2D10  addi r5, r10, 0x2d10
	ctx.r[5].s64 = ctx.r[10].s64 + 11536;
	// 8264D234: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D23C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D24C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264D250: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8264D254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264D258: 386B2CB0  addi r3, r11, 0x2cb0
	ctx.r[3].s64 = ctx.r[11].s64 + 11440;
	// 8264D25C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264D260: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D268: 4BE19BB9  bl 0x82466e20
	ctx.lr = 0x8264D26C;
	sub_82466E20(ctx, base);
	// 8264D26C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D280 size=112
    let mut pc: u32 = 0x8264D280;
    'dispatch: loop {
        match pc {
            0x8264D280 => {
    //   block [0x8264D280..0x8264D2F0)
	// 8264D280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D28C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D290: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D294: 38AA2B60  addi r5, r10, 0x2b60
	ctx.r[5].s64 = ctx.r[10].s64 + 11104;
	// 8264D298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D29C: 390B3EF0  addi r8, r11, 0x3ef0
	ctx.r[8].s64 = ctx.r[11].s64 + 16112;
	// 8264D2A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264D2A4: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8264D2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D2AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D2B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D2B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D2B8: 386A2CE0  addi r3, r10, 0x2ce0
	ctx.r[3].s64 = ctx.r[10].s64 + 11488;
	// 8264D2BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D2C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D2C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D2C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D2CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264D2D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D2D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D2D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D2DC: 4BE19B45  bl 0x82466e20
	ctx.lr = 0x8264D2E0;
	sub_82466E20(ctx, base);
	// 8264D2E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D2E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D2E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D2EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D2F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D2F0 size=112
    let mut pc: u32 = 0x8264D2F0;
    'dispatch: loop {
        match pc {
            0x8264D2F0 => {
    //   block [0x8264D2F0..0x8264D360)
	// 8264D2F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D2F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D2F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D2FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D300: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D304: 38AA3280  addi r5, r10, 0x3280
	ctx.r[5].s64 = ctx.r[10].s64 + 12928;
	// 8264D308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D30C: 390B3F08  addi r8, r11, 0x3f08
	ctx.r[8].s64 = ctx.r[11].s64 + 16136;
	// 8264D310: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264D314: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8264D318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D31C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D320: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D328: 386A2D10  addi r3, r10, 0x2d10
	ctx.r[3].s64 = ctx.r[10].s64 + 11536;
	// 8264D32C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D34C: 4BE19AD5  bl 0x82466e20
	ctx.lr = 0x8264D350;
	sub_82466E20(ctx, base);
	// 8264D350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D360 size=116
    let mut pc: u32 = 0x8264D360;
    'dispatch: loop {
        match pc {
            0x8264D360 => {
    //   block [0x8264D360..0x8264D3D4)
	// 8264D360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D36C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264D370: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8264D374: 390A3F20  addi r8, r10, 0x3f20
	ctx.r[8].s64 = ctx.r[10].s64 + 16160;
	// 8264D378: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D37C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264D380: 38AA2D10  addi r5, r10, 0x2d10
	ctx.r[5].s64 = ctx.r[10].s64 + 11536;
	// 8264D384: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D388: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264D38C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D394: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8264D398: 396B9EB0  addi r11, r11, -0x6150
	ctx.r[11].s64 = ctx.r[11].s64 + -24912;
	// 8264D39C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D3A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264D3A4: 386A2D40  addi r3, r10, 0x2d40
	ctx.r[3].s64 = ctx.r[10].s64 + 11584;
	// 8264D3A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264D3AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D3B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264D3B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D3C0: 4BE19A61  bl 0x82466e20
	ctx.lr = 0x8264D3C4;
	sub_82466E20(ctx, base);
	// 8264D3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D3D8 size=112
    let mut pc: u32 = 0x8264D3D8;
    'dispatch: loop {
        match pc {
            0x8264D3D8 => {
    //   block [0x8264D3D8..0x8264D448)
	// 8264D3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D3E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D3E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D3EC: 38AA2CE0  addi r5, r10, 0x2ce0
	ctx.r[5].s64 = ctx.r[10].s64 + 11488;
	// 8264D3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D3F4: 390B3F98  addi r8, r11, 0x3f98
	ctx.r[8].s64 = ctx.r[11].s64 + 16280;
	// 8264D3F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264D3FC: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8264D400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D404: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D410: 386A2D70  addi r3, r10, 0x2d70
	ctx.r[3].s64 = ctx.r[10].s64 + 11632;
	// 8264D414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D434: 4BE199ED  bl 0x82466e20
	ctx.lr = 0x8264D438;
	sub_82466E20(ctx, base);
	// 8264D438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D448 size=112
    let mut pc: u32 = 0x8264D448;
    'dispatch: loop {
        match pc {
            0x8264D448 => {
    //   block [0x8264D448..0x8264D4B8)
	// 8264D448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D458: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D45C: 38AA2D10  addi r5, r10, 0x2d10
	ctx.r[5].s64 = ctx.r[10].s64 + 11536;
	// 8264D460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D464: 390B3FE0  addi r8, r11, 0x3fe0
	ctx.r[8].s64 = ctx.r[11].s64 + 16352;
	// 8264D468: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264D46C: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8264D470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D480: 386A2DA0  addi r3, r10, 0x2da0
	ctx.r[3].s64 = ctx.r[10].s64 + 11680;
	// 8264D484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D4A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D4A4: 4BE1997D  bl 0x82466e20
	ctx.lr = 0x8264D4A8;
	sub_82466E20(ctx, base);
	// 8264D4A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D4B8 size=112
    let mut pc: u32 = 0x8264D4B8;
    'dispatch: loop {
        match pc {
            0x8264D4B8 => {
    //   block [0x8264D4B8..0x8264D528)
	// 8264D4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D4C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D4C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D4CC: 38AA2D10  addi r5, r10, 0x2d10
	ctx.r[5].s64 = ctx.r[10].s64 + 11536;
	// 8264D4D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D4D4: 390B4010  addi r8, r11, 0x4010
	ctx.r[8].s64 = ctx.r[11].s64 + 16400;
	// 8264D4D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264D4DC: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8264D4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D4E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D4F0: 386A2DD0  addi r3, r10, 0x2dd0
	ctx.r[3].s64 = ctx.r[10].s64 + 11728;
	// 8264D4F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D514: 4BE1990D  bl 0x82466e20
	ctx.lr = 0x8264D518;
	sub_82466E20(ctx, base);
	// 8264D518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D528 size=108
    let mut pc: u32 = 0x8264D528;
    'dispatch: loop {
        match pc {
            0x8264D528 => {
    //   block [0x8264D528..0x8264D594)
	// 8264D528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D534: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D53C: 38EB4040  addi r7, r11, 0x4040
	ctx.r[7].s64 = ctx.r[11].s64 + 16448;
	// 8264D540: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264D544: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8264D548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D54C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264D554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D558: 386A2E00  addi r3, r10, 0x2e00
	ctx.r[3].s64 = ctx.r[10].s64 + 11776;
	// 8264D55C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264D560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264D580: 4BE198A1  bl 0x82466e20
	ctx.lr = 0x8264D584;
	sub_82466E20(ctx, base);
	// 8264D584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D598 size=112
    let mut pc: u32 = 0x8264D598;
    'dispatch: loop {
        match pc {
            0x8264D598 => {
    //   block [0x8264D598..0x8264D608)
	// 8264D598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D5A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D5A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D5AC: 38AA2D10  addi r5, r10, 0x2d10
	ctx.r[5].s64 = ctx.r[10].s64 + 11536;
	// 8264D5B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D5B4: 390B4088  addi r8, r11, 0x4088
	ctx.r[8].s64 = ctx.r[11].s64 + 16520;
	// 8264D5B8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8264D5BC: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8264D5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D5C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D5D0: 386A2E30  addi r3, r10, 0x2e30
	ctx.r[3].s64 = ctx.r[10].s64 + 11824;
	// 8264D5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D5F4: 4BE1982D  bl 0x82466e20
	ctx.lr = 0x8264D5F8;
	sub_82466E20(ctx, base);
	// 8264D5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D608 size=116
    let mut pc: u32 = 0x8264D608;
    'dispatch: loop {
        match pc {
            0x8264D608 => {
    //   block [0x8264D608..0x8264D67C)
	// 8264D608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D614: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264D618: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D61C: 392B9EF0  addi r9, r11, -0x6110
	ctx.r[9].s64 = ctx.r[11].s64 + -24848;
	// 8264D620: 38AA2D10  addi r5, r10, 0x2d10
	ctx.r[5].s64 = ctx.r[10].s64 + 11536;
	// 8264D624: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D628: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8264D62C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8264D630: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D634: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8264D638: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D63C: 396B4108  addi r11, r11, 0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + 16648;
	// 8264D640: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8264D644: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D648: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264D64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D650: 386A2E60  addi r3, r10, 0x2e60
	ctx.r[3].s64 = ctx.r[10].s64 + 11872;
	// 8264D654: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264D658: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264D65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D660: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264D664: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264D668: 4BE197B9  bl 0x82466e20
	ctx.lr = 0x8264D66C;
	sub_82466E20(ctx, base);
	// 8264D66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264D680 size=36
    let mut pc: u32 = 0x8264D680;
    'dispatch: loop {
        match pc {
            0x8264D680 => {
    //   block [0x8264D680..0x8264D6A4)
	// 8264D680: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D684: 814B419C  lwz r10, 0x419c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16796 as u32) ) } as u64;
	// 8264D688: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D68C: 396B7FC0  addi r11, r11, 0x7fc0
	ctx.r[11].s64 = ctx.r[11].s64 + 32704;
	// 8264D690: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8264D694: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264D698: 814A4104  lwz r10, 0x4104(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16644 as u32) ) } as u64;
	// 8264D69C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8264D6A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D6A8 size=108
    let mut pc: u32 = 0x8264D6A8;
    'dispatch: loop {
        match pc {
            0x8264D6A8 => {
    //   block [0x8264D6A8..0x8264D714)
	// 8264D6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D6B4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D6B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D6BC: 38EB7FC0  addi r7, r11, 0x7fc0
	ctx.r[7].s64 = ctx.r[11].s64 + 32704;
	// 8264D6C0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8264D6C4: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 8264D6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D6CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D6D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264D6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D6D8: 386A2E90  addi r3, r10, 0x2e90
	ctx.r[3].s64 = ctx.r[10].s64 + 11920;
	// 8264D6DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264D6E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D6EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D6F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D6FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264D700: 4BE19721  bl 0x82466e20
	ctx.lr = 0x8264D704;
	sub_82466E20(ctx, base);
	// 8264D704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264D718 size=24
    let mut pc: u32 = 0x8264D718;
    'dispatch: loop {
        match pc {
            0x8264D718 => {
    //   block [0x8264D718..0x8264D730)
	// 8264D718: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D71C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8264D720: 394A8068  addi r10, r10, -0x7f98
	ctx.r[10].s64 = ctx.r[10].s64 + -32664;
	// 8264D724: 816B4104  lwz r11, 0x4104(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16644 as u32) ) } as u64;
	// 8264D728: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8264D72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D730 size=112
    let mut pc: u32 = 0x8264D730;
    'dispatch: loop {
        match pc {
            0x8264D730 => {
    //   block [0x8264D730..0x8264D7A0)
	// 8264D730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D73C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D740: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8264D744: 38AA2E90  addi r5, r10, 0x2e90
	ctx.r[5].s64 = ctx.r[10].s64 + 11920;
	// 8264D748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D74C: 390B8068  addi r8, r11, -0x7f98
	ctx.r[8].s64 = ctx.r[11].s64 + -32664;
	// 8264D750: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8264D754: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 8264D758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D75C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D768: 386A2EC0  addi r3, r10, 0x2ec0
	ctx.r[3].s64 = ctx.r[10].s64 + 11968;
	// 8264D76C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D78C: 4BE19695  bl 0x82466e20
	ctx.lr = 0x8264D790;
	sub_82466E20(ctx, base);
	// 8264D790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D7A0 size=112
    let mut pc: u32 = 0x8264D7A0;
    'dispatch: loop {
        match pc {
            0x8264D7A0 => {
    //   block [0x8264D7A0..0x8264D810)
	// 8264D7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D7AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D7B0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D7B4: 38AA2E90  addi r5, r10, 0x2e90
	ctx.r[5].s64 = ctx.r[10].s64 + 11920;
	// 8264D7B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D7BC: 390B41A0  addi r8, r11, 0x41a0
	ctx.r[8].s64 = ctx.r[11].s64 + 16800;
	// 8264D7C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264D7C4: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 8264D7C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D7CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D7D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D7D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D7D8: 386A2EF0  addi r3, r10, 0x2ef0
	ctx.r[3].s64 = ctx.r[10].s64 + 12016;
	// 8264D7DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264D7E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D7E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D7F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D7F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D7FC: 4BE19625  bl 0x82466e20
	ctx.lr = 0x8264D800;
	sub_82466E20(ctx, base);
	// 8264D800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D810 size=116
    let mut pc: u32 = 0x8264D810;
    'dispatch: loop {
        match pc {
            0x8264D810 => {
    //   block [0x8264D810..0x8264D884)
	// 8264D810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D81C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D820: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264D824: 390B4200  addi r8, r11, 0x4200
	ctx.r[8].s64 = ctx.r[11].s64 + 16896;
	// 8264D828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D82C: 392A9F70  addi r9, r10, -0x6090
	ctx.r[9].s64 = ctx.r[10].s64 + -24720;
	// 8264D830: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D834: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8264D838: 38AA2CE0  addi r5, r10, 0x2ce0
	ctx.r[5].s64 = ctx.r[10].s64 + 11488;
	// 8264D83C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D844: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D84C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D854: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264D858: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 8264D85C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264D860: 386B2F20  addi r3, r11, 0x2f20
	ctx.r[3].s64 = ctx.r[11].s64 + 12064;
	// 8264D864: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8264D868: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D86C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D870: 4BE195B1  bl 0x82466e20
	ctx.lr = 0x8264D874;
	sub_82466E20(ctx, base);
	// 8264D874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D888 size=100
    let mut pc: u32 = 0x8264D888;
    'dispatch: loop {
        match pc {
            0x8264D888 => {
    //   block [0x8264D888..0x8264D8EC)
	// 8264D888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D894: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D89C: 38AA3070  addi r5, r10, 0x3070
	ctx.r[5].s64 = ctx.r[10].s64 + 12400;
	// 8264D8A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D8A8: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8264D8AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D8BC: 386A2F50  addi r3, r10, 0x2f50
	ctx.r[3].s64 = ctx.r[10].s64 + 12112;
	// 8264D8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D8C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D8C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264D8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D8D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264D8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D8D8: 4BE19549  bl 0x82466e20
	ctx.lr = 0x8264D8DC;
	sub_82466E20(ctx, base);
	// 8264D8DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D8E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D8E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D8F0 size=100
    let mut pc: u32 = 0x8264D8F0;
    'dispatch: loop {
        match pc {
            0x8264D8F0 => {
    //   block [0x8264D8F0..0x8264D954)
	// 8264D8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D8FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D904: 38AA2B60  addi r5, r10, 0x2b60
	ctx.r[5].s64 = ctx.r[10].s64 + 11104;
	// 8264D908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D910: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8264D914: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D924: 386A2F80  addi r3, r10, 0x2f80
	ctx.r[3].s64 = ctx.r[10].s64 + 12160;
	// 8264D928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D92C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D930: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264D934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D938: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264D93C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D940: 4BE194E1  bl 0x82466e20
	ctx.lr = 0x8264D944;
	sub_82466E20(ctx, base);
	// 8264D944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D958 size=108
    let mut pc: u32 = 0x8264D958;
    'dispatch: loop {
        match pc {
            0x8264D958 => {
    //   block [0x8264D958..0x8264D9C4)
	// 8264D958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D964: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D96C: 38EB4290  addi r7, r11, 0x4290
	ctx.r[7].s64 = ctx.r[11].s64 + 17040;
	// 8264D970: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264D974: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8264D978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D97C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D980: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264D984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264D988: 386A2FB0  addi r3, r10, 0x2fb0
	ctx.r[3].s64 = ctx.r[10].s64 + 12208;
	// 8264D98C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264D990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264D994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264D998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264D99C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264D9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264D9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264D9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264D9AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264D9B0: 4BE19471  bl 0x82466e20
	ctx.lr = 0x8264D9B4;
	sub_82466E20(ctx, base);
	// 8264D9B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264D9B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264D9BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264D9C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264D9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264D9C8 size=112
    let mut pc: u32 = 0x8264D9C8;
    'dispatch: loop {
        match pc {
            0x8264D9C8 => {
    //   block [0x8264D9C8..0x8264DA38)
	// 8264D9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264D9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264D9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264D9D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D9D8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264D9DC: 38AA2CE0  addi r5, r10, 0x2ce0
	ctx.r[5].s64 = ctx.r[10].s64 + 11488;
	// 8264D9E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264D9E4: 390B42C0  addi r8, r11, 0x42c0
	ctx.r[8].s64 = ctx.r[11].s64 + 17088;
	// 8264D9E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264D9EC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8264D9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264D9F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264D9F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264D9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DA00: 386A2FE0  addi r3, r10, 0x2fe0
	ctx.r[3].s64 = ctx.r[10].s64 + 12256;
	// 8264DA04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264DA08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DA10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DA14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DA18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DA20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DA24: 4BE193FD  bl 0x82466e20
	ctx.lr = 0x8264DA28;
	sub_82466E20(ctx, base);
	// 8264DA28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DA38 size=108
    let mut pc: u32 = 0x8264DA38;
    'dispatch: loop {
        match pc {
            0x8264DA38 => {
    //   block [0x8264DA38..0x8264DAA4)
	// 8264DA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DA44: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DA4C: 38EB42D8  addi r7, r11, 0x42d8
	ctx.r[7].s64 = ctx.r[11].s64 + 17112;
	// 8264DA50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264DA54: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8264DA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DA5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DA60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264DA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DA68: 386A3010  addi r3, r10, 0x3010
	ctx.r[3].s64 = ctx.r[10].s64 + 12304;
	// 8264DA6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264DA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264DA90: 4BE19391  bl 0x82466e20
	ctx.lr = 0x8264DA94;
	sub_82466E20(ctx, base);
	// 8264DA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264DAA8 size=28
    let mut pc: u32 = 0x8264DAA8;
    'dispatch: loop {
        match pc {
            0x8264DAA8 => {
    //   block [0x8264DAA8..0x8264DAC4)
	// 8264DAA8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DAAC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8264DAB0: 394A8110  addi r10, r10, -0x7ef0
	ctx.r[10].s64 = ctx.r[10].s64 + -32496;
	// 8264DAB4: 816B42F0  lwz r11, 0x42f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17136 as u32) ) } as u64;
	// 8264DAB8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8264DABC: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8264DAC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DAC8 size=108
    let mut pc: u32 = 0x8264DAC8;
    'dispatch: loop {
        match pc {
            0x8264DAC8 => {
    //   block [0x8264DAC8..0x8264DB34)
	// 8264DAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DAD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DAD4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8264DAD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DADC: 38EB8110  addi r7, r11, -0x7ef0
	ctx.r[7].s64 = ctx.r[11].s64 + -32496;
	// 8264DAE0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8264DAE4: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8264DAE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DAEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DAF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264DAF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DAF8: 386A3040  addi r3, r10, 0x3040
	ctx.r[3].s64 = ctx.r[10].s64 + 12352;
	// 8264DAFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264DB00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DB04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DB08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DB10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DB14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DB18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DB1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264DB20: 4BE19301  bl 0x82466e20
	ctx.lr = 0x8264DB24;
	sub_82466E20(ctx, base);
	// 8264DB24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DB28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DB2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DB30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DB38 size=116
    let mut pc: u32 = 0x8264DB38;
    'dispatch: loop {
        match pc {
            0x8264DB38 => {
    //   block [0x8264DB38..0x8264DBAC)
	// 8264DB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DB40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DB44: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DB48: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264DB4C: 390B42F8  addi r8, r11, 0x42f8
	ctx.r[8].s64 = ctx.r[11].s64 + 17144;
	// 8264DB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DB54: 392A9FEC  addi r9, r10, -0x6014
	ctx.r[9].s64 = ctx.r[10].s64 + -24596;
	// 8264DB58: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DB5C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8264DB60: 38AA2CE0  addi r5, r10, 0x2ce0
	ctx.r[5].s64 = ctx.r[10].s64 + 11488;
	// 8264DB64: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264DB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DB6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DB7C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264DB80: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8264DB84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264DB88: 386B3070  addi r3, r11, 0x3070
	ctx.r[3].s64 = ctx.r[11].s64 + 12400;
	// 8264DB8C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8264DB90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DB98: 4BE19289  bl 0x82466e20
	ctx.lr = 0x8264DB9C;
	sub_82466E20(ctx, base);
	// 8264DB9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DBA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DBA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DBA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DBB0 size=112
    let mut pc: u32 = 0x8264DBB0;
    'dispatch: loop {
        match pc {
            0x8264DBB0 => {
    //   block [0x8264DBB0..0x8264DC20)
	// 8264DBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DBB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DBBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DBC0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DBC4: 38AA2C80  addi r5, r10, 0x2c80
	ctx.r[5].s64 = ctx.r[10].s64 + 11392;
	// 8264DBC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DBCC: 390B4370  addi r8, r11, 0x4370
	ctx.r[8].s64 = ctx.r[11].s64 + 17264;
	// 8264DBD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264DBD4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8264DBD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DBDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DBE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264DBE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DBE8: 386A30A0  addi r3, r10, 0x30a0
	ctx.r[3].s64 = ctx.r[10].s64 + 12448;
	// 8264DBEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264DBF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DBF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DBF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DBFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DC00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DC08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DC0C: 4BE19215  bl 0x82466e20
	ctx.lr = 0x8264DC10;
	sub_82466E20(ctx, base);
	// 8264DC10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DC20 size=108
    let mut pc: u32 = 0x8264DC20;
    'dispatch: loop {
        match pc {
            0x8264DC20 => {
    //   block [0x8264DC20..0x8264DC8C)
	// 8264DC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DC2C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DC30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DC34: 38EB4388  addi r7, r11, 0x4388
	ctx.r[7].s64 = ctx.r[11].s64 + 17288;
	// 8264DC38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264DC3C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8264DC40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DC44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DC48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264DC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DC50: 386A30D0  addi r3, r10, 0x30d0
	ctx.r[3].s64 = ctx.r[10].s64 + 12496;
	// 8264DC54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264DC58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DC5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DC60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DC68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DC6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DC70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DC74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264DC78: 4BE191A9  bl 0x82466e20
	ctx.lr = 0x8264DC7C;
	sub_82466E20(ctx, base);
	// 8264DC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DC90 size=112
    let mut pc: u32 = 0x8264DC90;
    'dispatch: loop {
        match pc {
            0x8264DC90 => {
    //   block [0x8264DC90..0x8264DD00)
	// 8264DC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DC9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DCA0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DCA4: 38AA2B60  addi r5, r10, 0x2b60
	ctx.r[5].s64 = ctx.r[10].s64 + 11104;
	// 8264DCA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DCAC: 390B43B8  addi r8, r11, 0x43b8
	ctx.r[8].s64 = ctx.r[11].s64 + 17336;
	// 8264DCB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264DCB4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8264DCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DCBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DCC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264DCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DCC8: 386A3100  addi r3, r10, 0x3100
	ctx.r[3].s64 = ctx.r[10].s64 + 12544;
	// 8264DCCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264DCD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DCE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DCEC: 4BE19135  bl 0x82466e20
	ctx.lr = 0x8264DCF0;
	sub_82466E20(ctx, base);
	// 8264DCF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DD00 size=112
    let mut pc: u32 = 0x8264DD00;
    'dispatch: loop {
        match pc {
            0x8264DD00 => {
    //   block [0x8264DD00..0x8264DD70)
	// 8264DD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DD0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DD10: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DD14: 38AA3280  addi r5, r10, 0x3280
	ctx.r[5].s64 = ctx.r[10].s64 + 12928;
	// 8264DD18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DD1C: 390B43E8  addi r8, r11, 0x43e8
	ctx.r[8].s64 = ctx.r[11].s64 + 17384;
	// 8264DD20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264DD24: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8264DD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DD2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DD30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264DD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DD38: 386A3130  addi r3, r10, 0x3130
	ctx.r[3].s64 = ctx.r[10].s64 + 12592;
	// 8264DD3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264DD40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DD48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DD50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DD54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DD58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DD5C: 4BE190C5  bl 0x82466e20
	ctx.lr = 0x8264DD60;
	sub_82466E20(ctx, base);
	// 8264DD60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DD70 size=100
    let mut pc: u32 = 0x8264DD70;
    'dispatch: loop {
        match pc {
            0x8264DD70 => {
    //   block [0x8264DD70..0x8264DDD4)
	// 8264DD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DD7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DD84: 38AA2B60  addi r5, r10, 0x2b60
	ctx.r[5].s64 = ctx.r[10].s64 + 11104;
	// 8264DD88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DD90: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8264DD94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DDA4: 386A3160  addi r3, r10, 0x3160
	ctx.r[3].s64 = ctx.r[10].s64 + 12640;
	// 8264DDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DDAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DDB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264DDB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DDB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264DDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DDC0: 4BE19061  bl 0x82466e20
	ctx.lr = 0x8264DDC4;
	sub_82466E20(ctx, base);
	// 8264DDC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DDC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DDCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DDD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DDD8 size=112
    let mut pc: u32 = 0x8264DDD8;
    'dispatch: loop {
        match pc {
            0x8264DDD8 => {
    //   block [0x8264DDD8..0x8264DE48)
	// 8264DDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DDE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DDE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DDE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DDEC: 38AA2F80  addi r5, r10, 0x2f80
	ctx.r[5].s64 = ctx.r[10].s64 + 12160;
	// 8264DDF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DDF4: 390B4418  addi r8, r11, 0x4418
	ctx.r[8].s64 = ctx.r[11].s64 + 17432;
	// 8264DDF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264DDFC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8264DE00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DE04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DE08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264DE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DE10: 386A3190  addi r3, r10, 0x3190
	ctx.r[3].s64 = ctx.r[10].s64 + 12688;
	// 8264DE14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264DE18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DE1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DE24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DE2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DE34: 4BE18FED  bl 0x82466e20
	ctx.lr = 0x8264DE38;
	sub_82466E20(ctx, base);
	// 8264DE38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DE48 size=112
    let mut pc: u32 = 0x8264DE48;
    'dispatch: loop {
        match pc {
            0x8264DE48 => {
    //   block [0x8264DE48..0x8264DEB8)
	// 8264DE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DE54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DE58: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DE5C: 38AA2F80  addi r5, r10, 0x2f80
	ctx.r[5].s64 = ctx.r[10].s64 + 12160;
	// 8264DE60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DE64: 390B4460  addi r8, r11, 0x4460
	ctx.r[8].s64 = ctx.r[11].s64 + 17504;
	// 8264DE68: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8264DE6C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8264DE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DE74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DE78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264DE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DE80: 386A31C0  addi r3, r10, 0x31c0
	ctx.r[3].s64 = ctx.r[10].s64 + 12736;
	// 8264DE84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264DE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DE94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DEA4: 4BE18F7D  bl 0x82466e20
	ctx.lr = 0x8264DEA8;
	sub_82466E20(ctx, base);
	// 8264DEA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DEB8 size=108
    let mut pc: u32 = 0x8264DEB8;
    'dispatch: loop {
        match pc {
            0x8264DEB8 => {
    //   block [0x8264DEB8..0x8264DF24)
	// 8264DEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DEC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DEC4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DECC: 38EB4508  addi r7, r11, 0x4508
	ctx.r[7].s64 = ctx.r[11].s64 + 17672;
	// 8264DED0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264DED4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8264DED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DEDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DEE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264DEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DEE8: 386A31F0  addi r3, r10, 0x31f0
	ctx.r[3].s64 = ctx.r[10].s64 + 12784;
	// 8264DEEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264DEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DF0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264DF10: 4BE18F11  bl 0x82466e20
	ctx.lr = 0x8264DF14;
	sub_82466E20(ctx, base);
	// 8264DF14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DF28 size=112
    let mut pc: u32 = 0x8264DF28;
    'dispatch: loop {
        match pc {
            0x8264DF28 => {
    //   block [0x8264DF28..0x8264DF98)
	// 8264DF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DF34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DF38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264DF3C: 38AA2CE0  addi r5, r10, 0x2ce0
	ctx.r[5].s64 = ctx.r[10].s64 + 11488;
	// 8264DF40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DF44: 390B4550  addi r8, r11, 0x4550
	ctx.r[8].s64 = ctx.r[11].s64 + 17744;
	// 8264DF48: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264DF4C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8264DF50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DF54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DF58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264DF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DF60: 386A3220  addi r3, r10, 0x3220
	ctx.r[3].s64 = ctx.r[10].s64 + 12832;
	// 8264DF64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264DF68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DF70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DF74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DF78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DF7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DF80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DF84: 4BE18E9D  bl 0x82466e20
	ctx.lr = 0x8264DF88;
	sub_82466E20(ctx, base);
	// 8264DF88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264DF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264DF98 size=100
    let mut pc: u32 = 0x8264DF98;
    'dispatch: loop {
        match pc {
            0x8264DF98 => {
    //   block [0x8264DF98..0x8264DFFC)
	// 8264DF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264DF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264DFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264DFA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264DFAC: 38AA2D10  addi r5, r10, 0x2d10
	ctx.r[5].s64 = ctx.r[10].s64 + 11536;
	// 8264DFB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264DFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264DFB8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8264DFBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264DFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264DFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264DFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264DFCC: 386A3250  addi r3, r10, 0x3250
	ctx.r[3].s64 = ctx.r[10].s64 + 12880;
	// 8264DFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264DFD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264DFD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264DFDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264DFE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264DFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264DFE8: 4BE18E39  bl 0x82466e20
	ctx.lr = 0x8264DFEC;
	sub_82466E20(ctx, base);
	// 8264DFEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264DFF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264DFF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264DFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E000 size=100
    let mut pc: u32 = 0x8264E000;
    'dispatch: loop {
        match pc {
            0x8264E000 => {
    //   block [0x8264E000..0x8264E064)
	// 8264E000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E00C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E014: 38AA2B60  addi r5, r10, 0x2b60
	ctx.r[5].s64 = ctx.r[10].s64 + 11104;
	// 8264E018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E020: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8264E024: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E034: 386A3280  addi r3, r10, 0x3280
	ctx.r[3].s64 = ctx.r[10].s64 + 12928;
	// 8264E038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E03C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E040: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264E044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E048: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264E04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E050: 4BE18DD1  bl 0x82466e20
	ctx.lr = 0x8264E054;
	sub_82466E20(ctx, base);
	// 8264E054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E068 size=112
    let mut pc: u32 = 0x8264E068;
    'dispatch: loop {
        match pc {
            0x8264E068 => {
    //   block [0x8264E068..0x8264E0D8)
	// 8264E068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E074: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E078: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E07C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264E080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E084: 390B45B0  addi r8, r11, 0x45b0
	ctx.r[8].s64 = ctx.r[11].s64 + 17840;
	// 8264E088: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264E08C: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8264E090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E094: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E09C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E0A0: 386A32B0  addi r3, r10, 0x32b0
	ctx.r[3].s64 = ctx.r[10].s64 + 12976;
	// 8264E0A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E0BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E0C4: 4BE18D5D  bl 0x82466e20
	ctx.lr = 0x8264E0C8;
	sub_82466E20(ctx, base);
	// 8264E0C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E0D8 size=112
    let mut pc: u32 = 0x8264E0D8;
    'dispatch: loop {
        match pc {
            0x8264E0D8 => {
    //   block [0x8264E0D8..0x8264E148)
	// 8264E0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E0E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E0E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E0EC: 38AA3070  addi r5, r10, 0x3070
	ctx.r[5].s64 = ctx.r[10].s64 + 12400;
	// 8264E0F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E0F4: 390B4640  addi r8, r11, 0x4640
	ctx.r[8].s64 = ctx.r[11].s64 + 17984;
	// 8264E0F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264E0FC: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8264E100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E104: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E10C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E110: 386A32E0  addi r3, r10, 0x32e0
	ctx.r[3].s64 = ctx.r[10].s64 + 13024;
	// 8264E114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E11C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E134: 4BE18CED  bl 0x82466e20
	ctx.lr = 0x8264E138;
	sub_82466E20(ctx, base);
	// 8264E138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E13C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E148 size=112
    let mut pc: u32 = 0x8264E148;
    'dispatch: loop {
        match pc {
            0x8264E148 => {
    //   block [0x8264E148..0x8264E1B8)
	// 8264E148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E154: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E158: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E15C: 38AA31C0  addi r5, r10, 0x31c0
	ctx.r[5].s64 = ctx.r[10].s64 + 12736;
	// 8264E160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E164: 390B4658  addi r8, r11, 0x4658
	ctx.r[8].s64 = ctx.r[11].s64 + 18008;
	// 8264E168: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264E16C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8264E170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E17C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E180: 386A3310  addi r3, r10, 0x3310
	ctx.r[3].s64 = ctx.r[10].s64 + 13072;
	// 8264E184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E18C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E1A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E1A4: 4BE18C7D  bl 0x82466e20
	ctx.lr = 0x8264E1A8;
	sub_82466E20(ctx, base);
	// 8264E1A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E1AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E1B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E1B8 size=112
    let mut pc: u32 = 0x8264E1B8;
    'dispatch: loop {
        match pc {
            0x8264E1B8 => {
    //   block [0x8264E1B8..0x8264E228)
	// 8264E1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E1C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E1C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E1CC: 38AA2B60  addi r5, r10, 0x2b60
	ctx.r[5].s64 = ctx.r[10].s64 + 11104;
	// 8264E1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E1D4: 390B4688  addi r8, r11, 0x4688
	ctx.r[8].s64 = ctx.r[11].s64 + 18056;
	// 8264E1D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264E1DC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8264E1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E1E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E1E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E1F0: 386A3340  addi r3, r10, 0x3340
	ctx.r[3].s64 = ctx.r[10].s64 + 13120;
	// 8264E1F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E1F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E214: 4BE18C0D  bl 0x82466e20
	ctx.lr = 0x8264E218;
	sub_82466E20(ctx, base);
	// 8264E218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E228 size=112
    let mut pc: u32 = 0x8264E228;
    'dispatch: loop {
        match pc {
            0x8264E228 => {
    //   block [0x8264E228..0x8264E298)
	// 8264E228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E234: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E238: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E23C: 38AA2D10  addi r5, r10, 0x2d10
	ctx.r[5].s64 = ctx.r[10].s64 + 11536;
	// 8264E240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E244: 390B46D0  addi r8, r11, 0x46d0
	ctx.r[8].s64 = ctx.r[11].s64 + 18128;
	// 8264E248: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264E24C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8264E250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E254: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E260: 386A3370  addi r3, r10, 0x3370
	ctx.r[3].s64 = ctx.r[10].s64 + 13168;
	// 8264E264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E284: 4BE18B9D  bl 0x82466e20
	ctx.lr = 0x8264E288;
	sub_82466E20(ctx, base);
	// 8264E288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E28C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E298 size=112
    let mut pc: u32 = 0x8264E298;
    'dispatch: loop {
        match pc {
            0x8264E298 => {
    //   block [0x8264E298..0x8264E308)
	// 8264E298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E2A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E2A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E2AC: 38AA2C80  addi r5, r10, 0x2c80
	ctx.r[5].s64 = ctx.r[10].s64 + 11392;
	// 8264E2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E2B4: 390B4718  addi r8, r11, 0x4718
	ctx.r[8].s64 = ctx.r[11].s64 + 18200;
	// 8264E2B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264E2BC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8264E2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E2C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E2D0: 386A33A0  addi r3, r10, 0x33a0
	ctx.r[3].s64 = ctx.r[10].s64 + 13216;
	// 8264E2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E2F4: 4BE18B2D  bl 0x82466e20
	ctx.lr = 0x8264E2F8;
	sub_82466E20(ctx, base);
	// 8264E2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E308 size=112
    let mut pc: u32 = 0x8264E308;
    'dispatch: loop {
        match pc {
            0x8264E308 => {
    //   block [0x8264E308..0x8264E378)
	// 8264E308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E314: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E318: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E31C: 38AA2CE0  addi r5, r10, 0x2ce0
	ctx.r[5].s64 = ctx.r[10].s64 + 11488;
	// 8264E320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E324: 390B4730  addi r8, r11, 0x4730
	ctx.r[8].s64 = ctx.r[11].s64 + 18224;
	// 8264E328: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264E32C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8264E330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E334: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E340: 386A33D0  addi r3, r10, 0x33d0
	ctx.r[3].s64 = ctx.r[10].s64 + 13264;
	// 8264E344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E34C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E35C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E364: 4BE18ABD  bl 0x82466e20
	ctx.lr = 0x8264E368;
	sub_82466E20(ctx, base);
	// 8264E368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264E378 size=24
    let mut pc: u32 = 0x8264E378;
    'dispatch: loop {
        match pc {
            0x8264E378 => {
    //   block [0x8264E378..0x8264E390)
	// 8264E378: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E37C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8264E380: 394A8248  addi r10, r10, -0x7db8
	ctx.r[10].s64 = ctx.r[10].s64 + -32184;
	// 8264E384: 816B4760  lwz r11, 0x4760(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18272 as u32) ) } as u64;
	// 8264E388: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8264E38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E390 size=112
    let mut pc: u32 = 0x8264E390;
    'dispatch: loop {
        match pc {
            0x8264E390 => {
    //   block [0x8264E390..0x8264E400)
	// 8264E390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E39C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264E3A0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8264E3A4: 392AA110  addi r9, r10, -0x5ef0
	ctx.r[9].s64 = ctx.r[10].s64 + -24304;
	// 8264E3A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E3AC: 390B8248  addi r8, r11, -0x7db8
	ctx.r[8].s64 = ctx.r[11].s64 + -32184;
	// 8264E3B0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8264E3B4: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 8264E3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E3BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E3C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E3C8: 386A3400  addi r3, r10, 0x3400
	ctx.r[3].s64 = ctx.r[10].s64 + 13312;
	// 8264E3CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264E3D0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8264E3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E3D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E3E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264E3E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E3EC: 4BE18A35  bl 0x82466e20
	ctx.lr = 0x8264E3F0;
	sub_82466E20(ctx, base);
	// 8264E3F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E400 size=112
    let mut pc: u32 = 0x8264E400;
    'dispatch: loop {
        match pc {
            0x8264E400 => {
    //   block [0x8264E400..0x8264E470)
	// 8264E400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E40C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E410: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E414: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E41C: 390B4768  addi r8, r11, 0x4768
	ctx.r[8].s64 = ctx.r[11].s64 + 18280;
	// 8264E420: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264E424: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 8264E428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E42C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E438: 386A3430  addi r3, r10, 0x3430
	ctx.r[3].s64 = ctx.r[10].s64 + 13360;
	// 8264E43C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E45C: 4BE189C5  bl 0x82466e20
	ctx.lr = 0x8264E460;
	sub_82466E20(ctx, base);
	// 8264E460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E470 size=108
    let mut pc: u32 = 0x8264E470;
    'dispatch: loop {
        match pc {
            0x8264E470 => {
    //   block [0x8264E470..0x8264E4DC)
	// 8264E470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E47C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E484: 38EB4798  addi r7, r11, 0x4798
	ctx.r[7].s64 = ctx.r[11].s64 + 18328;
	// 8264E488: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264E48C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 8264E490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E494: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264E49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E4A0: 386A3460  addi r3, r10, 0x3460
	ctx.r[3].s64 = ctx.r[10].s64 + 13408;
	// 8264E4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264E4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264E4C8: 4BE18959  bl 0x82466e20
	ctx.lr = 0x8264E4CC;
	sub_82466E20(ctx, base);
	// 8264E4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E4E0 size=100
    let mut pc: u32 = 0x8264E4E0;
    'dispatch: loop {
        match pc {
            0x8264E4E0 => {
    //   block [0x8264E4E0..0x8264E544)
	// 8264E4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E4EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E4F4: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E500: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 8264E504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E514: 386A3490  addi r3, r10, 0x3490
	ctx.r[3].s64 = ctx.r[10].s64 + 13456;
	// 8264E518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E51C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E520: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264E524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E528: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264E52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E530: 4BE188F1  bl 0x82466e20
	ctx.lr = 0x8264E534;
	sub_82466E20(ctx, base);
	// 8264E534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E548 size=112
    let mut pc: u32 = 0x8264E548;
    'dispatch: loop {
        match pc {
            0x8264E548 => {
    //   block [0x8264E548..0x8264E5B8)
	// 8264E548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E558: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E55C: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E564: 390B47B0  addi r8, r11, 0x47b0
	ctx.r[8].s64 = ctx.r[11].s64 + 18352;
	// 8264E568: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264E56C: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 8264E570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E578: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E580: 386A34C0  addi r3, r10, 0x34c0
	ctx.r[3].s64 = ctx.r[10].s64 + 13504;
	// 8264E584: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E5A4: 4BE1887D  bl 0x82466e20
	ctx.lr = 0x8264E5A8;
	sub_82466E20(ctx, base);
	// 8264E5A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E5B8 size=112
    let mut pc: u32 = 0x8264E5B8;
    'dispatch: loop {
        match pc {
            0x8264E5B8 => {
    //   block [0x8264E5B8..0x8264E628)
	// 8264E5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E5C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E5C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E5CC: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E5D4: 390B47C8  addi r8, r11, 0x47c8
	ctx.r[8].s64 = ctx.r[11].s64 + 18376;
	// 8264E5D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264E5DC: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 8264E5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E5E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E5E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E5F0: 386A34F0  addi r3, r10, 0x34f0
	ctx.r[3].s64 = ctx.r[10].s64 + 13552;
	// 8264E5F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E5FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E614: 4BE1880D  bl 0x82466e20
	ctx.lr = 0x8264E618;
	sub_82466E20(ctx, base);
	// 8264E618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E628 size=112
    let mut pc: u32 = 0x8264E628;
    'dispatch: loop {
        match pc {
            0x8264E628 => {
    //   block [0x8264E628..0x8264E698)
	// 8264E628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E634: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E638: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E63C: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E644: 390B47F8  addi r8, r11, 0x47f8
	ctx.r[8].s64 = ctx.r[11].s64 + 18424;
	// 8264E648: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264E64C: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 8264E650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E654: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E660: 386A3520  addi r3, r10, 0x3520
	ctx.r[3].s64 = ctx.r[10].s64 + 13600;
	// 8264E664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E684: 4BE1879D  bl 0x82466e20
	ctx.lr = 0x8264E688;
	sub_82466E20(ctx, base);
	// 8264E688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E698 size=112
    let mut pc: u32 = 0x8264E698;
    'dispatch: loop {
        match pc {
            0x8264E698 => {
    //   block [0x8264E698..0x8264E708)
	// 8264E698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E6A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E6A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E6AC: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E6B4: 390B4828  addi r8, r11, 0x4828
	ctx.r[8].s64 = ctx.r[11].s64 + 18472;
	// 8264E6B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264E6BC: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 8264E6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E6C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E6D0: 386A3550  addi r3, r10, 0x3550
	ctx.r[3].s64 = ctx.r[10].s64 + 13648;
	// 8264E6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E6F4: 4BE1872D  bl 0x82466e20
	ctx.lr = 0x8264E6F8;
	sub_82466E20(ctx, base);
	// 8264E6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E708 size=112
    let mut pc: u32 = 0x8264E708;
    'dispatch: loop {
        match pc {
            0x8264E708 => {
    //   block [0x8264E708..0x8264E778)
	// 8264E708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E714: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E718: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E71C: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E724: 390B4858  addi r8, r11, 0x4858
	ctx.r[8].s64 = ctx.r[11].s64 + 18520;
	// 8264E728: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264E72C: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 8264E730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E734: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E740: 386A3580  addi r3, r10, 0x3580
	ctx.r[3].s64 = ctx.r[10].s64 + 13696;
	// 8264E744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E764: 4BE186BD  bl 0x82466e20
	ctx.lr = 0x8264E768;
	sub_82466E20(ctx, base);
	// 8264E768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E778 size=112
    let mut pc: u32 = 0x8264E778;
    'dispatch: loop {
        match pc {
            0x8264E778 => {
    //   block [0x8264E778..0x8264E7E8)
	// 8264E778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E784: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E788: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E78C: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E794: 390B4870  addi r8, r11, 0x4870
	ctx.r[8].s64 = ctx.r[11].s64 + 18544;
	// 8264E798: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264E79C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 8264E7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E7A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E7B0: 386A35B0  addi r3, r10, 0x35b0
	ctx.r[3].s64 = ctx.r[10].s64 + 13744;
	// 8264E7B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E7D4: 4BE1864D  bl 0x82466e20
	ctx.lr = 0x8264E7D8;
	sub_82466E20(ctx, base);
	// 8264E7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E7E8 size=112
    let mut pc: u32 = 0x8264E7E8;
    'dispatch: loop {
        match pc {
            0x8264E7E8 => {
    //   block [0x8264E7E8..0x8264E858)
	// 8264E7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E7F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E7F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E7FC: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E804: 390B4888  addi r8, r11, 0x4888
	ctx.r[8].s64 = ctx.r[11].s64 + 18568;
	// 8264E808: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264E80C: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 8264E810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E820: 386A35E0  addi r3, r10, 0x35e0
	ctx.r[3].s64 = ctx.r[10].s64 + 13792;
	// 8264E824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E82C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E844: 4BE185DD  bl 0x82466e20
	ctx.lr = 0x8264E848;
	sub_82466E20(ctx, base);
	// 8264E848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E858 size=112
    let mut pc: u32 = 0x8264E858;
    'dispatch: loop {
        match pc {
            0x8264E858 => {
    //   block [0x8264E858..0x8264E8C8)
	// 8264E858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E868: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E86C: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E874: 390B48D0  addi r8, r11, 0x48d0
	ctx.r[8].s64 = ctx.r[11].s64 + 18640;
	// 8264E878: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264E87C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 8264E880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E890: 386A3610  addi r3, r10, 0x3610
	ctx.r[3].s64 = ctx.r[10].s64 + 13840;
	// 8264E894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E8A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E8AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E8B4: 4BE1856D  bl 0x82466e20
	ctx.lr = 0x8264E8B8;
	sub_82466E20(ctx, base);
	// 8264E8B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E8C8 size=112
    let mut pc: u32 = 0x8264E8C8;
    'dispatch: loop {
        match pc {
            0x8264E8C8 => {
    //   block [0x8264E8C8..0x8264E938)
	// 8264E8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E8D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E8D8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E8DC: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E8E4: 390B4918  addi r8, r11, 0x4918
	ctx.r[8].s64 = ctx.r[11].s64 + 18712;
	// 8264E8E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264E8EC: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 8264E8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E8F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E8F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E900: 386A3640  addi r3, r10, 0x3640
	ctx.r[3].s64 = ctx.r[10].s64 + 13888;
	// 8264E904: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E924: 4BE184FD  bl 0x82466e20
	ctx.lr = 0x8264E928;
	sub_82466E20(ctx, base);
	// 8264E928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E938 size=112
    let mut pc: u32 = 0x8264E938;
    'dispatch: loop {
        match pc {
            0x8264E938 => {
    //   block [0x8264E938..0x8264E9A8)
	// 8264E938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E944: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E948: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264E94C: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E954: 390B4930  addi r8, r11, 0x4930
	ctx.r[8].s64 = ctx.r[11].s64 + 18736;
	// 8264E958: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264E95C: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 8264E960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264E964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264E970: 386A3670  addi r3, r10, 0x3670
	ctx.r[3].s64 = ctx.r[10].s64 + 13936;
	// 8264E974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264E978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264E984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264E98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264E990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264E994: 4BE1848D  bl 0x82466e20
	ctx.lr = 0x8264E998;
	sub_82466E20(ctx, base);
	// 8264E998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264E99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264E9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264E9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264E9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264E9A8 size=116
    let mut pc: u32 = 0x8264E9A8;
    'dispatch: loop {
        match pc {
            0x8264E9A8 => {
    //   block [0x8264E9A8..0x8264EA1C)
	// 8264E9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264E9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264E9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264E9B4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264E9B8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8264E9BC: 390A4960  addi r8, r10, 0x4960
	ctx.r[8].s64 = ctx.r[10].s64 + 18784;
	// 8264E9C0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E9C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264E9C8: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264E9CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264E9D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264E9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264E9D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264E9DC: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 8264E9E0: 396BA138  addi r11, r11, -0x5ec8
	ctx.r[11].s64 = ctx.r[11].s64 + -24264;
	// 8264E9E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264E9E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264E9EC: 386A36A0  addi r3, r10, 0x36a0
	ctx.r[3].s64 = ctx.r[10].s64 + 13984;
	// 8264E9F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264E9F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264E9F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264E9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EA08: 4BE18419  bl 0x82466e20
	ctx.lr = 0x8264EA0C;
	sub_82466E20(ctx, base);
	// 8264EA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EA20 size=116
    let mut pc: u32 = 0x8264EA20;
    'dispatch: loop {
        match pc {
            0x8264EA20 => {
    //   block [0x8264EA20..0x8264EA94)
	// 8264EA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EA2C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264EA30: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8264EA34: 390A49D8  addi r8, r10, 0x49d8
	ctx.r[8].s64 = ctx.r[10].s64 + 18904;
	// 8264EA38: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EA3C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264EA40: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264EA44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EA48: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264EA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264EA50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264EA54: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 8264EA58: 396BA150  addi r11, r11, -0x5eb0
	ctx.r[11].s64 = ctx.r[11].s64 + -24240;
	// 8264EA5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EA60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EA64: 386A36D0  addi r3, r10, 0x36d0
	ctx.r[3].s64 = ctx.r[10].s64 + 14032;
	// 8264EA68: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264EA6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264EA70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264EA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EA80: 4BE183A1  bl 0x82466e20
	ctx.lr = 0x8264EA84;
	sub_82466E20(ctx, base);
	// 8264EA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264EA98 size=24
    let mut pc: u32 = 0x8264EA98;
    'dispatch: loop {
        match pc {
            0x8264EA98 => {
    //   block [0x8264EA98..0x8264EAB0)
	// 8264EA98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264EA9C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8264EAA0: 394A8260  addi r10, r10, -0x7da0
	ctx.r[10].s64 = ctx.r[10].s64 + -32160;
	// 8264EAA4: 816B4A68  lwz r11, 0x4a68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19048 as u32) ) } as u64;
	// 8264EAA8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8264EAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EAB0 size=116
    let mut pc: u32 = 0x8264EAB0;
    'dispatch: loop {
        match pc {
            0x8264EAB0 => {
    //   block [0x8264EAB0..0x8264EB24)
	// 8264EAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EABC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264EAC0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EAC4: 392BA17C  addi r9, r11, -0x5e84
	ctx.r[9].s64 = ctx.r[11].s64 + -24196;
	// 8264EAC8: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264EACC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EAD0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8264EAD4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8264EAD8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8264EADC: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 8264EAE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EAE4: 396B8260  addi r11, r11, -0x7da0
	ctx.r[11].s64 = ctx.r[11].s64 + -32160;
	// 8264EAE8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8264EAEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EAF0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264EAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EAF8: 386A3700  addi r3, r10, 0x3700
	ctx.r[3].s64 = ctx.r[10].s64 + 14080;
	// 8264EAFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264EB00: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264EB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EB08: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264EB0C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264EB10: 4BE18311  bl 0x82466e20
	ctx.lr = 0x8264EB14;
	sub_82466E20(ctx, base);
	// 8264EB14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EB28 size=112
    let mut pc: u32 = 0x8264EB28;
    'dispatch: loop {
        match pc {
            0x8264EB28 => {
    //   block [0x8264EB28..0x8264EB98)
	// 8264EB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EB34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EB38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264EB3C: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264EB40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EB44: 390B4A70  addi r8, r11, 0x4a70
	ctx.r[8].s64 = ctx.r[11].s64 + 19056;
	// 8264EB48: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264EB4C: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 8264EB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264EB54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EB58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264EB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EB60: 386A3730  addi r3, r10, 0x3730
	ctx.r[3].s64 = ctx.r[10].s64 + 14128;
	// 8264EB64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264EB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264EB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264EB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264EB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264EB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EB84: 4BE1829D  bl 0x82466e20
	ctx.lr = 0x8264EB88;
	sub_82466E20(ctx, base);
	// 8264EB88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EB98 size=112
    let mut pc: u32 = 0x8264EB98;
    'dispatch: loop {
        match pc {
            0x8264EB98 => {
    //   block [0x8264EB98..0x8264EC08)
	// 8264EB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EBA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EBA8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264EBAC: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264EBB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EBB4: 390B4AD0  addi r8, r11, 0x4ad0
	ctx.r[8].s64 = ctx.r[11].s64 + 19152;
	// 8264EBB8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8264EBBC: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 8264EBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264EBC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EBC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264EBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EBD0: 386A3760  addi r3, r10, 0x3760
	ctx.r[3].s64 = ctx.r[10].s64 + 14176;
	// 8264EBD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264EBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264EBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264EBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264EBE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264EBEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EBF4: 4BE1822D  bl 0x82466e20
	ctx.lr = 0x8264EBF8;
	sub_82466E20(ctx, base);
	// 8264EBF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EC08 size=112
    let mut pc: u32 = 0x8264EC08;
    'dispatch: loop {
        match pc {
            0x8264EC08 => {
    //   block [0x8264EC08..0x8264EC78)
	// 8264EC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EC14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EC18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264EC1C: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264EC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EC24: 390B4B78  addi r8, r11, 0x4b78
	ctx.r[8].s64 = ctx.r[11].s64 + 19320;
	// 8264EC28: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8264EC2C: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 8264EC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264EC34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EC38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264EC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EC40: 386A3790  addi r3, r10, 0x3790
	ctx.r[3].s64 = ctx.r[10].s64 + 14224;
	// 8264EC44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264EC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264EC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264EC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264EC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264EC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EC64: 4BE181BD  bl 0x82466e20
	ctx.lr = 0x8264EC68;
	sub_82466E20(ctx, base);
	// 8264EC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EC78 size=112
    let mut pc: u32 = 0x8264EC78;
    'dispatch: loop {
        match pc {
            0x8264EC78 => {
    //   block [0x8264EC78..0x8264ECE8)
	// 8264EC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EC84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EC88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264EC8C: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264EC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EC94: 390B4BF0  addi r8, r11, 0x4bf0
	ctx.r[8].s64 = ctx.r[11].s64 + 19440;
	// 8264EC98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264EC9C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 8264ECA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264ECA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ECA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264ECAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264ECB0: 386A37C0  addi r3, r10, 0x37c0
	ctx.r[3].s64 = ctx.r[10].s64 + 14272;
	// 8264ECB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264ECB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264ECBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264ECC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264ECC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264ECC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264ECCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264ECD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264ECD4: 4BE1814D  bl 0x82466e20
	ctx.lr = 0x8264ECD8;
	sub_82466E20(ctx, base);
	// 8264ECD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264ECDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264ECE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264ECE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264ECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264ECE8 size=112
    let mut pc: u32 = 0x8264ECE8;
    'dispatch: loop {
        match pc {
            0x8264ECE8 => {
    //   block [0x8264ECE8..0x8264ED58)
	// 8264ECE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264ECEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264ECF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264ECF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ECF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264ECFC: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264ED00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264ED04: 390B4C38  addi r8, r11, 0x4c38
	ctx.r[8].s64 = ctx.r[11].s64 + 19512;
	// 8264ED08: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264ED0C: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 8264ED10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264ED14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ED18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264ED1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264ED20: 386A37F0  addi r3, r10, 0x37f0
	ctx.r[3].s64 = ctx.r[10].s64 + 14320;
	// 8264ED24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264ED28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264ED2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264ED30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264ED34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264ED38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264ED3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264ED40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264ED44: 4BE180DD  bl 0x82466e20
	ctx.lr = 0x8264ED48;
	sub_82466E20(ctx, base);
	// 8264ED48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264ED4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264ED50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264ED54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264ED58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264ED58 size=112
    let mut pc: u32 = 0x8264ED58;
    'dispatch: loop {
        match pc {
            0x8264ED58 => {
    //   block [0x8264ED58..0x8264EDC8)
	// 8264ED58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264ED5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264ED60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264ED64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ED68: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264ED6C: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264ED70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264ED74: 390B4CC8  addi r8, r11, 0x4cc8
	ctx.r[8].s64 = ctx.r[11].s64 + 19656;
	// 8264ED78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264ED7C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 8264ED80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264ED84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ED88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264ED8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264ED90: 386A3820  addi r3, r10, 0x3820
	ctx.r[3].s64 = ctx.r[10].s64 + 14368;
	// 8264ED94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264ED98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264ED9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264EDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264EDA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264EDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EDB4: 4BE1806D  bl 0x82466e20
	ctx.lr = 0x8264EDB8;
	sub_82466E20(ctx, base);
	// 8264EDB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EDC8 size=112
    let mut pc: u32 = 0x8264EDC8;
    'dispatch: loop {
        match pc {
            0x8264EDC8 => {
    //   block [0x8264EDC8..0x8264EE38)
	// 8264EDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EDD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EDD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EDD8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264EDDC: 38AA3400  addi r5, r10, 0x3400
	ctx.r[5].s64 = ctx.r[10].s64 + 13312;
	// 8264EDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EDE4: 390B4D28  addi r8, r11, 0x4d28
	ctx.r[8].s64 = ctx.r[11].s64 + 19752;
	// 8264EDE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264EDEC: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 8264EDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264EDF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EDF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264EDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EE00: 386A3850  addi r3, r10, 0x3850
	ctx.r[3].s64 = ctx.r[10].s64 + 14416;
	// 8264EE04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264EE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264EE0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264EE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264EE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264EE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EE24: 4BE17FFD  bl 0x82466e20
	ctx.lr = 0x8264EE28;
	sub_82466E20(ctx, base);
	// 8264EE28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EE2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EE30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EE34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EE38 size=112
    let mut pc: u32 = 0x8264EE38;
    'dispatch: loop {
        match pc {
            0x8264EE38 => {
    //   block [0x8264EE38..0x8264EEA8)
	// 8264EE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EE44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EE48: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264EE4C: 38AA3850  addi r5, r10, 0x3850
	ctx.r[5].s64 = ctx.r[10].s64 + 14416;
	// 8264EE50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EE54: 390B4D70  addi r8, r11, 0x4d70
	ctx.r[8].s64 = ctx.r[11].s64 + 19824;
	// 8264EE58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264EE5C: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 8264EE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264EE64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EE68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264EE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EE70: 386A3880  addi r3, r10, 0x3880
	ctx.r[3].s64 = ctx.r[10].s64 + 14464;
	// 8264EE74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264EE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264EE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264EE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264EE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264EE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EE94: 4BE17F8D  bl 0x82466e20
	ctx.lr = 0x8264EE98;
	sub_82466E20(ctx, base);
	// 8264EE98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EEA8 size=112
    let mut pc: u32 = 0x8264EEA8;
    'dispatch: loop {
        match pc {
            0x8264EEA8 => {
    //   block [0x8264EEA8..0x8264EF18)
	// 8264EEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EEB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EEB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264EEBC: 38AA3850  addi r5, r10, 0x3850
	ctx.r[5].s64 = ctx.r[10].s64 + 14416;
	// 8264EEC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EEC4: 390B4DA0  addi r8, r11, 0x4da0
	ctx.r[8].s64 = ctx.r[11].s64 + 19872;
	// 8264EEC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264EECC: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 8264EED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264EED4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264EEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EEE0: 386A38B0  addi r3, r10, 0x38b0
	ctx.r[3].s64 = ctx.r[10].s64 + 14512;
	// 8264EEE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264EEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264EEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264EEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264EEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264EEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EF04: 4BE17F1D  bl 0x82466e20
	ctx.lr = 0x8264EF08;
	sub_82466E20(ctx, base);
	// 8264EF08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EF18 size=100
    let mut pc: u32 = 0x8264EF18;
    'dispatch: loop {
        match pc {
            0x8264EF18 => {
    //   block [0x8264EF18..0x8264EF7C)
	// 8264EF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EF24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264EF2C: 38AA3850  addi r5, r10, 0x3850
	ctx.r[5].s64 = ctx.r[10].s64 + 14416;
	// 8264EF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264EF38: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 8264EF3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264EF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264EF4C: 386A38E0  addi r3, r10, 0x38e0
	ctx.r[3].s64 = ctx.r[10].s64 + 14560;
	// 8264EF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EF54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264EF58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264EF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EF60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264EF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EF68: 4BE17EB9  bl 0x82466e20
	ctx.lr = 0x8264EF6C;
	sub_82466E20(ctx, base);
	// 8264EF6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EF70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EF74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EF78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EF80 size=112
    let mut pc: u32 = 0x8264EF80;
    'dispatch: loop {
        match pc {
            0x8264EF80 => {
    //   block [0x8264EF80..0x8264EFF0)
	// 8264EF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EF8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EF90: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264EF94: 38AA3850  addi r5, r10, 0x3850
	ctx.r[5].s64 = ctx.r[10].s64 + 14416;
	// 8264EF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264EF9C: 390B4DD0  addi r8, r11, 0x4dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 19920;
	// 8264EFA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264EFA4: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 8264EFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264EFAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264EFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264EFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264EFB8: 386A3910  addi r3, r10, 0x3910
	ctx.r[3].s64 = ctx.r[10].s64 + 14608;
	// 8264EFBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264EFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264EFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264EFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264EFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264EFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264EFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264EFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264EFDC: 4BE17E45  bl 0x82466e20
	ctx.lr = 0x8264EFE0;
	sub_82466E20(ctx, base);
	// 8264EFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264EFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264EFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264EFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264EFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264EFF0 size=112
    let mut pc: u32 = 0x8264EFF0;
    'dispatch: loop {
        match pc {
            0x8264EFF0 => {
    //   block [0x8264EFF0..0x8264F060)
	// 8264EFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264EFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264EFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264EFFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F000: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F004: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264F008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F00C: 390B4DE8  addi r8, r11, 0x4de8
	ctx.r[8].s64 = ctx.r[11].s64 + 19944;
	// 8264F010: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8264F014: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8264F018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F01C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F028: 386A3940  addi r3, r10, 0x3940
	ctx.r[3].s64 = ctx.r[10].s64 + 14656;
	// 8264F02C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264F030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F04C: 4BE17DD5  bl 0x82466e20
	ctx.lr = 0x8264F050;
	sub_82466E20(ctx, base);
	// 8264F050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F060 size=112
    let mut pc: u32 = 0x8264F060;
    'dispatch: loop {
        match pc {
            0x8264F060 => {
    //   block [0x8264F060..0x8264F0D0)
	// 8264F060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F06C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F070: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F074: 38AA3940  addi r5, r10, 0x3940
	ctx.r[5].s64 = ctx.r[10].s64 + 14656;
	// 8264F078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F07C: 390B4E48  addi r8, r11, 0x4e48
	ctx.r[8].s64 = ctx.r[11].s64 + 20040;
	// 8264F080: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264F084: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8264F088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F08C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F098: 386A3970  addi r3, r10, 0x3970
	ctx.r[3].s64 = ctx.r[10].s64 + 14704;
	// 8264F09C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264F0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F0BC: 4BE17D65  bl 0x82466e20
	ctx.lr = 0x8264F0C0;
	sub_82466E20(ctx, base);
	// 8264F0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F0D0 size=112
    let mut pc: u32 = 0x8264F0D0;
    'dispatch: loop {
        match pc {
            0x8264F0D0 => {
    //   block [0x8264F0D0..0x8264F140)
	// 8264F0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F0DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F0E0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F0E4: 38AA3940  addi r5, r10, 0x3940
	ctx.r[5].s64 = ctx.r[10].s64 + 14656;
	// 8264F0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F0EC: 390B4E60  addi r8, r11, 0x4e60
	ctx.r[8].s64 = ctx.r[11].s64 + 20064;
	// 8264F0F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264F0F4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8264F0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F0FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F108: 386A39A0  addi r3, r10, 0x39a0
	ctx.r[3].s64 = ctx.r[10].s64 + 14752;
	// 8264F10C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264F110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F12C: 4BE17CF5  bl 0x82466e20
	ctx.lr = 0x8264F130;
	sub_82466E20(ctx, base);
	// 8264F130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F140 size=112
    let mut pc: u32 = 0x8264F140;
    'dispatch: loop {
        match pc {
            0x8264F140 => {
    //   block [0x8264F140..0x8264F1B0)
	// 8264F140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F14C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F150: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F154: 38AA3940  addi r5, r10, 0x3940
	ctx.r[5].s64 = ctx.r[10].s64 + 14656;
	// 8264F158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F15C: 390B4E90  addi r8, r11, 0x4e90
	ctx.r[8].s64 = ctx.r[11].s64 + 20112;
	// 8264F160: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264F164: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8264F168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F16C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F178: 386A39D0  addi r3, r10, 0x39d0
	ctx.r[3].s64 = ctx.r[10].s64 + 14800;
	// 8264F17C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264F180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F19C: 4BE17C85  bl 0x82466e20
	ctx.lr = 0x8264F1A0;
	sub_82466E20(ctx, base);
	// 8264F1A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264F1B0 size=24
    let mut pc: u32 = 0x8264F1B0;
    'dispatch: loop {
        match pc {
            0x8264F1B0 => {
    //   block [0x8264F1B0..0x8264F1C8)
	// 8264F1B0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F1B4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8264F1B8: 394A8308  addi r10, r10, -0x7cf8
	ctx.r[10].s64 = ctx.r[10].s64 + -31992;
	// 8264F1BC: 816B4A6C  lwz r11, 0x4a6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19052 as u32) ) } as u64;
	// 8264F1C0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8264F1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F1C8 size=112
    let mut pc: u32 = 0x8264F1C8;
    'dispatch: loop {
        match pc {
            0x8264F1C8 => {
    //   block [0x8264F1C8..0x8264F238)
	// 8264F1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F1D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264F1D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8264F1DC: 392AA1D8  addi r9, r10, -0x5e28
	ctx.r[9].s64 = ctx.r[10].s64 + -24104;
	// 8264F1E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F1E4: 390B8308  addi r8, r11, -0x7cf8
	ctx.r[8].s64 = ctx.r[11].s64 + -31992;
	// 8264F1E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8264F1EC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8264F1F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F1F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F1F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F200: 386A3A00  addi r3, r10, 0x3a00
	ctx.r[3].s64 = ctx.r[10].s64 + 14848;
	// 8264F204: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264F208: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264F20C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F21C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264F220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F224: 4BE17BFD  bl 0x82466e20
	ctx.lr = 0x8264F228;
	sub_82466E20(ctx, base);
	// 8264F228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F22C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F238 size=108
    let mut pc: u32 = 0x8264F238;
    'dispatch: loop {
        match pc {
            0x8264F238 => {
    //   block [0x8264F238..0x8264F2A4)
	// 8264F238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F23C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F244: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F24C: 38EB4EA8  addi r7, r11, 0x4ea8
	ctx.r[7].s64 = ctx.r[11].s64 + 20136;
	// 8264F250: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264F254: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8264F258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F25C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F260: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264F264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F268: 386A3A30  addi r3, r10, 0x3a30
	ctx.r[3].s64 = ctx.r[10].s64 + 14896;
	// 8264F26C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264F270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F28C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264F290: 4BE17B91  bl 0x82466e20
	ctx.lr = 0x8264F294;
	sub_82466E20(ctx, base);
	// 8264F294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F29C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F2A8 size=108
    let mut pc: u32 = 0x8264F2A8;
    'dispatch: loop {
        match pc {
            0x8264F2A8 => {
    //   block [0x8264F2A8..0x8264F314)
	// 8264F2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F2B4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F2B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F2BC: 38EB4EC0  addi r7, r11, 0x4ec0
	ctx.r[7].s64 = ctx.r[11].s64 + 20160;
	// 8264F2C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264F2C4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8264F2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F2CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F2D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264F2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F2D8: 386A3A60  addi r3, r10, 0x3a60
	ctx.r[3].s64 = ctx.r[10].s64 + 14944;
	// 8264F2DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264F2E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F2E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F2F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F2F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264F300: 4BE17B21  bl 0x82466e20
	ctx.lr = 0x8264F304;
	sub_82466E20(ctx, base);
	// 8264F304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F318 size=116
    let mut pc: u32 = 0x8264F318;
    'dispatch: loop {
        match pc {
            0x8264F318 => {
    //   block [0x8264F318..0x8264F38C)
	// 8264F318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F324: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F328: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264F32C: 390B4F0C  addi r8, r11, 0x4f0c
	ctx.r[8].s64 = ctx.r[11].s64 + 20236;
	// 8264F330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F334: 392AA290  addi r9, r10, -0x5d70
	ctx.r[9].s64 = ctx.r[10].s64 + -23920;
	// 8264F338: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F33C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8264F340: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264F344: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F34C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F35C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264F360: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8264F364: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264F368: 386B3A90  addi r3, r11, 0x3a90
	ctx.r[3].s64 = ctx.r[11].s64 + 14992;
	// 8264F36C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264F370: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F378: 4BE17AA9  bl 0x82466e20
	ctx.lr = 0x8264F37C;
	sub_82466E20(ctx, base);
	// 8264F37C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F390 size=108
    let mut pc: u32 = 0x8264F390;
    'dispatch: loop {
        match pc {
            0x8264F390 => {
    //   block [0x8264F390..0x8264F3FC)
	// 8264F390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F39C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F3A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264F3A4: 38EB4F28  addi r7, r11, 0x4f28
	ctx.r[7].s64 = ctx.r[11].s64 + 20264;
	// 8264F3A8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264F3AC: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 8264F3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F3B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F3B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264F3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F3C0: 386A3AC0  addi r3, r10, 0x3ac0
	ctx.r[3].s64 = ctx.r[10].s64 + 15040;
	// 8264F3C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264F3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264F3E8: 4BE17A39  bl 0x82466e20
	ctx.lr = 0x8264F3EC;
	sub_82466E20(ctx, base);
	// 8264F3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264F400 size=24
    let mut pc: u32 = 0x8264F400;
    'dispatch: loop {
        match pc {
            0x8264F400 => {
    //   block [0x8264F400..0x8264F418)
	// 8264F400: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F404: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8264F408: 394A8350  addi r10, r10, -0x7cb0
	ctx.r[10].s64 = ctx.r[10].s64 + -31920;
	// 8264F40C: 816B4F24  lwz r11, 0x4f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20260 as u32) ) } as u64;
	// 8264F410: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8264F414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F418 size=116
    let mut pc: u32 = 0x8264F418;
    'dispatch: loop {
        match pc {
            0x8264F418 => {
    //   block [0x8264F418..0x8264F48C)
	// 8264F418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F424: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8264F428: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264F42C: 390B8350  addi r8, r11, -0x7cb0
	ctx.r[8].s64 = ctx.r[11].s64 + -31920;
	// 8264F430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F434: 392AA2EC  addi r9, r10, -0x5d14
	ctx.r[9].s64 = ctx.r[10].s64 + -23828;
	// 8264F438: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F43C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8264F440: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264F444: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F44C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F45C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264F460: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8264F464: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264F468: 386B3AF0  addi r3, r11, 0x3af0
	ctx.r[3].s64 = ctx.r[11].s64 + 15088;
	// 8264F46C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8264F470: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F478: 4BE179A9  bl 0x82466e20
	ctx.lr = 0x8264F47C;
	sub_82466E20(ctx, base);
	// 8264F47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F490 size=108
    let mut pc: u32 = 0x8264F490;
    'dispatch: loop {
        match pc {
            0x8264F490 => {
    //   block [0x8264F490..0x8264F4FC)
	// 8264F490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F49C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F4A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F4A4: 38EB4F90  addi r7, r11, 0x4f90
	ctx.r[7].s64 = ctx.r[11].s64 + 20368;
	// 8264F4A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264F4AC: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 8264F4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F4B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F4B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264F4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F4C0: 386A3B20  addi r3, r10, 0x3b20
	ctx.r[3].s64 = ctx.r[10].s64 + 15136;
	// 8264F4C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264F4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F4E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264F4E8: 4BE17939  bl 0x82466e20
	ctx.lr = 0x8264F4EC;
	sub_82466E20(ctx, base);
	// 8264F4EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F500 size=112
    let mut pc: u32 = 0x8264F500;
    'dispatch: loop {
        match pc {
            0x8264F500 => {
    //   block [0x8264F500..0x8264F570)
	// 8264F500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F50C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F510: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F514: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264F518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F51C: 390B4FC0  addi r8, r11, 0x4fc0
	ctx.r[8].s64 = ctx.r[11].s64 + 20416;
	// 8264F520: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264F524: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 8264F528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F52C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F538: 386A3B50  addi r3, r10, 0x3b50
	ctx.r[3].s64 = ctx.r[10].s64 + 15184;
	// 8264F53C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264F540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F55C: 4BE178C5  bl 0x82466e20
	ctx.lr = 0x8264F560;
	sub_82466E20(ctx, base);
	// 8264F560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F570 size=112
    let mut pc: u32 = 0x8264F570;
    'dispatch: loop {
        match pc {
            0x8264F570 => {
    //   block [0x8264F570..0x8264F5E0)
	// 8264F570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F57C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264F580: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F584: 392AA330  addi r9, r10, -0x5cd0
	ctx.r[9].s64 = ctx.r[10].s64 + -23760;
	// 8264F588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F58C: 390B4FE0  addi r8, r11, 0x4fe0
	ctx.r[8].s64 = ctx.r[11].s64 + 20448;
	// 8264F590: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8264F594: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 8264F598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F59C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F5A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F5A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F5A8: 386A3B80  addi r3, r10, 0x3b80
	ctx.r[3].s64 = ctx.r[10].s64 + 15232;
	// 8264F5AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264F5B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264F5B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F5B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F5BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F5C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F5C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264F5C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F5CC: 4BE17855  bl 0x82466e20
	ctx.lr = 0x8264F5D0;
	sub_82466E20(ctx, base);
	// 8264F5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F5E0 size=112
    let mut pc: u32 = 0x8264F5E0;
    'dispatch: loop {
        match pc {
            0x8264F5E0 => {
    //   block [0x8264F5E0..0x8264F650)
	// 8264F5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F5EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F5F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F5F4: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264F5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F5FC: 390B5028  addi r8, r11, 0x5028
	ctx.r[8].s64 = ctx.r[11].s64 + 20520;
	// 8264F600: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264F604: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 8264F608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F60C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F618: 386A3BB0  addi r3, r10, 0x3bb0
	ctx.r[3].s64 = ctx.r[10].s64 + 15280;
	// 8264F61C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264F620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F63C: 4BE177E5  bl 0x82466e20
	ctx.lr = 0x8264F640;
	sub_82466E20(ctx, base);
	// 8264F640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F650 size=112
    let mut pc: u32 = 0x8264F650;
    'dispatch: loop {
        match pc {
            0x8264F650 => {
    //   block [0x8264F650..0x8264F6C0)
	// 8264F650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F65C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264F660: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F664: 392AA35C  addi r9, r10, -0x5ca4
	ctx.r[9].s64 = ctx.r[10].s64 + -23716;
	// 8264F668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F66C: 390B5040  addi r8, r11, 0x5040
	ctx.r[8].s64 = ctx.r[11].s64 + 20544;
	// 8264F670: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8264F674: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 8264F678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F67C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F688: 386A3BE0  addi r3, r10, 0x3be0
	ctx.r[3].s64 = ctx.r[10].s64 + 15328;
	// 8264F68C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264F690: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264F694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F69C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264F6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F6AC: 4BE17775  bl 0x82466e20
	ctx.lr = 0x8264F6B0;
	sub_82466E20(ctx, base);
	// 8264F6B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F6B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F6B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F6C0 size=112
    let mut pc: u32 = 0x8264F6C0;
    'dispatch: loop {
        match pc {
            0x8264F6C0 => {
    //   block [0x8264F6C0..0x8264F730)
	// 8264F6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F6CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F6D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F6D4: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264F6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F6DC: 390B50D0  addi r8, r11, 0x50d0
	ctx.r[8].s64 = ctx.r[11].s64 + 20688;
	// 8264F6E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264F6E4: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 8264F6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F6EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F6F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F6F8: 386A3C10  addi r3, r10, 0x3c10
	ctx.r[3].s64 = ctx.r[10].s64 + 15376;
	// 8264F6FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264F700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F71C: 4BE17705  bl 0x82466e20
	ctx.lr = 0x8264F720;
	sub_82466E20(ctx, base);
	// 8264F720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F730 size=112
    let mut pc: u32 = 0x8264F730;
    'dispatch: loop {
        match pc {
            0x8264F730 => {
    //   block [0x8264F730..0x8264F7A0)
	// 8264F730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F73C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F740: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F744: 38AA3C70  addi r5, r10, 0x3c70
	ctx.r[5].s64 = ctx.r[10].s64 + 15472;
	// 8264F748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F74C: 390B50E8  addi r8, r11, 0x50e8
	ctx.r[8].s64 = ctx.r[11].s64 + 20712;
	// 8264F750: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8264F754: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 8264F758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F75C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F768: 386A3C40  addi r3, r10, 0x3c40
	ctx.r[3].s64 = ctx.r[10].s64 + 15424;
	// 8264F76C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264F770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F78C: 4BE17695  bl 0x82466e20
	ctx.lr = 0x8264F790;
	sub_82466E20(ctx, base);
	// 8264F790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F7A0 size=100
    let mut pc: u32 = 0x8264F7A0;
    'dispatch: loop {
        match pc {
            0x8264F7A0 => {
    //   block [0x8264F7A0..0x8264F804)
	// 8264F7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F7AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F7B4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264F7B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F7C0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8264F7C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F7D4: 386A3C70  addi r3, r10, 0x3c70
	ctx.r[3].s64 = ctx.r[10].s64 + 15472;
	// 8264F7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F7DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F7E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264F7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F7E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264F7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F7F0: 4BE17631  bl 0x82466e20
	ctx.lr = 0x8264F7F4;
	sub_82466E20(ctx, base);
	// 8264F7F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264F808 size=24
    let mut pc: u32 = 0x8264F808;
    'dispatch: loop {
        match pc {
            0x8264F808 => {
    //   block [0x8264F808..0x8264F820)
	// 8264F808: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F80C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8264F810: 394A8428  addi r10, r10, -0x7bd8
	ctx.r[10].s64 = ctx.r[10].s64 + -31704;
	// 8264F814: 816B5160  lwz r11, 0x5160(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20832 as u32) ) } as u64;
	// 8264F818: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8264F81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F820 size=116
    let mut pc: u32 = 0x8264F820;
    'dispatch: loop {
        match pc {
            0x8264F820 => {
    //   block [0x8264F820..0x8264F894)
	// 8264F820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F82C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8264F830: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264F834: 390B8428  addi r8, r11, -0x7bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -31704;
	// 8264F838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F83C: 392AA398  addi r9, r10, -0x5c68
	ctx.r[9].s64 = ctx.r[10].s64 + -23656;
	// 8264F840: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F844: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8264F848: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264F84C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F854: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F85C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F864: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264F868: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8264F86C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264F870: 386B3CA0  addi r3, r11, 0x3ca0
	ctx.r[3].s64 = ctx.r[11].s64 + 15520;
	// 8264F874: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264F878: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F87C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F880: 4BE175A1  bl 0x82466e20
	ctx.lr = 0x8264F884;
	sub_82466E20(ctx, base);
	// 8264F884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F88C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F898 size=108
    let mut pc: u32 = 0x8264F898;
    'dispatch: loop {
        match pc {
            0x8264F898 => {
    //   block [0x8264F898..0x8264F904)
	// 8264F898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F8A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F8A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F8AC: 38EB5164  addi r7, r11, 0x5164
	ctx.r[7].s64 = ctx.r[11].s64 + 20836;
	// 8264F8B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264F8B4: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 8264F8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F8BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F8C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264F8C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F8C8: 386A3CD0  addi r3, r10, 0x3cd0
	ctx.r[3].s64 = ctx.r[10].s64 + 15568;
	// 8264F8CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264F8D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F8D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F8E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F8EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264F8F0: 4BE17531  bl 0x82466e20
	ctx.lr = 0x8264F8F4;
	sub_82466E20(ctx, base);
	// 8264F8F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F8F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F8FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F908 size=112
    let mut pc: u32 = 0x8264F908;
    'dispatch: loop {
        match pc {
            0x8264F908 => {
    //   block [0x8264F908..0x8264F978)
	// 8264F908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F914: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F918: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F91C: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264F920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F924: 390B5194  addi r8, r11, 0x5194
	ctx.r[8].s64 = ctx.r[11].s64 + 20884;
	// 8264F928: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264F92C: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 8264F930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F934: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F940: 386A3D00  addi r3, r10, 0x3d00
	ctx.r[3].s64 = ctx.r[10].s64 + 15616;
	// 8264F944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264F948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264F94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264F950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F964: 4BE174BD  bl 0x82466e20
	ctx.lr = 0x8264F968;
	sub_82466E20(ctx, base);
	// 8264F968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F978 size=112
    let mut pc: u32 = 0x8264F978;
    'dispatch: loop {
        match pc {
            0x8264F978 => {
    //   block [0x8264F978..0x8264F9E8)
	// 8264F978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F984: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264F988: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F98C: 392AA3BC  addi r9, r10, -0x5c44
	ctx.r[9].s64 = ctx.r[10].s64 + -23620;
	// 8264F990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264F994: 390B51B0  addi r8, r11, 0x51b0
	ctx.r[8].s64 = ctx.r[11].s64 + 20912;
	// 8264F998: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8264F99C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 8264F9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264F9A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F9A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264F9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264F9B0: 386A3D30  addi r3, r10, 0x3d30
	ctx.r[3].s64 = ctx.r[10].s64 + 15664;
	// 8264F9B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264F9B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264F9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264F9C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264F9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264F9C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264F9CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264F9D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264F9D4: 4BE1744D  bl 0x82466e20
	ctx.lr = 0x8264F9D8;
	sub_82466E20(ctx, base);
	// 8264F9D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264F9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264F9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264F9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264F9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264F9E8 size=112
    let mut pc: u32 = 0x8264F9E8;
    'dispatch: loop {
        match pc {
            0x8264F9E8 => {
    //   block [0x8264F9E8..0x8264FA58)
	// 8264F9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264F9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264F9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264F9F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264F9F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264F9FC: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264FA00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FA04: 390B5258  addi r8, r11, 0x5258
	ctx.r[8].s64 = ctx.r[11].s64 + 21080;
	// 8264FA08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264FA0C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 8264FA10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FA14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FA18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264FA1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FA20: 386A3D60  addi r3, r10, 0x3d60
	ctx.r[3].s64 = ctx.r[10].s64 + 15712;
	// 8264FA24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264FA28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FA30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FA34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FA38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FA3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FA40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FA44: 4BE173DD  bl 0x82466e20
	ctx.lr = 0x8264FA48;
	sub_82466E20(ctx, base);
	// 8264FA48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FA58 size=108
    let mut pc: u32 = 0x8264FA58;
    'dispatch: loop {
        match pc {
            0x8264FA58 => {
    //   block [0x8264FA58..0x8264FAC4)
	// 8264FA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FA60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FA64: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FA68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FA6C: 38EB5270  addi r7, r11, 0x5270
	ctx.r[7].s64 = ctx.r[11].s64 + 21104;
	// 8264FA70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264FA74: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 8264FA78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FA7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FA80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264FA84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FA88: 386A3D90  addi r3, r10, 0x3d90
	ctx.r[3].s64 = ctx.r[10].s64 + 15760;
	// 8264FA8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264FA90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FA94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FA98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FAA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FAA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FAA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FAAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264FAB0: 4BE17371  bl 0x82466e20
	ctx.lr = 0x8264FAB4;
	sub_82466E20(ctx, base);
	// 8264FAB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FAB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FAC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FAC8 size=112
    let mut pc: u32 = 0x8264FAC8;
    'dispatch: loop {
        match pc {
            0x8264FAC8 => {
    //   block [0x8264FAC8..0x8264FB38)
	// 8264FAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FAD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FAD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FAD8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FADC: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264FAE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FAE4: 390B52A0  addi r8, r11, 0x52a0
	ctx.r[8].s64 = ctx.r[11].s64 + 21152;
	// 8264FAE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264FAEC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 8264FAF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FAF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FAF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264FAFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FB00: 386A3DC0  addi r3, r10, 0x3dc0
	ctx.r[3].s64 = ctx.r[10].s64 + 15808;
	// 8264FB04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264FB08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FB10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FB18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FB1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FB20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FB24: 4BE172FD  bl 0x82466e20
	ctx.lr = 0x8264FB28;
	sub_82466E20(ctx, base);
	// 8264FB28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FB38 size=112
    let mut pc: u32 = 0x8264FB38;
    'dispatch: loop {
        match pc {
            0x8264FB38 => {
    //   block [0x8264FB38..0x8264FBA8)
	// 8264FB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FB40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FB44: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264FB48: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FB4C: 392AA3F0  addi r9, r10, -0x5c10
	ctx.r[9].s64 = ctx.r[10].s64 + -23568;
	// 8264FB50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FB54: 390B52C0  addi r8, r11, 0x52c0
	ctx.r[8].s64 = ctx.r[11].s64 + 21184;
	// 8264FB58: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8264FB5C: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 8264FB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FB64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FB68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264FB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FB70: 386A3DF0  addi r3, r10, 0x3df0
	ctx.r[3].s64 = ctx.r[10].s64 + 15856;
	// 8264FB74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264FB78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264FB7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FB80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FB88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FB8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264FB90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FB94: 4BE1728D  bl 0x82466e20
	ctx.lr = 0x8264FB98;
	sub_82466E20(ctx, base);
	// 8264FB98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FBA8 size=112
    let mut pc: u32 = 0x8264FBA8;
    'dispatch: loop {
        match pc {
            0x8264FBA8 => {
    //   block [0x8264FBA8..0x8264FC18)
	// 8264FBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FBB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FBB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FBB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FBBC: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264FBC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FBC4: 390B5368  addi r8, r11, 0x5368
	ctx.r[8].s64 = ctx.r[11].s64 + 21352;
	// 8264FBC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8264FBCC: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 8264FBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FBD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FBD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264FBDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FBE0: 386A3E20  addi r3, r10, 0x3e20
	ctx.r[3].s64 = ctx.r[10].s64 + 15904;
	// 8264FBE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264FBE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FBEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FBF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FBF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FBF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FBFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FC00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FC04: 4BE1721D  bl 0x82466e20
	ctx.lr = 0x8264FC08;
	sub_82466E20(ctx, base);
	// 8264FC08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FC0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FC10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FC18 size=112
    let mut pc: u32 = 0x8264FC18;
    'dispatch: loop {
        match pc {
            0x8264FC18 => {
    //   block [0x8264FC18..0x8264FC88)
	// 8264FC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FC20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FC24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FC28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FC2C: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264FC30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FC34: 390B53B0  addi r8, r11, 0x53b0
	ctx.r[8].s64 = ctx.r[11].s64 + 21424;
	// 8264FC38: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8264FC3C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 8264FC40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FC44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FC48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264FC4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FC50: 386A3E50  addi r3, r10, 0x3e50
	ctx.r[3].s64 = ctx.r[10].s64 + 15952;
	// 8264FC54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264FC58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FC5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FC60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FC64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FC68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FC6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FC70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FC74: 4BE171AD  bl 0x82466e20
	ctx.lr = 0x8264FC78;
	sub_82466E20(ctx, base);
	// 8264FC78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FC7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FC80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FC84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FC88 size=100
    let mut pc: u32 = 0x8264FC88;
    'dispatch: loop {
        match pc {
            0x8264FC88 => {
    //   block [0x8264FC88..0x8264FCEC)
	// 8264FC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FC90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FC94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FC9C: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264FCA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FCA8: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8264FCAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FCBC: 386A3E80  addi r3, r10, 0x3e80
	ctx.r[3].s64 = ctx.r[10].s64 + 16000;
	// 8264FCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FCC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FCC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264FCCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FCD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264FCD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FCD8: 4BE17149  bl 0x82466e20
	ctx.lr = 0x8264FCDC;
	sub_82466E20(ctx, base);
	// 8264FCDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FCF0 size=112
    let mut pc: u32 = 0x8264FCF0;
    'dispatch: loop {
        match pc {
            0x8264FCF0 => {
    //   block [0x8264FCF0..0x8264FD60)
	// 8264FCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FCFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FD00: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FD04: 38AA3AF0  addi r5, r10, 0x3af0
	ctx.r[5].s64 = ctx.r[10].s64 + 15088;
	// 8264FD08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FD0C: 390B5488  addi r8, r11, 0x5488
	ctx.r[8].s64 = ctx.r[11].s64 + 21640;
	// 8264FD10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264FD14: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 8264FD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FD1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FD20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264FD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FD28: 386A3EB0  addi r3, r10, 0x3eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 16048;
	// 8264FD2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264FD30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FD34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FD4C: 4BE170D5  bl 0x82466e20
	ctx.lr = 0x8264FD50;
	sub_82466E20(ctx, base);
	// 8264FD50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FD60 size=112
    let mut pc: u32 = 0x8264FD60;
    'dispatch: loop {
        match pc {
            0x8264FD60 => {
    //   block [0x8264FD60..0x8264FDD0)
	// 8264FD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FD6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FD70: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FD74: 38AA3940  addi r5, r10, 0x3940
	ctx.r[5].s64 = ctx.r[10].s64 + 14656;
	// 8264FD78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FD7C: 390B54B8  addi r8, r11, 0x54b8
	ctx.r[8].s64 = ctx.r[11].s64 + 21688;
	// 8264FD80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264FD84: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 8264FD88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FD8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FD90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264FD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FD98: 386A3EE0  addi r3, r10, 0x3ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 16096;
	// 8264FD9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264FDA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FDA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FDA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FDAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FDB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FDB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FDB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FDBC: 4BE17065  bl 0x82466e20
	ctx.lr = 0x8264FDC0;
	sub_82466E20(ctx, base);
	// 8264FDC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FDC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FDC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FDD0 size=108
    let mut pc: u32 = 0x8264FDD0;
    'dispatch: loop {
        match pc {
            0x8264FDD0 => {
    //   block [0x8264FDD0..0x8264FE3C)
	// 8264FDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FDDC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FDE4: 38EB54D0  addi r7, r11, 0x54d0
	ctx.r[7].s64 = ctx.r[11].s64 + 21712;
	// 8264FDE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264FDEC: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 8264FDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FDF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FDF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264FDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FE00: 386A3F10  addi r3, r10, 0x3f10
	ctx.r[3].s64 = ctx.r[10].s64 + 16144;
	// 8264FE04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264FE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264FE28: 4BE16FF9  bl 0x82466e20
	ctx.lr = 0x8264FE2C;
	sub_82466E20(ctx, base);
	// 8264FE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FE40 size=112
    let mut pc: u32 = 0x8264FE40;
    'dispatch: loop {
        match pc {
            0x8264FE40 => {
    //   block [0x8264FE40..0x8264FEB0)
	// 8264FE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FE4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FE50: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FE54: 38AA3E80  addi r5, r10, 0x3e80
	ctx.r[5].s64 = ctx.r[10].s64 + 16000;
	// 8264FE58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FE5C: 390B5500  addi r8, r11, 0x5500
	ctx.r[8].s64 = ctx.r[11].s64 + 21760;
	// 8264FE60: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264FE64: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 8264FE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FE6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264FE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FE78: 386A3F40  addi r3, r10, 0x3f40
	ctx.r[3].s64 = ctx.r[10].s64 + 16192;
	// 8264FE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264FE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FE9C: 4BE16F85  bl 0x82466e20
	ctx.lr = 0x8264FEA0;
	sub_82466E20(ctx, base);
	// 8264FEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FEB0 size=112
    let mut pc: u32 = 0x8264FEB0;
    'dispatch: loop {
        match pc {
            0x8264FEB0 => {
    //   block [0x8264FEB0..0x8264FF20)
	// 8264FEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FEBC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264FEC0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FEC4: 392AA41C  addi r9, r10, -0x5be4
	ctx.r[9].s64 = ctx.r[10].s64 + -23524;
	// 8264FEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FECC: 390B5590  addi r8, r11, 0x5590
	ctx.r[8].s64 = ctx.r[11].s64 + 21904;
	// 8264FED0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8264FED4: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 8264FED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FEDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264FEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FEE8: 386A3F70  addi r3, r10, 0x3f70
	ctx.r[3].s64 = ctx.r[10].s64 + 16240;
	// 8264FEEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264FEF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264FEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FF04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264FF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FF0C: 4BE16F15  bl 0x82466e20
	ctx.lr = 0x8264FF10;
	sub_82466E20(ctx, base);
	// 8264FF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FF20 size=112
    let mut pc: u32 = 0x8264FF20;
    'dispatch: loop {
        match pc {
            0x8264FF20 => {
    //   block [0x8264FF20..0x8264FF90)
	// 8264FF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FF2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FF30: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FF34: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 8264FF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FF3C: 390B55D8  addi r8, r11, 0x55d8
	ctx.r[8].s64 = ctx.r[11].s64 + 21976;
	// 8264FF40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264FF44: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 8264FF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FF4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264FF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FF58: 386A3FA0  addi r3, r10, 0x3fa0
	ctx.r[3].s64 = ctx.r[10].s64 + 16288;
	// 8264FF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264FF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FF7C: 4BE16EA5  bl 0x82466e20
	ctx.lr = 0x8264FF80;
	sub_82466E20(ctx, base);
	// 8264FF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264FF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264FF90 size=108
    let mut pc: u32 = 0x8264FF90;
    'dispatch: loop {
        match pc {
            0x8264FF90 => {
    //   block [0x8264FF90..0x8264FFFC)
	// 8264FF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264FF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264FF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264FF9C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264FFA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264FFA4: 38EB55F0  addi r7, r11, 0x55f0
	ctx.r[7].s64 = ctx.r[11].s64 + 22000;
	// 8264FFA8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8264FFAC: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 8264FFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264FFB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264FFB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264FFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264FFC0: 386A3FD0  addi r3, r10, 0x3fd0
	ctx.r[3].s64 = ctx.r[10].s64 + 16336;
	// 8264FFC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264FFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264FFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264FFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264FFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264FFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264FFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264FFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264FFE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264FFE8: 4BE16E39  bl 0x82466e20
	ctx.lr = 0x8264FFEC;
	sub_82466E20(ctx, base);
	// 8264FFEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264FFF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264FFF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264FFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650000 size=116
    let mut pc: u32 = 0x82650000;
    'dispatch: loop {
        match pc {
            0x82650000 => {
    //   block [0x82650000..0x82650074)
	// 82650000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265000C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82650010: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82650014: 390A5680  addi r8, r10, 0x5680
	ctx.r[8].s64 = ctx.r[10].s64 + 22144;
	// 82650018: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265001C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82650020: 38AA3E80  addi r5, r10, 0x3e80
	ctx.r[5].s64 = ctx.r[10].s64 + 16000;
	// 82650024: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650028: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265002C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650034: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 82650038: 396BA430  addi r11, r11, -0x5bd0
	ctx.r[11].s64 = ctx.r[11].s64 + -23504;
	// 8265003C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650040: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650044: 386A4000  addi r3, r10, 0x4000
	ctx.r[3].s64 = ctx.r[10].s64 + 16384;
	// 82650048: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265004C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650050: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82650054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265005C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650060: 4BE16DC1  bl 0x82466e20
	ctx.lr = 0x82650064;
	sub_82466E20(ctx, base);
	// 82650064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265006C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650078 size=108
    let mut pc: u32 = 0x82650078;
    'dispatch: loop {
        match pc {
            0x82650078 => {
    //   block [0x82650078..0x826500E4)
	// 82650078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265007C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650084: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265008C: 38EB5758  addi r7, r11, 0x5758
	ctx.r[7].s64 = ctx.r[11].s64 + 22360;
	// 82650090: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82650094: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 82650098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265009C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826500A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826500A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826500A8: 386A4030  addi r3, r10, 0x4030
	ctx.r[3].s64 = ctx.r[10].s64 + 16432;
	// 826500AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826500B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826500B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826500B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826500BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826500C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826500C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826500C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826500CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826500D0: 4BE16D51  bl 0x82466e20
	ctx.lr = 0x826500D4;
	sub_82466E20(ctx, base);
	// 826500D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826500D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826500DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826500E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826500E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826500E8 size=112
    let mut pc: u32 = 0x826500E8;
    'dispatch: loop {
        match pc {
            0x826500E8 => {
    //   block [0x826500E8..0x82650158)
	// 826500E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826500EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826500F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826500F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826500F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826500FC: 38AA3E80  addi r5, r10, 0x3e80
	ctx.r[5].s64 = ctx.r[10].s64 + 16000;
	// 82650100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650104: 390B57A0  addi r8, r11, 0x57a0
	ctx.r[8].s64 = ctx.r[11].s64 + 22432;
	// 82650108: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8265010C: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 82650110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650114: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265011C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650120: 386A4060  addi r3, r10, 0x4060
	ctx.r[3].s64 = ctx.r[10].s64 + 16480;
	// 82650124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265012C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265013C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650144: 4BE16CDD  bl 0x82466e20
	ctx.lr = 0x82650148;
	sub_82466E20(ctx, base);
	// 82650148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265014C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650158 size=112
    let mut pc: u32 = 0x82650158;
    'dispatch: loop {
        match pc {
            0x82650158 => {
    //   block [0x82650158..0x826501C8)
	// 82650158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265015C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650168: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265016C: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 82650170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650174: 390B5818  addi r8, r11, 0x5818
	ctx.r[8].s64 = ctx.r[11].s64 + 22552;
	// 82650178: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265017C: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82650180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650184: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265018C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650190: 386A4090  addi r3, r10, 0x4090
	ctx.r[3].s64 = ctx.r[10].s64 + 16528;
	// 82650194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265019C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826501A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826501A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826501A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826501AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826501B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826501B4: 4BE16C6D  bl 0x82466e20
	ctx.lr = 0x826501B8;
	sub_82466E20(ctx, base);
	// 826501B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826501BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826501C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826501C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826501C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826501C8 size=108
    let mut pc: u32 = 0x826501C8;
    'dispatch: loop {
        match pc {
            0x826501C8 => {
    //   block [0x826501C8..0x82650234)
	// 826501C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826501CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826501D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826501D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826501D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826501DC: 38EB5848  addi r7, r11, 0x5848
	ctx.r[7].s64 = ctx.r[11].s64 + 22600;
	// 826501E0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826501E4: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826501E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826501EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826501F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826501F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826501F8: 386A40C0  addi r3, r10, 0x40c0
	ctx.r[3].s64 = ctx.r[10].s64 + 16576;
	// 826501FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82650200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265020C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265021C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82650220: 4BE16C01  bl 0x82466e20
	ctx.lr = 0x82650224;
	sub_82466E20(ctx, base);
	// 82650224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265022C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650238 size=112
    let mut pc: u32 = 0x82650238;
    'dispatch: loop {
        match pc {
            0x82650238 => {
    //   block [0x82650238..0x826502A8)
	// 82650238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265023C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650244: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650248: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265024C: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 82650250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650254: 390B58C0  addi r8, r11, 0x58c0
	ctx.r[8].s64 = ctx.r[11].s64 + 22720;
	// 82650258: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265025C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82650260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650264: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265026C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650270: 386A40F0  addi r3, r10, 0x40f0
	ctx.r[3].s64 = ctx.r[10].s64 + 16624;
	// 82650274: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265027C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265028C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650294: 4BE16B8D  bl 0x82466e20
	ctx.lr = 0x82650298;
	sub_82466E20(ctx, base);
	// 82650298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265029C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826502A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826502A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826502A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826502A8 size=24
    let mut pc: u32 = 0x826502A8;
    'dispatch: loop {
        match pc {
            0x826502A8 => {
    //   block [0x826502A8..0x826502C0)
	// 826502A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826502AC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826502B0: 394A84A0  addi r10, r10, -0x7b60
	ctx.r[10].s64 = ctx.r[10].s64 + -31584;
	// 826502B4: 816B5908  lwz r11, 0x5908(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22792 as u32) ) } as u64;
	// 826502B8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826502BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826502C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826502C0 size=116
    let mut pc: u32 = 0x826502C0;
    'dispatch: loop {
        match pc {
            0x826502C0 => {
    //   block [0x826502C0..0x82650334)
	// 826502C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826502C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826502C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826502CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826502D0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826502D4: 390B84A0  addi r8, r11, -0x7b60
	ctx.r[8].s64 = ctx.r[11].s64 + -31584;
	// 826502D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826502DC: 392AA48C  addi r9, r10, -0x5b74
	ctx.r[9].s64 = ctx.r[10].s64 + -23412;
	// 826502E0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826502E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826502E8: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826502EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826502F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826502F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826502F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826502FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650304: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82650308: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 8265030C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82650310: 386B4120  addi r3, r11, 0x4120
	ctx.r[3].s64 = ctx.r[11].s64 + 16672;
	// 82650314: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82650318: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265031C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650320: 4BE16B01  bl 0x82466e20
	ctx.lr = 0x82650324;
	sub_82466E20(ctx, base);
	// 82650324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265032C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650338 size=112
    let mut pc: u32 = 0x82650338;
    'dispatch: loop {
        match pc {
            0x82650338 => {
    //   block [0x82650338..0x826503A8)
	// 82650338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265033C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650344: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650348: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265034C: 38AA4120  addi r5, r10, 0x4120
	ctx.r[5].s64 = ctx.r[10].s64 + 16672;
	// 82650350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650354: 390B590C  addi r8, r11, 0x590c
	ctx.r[8].s64 = ctx.r[11].s64 + 22796;
	// 82650358: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265035C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82650360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650364: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265036C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650370: 386A4150  addi r3, r10, 0x4150
	ctx.r[3].s64 = ctx.r[10].s64 + 16720;
	// 82650374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265037C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265038C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650394: 4BE16A8D  bl 0x82466e20
	ctx.lr = 0x82650398;
	sub_82466E20(ctx, base);
	// 82650398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265039C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826503A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826503A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826503A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826503A8 size=112
    let mut pc: u32 = 0x826503A8;
    'dispatch: loop {
        match pc {
            0x826503A8 => {
    //   block [0x826503A8..0x82650418)
	// 826503A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826503AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826503B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826503B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826503B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826503BC: 38AA4150  addi r5, r10, 0x4150
	ctx.r[5].s64 = ctx.r[10].s64 + 16720;
	// 826503C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826503C4: 390B5940  addi r8, r11, 0x5940
	ctx.r[8].s64 = ctx.r[11].s64 + 22848;
	// 826503C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826503CC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826503D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826503D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826503D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826503DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826503E0: 386A4180  addi r3, r10, 0x4180
	ctx.r[3].s64 = ctx.r[10].s64 + 16768;
	// 826503E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826503E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826503EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826503F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826503F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826503F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826503FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650404: 4BE16A1D  bl 0x82466e20
	ctx.lr = 0x82650408;
	sub_82466E20(ctx, base);
	// 82650408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265040C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650418 size=112
    let mut pc: u32 = 0x82650418;
    'dispatch: loop {
        match pc {
            0x82650418 => {
    //   block [0x82650418..0x82650488)
	// 82650418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265041C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650428: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265042C: 38AA4150  addi r5, r10, 0x4150
	ctx.r[5].s64 = ctx.r[10].s64 + 16720;
	// 82650430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650434: 390B59A0  addi r8, r11, 0x59a0
	ctx.r[8].s64 = ctx.r[11].s64 + 22944;
	// 82650438: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265043C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82650440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650444: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265044C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650450: 386A41B0  addi r3, r10, 0x41b0
	ctx.r[3].s64 = ctx.r[10].s64 + 16816;
	// 82650454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265045C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265046C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650474: 4BE169AD  bl 0x82466e20
	ctx.lr = 0x82650478;
	sub_82466E20(ctx, base);
	// 82650478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265047C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650488 size=112
    let mut pc: u32 = 0x82650488;
    'dispatch: loop {
        match pc {
            0x82650488 => {
    //   block [0x82650488..0x826504F8)
	// 82650488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650494: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650498: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265049C: 38AA4150  addi r5, r10, 0x4150
	ctx.r[5].s64 = ctx.r[10].s64 + 16720;
	// 826504A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826504A4: 390B59D0  addi r8, r11, 0x59d0
	ctx.r[8].s64 = ctx.r[11].s64 + 22992;
	// 826504A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826504AC: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826504B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826504B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826504B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826504BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826504C0: 386A41E0  addi r3, r10, 0x41e0
	ctx.r[3].s64 = ctx.r[10].s64 + 16864;
	// 826504C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826504C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826504CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826504D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826504D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826504D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826504DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826504E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826504E4: 4BE1693D  bl 0x82466e20
	ctx.lr = 0x826504E8;
	sub_82466E20(ctx, base);
	// 826504E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826504EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826504F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826504F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826504F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826504F8 size=108
    let mut pc: u32 = 0x826504F8;
    'dispatch: loop {
        match pc {
            0x826504F8 => {
    //   block [0x826504F8..0x82650564)
	// 826504F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826504FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650504: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265050C: 38EB5A18  addi r7, r11, 0x5a18
	ctx.r[7].s64 = ctx.r[11].s64 + 23064;
	// 82650510: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82650514: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82650518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265051C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82650524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650528: 386A4210  addi r3, r10, 0x4210
	ctx.r[3].s64 = ctx.r[10].s64 + 16912;
	// 8265052C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82650530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265053C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265054C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82650550: 4BE168D1  bl 0x82466e20
	ctx.lr = 0x82650554;
	sub_82466E20(ctx, base);
	// 82650554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265055C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650568 size=112
    let mut pc: u32 = 0x82650568;
    'dispatch: loop {
        match pc {
            0x82650568 => {
    //   block [0x82650568..0x826505D8)
	// 82650568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265056C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650578: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265057C: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 82650580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650584: 390B5A48  addi r8, r11, 0x5a48
	ctx.r[8].s64 = ctx.r[11].s64 + 23112;
	// 82650588: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265058C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82650590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650594: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265059C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826505A0: 386A4240  addi r3, r10, 0x4240
	ctx.r[3].s64 = ctx.r[10].s64 + 16960;
	// 826505A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826505A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826505AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826505B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826505B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826505B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826505BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826505C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826505C4: 4BE1685D  bl 0x82466e20
	ctx.lr = 0x826505C8;
	sub_82466E20(ctx, base);
	// 826505C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826505CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826505D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826505D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826505D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826505D8 size=116
    let mut pc: u32 = 0x826505D8;
    'dispatch: loop {
        match pc {
            0x826505D8 => {
    //   block [0x826505D8..0x8265064C)
	// 826505D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826505DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826505E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826505E4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 826505E8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826505EC: 390A5A60  addi r8, r10, 0x5a60
	ctx.r[8].s64 = ctx.r[10].s64 + 23136;
	// 826505F0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826505F4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826505F8: 38AA46C0  addi r5, r10, 0x46c0
	ctx.r[5].s64 = ctx.r[10].s64 + 18112;
	// 826505FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650600: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82650604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265060C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82650610: 396BA4A0  addi r11, r11, -0x5b60
	ctx.r[11].s64 = ctx.r[11].s64 + -23392;
	// 82650614: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650618: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265061C: 386A4270  addi r3, r10, 0x4270
	ctx.r[3].s64 = ctx.r[10].s64 + 17008;
	// 82650620: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82650624: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650628: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265062C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650638: 4BE167E9  bl 0x82466e20
	ctx.lr = 0x8265063C;
	sub_82466E20(ctx, base);
	// 8265063C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650650 size=100
    let mut pc: u32 = 0x82650650;
    'dispatch: loop {
        match pc {
            0x82650650 => {
    //   block [0x82650650..0x826506B4)
	// 82650650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265065C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650664: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82650668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265066C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650670: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82650674: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265067C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650684: 386A42A0  addi r3, r10, 0x42a0
	ctx.r[3].s64 = ctx.r[10].s64 + 17056;
	// 82650688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265068C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650690: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650698: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265069C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826506A0: 4BE16781  bl 0x82466e20
	ctx.lr = 0x826506A4;
	sub_82466E20(ctx, base);
	// 826506A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826506A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826506AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826506B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826506B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826506B8 size=100
    let mut pc: u32 = 0x826506B8;
    'dispatch: loop {
        match pc {
            0x826506B8 => {
    //   block [0x826506B8..0x8265071C)
	// 826506B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826506BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826506C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826506C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826506C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826506CC: 38AA4330  addi r5, r10, 0x4330
	ctx.r[5].s64 = ctx.r[10].s64 + 17200;
	// 826506D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826506D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826506D8: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826506DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826506E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826506E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826506E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826506EC: 386A42D0  addi r3, r10, 0x42d0
	ctx.r[3].s64 = ctx.r[10].s64 + 17104;
	// 826506F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826506F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826506F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826506FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650700: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650708: 4BE16719  bl 0x82466e20
	ctx.lr = 0x8265070C;
	sub_82466E20(ctx, base);
	// 8265070C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650720 size=100
    let mut pc: u32 = 0x82650720;
    'dispatch: loop {
        match pc {
            0x82650720 => {
    //   block [0x82650720..0x82650784)
	// 82650720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265072C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650734: 38AA4270  addi r5, r10, 0x4270
	ctx.r[5].s64 = ctx.r[10].s64 + 17008;
	// 82650738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265073C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650740: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82650744: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265074C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650754: 386A4300  addi r3, r10, 0x4300
	ctx.r[3].s64 = ctx.r[10].s64 + 17152;
	// 82650758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265075C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650760: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650768: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265076C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650770: 4BE166B1  bl 0x82466e20
	ctx.lr = 0x82650774;
	sub_82466E20(ctx, base);
	// 82650774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265077C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650788 size=104
    let mut pc: u32 = 0x82650788;
    'dispatch: loop {
        match pc {
            0x82650788 => {
    //   block [0x82650788..0x826507F0)
	// 82650788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265078C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650794: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82650798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265079C: 392AA504  addi r9, r10, -0x5afc
	ctx.r[9].s64 = ctx.r[10].s64 + -23292;
	// 826507A0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826507A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826507A8: 38AA42A0  addi r5, r10, 0x42a0
	ctx.r[5].s64 = ctx.r[10].s64 + 17056;
	// 826507AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826507B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826507B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826507B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826507BC: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826507C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826507C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826507C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826507CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826507D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826507D4: 386A4330  addi r3, r10, 0x4330
	ctx.r[3].s64 = ctx.r[10].s64 + 17200;
	// 826507D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826507DC: 4BE16645  bl 0x82466e20
	ctx.lr = 0x826507E0;
	sub_82466E20(ctx, base);
	// 826507E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826507E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826507E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826507EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826507F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826507F0 size=108
    let mut pc: u32 = 0x826507F0;
    'dispatch: loop {
        match pc {
            0x826507F0 => {
    //   block [0x826507F0..0x8265085C)
	// 826507F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826507F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826507F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826507FC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650804: 38EB5BE0  addi r7, r11, 0x5be0
	ctx.r[7].s64 = ctx.r[11].s64 + 23520;
	// 82650808: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265080C: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82650810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650818: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265081C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650820: 386A4360  addi r3, r10, 0x4360
	ctx.r[3].s64 = ctx.r[10].s64 + 17248;
	// 82650824: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82650828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265082C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265083C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82650848: 4BE165D9  bl 0x82466e20
	ctx.lr = 0x8265084C;
	sub_82466E20(ctx, base);
	// 8265084C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650860 size=112
    let mut pc: u32 = 0x82650860;
    'dispatch: loop {
        match pc {
            0x82650860 => {
    //   block [0x82650860..0x826508D0)
	// 82650860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265086C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650870: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650874: 38AA4330  addi r5, r10, 0x4330
	ctx.r[5].s64 = ctx.r[10].s64 + 17200;
	// 82650878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265087C: 390B5C10  addi r8, r11, 0x5c10
	ctx.r[8].s64 = ctx.r[11].s64 + 23568;
	// 82650880: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82650884: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82650888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265088C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650898: 386A4390  addi r3, r10, 0x4390
	ctx.r[3].s64 = ctx.r[10].s64 + 17296;
	// 8265089C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826508A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826508A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826508A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826508AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826508B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826508B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826508B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826508BC: 4BE16565  bl 0x82466e20
	ctx.lr = 0x826508C0;
	sub_82466E20(ctx, base);
	// 826508C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826508C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826508C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826508CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826508D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826508D0 size=24
    let mut pc: u32 = 0x826508D0;
    'dispatch: loop {
        match pc {
            0x826508D0 => {
    //   block [0x826508D0..0x826508E8)
	// 826508D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826508D4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826508D8: 394A84B8  addi r10, r10, -0x7b48
	ctx.r[10].s64 = ctx.r[10].s64 + -31560;
	// 826508DC: 816B5CB8  lwz r11, 0x5cb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23736 as u32) ) } as u64;
	// 826508E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826508E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826508E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826508E8 size=116
    let mut pc: u32 = 0x826508E8;
    'dispatch: loop {
        match pc {
            0x826508E8 => {
    //   block [0x826508E8..0x8265095C)
	// 826508E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826508EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826508F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826508F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826508F8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826508FC: 390B84B8  addi r8, r11, -0x7b48
	ctx.r[8].s64 = ctx.r[11].s64 + -31560;
	// 82650900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650904: 392AA568  addi r9, r10, -0x5a98
	ctx.r[9].s64 = ctx.r[10].s64 + -23192;
	// 82650908: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265090C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82650910: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82650914: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265091C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265092C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82650930: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82650934: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82650938: 386B43C0  addi r3, r11, 0x43c0
	ctx.r[3].s64 = ctx.r[11].s64 + 17344;
	// 8265093C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82650940: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650948: 4BE164D9  bl 0x82466e20
	ctx.lr = 0x8265094C;
	sub_82466E20(ctx, base);
	// 8265094C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650960 size=100
    let mut pc: u32 = 0x82650960;
    'dispatch: loop {
        match pc {
            0x82650960 => {
    //   block [0x82650960..0x826509C4)
	// 82650960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265096C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650974: 38AA43C0  addi r5, r10, 0x43c0
	ctx.r[5].s64 = ctx.r[10].s64 + 17344;
	// 82650978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265097C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650980: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82650984: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265098C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650994: 386A43F0  addi r3, r10, 0x43f0
	ctx.r[3].s64 = ctx.r[10].s64 + 17392;
	// 82650998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265099C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826509A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826509A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826509A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826509AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826509B0: 4BE16471  bl 0x82466e20
	ctx.lr = 0x826509B4;
	sub_82466E20(ctx, base);
	// 826509B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826509B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826509BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826509C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826509C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826509C8 size=100
    let mut pc: u32 = 0x826509C8;
    'dispatch: loop {
        match pc {
            0x826509C8 => {
    //   block [0x826509C8..0x82650A2C)
	// 826509C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826509CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826509D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826509D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826509D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826509DC: 38AA4450  addi r5, r10, 0x4450
	ctx.r[5].s64 = ctx.r[10].s64 + 17488;
	// 826509E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826509E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826509E8: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826509EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826509F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826509F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826509F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826509FC: 386A4420  addi r3, r10, 0x4420
	ctx.r[3].s64 = ctx.r[10].s64 + 17440;
	// 82650A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650A04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650A08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650A10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650A18: 4BE16409  bl 0x82466e20
	ctx.lr = 0x82650A1C;
	sub_82466E20(ctx, base);
	// 82650A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650A30 size=112
    let mut pc: u32 = 0x82650A30;
    'dispatch: loop {
        match pc {
            0x82650A30 => {
    //   block [0x82650A30..0x82650AA0)
	// 82650A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650A3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650A40: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650A44: 38AA43C0  addi r5, r10, 0x43c0
	ctx.r[5].s64 = ctx.r[10].s64 + 17344;
	// 82650A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650A4C: 390B5CBC  addi r8, r11, 0x5cbc
	ctx.r[8].s64 = ctx.r[11].s64 + 23740;
	// 82650A50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82650A54: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82650A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650A5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650A60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650A68: 386A4450  addi r3, r10, 0x4450
	ctx.r[3].s64 = ctx.r[10].s64 + 17488;
	// 82650A6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650A8C: 4BE16395  bl 0x82466e20
	ctx.lr = 0x82650A90;
	sub_82466E20(ctx, base);
	// 82650A90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650A94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650A98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650A9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650AA0 size=100
    let mut pc: u32 = 0x82650AA0;
    'dispatch: loop {
        match pc {
            0x82650AA0 => {
    //   block [0x82650AA0..0x82650B04)
	// 82650AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650AAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650AB4: 38AA4450  addi r5, r10, 0x4450
	ctx.r[5].s64 = ctx.r[10].s64 + 17488;
	// 82650AB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650AC0: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 82650AC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650AD4: 386A4480  addi r3, r10, 0x4480
	ctx.r[3].s64 = ctx.r[10].s64 + 17536;
	// 82650AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650ADC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650AE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650AE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650AF0: 4BE16331  bl 0x82466e20
	ctx.lr = 0x82650AF4;
	sub_82466E20(ctx, base);
	// 82650AF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650AF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650AFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650B08 size=100
    let mut pc: u32 = 0x82650B08;
    'dispatch: loop {
        match pc {
            0x82650B08 => {
    //   block [0x82650B08..0x82650B6C)
	// 82650B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650B1C: 38AA43C0  addi r5, r10, 0x43c0
	ctx.r[5].s64 = ctx.r[10].s64 + 17344;
	// 82650B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650B28: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82650B2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650B3C: 386A44B0  addi r3, r10, 0x44b0
	ctx.r[3].s64 = ctx.r[10].s64 + 17584;
	// 82650B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650B44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650B48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650B50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650B58: 4BE162C9  bl 0x82466e20
	ctx.lr = 0x82650B5C;
	sub_82466E20(ctx, base);
	// 82650B5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650B70 size=100
    let mut pc: u32 = 0x82650B70;
    'dispatch: loop {
        match pc {
            0x82650B70 => {
    //   block [0x82650B70..0x82650BD4)
	// 82650B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650B7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650B84: 38AA43F0  addi r5, r10, 0x43f0
	ctx.r[5].s64 = ctx.r[10].s64 + 17392;
	// 82650B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650B90: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82650B94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650BA4: 386A44E0  addi r3, r10, 0x44e0
	ctx.r[3].s64 = ctx.r[10].s64 + 17632;
	// 82650BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650BAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650BB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650BB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650BC0: 4BE16261  bl 0x82466e20
	ctx.lr = 0x82650BC4;
	sub_82466E20(ctx, base);
	// 82650BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650BD8 size=100
    let mut pc: u32 = 0x82650BD8;
    'dispatch: loop {
        match pc {
            0x82650BD8 => {
    //   block [0x82650BD8..0x82650C3C)
	// 82650BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650BE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650BEC: 38AA44B0  addi r5, r10, 0x44b0
	ctx.r[5].s64 = ctx.r[10].s64 + 17584;
	// 82650BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650BF8: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82650BFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650C0C: 386A4510  addi r3, r10, 0x4510
	ctx.r[3].s64 = ctx.r[10].s64 + 17680;
	// 82650C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650C14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650C18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650C20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650C28: 4BE161F9  bl 0x82466e20
	ctx.lr = 0x82650C2C;
	sub_82466E20(ctx, base);
	// 82650C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650C40 size=100
    let mut pc: u32 = 0x82650C40;
    'dispatch: loop {
        match pc {
            0x82650C40 => {
    //   block [0x82650C40..0x82650CA4)
	// 82650C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650C4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650C54: 38AA43F0  addi r5, r10, 0x43f0
	ctx.r[5].s64 = ctx.r[10].s64 + 17392;
	// 82650C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650C60: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 82650C64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650C6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650C74: 386A4540  addi r3, r10, 0x4540
	ctx.r[3].s64 = ctx.r[10].s64 + 17728;
	// 82650C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650C7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650C80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82650C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650C88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650C90: 4BE16191  bl 0x82466e20
	ctx.lr = 0x82650C94;
	sub_82466E20(ctx, base);
	// 82650C94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650CA8 size=112
    let mut pc: u32 = 0x82650CA8;
    'dispatch: loop {
        match pc {
            0x82650CA8 => {
    //   block [0x82650CA8..0x82650D18)
	// 82650CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650CB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650CB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650CBC: 38AA45D0  addi r5, r10, 0x45d0
	ctx.r[5].s64 = ctx.r[10].s64 + 17872;
	// 82650CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650CC4: 390B5CEC  addi r8, r11, 0x5cec
	ctx.r[8].s64 = ctx.r[11].s64 + 23788;
	// 82650CC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82650CCC: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82650CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650CD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650CE0: 386A4570  addi r3, r10, 0x4570
	ctx.r[3].s64 = ctx.r[10].s64 + 17776;
	// 82650CE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650D04: 4BE1611D  bl 0x82466e20
	ctx.lr = 0x82650D08;
	sub_82466E20(ctx, base);
	// 82650D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650D18 size=112
    let mut pc: u32 = 0x82650D18;
    'dispatch: loop {
        match pc {
            0x82650D18 => {
    //   block [0x82650D18..0x82650D88)
	// 82650D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650D24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650D28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650D2C: 38AA4600  addi r5, r10, 0x4600
	ctx.r[5].s64 = ctx.r[10].s64 + 17920;
	// 82650D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650D34: 390B5D1C  addi r8, r11, 0x5d1c
	ctx.r[8].s64 = ctx.r[11].s64 + 23836;
	// 82650D38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82650D3C: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 82650D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650D44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650D50: 386A45A0  addi r3, r10, 0x45a0
	ctx.r[3].s64 = ctx.r[10].s64 + 17824;
	// 82650D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650D74: 4BE160AD  bl 0x82466e20
	ctx.lr = 0x82650D78;
	sub_82466E20(ctx, base);
	// 82650D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650D88 size=112
    let mut pc: u32 = 0x82650D88;
    'dispatch: loop {
        match pc {
            0x82650D88 => {
    //   block [0x82650D88..0x82650DF8)
	// 82650D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650D94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650D98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650D9C: 38AA46C0  addi r5, r10, 0x46c0
	ctx.r[5].s64 = ctx.r[10].s64 + 18112;
	// 82650DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650DA4: 390B5D34  addi r8, r11, 0x5d34
	ctx.r[8].s64 = ctx.r[11].s64 + 23860;
	// 82650DA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82650DAC: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82650DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650DB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650DC0: 386A45D0  addi r3, r10, 0x45d0
	ctx.r[3].s64 = ctx.r[10].s64 + 17872;
	// 82650DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650DE4: 4BE1603D  bl 0x82466e20
	ctx.lr = 0x82650DE8;
	sub_82466E20(ctx, base);
	// 82650DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650DF8 size=112
    let mut pc: u32 = 0x82650DF8;
    'dispatch: loop {
        match pc {
            0x82650DF8 => {
    //   block [0x82650DF8..0x82650E68)
	// 82650DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650E04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650E08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650E0C: 38AA45D0  addi r5, r10, 0x45d0
	ctx.r[5].s64 = ctx.r[10].s64 + 17872;
	// 82650E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650E14: 390B5D64  addi r8, r11, 0x5d64
	ctx.r[8].s64 = ctx.r[11].s64 + 23908;
	// 82650E18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82650E1C: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82650E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650E24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650E28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650E30: 386A4600  addi r3, r10, 0x4600
	ctx.r[3].s64 = ctx.r[10].s64 + 17920;
	// 82650E34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650E44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650E54: 4BE15FCD  bl 0x82466e20
	ctx.lr = 0x82650E58;
	sub_82466E20(ctx, base);
	// 82650E58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650E5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650E60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650E64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650E68 size=112
    let mut pc: u32 = 0x82650E68;
    'dispatch: loop {
        match pc {
            0x82650E68 => {
    //   block [0x82650E68..0x82650ED8)
	// 82650E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650E74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650E78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650E7C: 38AA4600  addi r5, r10, 0x4600
	ctx.r[5].s64 = ctx.r[10].s64 + 17920;
	// 82650E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650E84: 390B5D7C  addi r8, r11, 0x5d7c
	ctx.r[8].s64 = ctx.r[11].s64 + 23932;
	// 82650E88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82650E8C: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82650E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82650E94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650E98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650EA0: 386A4630  addi r3, r10, 0x4630
	ctx.r[3].s64 = ctx.r[10].s64 + 17968;
	// 82650EA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82650EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82650EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82650EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650EC4: 4BE15F5D  bl 0x82466e20
	ctx.lr = 0x82650EC8;
	sub_82466E20(ctx, base);
	// 82650EC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650ECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650ED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650ED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650ED8 size=116
    let mut pc: u32 = 0x82650ED8;
    'dispatch: loop {
        match pc {
            0x82650ED8 => {
    //   block [0x82650ED8..0x82650F4C)
	// 82650ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650EE4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82650EE8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82650EEC: 390A5D98  addi r8, r10, 0x5d98
	ctx.r[8].s64 = ctx.r[10].s64 + 23960;
	// 82650EF0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650EF4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82650EF8: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82650EFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650F00: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82650F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82650F08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82650F0C: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82650F10: 396BA57C  addi r11, r11, -0x5a84
	ctx.r[11].s64 = ctx.r[11].s64 + -23172;
	// 82650F14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650F18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650F1C: 386A4660  addi r3, r10, 0x4660
	ctx.r[3].s64 = ctx.r[10].s64 + 18016;
	// 82650F20: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82650F24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82650F28: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82650F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82650F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650F38: 4BE15EE9  bl 0x82466e20
	ctx.lr = 0x82650F3C;
	sub_82466E20(ctx, base);
	// 82650F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82650F50 size=48
    let mut pc: u32 = 0x82650F50;
    'dispatch: loop {
        match pc {
            0x82650F50 => {
    //   block [0x82650F50..0x82650F80)
	// 82650F50: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82650F54: 814B5E48  lwz r10, 0x5e48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24136 as u32) ) } as u64;
	// 82650F58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82650F5C: 396B8578  addi r11, r11, -0x7a88
	ctx.r[11].s64 = ctx.r[11].s64 + -31368;
	// 82650F60: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82650F64: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82650F68: 814A5E44  lwz r10, 0x5e44(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24132 as u32) ) } as u64;
	// 82650F6C: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 82650F70: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82650F74: 814A5E40  lwz r10, 0x5e40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24128 as u32) ) } as u64;
	// 82650F78: 914B0278  stw r10, 0x278(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(632 as u32), ctx.r[10].u32 ) };
	// 82650F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650F80 size=116
    let mut pc: u32 = 0x82650F80;
    'dispatch: loop {
        match pc {
            0x82650F80 => {
    //   block [0x82650F80..0x82650FF4)
	// 82650F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82650F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82650F8C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82650F90: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650F94: 392BA650  addi r9, r11, -0x59b0
	ctx.r[9].s64 = ctx.r[11].s64 + -22960;
	// 82650F98: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82650F9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82650FA0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 82650FA4: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82650FA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82650FAC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82650FB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82650FB4: 396B8578  addi r11, r11, -0x7a88
	ctx.r[11].s64 = ctx.r[11].s64 + -31368;
	// 82650FB8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82650FBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82650FC0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82650FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82650FC8: 386A4690  addi r3, r10, 0x4690
	ctx.r[3].s64 = ctx.r[10].s64 + 18064;
	// 82650FCC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82650FD0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82650FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82650FD8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82650FDC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82650FE0: 4BE15E41  bl 0x82466e20
	ctx.lr = 0x82650FE4;
	sub_82466E20(ctx, base);
	// 82650FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82650FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82650FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82650FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82650FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82650FF8 size=116
    let mut pc: u32 = 0x82650FF8;
    'dispatch: loop {
        match pc {
            0x82650FF8 => {
    //   block [0x82650FF8..0x8265106C)
	// 82650FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82650FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651004: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651008: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265100C: 390B5E50  addi r8, r11, 0x5e50
	ctx.r[8].s64 = ctx.r[11].s64 + 24144;
	// 82651010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651014: 392AA79C  addi r9, r10, -0x5864
	ctx.r[9].s64 = ctx.r[10].s64 + -22628;
	// 82651018: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265101C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82651020: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651024: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265102C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265103C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82651040: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82651044: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82651048: 386B46C0  addi r3, r11, 0x46c0
	ctx.r[3].s64 = ctx.r[11].s64 + 18112;
	// 8265104C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82651050: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651058: 4BE15DC9  bl 0x82466e20
	ctx.lr = 0x8265105C;
	sub_82466E20(ctx, base);
	// 8265105C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651070 size=112
    let mut pc: u32 = 0x82651070;
    'dispatch: loop {
        match pc {
            0x82651070 => {
    //   block [0x82651070..0x826510E0)
	// 82651070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265107C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651080: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651084: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265108C: 390B5EE0  addi r8, r11, 0x5ee0
	ctx.r[8].s64 = ctx.r[11].s64 + 24288;
	// 82651090: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82651094: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82651098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265109C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826510A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826510A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826510A8: 386A46F0  addi r3, r10, 0x46f0
	ctx.r[3].s64 = ctx.r[10].s64 + 18160;
	// 826510AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826510B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826510B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826510B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826510BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826510C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826510C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826510C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826510CC: 4BE15D55  bl 0x82466e20
	ctx.lr = 0x826510D0;
	sub_82466E20(ctx, base);
	// 826510D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826510D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826510D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826510DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826510E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826510E0 size=112
    let mut pc: u32 = 0x826510E0;
    'dispatch: loop {
        match pc {
            0x826510E0 => {
    //   block [0x826510E0..0x82651150)
	// 826510E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826510E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826510E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826510EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826510F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826510F4: 38AA2920  addi r5, r10, 0x2920
	ctx.r[5].s64 = ctx.r[10].s64 + 10528;
	// 826510F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826510FC: 390B5EF8  addi r8, r11, 0x5ef8
	ctx.r[8].s64 = ctx.r[11].s64 + 24312;
	// 82651100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82651104: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82651108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265110C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651118: 386A4720  addi r3, r10, 0x4720
	ctx.r[3].s64 = ctx.r[10].s64 + 18208;
	// 8265111C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265112C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265113C: 4BE15CE5  bl 0x82466e20
	ctx.lr = 0x82651140;
	sub_82466E20(ctx, base);
	// 82651140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265114C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651150 size=108
    let mut pc: u32 = 0x82651150;
    'dispatch: loop {
        match pc {
            0x82651150 => {
    //   block [0x82651150..0x826511BC)
	// 82651150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265115C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651164: 38EB5F10  addi r7, r11, 0x5f10
	ctx.r[7].s64 = ctx.r[11].s64 + 24336;
	// 82651168: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265116C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82651170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265117C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651180: 386A4750  addi r3, r10, 0x4750
	ctx.r[3].s64 = ctx.r[10].s64 + 18256;
	// 82651184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265118C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265119C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826511A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826511A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826511A8: 4BE15C79  bl 0x82466e20
	ctx.lr = 0x826511AC;
	sub_82466E20(ctx, base);
	// 826511AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826511B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826511B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826511B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826511C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826511C0 size=112
    let mut pc: u32 = 0x826511C0;
    'dispatch: loop {
        match pc {
            0x826511C0 => {
    //   block [0x826511C0..0x82651230)
	// 826511C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826511C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826511C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826511CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826511D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826511D4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826511D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826511DC: 390B5F28  addi r8, r11, 0x5f28
	ctx.r[8].s64 = ctx.r[11].s64 + 24360;
	// 826511E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826511E4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826511E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826511EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826511F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826511F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826511F8: 386A4780  addi r3, r10, 0x4780
	ctx.r[3].s64 = ctx.r[10].s64 + 18304;
	// 826511FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265120C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265121C: 4BE15C05  bl 0x82466e20
	ctx.lr = 0x82651220;
	sub_82466E20(ctx, base);
	// 82651220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265122C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651230 size=108
    let mut pc: u32 = 0x82651230;
    'dispatch: loop {
        match pc {
            0x82651230 => {
    //   block [0x82651230..0x8265129C)
	// 82651230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265123C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651244: 38EB5F70  addi r7, r11, 0x5f70
	ctx.r[7].s64 = ctx.r[11].s64 + 24432;
	// 82651248: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265124C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82651250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651254: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265125C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651260: 386A47B0  addi r3, r10, 0x47b0
	ctx.r[3].s64 = ctx.r[10].s64 + 18352;
	// 82651264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265126C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265127C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651288: 4BE15B99  bl 0x82466e20
	ctx.lr = 0x8265128C;
	sub_82466E20(ctx, base);
	// 8265128C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826512A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826512A0 size=112
    let mut pc: u32 = 0x826512A0;
    'dispatch: loop {
        match pc {
            0x826512A0 => {
    //   block [0x826512A0..0x82651310)
	// 826512A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826512A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826512A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826512AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826512B0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826512B4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826512B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826512BC: 390B5F88  addi r8, r11, 0x5f88
	ctx.r[8].s64 = ctx.r[11].s64 + 24456;
	// 826512C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826512C4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826512C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826512CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826512D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826512D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826512D8: 386A47E0  addi r3, r10, 0x47e0
	ctx.r[3].s64 = ctx.r[10].s64 + 18400;
	// 826512DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826512E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826512E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826512E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826512EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826512F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826512F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826512F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826512FC: 4BE15B25  bl 0x82466e20
	ctx.lr = 0x82651300;
	sub_82466E20(ctx, base);
	// 82651300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265130C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651310 size=112
    let mut pc: u32 = 0x82651310;
    'dispatch: loop {
        match pc {
            0x82651310 => {
    //   block [0x82651310..0x82651380)
	// 82651310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265131C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82651320: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651324: 392AA7F4  addi r9, r10, -0x580c
	ctx.r[9].s64 = ctx.r[10].s64 + -22540;
	// 82651328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265132C: 390B5FC0  addi r8, r11, 0x5fc0
	ctx.r[8].s64 = ctx.r[11].s64 + 24512;
	// 82651330: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82651334: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82651338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265133C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651348: 386A4810  addi r3, r10, 0x4810
	ctx.r[3].s64 = ctx.r[10].s64 + 18448;
	// 8265134C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82651350: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82651354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265135C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265136C: 4BE15AB5  bl 0x82466e20
	ctx.lr = 0x82651370;
	sub_82466E20(ctx, base);
	// 82651370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265137C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651380 size=116
    let mut pc: u32 = 0x82651380;
    'dispatch: loop {
        match pc {
            0x82651380 => {
    //   block [0x82651380..0x826513F4)
	// 82651380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265138C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651390: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82651394: 390B6068  addi r8, r11, 0x6068
	ctx.r[8].s64 = ctx.r[11].s64 + 24680;
	// 82651398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265139C: 392AA7C8  addi r9, r10, -0x5838
	ctx.r[9].s64 = ctx.r[10].s64 + -22584;
	// 826513A0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826513A4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826513A8: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 826513AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826513B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826513B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826513B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826513BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826513C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826513C4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826513C8: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826513CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826513D0: 386B4840  addi r3, r11, 0x4840
	ctx.r[3].s64 = ctx.r[11].s64 + 18496;
	// 826513D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826513D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826513DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826513E0: 4BE15A41  bl 0x82466e20
	ctx.lr = 0x826513E4;
	sub_82466E20(ctx, base);
	// 826513E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826513E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826513EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826513F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826513F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826513F8 size=112
    let mut pc: u32 = 0x826513F8;
    'dispatch: loop {
        match pc {
            0x826513F8 => {
    //   block [0x826513F8..0x82651468)
	// 826513F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826513FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651404: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82651408: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265140C: 392AA820  addi r9, r10, -0x57e0
	ctx.r[9].s64 = ctx.r[10].s64 + -22496;
	// 82651410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651414: 390B6088  addi r8, r11, 0x6088
	ctx.r[8].s64 = ctx.r[11].s64 + 24712;
	// 82651418: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8265141C: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82651420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651424: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265142C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651430: 386A4870  addi r3, r10, 0x4870
	ctx.r[3].s64 = ctx.r[10].s64 + 18544;
	// 82651434: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82651438: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265143C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265144C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651454: 4BE159CD  bl 0x82466e20
	ctx.lr = 0x82651458;
	sub_82466E20(ctx, base);
	// 82651458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265145C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651468 size=112
    let mut pc: u32 = 0x82651468;
    'dispatch: loop {
        match pc {
            0x82651468 => {
    //   block [0x82651468..0x826514D8)
	// 82651468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265146C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651478: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265147C: 38AA3A90  addi r5, r10, 0x3a90
	ctx.r[5].s64 = ctx.r[10].s64 + 14992;
	// 82651480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651484: 390B60E8  addi r8, r11, 0x60e8
	ctx.r[8].s64 = ctx.r[11].s64 + 24808;
	// 82651488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265148C: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 82651490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651494: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265149C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826514A0: 386A48A0  addi r3, r10, 0x48a0
	ctx.r[3].s64 = ctx.r[10].s64 + 18592;
	// 826514A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826514A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826514AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826514B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826514B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826514B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826514BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826514C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826514C4: 4BE1595D  bl 0x82466e20
	ctx.lr = 0x826514C8;
	sub_82466E20(ctx, base);
	// 826514C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826514CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826514D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826514D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826514D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826514D8 size=112
    let mut pc: u32 = 0x826514D8;
    'dispatch: loop {
        match pc {
            0x826514D8 => {
    //   block [0x826514D8..0x82651548)
	// 826514D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826514DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826514E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826514E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826514E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826514EC: 38AA39A0  addi r5, r10, 0x39a0
	ctx.r[5].s64 = ctx.r[10].s64 + 14752;
	// 826514F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826514F4: 390B6100  addi r8, r11, 0x6100
	ctx.r[8].s64 = ctx.r[11].s64 + 24832;
	// 826514F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826514FC: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82651500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265150C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651510: 386A48D0  addi r3, r10, 0x48d0
	ctx.r[3].s64 = ctx.r[10].s64 + 18640;
	// 82651514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265151C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265152C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651534: 4BE158ED  bl 0x82466e20
	ctx.lr = 0x82651538;
	sub_82466E20(ctx, base);
	// 82651538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265153C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651548 size=112
    let mut pc: u32 = 0x82651548;
    'dispatch: loop {
        match pc {
            0x82651548 => {
    //   block [0x82651548..0x826515B8)
	// 82651548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265154C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651558: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265155C: 38AA39A0  addi r5, r10, 0x39a0
	ctx.r[5].s64 = ctx.r[10].s64 + 14752;
	// 82651560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651564: 390B6148  addi r8, r11, 0x6148
	ctx.r[8].s64 = ctx.r[11].s64 + 24904;
	// 82651568: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265156C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82651570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651578: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265157C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651580: 386A4900  addi r3, r10, 0x4900
	ctx.r[3].s64 = ctx.r[10].s64 + 18688;
	// 82651584: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265158C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265159C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826515A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826515A4: 4BE1587D  bl 0x82466e20
	ctx.lr = 0x826515A8;
	sub_82466E20(ctx, base);
	// 826515A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826515AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826515B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826515B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826515B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826515B8 size=112
    let mut pc: u32 = 0x826515B8;
    'dispatch: loop {
        match pc {
            0x826515B8 => {
    //   block [0x826515B8..0x82651628)
	// 826515B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826515BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826515C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826515C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826515C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826515CC: 38AA39D0  addi r5, r10, 0x39d0
	ctx.r[5].s64 = ctx.r[10].s64 + 14800;
	// 826515D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826515D4: 390B61A8  addi r8, r11, 0x61a8
	ctx.r[8].s64 = ctx.r[11].s64 + 25000;
	// 826515D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826515DC: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826515E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826515E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826515E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826515EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826515F0: 386A4930  addi r3, r10, 0x4930
	ctx.r[3].s64 = ctx.r[10].s64 + 18736;
	// 826515F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826515F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826515FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265160C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651614: 4BE1580D  bl 0x82466e20
	ctx.lr = 0x82651618;
	sub_82466E20(ctx, base);
	// 82651618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265161C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651628 size=112
    let mut pc: u32 = 0x82651628;
    'dispatch: loop {
        match pc {
            0x82651628 => {
    //   block [0x82651628..0x82651698)
	// 82651628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265162C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651634: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651638: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265163C: 38AA39D0  addi r5, r10, 0x39d0
	ctx.r[5].s64 = ctx.r[10].s64 + 14800;
	// 82651640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651644: 390B6208  addi r8, r11, 0x6208
	ctx.r[8].s64 = ctx.r[11].s64 + 25096;
	// 82651648: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265164C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82651650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651654: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265165C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651660: 386A4960  addi r3, r10, 0x4960
	ctx.r[3].s64 = ctx.r[10].s64 + 18784;
	// 82651664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265166C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265167C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651684: 4BE1579D  bl 0x82466e20
	ctx.lr = 0x82651688;
	sub_82466E20(ctx, base);
	// 82651688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265168C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651698 size=112
    let mut pc: u32 = 0x82651698;
    'dispatch: loop {
        match pc {
            0x82651698 => {
    //   block [0x82651698..0x82651708)
	// 82651698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265169C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826516A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826516A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826516A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826516AC: 38AA39A0  addi r5, r10, 0x39a0
	ctx.r[5].s64 = ctx.r[10].s64 + 14752;
	// 826516B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826516B4: 390B6268  addi r8, r11, 0x6268
	ctx.r[8].s64 = ctx.r[11].s64 + 25192;
	// 826516B8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826516BC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826516C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826516C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826516C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826516CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826516D0: 386A4990  addi r3, r10, 0x4990
	ctx.r[3].s64 = ctx.r[10].s64 + 18832;
	// 826516D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826516D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826516DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826516E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826516E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826516E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826516EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826516F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826516F4: 4BE1572D  bl 0x82466e20
	ctx.lr = 0x826516F8;
	sub_82466E20(ctx, base);
	// 826516F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826516FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651708 size=112
    let mut pc: u32 = 0x82651708;
    'dispatch: loop {
        match pc {
            0x82651708 => {
    //   block [0x82651708..0x82651778)
	// 82651708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265170C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651714: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82651718: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8265171C: 38EA6328  addi r7, r10, 0x6328
	ctx.r[7].s64 = ctx.r[10].s64 + 25384;
	// 82651720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651724: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82651728: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8265172C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651730: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651734: 396BA838  addi r11, r11, -0x57c8
	ctx.r[11].s64 = ctx.r[11].s64 + -22472;
	// 82651738: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265173C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651740: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651744: 386A49C0  addi r3, r10, 0x49c0
	ctx.r[3].s64 = ctx.r[10].s64 + 18880;
	// 82651748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265174C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82651750: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651754: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82651758: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265175C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651760: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651764: 4BE156BD  bl 0x82466e20
	ctx.lr = 0x82651768;
	sub_82466E20(ctx, base);
	// 82651768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265176C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651778 size=112
    let mut pc: u32 = 0x82651778;
    'dispatch: loop {
        match pc {
            0x82651778 => {
    //   block [0x82651778..0x826517E8)
	// 82651778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265177C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651784: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651788: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265178C: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 82651790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651794: 390B64F0  addi r8, r11, 0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + 25840;
	// 82651798: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265179C: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826517A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826517A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826517A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826517AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826517B0: 386A49F0  addi r3, r10, 0x49f0
	ctx.r[3].s64 = ctx.r[10].s64 + 18928;
	// 826517B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826517B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826517BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826517C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826517C4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826517C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826517CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826517D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826517D4: 4BE1564D  bl 0x82466e20
	ctx.lr = 0x826517D8;
	sub_82466E20(ctx, base);
	// 826517D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826517DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826517E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826517E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826517E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826517E8 size=112
    let mut pc: u32 = 0x826517E8;
    'dispatch: loop {
        match pc {
            0x826517E8 => {
    //   block [0x826517E8..0x82651858)
	// 826517E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826517EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826517F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826517F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826517F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826517FC: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 82651800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651804: 390B6508  addi r8, r11, 0x6508
	ctx.r[8].s64 = ctx.r[11].s64 + 25864;
	// 82651808: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265180C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82651810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265181C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651820: 386A4A20  addi r3, r10, 0x4a20
	ctx.r[3].s64 = ctx.r[10].s64 + 18976;
	// 82651824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265182C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651834: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82651838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265183C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651844: 4BE155DD  bl 0x82466e20
	ctx.lr = 0x82651848;
	sub_82466E20(ctx, base);
	// 82651848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265184C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651858 size=112
    let mut pc: u32 = 0x82651858;
    'dispatch: loop {
        match pc {
            0x82651858 => {
    //   block [0x82651858..0x826518C8)
	// 82651858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265185C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651868: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265186C: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 82651870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651874: 390B6520  addi r8, r11, 0x6520
	ctx.r[8].s64 = ctx.r[11].s64 + 25888;
	// 82651878: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265187C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82651880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265188C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651890: 386A4A50  addi r3, r10, 0x4a50
	ctx.r[3].s64 = ctx.r[10].s64 + 19024;
	// 82651894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265189C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826518A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826518A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826518A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826518AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826518B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826518B4: 4BE1556D  bl 0x82466e20
	ctx.lr = 0x826518B8;
	sub_82466E20(ctx, base);
	// 826518B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826518BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826518C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826518C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826518C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826518C8 size=108
    let mut pc: u32 = 0x826518C8;
    'dispatch: loop {
        match pc {
            0x826518C8 => {
    //   block [0x826518C8..0x82651934)
	// 826518C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826518CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826518D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826518D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826518D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826518DC: 38EB6550  addi r7, r11, 0x6550
	ctx.r[7].s64 = ctx.r[11].s64 + 25936;
	// 826518E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826518E4: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826518E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826518EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826518F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826518F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826518F8: 386A4A80  addi r3, r10, 0x4a80
	ctx.r[3].s64 = ctx.r[10].s64 + 19072;
	// 826518FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265190C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265191C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651920: 4BE15501  bl 0x82466e20
	ctx.lr = 0x82651924;
	sub_82466E20(ctx, base);
	// 82651924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265192C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651938 size=112
    let mut pc: u32 = 0x82651938;
    'dispatch: loop {
        match pc {
            0x82651938 => {
    //   block [0x82651938..0x826519A8)
	// 82651938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265193C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651944: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651948: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265194C: 38AA29E0  addi r5, r10, 0x29e0
	ctx.r[5].s64 = ctx.r[10].s64 + 10720;
	// 82651950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651954: 390B6580  addi r8, r11, 0x6580
	ctx.r[8].s64 = ctx.r[11].s64 + 25984;
	// 82651958: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265195C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82651960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265196C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651970: 386A4AB0  addi r3, r10, 0x4ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 19120;
	// 82651974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265197C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651984: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82651988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265198C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651994: 4BE1548D  bl 0x82466e20
	ctx.lr = 0x82651998;
	sub_82466E20(ctx, base);
	// 82651998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265199C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826519A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826519A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826519A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826519A8 size=108
    let mut pc: u32 = 0x826519A8;
    'dispatch: loop {
        match pc {
            0x826519A8 => {
    //   block [0x826519A8..0x82651A14)
	// 826519A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826519AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826519B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826519B4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826519B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826519BC: 38EB6598  addi r7, r11, 0x6598
	ctx.r[7].s64 = ctx.r[11].s64 + 26008;
	// 826519C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826519C4: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826519C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826519CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826519D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826519D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826519D8: 386A4AE0  addi r3, r10, 0x4ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 19168;
	// 826519DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826519E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826519E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826519E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826519EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826519F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826519F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826519F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826519FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651A00: 4BE15421  bl 0x82466e20
	ctx.lr = 0x82651A04;
	sub_82466E20(ctx, base);
	// 82651A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651A18 size=108
    let mut pc: u32 = 0x82651A18;
    'dispatch: loop {
        match pc {
            0x82651A18 => {
    //   block [0x82651A18..0x82651A84)
	// 82651A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651A24: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651A2C: 38EB65C8  addi r7, r11, 0x65c8
	ctx.r[7].s64 = ctx.r[11].s64 + 26056;
	// 82651A30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82651A34: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82651A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651A3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651A40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651A48: 386A4B10  addi r3, r10, 0x4b10
	ctx.r[3].s64 = ctx.r[10].s64 + 19216;
	// 82651A4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651A70: 4BE153B1  bl 0x82466e20
	ctx.lr = 0x82651A74;
	sub_82466E20(ctx, base);
	// 82651A74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651A88 size=112
    let mut pc: u32 = 0x82651A88;
    'dispatch: loop {
        match pc {
            0x82651A88 => {
    //   block [0x82651A88..0x82651AF8)
	// 82651A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651A94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651A98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651A9C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651AA4: 390B6610  addi r8, r11, 0x6610
	ctx.r[8].s64 = ctx.r[11].s64 + 26128;
	// 82651AA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82651AAC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 82651AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651AB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651AC0: 386A4B40  addi r3, r10, 0x4b40
	ctx.r[3].s64 = ctx.r[10].s64 + 19264;
	// 82651AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651AE4: 4BE1533D  bl 0x82466e20
	ctx.lr = 0x82651AE8;
	sub_82466E20(ctx, base);
	// 82651AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651AF8 size=112
    let mut pc: u32 = 0x82651AF8;
    'dispatch: loop {
        match pc {
            0x82651AF8 => {
    //   block [0x82651AF8..0x82651B68)
	// 82651AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651B04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651B08: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651B0C: 38AA39D0  addi r5, r10, 0x39d0
	ctx.r[5].s64 = ctx.r[10].s64 + 14800;
	// 82651B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651B14: 390B6658  addi r8, r11, 0x6658
	ctx.r[8].s64 = ctx.r[11].s64 + 26200;
	// 82651B18: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82651B1C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82651B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651B24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651B30: 386A4B70  addi r3, r10, 0x4b70
	ctx.r[3].s64 = ctx.r[10].s64 + 19312;
	// 82651B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651B54: 4BE152CD  bl 0x82466e20
	ctx.lr = 0x82651B58;
	sub_82466E20(ctx, base);
	// 82651B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651B68 size=108
    let mut pc: u32 = 0x82651B68;
    'dispatch: loop {
        match pc {
            0x82651B68 => {
    //   block [0x82651B68..0x82651BD4)
	// 82651B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651B74: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651B7C: 38EB66E8  addi r7, r11, 0x66e8
	ctx.r[7].s64 = ctx.r[11].s64 + 26344;
	// 82651B80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82651B84: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 82651B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651B8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651B90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651B98: 386A4BA0  addi r3, r10, 0x4ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 19360;
	// 82651B9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651BBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651BC0: 4BE15261  bl 0x82466e20
	ctx.lr = 0x82651BC4;
	sub_82466E20(ctx, base);
	// 82651BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651BD8 size=108
    let mut pc: u32 = 0x82651BD8;
    'dispatch: loop {
        match pc {
            0x82651BD8 => {
    //   block [0x82651BD8..0x82651C44)
	// 82651BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651BE4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651BEC: 38EB6730  addi r7, r11, 0x6730
	ctx.r[7].s64 = ctx.r[11].s64 + 26416;
	// 82651BF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82651BF4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82651BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651BFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651C00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651C04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651C08: 386A4BD0  addi r3, r10, 0x4bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 19408;
	// 82651C0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651C2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651C30: 4BE151F1  bl 0x82466e20
	ctx.lr = 0x82651C34;
	sub_82466E20(ctx, base);
	// 82651C34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651C48 size=108
    let mut pc: u32 = 0x82651C48;
    'dispatch: loop {
        match pc {
            0x82651C48 => {
    //   block [0x82651C48..0x82651CB4)
	// 82651C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651C54: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651C5C: 38EB6760  addi r7, r11, 0x6760
	ctx.r[7].s64 = ctx.r[11].s64 + 26464;
	// 82651C60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82651C64: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 82651C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651C6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651C78: 386A4C00  addi r3, r10, 0x4c00
	ctx.r[3].s64 = ctx.r[10].s64 + 19456;
	// 82651C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651CA0: 4BE15181  bl 0x82466e20
	ctx.lr = 0x82651CA4;
	sub_82466E20(ctx, base);
	// 82651CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651CB8 size=112
    let mut pc: u32 = 0x82651CB8;
    'dispatch: loop {
        match pc {
            0x82651CB8 => {
    //   block [0x82651CB8..0x82651D28)
	// 82651CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651CC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651CC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651CCC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651CD4: 390B6790  addi r8, r11, 0x6790
	ctx.r[8].s64 = ctx.r[11].s64 + 26512;
	// 82651CD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82651CDC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 82651CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651CE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651CF0: 386A4C30  addi r3, r10, 0x4c30
	ctx.r[3].s64 = ctx.r[10].s64 + 19504;
	// 82651CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651D14: 4BE1510D  bl 0x82466e20
	ctx.lr = 0x82651D18;
	sub_82466E20(ctx, base);
	// 82651D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651D28 size=112
    let mut pc: u32 = 0x82651D28;
    'dispatch: loop {
        match pc {
            0x82651D28 => {
    //   block [0x82651D28..0x82651D98)
	// 82651D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651D34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651D38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651D3C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651D44: 390B67C0  addi r8, r11, 0x67c0
	ctx.r[8].s64 = ctx.r[11].s64 + 26560;
	// 82651D48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82651D4C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82651D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651D54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651D60: 386A4C60  addi r3, r10, 0x4c60
	ctx.r[3].s64 = ctx.r[10].s64 + 19552;
	// 82651D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651D84: 4BE1509D  bl 0x82466e20
	ctx.lr = 0x82651D88;
	sub_82466E20(ctx, base);
	// 82651D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651D98 size=112
    let mut pc: u32 = 0x82651D98;
    'dispatch: loop {
        match pc {
            0x82651D98 => {
    //   block [0x82651D98..0x82651E08)
	// 82651D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651DA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651DA8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651DAC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651DB4: 390B67D8  addi r8, r11, 0x67d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26584;
	// 82651DB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82651DBC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 82651DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651DC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651DC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651DD0: 386A4C90  addi r3, r10, 0x4c90
	ctx.r[3].s64 = ctx.r[10].s64 + 19600;
	// 82651DD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651DF4: 4BE1502D  bl 0x82466e20
	ctx.lr = 0x82651DF8;
	sub_82466E20(ctx, base);
	// 82651DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651E08 size=108
    let mut pc: u32 = 0x82651E08;
    'dispatch: loop {
        match pc {
            0x82651E08 => {
    //   block [0x82651E08..0x82651E74)
	// 82651E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651E14: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651E1C: 38EB67F0  addi r7, r11, 0x67f0
	ctx.r[7].s64 = ctx.r[11].s64 + 26608;
	// 82651E20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82651E24: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82651E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651E2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651E30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651E38: 386A4CC0  addi r3, r10, 0x4cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 19648;
	// 82651E3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651E40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651E44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651E5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651E60: 4BE14FC1  bl 0x82466e20
	ctx.lr = 0x82651E64;
	sub_82466E20(ctx, base);
	// 82651E64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651E78 size=112
    let mut pc: u32 = 0x82651E78;
    'dispatch: loop {
        match pc {
            0x82651E78 => {
    //   block [0x82651E78..0x82651EE8)
	// 82651E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651E84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651E88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651E8C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651E94: 390B6820  addi r8, r11, 0x6820
	ctx.r[8].s64 = ctx.r[11].s64 + 26656;
	// 82651E98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82651E9C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82651EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651EA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651EB0: 386A4CF0  addi r3, r10, 0x4cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 19696;
	// 82651EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651ED4: 4BE14F4D  bl 0x82466e20
	ctx.lr = 0x82651ED8;
	sub_82466E20(ctx, base);
	// 82651ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651EE8 size=108
    let mut pc: u32 = 0x82651EE8;
    'dispatch: loop {
        match pc {
            0x82651EE8 => {
    //   block [0x82651EE8..0x82651F54)
	// 82651EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651EF4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651EFC: 38EB6838  addi r7, r11, 0x6838
	ctx.r[7].s64 = ctx.r[11].s64 + 26680;
	// 82651F00: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82651F04: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82651F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651F0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651F10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651F18: 386A4D20  addi r3, r10, 0x4d20
	ctx.r[3].s64 = ctx.r[10].s64 + 19744;
	// 82651F1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82651F20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651F3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82651F40: 4BE14EE1  bl 0x82466e20
	ctx.lr = 0x82651F44;
	sub_82466E20(ctx, base);
	// 82651F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651F58 size=112
    let mut pc: u32 = 0x82651F58;
    'dispatch: loop {
        match pc {
            0x82651F58 => {
    //   block [0x82651F58..0x82651FC8)
	// 82651F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651F64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651F68: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651F6C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82651F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651F74: 390B6928  addi r8, r11, 0x6928
	ctx.r[8].s64 = ctx.r[11].s64 + 26920;
	// 82651F78: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82651F7C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82651F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651F84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651F88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82651F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82651F90: 386A4D50  addi r3, r10, 0x4d50
	ctx.r[3].s64 = ctx.r[10].s64 + 19792;
	// 82651F94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82651F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82651F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82651FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82651FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82651FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82651FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82651FB4: 4BE14E6D  bl 0x82466e20
	ctx.lr = 0x82651FB8;
	sub_82466E20(ctx, base);
	// 82651FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82651FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82651FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82651FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82651FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82651FC8 size=108
    let mut pc: u32 = 0x82651FC8;
    'dispatch: loop {
        match pc {
            0x82651FC8 => {
    //   block [0x82651FC8..0x82652034)
	// 82651FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82651FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82651FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82651FD4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82651FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82651FDC: 38EB6AD8  addi r7, r11, 0x6ad8
	ctx.r[7].s64 = ctx.r[11].s64 + 27352;
	// 82651FE0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82651FE4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82651FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82651FEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82651FF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82651FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82651FF8: 386A4D80  addi r3, r10, 0x4d80
	ctx.r[3].s64 = ctx.r[10].s64 + 19840;
	// 82651FFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265200C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265201C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652020: 4BE14E01  bl 0x82466e20
	ctx.lr = 0x82652024;
	sub_82466E20(ctx, base);
	// 82652024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265202C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652038 size=112
    let mut pc: u32 = 0x82652038;
    'dispatch: loop {
        match pc {
            0x82652038 => {
    //   block [0x82652038..0x826520A8)
	// 82652038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265203C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652044: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652048: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265204C: 38AA39D0  addi r5, r10, 0x39d0
	ctx.r[5].s64 = ctx.r[10].s64 + 14800;
	// 82652050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652054: 390B6C70  addi r8, r11, 0x6c70
	ctx.r[8].s64 = ctx.r[11].s64 + 27760;
	// 82652058: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8265205C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82652060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652064: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265206C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652070: 386A4DB0  addi r3, r10, 0x4db0
	ctx.r[3].s64 = ctx.r[10].s64 + 19888;
	// 82652074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265207C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265208C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652094: 4BE14D8D  bl 0x82466e20
	ctx.lr = 0x82652098;
	sub_82466E20(ctx, base);
	// 82652098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265209C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826520A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826520A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826520A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826520A8 size=100
    let mut pc: u32 = 0x826520A8;
    'dispatch: loop {
        match pc {
            0x826520A8 => {
    //   block [0x826520A8..0x8265210C)
	// 826520A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826520AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826520B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826520B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826520B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826520BC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826520C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826520C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826520C8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826520CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826520D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826520D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826520D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826520DC: 386A4DE0  addi r3, r10, 0x4de0
	ctx.r[3].s64 = ctx.r[10].s64 + 19936;
	// 826520E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826520E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826520E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826520EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826520F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826520F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826520F8: 4BE14D29  bl 0x82466e20
	ctx.lr = 0x826520FC;
	sub_82466E20(ctx, base);
	// 826520FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652110 size=112
    let mut pc: u32 = 0x82652110;
    'dispatch: loop {
        match pc {
            0x82652110 => {
    //   block [0x82652110..0x82652180)
	// 82652110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265211C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652120: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652124: 38AA4DE0  addi r5, r10, 0x4de0
	ctx.r[5].s64 = ctx.r[10].s64 + 19936;
	// 82652128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265212C: 390B6EC8  addi r8, r11, 0x6ec8
	ctx.r[8].s64 = ctx.r[11].s64 + 28360;
	// 82652130: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82652134: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82652138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265213C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652148: 386A4E10  addi r3, r10, 0x4e10
	ctx.r[3].s64 = ctx.r[10].s64 + 19984;
	// 8265214C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265215C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265216C: 4BE14CB5  bl 0x82466e20
	ctx.lr = 0x82652170;
	sub_82466E20(ctx, base);
	// 82652170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265217C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652180 size=100
    let mut pc: u32 = 0x82652180;
    'dispatch: loop {
        match pc {
            0x82652180 => {
    //   block [0x82652180..0x826521E4)
	// 82652180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265218C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652194: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265219C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826521A0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826521A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826521A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826521AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826521B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826521B4: 386A4E40  addi r3, r10, 0x4e40
	ctx.r[3].s64 = ctx.r[10].s64 + 20032;
	// 826521B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826521BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826521C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826521C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826521C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826521CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826521D0: 4BE14C51  bl 0x82466e20
	ctx.lr = 0x826521D4;
	sub_82466E20(ctx, base);
	// 826521D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826521D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826521DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826521E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826521E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826521E8 size=108
    let mut pc: u32 = 0x826521E8;
    'dispatch: loop {
        match pc {
            0x826521E8 => {
    //   block [0x826521E8..0x82652254)
	// 826521E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826521EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826521F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826521F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826521F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826521FC: 38EB6F40  addi r7, r11, 0x6f40
	ctx.r[7].s64 = ctx.r[11].s64 + 28480;
	// 82652200: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82652204: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82652208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265220C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82652214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652218: 386A4E70  addi r3, r10, 0x4e70
	ctx.r[3].s64 = ctx.r[10].s64 + 20080;
	// 8265221C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265222C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265223C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652240: 4BE14BE1  bl 0x82466e20
	ctx.lr = 0x82652244;
	sub_82466E20(ctx, base);
	// 82652244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652258 size=112
    let mut pc: u32 = 0x82652258;
    'dispatch: loop {
        match pc {
            0x82652258 => {
    //   block [0x82652258..0x826522C8)
	// 82652258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652264: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652268: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265226C: 38AA4E40  addi r5, r10, 0x4e40
	ctx.r[5].s64 = ctx.r[10].s64 + 20032;
	// 82652270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652274: 390B6F88  addi r8, r11, 0x6f88
	ctx.r[8].s64 = ctx.r[11].s64 + 28552;
	// 82652278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265227C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82652280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652284: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265228C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652290: 386A4EA0  addi r3, r10, 0x4ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 20128;
	// 82652294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265229C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826522A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826522A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826522A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826522AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826522B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826522B4: 4BE14B6D  bl 0x82466e20
	ctx.lr = 0x826522B8;
	sub_82466E20(ctx, base);
	// 826522B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826522BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826522C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826522C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826522C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826522C8 size=100
    let mut pc: u32 = 0x826522C8;
    'dispatch: loop {
        match pc {
            0x826522C8 => {
    //   block [0x826522C8..0x8265232C)
	// 826522C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826522CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826522D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826522D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826522D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826522DC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826522E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826522E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826522E8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826522EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826522F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826522F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826522F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826522FC: 386A4ED0  addi r3, r10, 0x4ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 20176;
	// 82652300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652304: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652308: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265230C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652310: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82652314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652318: 4BE14B09  bl 0x82466e20
	ctx.lr = 0x8265231C;
	sub_82466E20(ctx, base);
	// 8265231C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652330 size=100
    let mut pc: u32 = 0x82652330;
    'dispatch: loop {
        match pc {
            0x82652330 => {
    //   block [0x82652330..0x82652394)
	// 82652330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265233C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652344: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265234C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652350: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82652354: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265235C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652364: 386A4F00  addi r3, r10, 0x4f00
	ctx.r[3].s64 = ctx.r[10].s64 + 20224;
	// 82652368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265236C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652370: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82652374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652378: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265237C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652380: 4BE14AA1  bl 0x82466e20
	ctx.lr = 0x82652384;
	sub_82466E20(ctx, base);
	// 82652384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265238C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652398 size=112
    let mut pc: u32 = 0x82652398;
    'dispatch: loop {
        match pc {
            0x82652398 => {
    //   block [0x82652398..0x82652408)
	// 82652398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265239C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826523A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826523A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826523A8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826523AC: 38AA4ED0  addi r5, r10, 0x4ed0
	ctx.r[5].s64 = ctx.r[10].s64 + 20176;
	// 826523B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826523B4: 390B6FB8  addi r8, r11, 0x6fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 28600;
	// 826523B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826523BC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826523C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826523C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826523C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826523CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826523D0: 386A4F30  addi r3, r10, 0x4f30
	ctx.r[3].s64 = ctx.r[10].s64 + 20272;
	// 826523D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826523D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826523DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826523E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826523E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826523E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826523EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826523F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826523F4: 4BE14A2D  bl 0x82466e20
	ctx.lr = 0x826523F8;
	sub_82466E20(ctx, base);
	// 826523F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826523FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652408 size=112
    let mut pc: u32 = 0x82652408;
    'dispatch: loop {
        match pc {
            0x82652408 => {
    //   block [0x82652408..0x82652478)
	// 82652408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265240C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652414: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652418: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265241C: 38AA4F00  addi r5, r10, 0x4f00
	ctx.r[5].s64 = ctx.r[10].s64 + 20224;
	// 82652420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652424: 390B7018  addi r8, r11, 0x7018
	ctx.r[8].s64 = ctx.r[11].s64 + 28696;
	// 82652428: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265242C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82652430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652434: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265243C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652440: 386A4F60  addi r3, r10, 0x4f60
	ctx.r[3].s64 = ctx.r[10].s64 + 20320;
	// 82652444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265244C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265245C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652464: 4BE149BD  bl 0x82466e20
	ctx.lr = 0x82652468;
	sub_82466E20(ctx, base);
	// 82652468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265246C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652478 size=100
    let mut pc: u32 = 0x82652478;
    'dispatch: loop {
        match pc {
            0x82652478 => {
    //   block [0x82652478..0x826524DC)
	// 82652478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265247C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652484: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265248C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652498: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8265249C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826524A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826524A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826524A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826524AC: 386A4F90  addi r3, r10, 0x4f90
	ctx.r[3].s64 = ctx.r[10].s64 + 20368;
	// 826524B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826524B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826524B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826524BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826524C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826524C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826524C8: 4BE14959  bl 0x82466e20
	ctx.lr = 0x826524CC;
	sub_82466E20(ctx, base);
	// 826524CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826524D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826524D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826524D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826524E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826524E0 size=112
    let mut pc: u32 = 0x826524E0;
    'dispatch: loop {
        match pc {
            0x826524E0 => {
    //   block [0x826524E0..0x82652550)
	// 826524E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826524E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826524E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826524EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826524F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826524F4: 38AA4F90  addi r5, r10, 0x4f90
	ctx.r[5].s64 = ctx.r[10].s64 + 20368;
	// 826524F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826524FC: 390B7078  addi r8, r11, 0x7078
	ctx.r[8].s64 = ctx.r[11].s64 + 28792;
	// 82652500: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82652504: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82652508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265250C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652518: 386A4FC0  addi r3, r10, 0x4fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 20416;
	// 8265251C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265252C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265253C: 4BE148E5  bl 0x82466e20
	ctx.lr = 0x82652540;
	sub_82466E20(ctx, base);
	// 82652540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265254C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652550 size=108
    let mut pc: u32 = 0x82652550;
    'dispatch: loop {
        match pc {
            0x82652550 => {
    //   block [0x82652550..0x826525BC)
	// 82652550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265255C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652564: 38EB7168  addi r7, r11, 0x7168
	ctx.r[7].s64 = ctx.r[11].s64 + 29032;
	// 82652568: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8265256C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82652570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265257C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652580: 386A4FF0  addi r3, r10, 0x4ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 20464;
	// 82652584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265258C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265259C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826525A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826525A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826525A8: 4BE14879  bl 0x82466e20
	ctx.lr = 0x826525AC;
	sub_82466E20(ctx, base);
	// 826525AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826525B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826525B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826525B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826525C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826525C0 size=108
    let mut pc: u32 = 0x826525C0;
    'dispatch: loop {
        match pc {
            0x826525C0 => {
    //   block [0x826525C0..0x8265262C)
	// 826525C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826525C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826525C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826525CC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826525D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826525D4: 38EB7258  addi r7, r11, 0x7258
	ctx.r[7].s64 = ctx.r[11].s64 + 29272;
	// 826525D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826525DC: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826525E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826525E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826525E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826525EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826525F0: 386A5020  addi r3, r10, 0x5020
	ctx.r[3].s64 = ctx.r[10].s64 + 20512;
	// 826525F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826525F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826525FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265260C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652618: 4BE14809  bl 0x82466e20
	ctx.lr = 0x8265261C;
	sub_82466E20(ctx, base);
	// 8265261C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652630 size=108
    let mut pc: u32 = 0x82652630;
    'dispatch: loop {
        match pc {
            0x82652630 => {
    //   block [0x82652630..0x8265269C)
	// 82652630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265263C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652644: 38EB72A0  addi r7, r11, 0x72a0
	ctx.r[7].s64 = ctx.r[11].s64 + 29344;
	// 82652648: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8265264C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82652650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652654: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265265C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652660: 386A5050  addi r3, r10, 0x5050
	ctx.r[3].s64 = ctx.r[10].s64 + 20560;
	// 82652664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265266C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265267C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652688: 4BE14799  bl 0x82466e20
	ctx.lr = 0x8265268C;
	sub_82466E20(ctx, base);
	// 8265268C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826526A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826526A0 size=108
    let mut pc: u32 = 0x826526A0;
    'dispatch: loop {
        match pc {
            0x826526A0 => {
    //   block [0x826526A0..0x8265270C)
	// 826526A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826526A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826526A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826526AC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826526B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826526B4: 38EB7378  addi r7, r11, 0x7378
	ctx.r[7].s64 = ctx.r[11].s64 + 29560;
	// 826526B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826526BC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826526C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826526C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826526C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826526CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826526D0: 386A5080  addi r3, r10, 0x5080
	ctx.r[3].s64 = ctx.r[10].s64 + 20608;
	// 826526D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826526D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826526DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826526E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826526E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826526E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826526EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826526F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826526F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826526F8: 4BE14729  bl 0x82466e20
	ctx.lr = 0x826526FC;
	sub_82466E20(ctx, base);
	// 826526FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652710 size=100
    let mut pc: u32 = 0x82652710;
    'dispatch: loop {
        match pc {
            0x82652710 => {
    //   block [0x82652710..0x82652774)
	// 82652710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265271C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652724: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652728: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265272C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652730: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82652734: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265273C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652744: 386A50B0  addi r3, r10, 0x50b0
	ctx.r[3].s64 = ctx.r[10].s64 + 20656;
	// 82652748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265274C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652750: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82652754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652758: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265275C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652760: 4BE146C1  bl 0x82466e20
	ctx.lr = 0x82652764;
	sub_82466E20(ctx, base);
	// 82652764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265276C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652778 size=112
    let mut pc: u32 = 0x82652778;
    'dispatch: loop {
        match pc {
            0x82652778 => {
    //   block [0x82652778..0x826527E8)
	// 82652778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265277C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652784: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652788: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265278C: 38AA50B0  addi r5, r10, 0x50b0
	ctx.r[5].s64 = ctx.r[10].s64 + 20656;
	// 82652790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652794: 390B7390  addi r8, r11, 0x7390
	ctx.r[8].s64 = ctx.r[11].s64 + 29584;
	// 82652798: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265279C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826527A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826527A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826527A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826527AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826527B0: 386A50E0  addi r3, r10, 0x50e0
	ctx.r[3].s64 = ctx.r[10].s64 + 20704;
	// 826527B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826527B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826527BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826527C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826527C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826527C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826527CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826527D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826527D4: 4BE1464D  bl 0x82466e20
	ctx.lr = 0x826527D8;
	sub_82466E20(ctx, base);
	// 826527D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826527DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826527E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826527E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826527E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826527E8 size=108
    let mut pc: u32 = 0x826527E8;
    'dispatch: loop {
        match pc {
            0x826527E8 => {
    //   block [0x826527E8..0x82652854)
	// 826527E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826527EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826527F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826527F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826527F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826527FC: 38EB73D8  addi r7, r11, 0x73d8
	ctx.r[7].s64 = ctx.r[11].s64 + 29656;
	// 82652800: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82652804: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82652808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265280C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82652814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652818: 386A5110  addi r3, r10, 0x5110
	ctx.r[3].s64 = ctx.r[10].s64 + 20752;
	// 8265281C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265282C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265283C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652840: 4BE145E1  bl 0x82466e20
	ctx.lr = 0x82652844;
	sub_82466E20(ctx, base);
	// 82652844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265284C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652858 size=112
    let mut pc: u32 = 0x82652858;
    'dispatch: loop {
        match pc {
            0x82652858 => {
    //   block [0x82652858..0x826528C8)
	// 82652858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265285C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652868: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265286C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652874: 390B7420  addi r8, r11, 0x7420
	ctx.r[8].s64 = ctx.r[11].s64 + 29728;
	// 82652878: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265287C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82652880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652884: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265288C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652890: 386A5140  addi r3, r10, 0x5140
	ctx.r[3].s64 = ctx.r[10].s64 + 20800;
	// 82652894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265289C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826528A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826528A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826528A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826528AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826528B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826528B4: 4BE1456D  bl 0x82466e20
	ctx.lr = 0x826528B8;
	sub_82466E20(ctx, base);
	// 826528B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826528BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826528C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826528C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826528C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826528C8 size=108
    let mut pc: u32 = 0x826528C8;
    'dispatch: loop {
        match pc {
            0x826528C8 => {
    //   block [0x826528C8..0x82652934)
	// 826528C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826528CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826528D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826528D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826528D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826528DC: 38EB7438  addi r7, r11, 0x7438
	ctx.r[7].s64 = ctx.r[11].s64 + 29752;
	// 826528E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826528E4: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826528E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826528EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826528F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826528F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826528F8: 386A5170  addi r3, r10, 0x5170
	ctx.r[3].s64 = ctx.r[10].s64 + 20848;
	// 826528FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265290C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265291C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652920: 4BE14501  bl 0x82466e20
	ctx.lr = 0x82652924;
	sub_82466E20(ctx, base);
	// 82652924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265292C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652938 size=112
    let mut pc: u32 = 0x82652938;
    'dispatch: loop {
        match pc {
            0x82652938 => {
    //   block [0x82652938..0x826529A8)
	// 82652938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265293C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652944: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652948: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8265294C: 38AA5140  addi r5, r10, 0x5140
	ctx.r[5].s64 = ctx.r[10].s64 + 20800;
	// 82652950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652954: 390B7480  addi r8, r11, 0x7480
	ctx.r[8].s64 = ctx.r[11].s64 + 29824;
	// 82652958: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265295C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82652960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265296C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652970: 386A51A0  addi r3, r10, 0x51a0
	ctx.r[3].s64 = ctx.r[10].s64 + 20896;
	// 82652974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265297C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265298C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652994: 4BE1448D  bl 0x82466e20
	ctx.lr = 0x82652998;
	sub_82466E20(ctx, base);
	// 82652998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265299C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826529A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826529A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826529A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826529A8 size=100
    let mut pc: u32 = 0x826529A8;
    'dispatch: loop {
        match pc {
            0x826529A8 => {
    //   block [0x826529A8..0x82652A0C)
	// 826529A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826529AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826529B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826529B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826529B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826529BC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 826529C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826529C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826529C8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826529CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826529D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826529D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826529D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826529DC: 386A51D0  addi r3, r10, 0x51d0
	ctx.r[3].s64 = ctx.r[10].s64 + 20944;
	// 826529E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826529E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826529E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826529EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826529F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826529F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826529F8: 4BE14429  bl 0x82466e20
	ctx.lr = 0x826529FC;
	sub_82466E20(ctx, base);
	// 826529FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652A10 size=112
    let mut pc: u32 = 0x82652A10;
    'dispatch: loop {
        match pc {
            0x82652A10 => {
    //   block [0x82652A10..0x82652A80)
	// 82652A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652A1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652A20: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652A24: 38AA51D0  addi r5, r10, 0x51d0
	ctx.r[5].s64 = ctx.r[10].s64 + 20944;
	// 82652A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652A2C: 390B7498  addi r8, r11, 0x7498
	ctx.r[8].s64 = ctx.r[11].s64 + 29848;
	// 82652A30: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82652A34: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82652A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652A3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652A48: 386A5200  addi r3, r10, 0x5200
	ctx.r[3].s64 = ctx.r[10].s64 + 20992;
	// 82652A4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652A6C: 4BE143B5  bl 0x82466e20
	ctx.lr = 0x82652A70;
	sub_82466E20(ctx, base);
	// 82652A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652A80 size=108
    let mut pc: u32 = 0x82652A80;
    'dispatch: loop {
        match pc {
            0x82652A80 => {
    //   block [0x82652A80..0x82652AEC)
	// 82652A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652A8C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652A94: 38EB7540  addi r7, r11, 0x7540
	ctx.r[7].s64 = ctx.r[11].s64 + 30016;
	// 82652A98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82652A9C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 82652AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82652AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652AB0: 386A5230  addi r3, r10, 0x5230
	ctx.r[3].s64 = ctx.r[10].s64 + 21040;
	// 82652AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82652AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82652AD8: 4BE14349  bl 0x82466e20
	ctx.lr = 0x82652ADC;
	sub_82466E20(ctx, base);
	// 82652ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652AF0 size=112
    let mut pc: u32 = 0x82652AF0;
    'dispatch: loop {
        match pc {
            0x82652AF0 => {
    //   block [0x82652AF0..0x82652B60)
	// 82652AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652AFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652B00: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652B04: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652B0C: 390B7570  addi r8, r11, 0x7570
	ctx.r[8].s64 = ctx.r[11].s64 + 30064;
	// 82652B10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82652B14: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82652B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652B1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652B20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652B28: 386A5260  addi r3, r10, 0x5260
	ctx.r[3].s64 = ctx.r[10].s64 + 21088;
	// 82652B2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652B4C: 4BE142D5  bl 0x82466e20
	ctx.lr = 0x82652B50;
	sub_82466E20(ctx, base);
	// 82652B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652B54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652B58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652B60 size=112
    let mut pc: u32 = 0x82652B60;
    'dispatch: loop {
        match pc {
            0x82652B60 => {
    //   block [0x82652B60..0x82652BD0)
	// 82652B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652B6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652B70: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652B74: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652B7C: 390B75B8  addi r8, r11, 0x75b8
	ctx.r[8].s64 = ctx.r[11].s64 + 30136;
	// 82652B80: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82652B84: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 82652B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652B8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652B98: 386A5290  addi r3, r10, 0x5290
	ctx.r[3].s64 = ctx.r[10].s64 + 21136;
	// 82652B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652BBC: 4BE14265  bl 0x82466e20
	ctx.lr = 0x82652BC0;
	sub_82466E20(ctx, base);
	// 82652BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652BD0 size=100
    let mut pc: u32 = 0x82652BD0;
    'dispatch: loop {
        match pc {
            0x82652BD0 => {
    //   block [0x82652BD0..0x82652C34)
	// 82652BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652BDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652BE4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652BF0: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82652BF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652C04: 386A52C0  addi r3, r10, 0x52c0
	ctx.r[3].s64 = ctx.r[10].s64 + 21184;
	// 82652C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652C0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652C10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82652C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652C18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82652C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652C20: 4BE14201  bl 0x82466e20
	ctx.lr = 0x82652C24;
	sub_82466E20(ctx, base);
	// 82652C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652C38 size=112
    let mut pc: u32 = 0x82652C38;
    'dispatch: loop {
        match pc {
            0x82652C38 => {
    //   block [0x82652C38..0x82652CA8)
	// 82652C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652C44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652C48: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652C4C: 38AA52C0  addi r5, r10, 0x52c0
	ctx.r[5].s64 = ctx.r[10].s64 + 21184;
	// 82652C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652C54: 390B7600  addi r8, r11, 0x7600
	ctx.r[8].s64 = ctx.r[11].s64 + 30208;
	// 82652C58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82652C5C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82652C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652C64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652C70: 386A52F0  addi r3, r10, 0x52f0
	ctx.r[3].s64 = ctx.r[10].s64 + 21232;
	// 82652C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652C94: 4BE1418D  bl 0x82466e20
	ctx.lr = 0x82652C98;
	sub_82466E20(ctx, base);
	// 82652C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652CA8 size=112
    let mut pc: u32 = 0x82652CA8;
    'dispatch: loop {
        match pc {
            0x82652CA8 => {
    //   block [0x82652CA8..0x82652D18)
	// 82652CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652CB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652CB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652CBC: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652CC4: 390B7648  addi r8, r11, 0x7648
	ctx.r[8].s64 = ctx.r[11].s64 + 30280;
	// 82652CC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82652CCC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82652CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652CD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652CE0: 386A5320  addi r3, r10, 0x5320
	ctx.r[3].s64 = ctx.r[10].s64 + 21280;
	// 82652CE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652D04: 4BE1411D  bl 0x82466e20
	ctx.lr = 0x82652D08;
	sub_82466E20(ctx, base);
	// 82652D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652D18 size=112
    let mut pc: u32 = 0x82652D18;
    'dispatch: loop {
        match pc {
            0x82652D18 => {
    //   block [0x82652D18..0x82652D88)
	// 82652D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652D24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652D28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652D2C: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82652D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652D34: 390B7660  addi r8, r11, 0x7660
	ctx.r[8].s64 = ctx.r[11].s64 + 30304;
	// 82652D38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82652D3C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82652D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652D44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652D50: 386A5350  addi r3, r10, 0x5350
	ctx.r[3].s64 = ctx.r[10].s64 + 21328;
	// 82652D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652D64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82652D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652D74: 4BE140AD  bl 0x82466e20
	ctx.lr = 0x82652D78;
	sub_82466E20(ctx, base);
	// 82652D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82652D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82652D88 size=112
    let mut pc: u32 = 0x82652D88;
    'dispatch: loop {
        match pc {
            0x82652D88 => {
    //   block [0x82652D88..0x82652DF8)
	// 82652D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82652D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82652D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82652D94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652D98: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82652D9C: 38AA5320  addi r5, r10, 0x5320
	ctx.r[5].s64 = ctx.r[10].s64 + 21280;
	// 82652DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82652DA4: 390B7678  addi r8, r11, 0x7678
	ctx.r[8].s64 = ctx.r[11].s64 + 30328;
	// 82652DA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82652DAC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 82652DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82652DB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82652DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82652DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82652DC0: 386A5380  addi r3, r10, 0x5380
	ctx.r[3].s64 = ctx.r[10].s64 + 21376;
	// 82652DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82652DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82652DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82652DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82652DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82652DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82652DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82652DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82652DE4: 4BE1403D  bl 0x82466e20
	ctx.lr = 0x82652DE8;
	sub_82466E20(ctx, base);
	// 82652DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82652DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82652DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82652DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


