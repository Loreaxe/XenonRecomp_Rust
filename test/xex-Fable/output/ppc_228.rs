pub fn sub_8329A0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A0A0 size=108
    let mut pc: u32 = 0x8329A0A0;
    'dispatch: loop {
        match pc {
            0x8329A0A0 => {
    //   block [0x8329A0A0..0x8329A10C)
	// 8329A0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A0AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A0B4: 38EB6064  addi r7, r11, 0x6064
	ctx.r[7].s64 = ctx.r[11].s64 + 24676;
	// 8329A0B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A0BC: 388A60C4  addi r4, r10, 0x60c4
	ctx.r[4].s64 = ctx.r[10].s64 + 24772;
	// 8329A0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A0C4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A0C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A0D0: 386A7A74  addi r3, r10, 0x7a74
	ctx.r[3].s64 = ctx.r[10].s64 + 31348;
	// 8329A0D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A0E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A0EC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A0F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A0F8: 4BABBB49  bl 0x82d55c40
	ctx.lr = 0x8329A0FC;
	sub_82D55C40(ctx, base);
	// 8329A0FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A110 size=108
    let mut pc: u32 = 0x8329A110;
    'dispatch: loop {
        match pc {
            0x8329A110 => {
    //   block [0x8329A110..0x8329A17C)
	// 8329A110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A11C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A124: 38EB6094  addi r7, r11, 0x6094
	ctx.r[7].s64 = ctx.r[11].s64 + 24724;
	// 8329A128: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A12C: 388A60E4  addi r4, r10, 0x60e4
	ctx.r[4].s64 = ctx.r[10].s64 + 24804;
	// 8329A130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A134: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A140: 386A7AA4  addi r3, r10, 0x7aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 31396;
	// 8329A144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A15C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329A160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A168: 4BABBAD9  bl 0x82d55c40
	ctx.lr = 0x8329A16C;
	sub_82D55C40(ctx, base);
	// 8329A16C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A180 size=108
    let mut pc: u32 = 0x8329A180;
    'dispatch: loop {
        match pc {
            0x8329A180 => {
    //   block [0x8329A180..0x8329A1EC)
	// 8329A180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A18C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A194: 38EB6108  addi r7, r11, 0x6108
	ctx.r[7].s64 = ctx.r[11].s64 + 24840;
	// 8329A198: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A19C: 388A6138  addi r4, r10, 0x6138
	ctx.r[4].s64 = ctx.r[10].s64 + 24888;
	// 8329A1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A1A4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A1B0: 386A7AD4  addi r3, r10, 0x7ad4
	ctx.r[3].s64 = ctx.r[10].s64 + 31444;
	// 8329A1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A1CC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A1D8: 4BABBA69  bl 0x82d55c40
	ctx.lr = 0x8329A1DC;
	sub_82D55C40(ctx, base);
	// 8329A1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A1F0 size=108
    let mut pc: u32 = 0x8329A1F0;
    'dispatch: loop {
        match pc {
            0x8329A1F0 => {
    //   block [0x8329A1F0..0x8329A25C)
	// 8329A1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A1FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A204: 38EB6170  addi r7, r11, 0x6170
	ctx.r[7].s64 = ctx.r[11].s64 + 24944;
	// 8329A208: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329A20C: 388A61D0  addi r4, r10, 0x61d0
	ctx.r[4].s64 = ctx.r[10].s64 + 25040;
	// 8329A210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A214: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A220: 386A7B04  addi r3, r10, 0x7b04
	ctx.r[3].s64 = ctx.r[10].s64 + 31492;
	// 8329A224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A23C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329A240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A248: 4BABB9F9  bl 0x82d55c40
	ctx.lr = 0x8329A24C;
	sub_82D55C40(ctx, base);
	// 8329A24C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A260 size=100
    let mut pc: u32 = 0x8329A260;
    'dispatch: loop {
        match pc {
            0x8329A260 => {
    //   block [0x8329A260..0x8329A2C4)
	// 8329A260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A26C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329A270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A274: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329A278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A280: 388A61E4  addi r4, r10, 0x61e4
	ctx.r[4].s64 = ctx.r[10].s64 + 25060;
	// 8329A284: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A294: 386A7B34  addi r3, r10, 0x7b34
	ctx.r[3].s64 = ctx.r[10].s64 + 31540;
	// 8329A298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A29C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A2A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329A2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A2A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329A2AC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A2B0: 4BABB991  bl 0x82d55c40
	ctx.lr = 0x8329A2B4;
	sub_82D55C40(ctx, base);
	// 8329A2B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A2B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A2BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A2C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A2C8 size=24
    let mut pc: u32 = 0x8329A2C8;
    'dispatch: loop {
        match pc {
            0x8329A2C8 => {
    //   block [0x8329A2C8..0x8329A2E0)
	// 8329A2C8: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A2CC: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 8329A2D0: 394A7A98  addi r10, r10, 0x7a98
	ctx.r[10].s64 = ctx.r[10].s64 + 31384;
	// 8329A2D4: 816B7A90  lwz r11, 0x7a90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31376 as u32) ) } as u64;
	// 8329A2D8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329A2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A2E0 size=116
    let mut pc: u32 = 0x8329A2E0;
    'dispatch: loop {
        match pc {
            0x8329A2E0 => {
    //   block [0x8329A2E0..0x8329A354)
	// 8329A2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A2EC: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A2F4: 390B7A98  addi r8, r11, 0x7a98
	ctx.r[8].s64 = ctx.r[11].s64 + 31384;
	// 8329A2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A2FC: 392A62C4  addi r9, r10, 0x62c4
	ctx.r[9].s64 = ctx.r[10].s64 + 25284;
	// 8329A300: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329A304: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329A308: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329A30C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329A310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A314: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A324: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 8329A328: 388A630C  addi r4, r10, 0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + 25356;
	// 8329A32C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329A330: 386B7B64  addi r3, r11, 0x7b64
	ctx.r[3].s64 = ctx.r[11].s64 + 31588;
	// 8329A334: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329A338: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A33C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329A340: 4BABB901  bl 0x82d55c40
	ctx.lr = 0x8329A344;
	sub_82D55C40(ctx, base);
	// 8329A344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A358 size=108
    let mut pc: u32 = 0x8329A358;
    'dispatch: loop {
        match pc {
            0x8329A358 => {
    //   block [0x8329A358..0x8329A3C4)
	// 8329A358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A364: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A36C: 38EB69F8  addi r7, r11, 0x69f8
	ctx.r[7].s64 = ctx.r[11].s64 + 27128;
	// 8329A370: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329A374: 388A6A58  addi r4, r10, 0x6a58
	ctx.r[4].s64 = ctx.r[10].s64 + 27224;
	// 8329A378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A37C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A388: 386A7B94  addi r3, r10, 0x7b94
	ctx.r[3].s64 = ctx.r[10].s64 + 31636;
	// 8329A38C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A3A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A3A4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329A3A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A3AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A3B0: 4BABB891  bl 0x82d55c40
	ctx.lr = 0x8329A3B4;
	sub_82D55C40(ctx, base);
	// 8329A3B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A3C8 size=108
    let mut pc: u32 = 0x8329A3C8;
    'dispatch: loop {
        match pc {
            0x8329A3C8 => {
    //   block [0x8329A3C8..0x8329A434)
	// 8329A3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A3D4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A3DC: 38EB6A40  addi r7, r11, 0x6a40
	ctx.r[7].s64 = ctx.r[11].s64 + 27200;
	// 8329A3E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329A3E4: 388A6A7C  addi r4, r10, 0x6a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 27260;
	// 8329A3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A3EC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A3F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A3F8: 386A7BC4  addi r3, r10, 0x7bc4
	ctx.r[3].s64 = ctx.r[10].s64 + 31684;
	// 8329A3FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A414: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A41C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A420: 4BABB821  bl 0x82d55c40
	ctx.lr = 0x8329A424;
	sub_82D55C40(ctx, base);
	// 8329A424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A438 size=36
    let mut pc: u32 = 0x8329A438;
    'dispatch: loop {
        match pc {
            0x8329A438 => {
    //   block [0x8329A438..0x8329A45C)
	// 8329A438: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A43C: 816B6A94  lwz r11, 0x6a94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27284 as u32) ) } as u64;
	// 8329A440: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 8329A444: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8329A448: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A44C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8329A450: 396B7D58  addi r11, r11, 0x7d58
	ctx.r[11].s64 = ctx.r[11].s64 + 32088;
	// 8329A454: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 8329A458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A460 size=28
    let mut pc: u32 = 0x8329A460;
    'dispatch: loop {
        match pc {
            0x8329A460 => {
    //   block [0x8329A460..0x8329A47C)
	// 8329A460: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A464: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A468: 396B7DE4  addi r11, r11, 0x7de4
	ctx.r[11].s64 = ctx.r[11].s64 + 32228;
	// 8329A46C: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 8329A470: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8329A474: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8329A478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A480 size=28
    let mut pc: u32 = 0x8329A480;
    'dispatch: loop {
        match pc {
            0x8329A480 => {
    //   block [0x8329A480..0x8329A49C)
	// 8329A480: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A484: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A488: 396B7E9C  addi r11, r11, 0x7e9c
	ctx.r[11].s64 = ctx.r[11].s64 + 32412;
	// 8329A48C: 812A76FC  lwz r9, 0x76fc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(30460 as u32) ) } as u64;
	// 8329A490: 916A76FC  stw r11, 0x76fc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(30460 as u32), ctx.r[11].u32 ) };
	// 8329A494: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8329A498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A4A0 size=108
    let mut pc: u32 = 0x8329A4A0;
    'dispatch: loop {
        match pc {
            0x8329A4A0 => {
    //   block [0x8329A4A0..0x8329A50C)
	// 8329A4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A4AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A4B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A4B4: 38EB6E14  addi r7, r11, 0x6e14
	ctx.r[7].s64 = ctx.r[11].s64 + 28180;
	// 8329A4B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A4BC: 388A6E44  addi r4, r10, 0x6e44
	ctx.r[4].s64 = ctx.r[10].s64 + 28228;
	// 8329A4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A4C4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A4D0: 386A7BFC  addi r3, r10, 0x7bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 31740;
	// 8329A4D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A4EC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329A4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A4F8: 4BABB749  bl 0x82d55c40
	ctx.lr = 0x8329A4FC;
	sub_82D55C40(ctx, base);
	// 8329A4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A510 size=108
    let mut pc: u32 = 0x8329A510;
    'dispatch: loop {
        match pc {
            0x8329A510 => {
    //   block [0x8329A510..0x8329A57C)
	// 8329A510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A51C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A524: 38EB6E6C  addi r7, r11, 0x6e6c
	ctx.r[7].s64 = ctx.r[11].s64 + 28268;
	// 8329A528: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329A52C: 388A6E84  addi r4, r10, 0x6e84
	ctx.r[4].s64 = ctx.r[10].s64 + 28292;
	// 8329A530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A534: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A540: 386A7C2C  addi r3, r10, 0x7c2c
	ctx.r[3].s64 = ctx.r[10].s64 + 31788;
	// 8329A544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A55C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A568: 4BABB6D9  bl 0x82d55c40
	ctx.lr = 0x8329A56C;
	sub_82D55C40(ctx, base);
	// 8329A56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A580 size=108
    let mut pc: u32 = 0x8329A580;
    'dispatch: loop {
        match pc {
            0x8329A580 => {
    //   block [0x8329A580..0x8329A5EC)
	// 8329A580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A58C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A594: 38EB6EAC  addi r7, r11, 0x6eac
	ctx.r[7].s64 = ctx.r[11].s64 + 28332;
	// 8329A598: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A59C: 388A6EDC  addi r4, r10, 0x6edc
	ctx.r[4].s64 = ctx.r[10].s64 + 28380;
	// 8329A5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A5A4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A5B0: 386A7C5C  addi r3, r10, 0x7c5c
	ctx.r[3].s64 = ctx.r[10].s64 + 31836;
	// 8329A5B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A5CC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329A5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A5D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A5D8: 4BABB669  bl 0x82d55c40
	ctx.lr = 0x8329A5DC;
	sub_82D55C40(ctx, base);
	// 8329A5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A5F0 size=108
    let mut pc: u32 = 0x8329A5F0;
    'dispatch: loop {
        match pc {
            0x8329A5F0 => {
    //   block [0x8329A5F0..0x8329A65C)
	// 8329A5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A5F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A5FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A604: 38EB6EFC  addi r7, r11, 0x6efc
	ctx.r[7].s64 = ctx.r[11].s64 + 28412;
	// 8329A608: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329A60C: 388A6F14  addi r4, r10, 0x6f14
	ctx.r[4].s64 = ctx.r[10].s64 + 28436;
	// 8329A610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A614: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A620: 386A7C8C  addi r3, r10, 0x7c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 31884;
	// 8329A624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A63C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A648: 4BABB5F9  bl 0x82d55c40
	ctx.lr = 0x8329A64C;
	sub_82D55C40(ctx, base);
	// 8329A64C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A660 size=108
    let mut pc: u32 = 0x8329A660;
    'dispatch: loop {
        match pc {
            0x8329A660 => {
    //   block [0x8329A660..0x8329A6CC)
	// 8329A660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A66C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A674: 38EB7000  addi r7, r11, 0x7000
	ctx.r[7].s64 = ctx.r[11].s64 + 28672;
	// 8329A678: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8329A67C: 388A7120  addi r4, r10, 0x7120
	ctx.r[4].s64 = ctx.r[10].s64 + 28960;
	// 8329A680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A684: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A690: 386A7CBC  addi r3, r10, 0x7cbc
	ctx.r[3].s64 = ctx.r[10].s64 + 31932;
	// 8329A694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A6AC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329A6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A6B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A6B8: 4BABB589  bl 0x82d55c40
	ctx.lr = 0x8329A6BC;
	sub_82D55C40(ctx, base);
	// 8329A6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A6D0 size=108
    let mut pc: u32 = 0x8329A6D0;
    'dispatch: loop {
        match pc {
            0x8329A6D0 => {
    //   block [0x8329A6D0..0x8329A73C)
	// 8329A6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A6DC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A6E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A6E4: 38EB7140  addi r7, r11, 0x7140
	ctx.r[7].s64 = ctx.r[11].s64 + 28992;
	// 8329A6E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329A6EC: 388A7158  addi r4, r10, 0x7158
	ctx.r[4].s64 = ctx.r[10].s64 + 29016;
	// 8329A6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A6F4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A6F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A6FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A700: 386A7CEC  addi r3, r10, 0x7cec
	ctx.r[3].s64 = ctx.r[10].s64 + 31980;
	// 8329A704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A71C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329A720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A728: 4BABB519  bl 0x82d55c40
	ctx.lr = 0x8329A72C;
	sub_82D55C40(ctx, base);
	// 8329A72C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A740 size=108
    let mut pc: u32 = 0x8329A740;
    'dispatch: loop {
        match pc {
            0x8329A740 => {
    //   block [0x8329A740..0x8329A7AC)
	// 8329A740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A74C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A754: 38EB7188  addi r7, r11, 0x7188
	ctx.r[7].s64 = ctx.r[11].s64 + 29064;
	// 8329A758: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8329A75C: 388A7218  addi r4, r10, 0x7218
	ctx.r[4].s64 = ctx.r[10].s64 + 29208;
	// 8329A760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A764: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A770: 386A7D1C  addi r3, r10, 0x7d1c
	ctx.r[3].s64 = ctx.r[10].s64 + 32028;
	// 8329A774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A78C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329A790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A798: 4BABB4A9  bl 0x82d55c40
	ctx.lr = 0x8329A79C;
	sub_82D55C40(ctx, base);
	// 8329A79C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A7A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A7A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A7A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A7B0 size=108
    let mut pc: u32 = 0x8329A7B0;
    'dispatch: loop {
        match pc {
            0x8329A7B0 => {
    //   block [0x8329A7B0..0x8329A81C)
	// 8329A7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A7BC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A7C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A7C4: 38EB7230  addi r7, r11, 0x7230
	ctx.r[7].s64 = ctx.r[11].s64 + 29232;
	// 8329A7C8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8329A7CC: 388A72F0  addi r4, r10, 0x72f0
	ctx.r[4].s64 = ctx.r[10].s64 + 29424;
	// 8329A7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A7D4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A7D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A7E0: 386A7D4C  addi r3, r10, 0x7d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 32076;
	// 8329A7E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A7E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A7FC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329A800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A808: 4BABB439  bl 0x82d55c40
	ctx.lr = 0x8329A80C;
	sub_82D55C40(ctx, base);
	// 8329A80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A820 size=24
    let mut pc: u32 = 0x8329A820;
    'dispatch: loop {
        match pc {
            0x8329A820 => {
    //   block [0x8329A820..0x8329A838)
	// 8329A820: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329A824: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 8329A828: 394A7FDC  addi r10, r10, 0x7fdc
	ctx.r[10].s64 = ctx.r[10].s64 + 32732;
	// 8329A82C: 816B80DC  lwz r11, -0x7f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32548 as u32) ) } as u64;
	// 8329A830: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329A834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A838 size=108
    let mut pc: u32 = 0x8329A838;
    'dispatch: loop {
        match pc {
            0x8329A838 => {
    //   block [0x8329A838..0x8329A8A4)
	// 8329A838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A844: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 8329A848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A84C: 38EB7FDC  addi r7, r11, 0x7fdc
	ctx.r[7].s64 = ctx.r[11].s64 + 32732;
	// 8329A850: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A854: 388A7318  addi r4, r10, 0x7318
	ctx.r[4].s64 = ctx.r[10].s64 + 29464;
	// 8329A858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A85C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A860: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A868: 386A7D7C  addi r3, r10, 0x7d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 32124;
	// 8329A86C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A884: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329A888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A88C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A890: 4BABB3B1  bl 0x82d55c40
	ctx.lr = 0x8329A894;
	sub_82D55C40(ctx, base);
	// 8329A894: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A89C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A8A8 size=108
    let mut pc: u32 = 0x8329A8A8;
    'dispatch: loop {
        match pc {
            0x8329A8A8 => {
    //   block [0x8329A8A8..0x8329A914)
	// 8329A8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A8B4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A8BC: 38EB7348  addi r7, r11, 0x7348
	ctx.r[7].s64 = ctx.r[11].s64 + 29512;
	// 8329A8C0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8329A8C4: 388A7420  addi r4, r10, 0x7420
	ctx.r[4].s64 = ctx.r[10].s64 + 29728;
	// 8329A8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A8CC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A8D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A8D8: 386A7DAC  addi r3, r10, 0x7dac
	ctx.r[3].s64 = ctx.r[10].s64 + 32172;
	// 8329A8DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A8F4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329A8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A8FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A900: 4BABB341  bl 0x82d55c40
	ctx.lr = 0x8329A904;
	sub_82D55C40(ctx, base);
	// 8329A904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329A918 size=24
    let mut pc: u32 = 0x8329A918;
    'dispatch: loop {
        match pc {
            0x8329A918 => {
    //   block [0x8329A918..0x8329A930)
	// 8329A918: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329A91C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329A920: 394A802C  addi r10, r10, -0x7fd4
	ctx.r[10].s64 = ctx.r[10].s64 + -32724;
	// 8329A924: 816B80DC  lwz r11, -0x7f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32548 as u32) ) } as u64;
	// 8329A928: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329A92C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A930 size=108
    let mut pc: u32 = 0x8329A930;
    'dispatch: loop {
        match pc {
            0x8329A930 => {
    //   block [0x8329A930..0x8329A99C)
	// 8329A930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A93C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329A940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A944: 38EB802C  addi r7, r11, -0x7fd4
	ctx.r[7].s64 = ctx.r[11].s64 + -32724;
	// 8329A948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329A94C: 388A7448  addi r4, r10, 0x7448
	ctx.r[4].s64 = ctx.r[10].s64 + 29768;
	// 8329A950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A954: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A960: 386A7DDC  addi r3, r10, 0x7ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 32220;
	// 8329A964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A97C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329A980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A988: 4BABB2B9  bl 0x82d55c40
	ctx.lr = 0x8329A98C;
	sub_82D55C40(ctx, base);
	// 8329A98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329A990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329A994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329A998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329A9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329A9A0 size=108
    let mut pc: u32 = 0x8329A9A0;
    'dispatch: loop {
        match pc {
            0x8329A9A0 => {
    //   block [0x8329A9A0..0x8329AA0C)
	// 8329A9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329A9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329A9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329A9AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329A9B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329A9B4: 38EB7468  addi r7, r11, 0x7468
	ctx.r[7].s64 = ctx.r[11].s64 + 29800;
	// 8329A9B8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8329A9BC: 388A7528  addi r4, r10, 0x7528
	ctx.r[4].s64 = ctx.r[10].s64 + 29992;
	// 8329A9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329A9C4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329A9C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329A9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329A9D0: 386A7E0C  addi r3, r10, 0x7e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 32268;
	// 8329A9D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329A9D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329A9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329A9E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329A9E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329A9E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329A9EC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329A9F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329A9F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329A9F8: 4BABB249  bl 0x82d55c40
	ctx.lr = 0x8329A9FC;
	sub_82D55C40(ctx, base);
	// 8329A9FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AA10 size=108
    let mut pc: u32 = 0x8329AA10;
    'dispatch: loop {
        match pc {
            0x8329AA10 => {
    //   block [0x8329AA10..0x8329AA7C)
	// 8329AA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AA1C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AA20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AA24: 38EB754C  addi r7, r11, 0x754c
	ctx.r[7].s64 = ctx.r[11].s64 + 30028;
	// 8329AA28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AA2C: 388A7564  addi r4, r10, 0x7564
	ctx.r[4].s64 = ctx.r[10].s64 + 30052;
	// 8329AA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AA34: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AA38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AA40: 386A7E3C  addi r3, r10, 0x7e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 32316;
	// 8329AA44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AA5C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329AA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AA64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AA68: 4BABB1D9  bl 0x82d55c40
	ctx.lr = 0x8329AA6C;
	sub_82D55C40(ctx, base);
	// 8329AA6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AA70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AA74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AA80 size=108
    let mut pc: u32 = 0x8329AA80;
    'dispatch: loop {
        match pc {
            0x8329AA80 => {
    //   block [0x8329AA80..0x8329AAEC)
	// 8329AA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AA8C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AA90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AA94: 38EB7580  addi r7, r11, 0x7580
	ctx.r[7].s64 = ctx.r[11].s64 + 30080;
	// 8329AA98: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8329AA9C: 388A7628  addi r4, r10, 0x7628
	ctx.r[4].s64 = ctx.r[10].s64 + 30248;
	// 8329AAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AAA4: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AAA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AAB0: 386A7E6C  addi r3, r10, 0x7e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 32364;
	// 8329AAB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AACC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329AAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AAD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AAD8: 4BABB169  bl 0x82d55c40
	ctx.lr = 0x8329AADC;
	sub_82D55C40(ctx, base);
	// 8329AADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329AAF0 size=24
    let mut pc: u32 = 0x8329AAF0;
    'dispatch: loop {
        match pc {
            0x8329AAF0 => {
    //   block [0x8329AAF0..0x8329AB08)
	// 8329AAF0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329AAF4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329AAF8: 394A809C  addi r10, r10, -0x7f64
	ctx.r[10].s64 = ctx.r[10].s64 + -32612;
	// 8329AAFC: 816B80DC  lwz r11, -0x7f24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32548 as u32) ) } as u64;
	// 8329AB00: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329AB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AB08 size=108
    let mut pc: u32 = 0x8329AB08;
    'dispatch: loop {
        match pc {
            0x8329AB08 => {
    //   block [0x8329AB08..0x8329AB74)
	// 8329AB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AB14: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329AB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AB1C: 38EB809C  addi r7, r11, -0x7f64
	ctx.r[7].s64 = ctx.r[11].s64 + -32612;
	// 8329AB20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329AB24: 388A7648  addi r4, r10, 0x7648
	ctx.r[4].s64 = ctx.r[10].s64 + 30280;
	// 8329AB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AB2C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AB30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AB38: 386A7E9C  addi r3, r10, 0x7e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 32412;
	// 8329AB3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AB48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AB50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AB54: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329AB58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AB5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AB60: 4BABB0E1  bl 0x82d55c40
	ctx.lr = 0x8329AB64;
	sub_82D55C40(ctx, base);
	// 8329AB64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AB68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AB6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AB78 size=108
    let mut pc: u32 = 0x8329AB78;
    'dispatch: loop {
        match pc {
            0x8329AB78 => {
    //   block [0x8329AB78..0x8329ABE4)
	// 8329AB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AB84: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AB8C: 38EB7670  addi r7, r11, 0x7670
	ctx.r[7].s64 = ctx.r[11].s64 + 30320;
	// 8329AB90: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AB94: 388A7688  addi r4, r10, 0x7688
	ctx.r[4].s64 = ctx.r[10].s64 + 30344;
	// 8329AB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AB9C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329ABA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329ABA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329ABA8: 386A7ECC  addi r3, r10, 0x7ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 32460;
	// 8329ABAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329ABB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329ABB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329ABB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329ABBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329ABC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329ABC4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329ABC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329ABCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329ABD0: 4BABB071  bl 0x82d55c40
	ctx.lr = 0x8329ABD4;
	sub_82D55C40(ctx, base);
	// 8329ABD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329ABD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329ABDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ABE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ABE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ABE8 size=108
    let mut pc: u32 = 0x8329ABE8;
    'dispatch: loop {
        match pc {
            0x8329ABE8 => {
    //   block [0x8329ABE8..0x8329AC54)
	// 8329ABE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ABEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ABF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ABF4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329ABF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329ABFC: 392B7730  addi r9, r11, 0x7730
	ctx.r[9].s64 = ctx.r[11].s64 + 30512;
	// 8329AC00: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8329AC04: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 8329AC08: 388A7774  addi r4, r10, 0x7774
	ctx.r[4].s64 = ctx.r[10].s64 + 30580;
	// 8329AC0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AC10: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AC14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329AC18: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329AC1C: 386A7EFC  addi r3, r10, 0x7efc
	ctx.r[3].s64 = ctx.r[10].s64 + 32508;
	// 8329AC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329AC24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329AC28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AC2C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AC30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AC34: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AC38: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AC3C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AC40: 4BABB001  bl 0x82d55c40
	ctx.lr = 0x8329AC44;
	sub_82D55C40(ctx, base);
	// 8329AC44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AC48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AC4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AC58 size=108
    let mut pc: u32 = 0x8329AC58;
    'dispatch: loop {
        match pc {
            0x8329AC58 => {
    //   block [0x8329AC58..0x8329ACC4)
	// 8329AC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AC64: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AC6C: 38EB7798  addi r7, r11, 0x7798
	ctx.r[7].s64 = ctx.r[11].s64 + 30616;
	// 8329AC70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AC74: 388A77B0  addi r4, r10, 0x77b0
	ctx.r[4].s64 = ctx.r[10].s64 + 30640;
	// 8329AC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AC7C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AC80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AC88: 386A7F2C  addi r3, r10, 0x7f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 32556;
	// 8329AC8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AC90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AC94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329ACA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329ACA4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329ACA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329ACAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329ACB0: 4BABAF91  bl 0x82d55c40
	ctx.lr = 0x8329ACB4;
	sub_82D55C40(ctx, base);
	// 8329ACB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329ACB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329ACBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ACC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ACC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ACC8 size=108
    let mut pc: u32 = 0x8329ACC8;
    'dispatch: loop {
        match pc {
            0x8329ACC8 => {
    //   block [0x8329ACC8..0x8329AD34)
	// 8329ACC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ACCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ACD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ACD4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329ACD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329ACDC: 38EB77CC  addi r7, r11, 0x77cc
	ctx.r[7].s64 = ctx.r[11].s64 + 30668;
	// 8329ACE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329ACE4: 388A77FC  addi r4, r10, 0x77fc
	ctx.r[4].s64 = ctx.r[10].s64 + 30716;
	// 8329ACE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329ACEC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329ACF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329ACF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329ACF8: 386A7F5C  addi r3, r10, 0x7f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 32604;
	// 8329ACFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AD14: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329AD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AD20: 4BABAF21  bl 0x82d55c40
	ctx.lr = 0x8329AD24;
	sub_82D55C40(ctx, base);
	// 8329AD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AD38 size=108
    let mut pc: u32 = 0x8329AD38;
    'dispatch: loop {
        match pc {
            0x8329AD38 => {
    //   block [0x8329AD38..0x8329ADA4)
	// 8329AD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AD44: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AD48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AD4C: 38EB7820  addi r7, r11, 0x7820
	ctx.r[7].s64 = ctx.r[11].s64 + 30752;
	// 8329AD50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AD54: 388A7838  addi r4, r10, 0x7838
	ctx.r[4].s64 = ctx.r[10].s64 + 30776;
	// 8329AD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AD5C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AD60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AD68: 386A7F8C  addi r3, r10, 0x7f8c
	ctx.r[3].s64 = ctx.r[10].s64 + 32652;
	// 8329AD6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AD74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AD7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AD84: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329AD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AD8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AD90: 4BABAEB1  bl 0x82d55c40
	ctx.lr = 0x8329AD94;
	sub_82D55C40(ctx, base);
	// 8329AD94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AD98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AD9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ADA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ADA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ADA8 size=108
    let mut pc: u32 = 0x8329ADA8;
    'dispatch: loop {
        match pc {
            0x8329ADA8 => {
    //   block [0x8329ADA8..0x8329AE14)
	// 8329ADA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ADAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ADB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ADB4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329ADB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329ADBC: 38EB7870  addi r7, r11, 0x7870
	ctx.r[7].s64 = ctx.r[11].s64 + 30832;
	// 8329ADC0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8329ADC4: 388A7918  addi r4, r10, 0x7918
	ctx.r[4].s64 = ctx.r[10].s64 + 31000;
	// 8329ADC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329ADCC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329ADD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329ADD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329ADD8: 386A7FBC  addi r3, r10, 0x7fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 32700;
	// 8329ADDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329ADE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329ADE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329ADE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329ADEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329ADF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329ADF4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329ADF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329ADFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AE00: 4BABAE41  bl 0x82d55c40
	ctx.lr = 0x8329AE04;
	sub_82D55C40(ctx, base);
	// 8329AE04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AE18 size=108
    let mut pc: u32 = 0x8329AE18;
    'dispatch: loop {
        match pc {
            0x8329AE18 => {
    //   block [0x8329AE18..0x8329AE84)
	// 8329AE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AE24: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AE2C: 38EB7934  addi r7, r11, 0x7934
	ctx.r[7].s64 = ctx.r[11].s64 + 31028;
	// 8329AE30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AE34: 388A794C  addi r4, r10, 0x794c
	ctx.r[4].s64 = ctx.r[10].s64 + 31052;
	// 8329AE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AE3C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329AE40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AE48: 386A7FEC  addi r3, r10, 0x7fec
	ctx.r[3].s64 = ctx.r[10].s64 + 32748;
	// 8329AE4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AE64: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329AE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AE70: 4BABADD1  bl 0x82d55c40
	ctx.lr = 0x8329AE74;
	sub_82D55C40(ctx, base);
	// 8329AE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AE88 size=108
    let mut pc: u32 = 0x8329AE88;
    'dispatch: loop {
        match pc {
            0x8329AE88 => {
    //   block [0x8329AE88..0x8329AEF4)
	// 8329AE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AE94: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AE98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AE9C: 38EB7970  addi r7, r11, 0x7970
	ctx.r[7].s64 = ctx.r[11].s64 + 31088;
	// 8329AEA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329AEA4: 388A79B8  addi r4, r10, 0x79b8
	ctx.r[4].s64 = ctx.r[10].s64 + 31160;
	// 8329AEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AEAC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329AEB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AEB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AEB8: 386A801C  addi r3, r10, -0x7fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -32740;
	// 8329AEBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AEC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AED4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329AED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AEE0: 4BABAD61  bl 0x82d55c40
	ctx.lr = 0x8329AEE4;
	sub_82D55C40(ctx, base);
	// 8329AEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AEF8 size=108
    let mut pc: u32 = 0x8329AEF8;
    'dispatch: loop {
        match pc {
            0x8329AEF8 => {
    //   block [0x8329AEF8..0x8329AF64)
	// 8329AEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AF04: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329AF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AF0C: 38EB79A0  addi r7, r11, 0x79a0
	ctx.r[7].s64 = ctx.r[11].s64 + 31136;
	// 8329AF10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329AF14: 388A79D0  addi r4, r10, 0x79d0
	ctx.r[4].s64 = ctx.r[10].s64 + 31184;
	// 8329AF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AF1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329AF20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329AF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329AF28: 386A804C  addi r3, r10, -0x7fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -32692;
	// 8329AF2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329AF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329AF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AF44: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329AF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AF50: 4BABACF1  bl 0x82d55c40
	ctx.lr = 0x8329AF54;
	sub_82D55C40(ctx, base);
	// 8329AF54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329AF68 size=24
    let mut pc: u32 = 0x8329AF68;
    'dispatch: loop {
        match pc {
            0x8329AF68 => {
    //   block [0x8329AF68..0x8329AF80)
	// 8329AF68: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329AF6C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329AF70: 394A8178  addi r10, r10, -0x7e88
	ctx.r[10].s64 = ctx.r[10].s64 + -32392;
	// 8329AF74: 816B8160  lwz r11, -0x7ea0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32416 as u32) ) } as u64;
	// 8329AF78: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329AF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329AF80 size=112
    let mut pc: u32 = 0x8329AF80;
    'dispatch: loop {
        match pc {
            0x8329AF80 => {
    //   block [0x8329AF80..0x8329AFF0)
	// 8329AF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329AF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329AF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329AF8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AF90: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329AF94: 392A7A88  addi r9, r10, 0x7a88
	ctx.r[9].s64 = ctx.r[10].s64 + 31368;
	// 8329AF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329AF9C: 390B8178  addi r8, r11, -0x7e88
	ctx.r[8].s64 = ctx.r[11].s64 + -32392;
	// 8329AFA0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329AFA4: 388A7A9C  addi r4, r10, 0x7a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 31388;
	// 8329AFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329AFAC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329AFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329AFB4: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329AFB8: 386A807C  addi r3, r10, -0x7f84
	ctx.r[3].s64 = ctx.r[10].s64 + -32644;
	// 8329AFBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329AFC0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329AFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329AFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329AFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329AFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329AFD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329AFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329AFDC: 4BABAC65  bl 0x82d55c40
	ctx.lr = 0x8329AFE0;
	sub_82D55C40(ctx, base);
	// 8329AFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329AFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329AFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329AFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329AFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329AFF0 size=24
    let mut pc: u32 = 0x8329AFF0;
    'dispatch: loop {
        match pc {
            0x8329AFF0 => {
    //   block [0x8329AFF0..0x8329B008)
	// 8329AFF0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329AFF4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329AFF8: 394A8208  addi r10, r10, -0x7df8
	ctx.r[10].s64 = ctx.r[10].s64 + -32248;
	// 8329AFFC: 816B81F0  lwz r11, -0x7e10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32272 as u32) ) } as u64;
	// 8329B000: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329B004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B008 size=112
    let mut pc: u32 = 0x8329B008;
    'dispatch: loop {
        match pc {
            0x8329B008 => {
    //   block [0x8329B008..0x8329B078)
	// 8329B008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B014: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B018: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329B01C: 392A7B1C  addi r9, r10, 0x7b1c
	ctx.r[9].s64 = ctx.r[10].s64 + 31516;
	// 8329B020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B024: 390B8208  addi r8, r11, -0x7df8
	ctx.r[8].s64 = ctx.r[11].s64 + -32248;
	// 8329B028: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329B02C: 388A7B30  addi r4, r10, 0x7b30
	ctx.r[4].s64 = ctx.r[10].s64 + 31536;
	// 8329B030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B034: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B03C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329B040: 386A80AC  addi r3, r10, -0x7f54
	ctx.r[3].s64 = ctx.r[10].s64 + -32596;
	// 8329B044: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329B048: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329B04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B05C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B064: 4BABABDD  bl 0x82d55c40
	ctx.lr = 0x8329B068;
	sub_82D55C40(ctx, base);
	// 8329B068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B06C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B078 size=108
    let mut pc: u32 = 0x8329B078;
    'dispatch: loop {
        match pc {
            0x8329B078 => {
    //   block [0x8329B078..0x8329B0E4)
	// 8329B078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B084: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B08C: 38EB7C64  addi r7, r11, 0x7c64
	ctx.r[7].s64 = ctx.r[11].s64 + 31844;
	// 8329B090: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B094: 388A7D58  addi r4, r10, 0x7d58
	ctx.r[4].s64 = ctx.r[10].s64 + 32088;
	// 8329B098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B09C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B0A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B0A8: 386A80DC  addi r3, r10, -0x7f24
	ctx.r[3].s64 = ctx.r[10].s64 + -32548;
	// 8329B0AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B0B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B0C4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329B0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B0CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B0D0: 4BABAB71  bl 0x82d55c40
	ctx.lr = 0x8329B0D4;
	sub_82D55C40(ctx, base);
	// 8329B0D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B0E8 size=112
    let mut pc: u32 = 0x8329B0E8;
    'dispatch: loop {
        match pc {
            0x8329B0E8 => {
    //   block [0x8329B0E8..0x8329B158)
	// 8329B0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B0F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B0F8: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329B0FC: 392B7C50  addi r9, r11, 0x7c50
	ctx.r[9].s64 = ctx.r[11].s64 + 31824;
	// 8329B100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B104: 39090048  addi r8, r9, 0x48
	ctx.r[8].s64 = ctx.r[9].s64 + 72;
	// 8329B108: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8329B10C: 38AA7F8C  addi r5, r10, 0x7f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 32652;
	// 8329B110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B114: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B118: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329B11C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B120: 388A7D70  addi r4, r10, 0x7d70
	ctx.r[4].s64 = ctx.r[10].s64 + 32112;
	// 8329B124: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B128: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329B12C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329B130: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329B134: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B138: 386B810C  addi r3, r11, -0x7ef4
	ctx.r[3].s64 = ctx.r[11].s64 + -32500;
	// 8329B13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B140: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B144: 4BABAAFD  bl 0x82d55c40
	ctx.lr = 0x8329B148;
	sub_82D55C40(ctx, base);
	// 8329B148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329B158 size=24
    let mut pc: u32 = 0x8329B158;
    'dispatch: loop {
        match pc {
            0x8329B158 => {
    //   block [0x8329B158..0x8329B170)
	// 8329B158: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329B15C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329B160: 394A82C0  addi r10, r10, -0x7d40
	ctx.r[10].s64 = ctx.r[10].s64 + -32064;
	// 8329B164: 816B82A8  lwz r11, -0x7d58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32088 as u32) ) } as u64;
	// 8329B168: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329B16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B170 size=112
    let mut pc: u32 = 0x8329B170;
    'dispatch: loop {
        match pc {
            0x8329B170 => {
    //   block [0x8329B170..0x8329B1E0)
	// 8329B170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B17C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B180: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329B184: 392A7E14  addi r9, r10, 0x7e14
	ctx.r[9].s64 = ctx.r[10].s64 + 32276;
	// 8329B188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B18C: 390B82C0  addi r8, r11, -0x7d40
	ctx.r[8].s64 = ctx.r[11].s64 + -32064;
	// 8329B190: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8329B194: 388A7E28  addi r4, r10, 0x7e28
	ctx.r[4].s64 = ctx.r[10].s64 + 32296;
	// 8329B198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B19C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B1A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B1A4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B1A8: 386A813C  addi r3, r10, -0x7ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -32452;
	// 8329B1AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329B1B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329B1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B1B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B1C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B1C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B1C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B1CC: 4BABAA75  bl 0x82d55c40
	ctx.lr = 0x8329B1D0;
	sub_82D55C40(ctx, base);
	// 8329B1D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B1D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B1D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B1DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B1E0 size=112
    let mut pc: u32 = 0x8329B1E0;
    'dispatch: loop {
        match pc {
            0x8329B1E0 => {
    //   block [0x8329B1E0..0x8329B250)
	// 8329B1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B1EC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329B1F0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B1F4: 38AA7F8C  addi r5, r10, 0x7f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 32652;
	// 8329B1F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B1FC: 390B7E5C  addi r8, r11, 0x7e5c
	ctx.r[8].s64 = ctx.r[11].s64 + 32348;
	// 8329B200: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329B204: 388A7EBC  addi r4, r10, 0x7ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 32444;
	// 8329B208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B20C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B218: 386A816C  addi r3, r10, -0x7e94
	ctx.r[3].s64 = ctx.r[10].s64 + -32404;
	// 8329B21C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B234: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B23C: 4BABAA05  bl 0x82d55c40
	ctx.lr = 0x8329B240;
	sub_82D55C40(ctx, base);
	// 8329B240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B250 size=108
    let mut pc: u32 = 0x8329B250;
    'dispatch: loop {
        match pc {
            0x8329B250 => {
    //   block [0x8329B250..0x8329B2BC)
	// 8329B250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B25C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B264: 38EB7E8C  addi r7, r11, 0x7e8c
	ctx.r[7].s64 = ctx.r[11].s64 + 32396;
	// 8329B268: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B26C: 388A7ED4  addi r4, r10, 0x7ed4
	ctx.r[4].s64 = ctx.r[10].s64 + 32468;
	// 8329B270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B274: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B280: 386A819C  addi r3, r10, -0x7e64
	ctx.r[3].s64 = ctx.r[10].s64 + -32356;
	// 8329B284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B29C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B2A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B2A8: 4BABA999  bl 0x82d55c40
	ctx.lr = 0x8329B2AC;
	sub_82D55C40(ctx, base);
	// 8329B2AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B2C0 size=108
    let mut pc: u32 = 0x8329B2C0;
    'dispatch: loop {
        match pc {
            0x8329B2C0 => {
    //   block [0x8329B2C0..0x8329B32C)
	// 8329B2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B2CC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B2D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8329B2D4: 38EB7F10  addi r7, r11, 0x7f10
	ctx.r[7].s64 = ctx.r[11].s64 + 32528;
	// 8329B2D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329B2DC: 388A7F70  addi r4, r10, 0x7f70
	ctx.r[4].s64 = ctx.r[10].s64 + 32624;
	// 8329B2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B2E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B2E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B2F0: 386A81CC  addi r3, r10, -0x7e34
	ctx.r[3].s64 = ctx.r[10].s64 + -32308;
	// 8329B2F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B2FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B30C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8329B310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B318: 4BABA929  bl 0x82d55c40
	ctx.lr = 0x8329B31C;
	sub_82D55C40(ctx, base);
	// 8329B31C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B330 size=108
    let mut pc: u32 = 0x8329B330;
    'dispatch: loop {
        match pc {
            0x8329B330 => {
    //   block [0x8329B330..0x8329B39C)
	// 8329B330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B33C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B344: 38EB7FC0  addi r7, r11, 0x7fc0
	ctx.r[7].s64 = ctx.r[11].s64 + 32704;
	// 8329B348: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B34C: 388A8098  addi r4, r10, -0x7f68
	ctx.r[4].s64 = ctx.r[10].s64 + -32616;
	// 8329B350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B354: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B360: 386A81FC  addi r3, r10, -0x7e04
	ctx.r[3].s64 = ctx.r[10].s64 + -32260;
	// 8329B364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B37C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329B380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B388: 4BABA8B9  bl 0x82d55c40
	ctx.lr = 0x8329B38C;
	sub_82D55C40(ctx, base);
	// 8329B38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B3A0 size=112
    let mut pc: u32 = 0x8329B3A0;
    'dispatch: loop {
        match pc {
            0x8329B3A0 => {
    //   block [0x8329B3A0..0x8329B410)
	// 8329B3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B3AC: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329B3B0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 8329B3B4: 38AA7F8C  addi r5, r10, 0x7f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 32652;
	// 8329B3B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B3BC: 390B7FF0  addi r8, r11, 0x7ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 32752;
	// 8329B3C0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8329B3C4: 388A80B0  addi r4, r10, -0x7f50
	ctx.r[4].s64 = ctx.r[10].s64 + -32592;
	// 8329B3C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B3CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B3D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B3D8: 386A822C  addi r3, r10, -0x7dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -32212;
	// 8329B3DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B3E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B3E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B3E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B3F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B3F4: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 8329B3F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B3FC: 4BABA845  bl 0x82d55c40
	ctx.lr = 0x8329B400;
	sub_82D55C40(ctx, base);
	// 8329B400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B410 size=112
    let mut pc: u32 = 0x8329B410;
    'dispatch: loop {
        match pc {
            0x8329B410 => {
    //   block [0x8329B410..0x8329B480)
	// 8329B410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B41C: 3D408333  lis r10, -0x7ccd
	ctx.r[10].s64 = -2093809664;
	// 8329B420: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B424: 38AA7F8C  addi r5, r10, 0x7f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 32652;
	// 8329B428: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B42C: 390B80C8  addi r8, r11, -0x7f38
	ctx.r[8].s64 = ctx.r[11].s64 + -32568;
	// 8329B430: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329B434: 388A80F8  addi r4, r10, -0x7f08
	ctx.r[4].s64 = ctx.r[10].s64 + -32520;
	// 8329B438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B43C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B448: 386A825C  addi r3, r10, -0x7da4
	ctx.r[3].s64 = ctx.r[10].s64 + -32164;
	// 8329B44C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B464: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329B468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B46C: 4BABA7D5  bl 0x82d55c40
	ctx.lr = 0x8329B470;
	sub_82D55C40(ctx, base);
	// 8329B470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B480 size=108
    let mut pc: u32 = 0x8329B480;
    'dispatch: loop {
        match pc {
            0x8329B480 => {
    //   block [0x8329B480..0x8329B4EC)
	// 8329B480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B48C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B490: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B494: 396B81B8  addi r11, r11, -0x7e48
	ctx.r[11].s64 = ctx.r[11].s64 + -32328;
	// 8329B498: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8329B49C: 390B0138  addi r8, r11, 0x138
	ctx.r[8].s64 = ctx.r[11].s64 + 312;
	// 8329B4A0: 388A8354  addi r4, r10, -0x7cac
	ctx.r[4].s64 = ctx.r[10].s64 + -31916;
	// 8329B4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B4A8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B4AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329B4B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329B4B4: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 8329B4B8: 386A828C  addi r3, r10, -0x7d74
	ctx.r[3].s64 = ctx.r[10].s64 + -32116;
	// 8329B4BC: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329B4C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B4C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329B4CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B4D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329B4D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B4D8: 4BABA769  bl 0x82d55c40
	ctx.lr = 0x8329B4DC;
	sub_82D55C40(ctx, base);
	// 8329B4DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B4F0 size=108
    let mut pc: u32 = 0x8329B4F0;
    'dispatch: loop {
        match pc {
            0x8329B4F0 => {
    //   block [0x8329B4F0..0x8329B55C)
	// 8329B4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B4FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B500: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B504: 38EB8380  addi r7, r11, -0x7c80
	ctx.r[7].s64 = ctx.r[11].s64 + -31872;
	// 8329B508: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329B50C: 388A83E0  addi r4, r10, -0x7c20
	ctx.r[4].s64 = ctx.r[10].s64 + -31776;
	// 8329B510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B514: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B520: 386A82BC  addi r3, r10, -0x7d44
	ctx.r[3].s64 = ctx.r[10].s64 + -32068;
	// 8329B524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B53C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8329B540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B548: 4BABA6F9  bl 0x82d55c40
	ctx.lr = 0x8329B54C;
	sub_82D55C40(ctx, base);
	// 8329B54C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B560 size=108
    let mut pc: u32 = 0x8329B560;
    'dispatch: loop {
        match pc {
            0x8329B560 => {
    //   block [0x8329B560..0x8329B5CC)
	// 8329B560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B56C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B570: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B574: 38EB8400  addi r7, r11, -0x7c00
	ctx.r[7].s64 = ctx.r[11].s64 + -31744;
	// 8329B578: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B57C: 388A8430  addi r4, r10, -0x7bd0
	ctx.r[4].s64 = ctx.r[10].s64 + -31696;
	// 8329B580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B584: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B590: 386A82EC  addi r3, r10, -0x7d14
	ctx.r[3].s64 = ctx.r[10].s64 + -32020;
	// 8329B594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B5AC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B5B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B5B8: 4BABA689  bl 0x82d55c40
	ctx.lr = 0x8329B5BC;
	sub_82D55C40(ctx, base);
	// 8329B5BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B5D0 size=112
    let mut pc: u32 = 0x8329B5D0;
    'dispatch: loop {
        match pc {
            0x8329B5D0 => {
    //   block [0x8329B5D0..0x8329B640)
	// 8329B5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B5DC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B5E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B5E4: 38AA834C  addi r5, r10, -0x7cb4
	ctx.r[5].s64 = ctx.r[10].s64 + -31924;
	// 8329B5E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B5EC: 390B8448  addi r8, r11, -0x7bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -31672;
	// 8329B5F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329B5F4: 388A8460  addi r4, r10, -0x7ba0
	ctx.r[4].s64 = ctx.r[10].s64 + -31648;
	// 8329B5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B5FC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B608: 386A831C  addi r3, r10, -0x7ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -31972;
	// 8329B60C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B624: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329B628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B62C: 4BABA615  bl 0x82d55c40
	ctx.lr = 0x8329B630;
	sub_82D55C40(ctx, base);
	// 8329B630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B640 size=108
    let mut pc: u32 = 0x8329B640;
    'dispatch: loop {
        match pc {
            0x8329B640 => {
    //   block [0x8329B640..0x8329B6AC)
	// 8329B640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B64C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B650: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B654: 38EB8480  addi r7, r11, -0x7b80
	ctx.r[7].s64 = ctx.r[11].s64 + -31616;
	// 8329B658: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B65C: 388A84B0  addi r4, r10, -0x7b50
	ctx.r[4].s64 = ctx.r[10].s64 + -31568;
	// 8329B660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B664: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B670: 386A834C  addi r3, r10, -0x7cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -31924;
	// 8329B674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B68C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B698: 4BABA5A9  bl 0x82d55c40
	ctx.lr = 0x8329B69C;
	sub_82D55C40(ctx, base);
	// 8329B69C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B6A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B6A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B6B0 size=108
    let mut pc: u32 = 0x8329B6B0;
    'dispatch: loop {
        match pc {
            0x8329B6B0 => {
    //   block [0x8329B6B0..0x8329B71C)
	// 8329B6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B6BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B6C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B6C4: 38EB84D0  addi r7, r11, -0x7b30
	ctx.r[7].s64 = ctx.r[11].s64 + -31536;
	// 8329B6C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329B6CC: 388A8518  addi r4, r10, -0x7ae8
	ctx.r[4].s64 = ctx.r[10].s64 + -31464;
	// 8329B6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B6D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B6D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B6E0: 386A837C  addi r3, r10, -0x7c84
	ctx.r[3].s64 = ctx.r[10].s64 + -31876;
	// 8329B6E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B6FC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329B700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B708: 4BABA539  bl 0x82d55c40
	ctx.lr = 0x8329B70C;
	sub_82D55C40(ctx, base);
	// 8329B70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B720 size=108
    let mut pc: u32 = 0x8329B720;
    'dispatch: loop {
        match pc {
            0x8329B720 => {
    //   block [0x8329B720..0x8329B78C)
	// 8329B720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B72C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B734: 38EB84E8  addi r7, r11, -0x7b18
	ctx.r[7].s64 = ctx.r[11].s64 + -31512;
	// 8329B738: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329B73C: 388A853C  addi r4, r10, -0x7ac4
	ctx.r[4].s64 = ctx.r[10].s64 + -31428;
	// 8329B740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B744: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B750: 386A83AC  addi r3, r10, -0x7c54
	ctx.r[3].s64 = ctx.r[10].s64 + -31828;
	// 8329B754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B75C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B76C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329B770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B778: 4BABA4C9  bl 0x82d55c40
	ctx.lr = 0x8329B77C;
	sub_82D55C40(ctx, base);
	// 8329B77C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B790 size=108
    let mut pc: u32 = 0x8329B790;
    'dispatch: loop {
        match pc {
            0x8329B790 => {
    //   block [0x8329B790..0x8329B7FC)
	// 8329B790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B79C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B7A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B7A4: 38EB8564  addi r7, r11, -0x7a9c
	ctx.r[7].s64 = ctx.r[11].s64 + -31388;
	// 8329B7A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329B7AC: 388A857C  addi r4, r10, -0x7a84
	ctx.r[4].s64 = ctx.r[10].s64 + -31364;
	// 8329B7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B7B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B7B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329B7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B7C0: 386A83DC  addi r3, r10, -0x7c24
	ctx.r[3].s64 = ctx.r[10].s64 + -31780;
	// 8329B7C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329B7C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B7D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B7D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B7D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B7DC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329B7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B7E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329B7E8: 4BABA459  bl 0x82d55c40
	ctx.lr = 0x8329B7EC;
	sub_82D55C40(ctx, base);
	// 8329B7EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B800 size=100
    let mut pc: u32 = 0x8329B800;
    'dispatch: loop {
        match pc {
            0x8329B800 => {
    //   block [0x8329B800..0x8329B864)
	// 8329B800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B80C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B814: 38AA8660  addi r5, r10, -0x79a0
	ctx.r[5].s64 = ctx.r[10].s64 + -31136;
	// 8329B818: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B820: 388A8CC0  addi r4, r10, -0x7340
	ctx.r[4].s64 = ctx.r[10].s64 + -29504;
	// 8329B824: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B834: 386A8410  addi r3, r10, -0x7bf0
	ctx.r[3].s64 = ctx.r[10].s64 + -31728;
	// 8329B838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B83C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B840: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329B844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B848: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329B84C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329B850: 4BABA3F1  bl 0x82d55c40
	ctx.lr = 0x8329B854;
	sub_82D55C40(ctx, base);
	// 8329B854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B85C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B868 size=112
    let mut pc: u32 = 0x8329B868;
    'dispatch: loop {
        match pc {
            0x8329B868 => {
    //   block [0x8329B868..0x8329B8D8)
	// 8329B868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B874: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329B878: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B87C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329B880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B884: 390B8CF0  addi r8, r11, -0x7310
	ctx.r[8].s64 = ctx.r[11].s64 + -29456;
	// 8329B888: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329B88C: 388A8D50  addi r4, r10, -0x72b0
	ctx.r[4].s64 = ctx.r[10].s64 + -29360;
	// 8329B890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B894: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B8A0: 386A8440  addi r3, r10, -0x7bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -31680;
	// 8329B8A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B8AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B8BC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8329B8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B8C4: 4BABA37D  bl 0x82d55c40
	ctx.lr = 0x8329B8C8;
	sub_82D55C40(ctx, base);
	// 8329B8C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B8D8 size=92
    let mut pc: u32 = 0x8329B8D8;
    'dispatch: loop {
        match pc {
            0x8329B8D8 => {
    //   block [0x8329B8D8..0x8329B934)
	// 8329B8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B8E0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B8E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329B8E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329B8EC: 4BAEC0FD  bl 0x82d879e8
	ctx.lr = 0x8329B8F0;
	sub_82D879E8(ctx, base);
	// 8329B8F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B8F4: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329B8F8: 394B8D9C  addi r10, r11, -0x7264
	ctx.r[10].s64 = ctx.r[11].s64 + -29284;
	// 8329B8FC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329B900: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329B904: 396B8470  addi r11, r11, -0x7b90
	ctx.r[11].s64 = ctx.r[11].s64 + -31632;
	// 8329B908: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329B90C: 39480ED0  addi r10, r8, 0xed0
	ctx.r[10].s64 = ctx.r[8].s64 + 3792;
	// 8329B910: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329B914: 39490EB8  addi r10, r9, 0xeb8
	ctx.r[10].s64 = ctx.r[9].s64 + 3768;
	// 8329B918: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329B91C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329B920: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329B924: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8329B928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B92C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B938 size=112
    let mut pc: u32 = 0x8329B938;
    'dispatch: loop {
        match pc {
            0x8329B938 => {
    //   block [0x8329B938..0x8329B9A8)
	// 8329B938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B944: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B948: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B94C: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329B950: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B954: 390B8D6C  addi r8, r11, -0x7294
	ctx.r[8].s64 = ctx.r[11].s64 + -29332;
	// 8329B958: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329B95C: 388A8D9C  addi r4, r10, -0x7264
	ctx.r[4].s64 = ctx.r[10].s64 + -29284;
	// 8329B960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B964: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B970: 386A8480  addi r3, r10, -0x7b80
	ctx.r[3].s64 = ctx.r[10].s64 + -31616;
	// 8329B974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B98C: 38C00058  li r6, 0x58
	ctx.r[6].s64 = 88;
	// 8329B990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329B994: 4BABA2AD  bl 0x82d55c40
	ctx.lr = 0x8329B998;
	sub_82D55C40(ctx, base);
	// 8329B998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329B99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329B9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329B9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329B9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329B9A8 size=112
    let mut pc: u32 = 0x8329B9A8;
    'dispatch: loop {
        match pc {
            0x8329B9A8 => {
    //   block [0x8329B9A8..0x8329BA18)
	// 8329B9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329B9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329B9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329B9B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B9B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329B9BC: 38AA8440  addi r5, r10, -0x7bc0
	ctx.r[5].s64 = ctx.r[10].s64 + -31680;
	// 8329B9C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329B9C4: 390B8DC4  addi r8, r11, -0x723c
	ctx.r[8].s64 = ctx.r[11].s64 + -29244;
	// 8329B9C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329B9CC: 388A8DDC  addi r4, r10, -0x7224
	ctx.r[4].s64 = ctx.r[10].s64 + -29220;
	// 8329B9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329B9D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329B9D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329B9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329B9E0: 386A84B0  addi r3, r10, -0x7b50
	ctx.r[3].s64 = ctx.r[10].s64 + -31568;
	// 8329B9E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329B9E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329B9EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329B9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329B9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329B9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329B9FC: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8329BA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BA04: 4BABA23D  bl 0x82d55c40
	ctx.lr = 0x8329BA08;
	sub_82D55C40(ctx, base);
	// 8329BA08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BA0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BA10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BA18 size=108
    let mut pc: u32 = 0x8329BA18;
    'dispatch: loop {
        match pc {
            0x8329BA18 => {
    //   block [0x8329BA18..0x8329BA84)
	// 8329BA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BA24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BA28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BA2C: 392B8E18  addi r9, r11, -0x71e8
	ctx.r[9].s64 = ctx.r[11].s64 + -29160;
	// 8329BA30: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8329BA34: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329BA38: 388A8EC4  addi r4, r10, -0x713c
	ctx.r[4].s64 = ctx.r[10].s64 + -28988;
	// 8329BA3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BA40: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BA44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329BA48: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 8329BA4C: 386A84E0  addi r3, r10, -0x7b20
	ctx.r[3].s64 = ctx.r[10].s64 + -31520;
	// 8329BA50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BA54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329BA58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BA5C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BA60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BA64: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BA68: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329BA6C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BA70: 4BABA1D1  bl 0x82d55c40
	ctx.lr = 0x8329BA74;
	sub_82D55C40(ctx, base);
	// 8329BA74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BA80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BA88 size=112
    let mut pc: u32 = 0x8329BA88;
    'dispatch: loop {
        match pc {
            0x8329BA88 => {
    //   block [0x8329BA88..0x8329BAF8)
	// 8329BA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BA94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BA98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BA9C: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329BAA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BAA4: 390B8E78  addi r8, r11, -0x7188
	ctx.r[8].s64 = ctx.r[11].s64 + -29064;
	// 8329BAA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329BAAC: 388A8EE0  addi r4, r10, -0x7120
	ctx.r[4].s64 = ctx.r[10].s64 + -28960;
	// 8329BAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BAB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BAB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BAC0: 386A8510  addi r3, r10, -0x7af0
	ctx.r[3].s64 = ctx.r[10].s64 + -31472;
	// 8329BAC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329BAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BADC: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 8329BAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BAE4: 4BABA15D  bl 0x82d55c40
	ctx.lr = 0x8329BAE8;
	sub_82D55C40(ctx, base);
	// 8329BAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BAF8 size=112
    let mut pc: u32 = 0x8329BAF8;
    'dispatch: loop {
        match pc {
            0x8329BAF8 => {
    //   block [0x8329BAF8..0x8329BB68)
	// 8329BAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BB04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BB08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BB0C: 38AA8440  addi r5, r10, -0x7bc0
	ctx.r[5].s64 = ctx.r[10].s64 + -31680;
	// 8329BB10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BB14: 390B8F08  addi r8, r11, -0x70f8
	ctx.r[8].s64 = ctx.r[11].s64 + -28920;
	// 8329BB18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329BB1C: 388A8F38  addi r4, r10, -0x70c8
	ctx.r[4].s64 = ctx.r[10].s64 + -28872;
	// 8329BB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BB24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BB28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BB30: 386A8540  addi r3, r10, -0x7ac0
	ctx.r[3].s64 = ctx.r[10].s64 + -31424;
	// 8329BB34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329BB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BB3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BB4C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329BB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BB54: 4BABA0ED  bl 0x82d55c40
	ctx.lr = 0x8329BB58;
	sub_82D55C40(ctx, base);
	// 8329BB58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BB5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BB60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BB64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BB68 size=108
    let mut pc: u32 = 0x8329BB68;
    'dispatch: loop {
        match pc {
            0x8329BB68 => {
    //   block [0x8329BB68..0x8329BBD4)
	// 8329BB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BB74: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BB78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BB7C: 392B8FA0  addi r9, r11, -0x7060
	ctx.r[9].s64 = ctx.r[11].s64 + -28768;
	// 8329BB80: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8329BB84: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329BB88: 388A904C  addi r4, r10, -0x6fb4
	ctx.r[4].s64 = ctx.r[10].s64 + -28596;
	// 8329BB8C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BB90: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BB94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329BB98: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 8329BB9C: 386A8570  addi r3, r10, -0x7a90
	ctx.r[3].s64 = ctx.r[10].s64 + -31376;
	// 8329BBA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BBA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329BBA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BBAC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BBB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BBB4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BBB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329BBBC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BBC0: 4BABA081  bl 0x82d55c40
	ctx.lr = 0x8329BBC4;
	sub_82D55C40(ctx, base);
	// 8329BBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BBD8 size=112
    let mut pc: u32 = 0x8329BBD8;
    'dispatch: loop {
        match pc {
            0x8329BBD8 => {
    //   block [0x8329BBD8..0x8329BC48)
	// 8329BBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BBE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BBE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BBEC: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329BBF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BBF4: 390B9000  addi r8, r11, -0x7000
	ctx.r[8].s64 = ctx.r[11].s64 + -28672;
	// 8329BBF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329BBFC: 388A9064  addi r4, r10, -0x6f9c
	ctx.r[4].s64 = ctx.r[10].s64 + -28572;
	// 8329BC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BC04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BC08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BC0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BC10: 386A85A0  addi r3, r10, -0x7a60
	ctx.r[3].s64 = ctx.r[10].s64 + -31328;
	// 8329BC14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329BC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BC1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BC24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BC2C: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 8329BC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BC34: 4BABA00D  bl 0x82d55c40
	ctx.lr = 0x8329BC38;
	sub_82D55C40(ctx, base);
	// 8329BC38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BC3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BC40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BC48 size=108
    let mut pc: u32 = 0x8329BC48;
    'dispatch: loop {
        match pc {
            0x8329BC48 => {
    //   block [0x8329BC48..0x8329BCB4)
	// 8329BC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BC54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BC58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BC5C: 38EB9080  addi r7, r11, -0x6f80
	ctx.r[7].s64 = ctx.r[11].s64 + -28544;
	// 8329BC60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329BC64: 388A90FC  addi r4, r10, -0x6f04
	ctx.r[4].s64 = ctx.r[10].s64 + -28420;
	// 8329BC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BC6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BC70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329BC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BC78: 386A85D0  addi r3, r10, -0x7a30
	ctx.r[3].s64 = ctx.r[10].s64 + -31280;
	// 8329BC7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329BC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BC94: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329BC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BC9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329BCA0: 4BAB9FA1  bl 0x82d55c40
	ctx.lr = 0x8329BCA4;
	sub_82D55C40(ctx, base);
	// 8329BCA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BCB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BCB8 size=112
    let mut pc: u32 = 0x8329BCB8;
    'dispatch: loop {
        match pc {
            0x8329BCB8 => {
    //   block [0x8329BCB8..0x8329BD28)
	// 8329BCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BCC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BCC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BCCC: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329BCD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BCD4: 390B90B0  addi r8, r11, -0x6f50
	ctx.r[8].s64 = ctx.r[11].s64 + -28496;
	// 8329BCD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329BCDC: 388A9120  addi r4, r10, -0x6ee0
	ctx.r[4].s64 = ctx.r[10].s64 + -28384;
	// 8329BCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BCE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BCE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BCF0: 386A8600  addi r3, r10, -0x7a00
	ctx.r[3].s64 = ctx.r[10].s64 + -31232;
	// 8329BCF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329BCF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BCFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BD0C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329BD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BD14: 4BAB9F2D  bl 0x82d55c40
	ctx.lr = 0x8329BD18;
	sub_82D55C40(ctx, base);
	// 8329BD18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BD28 size=100
    let mut pc: u32 = 0x8329BD28;
    'dispatch: loop {
        match pc {
            0x8329BD28 => {
    //   block [0x8329BD28..0x8329BD8C)
	// 8329BD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BD34: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BD3C: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329BD40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BD48: 388A9140  addi r4, r10, -0x6ec0
	ctx.r[4].s64 = ctx.r[10].s64 + -28352;
	// 8329BD4C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BD5C: 386A8630  addi r3, r10, -0x79d0
	ctx.r[3].s64 = ctx.r[10].s64 + -31184;
	// 8329BD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BD64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BD68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329BD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BD70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329BD74: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329BD78: 4BAB9EC9  bl 0x82d55c40
	ctx.lr = 0x8329BD7C;
	sub_82D55C40(ctx, base);
	// 8329BD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BD90 size=100
    let mut pc: u32 = 0x8329BD90;
    'dispatch: loop {
        match pc {
            0x8329BD90 => {
    //   block [0x8329BD90..0x8329BDF4)
	// 8329BD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BD9C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BDA4: 38AA89F0  addi r5, r10, -0x7610
	ctx.r[5].s64 = ctx.r[10].s64 + -30224;
	// 8329BDA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BDB0: 388A91C8  addi r4, r10, -0x6e38
	ctx.r[4].s64 = ctx.r[10].s64 + -28216;
	// 8329BDB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BDC4: 386A8660  addi r3, r10, -0x79a0
	ctx.r[3].s64 = ctx.r[10].s64 + -31136;
	// 8329BDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BDCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BDD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329BDD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BDD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329BDDC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329BDE0: 4BAB9E61  bl 0x82d55c40
	ctx.lr = 0x8329BDE4;
	sub_82D55C40(ctx, base);
	// 8329BDE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BDE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BDEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BDF8 size=100
    let mut pc: u32 = 0x8329BDF8;
    'dispatch: loop {
        match pc {
            0x8329BDF8 => {
    //   block [0x8329BDF8..0x8329BE5C)
	// 8329BDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BE04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BE0C: 38AA8660  addi r5, r10, -0x79a0
	ctx.r[5].s64 = ctx.r[10].s64 + -31136;
	// 8329BE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BE14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BE18: 388A91E0  addi r4, r10, -0x6e20
	ctx.r[4].s64 = ctx.r[10].s64 + -28192;
	// 8329BE1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BE24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BE2C: 386A8690  addi r3, r10, -0x7970
	ctx.r[3].s64 = ctx.r[10].s64 + -31088;
	// 8329BE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BE34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BE38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329BE3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BE40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329BE44: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329BE48: 4BAB9DF9  bl 0x82d55c40
	ctx.lr = 0x8329BE4C;
	sub_82D55C40(ctx, base);
	// 8329BE4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BE60 size=108
    let mut pc: u32 = 0x8329BE60;
    'dispatch: loop {
        match pc {
            0x8329BE60 => {
    //   block [0x8329BE60..0x8329BECC)
	// 8329BE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BE6C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BE70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BE74: 38EB9238  addi r7, r11, -0x6dc8
	ctx.r[7].s64 = ctx.r[11].s64 + -28104;
	// 8329BE78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329BE7C: 388A92F8  addi r4, r10, -0x6d08
	ctx.r[4].s64 = ctx.r[10].s64 + -27912;
	// 8329BE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BE84: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BE88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329BE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BE90: 386A86C0  addi r3, r10, -0x7940
	ctx.r[3].s64 = ctx.r[10].s64 + -31040;
	// 8329BE94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329BE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BE9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BEA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BEAC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329BEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BEB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329BEB8: 4BAB9D89  bl 0x82d55c40
	ctx.lr = 0x8329BEBC;
	sub_82D55C40(ctx, base);
	// 8329BEBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BED0 size=92
    let mut pc: u32 = 0x8329BED0;
    'dispatch: loop {
        match pc {
            0x8329BED0 => {
    //   block [0x8329BED0..0x8329BF2C)
	// 8329BED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BED8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BEDC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329BEE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329BEE4: 4BB012ED  bl 0x82d9d1d0
	ctx.lr = 0x8329BEE8;
	sub_82D9D1D0(ctx, base);
	// 8329BEE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BEEC: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329BEF0: 394B9320  addi r10, r11, -0x6ce0
	ctx.r[10].s64 = ctx.r[11].s64 + -27872;
	// 8329BEF4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329BEF8: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329BEFC: 396B86F0  addi r11, r11, -0x7910
	ctx.r[11].s64 = ctx.r[11].s64 + -30992;
	// 8329BF00: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329BF04: 39481130  addi r10, r8, 0x1130
	ctx.r[10].s64 = ctx.r[8].s64 + 4400;
	// 8329BF08: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329BF0C: 39491148  addi r10, r9, 0x1148
	ctx.r[10].s64 = ctx.r[9].s64 + 4424;
	// 8329BF10: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329BF14: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329BF18: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329BF1C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8329BF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BF30 size=112
    let mut pc: u32 = 0x8329BF30;
    'dispatch: loop {
        match pc {
            0x8329BF30 => {
    //   block [0x8329BF30..0x8329BFA0)
	// 8329BF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BF3C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BF40: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BF44: 38AA8630  addi r5, r10, -0x79d0
	ctx.r[5].s64 = ctx.r[10].s64 + -31184;
	// 8329BF48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BF4C: 390B9268  addi r8, r11, -0x6d98
	ctx.r[8].s64 = ctx.r[11].s64 + -28056;
	// 8329BF50: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8329BF54: 388A9320  addi r4, r10, -0x6ce0
	ctx.r[4].s64 = ctx.r[10].s64 + -27872;
	// 8329BF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BF5C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329BF60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BF68: 386A8700  addi r3, r10, -0x7900
	ctx.r[3].s64 = ctx.r[10].s64 + -30976;
	// 8329BF6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329BF70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329BF74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329BF78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BF80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BF84: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 8329BF88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BF8C: 4BAB9CB5  bl 0x82d55c40
	ctx.lr = 0x8329BF90;
	sub_82D55C40(ctx, base);
	// 8329BF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329BF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329BF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329BF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329BFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329BFA0 size=112
    let mut pc: u32 = 0x8329BFA0;
    'dispatch: loop {
        match pc {
            0x8329BFA0 => {
    //   block [0x8329BFA0..0x8329C010)
	// 8329BFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329BFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329BFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329BFAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329BFB0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329BFB4: 392B9650  addi r9, r11, -0x69b0
	ctx.r[9].s64 = ctx.r[11].s64 + -27056;
	// 8329BFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329BFBC: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 8329BFC0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329BFC4: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329BFC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329BFCC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329BFD0: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329BFD4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329BFD8: 388A967C  addi r4, r10, -0x6984
	ctx.r[4].s64 = ctx.r[10].s64 + -27012;
	// 8329BFDC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329BFE0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329BFE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329BFE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329BFEC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329BFF0: 386B8730  addi r3, r11, -0x78d0
	ctx.r[3].s64 = ctx.r[11].s64 + -30928;
	// 8329BFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329BFF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329BFFC: 4BAB9C45  bl 0x82d55c40
	ctx.lr = 0x8329C000;
	sub_82D55C40(ctx, base);
	// 8329C000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C010 size=108
    let mut pc: u32 = 0x8329C010;
    'dispatch: loop {
        match pc {
            0x8329C010 => {
    //   block [0x8329C010..0x8329C07C)
	// 8329C010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C01C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C020: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C024: 392B96C0  addi r9, r11, -0x6940
	ctx.r[9].s64 = ctx.r[11].s64 + -26944;
	// 8329C028: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8329C02C: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329C030: 388A97B4  addi r4, r10, -0x684c
	ctx.r[4].s64 = ctx.r[10].s64 + -26700;
	// 8329C034: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C038: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C03C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329C040: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 8329C044: 386A8760  addi r3, r10, -0x78a0
	ctx.r[3].s64 = ctx.r[10].s64 + -30880;
	// 8329C048: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C04C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C050: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C054: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C058: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C05C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C060: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C064: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C068: 4BAB9BD9  bl 0x82d55c40
	ctx.lr = 0x8329C06C;
	sub_82D55C40(ctx, base);
	// 8329C06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C080 size=112
    let mut pc: u32 = 0x8329C080;
    'dispatch: loop {
        match pc {
            0x8329C080 => {
    //   block [0x8329C080..0x8329C0F0)
	// 8329C080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C08C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C090: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C094: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329C098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C09C: 390B9768  addi r8, r11, -0x6898
	ctx.r[8].s64 = ctx.r[11].s64 + -26776;
	// 8329C0A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329C0A4: 388A97D8  addi r4, r10, -0x6828
	ctx.r[4].s64 = ctx.r[10].s64 + -26664;
	// 8329C0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C0AC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C0B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C0B8: 386A8790  addi r3, r10, -0x7870
	ctx.r[3].s64 = ctx.r[10].s64 + -30832;
	// 8329C0BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329C0C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C0C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C0C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C0CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C0D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C0D4: 38C000E0  li r6, 0xe0
	ctx.r[6].s64 = 224;
	// 8329C0D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C0DC: 4BAB9B65  bl 0x82d55c40
	ctx.lr = 0x8329C0E0;
	sub_82D55C40(ctx, base);
	// 8329C0E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C0E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C0E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C0F0 size=108
    let mut pc: u32 = 0x8329C0F0;
    'dispatch: loop {
        match pc {
            0x8329C0F0 => {
    //   block [0x8329C0F0..0x8329C15C)
	// 8329C0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C0FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C100: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C104: 38EB9A80  addi r7, r11, -0x6580
	ctx.r[7].s64 = ctx.r[11].s64 + -25984;
	// 8329C108: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329C10C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8329C110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C114: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C11C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C120: 386A87C0  addi r3, r10, -0x7840
	ctx.r[3].s64 = ctx.r[10].s64 + -30784;
	// 8329C124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C13C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329C140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C148: 4BAB9AF9  bl 0x82d55c40
	ctx.lr = 0x8329C14C;
	sub_82D55C40(ctx, base);
	// 8329C14C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C160 size=108
    let mut pc: u32 = 0x8329C160;
    'dispatch: loop {
        match pc {
            0x8329C160 => {
    //   block [0x8329C160..0x8329C1CC)
	// 8329C160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C16C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C170: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C174: 38EB9AC8  addi r7, r11, -0x6538
	ctx.r[7].s64 = ctx.r[11].s64 + -25912;
	// 8329C178: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329C17C: 388A9D64  addi r4, r10, -0x629c
	ctx.r[4].s64 = ctx.r[10].s64 + -25244;
	// 8329C180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C184: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C18C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C190: 386A87F0  addi r3, r10, -0x7810
	ctx.r[3].s64 = ctx.r[10].s64 + -30736;
	// 8329C194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C19C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C1A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C1AC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329C1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C1B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C1B8: 4BAB9A89  bl 0x82d55c40
	ctx.lr = 0x8329C1BC;
	sub_82D55C40(ctx, base);
	// 8329C1BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C1C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C1C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C1D0 size=108
    let mut pc: u32 = 0x8329C1D0;
    'dispatch: loop {
        match pc {
            0x8329C1D0 => {
    //   block [0x8329C1D0..0x8329C23C)
	// 8329C1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C1D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C1DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C1E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C1E4: 38EB9B28  addi r7, r11, -0x64d8
	ctx.r[7].s64 = ctx.r[11].s64 + -25816;
	// 8329C1E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329C1EC: 388A9D84  addi r4, r10, -0x627c
	ctx.r[4].s64 = ctx.r[10].s64 + -25212;
	// 8329C1F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C1F4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C1F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C200: 386A8820  addi r3, r10, -0x77e0
	ctx.r[3].s64 = ctx.r[10].s64 + -30688;
	// 8329C204: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C20C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C21C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329C220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C228: 4BAB9A19  bl 0x82d55c40
	ctx.lr = 0x8329C22C;
	sub_82D55C40(ctx, base);
	// 8329C22C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C240 size=92
    let mut pc: u32 = 0x8329C240;
    'dispatch: loop {
        match pc {
            0x8329C240 => {
    //   block [0x8329C240..0x8329C29C)
	// 8329C240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C248: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C24C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329C250: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329C254: 4BAE98DD  bl 0x82d85b30
	ctx.lr = 0x8329C258;
	sub_82D85B30(ctx, base);
	// 8329C258: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C25C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329C260: 394B9DA0  addi r10, r11, -0x6260
	ctx.r[10].s64 = ctx.r[11].s64 + -25184;
	// 8329C264: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C268: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329C26C: 396B8850  addi r11, r11, -0x77b0
	ctx.r[11].s64 = ctx.r[11].s64 + -30640;
	// 8329C270: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329C274: 39481278  addi r10, r8, 0x1278
	ctx.r[10].s64 = ctx.r[8].s64 + 4728;
	// 8329C278: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329C27C: 39491260  addi r10, r9, 0x1260
	ctx.r[10].s64 = ctx.r[9].s64 + 4704;
	// 8329C280: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329C284: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329C288: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329C28C: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 8329C290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C2A0 size=116
    let mut pc: u32 = 0x8329C2A0;
    'dispatch: loop {
        match pc {
            0x8329C2A0 => {
    //   block [0x8329C2A0..0x8329C314)
	// 8329C2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C2AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C2B0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C2B4: 392B9A68  addi r9, r11, -0x6598
	ctx.r[9].s64 = ctx.r[11].s64 + -26008;
	// 8329C2B8: 38AA8D50  addi r5, r10, -0x72b0
	ctx.r[5].s64 = ctx.r[10].s64 + -29360;
	// 8329C2BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C2C0: 38E90288  addi r7, r9, 0x288
	ctx.r[7].s64 = ctx.r[9].s64 + 648;
	// 8329C2C4: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8329C2C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C2CC: 388A9DA0  addi r4, r10, -0x6260
	ctx.r[4].s64 = ctx.r[10].s64 + -25184;
	// 8329C2D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C2D4: 396B9B58  addi r11, r11, -0x64a8
	ctx.r[11].s64 = ctx.r[11].s64 + -25768;
	// 8329C2D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8329C2DC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C2E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8329C2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C2E8: 386A8860  addi r3, r10, -0x77a0
	ctx.r[3].s64 = ctx.r[10].s64 + -30624;
	// 8329C2EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C2F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8329C2F4: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 8329C2F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8329C2FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329C300: 4BAB9941  bl 0x82d55c40
	ctx.lr = 0x8329C304;
	sub_82D55C40(ctx, base);
	// 8329C304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329C318 size=24
    let mut pc: u32 = 0x8329C318;
    'dispatch: loop {
        match pc {
            0x8329C318 => {
    //   block [0x8329C318..0x8329C330)
	// 8329C318: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C31C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329C320: 394A89CC  addi r10, r10, -0x7634
	ctx.r[10].s64 = ctx.r[10].s64 + -30260;
	// 8329C324: 816B89C8  lwz r11, -0x7638(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30264 as u32) ) } as u64;
	// 8329C328: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329C32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C330 size=116
    let mut pc: u32 = 0x8329C330;
    'dispatch: loop {
        match pc {
            0x8329C330 => {
    //   block [0x8329C330..0x8329C3A4)
	// 8329C330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C33C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C344: 390B89CC  addi r8, r11, -0x7634
	ctx.r[8].s64 = ctx.r[11].s64 + -30260;
	// 8329C348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C34C: 392A9E3C  addi r9, r10, -0x61c4
	ctx.r[9].s64 = ctx.r[10].s64 + -25028;
	// 8329C350: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329C354: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329C358: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329C35C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C364: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C374: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C378: 388A9E50  addi r4, r10, -0x61b0
	ctx.r[4].s64 = ctx.r[10].s64 + -25008;
	// 8329C37C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329C380: 386B8890  addi r3, r11, -0x7770
	ctx.r[3].s64 = ctx.r[11].s64 + -30576;
	// 8329C384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C388: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C38C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329C390: 4BAB98B1  bl 0x82d55c40
	ctx.lr = 0x8329C394;
	sub_82D55C40(ctx, base);
	// 8329C394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C3A8 size=112
    let mut pc: u32 = 0x8329C3A8;
    'dispatch: loop {
        match pc {
            0x8329C3A8 => {
    //   block [0x8329C3A8..0x8329C418)
	// 8329C3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C3B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C3B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C3BC: 38AA8A50  addi r5, r10, -0x75b0
	ctx.r[5].s64 = ctx.r[10].s64 + -30128;
	// 8329C3C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C3C4: 390B9EB0  addi r8, r11, -0x6150
	ctx.r[8].s64 = ctx.r[11].s64 + -24912;
	// 8329C3C8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8329C3CC: 388A9F5C  addi r4, r10, -0x60a4
	ctx.r[4].s64 = ctx.r[10].s64 + -24740;
	// 8329C3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C3D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C3D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C3E0: 386A88C0  addi r3, r10, -0x7740
	ctx.r[3].s64 = ctx.r[10].s64 + -30528;
	// 8329C3E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329C3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C3FC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329C400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C404: 4BAB983D  bl 0x82d55c40
	ctx.lr = 0x8329C408;
	sub_82D55C40(ctx, base);
	// 8329C408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C418 size=100
    let mut pc: u32 = 0x8329C418;
    'dispatch: loop {
        match pc {
            0x8329C418 => {
    //   block [0x8329C418..0x8329C47C)
	// 8329C418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C424: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329C428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C42C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329C430: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C438: 388A9F78  addi r4, r10, -0x6088
	ctx.r[4].s64 = ctx.r[10].s64 + -24712;
	// 8329C43C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C44C: 386A88F0  addi r3, r10, -0x7710
	ctx.r[3].s64 = ctx.r[10].s64 + -30480;
	// 8329C450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C458: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329C45C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C460: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329C464: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329C468: 4BAB97D9  bl 0x82d55c40
	ctx.lr = 0x8329C46C;
	sub_82D55C40(ctx, base);
	// 8329C46C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C480 size=92
    let mut pc: u32 = 0x8329C480;
    'dispatch: loop {
        match pc {
            0x8329C480 => {
    //   block [0x8329C480..0x8329C4DC)
	// 8329C480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C48C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329C490: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329C494: 4BB01E95  bl 0x82d9e328
	ctx.lr = 0x8329C498;
	sub_82D9E328(ctx, base);
	// 8329C498: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C49C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329C4A0: 394B9FF8  addi r10, r11, -0x6008
	ctx.r[10].s64 = ctx.r[11].s64 + -24584;
	// 8329C4A4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C4A8: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329C4AC: 396B8920  addi r11, r11, -0x76e0
	ctx.r[11].s64 = ctx.r[11].s64 + -30432;
	// 8329C4B0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329C4B4: 39481538  addi r10, r8, 0x1538
	ctx.r[10].s64 = ctx.r[8].s64 + 5432;
	// 8329C4B8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329C4BC: 39491520  addi r10, r9, 0x1520
	ctx.r[10].s64 = ctx.r[9].s64 + 5408;
	// 8329C4C0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329C4C4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329C4C8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329C4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C4E0 size=112
    let mut pc: u32 = 0x8329C4E0;
    'dispatch: loop {
        match pc {
            0x8329C4E0 => {
    //   block [0x8329C4E0..0x8329C550)
	// 8329C4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C4EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C4F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C4F4: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329C4F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C4FC: 390B9FB0  addi r8, r11, -0x6050
	ctx.r[8].s64 = ctx.r[11].s64 + -24656;
	// 8329C500: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329C504: 388A9FF8  addi r4, r10, -0x6008
	ctx.r[4].s64 = ctx.r[10].s64 + -24584;
	// 8329C508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C50C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C518: 386A8930  addi r3, r10, -0x76d0
	ctx.r[3].s64 = ctx.r[10].s64 + -30416;
	// 8329C51C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329C520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C534: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329C538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C53C: 4BAB9705  bl 0x82d55c40
	ctx.lr = 0x8329C540;
	sub_82D55C40(ctx, base);
	// 8329C540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C550 size=112
    let mut pc: u32 = 0x8329C550;
    'dispatch: loop {
        match pc {
            0x8329C550 => {
    //   block [0x8329C550..0x8329C5C0)
	// 8329C550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C55C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C560: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C564: 38AA8890  addi r5, r10, -0x7770
	ctx.r[5].s64 = ctx.r[10].s64 + -30576;
	// 8329C568: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C56C: 390BA02C  addi r8, r11, -0x5fd4
	ctx.r[8].s64 = ctx.r[11].s64 + -24532;
	// 8329C570: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329C574: 388AA05C  addi r4, r10, -0x5fa4
	ctx.r[4].s64 = ctx.r[10].s64 + -24484;
	// 8329C578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C57C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C588: 386A8960  addi r3, r10, -0x76a0
	ctx.r[3].s64 = ctx.r[10].s64 + -30368;
	// 8329C58C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329C590: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C5A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C5A4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329C5A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C5AC: 4BAB9695  bl 0x82d55c40
	ctx.lr = 0x8329C5B0;
	sub_82D55C40(ctx, base);
	// 8329C5B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C5B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C5B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C5BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C5C0 size=108
    let mut pc: u32 = 0x8329C5C0;
    'dispatch: loop {
        match pc {
            0x8329C5C0 => {
    //   block [0x8329C5C0..0x8329C62C)
	// 8329C5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C5CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C5D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C5D4: 38EBA0E8  addi r7, r11, -0x5f18
	ctx.r[7].s64 = ctx.r[11].s64 + -24344;
	// 8329C5D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329C5DC: 388AA1C0  addi r4, r10, -0x5e40
	ctx.r[4].s64 = ctx.r[10].s64 + -24128;
	// 8329C5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C5E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C5E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C5F0: 386A8990  addi r3, r10, -0x7670
	ctx.r[3].s64 = ctx.r[10].s64 + -30320;
	// 8329C5F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C60C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329C610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C614: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C618: 4BAB9629  bl 0x82d55c40
	ctx.lr = 0x8329C61C;
	sub_82D55C40(ctx, base);
	// 8329C61C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C630 size=108
    let mut pc: u32 = 0x8329C630;
    'dispatch: loop {
        match pc {
            0x8329C630 => {
    //   block [0x8329C630..0x8329C69C)
	// 8329C630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C63C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C640: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C644: 38EBA148  addi r7, r11, -0x5eb8
	ctx.r[7].s64 = ctx.r[11].s64 + -24248;
	// 8329C648: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8329C64C: 388AA1F0  addi r4, r10, -0x5e10
	ctx.r[4].s64 = ctx.r[10].s64 + -24080;
	// 8329C650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C654: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C660: 386A89C0  addi r3, r10, -0x7640
	ctx.r[3].s64 = ctx.r[10].s64 + -30272;
	// 8329C664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C67C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329C680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C688: 4BAB95B9  bl 0x82d55c40
	ctx.lr = 0x8329C68C;
	sub_82D55C40(ctx, base);
	// 8329C68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329C6A0 size=24
    let mut pc: u32 = 0x8329C6A0;
    'dispatch: loop {
        match pc {
            0x8329C6A0 => {
    //   block [0x8329C6A0..0x8329C6B8)
	// 8329C6A0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C6A4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329C6A8: 394A8A68  addi r10, r10, -0x7598
	ctx.r[10].s64 = ctx.r[10].s64 + -30104;
	// 8329C6AC: 816B8A60  lwz r11, -0x75a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30112 as u32) ) } as u64;
	// 8329C6B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329C6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C6B8 size=116
    let mut pc: u32 = 0x8329C6B8;
    'dispatch: loop {
        match pc {
            0x8329C6B8 => {
    //   block [0x8329C6B8..0x8329C72C)
	// 8329C6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C6C4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C6C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C6CC: 390B8A68  addi r8, r11, -0x7598
	ctx.r[8].s64 = ctx.r[11].s64 + -30104;
	// 8329C6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C6D4: 392AA358  addi r9, r10, -0x5ca8
	ctx.r[9].s64 = ctx.r[10].s64 + -23720;
	// 8329C6D8: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329C6DC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8329C6E0: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329C6E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C6EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C6FC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C700: 388AA444  addi r4, r10, -0x5bbc
	ctx.r[4].s64 = ctx.r[10].s64 + -23484;
	// 8329C704: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329C708: 386B89F0  addi r3, r11, -0x7610
	ctx.r[3].s64 = ctx.r[11].s64 + -30224;
	// 8329C70C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C710: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C714: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329C718: 4BAB9529  bl 0x82d55c40
	ctx.lr = 0x8329C71C;
	sub_82D55C40(ctx, base);
	// 8329C71C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C730 size=116
    let mut pc: u32 = 0x8329C730;
    'dispatch: loop {
        match pc {
            0x8329C730 => {
    //   block [0x8329C730..0x8329C7A4)
	// 8329C730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C73C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C740: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8329C744: 390AA478  addi r8, r10, -0x5b88
	ctx.r[8].s64 = ctx.r[10].s64 + -23432;
	// 8329C748: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329C74C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C750: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329C754: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C758: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329C75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C764: 388AA558  addi r4, r10, -0x5aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -23208;
	// 8329C768: 396BA520  addi r11, r11, -0x5ae0
	ctx.r[11].s64 = ctx.r[11].s64 + -23264;
	// 8329C76C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C770: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C774: 386A8A20  addi r3, r10, -0x75e0
	ctx.r[3].s64 = ctx.r[10].s64 + -30176;
	// 8329C778: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329C77C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C780: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329C784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C78C: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 8329C790: 4BAB94B1  bl 0x82d55c40
	ctx.lr = 0x8329C794;
	sub_82D55C40(ctx, base);
	// 8329C794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C7A8 size=100
    let mut pc: u32 = 0x8329C7A8;
    'dispatch: loop {
        match pc {
            0x8329C7A8 => {
    //   block [0x8329C7A8..0x8329C80C)
	// 8329C7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C7B4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329C7B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C7BC: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329C7C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C7C8: 388AA56C  addi r4, r10, -0x5a94
	ctx.r[4].s64 = ctx.r[10].s64 + -23188;
	// 8329C7CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C7D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C7D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C7D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C7DC: 386A8A50  addi r3, r10, -0x75b0
	ctx.r[3].s64 = ctx.r[10].s64 + -30128;
	// 8329C7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C7E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C7E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329C7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C7F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329C7F4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329C7F8: 4BAB9449  bl 0x82d55c40
	ctx.lr = 0x8329C7FC;
	sub_82D55C40(ctx, base);
	// 8329C7FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C810 size=112
    let mut pc: u32 = 0x8329C810;
    'dispatch: loop {
        match pc {
            0x8329C810 => {
    //   block [0x8329C810..0x8329C880)
	// 8329C810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C818: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C81C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C820: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8329C824: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C828: 396BA6A8  addi r11, r11, -0x5958
	ctx.r[11].s64 = ctx.r[11].s64 + -22872;
	// 8329C82C: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 8329C830: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8329C834: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8329C838: 4BB01D69  bl 0x82d9e5a0
	ctx.lr = 0x8329C83C;
	sub_82D9E5A0(ctx, base);
	// 8329C83C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C840: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329C844: 394BA6D8  addi r10, r11, -0x5928
	ctx.r[10].s64 = ctx.r[11].s64 + -22824;
	// 8329C848: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C84C: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329C850: 396B8A80  addi r11, r11, -0x7580
	ctx.r[11].s64 = ctx.r[11].s64 + -30080;
	// 8329C854: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329C858: 39481748  addi r10, r8, 0x1748
	ctx.r[10].s64 = ctx.r[8].s64 + 5960;
	// 8329C85C: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329C860: 39491730  addi r10, r9, 0x1730
	ctx.r[10].s64 = ctx.r[9].s64 + 5936;
	// 8329C864: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329C868: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329C86C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329C870: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8329C874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329C880 size=24
    let mut pc: u32 = 0x8329C880;
    'dispatch: loop {
        match pc {
            0x8329C880 => {
    //   block [0x8329C880..0x8329C898)
	// 8329C880: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C884: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329C888: 394A8BA8  addi r10, r10, -0x7458
	ctx.r[10].s64 = ctx.r[10].s64 + -29784;
	// 8329C88C: 816B8BA0  lwz r11, -0x7460(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29792 as u32) ) } as u64;
	// 8329C890: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8329C894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C898 size=116
    let mut pc: u32 = 0x8329C898;
    'dispatch: loop {
        match pc {
            0x8329C898 => {
    //   block [0x8329C898..0x8329C90C)
	// 8329C898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C8A4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329C8A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C8AC: 390B8BA8  addi r8, r11, -0x7458
	ctx.r[8].s64 = ctx.r[11].s64 + -29784;
	// 8329C8B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C8B4: 392AA690  addi r9, r10, -0x5970
	ctx.r[9].s64 = ctx.r[10].s64 + -22896;
	// 8329C8B8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C8BC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329C8C0: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329C8C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C8CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C8D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C8DC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329C8E0: 388AA6D8  addi r4, r10, -0x5928
	ctx.r[4].s64 = ctx.r[10].s64 + -22824;
	// 8329C8E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329C8E8: 386B8A90  addi r3, r11, -0x7570
	ctx.r[3].s64 = ctx.r[11].s64 + -30064;
	// 8329C8EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329C8F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C8F4: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 8329C8F8: 4BAB9349  bl 0x82d55c40
	ctx.lr = 0x8329C8FC;
	sub_82D55C40(ctx, base);
	// 8329C8FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C910 size=112
    let mut pc: u32 = 0x8329C910;
    'dispatch: loop {
        match pc {
            0x8329C910 => {
    //   block [0x8329C910..0x8329C980)
	// 8329C910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C91C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C920: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C924: 38AA8D50  addi r5, r10, -0x72b0
	ctx.r[5].s64 = ctx.r[10].s64 + -29360;
	// 8329C928: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C92C: 390BA730  addi r8, r11, -0x58d0
	ctx.r[8].s64 = ctx.r[11].s64 + -22736;
	// 8329C930: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329C934: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 8329C938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C93C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329C944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C948: 386A8AC0  addi r3, r10, -0x7540
	ctx.r[3].s64 = ctx.r[10].s64 + -30016;
	// 8329C94C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329C950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C95C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C964: 38C00098  li r6, 0x98
	ctx.r[6].s64 = 152;
	// 8329C968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C96C: 4BAB92D5  bl 0x82d55c40
	ctx.lr = 0x8329C970;
	sub_82D55C40(ctx, base);
	// 8329C970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C980 size=108
    let mut pc: u32 = 0x8329C980;
    'dispatch: loop {
        match pc {
            0x8329C980 => {
    //   block [0x8329C980..0x8329C9EC)
	// 8329C980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C98C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329C990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329C994: 38EBA770  addi r7, r11, -0x5890
	ctx.r[7].s64 = ctx.r[11].s64 + -22672;
	// 8329C998: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329C99C: 388AA7EC  addi r4, r10, -0x5814
	ctx.r[4].s64 = ctx.r[10].s64 + -22548;
	// 8329C9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329C9A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329C9A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329C9AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329C9B0: 386A8AF0  addi r3, r10, -0x7510
	ctx.r[3].s64 = ctx.r[10].s64 + -29968;
	// 8329C9B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329C9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329C9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329C9C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329C9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329C9C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329C9CC: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 8329C9D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329C9D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329C9D8: 4BAB9269  bl 0x82d55c40
	ctx.lr = 0x8329C9DC;
	sub_82D55C40(ctx, base);
	// 8329C9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329C9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329C9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329C9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329C9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329C9F0 size=112
    let mut pc: u32 = 0x8329C9F0;
    'dispatch: loop {
        match pc {
            0x8329C9F0 => {
    //   block [0x8329C9F0..0x8329CA60)
	// 8329C9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329C9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329C9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329C9FC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CA00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CA04: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329CA08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CA0C: 390BA7A0  addi r8, r11, -0x5860
	ctx.r[8].s64 = ctx.r[11].s64 + -22624;
	// 8329CA10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329CA14: 388AA810  addi r4, r10, -0x57f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22512;
	// 8329CA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CA1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CA20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CA28: 386A8B20  addi r3, r10, -0x74e0
	ctx.r[3].s64 = ctx.r[10].s64 + -29920;
	// 8329CA2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CA34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CA3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CA44: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 8329CA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CA4C: 4BAB91F5  bl 0x82d55c40
	ctx.lr = 0x8329CA50;
	sub_82D55C40(ctx, base);
	// 8329CA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CA60 size=104
    let mut pc: u32 = 0x8329CA60;
    'dispatch: loop {
        match pc {
            0x8329CA60 => {
    //   block [0x8329CA60..0x8329CAC8)
	// 8329CA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CA6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CA74: 392AA8B0  addi r9, r10, -0x5750
	ctx.r[9].s64 = ctx.r[10].s64 + -22352;
	// 8329CA78: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CA80: 38AA88F0  addi r5, r10, -0x7710
	ctx.r[5].s64 = ctx.r[10].s64 + -30480;
	// 8329CA84: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CA94: 388AA8C4  addi r4, r10, -0x573c
	ctx.r[4].s64 = ctx.r[10].s64 + -22332;
	// 8329CA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CA9C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CAA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329CAA4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329CAA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329CAAC: 386A8B50  addi r3, r10, -0x74b0
	ctx.r[3].s64 = ctx.r[10].s64 + -29872;
	// 8329CAB0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329CAB4: 4BAB918D  bl 0x82d55c40
	ctx.lr = 0x8329CAB8;
	sub_82D55C40(ctx, base);
	// 8329CAB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CAC8 size=92
    let mut pc: u32 = 0x8329CAC8;
    'dispatch: loop {
        match pc {
            0x8329CAC8 => {
    //   block [0x8329CAC8..0x8329CB24)
	// 8329CAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CAD0: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CAD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329CAD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329CADC: 4BAE51F5  bl 0x82d81cd0
	ctx.lr = 0x8329CAE0;
	sub_82D81CD0(ctx, base);
	// 8329CAE0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CAE4: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329CAE8: 394BA980  addi r10, r11, -0x5680
	ctx.r[10].s64 = ctx.r[11].s64 + -22144;
	// 8329CAEC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329CAF0: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329CAF4: 396B8B80  addi r11, r11, -0x7480
	ctx.r[11].s64 = ctx.r[11].s64 + -29824;
	// 8329CAF8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329CAFC: 39481C88  addi r10, r8, 0x1c88
	ctx.r[10].s64 = ctx.r[8].s64 + 7304;
	// 8329CB00: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329CB04: 39491C70  addi r10, r9, 0x1c70
	ctx.r[10].s64 = ctx.r[9].s64 + 7280;
	// 8329CB08: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329CB0C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329CB10: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329CB14: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 8329CB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CB28 size=112
    let mut pc: u32 = 0x8329CB28;
    'dispatch: loop {
        match pc {
            0x8329CB28 => {
    //   block [0x8329CB28..0x8329CB98)
	// 8329CB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CB34: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CB38: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CB3C: 38AA8AC0  addi r5, r10, -0x7540
	ctx.r[5].s64 = ctx.r[10].s64 + -30016;
	// 8329CB40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CB44: 390BA918  addi r8, r11, -0x56e8
	ctx.r[8].s64 = ctx.r[11].s64 + -22248;
	// 8329CB48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329CB4C: 388AA980  addi r4, r10, -0x5680
	ctx.r[4].s64 = ctx.r[10].s64 + -22144;
	// 8329CB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CB54: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CB58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CB60: 386A8B90  addi r3, r10, -0x7470
	ctx.r[3].s64 = ctx.r[10].s64 + -29808;
	// 8329CB64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CB7C: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 8329CB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CB84: 4BAB90BD  bl 0x82d55c40
	ctx.lr = 0x8329CB88;
	sub_82D55C40(ctx, base);
	// 8329CB88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CB98 size=112
    let mut pc: u32 = 0x8329CB98;
    'dispatch: loop {
        match pc {
            0x8329CB98 => {
    //   block [0x8329CB98..0x8329CC08)
	// 8329CB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CBA4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CBA8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CBAC: 38AA8960  addi r5, r10, -0x76a0
	ctx.r[5].s64 = ctx.r[10].s64 + -30368;
	// 8329CBB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CBB4: 390BA9D0  addi r8, r11, -0x5630
	ctx.r[8].s64 = ctx.r[11].s64 + -22064;
	// 8329CBB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329CBBC: 388AAA6C  addi r4, r10, -0x5594
	ctx.r[4].s64 = ctx.r[10].s64 + -21908;
	// 8329CBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CBC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CBC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CBCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CBD0: 386A8BC0  addi r3, r10, -0x7440
	ctx.r[3].s64 = ctx.r[10].s64 + -29760;
	// 8329CBD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CBE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CBEC: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8329CBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CBF4: 4BAB904D  bl 0x82d55c40
	ctx.lr = 0x8329CBF8;
	sub_82D55C40(ctx, base);
	// 8329CBF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CBFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CC08 size=112
    let mut pc: u32 = 0x8329CC08;
    'dispatch: loop {
        match pc {
            0x8329CC08 => {
    //   block [0x8329CC08..0x8329CC78)
	// 8329CC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CC14: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CC18: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CC1C: 38AA8AC0  addi r5, r10, -0x7540
	ctx.r[5].s64 = ctx.r[10].s64 + -30016;
	// 8329CC20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CC24: 390BAA88  addi r8, r11, -0x5578
	ctx.r[8].s64 = ctx.r[11].s64 + -21880;
	// 8329CC28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329CC2C: 388AAAA0  addi r4, r10, -0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + -21856;
	// 8329CC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CC34: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CC38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CC40: 386A8BF0  addi r3, r10, -0x7410
	ctx.r[3].s64 = ctx.r[10].s64 + -29712;
	// 8329CC44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CC5C: 38C00150  li r6, 0x150
	ctx.r[6].s64 = 336;
	// 8329CC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CC64: 4BAB8FDD  bl 0x82d55c40
	ctx.lr = 0x8329CC68;
	sub_82D55C40(ctx, base);
	// 8329CC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CC78 size=108
    let mut pc: u32 = 0x8329CC78;
    'dispatch: loop {
        match pc {
            0x8329CC78 => {
    //   block [0x8329CC78..0x8329CCE4)
	// 8329CC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CC84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CC88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CC8C: 38EBAAF8  addi r7, r11, -0x5508
	ctx.r[7].s64 = ctx.r[11].s64 + -21768;
	// 8329CC90: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8329CC94: 388AAC94  addi r4, r10, -0x536c
	ctx.r[4].s64 = ctx.r[10].s64 + -21356;
	// 8329CC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CC9C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CCA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329CCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CCA8: 386A8C20  addi r3, r10, -0x73e0
	ctx.r[3].s64 = ctx.r[10].s64 + -29664;
	// 8329CCAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329CCB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CCBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CCC4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329CCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CCCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329CCD0: 4BAB8F71  bl 0x82d55c40
	ctx.lr = 0x8329CCD4;
	sub_82D55C40(ctx, base);
	// 8329CCD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CCD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CCDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CCE8 size=92
    let mut pc: u32 = 0x8329CCE8;
    'dispatch: loop {
        match pc {
            0x8329CCE8 => {
    //   block [0x8329CCE8..0x8329CD44)
	// 8329CCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CCF0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CCF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329CCF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329CCFC: 4BB039F5  bl 0x82da06f0
	ctx.lr = 0x8329CD00;
	sub_82DA06F0(ctx, base);
	// 8329CD00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CD04: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329CD08: 394BACB8  addi r10, r11, -0x5348
	ctx.r[10].s64 = ctx.r[11].s64 + -21320;
	// 8329CD0C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329CD10: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329CD14: 396B8C50  addi r11, r11, -0x73b0
	ctx.r[11].s64 = ctx.r[11].s64 + -29616;
	// 8329CD18: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329CD1C: 39481DF0  addi r10, r8, 0x1df0
	ctx.r[10].s64 = ctx.r[8].s64 + 7664;
	// 8329CD20: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329CD24: 39491E08  addi r10, r9, 0x1e08
	ctx.r[10].s64 = ctx.r[9].s64 + 7688;
	// 8329CD28: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329CD2C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329CD30: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329CD34: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8329CD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CD48 size=112
    let mut pc: u32 = 0x8329CD48;
    'dispatch: loop {
        match pc {
            0x8329CD48 => {
    //   block [0x8329CD48..0x8329CDB8)
	// 8329CD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CD54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CD58: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CD5C: 396BAB88  addi r11, r11, -0x5478
	ctx.r[11].s64 = ctx.r[11].s64 + -21624;
	// 8329CD60: 38AA8630  addi r5, r10, -0x79d0
	ctx.r[5].s64 = ctx.r[10].s64 + -31184;
	// 8329CD64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CD68: 390B00D8  addi r8, r11, 0xd8
	ctx.r[8].s64 = ctx.r[11].s64 + 216;
	// 8329CD6C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8329CD70: 388AACB8  addi r4, r10, -0x5348
	ctx.r[4].s64 = ctx.r[10].s64 + -21320;
	// 8329CD74: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329CD78: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CD7C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CD80: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329CD84: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329CD88: 386A8C60  addi r3, r10, -0x73a0
	ctx.r[3].s64 = ctx.r[10].s64 + -29600;
	// 8329CD8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329CD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CD94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CD98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329CD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CDA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329CDA4: 4BAB8E9D  bl 0x82d55c40
	ctx.lr = 0x8329CDA8;
	sub_82D55C40(ctx, base);
	// 8329CDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329CDB8 size=24
    let mut pc: u32 = 0x8329CDB8;
    'dispatch: loop {
        match pc {
            0x8329CDB8 => {
    //   block [0x8329CDB8..0x8329CDD0)
	// 8329CDB8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329CDBC: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329CDC0: 394A8D90  addi r10, r10, -0x7270
	ctx.r[10].s64 = ctx.r[10].s64 + -29296;
	// 8329CDC4: 816B8D78  lwz r11, -0x7288(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29320 as u32) ) } as u64;
	// 8329CDC8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329CDCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CDD0 size=112
    let mut pc: u32 = 0x8329CDD0;
    'dispatch: loop {
        match pc {
            0x8329CDD0 => {
    //   block [0x8329CDD0..0x8329CE40)
	// 8329CDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CDDC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CDE0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329CDE4: 392AAD74  addi r9, r10, -0x528c
	ctx.r[9].s64 = ctx.r[10].s64 + -21132;
	// 8329CDE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CDEC: 390B8D90  addi r8, r11, -0x7270
	ctx.r[8].s64 = ctx.r[11].s64 + -29296;
	// 8329CDF0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8329CDF4: 388AAD88  addi r4, r10, -0x5278
	ctx.r[4].s64 = ctx.r[10].s64 + -21112;
	// 8329CDF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CDFC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CE00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CE04: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329CE08: 386A8C90  addi r3, r10, -0x7370
	ctx.r[3].s64 = ctx.r[10].s64 + -29552;
	// 8329CE0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329CE10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329CE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CE18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CE20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329CE28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CE2C: 4BAB8E15  bl 0x82d55c40
	ctx.lr = 0x8329CE30;
	sub_82D55C40(ctx, base);
	// 8329CE30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CE3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CE40 size=112
    let mut pc: u32 = 0x8329CE40;
    'dispatch: loop {
        match pc {
            0x8329CE40 => {
    //   block [0x8329CE40..0x8329CEB0)
	// 8329CE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CE4C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CE50: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CE54: 38AA8440  addi r5, r10, -0x7bc0
	ctx.r[5].s64 = ctx.r[10].s64 + -31680;
	// 8329CE58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CE5C: 390BADAC  addi r8, r11, -0x5254
	ctx.r[8].s64 = ctx.r[11].s64 + -21076;
	// 8329CE60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329CE64: 388AADC4  addi r4, r10, -0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + -21052;
	// 8329CE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CE6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CE78: 386A8CC0  addi r3, r10, -0x7340
	ctx.r[3].s64 = ctx.r[10].s64 + -29504;
	// 8329CE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CE94: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329CE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CE9C: 4BAB8DA5  bl 0x82d55c40
	ctx.lr = 0x8329CEA0;
	sub_82D55C40(ctx, base);
	// 8329CEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CEB0 size=108
    let mut pc: u32 = 0x8329CEB0;
    'dispatch: loop {
        match pc {
            0x8329CEB0 => {
    //   block [0x8329CEB0..0x8329CF1C)
	// 8329CEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CEBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CEC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CEC4: 392BAE30  addi r9, r11, -0x51d0
	ctx.r[9].s64 = ctx.r[11].s64 + -20944;
	// 8329CEC8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8329CECC: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329CED0: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 8329CED4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CED8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CEDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329CEE0: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 8329CEE4: 386A8CF0  addi r3, r10, -0x7310
	ctx.r[3].s64 = ctx.r[10].s64 + -29456;
	// 8329CEE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CEEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329CEF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CEF4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CEF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CEFC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CF00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329CF04: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CF08: 4BAB8D39  bl 0x82d55c40
	ctx.lr = 0x8329CF0C;
	sub_82D55C40(ctx, base);
	// 8329CF0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CF20 size=112
    let mut pc: u32 = 0x8329CF20;
    'dispatch: loop {
        match pc {
            0x8329CF20 => {
    //   block [0x8329CF20..0x8329CF90)
	// 8329CF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CF2C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CF30: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CF34: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329CF38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CF3C: 390BAEF0  addi r8, r11, -0x5110
	ctx.r[8].s64 = ctx.r[11].s64 + -20752;
	// 8329CF40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329CF44: 388AAF5C  addi r4, r10, -0x50a4
	ctx.r[4].s64 = ctx.r[10].s64 + -20644;
	// 8329CF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CF4C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329CF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CF58: 386A8D20  addi r3, r10, -0x72e0
	ctx.r[3].s64 = ctx.r[10].s64 + -29408;
	// 8329CF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329CF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329CF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329CF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CF74: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 8329CF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CF7C: 4BAB8CC5  bl 0x82d55c40
	ctx.lr = 0x8329CF80;
	sub_82D55C40(ctx, base);
	// 8329CF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329CF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329CF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329CF90 size=116
    let mut pc: u32 = 0x8329CF90;
    'dispatch: loop {
        match pc {
            0x8329CF90 => {
    //   block [0x8329CF90..0x8329D004)
	// 8329CF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329CF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329CF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329CF9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329CFA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CFA4: 390BB098  addi r8, r11, -0x4f68
	ctx.r[8].s64 = ctx.r[11].s64 + -20328;
	// 8329CFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329CFAC: 392AB084  addi r9, r10, -0x4f7c
	ctx.r[9].s64 = ctx.r[10].s64 + -20348;
	// 8329CFB0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329CFB4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8329CFB8: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329CFBC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329CFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329CFC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329CFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329CFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329CFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329CFD4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329CFD8: 388AB128  addi r4, r10, -0x4ed8
	ctx.r[4].s64 = ctx.r[10].s64 + -20184;
	// 8329CFDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329CFE0: 386B8D50  addi r3, r11, -0x72b0
	ctx.r[3].s64 = ctx.r[11].s64 + -29360;
	// 8329CFE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329CFE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329CFEC: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 8329CFF0: 4BAB8C51  bl 0x82d55c40
	ctx.lr = 0x8329CFF4;
	sub_82D55C40(ctx, base);
	// 8329CFF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329CFF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329CFFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D008 size=100
    let mut pc: u32 = 0x8329D008;
    'dispatch: loop {
        match pc {
            0x8329D008 => {
    //   block [0x8329D008..0x8329D06C)
	// 8329D008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D014: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D01C: 38AA89F0  addi r5, r10, -0x7610
	ctx.r[5].s64 = ctx.r[10].s64 + -30224;
	// 8329D020: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D028: 388AB1A0  addi r4, r10, -0x4e60
	ctx.r[4].s64 = ctx.r[10].s64 + -20064;
	// 8329D02C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D03C: 386A8D80  addi r3, r10, -0x7280
	ctx.r[3].s64 = ctx.r[10].s64 + -29312;
	// 8329D040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D044: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D048: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329D04C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D050: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329D054: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329D058: 4BAB8BE9  bl 0x82d55c40
	ctx.lr = 0x8329D05C;
	sub_82D55C40(ctx, base);
	// 8329D05C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D070 size=108
    let mut pc: u32 = 0x8329D070;
    'dispatch: loop {
        match pc {
            0x8329D070 => {
    //   block [0x8329D070..0x8329D0DC)
	// 8329D070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D07C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D084: 38EBB1C8  addi r7, r11, -0x4e38
	ctx.r[7].s64 = ctx.r[11].s64 + -20024;
	// 8329D088: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329D08C: 388AB244  addi r4, r10, -0x4dbc
	ctx.r[4].s64 = ctx.r[10].s64 + -19900;
	// 8329D090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D094: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329D09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D0A0: 386A8DB0  addi r3, r10, -0x7250
	ctx.r[3].s64 = ctx.r[10].s64 + -29264;
	// 8329D0A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329D0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D0BC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329D0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D0C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D0C8: 4BAB8B79  bl 0x82d55c40
	ctx.lr = 0x8329D0CC;
	sub_82D55C40(ctx, base);
	// 8329D0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D0E0 size=112
    let mut pc: u32 = 0x8329D0E0;
    'dispatch: loop {
        match pc {
            0x8329D0E0 => {
    //   block [0x8329D0E0..0x8329D150)
	// 8329D0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D0EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D0F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D0F4: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329D0F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D0FC: 390BB1F8  addi r8, r11, -0x4e08
	ctx.r[8].s64 = ctx.r[11].s64 + -19976;
	// 8329D100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329D104: 388AB264  addi r4, r10, -0x4d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -19868;
	// 8329D108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D10C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D118: 386A8DE0  addi r3, r10, -0x7220
	ctx.r[3].s64 = ctx.r[10].s64 + -29216;
	// 8329D11C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D134: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 8329D138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D13C: 4BAB8B05  bl 0x82d55c40
	ctx.lr = 0x8329D140;
	sub_82D55C40(ctx, base);
	// 8329D140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D150 size=92
    let mut pc: u32 = 0x8329D150;
    'dispatch: loop {
        match pc {
            0x8329D150 => {
    //   block [0x8329D150..0x8329D1AC)
	// 8329D150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D158: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D15C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329D160: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329D164: 4BB04F15  bl 0x82da2078
	ctx.lr = 0x8329D168;
	sub_82DA2078(ctx, base);
	// 8329D168: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D16C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329D170: 394BB3B8  addi r10, r11, -0x4c48
	ctx.r[10].s64 = ctx.r[11].s64 + -19528;
	// 8329D174: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D178: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329D17C: 396B8E10  addi r11, r11, -0x71f0
	ctx.r[11].s64 = ctx.r[11].s64 + -29168;
	// 8329D180: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329D184: 39482030  addi r10, r8, 0x2030
	ctx.r[10].s64 = ctx.r[8].s64 + 8240;
	// 8329D188: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329D18C: 39492018  addi r10, r9, 0x2018
	ctx.r[10].s64 = ctx.r[9].s64 + 8216;
	// 8329D190: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329D194: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329D198: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329D19C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8329D1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D1B0 size=112
    let mut pc: u32 = 0x8329D1B0;
    'dispatch: loop {
        match pc {
            0x8329D1B0 => {
    //   block [0x8329D1B0..0x8329D220)
	// 8329D1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D1BC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D1C0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D1C4: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329D1C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D1CC: 390BB2F8  addi r8, r11, -0x4d08
	ctx.r[8].s64 = ctx.r[11].s64 + -19720;
	// 8329D1D0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8329D1D4: 388AB3B8  addi r4, r10, -0x4c48
	ctx.r[4].s64 = ctx.r[10].s64 + -19528;
	// 8329D1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D1DC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D1E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D1E8: 386A8E20  addi r3, r10, -0x71e0
	ctx.r[3].s64 = ctx.r[10].s64 + -29152;
	// 8329D1EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D204: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 8329D208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D20C: 4BAB8A35  bl 0x82d55c40
	ctx.lr = 0x8329D210;
	sub_82D55C40(ctx, base);
	// 8329D210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D220 size=108
    let mut pc: u32 = 0x8329D220;
    'dispatch: loop {
        match pc {
            0x8329D220 => {
    //   block [0x8329D220..0x8329D28C)
	// 8329D220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D22C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D234: 392BB4AC  addi r9, r11, -0x4b54
	ctx.r[9].s64 = ctx.r[11].s64 + -19284;
	// 8329D238: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8329D23C: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 8329D240: 388AB5B4  addi r4, r10, -0x4a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19020;
	// 8329D244: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D248: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D24C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329D250: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 8329D254: 386A8E50  addi r3, r10, -0x71b0
	ctx.r[3].s64 = ctx.r[10].s64 + -29104;
	// 8329D258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D25C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329D260: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D264: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D268: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D26C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D270: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D274: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D278: 4BAB89C9  bl 0x82d55c40
	ctx.lr = 0x8329D27C;
	sub_82D55C40(ctx, base);
	// 8329D27C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D290 size=112
    let mut pc: u32 = 0x8329D290;
    'dispatch: loop {
        match pc {
            0x8329D290 => {
    //   block [0x8329D290..0x8329D300)
	// 8329D290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D29C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D2A0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D2A4: 392BB480  addi r9, r11, -0x4b80
	ctx.r[9].s64 = ctx.r[11].s64 + -19328;
	// 8329D2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D2AC: 390900E8  addi r8, r9, 0xe8
	ctx.r[8].s64 = ctx.r[9].s64 + 232;
	// 8329D2B0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329D2B4: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329D2B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D2BC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D2C0: 38C00140  li r6, 0x140
	ctx.r[6].s64 = 320;
	// 8329D2C4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D2C8: 388AB5D4  addi r4, r10, -0x4a2c
	ctx.r[4].s64 = ctx.r[10].s64 + -18988;
	// 8329D2CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D2D0: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D2D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329D2D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329D2DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D2E0: 386B8E80  addi r3, r11, -0x7180
	ctx.r[3].s64 = ctx.r[11].s64 + -29056;
	// 8329D2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D2E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D2EC: 4BAB8955  bl 0x82d55c40
	ctx.lr = 0x8329D2F0;
	sub_82D55C40(ctx, base);
	// 8329D2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D300 size=92
    let mut pc: u32 = 0x8329D300;
    'dispatch: loop {
        match pc {
            0x8329D300 => {
    //   block [0x8329D300..0x8329D35C)
	// 8329D300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D308: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D30C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329D310: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329D314: 4BAE4E9D  bl 0x82d821b0
	ctx.lr = 0x8329D318;
	sub_82D821B0(ctx, base);
	// 8329D318: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D31C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329D320: 394BB61C  addi r10, r11, -0x49e4
	ctx.r[10].s64 = ctx.r[11].s64 + -18916;
	// 8329D324: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D328: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329D32C: 396B8EB0  addi r11, r11, -0x7150
	ctx.r[11].s64 = ctx.r[11].s64 + -29008;
	// 8329D330: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329D334: 39482168  addi r10, r8, 0x2168
	ctx.r[10].s64 = ctx.r[8].s64 + 8552;
	// 8329D338: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329D33C: 39492150  addi r10, r9, 0x2150
	ctx.r[10].s64 = ctx.r[9].s64 + 8528;
	// 8329D340: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329D344: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329D348: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329D34C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 8329D350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D360 size=112
    let mut pc: u32 = 0x8329D360;
    'dispatch: loop {
        match pc {
            0x8329D360 => {
    //   block [0x8329D360..0x8329D3D0)
	// 8329D360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D36C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D370: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D374: 38AA8BF0  addi r5, r10, -0x7410
	ctx.r[5].s64 = ctx.r[10].s64 + -29712;
	// 8329D378: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D37C: 390BB604  addi r8, r11, -0x49fc
	ctx.r[8].s64 = ctx.r[11].s64 + -18940;
	// 8329D380: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329D384: 388AB61C  addi r4, r10, -0x49e4
	ctx.r[4].s64 = ctx.r[10].s64 + -18916;
	// 8329D388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D38C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D398: 386A8EC0  addi r3, r10, -0x7140
	ctx.r[3].s64 = ctx.r[10].s64 + -28992;
	// 8329D39C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D3AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D3B4: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 8329D3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D3BC: 4BAB8885  bl 0x82d55c40
	ctx.lr = 0x8329D3C0;
	sub_82D55C40(ctx, base);
	// 8329D3C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D3C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D3C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D3CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D3D0 size=108
    let mut pc: u32 = 0x8329D3D0;
    'dispatch: loop {
        match pc {
            0x8329D3D0 => {
    //   block [0x8329D3D0..0x8329D43C)
	// 8329D3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D3DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D3E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D3E4: 392BB650  addi r9, r11, -0x49b0
	ctx.r[9].s64 = ctx.r[11].s64 + -18864;
	// 8329D3E8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8329D3EC: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329D3F0: 388AB714  addi r4, r10, -0x48ec
	ctx.r[4].s64 = ctx.r[10].s64 + -18668;
	// 8329D3F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D3F8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D3FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329D400: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 8329D404: 386A8EF0  addi r3, r10, -0x7110
	ctx.r[3].s64 = ctx.r[10].s64 + -28944;
	// 8329D408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D40C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329D410: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D414: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D418: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D41C: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D420: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D424: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D428: 4BAB8819  bl 0x82d55c40
	ctx.lr = 0x8329D42C;
	sub_82D55C40(ctx, base);
	// 8329D42C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D440 size=112
    let mut pc: u32 = 0x8329D440;
    'dispatch: loop {
        match pc {
            0x8329D440 => {
    //   block [0x8329D440..0x8329D4B0)
	// 8329D440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D44C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D450: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D454: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329D458: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D45C: 390BB6C8  addi r8, r11, -0x4938
	ctx.r[8].s64 = ctx.r[11].s64 + -18744;
	// 8329D460: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329D464: 388AB730  addi r4, r10, -0x48d0
	ctx.r[4].s64 = ctx.r[10].s64 + -18640;
	// 8329D468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D46C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D478: 386A8F20  addi r3, r10, -0x70e0
	ctx.r[3].s64 = ctx.r[10].s64 + -28896;
	// 8329D47C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D48C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D494: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 8329D498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D49C: 4BAB87A5  bl 0x82d55c40
	ctx.lr = 0x8329D4A0;
	sub_82D55C40(ctx, base);
	// 8329D4A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D4B0 size=108
    let mut pc: u32 = 0x8329D4B0;
    'dispatch: loop {
        match pc {
            0x8329D4B0 => {
    //   block [0x8329D4B0..0x8329D51C)
	// 8329D4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D4BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D4C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D4C4: 38EBB760  addi r7, r11, -0x48a0
	ctx.r[7].s64 = ctx.r[11].s64 + -18592;
	// 8329D4C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329D4CC: 388AB7C0  addi r4, r10, -0x4840
	ctx.r[4].s64 = ctx.r[10].s64 + -18496;
	// 8329D4D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D4D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D4D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329D4DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D4E0: 386A8F50  addi r3, r10, -0x70b0
	ctx.r[3].s64 = ctx.r[10].s64 + -28848;
	// 8329D4E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329D4E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D4EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D4F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D4F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D4F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D4FC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329D500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D508: 4BAB8739  bl 0x82d55c40
	ctx.lr = 0x8329D50C;
	sub_82D55C40(ctx, base);
	// 8329D50C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D520 size=108
    let mut pc: u32 = 0x8329D520;
    'dispatch: loop {
        match pc {
            0x8329D520 => {
    //   block [0x8329D520..0x8329D58C)
	// 8329D520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D52C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D534: 38EBB778  addi r7, r11, -0x4888
	ctx.r[7].s64 = ctx.r[11].s64 + -18568;
	// 8329D538: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329D53C: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 8329D540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D544: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329D54C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D550: 386A8F80  addi r3, r10, -0x7080
	ctx.r[3].s64 = ctx.r[10].s64 + -28800;
	// 8329D554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329D558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D56C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329D570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D578: 4BAB86C9  bl 0x82d55c40
	ctx.lr = 0x8329D57C;
	sub_82D55C40(ctx, base);
	// 8329D57C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329D590 size=24
    let mut pc: u32 = 0x8329D590;
    'dispatch: loop {
        match pc {
            0x8329D590 => {
    //   block [0x8329D590..0x8329D5A8)
	// 8329D590: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329D594: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329D598: 394A8FA0  addi r10, r10, -0x7060
	ctx.r[10].s64 = ctx.r[10].s64 + -28768;
	// 8329D59C: 816B8F88  lwz r11, -0x7078(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28792 as u32) ) } as u64;
	// 8329D5A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329D5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D5A8 size=116
    let mut pc: u32 = 0x8329D5A8;
    'dispatch: loop {
        match pc {
            0x8329D5A8 => {
    //   block [0x8329D5A8..0x8329D61C)
	// 8329D5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D5B4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329D5B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D5BC: 390B8FA0  addi r8, r11, -0x7060
	ctx.r[8].s64 = ctx.r[11].s64 + -28768;
	// 8329D5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D5C4: 392AB8D8  addi r9, r10, -0x4728
	ctx.r[9].s64 = ctx.r[10].s64 + -18216;
	// 8329D5C8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D5CC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329D5D0: 38AA8960  addi r5, r10, -0x76a0
	ctx.r[5].s64 = ctx.r[10].s64 + -30368;
	// 8329D5D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D5D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D5DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D5E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D5E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D5EC: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D5F0: 388AB900  addi r4, r10, -0x4700
	ctx.r[4].s64 = ctx.r[10].s64 + -18176;
	// 8329D5F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329D5F8: 386B8FB0  addi r3, r11, -0x7050
	ctx.r[3].s64 = ctx.r[11].s64 + -28752;
	// 8329D5FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329D600: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D604: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8329D608: 4BAB8639  bl 0x82d55c40
	ctx.lr = 0x8329D60C;
	sub_82D55C40(ctx, base);
	// 8329D60C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D620 size=96
    let mut pc: u32 = 0x8329D620;
    'dispatch: loop {
        match pc {
            0x8329D620 => {
    //   block [0x8329D620..0x8329D680)
	// 8329D620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D628: 9421FDA0  stwu r1, -0x260(r1)
	ea = ctx.r[1].u32.wrapping_add(-608 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D62C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329D630: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329D634: 4BAE84FD  bl 0x82d85b30
	ctx.lr = 0x8329D638;
	sub_82D85B30(ctx, base);
	// 8329D638: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D63C: 3CE082D8  lis r7, -0x7d28
	ctx.r[7].s64 = -2099773440;
	// 8329D640: 394BB960  addi r10, r11, -0x46a0
	ctx.r[10].s64 = ctx.r[11].s64 + -18080;
	// 8329D644: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D648: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329D64C: 392BB944  addi r9, r11, -0x46bc
	ctx.r[9].s64 = ctx.r[11].s64 + -18108;
	// 8329D650: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D654: 396B8FE0  addi r11, r11, -0x7020
	ctx.r[11].s64 = ctx.r[11].s64 + -28704;
	// 8329D658: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329D65C: 39472360  addi r10, r7, 0x2360
	ctx.r[10].s64 = ctx.r[7].s64 + 9056;
	// 8329D660: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329D664: 39482348  addi r10, r8, 0x2348
	ctx.r[10].s64 = ctx.r[8].s64 + 9032;
	// 8329D668: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329D66C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8329D670: 38210260  addi r1, r1, 0x260
	ctx.r[1].s64 = ctx.r[1].s64 + 608;
	// 8329D674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D680 size=100
    let mut pc: u32 = 0x8329D680;
    'dispatch: loop {
        match pc {
            0x8329D680 => {
    //   block [0x8329D680..0x8329D6E4)
	// 8329D680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D68C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D694: 38AA8860  addi r5, r10, -0x77a0
	ctx.r[5].s64 = ctx.r[10].s64 + -30624;
	// 8329D698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D69C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D6A0: 388AB960  addi r4, r10, -0x46a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18080;
	// 8329D6A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D6A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D6AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D6B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D6B4: 386A8FF0  addi r3, r10, -0x7010
	ctx.r[3].s64 = ctx.r[10].s64 + -28688;
	// 8329D6B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D6BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D6C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329D6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D6C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329D6CC: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 8329D6D0: 4BAB8571  bl 0x82d55c40
	ctx.lr = 0x8329D6D4;
	sub_82D55C40(ctx, base);
	// 8329D6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D6E8 size=100
    let mut pc: u32 = 0x8329D6E8;
    'dispatch: loop {
        match pc {
            0x8329D6E8 => {
    //   block [0x8329D6E8..0x8329D74C)
	// 8329D6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D6F4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D6FC: 38AA89F0  addi r5, r10, -0x7610
	ctx.r[5].s64 = ctx.r[10].s64 + -30224;
	// 8329D700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D708: 388AB9D8  addi r4, r10, -0x4628
	ctx.r[4].s64 = ctx.r[10].s64 + -17960;
	// 8329D70C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D71C: 386A9020  addi r3, r10, -0x6fe0
	ctx.r[3].s64 = ctx.r[10].s64 + -28640;
	// 8329D720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D724: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D728: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329D72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D730: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329D734: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329D738: 4BAB8509  bl 0x82d55c40
	ctx.lr = 0x8329D73C;
	sub_82D55C40(ctx, base);
	// 8329D73C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D750 size=108
    let mut pc: u32 = 0x8329D750;
    'dispatch: loop {
        match pc {
            0x8329D750 => {
    //   block [0x8329D750..0x8329D7BC)
	// 8329D750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D75C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D760: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D764: 38EBB9EC  addi r7, r11, -0x4614
	ctx.r[7].s64 = ctx.r[11].s64 + -17940;
	// 8329D768: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329D76C: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 8329D770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D774: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D778: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329D77C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D780: 386A9050  addi r3, r10, -0x6fb0
	ctx.r[3].s64 = ctx.r[10].s64 + -28592;
	// 8329D784: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329D788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D78C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D79C: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 8329D7A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D7A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D7A8: 4BAB8499  bl 0x82d55c40
	ctx.lr = 0x8329D7AC;
	sub_82D55C40(ctx, base);
	// 8329D7AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D7C0 size=112
    let mut pc: u32 = 0x8329D7C0;
    'dispatch: loop {
        match pc {
            0x8329D7C0 => {
    //   block [0x8329D7C0..0x8329D830)
	// 8329D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D7CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D7D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D7D4: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329D7D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D7DC: 390BBA1C  addi r8, r11, -0x45e4
	ctx.r[8].s64 = ctx.r[11].s64 + -17892;
	// 8329D7E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329D7E4: 388ABA8C  addi r4, r10, -0x4574
	ctx.r[4].s64 = ctx.r[10].s64 + -17780;
	// 8329D7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D7EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D7F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D7F8: 386A9080  addi r3, r10, -0x6f80
	ctx.r[3].s64 = ctx.r[10].s64 + -28544;
	// 8329D7FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D814: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 8329D818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D81C: 4BAB8425  bl 0x82d55c40
	ctx.lr = 0x8329D820;
	sub_82D55C40(ctx, base);
	// 8329D820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D830 size=112
    let mut pc: u32 = 0x8329D830;
    'dispatch: loop {
        match pc {
            0x8329D830 => {
    //   block [0x8329D830..0x8329D8A0)
	// 8329D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D83C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D840: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D844: 38AA91B0  addi r5, r10, -0x6e50
	ctx.r[5].s64 = ctx.r[10].s64 + -28240;
	// 8329D848: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D84C: 390BBAC0  addi r8, r11, -0x4540
	ctx.r[8].s64 = ctx.r[11].s64 + -17728;
	// 8329D850: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329D854: 388ABB30  addi r4, r10, -0x44d0
	ctx.r[4].s64 = ctx.r[10].s64 + -17616;
	// 8329D858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D85C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D868: 386A90B0  addi r3, r10, -0x6f50
	ctx.r[3].s64 = ctx.r[10].s64 + -28496;
	// 8329D86C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D884: 38C0003C  li r6, 0x3c
	ctx.r[6].s64 = 60;
	// 8329D888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D88C: 4BAB83B5  bl 0x82d55c40
	ctx.lr = 0x8329D890;
	sub_82D55C40(ctx, base);
	// 8329D890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D8A0 size=108
    let mut pc: u32 = 0x8329D8A0;
    'dispatch: loop {
        match pc {
            0x8329D8A0 => {
    //   block [0x8329D8A0..0x8329D90C)
	// 8329D8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D8AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D8B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D8B4: 38EBBB4C  addi r7, r11, -0x44b4
	ctx.r[7].s64 = ctx.r[11].s64 + -17588;
	// 8329D8B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329D8BC: 388ABB7C  addi r4, r10, -0x4484
	ctx.r[4].s64 = ctx.r[10].s64 + -17540;
	// 8329D8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D8C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D8C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329D8CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D8D0: 386A90E0  addi r3, r10, -0x6f20
	ctx.r[3].s64 = ctx.r[10].s64 + -28448;
	// 8329D8D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329D8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D8DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D8E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D8EC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329D8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D8F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329D8F8: 4BAB8349  bl 0x82d55c40
	ctx.lr = 0x8329D8FC;
	sub_82D55C40(ctx, base);
	// 8329D8FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D910 size=92
    let mut pc: u32 = 0x8329D910;
    'dispatch: loop {
        match pc {
            0x8329D910 => {
    //   block [0x8329D910..0x8329D96C)
	// 8329D910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D918: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D91C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329D920: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329D924: 4BAE4D3D  bl 0x82d82660
	ctx.lr = 0x8329D928;
	sub_82D82660(ctx, base);
	// 8329D928: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D92C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329D930: 394BBBA4  addi r10, r11, -0x445c
	ctx.r[10].s64 = ctx.r[11].s64 + -17500;
	// 8329D934: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329D938: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329D93C: 396B9110  addi r11, r11, -0x6ef0
	ctx.r[11].s64 = ctx.r[11].s64 + -28400;
	// 8329D940: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329D944: 39482618  addi r10, r8, 0x2618
	ctx.r[10].s64 = ctx.r[8].s64 + 9752;
	// 8329D948: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329D94C: 39492600  addi r10, r9, 0x2600
	ctx.r[10].s64 = ctx.r[9].s64 + 9728;
	// 8329D950: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329D954: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329D958: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329D95C: 382101C0  addi r1, r1, 0x1c0
	ctx.r[1].s64 = ctx.r[1].s64 + 448;
	// 8329D960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D970 size=112
    let mut pc: u32 = 0x8329D970;
    'dispatch: loop {
        match pc {
            0x8329D970 => {
    //   block [0x8329D970..0x8329D9E0)
	// 8329D970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D97C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D980: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D984: 38AA8BF0  addi r5, r10, -0x7410
	ctx.r[5].s64 = ctx.r[10].s64 + -29712;
	// 8329D988: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D98C: 390BBB64  addi r8, r11, -0x449c
	ctx.r[8].s64 = ctx.r[11].s64 + -17564;
	// 8329D990: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329D994: 388ABBA4  addi r4, r10, -0x445c
	ctx.r[4].s64 = ctx.r[10].s64 + -17500;
	// 8329D998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329D99C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D9A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329D9A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329D9A8: 386A9120  addi r3, r10, -0x6ee0
	ctx.r[3].s64 = ctx.r[10].s64 + -28384;
	// 8329D9AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329D9B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329D9B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329D9B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329D9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329D9C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329D9C4: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 8329D9C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329D9CC: 4BAB8275  bl 0x82d55c40
	ctx.lr = 0x8329D9D0;
	sub_82D55C40(ctx, base);
	// 8329D9D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329D9D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329D9D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329D9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329D9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329D9E0 size=112
    let mut pc: u32 = 0x8329D9E0;
    'dispatch: loop {
        match pc {
            0x8329D9E0 => {
    //   block [0x8329D9E0..0x8329DA50)
	// 8329D9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329D9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329D9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329D9EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329D9F0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329D9F4: 38AA8440  addi r5, r10, -0x7bc0
	ctx.r[5].s64 = ctx.r[10].s64 + -31680;
	// 8329D9F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329D9FC: 390BBBD0  addi r8, r11, -0x4430
	ctx.r[8].s64 = ctx.r[11].s64 + -17456;
	// 8329DA00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329DA04: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 8329DA08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DA0C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DA10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329DA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DA18: 386A9150  addi r3, r10, -0x6eb0
	ctx.r[3].s64 = ctx.r[10].s64 + -28336;
	// 8329DA1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329DA20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DA28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DA2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DA30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DA34: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329DA38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DA3C: 4BAB8205  bl 0x82d55c40
	ctx.lr = 0x8329DA40;
	sub_82D55C40(ctx, base);
	// 8329DA40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DA4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329DA50 size=36
    let mut pc: u32 = 0x8329DA50;
    'dispatch: loop {
        match pc {
            0x8329DA50 => {
    //   block [0x8329DA50..0x8329DA74)
	// 8329DA50: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329DA54: 814B91C8  lwz r10, -0x6e38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28216 as u32) ) } as u64;
	// 8329DA58: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329DA5C: 396B91E0  addi r11, r11, -0x6e20
	ctx.r[11].s64 = ctx.r[11].s64 + -28192;
	// 8329DA60: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8329DA64: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329DA68: 814A91C0  lwz r10, -0x6e40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28224 as u32) ) } as u64;
	// 8329DA6C: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 8329DA70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DA78 size=116
    let mut pc: u32 = 0x8329DA78;
    'dispatch: loop {
        match pc {
            0x8329DA78 => {
    //   block [0x8329DA78..0x8329DAEC)
	// 8329DA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DA84: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DA88: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329DA8C: 392ABDB8  addi r9, r10, -0x4248
	ctx.r[9].s64 = ctx.r[10].s64 + -16968;
	// 8329DA90: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329DA94: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329DA98: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329DA9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DAA0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 8329DAA4: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329DAA8: 388ABE20  addi r4, r10, -0x41e0
	ctx.r[4].s64 = ctx.r[10].s64 + -16864;
	// 8329DAAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DAB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8329DAB4: 396B91E0  addi r11, r11, -0x6e20
	ctx.r[11].s64 = ctx.r[11].s64 + -28192;
	// 8329DAB8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DAC0: 386A9180  addi r3, r10, -0x6e80
	ctx.r[3].s64 = ctx.r[10].s64 + -28288;
	// 8329DAC4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329DAC8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8329DACC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8329DAD0: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329DAD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329DAD8: 4BAB8169  bl 0x82d55c40
	ctx.lr = 0x8329DADC;
	sub_82D55C40(ctx, base);
	// 8329DADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329DAF0 size=24
    let mut pc: u32 = 0x8329DAF0;
    'dispatch: loop {
        match pc {
            0x8329DAF0 => {
    //   block [0x8329DAF0..0x8329DB08)
	// 8329DAF0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329DAF4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329DAF8: 394A92E0  addi r10, r10, -0x6d20
	ctx.r[10].s64 = ctx.r[10].s64 + -27936;
	// 8329DAFC: 816B92C0  lwz r11, -0x6d40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27968 as u32) ) } as u64;
	// 8329DB00: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8329DB04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DB08 size=116
    let mut pc: u32 = 0x8329DB08;
    'dispatch: loop {
        match pc {
            0x8329DB08 => {
    //   block [0x8329DB08..0x8329DB7C)
	// 8329DB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DB14: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329DB18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DB1C: 390B92E0  addi r8, r11, -0x6d20
	ctx.r[8].s64 = ctx.r[11].s64 + -27936;
	// 8329DB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DB24: 392AC020  addi r9, r10, -0x3fe0
	ctx.r[9].s64 = ctx.r[10].s64 + -16352;
	// 8329DB28: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329DB2C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8329DB30: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329DB34: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329DB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DB3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DB4C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329DB50: 388AC070  addi r4, r10, -0x3f90
	ctx.r[4].s64 = ctx.r[10].s64 + -16272;
	// 8329DB54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329DB58: 386B91B0  addi r3, r11, -0x6e50
	ctx.r[3].s64 = ctx.r[11].s64 + -28240;
	// 8329DB5C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8329DB60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DB64: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 8329DB68: 4BAB80D9  bl 0x82d55c40
	ctx.lr = 0x8329DB6C;
	sub_82D55C40(ctx, base);
	// 8329DB6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DB70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DB74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DB80 size=108
    let mut pc: u32 = 0x8329DB80;
    'dispatch: loop {
        match pc {
            0x8329DB80 => {
    //   block [0x8329DB80..0x8329DBEC)
	// 8329DB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DB8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DB90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DB94: 38EBC158  addi r7, r11, -0x3ea8
	ctx.r[7].s64 = ctx.r[11].s64 + -16040;
	// 8329DB98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329DB9C: 388AC24C  addi r4, r10, -0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + -15796;
	// 8329DBA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DBA4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DBA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329DBAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DBB0: 386A91E0  addi r3, r10, -0x6e20
	ctx.r[3].s64 = ctx.r[10].s64 + -28192;
	// 8329DBB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329DBB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DBBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DBC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DBC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DBC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DBCC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329DBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DBD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329DBD8: 4BAB8069  bl 0x82d55c40
	ctx.lr = 0x8329DBDC;
	sub_82D55C40(ctx, base);
	// 8329DBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DBF0 size=112
    let mut pc: u32 = 0x8329DBF0;
    'dispatch: loop {
        match pc {
            0x8329DBF0 => {
    //   block [0x8329DBF0..0x8329DC60)
	// 8329DBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DBFC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DC00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DC04: 38AA8B50  addi r5, r10, -0x74b0
	ctx.r[5].s64 = ctx.r[10].s64 + -29872;
	// 8329DC08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DC0C: 390BC188  addi r8, r11, -0x3e78
	ctx.r[8].s64 = ctx.r[11].s64 + -15992;
	// 8329DC10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8329DC14: 388AC274  addi r4, r10, -0x3d8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15756;
	// 8329DC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DC1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329DC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DC28: 386A9210  addi r3, r10, -0x6df0
	ctx.r[3].s64 = ctx.r[10].s64 + -28144;
	// 8329DC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329DC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DC44: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329DC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DC4C: 4BAB7FF5  bl 0x82d55c40
	ctx.lr = 0x8329DC50;
	sub_82D55C40(ctx, base);
	// 8329DC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DC60 size=100
    let mut pc: u32 = 0x8329DC60;
    'dispatch: loop {
        match pc {
            0x8329DC60 => {
    //   block [0x8329DC60..0x8329DCC4)
	// 8329DC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DC6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DC74: 38AA89F0  addi r5, r10, -0x7610
	ctx.r[5].s64 = ctx.r[10].s64 + -30224;
	// 8329DC78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DC80: 388AC300  addi r4, r10, -0x3d00
	ctx.r[4].s64 = ctx.r[10].s64 + -15616;
	// 8329DC84: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DC94: 386A9240  addi r3, r10, -0x6dc0
	ctx.r[3].s64 = ctx.r[10].s64 + -28096;
	// 8329DC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DC9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DCA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329DCA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DCA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329DCAC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329DCB0: 4BAB7F91  bl 0x82d55c40
	ctx.lr = 0x8329DCB4;
	sub_82D55C40(ctx, base);
	// 8329DCB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DCC8 size=100
    let mut pc: u32 = 0x8329DCC8;
    'dispatch: loop {
        match pc {
            0x8329DCC8 => {
    //   block [0x8329DCC8..0x8329DD2C)
	// 8329DCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DCD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DCDC: 38AA8B50  addi r5, r10, -0x74b0
	ctx.r[5].s64 = ctx.r[10].s64 + -29872;
	// 8329DCE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DCE8: 388AC32C  addi r4, r10, -0x3cd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15572;
	// 8329DCEC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DCF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DCF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DCF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DCFC: 386A9270  addi r3, r10, -0x6d90
	ctx.r[3].s64 = ctx.r[10].s64 + -28048;
	// 8329DD00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DD04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DD08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329DD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DD10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329DD14: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329DD18: 4BAB7F29  bl 0x82d55c40
	ctx.lr = 0x8329DD1C;
	sub_82D55C40(ctx, base);
	// 8329DD1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DD30 size=112
    let mut pc: u32 = 0x8329DD30;
    'dispatch: loop {
        match pc {
            0x8329DD30 => {
    //   block [0x8329DD30..0x8329DDA0)
	// 8329DD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DD3C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DD40: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DD44: 38AA8960  addi r5, r10, -0x76a0
	ctx.r[5].s64 = ctx.r[10].s64 + -30368;
	// 8329DD48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DD4C: 390BC368  addi r8, r11, -0x3c98
	ctx.r[8].s64 = ctx.r[11].s64 + -15512;
	// 8329DD50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329DD54: 388AC3AC  addi r4, r10, -0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + -15444;
	// 8329DD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DD5C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DD60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329DD64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DD68: 386A92A0  addi r3, r10, -0x6d60
	ctx.r[3].s64 = ctx.r[10].s64 + -28000;
	// 8329DD6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329DD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DD84: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329DD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DD8C: 4BAB7EB5  bl 0x82d55c40
	ctx.lr = 0x8329DD90;
	sub_82D55C40(ctx, base);
	// 8329DD90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DD94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DD98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DD9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DDA0 size=100
    let mut pc: u32 = 0x8329DDA0;
    'dispatch: loop {
        match pc {
            0x8329DDA0 => {
    //   block [0x8329DDA0..0x8329DE04)
	// 8329DDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DDA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DDAC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DDB4: 38AA8D80  addi r5, r10, -0x7280
	ctx.r[5].s64 = ctx.r[10].s64 + -29312;
	// 8329DDB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DDC0: 388AC438  addi r4, r10, -0x3bc8
	ctx.r[4].s64 = ctx.r[10].s64 + -15304;
	// 8329DDC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DDC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DDCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DDD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DDD4: 386A92D0  addi r3, r10, -0x6d30
	ctx.r[3].s64 = ctx.r[10].s64 + -27952;
	// 8329DDD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DDDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DDE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329DDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DDE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329DDEC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329DDF0: 4BAB7E51  bl 0x82d55c40
	ctx.lr = 0x8329DDF4;
	sub_82D55C40(ctx, base);
	// 8329DDF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DDF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DDFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DE00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DE08 size=108
    let mut pc: u32 = 0x8329DE08;
    'dispatch: loop {
        match pc {
            0x8329DE08 => {
    //   block [0x8329DE08..0x8329DE74)
	// 8329DE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DE14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DE18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DE1C: 38EBC460  addi r7, r11, -0x3ba0
	ctx.r[7].s64 = ctx.r[11].s64 + -15264;
	// 8329DE20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329DE24: 388AC520  addi r4, r10, -0x3ae0
	ctx.r[4].s64 = ctx.r[10].s64 + -15072;
	// 8329DE28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DE2C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DE30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329DE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DE38: 386A9300  addi r3, r10, -0x6d00
	ctx.r[3].s64 = ctx.r[10].s64 + -27904;
	// 8329DE3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329DE40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DE54: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329DE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DE5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329DE60: 4BAB7DE1  bl 0x82d55c40
	ctx.lr = 0x8329DE64;
	sub_82D55C40(ctx, base);
	// 8329DE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DE78 size=92
    let mut pc: u32 = 0x8329DE78;
    'dispatch: loop {
        match pc {
            0x8329DE78 => {
    //   block [0x8329DE78..0x8329DED4)
	// 8329DE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DE80: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DE84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329DE88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329DE8C: 4BAED8CD  bl 0x82d8b758
	ctx.lr = 0x8329DE90;
	sub_82D8B758(ctx, base);
	// 8329DE90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DE94: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329DE98: 394BC548  addi r10, r11, -0x3ab8
	ctx.r[10].s64 = ctx.r[11].s64 + -15032;
	// 8329DE9C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329DEA0: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329DEA4: 396B9330  addi r11, r11, -0x6cd0
	ctx.r[11].s64 = ctx.r[11].s64 + -27856;
	// 8329DEA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329DEAC: 39482A40  addi r10, r8, 0x2a40
	ctx.r[10].s64 = ctx.r[8].s64 + 10816;
	// 8329DEB0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329DEB4: 39492A58  addi r10, r9, 0x2a58
	ctx.r[10].s64 = ctx.r[9].s64 + 10840;
	// 8329DEB8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329DEBC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329DEC0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329DEC4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8329DEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DED8 size=112
    let mut pc: u32 = 0x8329DED8;
    'dispatch: loop {
        match pc {
            0x8329DED8 => {
    //   block [0x8329DED8..0x8329DF48)
	// 8329DED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DEE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DEE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DEEC: 38AA8630  addi r5, r10, -0x79d0
	ctx.r[5].s64 = ctx.r[10].s64 + -31184;
	// 8329DEF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DEF4: 390BC4A8  addi r8, r11, -0x3b58
	ctx.r[8].s64 = ctx.r[11].s64 + -15192;
	// 8329DEF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8329DEFC: 388AC548  addi r4, r10, -0x3ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -15032;
	// 8329DF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DF04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329DF0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DF10: 386A9340  addi r3, r10, -0x6cc0
	ctx.r[3].s64 = ctx.r[10].s64 + -27840;
	// 8329DF14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329DF18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DF1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DF2C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329DF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DF34: 4BAB7D0D  bl 0x82d55c40
	ctx.lr = 0x8329DF38;
	sub_82D55C40(ctx, base);
	// 8329DF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DF48 size=100
    let mut pc: u32 = 0x8329DF48;
    'dispatch: loop {
        match pc {
            0x8329DF48 => {
    //   block [0x8329DF48..0x8329DFAC)
	// 8329DF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DF54: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DF5C: 38AA9240  addi r5, r10, -0x6dc0
	ctx.r[5].s64 = ctx.r[10].s64 + -28096;
	// 8329DF60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DF68: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 8329DF6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DF70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DF74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DF78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DF7C: 386A9370  addi r3, r10, -0x6c90
	ctx.r[3].s64 = ctx.r[10].s64 + -27792;
	// 8329DF80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329DF84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DF88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329DF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DF90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329DF94: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329DF98: 4BAB7CA9  bl 0x82d55c40
	ctx.lr = 0x8329DF9C;
	sub_82D55C40(ctx, base);
	// 8329DF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329DFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329DFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329DFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329DFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329DFB0 size=108
    let mut pc: u32 = 0x8329DFB0;
    'dispatch: loop {
        match pc {
            0x8329DFB0 => {
    //   block [0x8329DFB0..0x8329E01C)
	// 8329DFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329DFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329DFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329DFBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329DFC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329DFC4: 38EBC5E4  addi r7, r11, -0x3a1c
	ctx.r[7].s64 = ctx.r[11].s64 + -14876;
	// 8329DFC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8329DFCC: 388AC660  addi r4, r10, -0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + -14752;
	// 8329DFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329DFD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329DFD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329DFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329DFE0: 386A93A0  addi r3, r10, -0x6c60
	ctx.r[3].s64 = ctx.r[10].s64 + -27744;
	// 8329DFE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329DFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329DFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329DFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329DFF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329DFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329DFFC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329E000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329E008: 4BAB7C39  bl 0x82d55c40
	ctx.lr = 0x8329E00C;
	sub_82D55C40(ctx, base);
	// 8329E00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E020 size=112
    let mut pc: u32 = 0x8329E020;
    'dispatch: loop {
        match pc {
            0x8329E020 => {
    //   block [0x8329E020..0x8329E090)
	// 8329E020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E02C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E030: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E034: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329E038: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E03C: 390BC614  addi r8, r11, -0x39ec
	ctx.r[8].s64 = ctx.r[11].s64 + -14828;
	// 8329E040: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329E044: 388AC684  addi r4, r10, -0x397c
	ctx.r[4].s64 = ctx.r[10].s64 + -14716;
	// 8329E048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E04C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E050: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E058: 386A93D0  addi r3, r10, -0x6c30
	ctx.r[3].s64 = ctx.r[10].s64 + -27696;
	// 8329E05C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E06C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E074: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329E078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E07C: 4BAB7BC5  bl 0x82d55c40
	ctx.lr = 0x8329E080;
	sub_82D55C40(ctx, base);
	// 8329E080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E08C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E090 size=108
    let mut pc: u32 = 0x8329E090;
    'dispatch: loop {
        match pc {
            0x8329E090 => {
    //   block [0x8329E090..0x8329E0FC)
	// 8329E090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E09C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E0A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E0A4: 392BC768  addi r9, r11, -0x3898
	ctx.r[9].s64 = ctx.r[11].s64 + -14488;
	// 8329E0A8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8329E0AC: 39090018  addi r8, r9, 0x18
	ctx.r[8].s64 = ctx.r[9].s64 + 24;
	// 8329E0B0: 388AC8A4  addi r4, r10, -0x375c
	ctx.r[4].s64 = ctx.r[10].s64 + -14172;
	// 8329E0B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E0B8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E0BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329E0C0: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 8329E0C4: 386A9400  addi r3, r10, -0x6c00
	ctx.r[3].s64 = ctx.r[10].s64 + -27648;
	// 8329E0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E0CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329E0D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E0D4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E0D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E0DC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E0E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329E0E4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E0E8: 4BAB7B59  bl 0x82d55c40
	ctx.lr = 0x8329E0EC;
	sub_82D55C40(ctx, base);
	// 8329E0EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E100 size=112
    let mut pc: u32 = 0x8329E100;
    'dispatch: loop {
        match pc {
            0x8329E100 => {
    //   block [0x8329E100..0x8329E170)
	// 8329E100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E10C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E110: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E114: 38AA8730  addi r5, r10, -0x78d0
	ctx.r[5].s64 = ctx.r[10].s64 + -30928;
	// 8329E118: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E11C: 390BC828  addi r8, r11, -0x37d8
	ctx.r[8].s64 = ctx.r[11].s64 + -14296;
	// 8329E120: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329E124: 388AC8C0  addi r4, r10, -0x3740
	ctx.r[4].s64 = ctx.r[10].s64 + -14144;
	// 8329E128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E12C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E138: 386A9430  addi r3, r10, -0x6bd0
	ctx.r[3].s64 = ctx.r[10].s64 + -27600;
	// 8329E13C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E154: 38C00160  li r6, 0x160
	ctx.r[6].s64 = 352;
	// 8329E158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E15C: 4BAB7AE5  bl 0x82d55c40
	ctx.lr = 0x8329E160;
	sub_82D55C40(ctx, base);
	// 8329E160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E170 size=100
    let mut pc: u32 = 0x8329E170;
    'dispatch: loop {
        match pc {
            0x8329E170 => {
    //   block [0x8329E170..0x8329E1D4)
	// 8329E170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E17C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E184: 38AA8D80  addi r5, r10, -0x7280
	ctx.r[5].s64 = ctx.r[10].s64 + -29312;
	// 8329E188: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E18C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E190: 388AC940  addi r4, r10, -0x36c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14016;
	// 8329E194: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E19C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E1A4: 386A9460  addi r3, r10, -0x6ba0
	ctx.r[3].s64 = ctx.r[10].s64 + -27552;
	// 8329E1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E1AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E1B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329E1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E1B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329E1BC: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 8329E1C0: 4BAB7A81  bl 0x82d55c40
	ctx.lr = 0x8329E1C4;
	sub_82D55C40(ctx, base);
	// 8329E1C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E1C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E1CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E1D8 size=112
    let mut pc: u32 = 0x8329E1D8;
    'dispatch: loop {
        match pc {
            0x8329E1D8 => {
    //   block [0x8329E1D8..0x8329E248)
	// 8329E1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E1E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E1E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E1EC: 38AA8960  addi r5, r10, -0x76a0
	ctx.r[5].s64 = ctx.r[10].s64 + -30368;
	// 8329E1F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E1F4: 390BC990  addi r8, r11, -0x3670
	ctx.r[8].s64 = ctx.r[11].s64 + -13936;
	// 8329E1F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329E1FC: 388AC9EC  addi r4, r10, -0x3614
	ctx.r[4].s64 = ctx.r[10].s64 + -13844;
	// 8329E200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E204: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E208: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E210: 386A9490  addi r3, r10, -0x6b70
	ctx.r[3].s64 = ctx.r[10].s64 + -27504;
	// 8329E214: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E22C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329E230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E234: 4BAB7A0D  bl 0x82d55c40
	ctx.lr = 0x8329E238;
	sub_82D55C40(ctx, base);
	// 8329E238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E248 size=92
    let mut pc: u32 = 0x8329E248;
    'dispatch: loop {
        match pc {
            0x8329E248 => {
    //   block [0x8329E248..0x8329E2A4)
	// 8329E248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E250: 9421FC90  stwu r1, -0x370(r1)
	ea = ctx.r[1].u32.wrapping_add(-880 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E254: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329E258: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329E25C: 4BADD70D  bl 0x82d7b968
	ctx.lr = 0x8329E260;
	sub_82D7B968(ctx, base);
	// 8329E260: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E264: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329E268: 394B8960  addi r10, r11, -0x76a0
	ctx.r[10].s64 = ctx.r[11].s64 + -30368;
	// 8329E26C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329E270: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329E274: 396B94C0  addi r11, r11, -0x6b40
	ctx.r[11].s64 = ctx.r[11].s64 + -27456;
	// 8329E278: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329E27C: 39482C28  addi r10, r8, 0x2c28
	ctx.r[10].s64 = ctx.r[8].s64 + 11304;
	// 8329E280: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329E284: 39492C40  addi r10, r9, 0x2c40
	ctx.r[10].s64 = ctx.r[9].s64 + 11328;
	// 8329E288: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329E28C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329E290: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329E294: 38210370  addi r1, r1, 0x370
	ctx.r[1].s64 = ctx.r[1].s64 + 880;
	// 8329E298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E29C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E2A8 size=116
    let mut pc: u32 = 0x8329E2A8;
    'dispatch: loop {
        match pc {
            0x8329E2A8 => {
    //   block [0x8329E2A8..0x8329E31C)
	// 8329E2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E2B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E2B8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329E2BC: 392AD3CC  addi r9, r10, -0x2c34
	ctx.r[9].s64 = ctx.r[10].s64 + -11316;
	// 8329E2C0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329E2C4: 38C00043  li r6, 0x43
	ctx.r[6].s64 = 67;
	// 8329E2C8: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329E2CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E2D0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 8329E2D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E2D8: 388A8960  addi r4, r10, -0x76a0
	ctx.r[4].s64 = ctx.r[10].s64 + -30368;
	// 8329E2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E2E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8329E2E4: 396BD3F8  addi r11, r11, -0x2c08
	ctx.r[11].s64 = ctx.r[11].s64 + -11272;
	// 8329E2E8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E2F0: 386A94D0  addi r3, r10, -0x6b30
	ctx.r[3].s64 = ctx.r[10].s64 + -27440;
	// 8329E2F4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329E2F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8329E2FC: 38C00310  li r6, 0x310
	ctx.r[6].s64 = 784;
	// 8329E300: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329E304: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329E308: 4BAB7939  bl 0x82d55c40
	ctx.lr = 0x8329E30C;
	sub_82D55C40(ctx, base);
	// 8329E30C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E320 size=92
    let mut pc: u32 = 0x8329E320;
    'dispatch: loop {
        match pc {
            0x8329E320 => {
    //   block [0x8329E320..0x8329E37C)
	// 8329E320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E328: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E32C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329E330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329E334: 4BAE38FD  bl 0x82d81c30
	ctx.lr = 0x8329E338;
	sub_82D81C30(ctx, base);
	// 8329E338: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E33C: 3D0082D8  lis r8, -0x7d28
	ctx.r[8].s64 = -2099773440;
	// 8329E340: 394BDF18  addi r10, r11, -0x20e8
	ctx.r[10].s64 = ctx.r[11].s64 + -8424;
	// 8329E344: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329E348: 3D2082D8  lis r9, -0x7d28
	ctx.r[9].s64 = -2099773440;
	// 8329E34C: 396B9500  addi r11, r11, -0x6b00
	ctx.r[11].s64 = ctx.r[11].s64 + -27392;
	// 8329E350: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329E354: 39483BB8  addi r10, r8, 0x3bb8
	ctx.r[10].s64 = ctx.r[8].s64 + 15288;
	// 8329E358: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329E35C: 39493BD0  addi r10, r9, 0x3bd0
	ctx.r[10].s64 = ctx.r[9].s64 + 15312;
	// 8329E360: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329E364: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329E368: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329E36C: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 8329E370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329E380 size=48
    let mut pc: u32 = 0x8329E380;
    'dispatch: loop {
        match pc {
            0x8329E380 => {
    //   block [0x8329E380..0x8329E3B0)
	// 8329E380: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329E384: 814B96AC  lwz r10, -0x6954(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26964 as u32) ) } as u64;
	// 8329E388: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329E38C: 396B96B0  addi r11, r11, -0x6950
	ctx.r[11].s64 = ctx.r[11].s64 + -26960;
	// 8329E390: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8329E394: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329E398: 814A96A8  lwz r10, -0x6958(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26968 as u32) ) } as u64;
	// 8329E39C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 8329E3A0: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329E3A4: 814A96A4  lwz r10, -0x695c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-26972 as u32) ) } as u64;
	// 8329E3A8: 914B0338  stw r10, 0x338(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(824 as u32), ctx.r[10].u32 ) };
	// 8329E3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E3B0 size=116
    let mut pc: u32 = 0x8329E3B0;
    'dispatch: loop {
        match pc {
            0x8329E3B0 => {
    //   block [0x8329E3B0..0x8329E424)
	// 8329E3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E3BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E3C0: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329E3C4: 392BDDC8  addi r9, r11, -0x2238
	ctx.r[9].s64 = ctx.r[11].s64 + -8760;
	// 8329E3C8: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329E3CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E3D0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8329E3D4: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 8329E3D8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329E3DC: 388ADF18  addi r4, r10, -0x20e8
	ctx.r[4].s64 = ctx.r[10].s64 + -8424;
	// 8329E3E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E3E4: 396B96B0  addi r11, r11, -0x6950
	ctx.r[11].s64 = ctx.r[11].s64 + -26960;
	// 8329E3E8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8329E3EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E3F0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8329E3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E3F8: 386A9510  addi r3, r10, -0x6af0
	ctx.r[3].s64 = ctx.r[10].s64 + -27376;
	// 8329E3FC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8329E400: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8329E404: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 8329E408: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8329E40C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329E410: 4BAB7831  bl 0x82d55c40
	ctx.lr = 0x8329E414;
	sub_82D55C40(ctx, base);
	// 8329E414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E41C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E428 size=112
    let mut pc: u32 = 0x8329E428;
    'dispatch: loop {
        match pc {
            0x8329E428 => {
    //   block [0x8329E428..0x8329E498)
	// 8329E428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E434: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329E438: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E43C: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329E440: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E444: 390BE0E4  addi r8, r11, -0x1f1c
	ctx.r[8].s64 = ctx.r[11].s64 + -7964;
	// 8329E448: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329E44C: 388AE0FC  addi r4, r10, -0x1f04
	ctx.r[4].s64 = ctx.r[10].s64 + -7940;
	// 8329E450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E454: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E458: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E45C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E460: 386A9540  addi r3, r10, -0x6ac0
	ctx.r[3].s64 = ctx.r[10].s64 + -27328;
	// 8329E464: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E46C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E47C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329E480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E484: 4BAB77BD  bl 0x82d55c40
	ctx.lr = 0x8329E488;
	sub_82D55C40(ctx, base);
	// 8329E488: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329E498 size=24
    let mut pc: u32 = 0x8329E498;
    'dispatch: loop {
        match pc {
            0x8329E498 => {
    //   block [0x8329E498..0x8329E4B0)
	// 8329E498: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329E49C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329E4A0: 394AA10C  addi r10, r10, -0x5ef4
	ctx.r[10].s64 = ctx.r[10].s64 + -24308;
	// 8329E4A4: 816B9F30  lwz r11, -0x60d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24784 as u32) ) } as u64;
	// 8329E4A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8329E4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E4B0 size=112
    let mut pc: u32 = 0x8329E4B0;
    'dispatch: loop {
        match pc {
            0x8329E4B0 => {
    //   block [0x8329E4B0..0x8329E520)
	// 8329E4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E4BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E4C0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329E4C4: 392AEFD0  addi r9, r10, -0x1030
	ctx.r[9].s64 = ctx.r[10].s64 + -4144;
	// 8329E4C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E4CC: 390BA10C  addi r8, r11, -0x5ef4
	ctx.r[8].s64 = ctx.r[11].s64 + -24308;
	// 8329E4D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8329E4D4: 388AF7E0  addi r4, r10, -0x820
	ctx.r[4].s64 = ctx.r[10].s64 + -2080;
	// 8329E4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E4DC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E4E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E4E4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8329E4E8: 386A9570  addi r3, r10, -0x6a90
	ctx.r[3].s64 = ctx.r[10].s64 + -27280;
	// 8329E4EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329E4F0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8329E4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329E508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E50C: 4BAB7735  bl 0x82d55c40
	ctx.lr = 0x8329E510;
	sub_82D55C40(ctx, base);
	// 8329E510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E520 size=112
    let mut pc: u32 = 0x8329E520;
    'dispatch: loop {
        match pc {
            0x8329E520 => {
    //   block [0x8329E520..0x8329E590)
	// 8329E520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E52C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E530: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E534: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E53C: 390BEFF8  addi r8, r11, -0x1008
	ctx.r[8].s64 = ctx.r[11].s64 + -4104;
	// 8329E540: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329E544: 388AF7F4  addi r4, r10, -0x80c
	ctx.r[4].s64 = ctx.r[10].s64 + -2060;
	// 8329E548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E54C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E558: 386A95A0  addi r3, r10, -0x6a60
	ctx.r[3].s64 = ctx.r[10].s64 + -27232;
	// 8329E55C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E574: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329E578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E57C: 4BAB76C5  bl 0x82d55c40
	ctx.lr = 0x8329E580;
	sub_82D55C40(ctx, base);
	// 8329E580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E58C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E590 size=108
    let mut pc: u32 = 0x8329E590;
    'dispatch: loop {
        match pc {
            0x8329E590 => {
    //   block [0x8329E590..0x8329E5FC)
	// 8329E590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E59C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E5A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E5A4: 38EBF028  addi r7, r11, -0xfd8
	ctx.r[7].s64 = ctx.r[11].s64 + -4056;
	// 8329E5A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329E5AC: 388AF80C  addi r4, r10, -0x7f4
	ctx.r[4].s64 = ctx.r[10].s64 + -2036;
	// 8329E5B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E5B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E5B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329E5BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E5C0: 386A95D0  addi r3, r10, -0x6a30
	ctx.r[3].s64 = ctx.r[10].s64 + -27184;
	// 8329E5C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329E5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E5CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E5DC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329E5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E5E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329E5E8: 4BAB7659  bl 0x82d55c40
	ctx.lr = 0x8329E5EC;
	sub_82D55C40(ctx, base);
	// 8329E5EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E5F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E5F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E5F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E600 size=112
    let mut pc: u32 = 0x8329E600;
    'dispatch: loop {
        match pc {
            0x8329E600 => {
    //   block [0x8329E600..0x8329E670)
	// 8329E600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E60C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E610: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E614: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E618: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E61C: 390BF040  addi r8, r11, -0xfc0
	ctx.r[8].s64 = ctx.r[11].s64 + -4032;
	// 8329E620: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8329E624: 388AF81C  addi r4, r10, -0x7e4
	ctx.r[4].s64 = ctx.r[10].s64 + -2020;
	// 8329E628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E62C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E638: 386A9600  addi r3, r10, -0x6a00
	ctx.r[3].s64 = ctx.r[10].s64 + -27136;
	// 8329E63C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E654: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329E658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E65C: 4BAB75E5  bl 0x82d55c40
	ctx.lr = 0x8329E660;
	sub_82D55C40(ctx, base);
	// 8329E660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E66C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E670 size=100
    let mut pc: u32 = 0x8329E670;
    'dispatch: loop {
        match pc {
            0x8329E670 => {
    //   block [0x8329E670..0x8329E6D4)
	// 8329E670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E67C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E684: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E688: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E690: 388AF83C  addi r4, r10, -0x7c4
	ctx.r[4].s64 = ctx.r[10].s64 + -1988;
	// 8329E694: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E6A4: 386A9630  addi r3, r10, -0x69d0
	ctx.r[3].s64 = ctx.r[10].s64 + -27088;
	// 8329E6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E6AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E6B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329E6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E6B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329E6BC: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8329E6C0: 4BAB7581  bl 0x82d55c40
	ctx.lr = 0x8329E6C4;
	sub_82D55C40(ctx, base);
	// 8329E6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E6D8 size=112
    let mut pc: u32 = 0x8329E6D8;
    'dispatch: loop {
        match pc {
            0x8329E6D8 => {
    //   block [0x8329E6D8..0x8329E748)
	// 8329E6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E6E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E6E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E6EC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E6F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E6F4: 390BF100  addi r8, r11, -0xf00
	ctx.r[8].s64 = ctx.r[11].s64 + -3840;
	// 8329E6F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329E6FC: 388AF858  addi r4, r10, -0x7a8
	ctx.r[4].s64 = ctx.r[10].s64 + -1960;
	// 8329E700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E704: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E710: 386A9660  addi r3, r10, -0x69a0
	ctx.r[3].s64 = ctx.r[10].s64 + -27040;
	// 8329E714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E71C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E72C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329E730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E734: 4BAB750D  bl 0x82d55c40
	ctx.lr = 0x8329E738;
	sub_82D55C40(ctx, base);
	// 8329E738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E748 size=112
    let mut pc: u32 = 0x8329E748;
    'dispatch: loop {
        match pc {
            0x8329E748 => {
    //   block [0x8329E748..0x8329E7B8)
	// 8329E748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E754: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E758: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E75C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E760: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E764: 390BF118  addi r8, r11, -0xee8
	ctx.r[8].s64 = ctx.r[11].s64 + -3816;
	// 8329E768: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329E76C: 388AF878  addi r4, r10, -0x788
	ctx.r[4].s64 = ctx.r[10].s64 + -1928;
	// 8329E770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E774: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E780: 386A9690  addi r3, r10, -0x6970
	ctx.r[3].s64 = ctx.r[10].s64 + -26992;
	// 8329E784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E79C: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 8329E7A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E7A4: 4BAB749D  bl 0x82d55c40
	ctx.lr = 0x8329E7A8;
	sub_82D55C40(ctx, base);
	// 8329E7A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E7AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E7B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E7B8 size=112
    let mut pc: u32 = 0x8329E7B8;
    'dispatch: loop {
        match pc {
            0x8329E7B8 => {
    //   block [0x8329E7B8..0x8329E828)
	// 8329E7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E7C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E7C8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E7CC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E7D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E7D4: 390BF148  addi r8, r11, -0xeb8
	ctx.r[8].s64 = ctx.r[11].s64 + -3768;
	// 8329E7D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329E7DC: 388AF89C  addi r4, r10, -0x764
	ctx.r[4].s64 = ctx.r[10].s64 + -1892;
	// 8329E7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E7E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E7E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E7F0: 386A96C0  addi r3, r10, -0x6940
	ctx.r[3].s64 = ctx.r[10].s64 + -26944;
	// 8329E7F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E80C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329E810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E814: 4BAB742D  bl 0x82d55c40
	ctx.lr = 0x8329E818;
	sub_82D55C40(ctx, base);
	// 8329E818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E828 size=112
    let mut pc: u32 = 0x8329E828;
    'dispatch: loop {
        match pc {
            0x8329E828 => {
    //   block [0x8329E828..0x8329E898)
	// 8329E828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E834: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E838: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E83C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E840: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E844: 390BF178  addi r8, r11, -0xe88
	ctx.r[8].s64 = ctx.r[11].s64 + -3720;
	// 8329E848: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329E84C: 388AF8C4  addi r4, r10, -0x73c
	ctx.r[4].s64 = ctx.r[10].s64 + -1852;
	// 8329E850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E854: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E860: 386A96F0  addi r3, r10, -0x6910
	ctx.r[3].s64 = ctx.r[10].s64 + -26896;
	// 8329E864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E87C: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329E880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E884: 4BAB73BD  bl 0x82d55c40
	ctx.lr = 0x8329E888;
	sub_82D55C40(ctx, base);
	// 8329E888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E898 size=112
    let mut pc: u32 = 0x8329E898;
    'dispatch: loop {
        match pc {
            0x8329E898 => {
    //   block [0x8329E898..0x8329E908)
	// 8329E898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E8A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E8A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E8AC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E8B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E8B4: 390BF1A8  addi r8, r11, -0xe58
	ctx.r[8].s64 = ctx.r[11].s64 + -3672;
	// 8329E8B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329E8BC: 388AF8E8  addi r4, r10, -0x718
	ctx.r[4].s64 = ctx.r[10].s64 + -1816;
	// 8329E8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E8C4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E8D0: 386A9720  addi r3, r10, -0x68e0
	ctx.r[3].s64 = ctx.r[10].s64 + -26848;
	// 8329E8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E8EC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329E8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E8F4: 4BAB734D  bl 0x82d55c40
	ctx.lr = 0x8329E8F8;
	sub_82D55C40(ctx, base);
	// 8329E8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E908 size=112
    let mut pc: u32 = 0x8329E908;
    'dispatch: loop {
        match pc {
            0x8329E908 => {
    //   block [0x8329E908..0x8329E978)
	// 8329E908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E914: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E918: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E91C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E920: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E924: 390BF1C0  addi r8, r11, -0xe40
	ctx.r[8].s64 = ctx.r[11].s64 + -3648;
	// 8329E928: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329E92C: 388AF908  addi r4, r10, -0x6f8
	ctx.r[4].s64 = ctx.r[10].s64 + -1784;
	// 8329E930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E934: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E940: 386A9750  addi r3, r10, -0x68b0
	ctx.r[3].s64 = ctx.r[10].s64 + -26800;
	// 8329E944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E94C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E95C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329E960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E964: 4BAB72DD  bl 0x82d55c40
	ctx.lr = 0x8329E968;
	sub_82D55C40(ctx, base);
	// 8329E968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E96C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E978 size=112
    let mut pc: u32 = 0x8329E978;
    'dispatch: loop {
        match pc {
            0x8329E978 => {
    //   block [0x8329E978..0x8329E9E8)
	// 8329E978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E984: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E988: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E98C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329E990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329E994: 390BF1D8  addi r8, r11, -0xe28
	ctx.r[8].s64 = ctx.r[11].s64 + -3624;
	// 8329E998: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329E99C: 388AF920  addi r4, r10, -0x6e0
	ctx.r[4].s64 = ctx.r[10].s64 + -1760;
	// 8329E9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329E9A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E9A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329E9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329E9B0: 386A9780  addi r3, r10, -0x6880
	ctx.r[3].s64 = ctx.r[10].s64 + -26752;
	// 8329E9B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329E9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329E9BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329E9C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329E9C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329E9C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329E9CC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329E9D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329E9D4: 4BAB726D  bl 0x82d55c40
	ctx.lr = 0x8329E9D8;
	sub_82D55C40(ctx, base);
	// 8329E9D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329E9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329E9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329E9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329E9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329E9E8 size=112
    let mut pc: u32 = 0x8329E9E8;
    'dispatch: loop {
        match pc {
            0x8329E9E8 => {
    //   block [0x8329E9E8..0x8329EA58)
	// 8329E9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329E9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329E9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329E9F4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329E9F8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329E9FC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EA00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EA04: 390BF220  addi r8, r11, -0xde0
	ctx.r[8].s64 = ctx.r[11].s64 + -3552;
	// 8329EA08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329EA0C: 388AF93C  addi r4, r10, -0x6c4
	ctx.r[4].s64 = ctx.r[10].s64 + -1732;
	// 8329EA10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EA14: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EA18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EA1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EA20: 386A97B0  addi r3, r10, -0x6850
	ctx.r[3].s64 = ctx.r[10].s64 + -26704;
	// 8329EA24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EA28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EA30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EA34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EA38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EA3C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329EA40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EA44: 4BAB71FD  bl 0x82d55c40
	ctx.lr = 0x8329EA48;
	sub_82D55C40(ctx, base);
	// 8329EA48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EA4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EA50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EA54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EA58 size=112
    let mut pc: u32 = 0x8329EA58;
    'dispatch: loop {
        match pc {
            0x8329EA58 => {
    //   block [0x8329EA58..0x8329EAC8)
	// 8329EA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EA5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EA60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EA64: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EA68: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EA6C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EA70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EA74: 390BF268  addi r8, r11, -0xd98
	ctx.r[8].s64 = ctx.r[11].s64 + -3480;
	// 8329EA78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329EA7C: 388AF958  addi r4, r10, -0x6a8
	ctx.r[4].s64 = ctx.r[10].s64 + -1704;
	// 8329EA80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EA84: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EA88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EA90: 386A97E0  addi r3, r10, -0x6820
	ctx.r[3].s64 = ctx.r[10].s64 + -26656;
	// 8329EA94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EA98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EAA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EAA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EAA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EAAC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329EAB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EAB4: 4BAB718D  bl 0x82d55c40
	ctx.lr = 0x8329EAB8;
	sub_82D55C40(ctx, base);
	// 8329EAB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EAC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EAC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EAC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EAC8 size=112
    let mut pc: u32 = 0x8329EAC8;
    'dispatch: loop {
        match pc {
            0x8329EAC8 => {
    //   block [0x8329EAC8..0x8329EB38)
	// 8329EAC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EAD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EAD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EAD8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EADC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EAE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EAE4: 390BF280  addi r8, r11, -0xd80
	ctx.r[8].s64 = ctx.r[11].s64 + -3456;
	// 8329EAE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329EAEC: 388AF970  addi r4, r10, -0x690
	ctx.r[4].s64 = ctx.r[10].s64 + -1680;
	// 8329EAF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EAF4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EAF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EAFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EB00: 386A9810  addi r3, r10, -0x67f0
	ctx.r[3].s64 = ctx.r[10].s64 + -26608;
	// 8329EB04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EB08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EB10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EB18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EB1C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329EB20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EB24: 4BAB711D  bl 0x82d55c40
	ctx.lr = 0x8329EB28;
	sub_82D55C40(ctx, base);
	// 8329EB28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EB2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EB30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EB38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EB38 size=112
    let mut pc: u32 = 0x8329EB38;
    'dispatch: loop {
        match pc {
            0x8329EB38 => {
    //   block [0x8329EB38..0x8329EBA8)
	// 8329EB38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EB3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EB40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EB44: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EB48: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EB4C: 396BF2B0  addi r11, r11, -0xd50
	ctx.r[11].s64 = ctx.r[11].s64 + -3408;
	// 8329EB50: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EB54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EB58: 390B0078  addi r8, r11, 0x78
	ctx.r[8].s64 = ctx.r[11].s64 + 120;
	// 8329EB5C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329EB60: 388AF988  addi r4, r10, -0x678
	ctx.r[4].s64 = ctx.r[10].s64 + -1656;
	// 8329EB64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329EB68: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EB6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EB70: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329EB74: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329EB78: 386A9840  addi r3, r10, -0x67c0
	ctx.r[3].s64 = ctx.r[10].s64 + -26560;
	// 8329EB7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329EB80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EB88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329EB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EB90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329EB94: 4BAB70AD  bl 0x82d55c40
	ctx.lr = 0x8329EB98;
	sub_82D55C40(ctx, base);
	// 8329EB98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EB9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EBA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EBA8 size=112
    let mut pc: u32 = 0x8329EBA8;
    'dispatch: loop {
        match pc {
            0x8329EBA8 => {
    //   block [0x8329EBA8..0x8329EC18)
	// 8329EBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EBB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EBB4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EBB8: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EBBC: 396BF340  addi r11, r11, -0xcc0
	ctx.r[11].s64 = ctx.r[11].s64 + -3264;
	// 8329EBC0: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EBC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EBC8: 390B0090  addi r8, r11, 0x90
	ctx.r[8].s64 = ctx.r[11].s64 + 144;
	// 8329EBCC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8329EBD0: 388AF9A4  addi r4, r10, -0x65c
	ctx.r[4].s64 = ctx.r[10].s64 + -1628;
	// 8329EBD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329EBD8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EBDC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EBE0: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8329EBE4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329EBE8: 386A9870  addi r3, r10, -0x6790
	ctx.r[3].s64 = ctx.r[10].s64 + -26512;
	// 8329EBEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329EBF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EBF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EBF8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329EBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EC00: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329EC04: 4BAB703D  bl 0x82d55c40
	ctx.lr = 0x8329EC08;
	sub_82D55C40(ctx, base);
	// 8329EC08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EC0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EC10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329EC18 size=24
    let mut pc: u32 = 0x8329EC18;
    'dispatch: loop {
        match pc {
            0x8329EC18 => {
    //   block [0x8329EC18..0x8329EC30)
	// 8329EC18: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329EC1C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329EC20: 394AA128  addi r10, r10, -0x5ed8
	ctx.r[10].s64 = ctx.r[10].s64 + -24280;
	// 8329EC24: 816B9F38  lwz r11, -0x60c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24776 as u32) ) } as u64;
	// 8329EC28: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8329EC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EC30 size=116
    let mut pc: u32 = 0x8329EC30;
    'dispatch: loop {
        match pc {
            0x8329EC30 => {
    //   block [0x8329EC30..0x8329ECA4)
	// 8329EC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EC3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EC40: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EC44: 392BF3FC  addi r9, r11, -0xc04
	ctx.r[9].s64 = ctx.r[11].s64 + -3076;
	// 8329EC48: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EC4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EC50: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8329EC54: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329EC58: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329EC5C: 388AF9C0  addi r4, r10, -0x640
	ctx.r[4].s64 = ctx.r[10].s64 + -1600;
	// 8329EC60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EC64: 396BA128  addi r11, r11, -0x5ed8
	ctx.r[11].s64 = ctx.r[11].s64 + -24280;
	// 8329EC68: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8329EC6C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EC70: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8329EC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EC78: 386A98A0  addi r3, r10, -0x6760
	ctx.r[3].s64 = ctx.r[10].s64 + -26464;
	// 8329EC7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329EC80: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8329EC84: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329EC88: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8329EC8C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329EC90: 4BAB6FB1  bl 0x82d55c40
	ctx.lr = 0x8329EC94;
	sub_82D55C40(ctx, base);
	// 8329EC94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EC98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EC9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ECA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ECA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ECA8 size=112
    let mut pc: u32 = 0x8329ECA8;
    'dispatch: loop {
        match pc {
            0x8329ECA8 => {
    //   block [0x8329ECA8..0x8329ED18)
	// 8329ECA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ECAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ECB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ECB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329ECB8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329ECBC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329ECC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329ECC4: 390BF438  addi r8, r11, -0xbc8
	ctx.r[8].s64 = ctx.r[11].s64 + -3016;
	// 8329ECC8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329ECCC: 388AF9F4  addi r4, r10, -0x60c
	ctx.r[4].s64 = ctx.r[10].s64 + -1548;
	// 8329ECD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329ECD4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329ECD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329ECDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329ECE0: 386A98D0  addi r3, r10, -0x6730
	ctx.r[3].s64 = ctx.r[10].s64 + -26416;
	// 8329ECE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329ECE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329ECEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329ECF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329ECF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329ECF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329ECFC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329ED00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329ED04: 4BAB6F3D  bl 0x82d55c40
	ctx.lr = 0x8329ED08;
	sub_82D55C40(ctx, base);
	// 8329ED08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329ED0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329ED10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ED14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ED18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ED18 size=112
    let mut pc: u32 = 0x8329ED18;
    'dispatch: loop {
        match pc {
            0x8329ED18 => {
    //   block [0x8329ED18..0x8329ED88)
	// 8329ED18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ED1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ED20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ED24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329ED28: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329ED2C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329ED30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329ED34: 390BF498  addi r8, r11, -0xb68
	ctx.r[8].s64 = ctx.r[11].s64 + -2920;
	// 8329ED38: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8329ED3C: 388AFA14  addi r4, r10, -0x5ec
	ctx.r[4].s64 = ctx.r[10].s64 + -1516;
	// 8329ED40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329ED44: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329ED48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329ED4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329ED50: 386A9900  addi r3, r10, -0x6700
	ctx.r[3].s64 = ctx.r[10].s64 + -26368;
	// 8329ED54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329ED58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329ED5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329ED60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329ED64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329ED68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329ED6C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8329ED70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329ED74: 4BAB6ECD  bl 0x82d55c40
	ctx.lr = 0x8329ED78;
	sub_82D55C40(ctx, base);
	// 8329ED78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329ED7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329ED80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329ED84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329ED88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329ED88 size=112
    let mut pc: u32 = 0x8329ED88;
    'dispatch: loop {
        match pc {
            0x8329ED88 => {
    //   block [0x8329ED88..0x8329EDF8)
	// 8329ED88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329ED8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329ED90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329ED94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329ED98: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329ED9C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EDA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EDA4: 390BF540  addi r8, r11, -0xac0
	ctx.r[8].s64 = ctx.r[11].s64 + -2752;
	// 8329EDA8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8329EDAC: 388AFA30  addi r4, r10, -0x5d0
	ctx.r[4].s64 = ctx.r[10].s64 + -1488;
	// 8329EDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EDB4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EDB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EDC0: 386A9930  addi r3, r10, -0x66d0
	ctx.r[3].s64 = ctx.r[10].s64 + -26320;
	// 8329EDC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EDDC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329EDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EDE4: 4BAB6E5D  bl 0x82d55c40
	ctx.lr = 0x8329EDE8;
	sub_82D55C40(ctx, base);
	// 8329EDE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EDF8 size=112
    let mut pc: u32 = 0x8329EDF8;
    'dispatch: loop {
        match pc {
            0x8329EDF8 => {
    //   block [0x8329EDF8..0x8329EE68)
	// 8329EDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EE04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EE08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EE0C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EE14: 390BF5B8  addi r8, r11, -0xa48
	ctx.r[8].s64 = ctx.r[11].s64 + -2632;
	// 8329EE18: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329EE1C: 388AFA50  addi r4, r10, -0x5b0
	ctx.r[4].s64 = ctx.r[10].s64 + -1456;
	// 8329EE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EE24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EE28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EE2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EE30: 386A9960  addi r3, r10, -0x66a0
	ctx.r[3].s64 = ctx.r[10].s64 + -26272;
	// 8329EE34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EE4C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329EE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EE54: 4BAB6DED  bl 0x82d55c40
	ctx.lr = 0x8329EE58;
	sub_82D55C40(ctx, base);
	// 8329EE58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EE5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EE60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EE68 size=112
    let mut pc: u32 = 0x8329EE68;
    'dispatch: loop {
        match pc {
            0x8329EE68 => {
    //   block [0x8329EE68..0x8329EED8)
	// 8329EE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EE74: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EE78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EE7C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EE80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EE84: 390BF600  addi r8, r11, -0xa00
	ctx.r[8].s64 = ctx.r[11].s64 + -2560;
	// 8329EE88: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8329EE8C: 388AFA70  addi r4, r10, -0x590
	ctx.r[4].s64 = ctx.r[10].s64 + -1424;
	// 8329EE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EE94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EE98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EEA0: 386A9990  addi r3, r10, -0x6670
	ctx.r[3].s64 = ctx.r[10].s64 + -26224;
	// 8329EEA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EEBC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329EEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EEC4: 4BAB6D7D  bl 0x82d55c40
	ctx.lr = 0x8329EEC8;
	sub_82D55C40(ctx, base);
	// 8329EEC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EED8 size=112
    let mut pc: u32 = 0x8329EED8;
    'dispatch: loop {
        match pc {
            0x8329EED8 => {
    //   block [0x8329EED8..0x8329EF48)
	// 8329EED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EEE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EEE8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EEEC: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EEF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EEF4: 390BF690  addi r8, r11, -0x970
	ctx.r[8].s64 = ctx.r[11].s64 + -2416;
	// 8329EEF8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329EEFC: 388AFA8C  addi r4, r10, -0x574
	ctx.r[4].s64 = ctx.r[10].s64 + -1396;
	// 8329EF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EF04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EF0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EF10: 386A99C0  addi r3, r10, -0x6640
	ctx.r[3].s64 = ctx.r[10].s64 + -26176;
	// 8329EF14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EF18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EF1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EF24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EF2C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329EF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EF34: 4BAB6D0D  bl 0x82d55c40
	ctx.lr = 0x8329EF38;
	sub_82D55C40(ctx, base);
	// 8329EF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EF48 size=112
    let mut pc: u32 = 0x8329EF48;
    'dispatch: loop {
        match pc {
            0x8329EF48 => {
    //   block [0x8329EF48..0x8329EFB8)
	// 8329EF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EF54: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EF58: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EF5C: 38AA9570  addi r5, r10, -0x6a90
	ctx.r[5].s64 = ctx.r[10].s64 + -27280;
	// 8329EF60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EF64: 390BF6F0  addi r8, r11, -0x910
	ctx.r[8].s64 = ctx.r[11].s64 + -2320;
	// 8329EF68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329EF6C: 388AFAA4  addi r4, r10, -0x55c
	ctx.r[4].s64 = ctx.r[10].s64 + -1372;
	// 8329EF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EF74: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EF78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EF80: 386A99F0  addi r3, r10, -0x6610
	ctx.r[3].s64 = ctx.r[10].s64 + -26128;
	// 8329EF84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329EF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329EF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329EF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329EF9C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329EFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329EFA4: 4BAB6C9D  bl 0x82d55c40
	ctx.lr = 0x8329EFA8;
	sub_82D55C40(ctx, base);
	// 8329EFA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329EFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329EFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329EFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329EFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329EFB8 size=112
    let mut pc: u32 = 0x8329EFB8;
    'dispatch: loop {
        match pc {
            0x8329EFB8 => {
    //   block [0x8329EFB8..0x8329F028)
	// 8329EFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329EFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329EFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329EFC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EFC8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329EFCC: 38AA99F0  addi r5, r10, -0x6610
	ctx.r[5].s64 = ctx.r[10].s64 + -26128;
	// 8329EFD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329EFD4: 390BF750  addi r8, r11, -0x8b0
	ctx.r[8].s64 = ctx.r[11].s64 + -2224;
	// 8329EFD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329EFDC: 388AFAC0  addi r4, r10, -0x540
	ctx.r[4].s64 = ctx.r[10].s64 + -1344;
	// 8329EFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329EFE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329EFE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329EFEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329EFF0: 386A9A20  addi r3, r10, -0x65e0
	ctx.r[3].s64 = ctx.r[10].s64 + -26080;
	// 8329EFF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329EFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329EFFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F00C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329F010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F014: 4BAB6C2D  bl 0x82d55c40
	ctx.lr = 0x8329F018;
	sub_82D55C40(ctx, base);
	// 8329F018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F028 size=112
    let mut pc: u32 = 0x8329F028;
    'dispatch: loop {
        match pc {
            0x8329F028 => {
    //   block [0x8329F028..0x8329F098)
	// 8329F028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F034: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F038: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F03C: 38AA99F0  addi r5, r10, -0x6610
	ctx.r[5].s64 = ctx.r[10].s64 + -26128;
	// 8329F040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F044: 390BF780  addi r8, r11, -0x880
	ctx.r[8].s64 = ctx.r[11].s64 + -2176;
	// 8329F048: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329F04C: 388AFAE8  addi r4, r10, -0x518
	ctx.r[4].s64 = ctx.r[10].s64 + -1304;
	// 8329F050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F054: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F060: 386A9A50  addi r3, r10, -0x65b0
	ctx.r[3].s64 = ctx.r[10].s64 + -26032;
	// 8329F064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F07C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329F080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F084: 4BAB6BBD  bl 0x82d55c40
	ctx.lr = 0x8329F088;
	sub_82D55C40(ctx, base);
	// 8329F088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F098 size=100
    let mut pc: u32 = 0x8329F098;
    'dispatch: loop {
        match pc {
            0x8329F098 => {
    //   block [0x8329F098..0x8329F0FC)
	// 8329F098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F0A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F0AC: 38AA99F0  addi r5, r10, -0x6610
	ctx.r[5].s64 = ctx.r[10].s64 + -26128;
	// 8329F0B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F0B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F0B8: 388AFB10  addi r4, r10, -0x4f0
	ctx.r[4].s64 = ctx.r[10].s64 + -1264;
	// 8329F0BC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F0C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F0C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F0C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F0CC: 386A9A80  addi r3, r10, -0x6580
	ctx.r[3].s64 = ctx.r[10].s64 + -25984;
	// 8329F0D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F0D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F0D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329F0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F0E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329F0E4: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329F0E8: 4BAB6B59  bl 0x82d55c40
	ctx.lr = 0x8329F0EC;
	sub_82D55C40(ctx, base);
	// 8329F0EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F100 size=112
    let mut pc: u32 = 0x8329F100;
    'dispatch: loop {
        match pc {
            0x8329F100 => {
    //   block [0x8329F100..0x8329F170)
	// 8329F100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F10C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F110: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F114: 38AA99F0  addi r5, r10, -0x6610
	ctx.r[5].s64 = ctx.r[10].s64 + -26128;
	// 8329F118: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F11C: 390BF7C8  addi r8, r11, -0x838
	ctx.r[8].s64 = ctx.r[11].s64 + -2104;
	// 8329F120: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329F124: 388AFB38  addi r4, r10, -0x4c8
	ctx.r[4].s64 = ctx.r[10].s64 + -1224;
	// 8329F128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F12C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F138: 386A9AB0  addi r3, r10, -0x6550
	ctx.r[3].s64 = ctx.r[10].s64 + -25936;
	// 8329F13C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F154: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329F158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F15C: 4BAB6AE5  bl 0x82d55c40
	ctx.lr = 0x8329F160;
	sub_82D55C40(ctx, base);
	// 8329F160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329F170 size=12
    let mut pc: u32 = 0x8329F170;
    'dispatch: loop {
        match pc {
            0x8329F170 => {
    //   block [0x8329F170..0x8329F17C)
	// 8329F170: 3D60832B  lis r11, -0x7cd5
	ctx.r[11].s64 = -2094333952;
	// 8329F174: 386B7ED0  addi r3, r11, 0x7ed0
	ctx.r[3].s64 = ctx.r[11].s64 + 32464;
	// 8329F178: 4BA0ADA8  b 0x82ca9f20
	sub_82CA9F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F180 size=108
    let mut pc: u32 = 0x8329F180;
    'dispatch: loop {
        match pc {
            0x8329F180 => {
    //   block [0x8329F180..0x8329F1EC)
	// 8329F180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F18C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F190: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F194: 38EBFF20  addi r7, r11, -0xe0
	ctx.r[7].s64 = ctx.r[11].s64 + -224;
	// 8329F198: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329F19C: 388AFF68  addi r4, r10, -0x98
	ctx.r[4].s64 = ctx.r[10].s64 + -152;
	// 8329F1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F1A4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329F1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F1B0: 386A9B24  addi r3, r10, -0x64dc
	ctx.r[3].s64 = ctx.r[10].s64 + -25820;
	// 8329F1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329F1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F1CC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329F1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329F1D8: 4BAB6A69  bl 0x82d55c40
	ctx.lr = 0x8329F1DC;
	sub_82D55C40(ctx, base);
	// 8329F1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F1F0 size=96
    let mut pc: u32 = 0x8329F1F0;
    'dispatch: loop {
        match pc {
            0x8329F1F0 => {
    //   block [0x8329F1F0..0x8329F250)
	// 8329F1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F1FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F204: 388A0108  addi r4, r10, 0x108
	ctx.r[4].s64 = ctx.r[10].s64 + 264;
	// 8329F208: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F210: 386A9B74  addi r3, r10, -0x648c
	ctx.r[3].s64 = ctx.r[10].s64 + -25740;
	// 8329F214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F21C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8329F220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F22C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8329F230: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329F234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329F238: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329F23C: 4BAB6A05  bl 0x82d55c40
	ctx.lr = 0x8329F240;
	sub_82D55C40(ctx, base);
	// 8329F240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F250 size=112
    let mut pc: u32 = 0x8329F250;
    'dispatch: loop {
        match pc {
            0x8329F250 => {
    //   block [0x8329F250..0x8329F2C0)
	// 8329F250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F25C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F260: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F264: 38AA9B74  addi r5, r10, -0x648c
	ctx.r[5].s64 = ctx.r[10].s64 + -25740;
	// 8329F268: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F26C: 390B00A8  addi r8, r11, 0xa8
	ctx.r[8].s64 = ctx.r[11].s64 + 168;
	// 8329F270: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329F274: 388A011C  addi r4, r10, 0x11c
	ctx.r[4].s64 = ctx.r[10].s64 + 284;
	// 8329F278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F27C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F280: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F288: 386A9BA4  addi r3, r10, -0x645c
	ctx.r[3].s64 = ctx.r[10].s64 + -25692;
	// 8329F28C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F29C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F2A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F2A4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8329F2A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F2AC: 4BAB6995  bl 0x82d55c40
	ctx.lr = 0x8329F2B0;
	sub_82D55C40(ctx, base);
	// 8329F2B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F2B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F2B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F2C0 size=108
    let mut pc: u32 = 0x8329F2C0;
    'dispatch: loop {
        match pc {
            0x8329F2C0 => {
    //   block [0x8329F2C0..0x8329F32C)
	// 8329F2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F2CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F2D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F2D4: 38EB0178  addi r7, r11, 0x178
	ctx.r[7].s64 = ctx.r[11].s64 + 376;
	// 8329F2D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329F2DC: 388A023C  addi r4, r10, 0x23c
	ctx.r[4].s64 = ctx.r[10].s64 + 572;
	// 8329F2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F2E4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F2E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329F2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F2F0: 386A9BD4  addi r3, r10, -0x642c
	ctx.r[3].s64 = ctx.r[10].s64 + -25644;
	// 8329F2F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329F2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F2FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F30C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329F310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329F318: 4BAB6929  bl 0x82d55c40
	ctx.lr = 0x8329F31C;
	sub_82D55C40(ctx, base);
	// 8329F31C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329F330 size=24
    let mut pc: u32 = 0x8329F330;
    'dispatch: loop {
        match pc {
            0x8329F330 => {
    //   block [0x8329F330..0x8329F348)
	// 8329F330: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329F334: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329F338: 394AA4B0  addi r10, r10, -0x5b50
	ctx.r[10].s64 = ctx.r[10].s64 + -23376;
	// 8329F33C: 816BA568  lwz r11, -0x5a98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23192 as u32) ) } as u64;
	// 8329F340: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8329F344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F348 size=116
    let mut pc: u32 = 0x8329F348;
    'dispatch: loop {
        match pc {
            0x8329F348 => {
    //   block [0x8329F348..0x8329F3BC)
	// 8329F348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F354: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329F358: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8329F35C: 390AA4B0  addi r8, r10, -0x5b50
	ctx.r[8].s64 = ctx.r[10].s64 + -23376;
	// 8329F360: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F364: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F368: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 8329F36C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F370: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329F374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F37C: 388A0258  addi r4, r10, 0x258
	ctx.r[4].s64 = ctx.r[10].s64 + 600;
	// 8329F380: 396B01D8  addi r11, r11, 0x1d8
	ctx.r[11].s64 = ctx.r[11].s64 + 472;
	// 8329F384: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F388: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F38C: 386A9C04  addi r3, r10, -0x63fc
	ctx.r[3].s64 = ctx.r[10].s64 + -25596;
	// 8329F390: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329F394: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F398: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329F39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F3A4: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 8329F3A8: 4BAB6899  bl 0x82d55c40
	ctx.lr = 0x8329F3AC;
	sub_82D55C40(ctx, base);
	// 8329F3AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F3B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F3C0 size=112
    let mut pc: u32 = 0x8329F3C0;
    'dispatch: loop {
        match pc {
            0x8329F3C0 => {
    //   block [0x8329F3C0..0x8329F430)
	// 8329F3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F3CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F3D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F3D4: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 8329F3D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F3DC: 390B026C  addi r8, r11, 0x26c
	ctx.r[8].s64 = ctx.r[11].s64 + 620;
	// 8329F3E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329F3E4: 388A0284  addi r4, r10, 0x284
	ctx.r[4].s64 = ctx.r[10].s64 + 644;
	// 8329F3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F3EC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F3F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F3F8: 386A9C34  addi r3, r10, -0x63cc
	ctx.r[3].s64 = ctx.r[10].s64 + -25548;
	// 8329F3FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F40C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F414: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329F418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F41C: 4BAB6825  bl 0x82d55c40
	ctx.lr = 0x8329F420;
	sub_82D55C40(ctx, base);
	// 8329F420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F430 size=112
    let mut pc: u32 = 0x8329F430;
    'dispatch: loop {
        match pc {
            0x8329F430 => {
    //   block [0x8329F430..0x8329F4A0)
	// 8329F430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F43C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F440: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F444: 38AAAD98  addi r5, r10, -0x5268
	ctx.r[5].s64 = ctx.r[10].s64 + -21096;
	// 8329F448: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F44C: 390B02C8  addi r8, r11, 0x2c8
	ctx.r[8].s64 = ctx.r[11].s64 + 712;
	// 8329F450: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329F454: 388A0328  addi r4, r10, 0x328
	ctx.r[4].s64 = ctx.r[10].s64 + 808;
	// 8329F458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F45C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F468: 386A9C64  addi r3, r10, -0x639c
	ctx.r[3].s64 = ctx.r[10].s64 + -25500;
	// 8329F46C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F484: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8329F488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F48C: 4BAB67B5  bl 0x82d55c40
	ctx.lr = 0x8329F490;
	sub_82D55C40(ctx, base);
	// 8329F490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F4A0 size=92
    let mut pc: u32 = 0x8329F4A0;
    'dispatch: loop {
        match pc {
            0x8329F4A0 => {
    //   block [0x8329F4A0..0x8329F4FC)
	// 8329F4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F4A8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F4AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329F4B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329F4B4: 4BB23BCD  bl 0x82dc3080
	ctx.lr = 0x8329F4B8;
	sub_82DC3080(ctx, base);
	// 8329F4B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F4BC: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 8329F4C0: 394B04D0  addi r10, r11, 0x4d0
	ctx.r[10].s64 = ctx.r[11].s64 + 1232;
	// 8329F4C4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329F4C8: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 8329F4CC: 396B9C94  addi r11, r11, -0x636c
	ctx.r[11].s64 = ctx.r[11].s64 + -25452;
	// 8329F4D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329F4D4: 39481E90  addi r10, r8, 0x1e90
	ctx.r[10].s64 = ctx.r[8].s64 + 7824;
	// 8329F4D8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329F4DC: 39491EA8  addi r10, r9, 0x1ea8
	ctx.r[10].s64 = ctx.r[9].s64 + 7848;
	// 8329F4E0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329F4E4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329F4E8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329F4EC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8329F4F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F4F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F4F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F500 size=112
    let mut pc: u32 = 0x8329F500;
    'dispatch: loop {
        match pc {
            0x8329F500 => {
    //   block [0x8329F500..0x8329F570)
	// 8329F500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F50C: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329F510: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F514: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329F518: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F51C: 390B0398  addi r8, r11, 0x398
	ctx.r[8].s64 = ctx.r[11].s64 + 920;
	// 8329F520: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8329F524: 388A04D0  addi r4, r10, 0x4d0
	ctx.r[4].s64 = ctx.r[10].s64 + 1232;
	// 8329F528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F52C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F538: 386A9CA4  addi r3, r10, -0x635c
	ctx.r[3].s64 = ctx.r[10].s64 + -25436;
	// 8329F53C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F554: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329F558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F55C: 4BAB66E5  bl 0x82d55c40
	ctx.lr = 0x8329F560;
	sub_82D55C40(ctx, base);
	// 8329F560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F570 size=92
    let mut pc: u32 = 0x8329F570;
    'dispatch: loop {
        match pc {
            0x8329F570 => {
    //   block [0x8329F570..0x8329F5CC)
	// 8329F570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F578: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F57C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329F580: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329F584: 4BB236E5  bl 0x82dc2c68
	ctx.lr = 0x8329F588;
	sub_82DC2C68(ctx, base);
	// 8329F588: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F58C: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 8329F590: 394B0500  addi r10, r11, 0x500
	ctx.r[10].s64 = ctx.r[11].s64 + 1280;
	// 8329F594: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329F598: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 8329F59C: 396B9CD4  addi r11, r11, -0x632c
	ctx.r[11].s64 = ctx.r[11].s64 + -25388;
	// 8329F5A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329F5A4: 39481EF0  addi r10, r8, 0x1ef0
	ctx.r[10].s64 = ctx.r[8].s64 + 7920;
	// 8329F5A8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329F5AC: 39491F08  addi r10, r9, 0x1f08
	ctx.r[10].s64 = ctx.r[9].s64 + 7944;
	// 8329F5B0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329F5B4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329F5B8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329F5BC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8329F5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F5D0 size=112
    let mut pc: u32 = 0x8329F5D0;
    'dispatch: loop {
        match pc {
            0x8329F5D0 => {
    //   block [0x8329F5D0..0x8329F640)
	// 8329F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F5DC: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329F5E0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F5E4: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329F5E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F5EC: 390B0428  addi r8, r11, 0x428
	ctx.r[8].s64 = ctx.r[11].s64 + 1064;
	// 8329F5F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8329F5F4: 388A0500  addi r4, r10, 0x500
	ctx.r[4].s64 = ctx.r[10].s64 + 1280;
	// 8329F5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F5FC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F608: 386A9CE4  addi r3, r10, -0x631c
	ctx.r[3].s64 = ctx.r[10].s64 + -25372;
	// 8329F60C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F624: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 8329F628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F62C: 4BAB6615  bl 0x82d55c40
	ctx.lr = 0x8329F630;
	sub_82D55C40(ctx, base);
	// 8329F630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F640 size=92
    let mut pc: u32 = 0x8329F640;
    'dispatch: loop {
        match pc {
            0x8329F640 => {
    //   block [0x8329F640..0x8329F69C)
	// 8329F640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F648: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F64C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329F650: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329F654: 4BB2362D  bl 0x82dc2c80
	ctx.lr = 0x8329F658;
	sub_82DC2C80(ctx, base);
	// 8329F658: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F65C: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 8329F660: 394B0530  addi r10, r11, 0x530
	ctx.r[10].s64 = ctx.r[11].s64 + 1328;
	// 8329F664: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329F668: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 8329F66C: 396B9D14  addi r11, r11, -0x62ec
	ctx.r[11].s64 = ctx.r[11].s64 + -25324;
	// 8329F670: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329F674: 39481F50  addi r10, r8, 0x1f50
	ctx.r[10].s64 = ctx.r[8].s64 + 8016;
	// 8329F678: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329F67C: 39491F68  addi r10, r9, 0x1f68
	ctx.r[10].s64 = ctx.r[9].s64 + 8040;
	// 8329F680: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329F684: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329F688: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329F68C: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 8329F690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F6A0 size=112
    let mut pc: u32 = 0x8329F6A0;
    'dispatch: loop {
        match pc {
            0x8329F6A0 => {
    //   block [0x8329F6A0..0x8329F710)
	// 8329F6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F6AC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F6B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F6B4: 38AAA114  addi r5, r10, -0x5eec
	ctx.r[5].s64 = ctx.r[10].s64 + -24300;
	// 8329F6B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F6BC: 390B0488  addi r8, r11, 0x488
	ctx.r[8].s64 = ctx.r[11].s64 + 1160;
	// 8329F6C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329F6C4: 388A0530  addi r4, r10, 0x530
	ctx.r[4].s64 = ctx.r[10].s64 + 1328;
	// 8329F6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F6CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F6D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F6D8: 386A9D24  addi r3, r10, -0x62dc
	ctx.r[3].s64 = ctx.r[10].s64 + -25308;
	// 8329F6DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F6E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F6F4: 38C00130  li r6, 0x130
	ctx.r[6].s64 = 304;
	// 8329F6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F6FC: 4BAB6545  bl 0x82d55c40
	ctx.lr = 0x8329F700;
	sub_82D55C40(ctx, base);
	// 8329F700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F710 size=100
    let mut pc: u32 = 0x8329F710;
    'dispatch: loop {
        match pc {
            0x8329F710 => {
    //   block [0x8329F710..0x8329F774)
	// 8329F710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F71C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F724: 392A065C  addi r9, r10, 0x65c
	ctx.r[9].s64 = ctx.r[10].s64 + 1628;
	// 8329F728: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F730: 388A0698  addi r4, r10, 0x698
	ctx.r[4].s64 = ctx.r[10].s64 + 1688;
	// 8329F734: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F744: 386A9D54  addi r3, r10, -0x62ac
	ctx.r[3].s64 = ctx.r[10].s64 + -25260;
	// 8329F748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F74C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8329F750: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8329F754: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8329F758: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8329F75C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329F760: 4BAB64E1  bl 0x82d55c40
	ctx.lr = 0x8329F764;
	sub_82D55C40(ctx, base);
	// 8329F764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F76C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F778 size=92
    let mut pc: u32 = 0x8329F778;
    'dispatch: loop {
        match pc {
            0x8329F778 => {
    //   block [0x8329F778..0x8329F7D4)
	// 8329F778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F780: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F784: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329F788: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329F78C: 4BB2554D  bl 0x82dc4cd8
	ctx.lr = 0x8329F790;
	sub_82DC4CD8(ctx, base);
	// 8329F790: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F794: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 8329F798: 394B0760  addi r10, r11, 0x760
	ctx.r[10].s64 = ctx.r[11].s64 + 1888;
	// 8329F79C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329F7A0: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 8329F7A4: 396B9D84  addi r11, r11, -0x627c
	ctx.r[11].s64 = ctx.r[11].s64 + -25212;
	// 8329F7A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329F7AC: 39482050  addi r10, r8, 0x2050
	ctx.r[10].s64 = ctx.r[8].s64 + 8272;
	// 8329F7B0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329F7B4: 39492068  addi r10, r9, 0x2068
	ctx.r[10].s64 = ctx.r[9].s64 + 8296;
	// 8329F7B8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329F7BC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329F7C0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329F7C4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8329F7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F7D8 size=112
    let mut pc: u32 = 0x8329F7D8;
    'dispatch: loop {
        match pc {
            0x8329F7D8 => {
    //   block [0x8329F7D8..0x8329F848)
	// 8329F7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F7E4: 3D408336  lis r10, -0x7cca
	ctx.r[10].s64 = -2093613056;
	// 8329F7E8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F7EC: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 8329F7F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F7F4: 390B06B8  addi r8, r11, 0x6b8
	ctx.r[8].s64 = ctx.r[11].s64 + 1720;
	// 8329F7F8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8329F7FC: 388A0760  addi r4, r10, 0x760
	ctx.r[4].s64 = ctx.r[10].s64 + 1888;
	// 8329F800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F804: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F810: 386A9D94  addi r3, r10, -0x626c
	ctx.r[3].s64 = ctx.r[10].s64 + -25196;
	// 8329F814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F82C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 8329F830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F834: 4BAB640D  bl 0x82d55c40
	ctx.lr = 0x8329F838;
	sub_82D55C40(ctx, base);
	// 8329F838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F848 size=92
    let mut pc: u32 = 0x8329F848;
    'dispatch: loop {
        match pc {
            0x8329F848 => {
    //   block [0x8329F848..0x8329F8A4)
	// 8329F848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F850: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F854: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8329F858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8329F85C: 4BB254AD  bl 0x82dc4d08
	ctx.lr = 0x8329F860;
	sub_82DC4D08(ctx, base);
	// 8329F860: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F864: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 8329F868: 394B0784  addi r10, r11, 0x784
	ctx.r[10].s64 = ctx.r[11].s64 + 1924;
	// 8329F86C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329F870: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 8329F874: 396B9DC4  addi r11, r11, -0x623c
	ctx.r[11].s64 = ctx.r[11].s64 + -25148;
	// 8329F878: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8329F87C: 394820B0  addi r10, r8, 0x20b0
	ctx.r[10].s64 = ctx.r[8].s64 + 8368;
	// 8329F880: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8329F884: 394920C8  addi r10, r9, 0x20c8
	ctx.r[10].s64 = ctx.r[9].s64 + 8392;
	// 8329F888: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329F88C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8329F890: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8329F894: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8329F898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F89C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F8A8 size=112
    let mut pc: u32 = 0x8329F8A8;
    'dispatch: loop {
        match pc {
            0x8329F8A8 => {
    //   block [0x8329F8A8..0x8329F918)
	// 8329F8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F8B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F8B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F8BC: 38AAA314  addi r5, r10, -0x5cec
	ctx.r[5].s64 = ctx.r[10].s64 + -23788;
	// 8329F8C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F8C4: 390B0748  addi r8, r11, 0x748
	ctx.r[8].s64 = ctx.r[11].s64 + 1864;
	// 8329F8C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329F8CC: 388A0784  addi r4, r10, 0x784
	ctx.r[4].s64 = ctx.r[10].s64 + 1924;
	// 8329F8D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F8D4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F8D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F8E0: 386A9DD4  addi r3, r10, -0x622c
	ctx.r[3].s64 = ctx.r[10].s64 + -25132;
	// 8329F8E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F8E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F8FC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329F900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F904: 4BAB633D  bl 0x82d55c40
	ctx.lr = 0x8329F908;
	sub_82D55C40(ctx, base);
	// 8329F908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F918 size=112
    let mut pc: u32 = 0x8329F918;
    'dispatch: loop {
        match pc {
            0x8329F918 => {
    //   block [0x8329F918..0x8329F988)
	// 8329F918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F924: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F928: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F92C: 38AAA854  addi r5, r10, -0x57ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22444;
	// 8329F930: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F934: 390B07C0  addi r8, r11, 0x7c0
	ctx.r[8].s64 = ctx.r[11].s64 + 1984;
	// 8329F938: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329F93C: 388A0860  addi r4, r10, 0x860
	ctx.r[4].s64 = ctx.r[10].s64 + 2144;
	// 8329F940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F944: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F950: 386A9E04  addi r3, r10, -0x61fc
	ctx.r[3].s64 = ctx.r[10].s64 + -25084;
	// 8329F954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F96C: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329F970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F974: 4BAB62CD  bl 0x82d55c40
	ctx.lr = 0x8329F978;
	sub_82D55C40(ctx, base);
	// 8329F978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F988 size=112
    let mut pc: u32 = 0x8329F988;
    'dispatch: loop {
        match pc {
            0x8329F988 => {
    //   block [0x8329F988..0x8329F9F8)
	// 8329F988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329F990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329F994: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F998: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329F99C: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 8329F9A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329F9A4: 390B0890  addi r8, r11, 0x890
	ctx.r[8].s64 = ctx.r[11].s64 + 2192;
	// 8329F9A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8329F9AC: 388A0900  addi r4, r10, 0x900
	ctx.r[4].s64 = ctx.r[10].s64 + 2304;
	// 8329F9B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329F9B4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329F9B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329F9BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329F9C0: 386A9E34  addi r3, r10, -0x61cc
	ctx.r[3].s64 = ctx.r[10].s64 + -25036;
	// 8329F9C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329F9C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329F9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329F9D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329F9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329F9D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329F9DC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 8329F9E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329F9E4: 4BAB625D  bl 0x82d55c40
	ctx.lr = 0x8329F9E8;
	sub_82D55C40(ctx, base);
	// 8329F9E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329F9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329F9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329F9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329F9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329F9F8 size=112
    let mut pc: u32 = 0x8329F9F8;
    'dispatch: loop {
        match pc {
            0x8329F9F8 => {
    //   block [0x8329F9F8..0x8329FA68)
	// 8329F9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329F9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FA04: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FA08: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FA0C: 38AAA6A4  addi r5, r10, -0x595c
	ctx.r[5].s64 = ctx.r[10].s64 + -22876;
	// 8329FA10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FA14: 390B092C  addi r8, r11, 0x92c
	ctx.r[8].s64 = ctx.r[11].s64 + 2348;
	// 8329FA18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329FA1C: 388A095C  addi r4, r10, 0x95c
	ctx.r[4].s64 = ctx.r[10].s64 + 2396;
	// 8329FA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FA24: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FA28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FA2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FA30: 386A9E64  addi r3, r10, -0x619c
	ctx.r[3].s64 = ctx.r[10].s64 + -24988;
	// 8329FA34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329FA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FA4C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329FA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FA54: 4BAB61ED  bl 0x82d55c40
	ctx.lr = 0x8329FA58;
	sub_82D55C40(ctx, base);
	// 8329FA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FA68 size=112
    let mut pc: u32 = 0x8329FA68;
    'dispatch: loop {
        match pc {
            0x8329FA68 => {
    //   block [0x8329FA68..0x8329FAD8)
	// 8329FA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FA74: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FA78: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FA7C: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 8329FA80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FA84: 390B0980  addi r8, r11, 0x980
	ctx.r[8].s64 = ctx.r[11].s64 + 2432;
	// 8329FA88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329FA8C: 388A09FC  addi r4, r10, 0x9fc
	ctx.r[4].s64 = ctx.r[10].s64 + 2556;
	// 8329FA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FA94: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FAA0: 386A9E94  addi r3, r10, -0x616c
	ctx.r[3].s64 = ctx.r[10].s64 + -24940;
	// 8329FAA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329FAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FABC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8329FAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FAC4: 4BAB617D  bl 0x82d55c40
	ctx.lr = 0x8329FAC8;
	sub_82D55C40(ctx, base);
	// 8329FAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329FAD8 size=24
    let mut pc: u32 = 0x8329FAD8;
    'dispatch: loop {
        match pc {
            0x8329FAD8 => {
    //   block [0x8329FAD8..0x8329FAF0)
	// 8329FAD8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329FADC: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329FAE0: 394AA720  addi r10, r10, -0x58e0
	ctx.r[10].s64 = ctx.r[10].s64 + -22752;
	// 8329FAE4: 816BA568  lwz r11, -0x5a98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23192 as u32) ) } as u64;
	// 8329FAE8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8329FAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FAF0 size=116
    let mut pc: u32 = 0x8329FAF0;
    'dispatch: loop {
        match pc {
            0x8329FAF0 => {
    //   block [0x8329FAF0..0x8329FB64)
	// 8329FAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FAFC: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329FB00: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8329FB04: 390AA720  addi r8, r10, -0x58e0
	ctx.r[8].s64 = ctx.r[10].s64 + -22752;
	// 8329FB08: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FB0C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FB10: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 8329FB14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FB18: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329FB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FB24: 388A0A40  addi r4, r10, 0xa40
	ctx.r[4].s64 = ctx.r[10].s64 + 2624;
	// 8329FB28: 396B0A20  addi r11, r11, 0xa20
	ctx.r[11].s64 = ctx.r[11].s64 + 2592;
	// 8329FB2C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FB30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FB34: 386A9EC4  addi r3, r10, -0x613c
	ctx.r[3].s64 = ctx.r[10].s64 + -24892;
	// 8329FB38: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329FB3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FB40: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329FB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FB4C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8329FB50: 4BAB60F1  bl 0x82d55c40
	ctx.lr = 0x8329FB54;
	sub_82D55C40(ctx, base);
	// 8329FB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FB68 size=108
    let mut pc: u32 = 0x8329FB68;
    'dispatch: loop {
        match pc {
            0x8329FB68 => {
    //   block [0x8329FB68..0x8329FBD4)
	// 8329FB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FB74: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FB78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FB7C: 38EB0A90  addi r7, r11, 0xa90
	ctx.r[7].s64 = ctx.r[11].s64 + 2704;
	// 8329FB80: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8329FB84: 388A0AF0  addi r4, r10, 0xaf0
	ctx.r[4].s64 = ctx.r[10].s64 + 2800;
	// 8329FB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FB8C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FB90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329FB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FB98: 386A9EF4  addi r3, r10, -0x610c
	ctx.r[3].s64 = ctx.r[10].s64 + -24844;
	// 8329FB9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329FBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FBB4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8329FBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FBBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329FBC0: 4BAB6081  bl 0x82d55c40
	ctx.lr = 0x8329FBC4;
	sub_82D55C40(ctx, base);
	// 8329FBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FBD8 size=108
    let mut pc: u32 = 0x8329FBD8;
    'dispatch: loop {
        match pc {
            0x8329FBD8 => {
    //   block [0x8329FBD8..0x8329FC44)
	// 8329FBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FBE4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FBE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FBEC: 38EB0C50  addi r7, r11, 0xc50
	ctx.r[7].s64 = ctx.r[11].s64 + 3152;
	// 8329FBF0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8329FBF4: 388A0DA0  addi r4, r10, 0xda0
	ctx.r[4].s64 = ctx.r[10].s64 + 3488;
	// 8329FBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FBFC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FC00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329FC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FC08: 386A9F24  addi r3, r10, -0x60dc
	ctx.r[3].s64 = ctx.r[10].s64 + -24796;
	// 8329FC0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329FC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FC24: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8329FC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FC2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329FC30: 4BAB6011  bl 0x82d55c40
	ctx.lr = 0x8329FC34;
	sub_82D55C40(ctx, base);
	// 8329FC34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FC38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FC3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FC48 size=116
    let mut pc: u32 = 0x8329FC48;
    'dispatch: loop {
        match pc {
            0x8329FC48 => {
    //   block [0x8329FC48..0x8329FCBC)
	// 8329FC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FC54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FC58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FC5C: 390B0D10  addi r8, r11, 0xd10
	ctx.r[8].s64 = ctx.r[11].s64 + 3344;
	// 8329FC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FC64: 392A0C38  addi r9, r10, 0xc38
	ctx.r[9].s64 = ctx.r[10].s64 + 3128;
	// 8329FC68: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FC6C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8329FC70: 38AA9EF4  addi r5, r10, -0x610c
	ctx.r[5].s64 = ctx.r[10].s64 + -24844;
	// 8329FC74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FC7C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FC8C: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329FC90: 388A0DC0  addi r4, r10, 0xdc0
	ctx.r[4].s64 = ctx.r[10].s64 + 3520;
	// 8329FC94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329FC98: 386B9F54  addi r3, r11, -0x60ac
	ctx.r[3].s64 = ctx.r[11].s64 + -24748;
	// 8329FC9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329FCA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FCA4: 38C0004C  li r6, 0x4c
	ctx.r[6].s64 = 76;
	// 8329FCA8: 4BAB5F99  bl 0x82d55c40
	ctx.lr = 0x8329FCAC;
	sub_82D55C40(ctx, base);
	// 8329FCAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FCB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FCB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FCC0 size=112
    let mut pc: u32 = 0x8329FCC0;
    'dispatch: loop {
        match pc {
            0x8329FCC0 => {
    //   block [0x8329FCC0..0x8329FD30)
	// 8329FCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FCC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FCCC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FCD0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FCD4: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 8329FCD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FCDC: 390B0DDC  addi r8, r11, 0xddc
	ctx.r[8].s64 = ctx.r[11].s64 + 3548;
	// 8329FCE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8329FCE4: 388A0DF4  addi r4, r10, 0xdf4
	ctx.r[4].s64 = ctx.r[10].s64 + 3572;
	// 8329FCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FCEC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FCF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FCF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FCF8: 386A9F84  addi r3, r10, -0x607c
	ctx.r[3].s64 = ctx.r[10].s64 + -24700;
	// 8329FCFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329FD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FD14: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329FD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FD1C: 4BAB5F25  bl 0x82d55c40
	ctx.lr = 0x8329FD20;
	sub_82D55C40(ctx, base);
	// 8329FD20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FD30 size=112
    let mut pc: u32 = 0x8329FD30;
    'dispatch: loop {
        match pc {
            0x8329FD30 => {
    //   block [0x8329FD30..0x8329FDA0)
	// 8329FD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FD3C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FD40: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FD44: 38AAA884  addi r5, r10, -0x577c
	ctx.r[5].s64 = ctx.r[10].s64 + -22396;
	// 8329FD48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FD4C: 390B0E14  addi r8, r11, 0xe14
	ctx.r[8].s64 = ctx.r[11].s64 + 3604;
	// 8329FD50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8329FD54: 388A0E6C  addi r4, r10, 0xe6c
	ctx.r[4].s64 = ctx.r[10].s64 + 3692;
	// 8329FD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FD5C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FD60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FD64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FD68: 386A9FB4  addi r3, r10, -0x604c
	ctx.r[3].s64 = ctx.r[10].s64 + -24652;
	// 8329FD6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329FD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FD84: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8329FD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FD8C: 4BAB5EB5  bl 0x82d55c40
	ctx.lr = 0x8329FD90;
	sub_82D55C40(ctx, base);
	// 8329FD90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FD94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FD98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FD9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FDA0 size=108
    let mut pc: u32 = 0x8329FDA0;
    'dispatch: loop {
        match pc {
            0x8329FDA0 => {
    //   block [0x8329FDA0..0x8329FE0C)
	// 8329FDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FDA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FDAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FDB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FDB4: 38EB0EC0  addi r7, r11, 0xec0
	ctx.r[7].s64 = ctx.r[11].s64 + 3776;
	// 8329FDB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8329FDBC: 388A0F80  addi r4, r10, 0xf80
	ctx.r[4].s64 = ctx.r[10].s64 + 3968;
	// 8329FDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FDC4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FDC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329FDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FDD0: 386A9FE4  addi r3, r10, -0x601c
	ctx.r[3].s64 = ctx.r[10].s64 + -24604;
	// 8329FDD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329FDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FDDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FDEC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8329FDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FDF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329FDF8: 4BAB5E49  bl 0x82d55c40
	ctx.lr = 0x8329FDFC;
	sub_82D55C40(ctx, base);
	// 8329FDFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FE10 size=112
    let mut pc: u32 = 0x8329FE10;
    'dispatch: loop {
        match pc {
            0x8329FE10 => {
    //   block [0x8329FE10..0x8329FE80)
	// 8329FE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FE1C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FE20: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FE24: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 8329FE28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FE2C: 390B0F08  addi r8, r11, 0xf08
	ctx.r[8].s64 = ctx.r[11].s64 + 3848;
	// 8329FE30: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8329FE34: 388A0FA4  addi r4, r10, 0xfa4
	ctx.r[4].s64 = ctx.r[10].s64 + 4004;
	// 8329FE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FE3C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FE40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FE44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FE48: 386AA014  addi r3, r10, -0x5fec
	ctx.r[3].s64 = ctx.r[10].s64 + -24556;
	// 8329FE4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8329FE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FE64: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8329FE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FE6C: 4BAB5DD5  bl 0x82d55c40
	ctx.lr = 0x8329FE70;
	sub_82D55C40(ctx, base);
	// 8329FE70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FE74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FE78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FE80 size=116
    let mut pc: u32 = 0x8329FE80;
    'dispatch: loop {
        match pc {
            0x8329FE80 => {
    //   block [0x8329FE80..0x8329FEF4)
	// 8329FE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FE88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FE8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FE90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FE94: 390B1018  addi r8, r11, 0x1018
	ctx.r[8].s64 = ctx.r[11].s64 + 4120;
	// 8329FE98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FE9C: 392A1004  addi r9, r10, 0x1004
	ctx.r[9].s64 = ctx.r[10].s64 + 4100;
	// 8329FEA0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FEA4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8329FEA8: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 8329FEAC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FEB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FEBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FEC4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 8329FEC8: 388A1048  addi r4, r10, 0x1048
	ctx.r[4].s64 = ctx.r[10].s64 + 4168;
	// 8329FECC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329FED0: 386BA044  addi r3, r11, -0x5fbc
	ctx.r[3].s64 = ctx.r[11].s64 + -24508;
	// 8329FED4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8329FED8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FEDC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 8329FEE0: 4BAB5D61  bl 0x82d55c40
	ctx.lr = 0x8329FEE4;
	sub_82D55C40(ctx, base);
	// 8329FEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329FEF8 size=36
    let mut pc: u32 = 0x8329FEF8;
    'dispatch: loop {
        match pc {
            0x8329FEF8 => {
    //   block [0x8329FEF8..0x8329FF1C)
	// 8329FEF8: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329FEFC: 814BA890  lwz r10, -0x5770(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22384 as u32) ) } as u64;
	// 8329FF00: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329FF04: 396BA8C8  addi r11, r11, -0x5738
	ctx.r[11].s64 = ctx.r[11].s64 + -22328;
	// 8329FF08: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8329FF0C: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329FF10: 814AA888  lwz r10, -0x5778(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-22392 as u32) ) } as u64;
	// 8329FF14: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8329FF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FF20 size=108
    let mut pc: u32 = 0x8329FF20;
    'dispatch: loop {
        match pc {
            0x8329FF20 => {
    //   block [0x8329FF20..0x8329FF8C)
	// 8329FF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FF2C: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329FF30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FF34: 38EBA8C8  addi r7, r11, -0x5738
	ctx.r[7].s64 = ctx.r[11].s64 + -22328;
	// 8329FF38: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8329FF3C: 388A1380  addi r4, r10, 0x1380
	ctx.r[4].s64 = ctx.r[10].s64 + 4992;
	// 8329FF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8329FF44: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FF48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8329FF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FF50: 386AA074  addi r3, r10, -0x5f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -24460;
	// 8329FF54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8329FF58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8329FF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8329FF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8329FF6C: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8329FF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8329FF74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8329FF78: 4BAB5CC9  bl 0x82d55c40
	ctx.lr = 0x8329FF7C;
	sub_82D55C40(ctx, base);
	// 8329FF7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8329FF80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8329FF84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8329FF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8329FF90 size=24
    let mut pc: u32 = 0x8329FF90;
    'dispatch: loop {
        match pc {
            0x8329FF90 => {
    //   block [0x8329FF90..0x8329FFA8)
	// 8329FF90: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 8329FF94: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329FF98: 394AA9A0  addi r10, r10, -0x5660
	ctx.r[10].s64 = ctx.r[10].s64 + -22112;
	// 8329FF9C: 816BA888  lwz r11, -0x5778(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22392 as u32) ) } as u64;
	// 8329FFA0: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 8329FFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8329FFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8329FFA8 size=116
    let mut pc: u32 = 0x8329FFA8;
    'dispatch: loop {
        match pc {
            0x8329FFA8 => {
    //   block [0x8329FFA8..0x832A001C)
	// 8329FFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8329FFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8329FFB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8329FFB4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 8329FFB8: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8329FFBC: 390AA9A0  addi r8, r10, -0x5660
	ctx.r[8].s64 = ctx.r[10].s64 + -22112;
	// 8329FFC0: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FFC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8329FFC8: 38AAA074  addi r5, r10, -0x5f8c
	ctx.r[5].s64 = ctx.r[10].s64 + -24460;
	// 8329FFCC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8329FFD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8329FFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8329FFD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8329FFDC: 388A1424  addi r4, r10, 0x1424
	ctx.r[4].s64 = ctx.r[10].s64 + 5156;
	// 8329FFE0: 396B12AC  addi r11, r11, 0x12ac
	ctx.r[11].s64 = ctx.r[11].s64 + 4780;
	// 8329FFE4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 8329FFE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8329FFEC: 386AA0A4  addi r3, r10, -0x5f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -24412;
	// 8329FFF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8329FFF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8329FFF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8329FFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0004: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 832A0008: 4BAB5C39  bl 0x82d55c40
	ctx.lr = 0x832A000C;
	sub_82D55C40(ctx, base);
	// 832A000C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0020 size=112
    let mut pc: u32 = 0x832A0020;
    'dispatch: loop {
        match pc {
            0x832A0020 => {
    //   block [0x832A0020..0x832A0090)
	// 832A0020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A002C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0030: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0034: 38AAA074  addi r5, r10, -0x5f8c
	ctx.r[5].s64 = ctx.r[10].s64 + -24460;
	// 832A0038: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A003C: 390B12D8  addi r8, r11, 0x12d8
	ctx.r[8].s64 = ctx.r[11].s64 + 4824;
	// 832A0040: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 832A0044: 388A1474  addi r4, r10, 0x1474
	ctx.r[4].s64 = ctx.r[10].s64 + 5236;
	// 832A0048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A004C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0050: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0058: 386AA0D4  addi r3, r10, -0x5f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -24364;
	// 832A005C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A006C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0074: 38C00090  li r6, 0x90
	ctx.r[6].s64 = 144;
	// 832A0078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A007C: 4BAB5BC5  bl 0x82d55c40
	ctx.lr = 0x832A0080;
	sub_82D55C40(ctx, base);
	// 832A0080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A008C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0090 size=92
    let mut pc: u32 = 0x832A0090;
    'dispatch: loop {
        match pc {
            0x832A0090 => {
    //   block [0x832A0090..0x832A00EC)
	// 832A0090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0098: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A009C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A00A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832A00A4: 4BB14E35  bl 0x82db4ed8
	ctx.lr = 0x832A00A8;
	sub_82DB4ED8(ctx, base);
	// 832A00A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A00AC: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 832A00B0: 394B1498  addi r10, r11, 0x1498
	ctx.r[10].s64 = ctx.r[11].s64 + 5272;
	// 832A00B4: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A00B8: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 832A00BC: 396BA104  addi r11, r11, -0x5efc
	ctx.r[11].s64 = ctx.r[11].s64 + -24316;
	// 832A00C0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A00C4: 39482A28  addi r10, r8, 0x2a28
	ctx.r[10].s64 = ctx.r[8].s64 + 10792;
	// 832A00C8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A00CC: 39492A40  addi r10, r9, 0x2a40
	ctx.r[10].s64 = ctx.r[9].s64 + 10816;
	// 832A00D0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A00D4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832A00D8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 832A00DC: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 832A00E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A00E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A00E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A00F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x832A00F0 size=24
    let mut pc: u32 = 0x832A00F0;
    'dispatch: loop {
        match pc {
            0x832A00F0 => {
    //   block [0x832A00F0..0x832A0108)
	// 832A00F0: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A00F4: 3D408330  lis r10, -0x7cd0
	ctx.r[10].s64 = -2094006272;
	// 832A00F8: 394AAA90  addi r10, r10, -0x5570
	ctx.r[10].s64 = ctx.r[10].s64 + -21872;
	// 832A00FC: 816BA568  lwz r11, -0x5a98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23192 as u32) ) } as u64;
	// 832A0100: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 832A0104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0108 size=116
    let mut pc: u32 = 0x832A0108;
    'dispatch: loop {
        match pc {
            0x832A0108 => {
    //   block [0x832A0108..0x832A017C)
	// 832A0108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A010C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A0114: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0118: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A011C: 392B1270  addi r9, r11, 0x1270
	ctx.r[9].s64 = ctx.r[11].s64 + 4720;
	// 832A0120: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 832A0124: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A0128: 38E900E0  addi r7, r9, 0xe0
	ctx.r[7].s64 = ctx.r[9].s64 + 224;
	// 832A012C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 832A0130: 3D608330  lis r11, -0x7cd0
	ctx.r[11].s64 = -2094006272;
	// 832A0134: 388A1498  addi r4, r10, 0x1498
	ctx.r[4].s64 = ctx.r[10].s64 + 5272;
	// 832A0138: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A013C: 396BAA90  addi r11, r11, -0x5570
	ctx.r[11].s64 = ctx.r[11].s64 + -21872;
	// 832A0140: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 832A0144: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0148: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 832A014C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0150: 386AA114  addi r3, r10, -0x5eec
	ctx.r[3].s64 = ctx.r[10].s64 + -24300;
	// 832A0154: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 832A0158: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 832A015C: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 832A0160: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 832A0164: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 832A0168: 4BAB5AD9  bl 0x82d55c40
	ctx.lr = 0x832A016C;
	sub_82D55C40(ctx, base);
	// 832A016C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0180 size=112
    let mut pc: u32 = 0x832A0180;
    'dispatch: loop {
        match pc {
            0x832A0180 => {
    //   block [0x832A0180..0x832A01F0)
	// 832A0180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A018C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0190: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0194: 38AAA704  addi r5, r10, -0x58fc
	ctx.r[5].s64 = ctx.r[10].s64 + -22780;
	// 832A0198: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A019C: 390B14F0  addi r8, r11, 0x14f0
	ctx.r[8].s64 = ctx.r[11].s64 + 5360;
	// 832A01A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 832A01A4: 388A1550  addi r4, r10, 0x1550
	ctx.r[4].s64 = ctx.r[10].s64 + 5456;
	// 832A01A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A01AC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A01B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A01B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A01B8: 386AA144  addi r3, r10, -0x5ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -24252;
	// 832A01BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A01C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A01C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A01C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A01CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A01D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A01D4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 832A01D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A01DC: 4BAB5A65  bl 0x82d55c40
	ctx.lr = 0x832A01E0;
	sub_82D55C40(ctx, base);
	// 832A01E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A01E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A01E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A01EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A01F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A01F0 size=112
    let mut pc: u32 = 0x832A01F0;
    'dispatch: loop {
        match pc {
            0x832A01F0 => {
    //   block [0x832A01F0..0x832A0260)
	// 832A01F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A01F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A01F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A01FC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0200: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0204: 38AAA764  addi r5, r10, -0x589c
	ctx.r[5].s64 = ctx.r[10].s64 + -22684;
	// 832A0208: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A020C: 390B15A8  addi r8, r11, 0x15a8
	ctx.r[8].s64 = ctx.r[11].s64 + 5544;
	// 832A0210: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 832A0214: 388A1620  addi r4, r10, 0x1620
	ctx.r[4].s64 = ctx.r[10].s64 + 5664;
	// 832A0218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A021C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0228: 386AA174  addi r3, r10, -0x5e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -24204;
	// 832A022C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A0230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A0234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A023C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 832A0240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A0244: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 832A0248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A024C: 4BAB59F5  bl 0x82d55c40
	ctx.lr = 0x832A0250;
	sub_82D55C40(ctx, base);
	// 832A0250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A025C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0260 size=112
    let mut pc: u32 = 0x832A0260;
    'dispatch: loop {
        match pc {
            0x832A0260 => {
    //   block [0x832A0260..0x832A02D0)
	// 832A0260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A026C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0270: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A0274: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 832A0278: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A027C: 390B1658  addi r8, r11, 0x1658
	ctx.r[8].s64 = ctx.r[11].s64 + 5720;
	// 832A0280: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 832A0284: 388A16EC  addi r4, r10, 0x16ec
	ctx.r[4].s64 = ctx.r[10].s64 + 5868;
	// 832A0288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A028C: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A0290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A0294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0298: 386AA1A4  addi r3, r10, -0x5e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -24156;
	// 832A029C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A02A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A02A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A02A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A02AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A02B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A02B4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 832A02B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A02BC: 4BAB5985  bl 0x82d55c40
	ctx.lr = 0x832A02C0;
	sub_82D55C40(ctx, base);
	// 832A02C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A02C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A02C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A02CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A02D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A02D0 size=108
    let mut pc: u32 = 0x832A02D0;
    'dispatch: loop {
        match pc {
            0x832A02D0 => {
    //   block [0x832A02D0..0x832A033C)
	// 832A02D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A02D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A02D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A02DC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A02E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A02E4: 38EB1740  addi r7, r11, 0x1740
	ctx.r[7].s64 = ctx.r[11].s64 + 5952;
	// 832A02E8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 832A02EC: 388A1800  addi r4, r10, 0x1800
	ctx.r[4].s64 = ctx.r[10].s64 + 6144;
	// 832A02F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A02F4: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A02F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 832A02FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A0300: 386AA1D4  addi r3, r10, -0x5e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -24108;
	// 832A0304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 832A0308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A030C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A0310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A0314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A0318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A031C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 832A0320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A0324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 832A0328: 4BAB5919  bl 0x82d55c40
	ctx.lr = 0x832A032C;
	sub_82D55C40(ctx, base);
	// 832A032C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A0340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A0340 size=92
    let mut pc: u32 = 0x832A0340;
    'dispatch: loop {
        match pc {
            0x832A0340 => {
    //   block [0x832A0340..0x832A039C)
	// 832A0340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A0344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A0348: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A034C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 832A0350: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 832A0354: 4BB2715D  bl 0x82dc74b0
	ctx.lr = 0x832A0358;
	sub_82DC74B0(ctx, base);
	// 832A0358: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A035C: 3D0082DB  lis r8, -0x7d25
	ctx.r[8].s64 = -2099576832;
	// 832A0360: 394B1818  addi r10, r11, 0x1818
	ctx.r[10].s64 = ctx.r[11].s64 + 6168;
	// 832A0364: 3D608334  lis r11, -0x7ccc
	ctx.r[11].s64 = -2093744128;
	// 832A0368: 3D2082DB  lis r9, -0x7d25
	ctx.r[9].s64 = -2099576832;
	// 832A036C: 396BA204  addi r11, r11, -0x5dfc
	ctx.r[11].s64 = ctx.r[11].s64 + -24060;
	// 832A0370: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 832A0374: 39482D98  addi r10, r8, 0x2d98
	ctx.r[10].s64 = ctx.r[8].s64 + 11672;
	// 832A0378: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 832A037C: 39492D80  addi r10, r9, 0x2d80
	ctx.r[10].s64 = ctx.r[9].s64 + 11648;
	// 832A0380: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 832A0384: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 832A0388: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 832A038C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 832A0390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A0398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_832A03A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x832A03A0 size=112
    let mut pc: u32 = 0x832A03A0;
    'dispatch: loop {
        match pc {
            0x832A03A0 => {
    //   block [0x832A03A0..0x832A0410)
	// 832A03A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 832A03A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 832A03A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 832A03AC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A03B0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 832A03B4: 38AAA8B4  addi r5, r10, -0x574c
	ctx.r[5].s64 = ctx.r[10].s64 + -22348;
	// 832A03B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 832A03BC: 390B17A0  addi r8, r11, 0x17a0
	ctx.r[8].s64 = ctx.r[11].s64 + 6048;
	// 832A03C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 832A03C4: 388A1818  addi r4, r10, 0x1818
	ctx.r[4].s64 = ctx.r[10].s64 + 6168;
	// 832A03C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 832A03CC: 3D408334  lis r10, -0x7ccc
	ctx.r[10].s64 = -2093744128;
	// 832A03D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 832A03D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 832A03D8: 386AA214  addi r3, r10, -0x5dec
	ctx.r[3].s64 = ctx.r[10].s64 + -24044;
	// 832A03DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 832A03E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 832A03E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 832A03E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 832A03EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 832A03F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 832A03F4: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 832A03F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 832A03FC: 4BAB5845  bl 0x82d55c40
	ctx.lr = 0x832A0400;
	sub_82D55C40(ctx, base);
	// 832A0400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 832A0404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 832A0408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 832A040C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


