pub fn sub_82683CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683CE0 size=100
    let mut pc: u32 = 0x82683CE0;
    'dispatch: loop {
        match pc {
            0x82683CE0 => {
    //   block [0x82683CE0..0x82683D44)
	// 82683CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683CEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683CF4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683D00: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 82683D04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683D14: 386A9E28  addi r3, r10, -0x61d8
	ctx.r[3].s64 = ctx.r[10].s64 + -25048;
	// 82683D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683D1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683D20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82683D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683D28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82683D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683D30: 4BDE30F1  bl 0x82466e20
	ctx.lr = 0x82683D34;
	sub_82466E20(ctx, base);
	// 82683D34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683D48 size=112
    let mut pc: u32 = 0x82683D48;
    'dispatch: loop {
        match pc {
            0x82683D48 => {
    //   block [0x82683D48..0x82683DB8)
	// 82683D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683D54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683D58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683D5C: 38AA9E28  addi r5, r10, -0x61d8
	ctx.r[5].s64 = ctx.r[10].s64 + -25048;
	// 82683D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683D64: 390BDBE0  addi r8, r11, -0x2420
	ctx.r[8].s64 = ctx.r[11].s64 + -9248;
	// 82683D68: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82683D6C: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82683D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683D74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683D80: 386A9E58  addi r3, r10, -0x61a8
	ctx.r[3].s64 = ctx.r[10].s64 + -25000;
	// 82683D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683DA4: 4BDE307D  bl 0x82466e20
	ctx.lr = 0x82683DA8;
	sub_82466E20(ctx, base);
	// 82683DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683DB8 size=100
    let mut pc: u32 = 0x82683DB8;
    'dispatch: loop {
        match pc {
            0x82683DB8 => {
    //   block [0x82683DB8..0x82683E1C)
	// 82683DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683DC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683DCC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683DD8: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 82683DDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683DEC: 386A9E88  addi r3, r10, -0x6178
	ctx.r[3].s64 = ctx.r[10].s64 + -24952;
	// 82683DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683DF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683DF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82683DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683E00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82683E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683E08: 4BDE3019  bl 0x82466e20
	ctx.lr = 0x82683E0C;
	sub_82466E20(ctx, base);
	// 82683E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683E20 size=108
    let mut pc: u32 = 0x82683E20;
    'dispatch: loop {
        match pc {
            0x82683E20 => {
    //   block [0x82683E20..0x82683E8C)
	// 82683E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683E2C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683E30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683E34: 38EBDC58  addi r7, r11, -0x23a8
	ctx.r[7].s64 = ctx.r[11].s64 + -9128;
	// 82683E38: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82683E3C: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82683E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683E44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683E48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82683E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683E50: 386A9EB8  addi r3, r10, -0x6148
	ctx.r[3].s64 = ctx.r[10].s64 + -24904;
	// 82683E54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82683E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82683E78: 4BDE2FA9  bl 0x82466e20
	ctx.lr = 0x82683E7C;
	sub_82466E20(ctx, base);
	// 82683E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683E90 size=112
    let mut pc: u32 = 0x82683E90;
    'dispatch: loop {
        match pc {
            0x82683E90 => {
    //   block [0x82683E90..0x82683F00)
	// 82683E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683E9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683EA0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683EA4: 38AA9E88  addi r5, r10, -0x6178
	ctx.r[5].s64 = ctx.r[10].s64 + -24952;
	// 82683EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683EAC: 390BDCA0  addi r8, r11, -0x2360
	ctx.r[8].s64 = ctx.r[11].s64 + -9056;
	// 82683EB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82683EB4: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82683EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683EBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683EC8: 386A9EE8  addi r3, r10, -0x6118
	ctx.r[3].s64 = ctx.r[10].s64 + -24856;
	// 82683ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683EEC: 4BDE2F35  bl 0x82466e20
	ctx.lr = 0x82683EF0;
	sub_82466E20(ctx, base);
	// 82683EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683F00 size=100
    let mut pc: u32 = 0x82683F00;
    'dispatch: loop {
        match pc {
            0x82683F00 => {
    //   block [0x82683F00..0x82683F64)
	// 82683F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683F0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683F14: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683F20: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 82683F24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683F34: 386A9F18  addi r3, r10, -0x60e8
	ctx.r[3].s64 = ctx.r[10].s64 + -24808;
	// 82683F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683F3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683F40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82683F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683F48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82683F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683F50: 4BDE2ED1  bl 0x82466e20
	ctx.lr = 0x82683F54;
	sub_82466E20(ctx, base);
	// 82683F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683F68 size=100
    let mut pc: u32 = 0x82683F68;
    'dispatch: loop {
        match pc {
            0x82683F68 => {
    //   block [0x82683F68..0x82683FCC)
	// 82683F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683F74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683F7C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683F88: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82683F8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683F9C: 386A9F48  addi r3, r10, -0x60b8
	ctx.r[3].s64 = ctx.r[10].s64 + -24760;
	// 82683FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683FA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683FA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82683FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683FB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82683FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683FB8: 4BDE2E69  bl 0x82466e20
	ctx.lr = 0x82683FBC;
	sub_82466E20(ctx, base);
	// 82683FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683FD0 size=112
    let mut pc: u32 = 0x82683FD0;
    'dispatch: loop {
        match pc {
            0x82683FD0 => {
    //   block [0x82683FD0..0x82684040)
	// 82683FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683FDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683FE0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683FE4: 38AA9F18  addi r5, r10, -0x60e8
	ctx.r[5].s64 = ctx.r[10].s64 + -24808;
	// 82683FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683FEC: 390BDCD0  addi r8, r11, -0x2330
	ctx.r[8].s64 = ctx.r[11].s64 + -9008;
	// 82683FF0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82683FF4: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 82683FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683FFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684000: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684008: 386A9F78  addi r3, r10, -0x6088
	ctx.r[3].s64 = ctx.r[10].s64 + -24712;
	// 8268400C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268401C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268402C: 4BDE2DF5  bl 0x82466e20
	ctx.lr = 0x82684030;
	sub_82466E20(ctx, base);
	// 82684030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268403C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684040 size=112
    let mut pc: u32 = 0x82684040;
    'dispatch: loop {
        match pc {
            0x82684040 => {
    //   block [0x82684040..0x826840B0)
	// 82684040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268404C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684050: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684054: 38AA9F48  addi r5, r10, -0x60b8
	ctx.r[5].s64 = ctx.r[10].s64 + -24760;
	// 82684058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268405C: 390BDD30  addi r8, r11, -0x22d0
	ctx.r[8].s64 = ctx.r[11].s64 + -8912;
	// 82684060: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82684064: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82684068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268406C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684078: 386A9FA8  addi r3, r10, -0x6058
	ctx.r[3].s64 = ctx.r[10].s64 + -24664;
	// 8268407C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268408C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268409C: 4BDE2D85  bl 0x82466e20
	ctx.lr = 0x826840A0;
	sub_82466E20(ctx, base);
	// 826840A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826840A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826840A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826840AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826840B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826840B0 size=100
    let mut pc: u32 = 0x826840B0;
    'dispatch: loop {
        match pc {
            0x826840B0 => {
    //   block [0x826840B0..0x82684114)
	// 826840B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826840B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826840B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826840BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826840C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826840C4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826840C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826840CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826840D0: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826840D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826840D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826840DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826840E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826840E4: 386A9FD8  addi r3, r10, -0x6028
	ctx.r[3].s64 = ctx.r[10].s64 + -24616;
	// 826840E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826840EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826840F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826840F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826840F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826840FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684100: 4BDE2D21  bl 0x82466e20
	ctx.lr = 0x82684104;
	sub_82466E20(ctx, base);
	// 82684104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268410C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684118 size=112
    let mut pc: u32 = 0x82684118;
    'dispatch: loop {
        match pc {
            0x82684118 => {
    //   block [0x82684118..0x82684188)
	// 82684118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268411C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684124: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684128: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268412C: 38AA9FD8  addi r5, r10, -0x6028
	ctx.r[5].s64 = ctx.r[10].s64 + -24616;
	// 82684130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684134: 390BDD90  addi r8, r11, -0x2270
	ctx.r[8].s64 = ctx.r[11].s64 + -8816;
	// 82684138: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8268413C: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82684140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684144: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268414C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684150: 386AA008  addi r3, r10, -0x5ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -24568;
	// 82684154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268415C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268416C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684174: 4BDE2CAD  bl 0x82466e20
	ctx.lr = 0x82684178;
	sub_82466E20(ctx, base);
	// 82684178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268417C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684188 size=108
    let mut pc: u32 = 0x82684188;
    'dispatch: loop {
        match pc {
            0x82684188 => {
    //   block [0x82684188..0x826841F4)
	// 82684188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268418C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684194: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268419C: 38EBDE80  addi r7, r11, -0x2180
	ctx.r[7].s64 = ctx.r[11].s64 + -8576;
	// 826841A0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826841A4: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826841A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826841AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826841B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826841B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826841B8: 386AA038  addi r3, r10, -0x5fc8
	ctx.r[3].s64 = ctx.r[10].s64 + -24520;
	// 826841BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826841C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826841C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826841C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826841CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826841D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826841D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826841D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826841DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826841E0: 4BDE2C41  bl 0x82466e20
	ctx.lr = 0x826841E4;
	sub_82466E20(ctx, base);
	// 826841E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826841E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826841EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826841F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826841F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826841F8 size=108
    let mut pc: u32 = 0x826841F8;
    'dispatch: loop {
        match pc {
            0x826841F8 => {
    //   block [0x826841F8..0x82684264)
	// 826841F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826841FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684204: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268420C: 38EBDF70  addi r7, r11, -0x2090
	ctx.r[7].s64 = ctx.r[11].s64 + -8336;
	// 82684210: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82684214: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 82684218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268421C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684220: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684228: 386AA068  addi r3, r10, -0x5f98
	ctx.r[3].s64 = ctx.r[10].s64 + -24472;
	// 8268422C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268423C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268424C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684250: 4BDE2BD1  bl 0x82466e20
	ctx.lr = 0x82684254;
	sub_82466E20(ctx, base);
	// 82684254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268425C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684268 size=108
    let mut pc: u32 = 0x82684268;
    'dispatch: loop {
        match pc {
            0x82684268 => {
    //   block [0x82684268..0x826842D4)
	// 82684268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268426C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684274: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268427C: 38EBDFB8  addi r7, r11, -0x2048
	ctx.r[7].s64 = ctx.r[11].s64 + -8264;
	// 82684280: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82684284: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82684288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268428C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684298: 386AA098  addi r3, r10, -0x5f68
	ctx.r[3].s64 = ctx.r[10].s64 + -24424;
	// 8268429C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826842A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826842A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826842A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826842AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826842B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826842B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826842B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826842BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826842C0: 4BDE2B61  bl 0x82466e20
	ctx.lr = 0x826842C4;
	sub_82466E20(ctx, base);
	// 826842C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826842C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826842CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826842D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826842D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826842D8 size=108
    let mut pc: u32 = 0x826842D8;
    'dispatch: loop {
        match pc {
            0x826842D8 => {
    //   block [0x826842D8..0x82684344)
	// 826842D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826842DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826842E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826842E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826842E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826842EC: 38EBE090  addi r7, r11, -0x1f70
	ctx.r[7].s64 = ctx.r[11].s64 + -8048;
	// 826842F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826842F4: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826842F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826842FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684308: 386AA0C8  addi r3, r10, -0x5f38
	ctx.r[3].s64 = ctx.r[10].s64 + -24376;
	// 8268430C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268431C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268432C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684330: 4BDE2AF1  bl 0x82466e20
	ctx.lr = 0x82684334;
	sub_82466E20(ctx, base);
	// 82684334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268433C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684348 size=100
    let mut pc: u32 = 0x82684348;
    'dispatch: loop {
        match pc {
            0x82684348 => {
    //   block [0x82684348..0x826843AC)
	// 82684348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268434C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684354: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82684358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268435C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82684360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684368: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8268436C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268437C: 386AA0F8  addi r3, r10, -0x5f08
	ctx.r[3].s64 = ctx.r[10].s64 + -24328;
	// 82684380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684384: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684388: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268438C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82684394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684398: 4BDE2A89  bl 0x82466e20
	ctx.lr = 0x8268439C;
	sub_82466E20(ctx, base);
	// 8268439C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826843A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826843A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826843A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826843B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826843B0 size=112
    let mut pc: u32 = 0x826843B0;
    'dispatch: loop {
        match pc {
            0x826843B0 => {
    //   block [0x826843B0..0x82684420)
	// 826843B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826843B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826843B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826843BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826843C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826843C4: 38AAA0F8  addi r5, r10, -0x5f08
	ctx.r[5].s64 = ctx.r[10].s64 + -24328;
	// 826843C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826843CC: 390BE0A8  addi r8, r11, -0x1f58
	ctx.r[8].s64 = ctx.r[11].s64 + -8024;
	// 826843D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826843D4: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826843D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826843DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826843E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826843E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826843E8: 386AA128  addi r3, r10, -0x5ed8
	ctx.r[3].s64 = ctx.r[10].s64 + -24280;
	// 826843EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826843F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826843F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826843F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826843FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268440C: 4BDE2A15  bl 0x82466e20
	ctx.lr = 0x82684410;
	sub_82466E20(ctx, base);
	// 82684410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268441C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684420 size=108
    let mut pc: u32 = 0x82684420;
    'dispatch: loop {
        match pc {
            0x82684420 => {
    //   block [0x82684420..0x8268448C)
	// 82684420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268442C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684434: 38EBE0F0  addi r7, r11, -0x1f10
	ctx.r[7].s64 = ctx.r[11].s64 + -7952;
	// 82684438: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268443C: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82684440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268444C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684450: 386AA158  addi r3, r10, -0x5ea8
	ctx.r[3].s64 = ctx.r[10].s64 + -24232;
	// 82684454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268445C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268446C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684478: 4BDE29A9  bl 0x82466e20
	ctx.lr = 0x8268447C;
	sub_82466E20(ctx, base);
	// 8268447C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684490 size=112
    let mut pc: u32 = 0x82684490;
    'dispatch: loop {
        match pc {
            0x82684490 => {
    //   block [0x82684490..0x82684500)
	// 82684490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268449C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826844A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826844A4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826844A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826844AC: 390BE138  addi r8, r11, -0x1ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -7880;
	// 826844B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826844B4: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826844B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826844BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826844C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826844C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826844C8: 386AA188  addi r3, r10, -0x5e78
	ctx.r[3].s64 = ctx.r[10].s64 + -24184;
	// 826844CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826844D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826844D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826844D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826844DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826844E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826844E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826844E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826844EC: 4BDE2935  bl 0x82466e20
	ctx.lr = 0x826844F0;
	sub_82466E20(ctx, base);
	// 826844F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826844F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826844F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826844FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684500 size=108
    let mut pc: u32 = 0x82684500;
    'dispatch: loop {
        match pc {
            0x82684500 => {
    //   block [0x82684500..0x8268456C)
	// 82684500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268450C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684514: 38EBE150  addi r7, r11, -0x1eb0
	ctx.r[7].s64 = ctx.r[11].s64 + -7856;
	// 82684518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268451C: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82684520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684524: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268452C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684530: 386AA1B8  addi r3, r10, -0x5e48
	ctx.r[3].s64 = ctx.r[10].s64 + -24136;
	// 82684534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268453C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268454C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684558: 4BDE28C9  bl 0x82466e20
	ctx.lr = 0x8268455C;
	sub_82466E20(ctx, base);
	// 8268455C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684570 size=112
    let mut pc: u32 = 0x82684570;
    'dispatch: loop {
        match pc {
            0x82684570 => {
    //   block [0x82684570..0x826845E0)
	// 82684570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268457C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684580: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684584: 38AAA188  addi r5, r10, -0x5e78
	ctx.r[5].s64 = ctx.r[10].s64 + -24184;
	// 82684588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268458C: 390BE198  addi r8, r11, -0x1e68
	ctx.r[8].s64 = ctx.r[11].s64 + -7784;
	// 82684590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82684594: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82684598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268459C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826845A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826845A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826845A8: 386AA1E8  addi r3, r10, -0x5e18
	ctx.r[3].s64 = ctx.r[10].s64 + -24088;
	// 826845AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826845B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826845B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826845B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826845BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826845C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826845C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826845C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826845CC: 4BDE2855  bl 0x82466e20
	ctx.lr = 0x826845D0;
	sub_82466E20(ctx, base);
	// 826845D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826845D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826845D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826845DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826845E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826845E0 size=100
    let mut pc: u32 = 0x826845E0;
    'dispatch: loop {
        match pc {
            0x826845E0 => {
    //   block [0x826845E0..0x82684644)
	// 826845E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826845E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826845E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826845EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826845F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826845F4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826845F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826845FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684600: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 82684604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268460C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684614: 386AA218  addi r3, r10, -0x5de8
	ctx.r[3].s64 = ctx.r[10].s64 + -24040;
	// 82684618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268461C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684620: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82684624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684628: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268462C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684630: 4BDE27F1  bl 0x82466e20
	ctx.lr = 0x82684634;
	sub_82466E20(ctx, base);
	// 82684634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268463C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684648 size=112
    let mut pc: u32 = 0x82684648;
    'dispatch: loop {
        match pc {
            0x82684648 => {
    //   block [0x82684648..0x826846B8)
	// 82684648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268464C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684654: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684658: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268465C: 38AAA218  addi r5, r10, -0x5de8
	ctx.r[5].s64 = ctx.r[10].s64 + -24040;
	// 82684660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684664: 390BE1B0  addi r8, r11, -0x1e50
	ctx.r[8].s64 = ctx.r[11].s64 + -7760;
	// 82684668: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8268466C: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82684670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684674: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268467C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684680: 386AA248  addi r3, r10, -0x5db8
	ctx.r[3].s64 = ctx.r[10].s64 + -23992;
	// 82684684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268468C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268469C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826846A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826846A4: 4BDE277D  bl 0x82466e20
	ctx.lr = 0x826846A8;
	sub_82466E20(ctx, base);
	// 826846A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826846AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826846B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826846B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826846B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826846B8 size=108
    let mut pc: u32 = 0x826846B8;
    'dispatch: loop {
        match pc {
            0x826846B8 => {
    //   block [0x826846B8..0x82684724)
	// 826846B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826846BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826846C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826846C4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826846C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826846CC: 38EBE258  addi r7, r11, -0x1da8
	ctx.r[7].s64 = ctx.r[11].s64 + -7592;
	// 826846D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826846D4: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826846D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826846DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826846E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826846E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826846E8: 386AA278  addi r3, r10, -0x5d88
	ctx.r[3].s64 = ctx.r[10].s64 + -23944;
	// 826846EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826846F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826846F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826846F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826846FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268470C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684710: 4BDE2711  bl 0x82466e20
	ctx.lr = 0x82684714;
	sub_82466E20(ctx, base);
	// 82684714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268471C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684728 size=112
    let mut pc: u32 = 0x82684728;
    'dispatch: loop {
        match pc {
            0x82684728 => {
    //   block [0x82684728..0x82684798)
	// 82684728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268472C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82684738: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268473C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82684740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684744: 390BE288  addi r8, r11, -0x1d78
	ctx.r[8].s64 = ctx.r[11].s64 + -7544;
	// 82684748: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268474C: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82684750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684754: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268475C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684760: 386AA2A8  addi r3, r10, -0x5d58
	ctx.r[3].s64 = ctx.r[10].s64 + -23896;
	// 82684764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268476C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268477C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684784: 4BDE269D  bl 0x82466e20
	ctx.lr = 0x82684788;
	sub_82466E20(ctx, base);
	// 82684788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268478C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684798 size=112
    let mut pc: u32 = 0x82684798;
    'dispatch: loop {
        match pc {
            0x82684798 => {
    //   block [0x82684798..0x82684808)
	// 82684798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268479C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826847A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826847A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826847A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826847AC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826847B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826847B4: 390BE2D0  addi r8, r11, -0x1d30
	ctx.r[8].s64 = ctx.r[11].s64 + -7472;
	// 826847B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826847BC: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826847C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826847C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826847C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826847CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826847D0: 386AA2D8  addi r3, r10, -0x5d28
	ctx.r[3].s64 = ctx.r[10].s64 + -23848;
	// 826847D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826847D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826847DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826847E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826847E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826847E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826847EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826847F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826847F4: 4BDE262D  bl 0x82466e20
	ctx.lr = 0x826847F8;
	sub_82466E20(ctx, base);
	// 826847F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826847FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684808 size=100
    let mut pc: u32 = 0x82684808;
    'dispatch: loop {
        match pc {
            0x82684808 => {
    //   block [0x82684808..0x8268486C)
	// 82684808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268480C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82684818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268481C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82684820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684828: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8268482C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268483C: 386AA308  addi r3, r10, -0x5cf8
	ctx.r[3].s64 = ctx.r[10].s64 + -23800;
	// 82684840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684844: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684848: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268484C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684850: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82684854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684858: 4BDE25C9  bl 0x82466e20
	ctx.lr = 0x8268485C;
	sub_82466E20(ctx, base);
	// 8268485C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684870 size=112
    let mut pc: u32 = 0x82684870;
    'dispatch: loop {
        match pc {
            0x82684870 => {
    //   block [0x82684870..0x826848E0)
	// 82684870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268487C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684880: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684884: 38AAA308  addi r5, r10, -0x5cf8
	ctx.r[5].s64 = ctx.r[10].s64 + -23800;
	// 82684888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268488C: 390BE318  addi r8, r11, -0x1ce8
	ctx.r[8].s64 = ctx.r[11].s64 + -7400;
	// 82684890: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82684894: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82684898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268489C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826848A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826848A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826848A8: 386AA338  addi r3, r10, -0x5cc8
	ctx.r[3].s64 = ctx.r[10].s64 + -23752;
	// 826848AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826848B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826848B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826848B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826848BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826848C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826848C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826848C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826848CC: 4BDE2555  bl 0x82466e20
	ctx.lr = 0x826848D0;
	sub_82466E20(ctx, base);
	// 826848D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826848D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826848D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826848DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826848E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826848E0 size=112
    let mut pc: u32 = 0x826848E0;
    'dispatch: loop {
        match pc {
            0x826848E0 => {
    //   block [0x826848E0..0x82684950)
	// 826848E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826848E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826848E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826848EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826848F0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826848F4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826848F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826848FC: 390BE360  addi r8, r11, -0x1ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -7328;
	// 82684900: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82684904: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 82684908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268490C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684918: 386AA368  addi r3, r10, -0x5c98
	ctx.r[3].s64 = ctx.r[10].s64 + -23704;
	// 8268491C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268492C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268493C: 4BDE24E5  bl 0x82466e20
	ctx.lr = 0x82684940;
	sub_82466E20(ctx, base);
	// 82684940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268494C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684950 size=112
    let mut pc: u32 = 0x82684950;
    'dispatch: loop {
        match pc {
            0x82684950 => {
    //   block [0x82684950..0x826849C0)
	// 82684950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268495C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82684960: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684964: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82684968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268496C: 390BE378  addi r8, r11, -0x1c88
	ctx.r[8].s64 = ctx.r[11].s64 + -7304;
	// 82684970: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82684974: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82684978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268497C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684988: 386AA398  addi r3, r10, -0x5c68
	ctx.r[3].s64 = ctx.r[10].s64 + -23656;
	// 8268498C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268499C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826849A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826849A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826849A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826849AC: 4BDE2475  bl 0x82466e20
	ctx.lr = 0x826849B0;
	sub_82466E20(ctx, base);
	// 826849B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826849B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826849B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826849BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826849C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826849C0 size=112
    let mut pc: u32 = 0x826849C0;
    'dispatch: loop {
        match pc {
            0x826849C0 => {
    //   block [0x826849C0..0x82684A30)
	// 826849C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826849C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826849C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826849CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826849D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826849D4: 38AAA368  addi r5, r10, -0x5c98
	ctx.r[5].s64 = ctx.r[10].s64 + -23704;
	// 826849D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826849DC: 390BE390  addi r8, r11, -0x1c70
	ctx.r[8].s64 = ctx.r[11].s64 + -7280;
	// 826849E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826849E4: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826849E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826849EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826849F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826849F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826849F8: 386AA3C8  addi r3, r10, -0x5c38
	ctx.r[3].s64 = ctx.r[10].s64 + -23608;
	// 826849FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684A1C: 4BDE2405  bl 0x82466e20
	ctx.lr = 0x82684A20;
	sub_82466E20(ctx, base);
	// 82684A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684A30 size=72
    let mut pc: u32 = 0x82684A30;
    'dispatch: loop {
        match pc {
            0x82684A30 => {
    //   block [0x82684A30..0x82684A78)
	// 82684A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684A3C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82684A40: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82684A44: 38CB3BC0  addi r6, r11, 0x3bc0
	ctx.r[6].s64 = ctx.r[11].s64 + 15296;
	// 82684A48: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82684A4C: 388B5198  addi r4, r11, 0x5198
	ctx.r[4].s64 = ctx.r[11].s64 + 20888;
	// 82684A50: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82684A54: 386BA3F8  addi r3, r11, -0x5c08
	ctx.r[3].s64 = ctx.r[11].s64 + -23560;
	// 82684A58: 4BDF7031  bl 0x8247ba88
	ctx.lr = 0x82684A5C;
	sub_8247BA88(ctx, base);
	// 82684A5C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82684A60: 386BCE40  addi r3, r11, -0x31c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12736;
	// 82684A64: 4BEAE0D5  bl 0x82532b38
	ctx.lr = 0x82684A68;
	sub_82532B38(ctx, base);
	// 82684A68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82684A6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684A70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684A78 size=108
    let mut pc: u32 = 0x82684A78;
    'dispatch: loop {
        match pc {
            0x82684A78 => {
    //   block [0x82684A78..0x82684AE4)
	// 82684A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684A84: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684A88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684A8C: 38EBFC88  addi r7, r11, -0x378
	ctx.r[7].s64 = ctx.r[11].s64 + -888;
	// 82684A90: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82684A94: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82684A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684A9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684AA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684AA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684AA8: 386AA410  addi r3, r10, -0x5bf0
	ctx.r[3].s64 = ctx.r[10].s64 + -23536;
	// 82684AAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684AB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684AD0: 4BDE2351  bl 0x82466e20
	ctx.lr = 0x82684AD4;
	sub_82466E20(ctx, base);
	// 82684AD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684AD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684ADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684AE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82684AE8 size=24
    let mut pc: u32 = 0x82684AE8;
    'dispatch: loop {
        match pc {
            0x82684AE8 => {
    //   block [0x82684AE8..0x82684B00)
	// 82684AE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684AEC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82684AF0: 394A4BF0  addi r10, r10, 0x4bf0
	ctx.r[10].s64 = ctx.r[10].s64 + 19440;
	// 82684AF4: 816BFD00  lwz r11, -0x300(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-768 as u32) ) } as u64;
	// 82684AF8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82684AFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684B00 size=112
    let mut pc: u32 = 0x82684B00;
    'dispatch: loop {
        match pc {
            0x82684B00 => {
    //   block [0x82684B00..0x82684B70)
	// 82684B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684B0C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82684B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684B14: 392B5A6C  addi r9, r11, 0x5a6c
	ctx.r[9].s64 = ctx.r[11].s64 + 23148;
	// 82684B18: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 82684B1C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82684B20: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684B24: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 82684B28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684B2C: 396B4BF0  addi r11, r11, 0x4bf0
	ctx.r[11].s64 = ctx.r[11].s64 + 19440;
	// 82684B30: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82684B34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684B38: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82684B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684B40: 386AA440  addi r3, r10, -0x5bc0
	ctx.r[3].s64 = ctx.r[10].s64 + -23488;
	// 82684B44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82684B48: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82684B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684B50: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82684B54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684B58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82684B5C: 4BDE22C5  bl 0x82466e20
	ctx.lr = 0x82684B60;
	sub_82466E20(ctx, base);
	// 82684B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684B70 size=108
    let mut pc: u32 = 0x82684B70;
    'dispatch: loop {
        match pc {
            0x82684B70 => {
    //   block [0x82684B70..0x82684BDC)
	// 82684B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684B7C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684B84: 38EBFD04  addi r7, r11, -0x2fc
	ctx.r[7].s64 = ctx.r[11].s64 + -764;
	// 82684B88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82684B8C: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82684B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684B94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684B98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684BA0: 386AA470  addi r3, r10, -0x5b90
	ctx.r[3].s64 = ctx.r[10].s64 + -23440;
	// 82684BA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684BC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684BC8: 4BDE2259  bl 0x82466e20
	ctx.lr = 0x82684BCC;
	sub_82466E20(ctx, base);
	// 82684BCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684BD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684BD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684BE0 size=108
    let mut pc: u32 = 0x82684BE0;
    'dispatch: loop {
        match pc {
            0x82684BE0 => {
    //   block [0x82684BE0..0x82684C4C)
	// 82684BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684BEC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684BF4: 38EBFD34  addi r7, r11, -0x2cc
	ctx.r[7].s64 = ctx.r[11].s64 + -716;
	// 82684BF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82684BFC: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 82684C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684C04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684C08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684C10: 386AA4A0  addi r3, r10, -0x5b60
	ctx.r[3].s64 = ctx.r[10].s64 + -23392;
	// 82684C14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684C38: 4BDE21E9  bl 0x82466e20
	ctx.lr = 0x82684C3C;
	sub_82466E20(ctx, base);
	// 82684C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684C50 size=112
    let mut pc: u32 = 0x82684C50;
    'dispatch: loop {
        match pc {
            0x82684C50 => {
    //   block [0x82684C50..0x82684CC0)
	// 82684C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684C5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684C60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684C64: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 82684C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684C6C: 390BFD68  addi r8, r11, -0x298
	ctx.r[8].s64 = ctx.r[11].s64 + -664;
	// 82684C70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82684C74: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 82684C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684C7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684C88: 386AA4D0  addi r3, r10, -0x5b30
	ctx.r[3].s64 = ctx.r[10].s64 + -23344;
	// 82684C8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684CAC: 4BDE2175  bl 0x82466e20
	ctx.lr = 0x82684CB0;
	sub_82466E20(ctx, base);
	// 82684CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684CC0 size=108
    let mut pc: u32 = 0x82684CC0;
    'dispatch: loop {
        match pc {
            0x82684CC0 => {
    //   block [0x82684CC0..0x82684D2C)
	// 82684CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684CCC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684CD4: 38EBFDC8  addi r7, r11, -0x238
	ctx.r[7].s64 = ctx.r[11].s64 + -568;
	// 82684CD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82684CDC: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 82684CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684CE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684CE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684CF0: 386AA500  addi r3, r10, -0x5b00
	ctx.r[3].s64 = ctx.r[10].s64 + -23296;
	// 82684CF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684D18: 4BDE2109  bl 0x82466e20
	ctx.lr = 0x82684D1C;
	sub_82466E20(ctx, base);
	// 82684D1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684D30 size=112
    let mut pc: u32 = 0x82684D30;
    'dispatch: loop {
        match pc {
            0x82684D30 => {
    //   block [0x82684D30..0x82684DA0)
	// 82684D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684D3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684D40: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684D44: 38AAA4D0  addi r5, r10, -0x5b30
	ctx.r[5].s64 = ctx.r[10].s64 + -23344;
	// 82684D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684D4C: 390BFE28  addi r8, r11, -0x1d8
	ctx.r[8].s64 = ctx.r[11].s64 + -472;
	// 82684D50: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82684D54: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82684D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684D5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684D60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684D68: 386AA530  addi r3, r10, -0x5ad0
	ctx.r[3].s64 = ctx.r[10].s64 + -23248;
	// 82684D6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684D74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684D8C: 4BDE2095  bl 0x82466e20
	ctx.lr = 0x82684D90;
	sub_82466E20(ctx, base);
	// 82684D90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684DA0 size=112
    let mut pc: u32 = 0x82684DA0;
    'dispatch: loop {
        match pc {
            0x82684DA0 => {
    //   block [0x82684DA0..0x82684E10)
	// 82684DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684DAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684DB0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684DB4: 38AAA4D0  addi r5, r10, -0x5b30
	ctx.r[5].s64 = ctx.r[10].s64 + -23344;
	// 82684DB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684DBC: 390BFEB8  addi r8, r11, -0x148
	ctx.r[8].s64 = ctx.r[11].s64 + -328;
	// 82684DC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82684DC4: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 82684DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684DCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684DD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684DD8: 386AA560  addi r3, r10, -0x5aa0
	ctx.r[3].s64 = ctx.r[10].s64 + -23200;
	// 82684DDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684DFC: 4BDE2025  bl 0x82466e20
	ctx.lr = 0x82684E00;
	sub_82466E20(ctx, base);
	// 82684E00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684E10 size=108
    let mut pc: u32 = 0x82684E10;
    'dispatch: loop {
        match pc {
            0x82684E10 => {
    //   block [0x82684E10..0x82684E7C)
	// 82684E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684E1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684E24: 38EBFED0  addi r7, r11, -0x130
	ctx.r[7].s64 = ctx.r[11].s64 + -304;
	// 82684E28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82684E2C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82684E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684E34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684E38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684E40: 386AA590  addi r3, r10, -0x5a70
	ctx.r[3].s64 = ctx.r[10].s64 + -23152;
	// 82684E44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684E68: 4BDE1FB9  bl 0x82466e20
	ctx.lr = 0x82684E6C;
	sub_82466E20(ctx, base);
	// 82684E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684E80 size=112
    let mut pc: u32 = 0x82684E80;
    'dispatch: loop {
        match pc {
            0x82684E80 => {
    //   block [0x82684E80..0x82684EF0)
	// 82684E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684E8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684E90: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684E94: 38AAA4D0  addi r5, r10, -0x5b30
	ctx.r[5].s64 = ctx.r[10].s64 + -23344;
	// 82684E98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684E9C: 390BFF30  addi r8, r11, -0xd0
	ctx.r[8].s64 = ctx.r[11].s64 + -208;
	// 82684EA0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82684EA4: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 82684EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684EAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82684EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684EB8: 386AA5C0  addi r3, r10, -0x5a40
	ctx.r[3].s64 = ctx.r[10].s64 + -23104;
	// 82684EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82684EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684EDC: 4BDE1F45  bl 0x82466e20
	ctx.lr = 0x82684EE0;
	sub_82466E20(ctx, base);
	// 82684EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684EF0 size=108
    let mut pc: u32 = 0x82684EF0;
    'dispatch: loop {
        match pc {
            0x82684EF0 => {
    //   block [0x82684EF0..0x82684F5C)
	// 82684EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684EFC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684F04: 38EBFFD8  addi r7, r11, -0x28
	ctx.r[7].s64 = ctx.r[11].s64 + -40;
	// 82684F08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82684F0C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 82684F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684F14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684F20: 386AA5F0  addi r3, r10, -0x5a10
	ctx.r[3].s64 = ctx.r[10].s64 + -23056;
	// 82684F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684F48: 4BDE1ED9  bl 0x82466e20
	ctx.lr = 0x82684F4C;
	sub_82466E20(ctx, base);
	// 82684F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684F60 size=108
    let mut pc: u32 = 0x82684F60;
    'dispatch: loop {
        match pc {
            0x82684F60 => {
    //   block [0x82684F60..0x82684FCC)
	// 82684F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684F6C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684F74: 38EBFFF0  addi r7, r11, -0x10
	ctx.r[7].s64 = ctx.r[11].s64 + -16;
	// 82684F78: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82684F7C: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82684F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82684F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82684F90: 386AA620  addi r3, r10, -0x59e0
	ctx.r[3].s64 = ctx.r[10].s64 + -23008;
	// 82684F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82684F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82684F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82684FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82684FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82684FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82684FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82684FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82684FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82684FB8: 4BDE1E69  bl 0x82466e20
	ctx.lr = 0x82684FBC;
	sub_82466E20(ctx, base);
	// 82684FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82684FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82684FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82684FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82684FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82684FD0 size=112
    let mut pc: u32 = 0x82684FD0;
    'dispatch: loop {
        match pc {
            0x82684FD0 => {
    //   block [0x82684FD0..0x82685040)
	// 82684FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82684FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82684FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82684FDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82684FE0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82684FE4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 82684FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82684FEC: 390B0050  addi r8, r11, 0x50
	ctx.r[8].s64 = ctx.r[11].s64 + 80;
	// 82684FF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82684FF4: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82684FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82684FFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685000: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82685004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685008: 386AA650  addi r3, r10, -0x59b0
	ctx.r[3].s64 = ctx.r[10].s64 + -22960;
	// 8268500C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82685010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268501C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268502C: 4BDE1DF5  bl 0x82466e20
	ctx.lr = 0x82685030;
	sub_82466E20(ctx, base);
	// 82685030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268503C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685040 size=108
    let mut pc: u32 = 0x82685040;
    'dispatch: loop {
        match pc {
            0x82685040 => {
    //   block [0x82685040..0x826850AC)
	// 82685040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268504C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685054: 38EB0068  addi r7, r11, 0x68
	ctx.r[7].s64 = ctx.r[11].s64 + 104;
	// 82685058: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268505C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82685060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685068: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268506C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685070: 386AA680  addi r3, r10, -0x5980
	ctx.r[3].s64 = ctx.r[10].s64 + -22912;
	// 82685074: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268507C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268508C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685098: 4BDE1D89  bl 0x82466e20
	ctx.lr = 0x8268509C;
	sub_82466E20(ctx, base);
	// 8268509C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826850A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826850A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826850A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826850B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826850B0 size=108
    let mut pc: u32 = 0x826850B0;
    'dispatch: loop {
        match pc {
            0x826850B0 => {
    //   block [0x826850B0..0x8268511C)
	// 826850B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826850B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826850B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826850BC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826850C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826850C4: 38EB00B0  addi r7, r11, 0xb0
	ctx.r[7].s64 = ctx.r[11].s64 + 176;
	// 826850C8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826850CC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826850D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826850D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826850D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826850DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826850E0: 386AA6B0  addi r3, r10, -0x5950
	ctx.r[3].s64 = ctx.r[10].s64 + -22864;
	// 826850E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826850E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826850EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826850F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826850F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826850F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826850FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685108: 4BDE1D19  bl 0x82466e20
	ctx.lr = 0x8268510C;
	sub_82466E20(ctx, base);
	// 8268510C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685120 size=108
    let mut pc: u32 = 0x82685120;
    'dispatch: loop {
        match pc {
            0x82685120 => {
    //   block [0x82685120..0x8268518C)
	// 82685120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268512C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685134: 38EB0140  addi r7, r11, 0x140
	ctx.r[7].s64 = ctx.r[11].s64 + 320;
	// 82685138: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8268513C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82685140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685144: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685148: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268514C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685150: 386AA6E0  addi r3, r10, -0x5920
	ctx.r[3].s64 = ctx.r[10].s64 + -22816;
	// 82685154: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268515C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268516C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685178: 4BDE1CA9  bl 0x82466e20
	ctx.lr = 0x8268517C;
	sub_82466E20(ctx, base);
	// 8268517C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685190 size=100
    let mut pc: u32 = 0x82685190;
    'dispatch: loop {
        match pc {
            0x82685190 => {
    //   block [0x82685190..0x826851F4)
	// 82685190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268519C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826851A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826851A4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 826851A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826851AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826851B0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826851B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826851B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826851BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826851C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826851C4: 386AA710  addi r3, r10, -0x58f0
	ctx.r[3].s64 = ctx.r[10].s64 + -22768;
	// 826851C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826851CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826851D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826851D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826851D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826851DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826851E0: 4BDE1C41  bl 0x82466e20
	ctx.lr = 0x826851E4;
	sub_82466E20(ctx, base);
	// 826851E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826851E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826851EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826851F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826851F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826851F8 size=112
    let mut pc: u32 = 0x826851F8;
    'dispatch: loop {
        match pc {
            0x826851F8 => {
    //   block [0x826851F8..0x82685268)
	// 826851F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826851FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685204: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685208: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268520C: 38AAA710  addi r5, r10, -0x58f0
	ctx.r[5].s64 = ctx.r[10].s64 + -22768;
	// 82685210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685214: 390B01D0  addi r8, r11, 0x1d0
	ctx.r[8].s64 = ctx.r[11].s64 + 464;
	// 82685218: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268521C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82685220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268522C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685230: 386AA740  addi r3, r10, -0x58c0
	ctx.r[3].s64 = ctx.r[10].s64 + -22720;
	// 82685234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82685238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268523C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268524C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685254: 4BDE1BCD  bl 0x82466e20
	ctx.lr = 0x82685258;
	sub_82466E20(ctx, base);
	// 82685258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268525C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685268 size=108
    let mut pc: u32 = 0x82685268;
    'dispatch: loop {
        match pc {
            0x82685268 => {
    //   block [0x82685268..0x826852D4)
	// 82685268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268526C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685274: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268527C: 38EB0230  addi r7, r11, 0x230
	ctx.r[7].s64 = ctx.r[11].s64 + 560;
	// 82685280: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685284: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82685288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268528C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685298: 386AA770  addi r3, r10, -0x5890
	ctx.r[3].s64 = ctx.r[10].s64 + -22672;
	// 8268529C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826852A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826852A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826852A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826852AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826852B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826852B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826852B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826852BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826852C0: 4BDE1B61  bl 0x82466e20
	ctx.lr = 0x826852C4;
	sub_82466E20(ctx, base);
	// 826852C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826852C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826852CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826852D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826852D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826852D8 size=108
    let mut pc: u32 = 0x826852D8;
    'dispatch: loop {
        match pc {
            0x826852D8 => {
    //   block [0x826852D8..0x82685344)
	// 826852D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826852DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826852E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826852E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826852E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826852EC: 38EB0260  addi r7, r11, 0x260
	ctx.r[7].s64 = ctx.r[11].s64 + 608;
	// 826852F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826852F4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826852F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826852FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685308: 386AA7A0  addi r3, r10, -0x5860
	ctx.r[3].s64 = ctx.r[10].s64 + -22624;
	// 8268530C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268531C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268532C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685330: 4BDE1AF1  bl 0x82466e20
	ctx.lr = 0x82685334;
	sub_82466E20(ctx, base);
	// 82685334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268533C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685348 size=108
    let mut pc: u32 = 0x82685348;
    'dispatch: loop {
        match pc {
            0x82685348 => {
    //   block [0x82685348..0x826853B4)
	// 82685348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268534C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685354: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268535C: 38EB02A8  addi r7, r11, 0x2a8
	ctx.r[7].s64 = ctx.r[11].s64 + 680;
	// 82685360: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82685364: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82685368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268536C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685378: 386AA7D0  addi r3, r10, -0x5830
	ctx.r[3].s64 = ctx.r[10].s64 + -22576;
	// 8268537C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268538C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268539C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826853A0: 4BDE1A81  bl 0x82466e20
	ctx.lr = 0x826853A4;
	sub_82466E20(ctx, base);
	// 826853A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826853A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826853AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826853B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826853B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826853B8 size=96
    let mut pc: u32 = 0x826853B8;
    'dispatch: loop {
        match pc {
            0x826853B8 => {
    //   block [0x826853B8..0x82685418)
	// 826853B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826853BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826853C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826853C4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826853C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826853CC: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826853D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826853D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826853D8: 386AA800  addi r3, r10, -0x5800
	ctx.r[3].s64 = ctx.r[10].s64 + -22528;
	// 826853DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826853E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826853E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826853E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826853EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826853F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826853F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826853F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826853FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685400: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82685404: 4BDE1A1D  bl 0x82466e20
	ctx.lr = 0x82685408;
	sub_82466E20(ctx, base);
	// 82685408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268540C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685418 size=112
    let mut pc: u32 = 0x82685418;
    'dispatch: loop {
        match pc {
            0x82685418 => {
    //   block [0x82685418..0x82685488)
	// 82685418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685424: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685428: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268542C: 38AAA800  addi r5, r10, -0x5800
	ctx.r[5].s64 = ctx.r[10].s64 + -22528;
	// 82685430: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82685434: 390B0308  addi r8, r11, 0x308
	ctx.r[8].s64 = ctx.r[11].s64 + 776;
	// 82685438: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268543C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82685440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268544C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685450: 386AA830  addi r3, r10, -0x57d0
	ctx.r[3].s64 = ctx.r[10].s64 + -22480;
	// 82685454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82685458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268545C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268546C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685474: 4BDE19AD  bl 0x82466e20
	ctx.lr = 0x82685478;
	sub_82466E20(ctx, base);
	// 82685478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268547C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685488 size=112
    let mut pc: u32 = 0x82685488;
    'dispatch: loop {
        match pc {
            0x82685488 => {
    //   block [0x82685488..0x826854F8)
	// 82685488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268548C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685494: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82685498: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268549C: 392A5A98  addi r9, r10, 0x5a98
	ctx.r[9].s64 = ctx.r[10].s64 + 23192;
	// 826854A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826854A4: 390B0338  addi r8, r11, 0x338
	ctx.r[8].s64 = ctx.r[11].s64 + 824;
	// 826854A8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826854AC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826854B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826854B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826854B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826854BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826854C0: 386AA860  addi r3, r10, -0x57a0
	ctx.r[3].s64 = ctx.r[10].s64 + -22432;
	// 826854C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826854C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826854CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826854D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826854D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826854D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826854DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826854E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826854E4: 4BDE193D  bl 0x82466e20
	ctx.lr = 0x826854E8;
	sub_82466E20(ctx, base);
	// 826854E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826854EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826854F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826854F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826854F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826854F8 size=108
    let mut pc: u32 = 0x826854F8;
    'dispatch: loop {
        match pc {
            0x826854F8 => {
    //   block [0x826854F8..0x82685564)
	// 826854F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826854FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685504: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685508: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268550C: 38EB03E0  addi r7, r11, 0x3e0
	ctx.r[7].s64 = ctx.r[11].s64 + 992;
	// 82685510: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685514: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82685518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268551C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685528: 386AA890  addi r3, r10, -0x5770
	ctx.r[3].s64 = ctx.r[10].s64 + -22384;
	// 8268552C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268553C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268554C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685550: 4BDE18D1  bl 0x82466e20
	ctx.lr = 0x82685554;
	sub_82466E20(ctx, base);
	// 82685554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268555C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685568 size=108
    let mut pc: u32 = 0x82685568;
    'dispatch: loop {
        match pc {
            0x82685568 => {
    //   block [0x82685568..0x826855D4)
	// 82685568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268556C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685574: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268557C: 38EB0410  addi r7, r11, 0x410
	ctx.r[7].s64 = ctx.r[11].s64 + 1040;
	// 82685580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685584: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82685588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268558C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685598: 386AA8C0  addi r3, r10, -0x5740
	ctx.r[3].s64 = ctx.r[10].s64 + -22336;
	// 8268559C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826855A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826855A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826855A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826855AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826855B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826855B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826855B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826855BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826855C0: 4BDE1861  bl 0x82466e20
	ctx.lr = 0x826855C4;
	sub_82466E20(ctx, base);
	// 826855C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826855C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826855CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826855D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826855D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826855D8 size=28
    let mut pc: u32 = 0x826855D8;
    'dispatch: loop {
        match pc {
            0x826855D8 => {
    //   block [0x826855D8..0x826855F4)
	// 826855D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826855DC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 826855E0: 394A4C38  addi r10, r10, 0x4c38
	ctx.r[10].s64 = ctx.r[10].s64 + 19512;
	// 826855E4: 816B0440  lwz r11, 0x440(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1088 as u32) ) } as u64;
	// 826855E8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826855EC: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826855F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826855F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826855F8 size=112
    let mut pc: u32 = 0x826855F8;
    'dispatch: loop {
        match pc {
            0x826855F8 => {
    //   block [0x826855F8..0x82685668)
	// 826855F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826855FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685604: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82685608: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268560C: 392A5C08  addi r9, r10, 0x5c08
	ctx.r[9].s64 = ctx.r[10].s64 + 23560;
	// 82685610: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82685614: 390B4C38  addi r8, r11, 0x4c38
	ctx.r[8].s64 = ctx.r[11].s64 + 19512;
	// 82685618: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8268561C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82685620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685624: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268562C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685630: 386AA8F0  addi r3, r10, -0x5710
	ctx.r[3].s64 = ctx.r[10].s64 + -22288;
	// 82685634: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82685638: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8268563C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268564C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685654: 4BDE17CD  bl 0x82466e20
	ctx.lr = 0x82685658;
	sub_82466E20(ctx, base);
	// 82685658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268565C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685668 size=108
    let mut pc: u32 = 0x82685668;
    'dispatch: loop {
        match pc {
            0x82685668 => {
    //   block [0x82685668..0x826856D4)
	// 82685668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268566C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685674: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685678: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268567C: 38EB044C  addi r7, r11, 0x44c
	ctx.r[7].s64 = ctx.r[11].s64 + 1100;
	// 82685680: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685684: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 82685688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268568C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685698: 386AA920  addi r3, r10, -0x56e0
	ctx.r[3].s64 = ctx.r[10].s64 + -22240;
	// 8268569C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826856A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826856A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826856A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826856AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826856B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826856B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826856B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826856BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826856C0: 4BDE1761  bl 0x82466e20
	ctx.lr = 0x826856C4;
	sub_82466E20(ctx, base);
	// 826856C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826856C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826856CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826856D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826856D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826856D8 size=108
    let mut pc: u32 = 0x826856D8;
    'dispatch: loop {
        match pc {
            0x826856D8 => {
    //   block [0x826856D8..0x82685744)
	// 826856D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826856DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826856E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826856E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826856E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826856EC: 38EB047C  addi r7, r11, 0x47c
	ctx.r[7].s64 = ctx.r[11].s64 + 1148;
	// 826856F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826856F4: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826856F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826856FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685708: 386AA950  addi r3, r10, -0x56b0
	ctx.r[3].s64 = ctx.r[10].s64 + -22192;
	// 8268570C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268571C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268572C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685730: 4BDE16F1  bl 0x82466e20
	ctx.lr = 0x82685734;
	sub_82466E20(ctx, base);
	// 82685734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268573C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82685748 size=24
    let mut pc: u32 = 0x82685748;
    'dispatch: loop {
        match pc {
            0x82685748 => {
    //   block [0x82685748..0x82685760)
	// 82685748: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268574C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82685750: 394A4CF8  addi r10, r10, 0x4cf8
	ctx.r[10].s64 = ctx.r[10].s64 + 19704;
	// 82685754: 816B0494  lwz r11, 0x494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1172 as u32) ) } as u64;
	// 82685758: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8268575C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685760 size=112
    let mut pc: u32 = 0x82685760;
    'dispatch: loop {
        match pc {
            0x82685760 => {
    //   block [0x82685760..0x826857D0)
	// 82685760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268576C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82685770: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685774: 392A5C5C  addi r9, r10, 0x5c5c
	ctx.r[9].s64 = ctx.r[10].s64 + 23644;
	// 82685778: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8268577C: 390B4CF8  addi r8, r11, 0x4cf8
	ctx.r[8].s64 = ctx.r[11].s64 + 19704;
	// 82685780: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82685784: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 82685788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268578C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82685794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685798: 386AA980  addi r3, r10, -0x5680
	ctx.r[3].s64 = ctx.r[10].s64 + -22144;
	// 8268579C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826857A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826857A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826857A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826857AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826857B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826857B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826857B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826857BC: 4BDE1665  bl 0x82466e20
	ctx.lr = 0x826857C0;
	sub_82466E20(ctx, base);
	// 826857C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826857C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826857C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826857CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826857D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826857D0 size=108
    let mut pc: u32 = 0x826857D0;
    'dispatch: loop {
        match pc {
            0x826857D0 => {
    //   block [0x826857D0..0x8268583C)
	// 826857D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826857D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826857D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826857DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826857E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826857E4: 38EB0498  addi r7, r11, 0x498
	ctx.r[7].s64 = ctx.r[11].s64 + 1176;
	// 826857E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826857EC: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826857F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826857F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826857F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826857FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685800: 386AA9B0  addi r3, r10, -0x5650
	ctx.r[3].s64 = ctx.r[10].s64 + -22096;
	// 82685804: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268580C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268581C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685828: 4BDE15F9  bl 0x82466e20
	ctx.lr = 0x8268582C;
	sub_82466E20(ctx, base);
	// 8268582C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685840 size=108
    let mut pc: u32 = 0x82685840;
    'dispatch: loop {
        match pc {
            0x82685840 => {
    //   block [0x82685840..0x826858AC)
	// 82685840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268584C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685850: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82685854: 38EB04C8  addi r7, r11, 0x4c8
	ctx.r[7].s64 = ctx.r[11].s64 + 1224;
	// 82685858: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268585C: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82685860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685864: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268586C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685870: 386AA9E0  addi r3, r10, -0x5620
	ctx.r[3].s64 = ctx.r[10].s64 + -22048;
	// 82685874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268587C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268588C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685898: 4BDE1589  bl 0x82466e20
	ctx.lr = 0x8268589C;
	sub_82466E20(ctx, base);
	// 8268589C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826858A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826858A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826858A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826858B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826858B0 size=112
    let mut pc: u32 = 0x826858B0;
    'dispatch: loop {
        match pc {
            0x826858B0 => {
    //   block [0x826858B0..0x82685920)
	// 826858B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826858B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826858B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826858BC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826858C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826858C4: 392A5C80  addi r9, r10, 0x5c80
	ctx.r[9].s64 = ctx.r[10].s64 + 23680;
	// 826858C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826858CC: 390B0500  addi r8, r11, 0x500
	ctx.r[8].s64 = ctx.r[11].s64 + 1280;
	// 826858D0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826858D4: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826858D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826858DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826858E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826858E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826858E8: 386AAA10  addi r3, r10, -0x55f0
	ctx.r[3].s64 = ctx.r[10].s64 + -22000;
	// 826858EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826858F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826858F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826858F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826858FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268590C: 4BDE1515  bl 0x82466e20
	ctx.lr = 0x82685910;
	sub_82466E20(ctx, base);
	// 82685910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268591C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685920 size=108
    let mut pc: u32 = 0x82685920;
    'dispatch: loop {
        match pc {
            0x82685920 => {
    //   block [0x82685920..0x8268598C)
	// 82685920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268592C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685930: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82685934: 38EB0560  addi r7, r11, 0x560
	ctx.r[7].s64 = ctx.r[11].s64 + 1376;
	// 82685938: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8268593C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82685940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685944: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268594C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685950: 386AAA40  addi r3, r10, -0x55c0
	ctx.r[3].s64 = ctx.r[10].s64 + -21952;
	// 82685954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268595C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268596C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685978: 4BDE14A9  bl 0x82466e20
	ctx.lr = 0x8268597C;
	sub_82466E20(ctx, base);
	// 8268597C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685990 size=108
    let mut pc: u32 = 0x82685990;
    'dispatch: loop {
        match pc {
            0x82685990 => {
    //   block [0x82685990..0x826859FC)
	// 82685990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268599C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826859A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826859A4: 38EB0620  addi r7, r11, 0x620
	ctx.r[7].s64 = ctx.r[11].s64 + 1568;
	// 826859A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826859AC: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826859B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826859B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826859B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826859BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826859C0: 386AAA70  addi r3, r10, -0x5590
	ctx.r[3].s64 = ctx.r[10].s64 + -21904;
	// 826859C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826859C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826859CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826859D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826859D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826859D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826859DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826859E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826859E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826859E8: 4BDE1439  bl 0x82466e20
	ctx.lr = 0x826859EC;
	sub_82466E20(ctx, base);
	// 826859EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826859F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826859F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826859F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685A00 size=108
    let mut pc: u32 = 0x82685A00;
    'dispatch: loop {
        match pc {
            0x82685A00 => {
    //   block [0x82685A00..0x82685A6C)
	// 82685A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685A0C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685A14: 38EB0638  addi r7, r11, 0x638
	ctx.r[7].s64 = ctx.r[11].s64 + 1592;
	// 82685A18: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82685A1C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82685A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685A24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685A30: 386AAAA0  addi r3, r10, -0x5560
	ctx.r[3].s64 = ctx.r[10].s64 + -21856;
	// 82685A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685A58: 4BDE13C9  bl 0x82466e20
	ctx.lr = 0x82685A5C;
	sub_82466E20(ctx, base);
	// 82685A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82685A70 size=24
    let mut pc: u32 = 0x82685A70;
    'dispatch: loop {
        match pc {
            0x82685A70 => {
    //   block [0x82685A70..0x82685A88)
	// 82685A70: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685A74: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82685A78: 394A4D88  addi r10, r10, 0x4d88
	ctx.r[10].s64 = ctx.r[10].s64 + 19848;
	// 82685A7C: 816B04FC  lwz r11, 0x4fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1276 as u32) ) } as u64;
	// 82685A80: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82685A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685A88 size=108
    let mut pc: u32 = 0x82685A88;
    'dispatch: loop {
        match pc {
            0x82685A88 => {
    //   block [0x82685A88..0x82685AF4)
	// 82685A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685A94: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685A9C: 38EB4D88  addi r7, r11, 0x4d88
	ctx.r[7].s64 = ctx.r[11].s64 + 19848;
	// 82685AA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685AA4: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82685AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685AAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685AB8: 386AAAD0  addi r3, r10, -0x5530
	ctx.r[3].s64 = ctx.r[10].s64 + -21808;
	// 82685ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685AE0: 4BDE1341  bl 0x82466e20
	ctx.lr = 0x82685AE4;
	sub_82466E20(ctx, base);
	// 82685AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82685AF8 size=24
    let mut pc: u32 = 0x82685AF8;
    'dispatch: loop {
        match pc {
            0x82685AF8 => {
    //   block [0x82685AF8..0x82685B10)
	// 82685AF8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685AFC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82685B00: 394A4DB8  addi r10, r10, 0x4db8
	ctx.r[10].s64 = ctx.r[10].s64 + 19896;
	// 82685B04: 816B04FC  lwz r11, 0x4fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1276 as u32) ) } as u64;
	// 82685B08: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82685B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685B10 size=108
    let mut pc: u32 = 0x82685B10;
    'dispatch: loop {
        match pc {
            0x82685B10 => {
    //   block [0x82685B10..0x82685B7C)
	// 82685B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685B1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685B24: 38EB4DB8  addi r7, r11, 0x4db8
	ctx.r[7].s64 = ctx.r[11].s64 + 19896;
	// 82685B28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685B2C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82685B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685B34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685B38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685B40: 386AAB00  addi r3, r10, -0x5500
	ctx.r[3].s64 = ctx.r[10].s64 + -21760;
	// 82685B44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685B68: 4BDE12B9  bl 0x82466e20
	ctx.lr = 0x82685B6C;
	sub_82466E20(ctx, base);
	// 82685B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685B80 size=108
    let mut pc: u32 = 0x82685B80;
    'dispatch: loop {
        match pc {
            0x82685B80 => {
    //   block [0x82685B80..0x82685BEC)
	// 82685B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685B8C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685B94: 38EB06B0  addi r7, r11, 0x6b0
	ctx.r[7].s64 = ctx.r[11].s64 + 1712;
	// 82685B98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82685B9C: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82685BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685BA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685BA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685BB0: 386AAB30  addi r3, r10, -0x54d0
	ctx.r[3].s64 = ctx.r[10].s64 + -21712;
	// 82685BB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685BD8: 4BDE1249  bl 0x82466e20
	ctx.lr = 0x82685BDC;
	sub_82466E20(ctx, base);
	// 82685BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82685BF0 size=24
    let mut pc: u32 = 0x82685BF0;
    'dispatch: loop {
        match pc {
            0x82685BF0 => {
    //   block [0x82685BF0..0x82685C08)
	// 82685BF0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685BF4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82685BF8: 394A4DE8  addi r10, r10, 0x4de8
	ctx.r[10].s64 = ctx.r[10].s64 + 19944;
	// 82685BFC: 816B04FC  lwz r11, 0x4fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1276 as u32) ) } as u64;
	// 82685C00: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82685C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685C08 size=108
    let mut pc: u32 = 0x82685C08;
    'dispatch: loop {
        match pc {
            0x82685C08 => {
    //   block [0x82685C08..0x82685C74)
	// 82685C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685C14: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685C1C: 38EB4DE8  addi r7, r11, 0x4de8
	ctx.r[7].s64 = ctx.r[11].s64 + 19944;
	// 82685C20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685C24: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82685C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685C2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685C30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685C38: 386AAB60  addi r3, r10, -0x54a0
	ctx.r[3].s64 = ctx.r[10].s64 + -21664;
	// 82685C3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685C60: 4BDE11C1  bl 0x82466e20
	ctx.lr = 0x82685C64;
	sub_82466E20(ctx, base);
	// 82685C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685C78 size=112
    let mut pc: u32 = 0x82685C78;
    'dispatch: loop {
        match pc {
            0x82685C78 => {
    //   block [0x82685C78..0x82685CE8)
	// 82685C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685C84: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82685C88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685C8C: 392A5CC4  addi r9, r10, 0x5cc4
	ctx.r[9].s64 = ctx.r[10].s64 + 23748;
	// 82685C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685C94: 390B06C8  addi r8, r11, 0x6c8
	ctx.r[8].s64 = ctx.r[11].s64 + 1736;
	// 82685C98: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82685C9C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82685CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685CA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82685CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685CB0: 386AAB90  addi r3, r10, -0x5470
	ctx.r[3].s64 = ctx.r[10].s64 + -21616;
	// 82685CB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82685CB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82685CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685CCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685CD4: 4BDE114D  bl 0x82466e20
	ctx.lr = 0x82685CD8;
	sub_82466E20(ctx, base);
	// 82685CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685CE8 size=108
    let mut pc: u32 = 0x82685CE8;
    'dispatch: loop {
        match pc {
            0x82685CE8 => {
    //   block [0x82685CE8..0x82685D54)
	// 82685CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685CF4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685CFC: 38EB06F8  addi r7, r11, 0x6f8
	ctx.r[7].s64 = ctx.r[11].s64 + 1784;
	// 82685D00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685D04: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82685D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685D0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685D10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685D18: 386AABC0  addi r3, r10, -0x5440
	ctx.r[3].s64 = ctx.r[10].s64 + -21568;
	// 82685D1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685D20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685D3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685D40: 4BDE10E1  bl 0x82466e20
	ctx.lr = 0x82685D44;
	sub_82466E20(ctx, base);
	// 82685D44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685D58 size=108
    let mut pc: u32 = 0x82685D58;
    'dispatch: loop {
        match pc {
            0x82685D58 => {
    //   block [0x82685D58..0x82685DC4)
	// 82685D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685D64: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685D6C: 38EB0728  addi r7, r11, 0x728
	ctx.r[7].s64 = ctx.r[11].s64 + 1832;
	// 82685D70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685D74: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 82685D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685D7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685D88: 386AABF0  addi r3, r10, -0x5410
	ctx.r[3].s64 = ctx.r[10].s64 + -21520;
	// 82685D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685DB0: 4BDE1071  bl 0x82466e20
	ctx.lr = 0x82685DB4;
	sub_82466E20(ctx, base);
	// 82685DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685DC8 size=112
    let mut pc: u32 = 0x82685DC8;
    'dispatch: loop {
        match pc {
            0x82685DC8 => {
    //   block [0x82685DC8..0x82685E38)
	// 82685DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685DD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685DD8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685DDC: 38AAAC50  addi r5, r10, -0x53b0
	ctx.r[5].s64 = ctx.r[10].s64 + -21424;
	// 82685DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685DE4: 390B0758  addi r8, r11, 0x758
	ctx.r[8].s64 = ctx.r[11].s64 + 1880;
	// 82685DE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82685DEC: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82685DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685DF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82685DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685E00: 386AAC20  addi r3, r10, -0x53e0
	ctx.r[3].s64 = ctx.r[10].s64 + -21472;
	// 82685E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82685E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685E24: 4BDE0FFD  bl 0x82466e20
	ctx.lr = 0x82685E28;
	sub_82466E20(ctx, base);
	// 82685E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685E38 size=108
    let mut pc: u32 = 0x82685E38;
    'dispatch: loop {
        match pc {
            0x82685E38 => {
    //   block [0x82685E38..0x82685EA4)
	// 82685E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685E44: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685E4C: 38EB0770  addi r7, r11, 0x770
	ctx.r[7].s64 = ctx.r[11].s64 + 1904;
	// 82685E50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685E54: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82685E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685E5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685E60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685E68: 386AAC50  addi r3, r10, -0x53b0
	ctx.r[3].s64 = ctx.r[10].s64 + -21424;
	// 82685E6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685E90: 4BDE0F91  bl 0x82466e20
	ctx.lr = 0x82685E94;
	sub_82466E20(ctx, base);
	// 82685E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685EA8 size=108
    let mut pc: u32 = 0x82685EA8;
    'dispatch: loop {
        match pc {
            0x82685EA8 => {
    //   block [0x82685EA8..0x82685F14)
	// 82685EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685EB4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685EBC: 38EB07A0  addi r7, r11, 0x7a0
	ctx.r[7].s64 = ctx.r[11].s64 + 1952;
	// 82685EC0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82685EC4: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82685EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685ECC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685ED8: 386AAC80  addi r3, r10, -0x5380
	ctx.r[3].s64 = ctx.r[10].s64 + -21376;
	// 82685EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685F00: 4BDE0F21  bl 0x82466e20
	ctx.lr = 0x82685F04;
	sub_82466E20(ctx, base);
	// 82685F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685F18 size=108
    let mut pc: u32 = 0x82685F18;
    'dispatch: loop {
        match pc {
            0x82685F18 => {
    //   block [0x82685F18..0x82685F84)
	// 82685F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685F24: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685F2C: 38EB07B8  addi r7, r11, 0x7b8
	ctx.r[7].s64 = ctx.r[11].s64 + 1976;
	// 82685F30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82685F34: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82685F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685F3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685F48: 386AACB0  addi r3, r10, -0x5350
	ctx.r[3].s64 = ctx.r[10].s64 + -21328;
	// 82685F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685F70: 4BDE0EB1  bl 0x82466e20
	ctx.lr = 0x82685F74;
	sub_82466E20(ctx, base);
	// 82685F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685F88 size=108
    let mut pc: u32 = 0x82685F88;
    'dispatch: loop {
        match pc {
            0x82685F88 => {
    //   block [0x82685F88..0x82685FF4)
	// 82685F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82685F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82685F94: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82685F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82685F9C: 38EB07E8  addi r7, r11, 0x7e8
	ctx.r[7].s64 = ctx.r[11].s64 + 2024;
	// 82685FA0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82685FA4: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82685FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82685FAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82685FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82685FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82685FB8: 386AACE0  addi r3, r10, -0x5320
	ctx.r[3].s64 = ctx.r[10].s64 + -21280;
	// 82685FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82685FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82685FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82685FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82685FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82685FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82685FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82685FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82685FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82685FE0: 4BDE0E41  bl 0x82466e20
	ctx.lr = 0x82685FE4;
	sub_82466E20(ctx, base);
	// 82685FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82685FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82685FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82685FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82685FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82685FF8 size=108
    let mut pc: u32 = 0x82685FF8;
    'dispatch: loop {
        match pc {
            0x82685FF8 => {
    //   block [0x82685FF8..0x82686064)
	// 82685FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82685FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686004: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268600C: 38EB0890  addi r7, r11, 0x890
	ctx.r[7].s64 = ctx.r[11].s64 + 2192;
	// 82686010: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82686014: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82686018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268601C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686028: 386AAD10  addi r3, r10, -0x52f0
	ctx.r[3].s64 = ctx.r[10].s64 + -21232;
	// 8268602C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268603C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268604C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686050: 4BDE0DD1  bl 0x82466e20
	ctx.lr = 0x82686054;
	sub_82466E20(ctx, base);
	// 82686054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268605C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686068 size=108
    let mut pc: u32 = 0x82686068;
    'dispatch: loop {
        match pc {
            0x82686068 => {
    //   block [0x82686068..0x826860D4)
	// 82686068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268606C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686074: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268607C: 38EB08C0  addi r7, r11, 0x8c0
	ctx.r[7].s64 = ctx.r[11].s64 + 2240;
	// 82686080: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82686084: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82686088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268608C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686098: 386AAD40  addi r3, r10, -0x52c0
	ctx.r[3].s64 = ctx.r[10].s64 + -21184;
	// 8268609C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826860A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826860A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826860A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826860AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826860B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826860B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826860B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826860BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826860C0: 4BDE0D61  bl 0x82466e20
	ctx.lr = 0x826860C4;
	sub_82466E20(ctx, base);
	// 826860C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826860C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826860CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826860D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826860D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826860D8 size=24
    let mut pc: u32 = 0x826860D8;
    'dispatch: loop {
        match pc {
            0x826860D8 => {
    //   block [0x826860D8..0x826860F0)
	// 826860D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826860DC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 826860E0: 394A4E18  addi r10, r10, 0x4e18
	ctx.r[10].s64 = ctx.r[10].s64 + 19992;
	// 826860E4: 816B0980  lwz r11, 0x980(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2432 as u32) ) } as u64;
	// 826860E8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826860EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826860F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826860F0 size=112
    let mut pc: u32 = 0x826860F0;
    'dispatch: loop {
        match pc {
            0x826860F0 => {
    //   block [0x826860F0..0x82686160)
	// 826860F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826860F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826860F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826860FC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82686100: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686104: 392A5CF0  addi r9, r10, 0x5cf0
	ctx.r[9].s64 = ctx.r[10].s64 + 23792;
	// 82686108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268610C: 390B4E18  addi r8, r11, 0x4e18
	ctx.r[8].s64 = ctx.r[11].s64 + 19992;
	// 82686110: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82686114: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82686118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268611C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82686124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686128: 386AAD70  addi r3, r10, -0x5290
	ctx.r[3].s64 = ctx.r[10].s64 + -21136;
	// 8268612C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82686130: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82686134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268613C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268614C: 4BDE0CD5  bl 0x82466e20
	ctx.lr = 0x82686150;
	sub_82466E20(ctx, base);
	// 82686150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268615C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686160 size=108
    let mut pc: u32 = 0x82686160;
    'dispatch: loop {
        match pc {
            0x82686160 => {
    //   block [0x82686160..0x826861CC)
	// 82686160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268616C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686174: 38EB0988  addi r7, r11, 0x988
	ctx.r[7].s64 = ctx.r[11].s64 + 2440;
	// 82686178: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268617C: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82686180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686184: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268618C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686190: 386AADA0  addi r3, r10, -0x5260
	ctx.r[3].s64 = ctx.r[10].s64 + -21088;
	// 82686194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268619C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826861A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826861A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826861A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826861AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826861B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826861B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826861B8: 4BDE0C69  bl 0x82466e20
	ctx.lr = 0x826861BC;
	sub_82466E20(ctx, base);
	// 826861BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826861C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826861C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826861C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826861D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826861D0 size=112
    let mut pc: u32 = 0x826861D0;
    'dispatch: loop {
        match pc {
            0x826861D0 => {
    //   block [0x826861D0..0x82686240)
	// 826861D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826861D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826861D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826861DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826861E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826861E4: 392A5D34  addi r9, r10, 0x5d34
	ctx.r[9].s64 = ctx.r[10].s64 + 23860;
	// 826861E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826861EC: 390B09B8  addi r8, r11, 0x9b8
	ctx.r[8].s64 = ctx.r[11].s64 + 2488;
	// 826861F0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826861F4: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826861F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826861FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82686204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686208: 386AADD0  addi r3, r10, -0x5230
	ctx.r[3].s64 = ctx.r[10].s64 + -21040;
	// 8268620C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82686210: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82686214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268621C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268622C: 4BDE0BF5  bl 0x82466e20
	ctx.lr = 0x82686230;
	sub_82466E20(ctx, base);
	// 82686230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268623C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82686240 size=24
    let mut pc: u32 = 0x82686240;
    'dispatch: loop {
        match pc {
            0x82686240 => {
    //   block [0x82686240..0x82686258)
	// 82686240: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686244: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82686248: 394A4E90  addi r10, r10, 0x4e90
	ctx.r[10].s64 = ctx.r[10].s64 + 20112;
	// 8268624C: 816B0A78  lwz r11, 0xa78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2680 as u32) ) } as u64;
	// 82686250: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82686254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686258 size=112
    let mut pc: u32 = 0x82686258;
    'dispatch: loop {
        match pc {
            0x82686258 => {
    //   block [0x82686258..0x826862C8)
	// 82686258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268625C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686264: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82686268: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268626C: 392A5D70  addi r9, r10, 0x5d70
	ctx.r[9].s64 = ctx.r[10].s64 + 23920;
	// 82686270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686274: 390B4E90  addi r8, r11, 0x4e90
	ctx.r[8].s64 = ctx.r[11].s64 + 20112;
	// 82686278: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8268627C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82686280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686284: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268628C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686290: 386AAE00  addi r3, r10, -0x5200
	ctx.r[3].s64 = ctx.r[10].s64 + -20992;
	// 82686294: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82686298: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268629C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826862A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826862A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826862A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826862AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826862B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826862B4: 4BDE0B6D  bl 0x82466e20
	ctx.lr = 0x826862B8;
	sub_82466E20(ctx, base);
	// 826862B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826862BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826862C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826862C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826862C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826862C8 size=108
    let mut pc: u32 = 0x826862C8;
    'dispatch: loop {
        match pc {
            0x826862C8 => {
    //   block [0x826862C8..0x82686334)
	// 826862C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826862CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826862D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826862D4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826862D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826862DC: 38EB0A7C  addi r7, r11, 0xa7c
	ctx.r[7].s64 = ctx.r[11].s64 + 2684;
	// 826862E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826862E4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826862E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826862EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826862F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826862F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826862F8: 386AAE30  addi r3, r10, -0x51d0
	ctx.r[3].s64 = ctx.r[10].s64 + -20944;
	// 826862FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268630C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268631C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686320: 4BDE0B01  bl 0x82466e20
	ctx.lr = 0x82686324;
	sub_82466E20(ctx, base);
	// 82686324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268632C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686338 size=108
    let mut pc: u32 = 0x82686338;
    'dispatch: loop {
        match pc {
            0x82686338 => {
    //   block [0x82686338..0x826863A4)
	// 82686338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268633C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686344: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268634C: 38EB0A94  addi r7, r11, 0xa94
	ctx.r[7].s64 = ctx.r[11].s64 + 2708;
	// 82686350: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82686354: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82686358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268635C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686368: 386AAE60  addi r3, r10, -0x51a0
	ctx.r[3].s64 = ctx.r[10].s64 + -20896;
	// 8268636C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268637C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268638C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686390: 4BDE0A91  bl 0x82466e20
	ctx.lr = 0x82686394;
	sub_82466E20(ctx, base);
	// 82686394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268639C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826863A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826863A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826863A8 size=24
    let mut pc: u32 = 0x826863A8;
    'dispatch: loop {
        match pc {
            0x826863A8 => {
    //   block [0x826863A8..0x826863C0)
	// 826863A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826863AC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 826863B0: 394A4ED8  addi r10, r10, 0x4ed8
	ctx.r[10].s64 = ctx.r[10].s64 + 20184;
	// 826863B4: 816B0AC4  lwz r11, 0xac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2756 as u32) ) } as u64;
	// 826863B8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826863BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826863C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826863C0 size=112
    let mut pc: u32 = 0x826863C0;
    'dispatch: loop {
        match pc {
            0x826863C0 => {
    //   block [0x826863C0..0x82686430)
	// 826863C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826863C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826863C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826863CC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826863D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826863D4: 392A5DAC  addi r9, r10, 0x5dac
	ctx.r[9].s64 = ctx.r[10].s64 + 23980;
	// 826863D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826863DC: 390B4ED8  addi r8, r11, 0x4ed8
	ctx.r[8].s64 = ctx.r[11].s64 + 20184;
	// 826863E0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826863E4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826863E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826863EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826863F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826863F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826863F8: 386AAE90  addi r3, r10, -0x5170
	ctx.r[3].s64 = ctx.r[10].s64 + -20848;
	// 826863FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82686400: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82686404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268640C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686414: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268641C: 4BDE0A05  bl 0x82466e20
	ctx.lr = 0x82686420;
	sub_82466E20(ctx, base);
	// 82686420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268642C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686430 size=108
    let mut pc: u32 = 0x82686430;
    'dispatch: loop {
        match pc {
            0x82686430 => {
    //   block [0x82686430..0x8268649C)
	// 82686430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268643C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686444: 38EB0AC8  addi r7, r11, 0xac8
	ctx.r[7].s64 = ctx.r[11].s64 + 2760;
	// 82686448: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8268644C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82686450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686454: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268645C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686460: 386AAEC0  addi r3, r10, -0x5140
	ctx.r[3].s64 = ctx.r[10].s64 + -20800;
	// 82686464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268646C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268647C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686488: 4BDE0999  bl 0x82466e20
	ctx.lr = 0x8268648C;
	sub_82466E20(ctx, base);
	// 8268648C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826864A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826864A0 size=108
    let mut pc: u32 = 0x826864A0;
    'dispatch: loop {
        match pc {
            0x826864A0 => {
    //   block [0x826864A0..0x8268650C)
	// 826864A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826864A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826864A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826864AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826864B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826864B4: 38EB0AE0  addi r7, r11, 0xae0
	ctx.r[7].s64 = ctx.r[11].s64 + 2784;
	// 826864B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826864BC: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826864C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826864C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826864C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826864CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826864D0: 386AAEF0  addi r3, r10, -0x5110
	ctx.r[3].s64 = ctx.r[10].s64 + -20752;
	// 826864D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826864D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826864DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826864E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826864E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826864E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826864EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826864F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826864F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826864F8: 4BDE0929  bl 0x82466e20
	ctx.lr = 0x826864FC;
	sub_82466E20(ctx, base);
	// 826864FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686510 size=108
    let mut pc: u32 = 0x82686510;
    'dispatch: loop {
        match pc {
            0x82686510 => {
    //   block [0x82686510..0x8268657C)
	// 82686510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268651C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686524: 38EB0B28  addi r7, r11, 0xb28
	ctx.r[7].s64 = ctx.r[11].s64 + 2856;
	// 82686528: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268652C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 82686530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686534: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268653C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686540: 386AAF20  addi r3, r10, -0x50e0
	ctx.r[3].s64 = ctx.r[10].s64 + -20704;
	// 82686544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268654C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268655C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686568: 4BDE08B9  bl 0x82466e20
	ctx.lr = 0x8268656C;
	sub_82466E20(ctx, base);
	// 8268656C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686580 size=108
    let mut pc: u32 = 0x82686580;
    'dispatch: loop {
        match pc {
            0x82686580 => {
    //   block [0x82686580..0x826865EC)
	// 82686580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268658C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686594: 38EB0B58  addi r7, r11, 0xb58
	ctx.r[7].s64 = ctx.r[11].s64 + 2904;
	// 82686598: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8268659C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826865A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826865A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826865A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826865AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826865B0: 386AAF50  addi r3, r10, -0x50b0
	ctx.r[3].s64 = ctx.r[10].s64 + -20656;
	// 826865B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826865B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826865BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826865C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826865C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826865C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826865CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826865D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826865D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826865D8: 4BDE0849  bl 0x82466e20
	ctx.lr = 0x826865DC;
	sub_82466E20(ctx, base);
	// 826865DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826865E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826865E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826865E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826865F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826865F0 size=108
    let mut pc: u32 = 0x826865F0;
    'dispatch: loop {
        match pc {
            0x826865F0 => {
    //   block [0x826865F0..0x8268665C)
	// 826865F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826865F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826865F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826865FC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686604: 38EB0C78  addi r7, r11, 0xc78
	ctx.r[7].s64 = ctx.r[11].s64 + 3192;
	// 82686608: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8268660C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82686610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686614: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268661C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686620: 386AAF80  addi r3, r10, -0x5080
	ctx.r[3].s64 = ctx.r[10].s64 + -20608;
	// 82686624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268662C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268663C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686648: 4BDE07D9  bl 0x82466e20
	ctx.lr = 0x8268664C;
	sub_82466E20(ctx, base);
	// 8268664C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686660 size=108
    let mut pc: u32 = 0x82686660;
    'dispatch: loop {
        match pc {
            0x82686660 => {
    //   block [0x82686660..0x826866CC)
	// 82686660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268666C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686674: 38EB0D08  addi r7, r11, 0xd08
	ctx.r[7].s64 = ctx.r[11].s64 + 3336;
	// 82686678: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8268667C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 82686680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686684: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268668C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686690: 386AAFB0  addi r3, r10, -0x5050
	ctx.r[3].s64 = ctx.r[10].s64 + -20560;
	// 82686694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268669C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826866A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826866A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826866A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826866AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826866B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826866B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826866B8: 4BDE0769  bl 0x82466e20
	ctx.lr = 0x826866BC;
	sub_82466E20(ctx, base);
	// 826866BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826866C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826866C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826866C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826866D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826866D0 size=108
    let mut pc: u32 = 0x826866D0;
    'dispatch: loop {
        match pc {
            0x826866D0 => {
    //   block [0x826866D0..0x8268673C)
	// 826866D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826866D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826866D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826866DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826866E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826866E4: 38EB0DC8  addi r7, r11, 0xdc8
	ctx.r[7].s64 = ctx.r[11].s64 + 3528;
	// 826866E8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826866EC: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826866F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826866F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826866F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826866FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686700: 386AAFE0  addi r3, r10, -0x5020
	ctx.r[3].s64 = ctx.r[10].s64 + -20512;
	// 82686704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268670C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268671C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686728: 4BDE06F9  bl 0x82466e20
	ctx.lr = 0x8268672C;
	sub_82466E20(ctx, base);
	// 8268672C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686740 size=108
    let mut pc: u32 = 0x82686740;
    'dispatch: loop {
        match pc {
            0x82686740 => {
    //   block [0x82686740..0x826867AC)
	// 82686740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268674C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686754: 38EB0EA0  addi r7, r11, 0xea0
	ctx.r[7].s64 = ctx.r[11].s64 + 3744;
	// 82686758: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8268675C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 82686760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686764: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268676C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686770: 386AB010  addi r3, r10, -0x4ff0
	ctx.r[3].s64 = ctx.r[10].s64 + -20464;
	// 82686774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268677C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268678C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686798: 4BDE0689  bl 0x82466e20
	ctx.lr = 0x8268679C;
	sub_82466E20(ctx, base);
	// 8268679C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826867A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826867A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826867A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826867B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826867B0 size=108
    let mut pc: u32 = 0x826867B0;
    'dispatch: loop {
        match pc {
            0x826867B0 => {
    //   block [0x826867B0..0x8268681C)
	// 826867B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826867B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826867B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826867BC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826867C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826867C4: 38EB0F60  addi r7, r11, 0xf60
	ctx.r[7].s64 = ctx.r[11].s64 + 3936;
	// 826867C8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826867CC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826867D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826867D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826867D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826867DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826867E0: 386AB040  addi r3, r10, -0x4fc0
	ctx.r[3].s64 = ctx.r[10].s64 + -20416;
	// 826867E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826867E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826867EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826867F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826867F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826867F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826867FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686808: 4BDE0619  bl 0x82466e20
	ctx.lr = 0x8268680C;
	sub_82466E20(ctx, base);
	// 8268680C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686820 size=112
    let mut pc: u32 = 0x82686820;
    'dispatch: loop {
        match pc {
            0x82686820 => {
    //   block [0x82686820..0x82686890)
	// 82686820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268682C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82686830: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82686834: 38EA1008  addi r7, r10, 0x1008
	ctx.r[7].s64 = ctx.r[10].s64 + 4104;
	// 82686838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268683C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82686840: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 82686844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686848: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268684C: 396B5DC0  addi r11, r11, 0x5dc0
	ctx.r[11].s64 = ctx.r[11].s64 + 24000;
	// 82686850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686854: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686858: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268685C: 386AB070  addi r3, r10, -0x4f90
	ctx.r[3].s64 = ctx.r[10].s64 + -20368;
	// 82686860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686864: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82686868: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268686C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82686870: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686874: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686878: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268687C: 4BDE05A5  bl 0x82466e20
	ctx.lr = 0x82686880;
	sub_82466E20(ctx, base);
	// 82686880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268688C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686890 size=108
    let mut pc: u32 = 0x82686890;
    'dispatch: loop {
        match pc {
            0x82686890 => {
    //   block [0x82686890..0x826868FC)
	// 82686890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268689C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826868A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826868A4: 38EB1128  addi r7, r11, 0x1128
	ctx.r[7].s64 = ctx.r[11].s64 + 4392;
	// 826868A8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826868AC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826868B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826868B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826868B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826868BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826868C0: 386AB0A0  addi r3, r10, -0x4f60
	ctx.r[3].s64 = ctx.r[10].s64 + -20320;
	// 826868C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826868C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826868CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826868D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826868D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826868D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826868DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826868E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826868E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826868E8: 4BDE0539  bl 0x82466e20
	ctx.lr = 0x826868EC;
	sub_82466E20(ctx, base);
	// 826868EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826868F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826868F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826868F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686900 size=108
    let mut pc: u32 = 0x82686900;
    'dispatch: loop {
        match pc {
            0x82686900 => {
    //   block [0x82686900..0x8268696C)
	// 82686900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268690C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686914: 38EB1188  addi r7, r11, 0x1188
	ctx.r[7].s64 = ctx.r[11].s64 + 4488;
	// 82686918: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8268691C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82686920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686924: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268692C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686930: 386AB0D0  addi r3, r10, -0x4f30
	ctx.r[3].s64 = ctx.r[10].s64 + -20272;
	// 82686934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268693C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268694C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686958: 4BDE04C9  bl 0x82466e20
	ctx.lr = 0x8268695C;
	sub_82466E20(ctx, base);
	// 8268695C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686970 size=108
    let mut pc: u32 = 0x82686970;
    'dispatch: loop {
        match pc {
            0x82686970 => {
    //   block [0x82686970..0x826869DC)
	// 82686970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268697C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686984: 38EB1290  addi r7, r11, 0x1290
	ctx.r[7].s64 = ctx.r[11].s64 + 4752;
	// 82686988: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8268698C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 82686990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686994: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268699C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826869A0: 386AB100  addi r3, r10, -0x4f00
	ctx.r[3].s64 = ctx.r[10].s64 + -20224;
	// 826869A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826869A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826869AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826869B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826869B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826869B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826869BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826869C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826869C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826869C8: 4BDE0459  bl 0x82466e20
	ctx.lr = 0x826869CC;
	sub_82466E20(ctx, base);
	// 826869CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826869D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826869D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826869D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826869E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826869E0 size=108
    let mut pc: u32 = 0x826869E0;
    'dispatch: loop {
        match pc {
            0x826869E0 => {
    //   block [0x826869E0..0x82686A4C)
	// 826869E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826869E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826869E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826869EC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826869F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826869F4: 38EB1368  addi r7, r11, 0x1368
	ctx.r[7].s64 = ctx.r[11].s64 + 4968;
	// 826869F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826869FC: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82686A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686A04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686A08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686A10: 386AB130  addi r3, r10, -0x4ed0
	ctx.r[3].s64 = ctx.r[10].s64 + -20176;
	// 82686A14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686A38: 4BDE03E9  bl 0x82466e20
	ctx.lr = 0x82686A3C;
	sub_82466E20(ctx, base);
	// 82686A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686A50 size=108
    let mut pc: u32 = 0x82686A50;
    'dispatch: loop {
        match pc {
            0x82686A50 => {
    //   block [0x82686A50..0x82686ABC)
	// 82686A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686A5C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686A64: 38EB1398  addi r7, r11, 0x1398
	ctx.r[7].s64 = ctx.r[11].s64 + 5016;
	// 82686A68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82686A6C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 82686A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686A74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686A80: 386AB160  addi r3, r10, -0x4ea0
	ctx.r[3].s64 = ctx.r[10].s64 + -20128;
	// 82686A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686AA8: 4BDE0379  bl 0x82466e20
	ctx.lr = 0x82686AAC;
	sub_82466E20(ctx, base);
	// 82686AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686AC0 size=108
    let mut pc: u32 = 0x82686AC0;
    'dispatch: loop {
        match pc {
            0x82686AC0 => {
    //   block [0x82686AC0..0x82686B2C)
	// 82686AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686ACC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686AD4: 38EB13B0  addi r7, r11, 0x13b0
	ctx.r[7].s64 = ctx.r[11].s64 + 5040;
	// 82686AD8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82686ADC: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 82686AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686AE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686AF0: 386AB190  addi r3, r10, -0x4e70
	ctx.r[3].s64 = ctx.r[10].s64 + -20080;
	// 82686AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686B18: 4BDE0309  bl 0x82466e20
	ctx.lr = 0x82686B1C;
	sub_82466E20(ctx, base);
	// 82686B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686B30 size=108
    let mut pc: u32 = 0x82686B30;
    'dispatch: loop {
        match pc {
            0x82686B30 => {
    //   block [0x82686B30..0x82686B9C)
	// 82686B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686B3C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686B40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686B44: 38EB13F8  addi r7, r11, 0x13f8
	ctx.r[7].s64 = ctx.r[11].s64 + 5112;
	// 82686B48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82686B4C: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 82686B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686B54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686B58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686B60: 386AB1C0  addi r3, r10, -0x4e40
	ctx.r[3].s64 = ctx.r[10].s64 + -20032;
	// 82686B64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686B88: 4BDE0299  bl 0x82466e20
	ctx.lr = 0x82686B8C;
	sub_82466E20(ctx, base);
	// 82686B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686BA0 size=112
    let mut pc: u32 = 0x82686BA0;
    'dispatch: loop {
        match pc {
            0x82686BA0 => {
    //   block [0x82686BA0..0x82686C10)
	// 82686BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686BAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686BB0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686BB4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 82686BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686BBC: 390B1410  addi r8, r11, 0x1410
	ctx.r[8].s64 = ctx.r[11].s64 + 5136;
	// 82686BC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82686BC4: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82686BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686BCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686BD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82686BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686BD8: 386AB1F0  addi r3, r10, -0x4e10
	ctx.r[3].s64 = ctx.r[10].s64 + -19984;
	// 82686BDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82686BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686BFC: 4BDE0225  bl 0x82466e20
	ctx.lr = 0x82686C00;
	sub_82466E20(ctx, base);
	// 82686C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686C10 size=108
    let mut pc: u32 = 0x82686C10;
    'dispatch: loop {
        match pc {
            0x82686C10 => {
    //   block [0x82686C10..0x82686C7C)
	// 82686C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686C1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686C24: 38EB1458  addi r7, r11, 0x1458
	ctx.r[7].s64 = ctx.r[11].s64 + 5208;
	// 82686C28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82686C2C: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82686C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686C34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82686C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686C40: 386AB220  addi r3, r10, -0x4de0
	ctx.r[3].s64 = ctx.r[10].s64 + -19936;
	// 82686C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82686C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686C68: 4BDE01B9  bl 0x82466e20
	ctx.lr = 0x82686C6C;
	sub_82466E20(ctx, base);
	// 82686C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686C80 size=112
    let mut pc: u32 = 0x82686C80;
    'dispatch: loop {
        match pc {
            0x82686C80 => {
    //   block [0x82686C80..0x82686CF0)
	// 82686C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686C8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686C90: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686C94: 38AAB220  addi r5, r10, -0x4de0
	ctx.r[5].s64 = ctx.r[10].s64 + -19936;
	// 82686C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686C9C: 390B14B8  addi r8, r11, 0x14b8
	ctx.r[8].s64 = ctx.r[11].s64 + 5304;
	// 82686CA0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82686CA4: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82686CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686CAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686CB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82686CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686CB8: 386AB250  addi r3, r10, -0x4db0
	ctx.r[3].s64 = ctx.r[10].s64 + -19888;
	// 82686CBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82686CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686CDC: 4BDE0145  bl 0x82466e20
	ctx.lr = 0x82686CE0;
	sub_82466E20(ctx, base);
	// 82686CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686CF0 size=96
    let mut pc: u32 = 0x82686CF0;
    'dispatch: loop {
        match pc {
            0x82686CF0 => {
    //   block [0x82686CF0..0x82686D50)
	// 82686CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686CFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686D04: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82686D08: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686D10: 386AB280  addi r3, r10, -0x4d80
	ctx.r[3].s64 = ctx.r[10].s64 + -19840;
	// 82686D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686D1C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82686D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686D30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82686D34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686D38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82686D3C: 4BDE00E5  bl 0x82466e20
	ctx.lr = 0x82686D40;
	sub_82466E20(ctx, base);
	// 82686D40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686D50 size=112
    let mut pc: u32 = 0x82686D50;
    'dispatch: loop {
        match pc {
            0x82686D50 => {
    //   block [0x82686D50..0x82686DC0)
	// 82686D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686D5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686D60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686D64: 38AACAB0  addi r5, r10, -0x3550
	ctx.r[5].s64 = ctx.r[10].s64 + -13648;
	// 82686D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686D6C: 390B1500  addi r8, r11, 0x1500
	ctx.r[8].s64 = ctx.r[11].s64 + 5376;
	// 82686D70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82686D74: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82686D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686D7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82686D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686D88: 386AB2B0  addi r3, r10, -0x4d50
	ctx.r[3].s64 = ctx.r[10].s64 + -19792;
	// 82686D8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82686D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686DAC: 4BDE0075  bl 0x82466e20
	ctx.lr = 0x82686DB0;
	sub_82466E20(ctx, base);
	// 82686DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686DC0 size=96
    let mut pc: u32 = 0x82686DC0;
    'dispatch: loop {
        match pc {
            0x82686DC0 => {
    //   block [0x82686DC0..0x82686E20)
	// 82686DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686DCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686DD4: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82686DD8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686DE0: 386AB2E0  addi r3, r10, -0x4d20
	ctx.r[3].s64 = ctx.r[10].s64 + -19744;
	// 82686DE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686DEC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82686DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686E00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82686E04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686E08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82686E0C: 4BDE0015  bl 0x82466e20
	ctx.lr = 0x82686E10;
	sub_82466E20(ctx, base);
	// 82686E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686E20 size=100
    let mut pc: u32 = 0x82686E20;
    'dispatch: loop {
        match pc {
            0x82686E20 => {
    //   block [0x82686E20..0x82686E84)
	// 82686E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686E2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686E34: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 82686E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686E40: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82686E44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686E4C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82686E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686E54: 386AB310  addi r3, r10, -0x4cf0
	ctx.r[3].s64 = ctx.r[10].s64 + -19696;
	// 82686E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686E5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686E60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82686E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686E68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82686E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686E70: 4BDDFFB1  bl 0x82466e20
	ctx.lr = 0x82686E74;
	sub_82466E20(ctx, base);
	// 82686E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686E88 size=96
    let mut pc: u32 = 0x82686E88;
    'dispatch: loop {
        match pc {
            0x82686E88 => {
    //   block [0x82686E88..0x82686EE8)
	// 82686E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686E94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686E9C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82686EA0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686EA8: 386AB340  addi r3, r10, -0x4cc0
	ctx.r[3].s64 = ctx.r[10].s64 + -19648;
	// 82686EAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686EB4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82686EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686EC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82686ECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82686ED0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82686ED4: 4BDDFF4D  bl 0x82466e20
	ctx.lr = 0x82686ED8;
	sub_82466E20(ctx, base);
	// 82686ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686EE8 size=112
    let mut pc: u32 = 0x82686EE8;
    'dispatch: loop {
        match pc {
            0x82686EE8 => {
    //   block [0x82686EE8..0x82686F58)
	// 82686EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686EF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686EF8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686EFC: 38AAB310  addi r5, r10, -0x4cf0
	ctx.r[5].s64 = ctx.r[10].s64 + -19696;
	// 82686F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686F04: 390B1560  addi r8, r11, 0x1560
	ctx.r[8].s64 = ctx.r[11].s64 + 5472;
	// 82686F08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82686F0C: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82686F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686F14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82686F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686F20: 386AB370  addi r3, r10, -0x4c90
	ctx.r[3].s64 = ctx.r[10].s64 + -19600;
	// 82686F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82686F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686F44: 4BDDFEDD  bl 0x82466e20
	ctx.lr = 0x82686F48;
	sub_82466E20(ctx, base);
	// 82686F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686F58 size=112
    let mut pc: u32 = 0x82686F58;
    'dispatch: loop {
        match pc {
            0x82686F58 => {
    //   block [0x82686F58..0x82686FC8)
	// 82686F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686F64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686F68: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82686F6C: 38AAB310  addi r5, r10, -0x4cf0
	ctx.r[5].s64 = ctx.r[10].s64 + -19696;
	// 82686F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686F74: 390B1590  addi r8, r11, 0x1590
	ctx.r[8].s64 = ctx.r[11].s64 + 5520;
	// 82686F78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82686F7C: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82686F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686F88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82686F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82686F90: 386AB3A0  addi r3, r10, -0x4c60
	ctx.r[3].s64 = ctx.r[10].s64 + -19552;
	// 82686F94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82686F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82686F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82686FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82686FB4: 4BDDFE6D  bl 0x82466e20
	ctx.lr = 0x82686FB8;
	sub_82466E20(ctx, base);
	// 82686FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82686FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82686FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82686FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82686FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82686FC8 size=100
    let mut pc: u32 = 0x82686FC8;
    'dispatch: loop {
        match pc {
            0x82686FC8 => {
    //   block [0x82686FC8..0x8268702C)
	// 82686FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82686FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82686FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82686FD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82686FDC: 38AAB310  addi r5, r10, -0x4cf0
	ctx.r[5].s64 = ctx.r[10].s64 + -19696;
	// 82686FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82686FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82686FE8: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82686FEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82686FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82686FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82686FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82686FFC: 386AB3D0  addi r3, r10, -0x4c30
	ctx.r[3].s64 = ctx.r[10].s64 + -19504;
	// 82687000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687004: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687008: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268700C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687010: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82687014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687018: 4BDDFE09  bl 0x82466e20
	ctx.lr = 0x8268701C;
	sub_82466E20(ctx, base);
	// 8268701C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687030 size=96
    let mut pc: u32 = 0x82687030;
    'dispatch: loop {
        match pc {
            0x82687030 => {
    //   block [0x82687030..0x82687090)
	// 82687030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268703C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687044: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82687048: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268704C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687050: 386AB400  addi r3, r10, -0x4c00
	ctx.r[3].s64 = ctx.r[10].s64 + -19456;
	// 82687054: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268705C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82687060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268706C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687070: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82687074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82687078: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268707C: 4BDDFDA5  bl 0x82466e20
	ctx.lr = 0x82687080;
	sub_82466E20(ctx, base);
	// 82687080: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268708C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687090 size=112
    let mut pc: u32 = 0x82687090;
    'dispatch: loop {
        match pc {
            0x82687090 => {
    //   block [0x82687090..0x82687100)
	// 82687090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268709C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826870A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826870A4: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 826870A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826870AC: 390B15A8  addi r8, r11, 0x15a8
	ctx.r[8].s64 = ctx.r[11].s64 + 5544;
	// 826870B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826870B4: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826870B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826870BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826870C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826870C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826870C8: 386AB430  addi r3, r10, -0x4bd0
	ctx.r[3].s64 = ctx.r[10].s64 + -19408;
	// 826870CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826870D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826870D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826870D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826870DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826870E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826870E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826870E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826870EC: 4BDDFD35  bl 0x82466e20
	ctx.lr = 0x826870F0;
	sub_82466E20(ctx, base);
	// 826870F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826870F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826870F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826870FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687100 size=108
    let mut pc: u32 = 0x82687100;
    'dispatch: loop {
        match pc {
            0x82687100 => {
    //   block [0x82687100..0x8268716C)
	// 82687100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268710C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687114: 38EB15C0  addi r7, r11, 0x15c0
	ctx.r[7].s64 = ctx.r[11].s64 + 5568;
	// 82687118: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8268711C: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82687120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687124: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687128: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268712C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687130: 386AB460  addi r3, r10, -0x4ba0
	ctx.r[3].s64 = ctx.r[10].s64 + -19360;
	// 82687134: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82687138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268713C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268714C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82687158: 4BDDFCC9  bl 0x82466e20
	ctx.lr = 0x8268715C;
	sub_82466E20(ctx, base);
	// 8268715C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687170 size=112
    let mut pc: u32 = 0x82687170;
    'dispatch: loop {
        match pc {
            0x82687170 => {
    //   block [0x82687170..0x826871E0)
	// 82687170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268717C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687180: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687184: 38AAB580  addi r5, r10, -0x4a80
	ctx.r[5].s64 = ctx.r[10].s64 + -19072;
	// 82687188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268718C: 390B1620  addi r8, r11, 0x1620
	ctx.r[8].s64 = ctx.r[11].s64 + 5664;
	// 82687190: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82687194: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82687198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268719C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826871A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826871A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826871A8: 386AB490  addi r3, r10, -0x4b70
	ctx.r[3].s64 = ctx.r[10].s64 + -19312;
	// 826871AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826871B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826871B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826871B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826871BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826871C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826871C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826871C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826871CC: 4BDDFC55  bl 0x82466e20
	ctx.lr = 0x826871D0;
	sub_82466E20(ctx, base);
	// 826871D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826871D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826871D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826871DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826871E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826871E0 size=112
    let mut pc: u32 = 0x826871E0;
    'dispatch: loop {
        match pc {
            0x826871E0 => {
    //   block [0x826871E0..0x82687250)
	// 826871E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826871E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826871E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826871EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826871F0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826871F4: 38AAB430  addi r5, r10, -0x4bd0
	ctx.r[5].s64 = ctx.r[10].s64 + -19408;
	// 826871F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826871FC: 390B1638  addi r8, r11, 0x1638
	ctx.r[8].s64 = ctx.r[11].s64 + 5688;
	// 82687200: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82687204: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 82687208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268720C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687218: 386AB4C0  addi r3, r10, -0x4b40
	ctx.r[3].s64 = ctx.r[10].s64 + -19264;
	// 8268721C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268722C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268723C: 4BDDFBE5  bl 0x82466e20
	ctx.lr = 0x82687240;
	sub_82466E20(ctx, base);
	// 82687240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268724C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687250 size=112
    let mut pc: u32 = 0x82687250;
    'dispatch: loop {
        match pc {
            0x82687250 => {
    //   block [0x82687250..0x826872C0)
	// 82687250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268725C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687260: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687264: 38AAB430  addi r5, r10, -0x4bd0
	ctx.r[5].s64 = ctx.r[10].s64 + -19408;
	// 82687268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268726C: 390B1668  addi r8, r11, 0x1668
	ctx.r[8].s64 = ctx.r[11].s64 + 5736;
	// 82687270: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82687274: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 82687278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268727C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687280: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687288: 386AB4F0  addi r3, r10, -0x4b10
	ctx.r[3].s64 = ctx.r[10].s64 + -19216;
	// 8268728C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268729C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826872A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826872A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826872A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826872AC: 4BDDFB75  bl 0x82466e20
	ctx.lr = 0x826872B0;
	sub_82466E20(ctx, base);
	// 826872B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826872B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826872B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826872BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826872C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826872C0 size=112
    let mut pc: u32 = 0x826872C0;
    'dispatch: loop {
        match pc {
            0x826872C0 => {
    //   block [0x826872C0..0x82687330)
	// 826872C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826872C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826872C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826872CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826872D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826872D4: 38AAB580  addi r5, r10, -0x4a80
	ctx.r[5].s64 = ctx.r[10].s64 + -19072;
	// 826872D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826872DC: 390B1680  addi r8, r11, 0x1680
	ctx.r[8].s64 = ctx.r[11].s64 + 5760;
	// 826872E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826872E4: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826872E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826872EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826872F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826872F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826872F8: 386AB520  addi r3, r10, -0x4ae0
	ctx.r[3].s64 = ctx.r[10].s64 + -19168;
	// 826872FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268730C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268731C: 4BDDFB05  bl 0x82466e20
	ctx.lr = 0x82687320;
	sub_82466E20(ctx, base);
	// 82687320: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268732C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687330 size=112
    let mut pc: u32 = 0x82687330;
    'dispatch: loop {
        match pc {
            0x82687330 => {
    //   block [0x82687330..0x826873A0)
	// 82687330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268733C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687340: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687344: 38AAB430  addi r5, r10, -0x4bd0
	ctx.r[5].s64 = ctx.r[10].s64 + -19408;
	// 82687348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268734C: 390B16B0  addi r8, r11, 0x16b0
	ctx.r[8].s64 = ctx.r[11].s64 + 5808;
	// 82687350: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82687354: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82687358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268735C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687368: 386AB550  addi r3, r10, -0x4ab0
	ctx.r[3].s64 = ctx.r[10].s64 + -19120;
	// 8268736C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268737C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268738C: 4BDDFA95  bl 0x82466e20
	ctx.lr = 0x82687390;
	sub_82466E20(ctx, base);
	// 82687390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268739C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826873A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826873A0 size=112
    let mut pc: u32 = 0x826873A0;
    'dispatch: loop {
        match pc {
            0x826873A0 => {
    //   block [0x826873A0..0x82687410)
	// 826873A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826873A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826873A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826873AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826873B0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826873B4: 38AABA30  addi r5, r10, -0x45d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17872;
	// 826873B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826873BC: 390B16C8  addi r8, r11, 0x16c8
	ctx.r[8].s64 = ctx.r[11].s64 + 5832;
	// 826873C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826873C4: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826873C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826873CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826873D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826873D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826873D8: 386AB580  addi r3, r10, -0x4a80
	ctx.r[3].s64 = ctx.r[10].s64 + -19072;
	// 826873DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826873E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826873E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826873E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826873EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826873F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826873F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826873F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826873FC: 4BDDFA25  bl 0x82466e20
	ctx.lr = 0x82687400;
	sub_82466E20(ctx, base);
	// 82687400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268740C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687410 size=112
    let mut pc: u32 = 0x82687410;
    'dispatch: loop {
        match pc {
            0x82687410 => {
    //   block [0x82687410..0x82687480)
	// 82687410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268741C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687420: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687424: 38AAB790  addi r5, r10, -0x4870
	ctx.r[5].s64 = ctx.r[10].s64 + -18544;
	// 82687428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268742C: 390B16E0  addi r8, r11, 0x16e0
	ctx.r[8].s64 = ctx.r[11].s64 + 5856;
	// 82687430: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82687434: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82687438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268743C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687448: 386AB5B0  addi r3, r10, -0x4a50
	ctx.r[3].s64 = ctx.r[10].s64 + -19024;
	// 8268744C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268745C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268746C: 4BDDF9B5  bl 0x82466e20
	ctx.lr = 0x82687470;
	sub_82466E20(ctx, base);
	// 82687470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268747C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687480 size=112
    let mut pc: u32 = 0x82687480;
    'dispatch: loop {
        match pc {
            0x82687480 => {
    //   block [0x82687480..0x826874F0)
	// 82687480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268748C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687490: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687494: 38AAB550  addi r5, r10, -0x4ab0
	ctx.r[5].s64 = ctx.r[10].s64 + -19120;
	// 82687498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268749C: 390B16F8  addi r8, r11, 0x16f8
	ctx.r[8].s64 = ctx.r[11].s64 + 5880;
	// 826874A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826874A4: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826874A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826874AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826874B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826874B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826874B8: 386AB5E0  addi r3, r10, -0x4a20
	ctx.r[3].s64 = ctx.r[10].s64 + -18976;
	// 826874BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826874C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826874C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826874C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826874CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826874D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826874D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826874D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826874DC: 4BDDF945  bl 0x82466e20
	ctx.lr = 0x826874E0;
	sub_82466E20(ctx, base);
	// 826874E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826874E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826874E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826874EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826874F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826874F0 size=112
    let mut pc: u32 = 0x826874F0;
    'dispatch: loop {
        match pc {
            0x826874F0 => {
    //   block [0x826874F0..0x82687560)
	// 826874F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826874F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826874F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826874FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687500: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687504: 38AAB580  addi r5, r10, -0x4a80
	ctx.r[5].s64 = ctx.r[10].s64 + -19072;
	// 82687508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268750C: 390B1740  addi r8, r11, 0x1740
	ctx.r[8].s64 = ctx.r[11].s64 + 5952;
	// 82687510: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82687514: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82687518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268751C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687528: 386AB610  addi r3, r10, -0x49f0
	ctx.r[3].s64 = ctx.r[10].s64 + -18928;
	// 8268752C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268753C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268754C: 4BDDF8D5  bl 0x82466e20
	ctx.lr = 0x82687550;
	sub_82466E20(ctx, base);
	// 82687550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268755C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687560 size=112
    let mut pc: u32 = 0x82687560;
    'dispatch: loop {
        match pc {
            0x82687560 => {
    //   block [0x82687560..0x826875D0)
	// 82687560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268756C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687570: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687574: 38AAB580  addi r5, r10, -0x4a80
	ctx.r[5].s64 = ctx.r[10].s64 + -19072;
	// 82687578: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268757C: 390B1770  addi r8, r11, 0x1770
	ctx.r[8].s64 = ctx.r[11].s64 + 6000;
	// 82687580: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82687584: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82687588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268758C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687590: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687598: 386AB640  addi r3, r10, -0x49c0
	ctx.r[3].s64 = ctx.r[10].s64 + -18880;
	// 8268759C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826875A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826875A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826875A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826875AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826875B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826875B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826875B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826875BC: 4BDDF865  bl 0x82466e20
	ctx.lr = 0x826875C0;
	sub_82466E20(ctx, base);
	// 826875C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826875C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826875C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826875CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826875D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826875D0 size=108
    let mut pc: u32 = 0x826875D0;
    'dispatch: loop {
        match pc {
            0x826875D0 => {
    //   block [0x826875D0..0x8268763C)
	// 826875D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826875D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826875D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826875DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826875E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826875E4: 38EB17A0  addi r7, r11, 0x17a0
	ctx.r[7].s64 = ctx.r[11].s64 + 6048;
	// 826875E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826875EC: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826875F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826875F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826875F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826875FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687600: 386AB670  addi r3, r10, -0x4990
	ctx.r[3].s64 = ctx.r[10].s64 + -18832;
	// 82687604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82687608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268760C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268761C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82687628: 4BDDF7F9  bl 0x82466e20
	ctx.lr = 0x8268762C;
	sub_82466E20(ctx, base);
	// 8268762C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687640 size=112
    let mut pc: u32 = 0x82687640;
    'dispatch: loop {
        match pc {
            0x82687640 => {
    //   block [0x82687640..0x826876B0)
	// 82687640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268764C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687650: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687654: 38AAB580  addi r5, r10, -0x4a80
	ctx.r[5].s64 = ctx.r[10].s64 + -19072;
	// 82687658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268765C: 390B17E8  addi r8, r11, 0x17e8
	ctx.r[8].s64 = ctx.r[11].s64 + 6120;
	// 82687660: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82687664: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82687668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268766C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687670: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687678: 386AB6A0  addi r3, r10, -0x4960
	ctx.r[3].s64 = ctx.r[10].s64 + -18784;
	// 8268767C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268768C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268769C: 4BDDF785  bl 0x82466e20
	ctx.lr = 0x826876A0;
	sub_82466E20(ctx, base);
	// 826876A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826876A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826876A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826876AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826876B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826876B0 size=116
    let mut pc: u32 = 0x826876B0;
    'dispatch: loop {
        match pc {
            0x826876B0 => {
    //   block [0x826876B0..0x82687724)
	// 826876B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826876B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826876B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826876BC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826876C0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826876C4: 390B1868  addi r8, r11, 0x1868
	ctx.r[8].s64 = ctx.r[11].s64 + 6248;
	// 826876C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826876CC: 392A5E48  addi r9, r10, 0x5e48
	ctx.r[9].s64 = ctx.r[10].s64 + 24136;
	// 826876D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826876D4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826876D8: 38AAB580  addi r5, r10, -0x4a80
	ctx.r[5].s64 = ctx.r[10].s64 + -19072;
	// 826876DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826876E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826876E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826876E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826876EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826876F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826876F4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826876F8: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826876FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82687700: 386BB6D0  addi r3, r11, -0x4930
	ctx.r[3].s64 = ctx.r[11].s64 + -18736;
	// 82687704: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82687708: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268770C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687710: 4BDDF711  bl 0x82466e20
	ctx.lr = 0x82687714;
	sub_82466E20(ctx, base);
	// 82687714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268771C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687728 size=100
    let mut pc: u32 = 0x82687728;
    'dispatch: loop {
        match pc {
            0x82687728 => {
    //   block [0x82687728..0x8268778C)
	// 82687728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268772C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687734: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268773C: 38AAB820  addi r5, r10, -0x47e0
	ctx.r[5].s64 = ctx.r[10].s64 + -18400;
	// 82687740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687748: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8268774C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268775C: 386AB700  addi r3, r10, -0x4900
	ctx.r[3].s64 = ctx.r[10].s64 + -18688;
	// 82687760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687764: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687768: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268776C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687770: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82687774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687778: 4BDDF6A9  bl 0x82466e20
	ctx.lr = 0x8268777C;
	sub_82466E20(ctx, base);
	// 8268777C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687790 size=100
    let mut pc: u32 = 0x82687790;
    'dispatch: loop {
        match pc {
            0x82687790 => {
    //   block [0x82687790..0x826877F4)
	// 82687790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268779C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826877A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826877A4: 38AAB430  addi r5, r10, -0x4bd0
	ctx.r[5].s64 = ctx.r[10].s64 + -19408;
	// 826877A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826877AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826877B0: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826877B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826877B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826877BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826877C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826877C4: 386AB730  addi r3, r10, -0x48d0
	ctx.r[3].s64 = ctx.r[10].s64 + -18640;
	// 826877C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826877CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826877D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826877D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826877D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826877DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826877E0: 4BDDF641  bl 0x82466e20
	ctx.lr = 0x826877E4;
	sub_82466E20(ctx, base);
	// 826877E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826877E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826877EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826877F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826877F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826877F8 size=108
    let mut pc: u32 = 0x826877F8;
    'dispatch: loop {
        match pc {
            0x826877F8 => {
    //   block [0x826877F8..0x82687864)
	// 826877F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826877FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687804: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268780C: 38EB18E0  addi r7, r11, 0x18e0
	ctx.r[7].s64 = ctx.r[11].s64 + 6368;
	// 82687810: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82687814: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 82687818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268781C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687820: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82687824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687828: 386AB760  addi r3, r10, -0x48a0
	ctx.r[3].s64 = ctx.r[10].s64 + -18592;
	// 8268782C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82687830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268783C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268784C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82687850: 4BDDF5D1  bl 0x82466e20
	ctx.lr = 0x82687854;
	sub_82466E20(ctx, base);
	// 82687854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687858: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268785C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687868 size=112
    let mut pc: u32 = 0x82687868;
    'dispatch: loop {
        match pc {
            0x82687868 => {
    //   block [0x82687868..0x826878D8)
	// 82687868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268786C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687874: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687878: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268787C: 38AAB550  addi r5, r10, -0x4ab0
	ctx.r[5].s64 = ctx.r[10].s64 + -19120;
	// 82687880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687884: 390B1910  addi r8, r11, 0x1910
	ctx.r[8].s64 = ctx.r[11].s64 + 6416;
	// 82687888: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268788C: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82687890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687894: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268789C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826878A0: 386AB790  addi r3, r10, -0x4870
	ctx.r[3].s64 = ctx.r[10].s64 + -18544;
	// 826878A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826878A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826878AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826878B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826878B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826878B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826878BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826878C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826878C4: 4BDDF55D  bl 0x82466e20
	ctx.lr = 0x826878C8;
	sub_82466E20(ctx, base);
	// 826878C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826878CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826878D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826878D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826878D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826878D8 size=108
    let mut pc: u32 = 0x826878D8;
    'dispatch: loop {
        match pc {
            0x826878D8 => {
    //   block [0x826878D8..0x82687944)
	// 826878D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826878DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826878E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826878E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826878E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826878EC: 38EB1928  addi r7, r11, 0x1928
	ctx.r[7].s64 = ctx.r[11].s64 + 6440;
	// 826878F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826878F4: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826878F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826878FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82687904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687908: 386AB7C0  addi r3, r10, -0x4840
	ctx.r[3].s64 = ctx.r[10].s64 + -18496;
	// 8268790C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82687910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268791C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268792C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82687930: 4BDDF4F1  bl 0x82466e20
	ctx.lr = 0x82687934;
	sub_82466E20(ctx, base);
	// 82687934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268793C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82687948 size=28
    let mut pc: u32 = 0x82687948;
    'dispatch: loop {
        match pc {
            0x82687948 => {
    //   block [0x82687948..0x82687964)
	// 82687948: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268794C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82687950: 394A4F50  addi r10, r10, 0x4f50
	ctx.r[10].s64 = ctx.r[10].s64 + 20304;
	// 82687954: 816B1864  lwz r11, 0x1864(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6244 as u32) ) } as u64;
	// 82687958: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8268795C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82687960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687968 size=108
    let mut pc: u32 = 0x82687968;
    'dispatch: loop {
        match pc {
            0x82687968 => {
    //   block [0x82687968..0x826879D4)
	// 82687968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268796C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687974: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268797C: 38EB4F50  addi r7, r11, 0x4f50
	ctx.r[7].s64 = ctx.r[11].s64 + 20304;
	// 82687980: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82687984: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82687988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268798C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82687994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687998: 386AB7F0  addi r3, r10, -0x4810
	ctx.r[3].s64 = ctx.r[10].s64 + -18448;
	// 8268799C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826879A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826879A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826879A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826879AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826879B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826879B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826879B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826879BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826879C0: 4BDDF461  bl 0x82466e20
	ctx.lr = 0x826879C4;
	sub_82466E20(ctx, base);
	// 826879C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826879C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826879CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826879D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826879D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826879D8 size=116
    let mut pc: u32 = 0x826879D8;
    'dispatch: loop {
        match pc {
            0x826879D8 => {
    //   block [0x826879D8..0x82687A4C)
	// 826879D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826879DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826879E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826879E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826879E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826879EC: 390B1948  addi r8, r11, 0x1948
	ctx.r[8].s64 = ctx.r[11].s64 + 6472;
	// 826879F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826879F4: 392A5E9C  addi r9, r10, 0x5e9c
	ctx.r[9].s64 = ctx.r[10].s64 + 24220;
	// 826879F8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826879FC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82687A00: 38AAB550  addi r5, r10, -0x4ab0
	ctx.r[5].s64 = ctx.r[10].s64 + -19120;
	// 82687A04: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687A0C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687A1C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82687A20: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 82687A24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82687A28: 386BB820  addi r3, r11, -0x47e0
	ctx.r[3].s64 = ctx.r[11].s64 + -18400;
	// 82687A2C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82687A30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687A38: 4BDDF3E9  bl 0x82466e20
	ctx.lr = 0x82687A3C;
	sub_82466E20(ctx, base);
	// 82687A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687A50 size=112
    let mut pc: u32 = 0x82687A50;
    'dispatch: loop {
        match pc {
            0x82687A50 => {
    //   block [0x82687A50..0x82687AC0)
	// 82687A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687A5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687A60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687A64: 38AAB4F0  addi r5, r10, -0x4b10
	ctx.r[5].s64 = ctx.r[10].s64 + -19216;
	// 82687A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687A6C: 390B19A8  addi r8, r11, 0x19a8
	ctx.r[8].s64 = ctx.r[11].s64 + 6568;
	// 82687A70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82687A74: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82687A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687A7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687A80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687A88: 386AB850  addi r3, r10, -0x47b0
	ctx.r[3].s64 = ctx.r[10].s64 + -18352;
	// 82687A8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687A9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687AAC: 4BDDF375  bl 0x82466e20
	ctx.lr = 0x82687AB0;
	sub_82466E20(ctx, base);
	// 82687AB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687AB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687AB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687ABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687AC0 size=108
    let mut pc: u32 = 0x82687AC0;
    'dispatch: loop {
        match pc {
            0x82687AC0 => {
    //   block [0x82687AC0..0x82687B2C)
	// 82687AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687ACC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687AD4: 38EB19C0  addi r7, r11, 0x19c0
	ctx.r[7].s64 = ctx.r[11].s64 + 6592;
	// 82687AD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82687ADC: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82687AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687AE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82687AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687AF0: 386AB880  addi r3, r10, -0x4780
	ctx.r[3].s64 = ctx.r[10].s64 + -18304;
	// 82687AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82687AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82687B18: 4BDDF309  bl 0x82466e20
	ctx.lr = 0x82687B1C;
	sub_82466E20(ctx, base);
	// 82687B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687B30 size=112
    let mut pc: u32 = 0x82687B30;
    'dispatch: loop {
        match pc {
            0x82687B30 => {
    //   block [0x82687B30..0x82687BA0)
	// 82687B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687B3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687B40: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687B44: 38AAB430  addi r5, r10, -0x4bd0
	ctx.r[5].s64 = ctx.r[10].s64 + -19408;
	// 82687B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687B4C: 390B19F0  addi r8, r11, 0x19f0
	ctx.r[8].s64 = ctx.r[11].s64 + 6640;
	// 82687B50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82687B54: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82687B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687B5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687B68: 386AB8B0  addi r3, r10, -0x4750
	ctx.r[3].s64 = ctx.r[10].s64 + -18256;
	// 82687B6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687B8C: 4BDDF295  bl 0x82466e20
	ctx.lr = 0x82687B90;
	sub_82466E20(ctx, base);
	// 82687B90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687BA0 size=112
    let mut pc: u32 = 0x82687BA0;
    'dispatch: loop {
        match pc {
            0x82687BA0 => {
    //   block [0x82687BA0..0x82687C10)
	// 82687BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687BAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687BB0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687BB4: 38AABA30  addi r5, r10, -0x45d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17872;
	// 82687BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687BBC: 390B1A20  addi r8, r11, 0x1a20
	ctx.r[8].s64 = ctx.r[11].s64 + 6688;
	// 82687BC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82687BC4: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82687BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687BCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687BD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687BD8: 386AB8E0  addi r3, r10, -0x4720
	ctx.r[3].s64 = ctx.r[10].s64 + -18208;
	// 82687BDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687BFC: 4BDDF225  bl 0x82466e20
	ctx.lr = 0x82687C00;
	sub_82466E20(ctx, base);
	// 82687C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687C10 size=100
    let mut pc: u32 = 0x82687C10;
    'dispatch: loop {
        match pc {
            0x82687C10 => {
    //   block [0x82687C10..0x82687C74)
	// 82687C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687C1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687C24: 38AAB430  addi r5, r10, -0x4bd0
	ctx.r[5].s64 = ctx.r[10].s64 + -19408;
	// 82687C28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687C30: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 82687C34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687C44: 386AB910  addi r3, r10, -0x46f0
	ctx.r[3].s64 = ctx.r[10].s64 + -18160;
	// 82687C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687C4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687C50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82687C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687C58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82687C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687C60: 4BDDF1C1  bl 0x82466e20
	ctx.lr = 0x82687C64;
	sub_82466E20(ctx, base);
	// 82687C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687C78 size=112
    let mut pc: u32 = 0x82687C78;
    'dispatch: loop {
        match pc {
            0x82687C78 => {
    //   block [0x82687C78..0x82687CE8)
	// 82687C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687C84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687C88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687C8C: 38AAB730  addi r5, r10, -0x48d0
	ctx.r[5].s64 = ctx.r[10].s64 + -18640;
	// 82687C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687C94: 390B1A50  addi r8, r11, 0x1a50
	ctx.r[8].s64 = ctx.r[11].s64 + 6736;
	// 82687C98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82687C9C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82687CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687CA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687CB0: 386AB940  addi r3, r10, -0x46c0
	ctx.r[3].s64 = ctx.r[10].s64 + -18112;
	// 82687CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687CD4: 4BDDF14D  bl 0x82466e20
	ctx.lr = 0x82687CD8;
	sub_82466E20(ctx, base);
	// 82687CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687CE8 size=112
    let mut pc: u32 = 0x82687CE8;
    'dispatch: loop {
        match pc {
            0x82687CE8 => {
    //   block [0x82687CE8..0x82687D58)
	// 82687CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687CF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687CF8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687CFC: 38AAB730  addi r5, r10, -0x48d0
	ctx.r[5].s64 = ctx.r[10].s64 + -18640;
	// 82687D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687D04: 390B1A98  addi r8, r11, 0x1a98
	ctx.r[8].s64 = ctx.r[11].s64 + 6808;
	// 82687D08: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82687D0C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82687D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687D14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687D20: 386AB970  addi r3, r10, -0x4690
	ctx.r[3].s64 = ctx.r[10].s64 + -18064;
	// 82687D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687D44: 4BDDF0DD  bl 0x82466e20
	ctx.lr = 0x82687D48;
	sub_82466E20(ctx, base);
	// 82687D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687D58 size=108
    let mut pc: u32 = 0x82687D58;
    'dispatch: loop {
        match pc {
            0x82687D58 => {
    //   block [0x82687D58..0x82687DC4)
	// 82687D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687D64: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687D6C: 38EB1B40  addi r7, r11, 0x1b40
	ctx.r[7].s64 = ctx.r[11].s64 + 6976;
	// 82687D70: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82687D74: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 82687D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687D7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82687D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687D88: 386AB9A0  addi r3, r10, -0x4660
	ctx.r[3].s64 = ctx.r[10].s64 + -18016;
	// 82687D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82687D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82687DB0: 4BDDF071  bl 0x82466e20
	ctx.lr = 0x82687DB4;
	sub_82466E20(ctx, base);
	// 82687DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687DC8 size=112
    let mut pc: u32 = 0x82687DC8;
    'dispatch: loop {
        match pc {
            0x82687DC8 => {
    //   block [0x82687DC8..0x82687E38)
	// 82687DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687DD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687DD8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687DDC: 38AAB550  addi r5, r10, -0x4ab0
	ctx.r[5].s64 = ctx.r[10].s64 + -19120;
	// 82687DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687DE4: 390B1B88  addi r8, r11, 0x1b88
	ctx.r[8].s64 = ctx.r[11].s64 + 7048;
	// 82687DE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82687DEC: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82687DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687DF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687DF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687E00: 386AB9D0  addi r3, r10, -0x4630
	ctx.r[3].s64 = ctx.r[10].s64 + -17968;
	// 82687E04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687E08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687E0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687E10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687E18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687E20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687E24: 4BDDEFFD  bl 0x82466e20
	ctx.lr = 0x82687E28;
	sub_82466E20(ctx, base);
	// 82687E28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687E2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687E30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687E34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687E38 size=100
    let mut pc: u32 = 0x82687E38;
    'dispatch: loop {
        match pc {
            0x82687E38 => {
    //   block [0x82687E38..0x82687E9C)
	// 82687E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687E44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687E4C: 38AAB580  addi r5, r10, -0x4a80
	ctx.r[5].s64 = ctx.r[10].s64 + -19072;
	// 82687E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687E58: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82687E5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687E6C: 386ABA00  addi r3, r10, -0x4600
	ctx.r[3].s64 = ctx.r[10].s64 + -17920;
	// 82687E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82687E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82687E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687E88: 4BDDEF99  bl 0x82466e20
	ctx.lr = 0x82687E8C;
	sub_82466E20(ctx, base);
	// 82687E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687EA0 size=100
    let mut pc: u32 = 0x82687EA0;
    'dispatch: loop {
        match pc {
            0x82687EA0 => {
    //   block [0x82687EA0..0x82687F04)
	// 82687EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687EAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687EB4: 38AAB430  addi r5, r10, -0x4bd0
	ctx.r[5].s64 = ctx.r[10].s64 + -19408;
	// 82687EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687EC0: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 82687EC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687ED4: 386ABA30  addi r3, r10, -0x45d0
	ctx.r[3].s64 = ctx.r[10].s64 + -17872;
	// 82687ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687EDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687EE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82687EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687EE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82687EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687EF0: 4BDDEF31  bl 0x82466e20
	ctx.lr = 0x82687EF4;
	sub_82466E20(ctx, base);
	// 82687EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687F08 size=108
    let mut pc: u32 = 0x82687F08;
    'dispatch: loop {
        match pc {
            0x82687F08 => {
    //   block [0x82687F08..0x82687F74)
	// 82687F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687F14: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687F1C: 38EB1BE8  addi r7, r11, 0x1be8
	ctx.r[7].s64 = ctx.r[11].s64 + 7144;
	// 82687F20: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82687F24: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82687F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687F2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82687F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687F38: 386ABA60  addi r3, r10, -0x45a0
	ctx.r[3].s64 = ctx.r[10].s64 + -17824;
	// 82687F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82687F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82687F60: 4BDDEEC1  bl 0x82466e20
	ctx.lr = 0x82687F64;
	sub_82466E20(ctx, base);
	// 82687F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687F78 size=112
    let mut pc: u32 = 0x82687F78;
    'dispatch: loop {
        match pc {
            0x82687F78 => {
    //   block [0x82687F78..0x82687FE8)
	// 82687F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687F88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687F8C: 38AAB820  addi r5, r10, -0x47e0
	ctx.r[5].s64 = ctx.r[10].s64 + -18400;
	// 82687F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82687F94: 390B1C78  addi r8, r11, 0x1c78
	ctx.r[8].s64 = ctx.r[11].s64 + 7288;
	// 82687F98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82687F9C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 82687FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82687FA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82687FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82687FB0: 386ABA90  addi r3, r10, -0x4570
	ctx.r[3].s64 = ctx.r[10].s64 + -17776;
	// 82687FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82687FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82687FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82687FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82687FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82687FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82687FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82687FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82687FD4: 4BDDEE4D  bl 0x82466e20
	ctx.lr = 0x82687FD8;
	sub_82466E20(ctx, base);
	// 82687FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82687FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82687FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82687FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82687FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82687FE8 size=112
    let mut pc: u32 = 0x82687FE8;
    'dispatch: loop {
        match pc {
            0x82687FE8 => {
    //   block [0x82687FE8..0x82688058)
	// 82687FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82687FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82687FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82687FF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82687FF8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82687FFC: 38AAB970  addi r5, r10, -0x4690
	ctx.r[5].s64 = ctx.r[10].s64 + -18064;
	// 82688000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688004: 390B1C90  addi r8, r11, 0x1c90
	ctx.r[8].s64 = ctx.r[11].s64 + 7312;
	// 82688008: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268800C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 82688010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688014: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268801C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688020: 386ABAC0  addi r3, r10, -0x4540
	ctx.r[3].s64 = ctx.r[10].s64 + -17728;
	// 82688024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268802C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268803C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688044: 4BDDEDDD  bl 0x82466e20
	ctx.lr = 0x82688048;
	sub_82466E20(ctx, base);
	// 82688048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268804C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688058 size=112
    let mut pc: u32 = 0x82688058;
    'dispatch: loop {
        match pc {
            0x82688058 => {
    //   block [0x82688058..0x826880C8)
	// 82688058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268805C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688068: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268806C: 38AAB430  addi r5, r10, -0x4bd0
	ctx.r[5].s64 = ctx.r[10].s64 + -19408;
	// 82688070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688074: 390B1CC0  addi r8, r11, 0x1cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 7360;
	// 82688078: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268807C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 82688080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268808C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688090: 386ABAF0  addi r3, r10, -0x4510
	ctx.r[3].s64 = ctx.r[10].s64 + -17680;
	// 82688094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268809C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826880A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826880A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826880A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826880AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826880B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826880B4: 4BDDED6D  bl 0x82466e20
	ctx.lr = 0x826880B8;
	sub_82466E20(ctx, base);
	// 826880B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826880BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826880C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826880C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826880C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826880C8 size=112
    let mut pc: u32 = 0x826880C8;
    'dispatch: loop {
        match pc {
            0x826880C8 => {
    //   block [0x826880C8..0x82688138)
	// 826880C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826880CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826880D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826880D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826880D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826880DC: 38AAB580  addi r5, r10, -0x4a80
	ctx.r[5].s64 = ctx.r[10].s64 + -19072;
	// 826880E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826880E4: 390B1D08  addi r8, r11, 0x1d08
	ctx.r[8].s64 = ctx.r[11].s64 + 7432;
	// 826880E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826880EC: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826880F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826880F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826880F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826880FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688100: 386ABB20  addi r3, r10, -0x44e0
	ctx.r[3].s64 = ctx.r[10].s64 + -17632;
	// 82688104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268810C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268811C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688124: 4BDDECFD  bl 0x82466e20
	ctx.lr = 0x82688128;
	sub_82466E20(ctx, base);
	// 82688128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268812C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688138 size=108
    let mut pc: u32 = 0x82688138;
    'dispatch: loop {
        match pc {
            0x82688138 => {
    //   block [0x82688138..0x826881A4)
	// 82688138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268813C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688144: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688148: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268814C: 38EB1D50  addi r7, r11, 0x1d50
	ctx.r[7].s64 = ctx.r[11].s64 + 7504;
	// 82688150: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82688154: 388A21D8  addi r4, r10, 0x21d8
	ctx.r[4].s64 = ctx.r[10].s64 + 8664;
	// 82688158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268815C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688160: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82688164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688168: 386ABB50  addi r3, r10, -0x44b0
	ctx.r[3].s64 = ctx.r[10].s64 + -17584;
	// 8268816C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268817C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268818C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82688190: 4BDDEC91  bl 0x82466e20
	ctx.lr = 0x82688194;
	sub_82466E20(ctx, base);
	// 82688194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268819C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826881A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826881A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826881A8 size=112
    let mut pc: u32 = 0x826881A8;
    'dispatch: loop {
        match pc {
            0x826881A8 => {
    //   block [0x826881A8..0x82688218)
	// 826881A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826881AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826881B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826881B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826881B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826881BC: 38AAB4F0  addi r5, r10, -0x4b10
	ctx.r[5].s64 = ctx.r[10].s64 + -19216;
	// 826881C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826881C4: 390B1DB0  addi r8, r11, 0x1db0
	ctx.r[8].s64 = ctx.r[11].s64 + 7600;
	// 826881C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826881CC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826881D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826881D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826881D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826881DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826881E0: 386ABB80  addi r3, r10, -0x4480
	ctx.r[3].s64 = ctx.r[10].s64 + -17536;
	// 826881E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826881E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826881EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826881F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826881F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826881F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826881FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688204: 4BDDEC1D  bl 0x82466e20
	ctx.lr = 0x82688208;
	sub_82466E20(ctx, base);
	// 82688208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268820C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688218 size=112
    let mut pc: u32 = 0x82688218;
    'dispatch: loop {
        match pc {
            0x82688218 => {
    //   block [0x82688218..0x82688288)
	// 82688218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268821C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688228: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268822C: 38AAB550  addi r5, r10, -0x4ab0
	ctx.r[5].s64 = ctx.r[10].s64 + -19120;
	// 82688230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688234: 390B1DC8  addi r8, r11, 0x1dc8
	ctx.r[8].s64 = ctx.r[11].s64 + 7624;
	// 82688238: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268823C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 82688240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268824C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688250: 386ABBB0  addi r3, r10, -0x4450
	ctx.r[3].s64 = ctx.r[10].s64 + -17488;
	// 82688254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268825C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268826C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688274: 4BDDEBAD  bl 0x82466e20
	ctx.lr = 0x82688278;
	sub_82466E20(ctx, base);
	// 82688278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268827C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688288 size=112
    let mut pc: u32 = 0x82688288;
    'dispatch: loop {
        match pc {
            0x82688288 => {
    //   block [0x82688288..0x826882F8)
	// 82688288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268828C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688298: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268829C: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 826882A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826882A4: 390B1DF8  addi r8, r11, 0x1df8
	ctx.r[8].s64 = ctx.r[11].s64 + 7672;
	// 826882A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826882AC: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826882B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826882B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826882B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826882BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826882C0: 386ABBE0  addi r3, r10, -0x4420
	ctx.r[3].s64 = ctx.r[10].s64 + -17440;
	// 826882C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826882C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826882CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826882D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826882D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826882D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826882DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826882E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826882E4: 4BDDEB3D  bl 0x82466e20
	ctx.lr = 0x826882E8;
	sub_82466E20(ctx, base);
	// 826882E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826882EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826882F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826882F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826882F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826882F8 size=112
    let mut pc: u32 = 0x826882F8;
    'dispatch: loop {
        match pc {
            0x826882F8 => {
    //   block [0x826882F8..0x82688368)
	// 826882F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826882FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688304: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688308: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268830C: 38AABBE0  addi r5, r10, -0x4420
	ctx.r[5].s64 = ctx.r[10].s64 + -17440;
	// 82688310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688314: 390B1E58  addi r8, r11, 0x1e58
	ctx.r[8].s64 = ctx.r[11].s64 + 7768;
	// 82688318: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268831C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82688320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688324: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268832C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688330: 386ABC10  addi r3, r10, -0x43f0
	ctx.r[3].s64 = ctx.r[10].s64 + -17392;
	// 82688334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268833C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268834C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688354: 4BDDEACD  bl 0x82466e20
	ctx.lr = 0x82688358;
	sub_82466E20(ctx, base);
	// 82688358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268835C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688368 size=112
    let mut pc: u32 = 0x82688368;
    'dispatch: loop {
        match pc {
            0x82688368 => {
    //   block [0x82688368..0x826883D8)
	// 82688368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268836C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688374: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688378: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268837C: 38AABBE0  addi r5, r10, -0x4420
	ctx.r[5].s64 = ctx.r[10].s64 + -17440;
	// 82688380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688384: 390B1E70  addi r8, r11, 0x1e70
	ctx.r[8].s64 = ctx.r[11].s64 + 7792;
	// 82688388: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268838C: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 82688390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688394: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268839C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826883A0: 386ABC40  addi r3, r10, -0x43c0
	ctx.r[3].s64 = ctx.r[10].s64 + -17344;
	// 826883A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826883A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826883AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826883B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826883B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826883B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826883BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826883C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826883C4: 4BDDEA5D  bl 0x82466e20
	ctx.lr = 0x826883C8;
	sub_82466E20(ctx, base);
	// 826883C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826883CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826883D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826883D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826883D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826883D8 size=112
    let mut pc: u32 = 0x826883D8;
    'dispatch: loop {
        match pc {
            0x826883D8 => {
    //   block [0x826883D8..0x82688448)
	// 826883D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826883DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826883E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826883E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826883E8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826883EC: 38AABBE0  addi r5, r10, -0x4420
	ctx.r[5].s64 = ctx.r[10].s64 + -17440;
	// 826883F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826883F4: 390B1EA0  addi r8, r11, 0x1ea0
	ctx.r[8].s64 = ctx.r[11].s64 + 7840;
	// 826883F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826883FC: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82688400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688404: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268840C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688410: 386ABC70  addi r3, r10, -0x4390
	ctx.r[3].s64 = ctx.r[10].s64 + -17296;
	// 82688414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268841C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268842C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688434: 4BDDE9ED  bl 0x82466e20
	ctx.lr = 0x82688438;
	sub_82466E20(ctx, base);
	// 82688438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268843C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82688448 size=24
    let mut pc: u32 = 0x82688448;
    'dispatch: loop {
        match pc {
            0x82688448 => {
    //   block [0x82688448..0x82688460)
	// 82688448: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268844C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82688450: 394A5088  addi r10, r10, 0x5088
	ctx.r[10].s64 = ctx.r[10].s64 + 20616;
	// 82688454: 816B1944  lwz r11, 0x1944(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6468 as u32) ) } as u64;
	// 82688458: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8268845C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688460 size=112
    let mut pc: u32 = 0x82688460;
    'dispatch: loop {
        match pc {
            0x82688460 => {
    //   block [0x82688460..0x826884D0)
	// 82688460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268846C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82688470: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688474: 392A5EEC  addi r9, r10, 0x5eec
	ctx.r[9].s64 = ctx.r[10].s64 + 24300;
	// 82688478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268847C: 390B5088  addi r8, r11, 0x5088
	ctx.r[8].s64 = ctx.r[11].s64 + 20616;
	// 82688480: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82688484: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 82688488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268848C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82688494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688498: 386ABCA0  addi r3, r10, -0x4360
	ctx.r[3].s64 = ctx.r[10].s64 + -17248;
	// 8268849C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826884A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826884A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826884A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826884AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826884B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826884B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826884B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826884BC: 4BDDE965  bl 0x82466e20
	ctx.lr = 0x826884C0;
	sub_82466E20(ctx, base);
	// 826884C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826884C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826884C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826884CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826884D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826884D0 size=108
    let mut pc: u32 = 0x826884D0;
    'dispatch: loop {
        match pc {
            0x826884D0 => {
    //   block [0x826884D0..0x8268853C)
	// 826884D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826884D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826884D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826884DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826884E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826884E4: 38EB1EB8  addi r7, r11, 0x1eb8
	ctx.r[7].s64 = ctx.r[11].s64 + 7864;
	// 826884E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826884EC: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826884F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826884F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826884F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826884FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688500: 386ABCD0  addi r3, r10, -0x4330
	ctx.r[3].s64 = ctx.r[10].s64 + -17200;
	// 82688504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268850C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268851C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82688528: 4BDDE8F9  bl 0x82466e20
	ctx.lr = 0x8268852C;
	sub_82466E20(ctx, base);
	// 8268852C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688540 size=108
    let mut pc: u32 = 0x82688540;
    'dispatch: loop {
        match pc {
            0x82688540 => {
    //   block [0x82688540..0x826885AC)
	// 82688540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268854C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688554: 38EB1ED0  addi r7, r11, 0x1ed0
	ctx.r[7].s64 = ctx.r[11].s64 + 7888;
	// 82688558: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268855C: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 82688560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688564: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268856C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688570: 386ABD00  addi r3, r10, -0x4300
	ctx.r[3].s64 = ctx.r[10].s64 + -17152;
	// 82688574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268857C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268858C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82688598: 4BDDE889  bl 0x82466e20
	ctx.lr = 0x8268859C;
	sub_82466E20(ctx, base);
	// 8268859C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826885A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826885A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826885A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826885B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826885B0 size=116
    let mut pc: u32 = 0x826885B0;
    'dispatch: loop {
        match pc {
            0x826885B0 => {
    //   block [0x826885B0..0x82688624)
	// 826885B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826885B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826885B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826885BC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826885C0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826885C4: 390B1F1C  addi r8, r11, 0x1f1c
	ctx.r[8].s64 = ctx.r[11].s64 + 7964;
	// 826885C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826885CC: 392A5FB8  addi r9, r10, 0x5fb8
	ctx.r[9].s64 = ctx.r[10].s64 + 24504;
	// 826885D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826885D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826885D8: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 826885DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826885E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826885E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826885E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826885EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826885F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826885F4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826885F8: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826885FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82688600: 386BBD30  addi r3, r11, -0x42d0
	ctx.r[3].s64 = ctx.r[11].s64 + -17104;
	// 82688604: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82688608: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268860C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688610: 4BDDE811  bl 0x82466e20
	ctx.lr = 0x82688614;
	sub_82466E20(ctx, base);
	// 82688614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268861C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688628 size=108
    let mut pc: u32 = 0x82688628;
    'dispatch: loop {
        match pc {
            0x82688628 => {
    //   block [0x82688628..0x82688694)
	// 82688628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268862C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688634: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688638: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268863C: 38EB1F38  addi r7, r11, 0x1f38
	ctx.r[7].s64 = ctx.r[11].s64 + 7992;
	// 82688640: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82688644: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 82688648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268864C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82688654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688658: 386ABD60  addi r3, r10, -0x42a0
	ctx.r[3].s64 = ctx.r[10].s64 + -17056;
	// 8268865C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268866C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268867C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82688680: 4BDDE7A1  bl 0x82466e20
	ctx.lr = 0x82688684;
	sub_82466E20(ctx, base);
	// 82688684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268868C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82688698 size=24
    let mut pc: u32 = 0x82688698;
    'dispatch: loop {
        match pc {
            0x82688698 => {
    //   block [0x82688698..0x826886B0)
	// 82688698: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268869C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 826886A0: 394A50D0  addi r10, r10, 0x50d0
	ctx.r[10].s64 = ctx.r[10].s64 + 20688;
	// 826886A4: 816B1F34  lwz r11, 0x1f34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7988 as u32) ) } as u64;
	// 826886A8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826886AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826886B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826886B0 size=116
    let mut pc: u32 = 0x826886B0;
    'dispatch: loop {
        match pc {
            0x826886B0 => {
    //   block [0x826886B0..0x82688724)
	// 826886B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826886B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826886B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826886BC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826886C0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826886C4: 390B50D0  addi r8, r11, 0x50d0
	ctx.r[8].s64 = ctx.r[11].s64 + 20688;
	// 826886C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826886CC: 392A6014  addi r9, r10, 0x6014
	ctx.r[9].s64 = ctx.r[10].s64 + 24596;
	// 826886D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826886D4: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826886D8: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 826886DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826886E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826886E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826886E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826886EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826886F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826886F4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826886F8: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826886FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82688700: 386BBD90  addi r3, r11, -0x4270
	ctx.r[3].s64 = ctx.r[11].s64 + -17008;
	// 82688704: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82688708: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268870C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688710: 4BDDE711  bl 0x82466e20
	ctx.lr = 0x82688714;
	sub_82466E20(ctx, base);
	// 82688714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268871C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688728 size=112
    let mut pc: u32 = 0x82688728;
    'dispatch: loop {
        match pc {
            0x82688728 => {
    //   block [0x82688728..0x82688798)
	// 82688728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268872C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688734: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688738: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268873C: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82688740: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82688744: 390B1FA0  addi r8, r11, 0x1fa0
	ctx.r[8].s64 = ctx.r[11].s64 + 8096;
	// 82688748: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268874C: 388A2200  addi r4, r10, 0x2200
	ctx.r[4].s64 = ctx.r[10].s64 + 8704;
	// 82688750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688754: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268875C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688760: 386ABDC0  addi r3, r10, -0x4240
	ctx.r[3].s64 = ctx.r[10].s64 + -16960;
	// 82688764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268876C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268877C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688784: 4BDDE69D  bl 0x82466e20
	ctx.lr = 0x82688788;
	sub_82466E20(ctx, base);
	// 82688788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268878C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688798 size=112
    let mut pc: u32 = 0x82688798;
    'dispatch: loop {
        match pc {
            0x82688798 => {
    //   block [0x82688798..0x82688808)
	// 82688798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268879C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826887A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826887A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826887A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826887AC: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 826887B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826887B4: 390B1FB8  addi r8, r11, 0x1fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 8120;
	// 826887B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826887BC: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826887C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826887C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826887C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826887CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826887D0: 386ABDF0  addi r3, r10, -0x4210
	ctx.r[3].s64 = ctx.r[10].s64 + -16912;
	// 826887D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826887D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826887DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826887E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826887E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826887E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826887EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826887F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826887F4: 4BDDE62D  bl 0x82466e20
	ctx.lr = 0x826887F8;
	sub_82466E20(ctx, base);
	// 826887F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826887FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688808 size=108
    let mut pc: u32 = 0x82688808;
    'dispatch: loop {
        match pc {
            0x82688808 => {
    //   block [0x82688808..0x82688874)
	// 82688808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268880C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688814: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688818: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268881C: 38EB1FE8  addi r7, r11, 0x1fe8
	ctx.r[7].s64 = ctx.r[11].s64 + 8168;
	// 82688820: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82688824: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 82688828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268882C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82688834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688838: 386ABE20  addi r3, r10, -0x41e0
	ctx.r[3].s64 = ctx.r[10].s64 + -16864;
	// 8268883C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268884C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268885C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82688860: 4BDDE5C1  bl 0x82466e20
	ctx.lr = 0x82688864;
	sub_82466E20(ctx, base);
	// 82688864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268886C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688878 size=108
    let mut pc: u32 = 0x82688878;
    'dispatch: loop {
        match pc {
            0x82688878 => {
    //   block [0x82688878..0x826888E4)
	// 82688878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268887C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688884: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688888: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268888C: 38EB2030  addi r7, r11, 0x2030
	ctx.r[7].s64 = ctx.r[11].s64 + 8240;
	// 82688890: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82688894: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 82688898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268889C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826888A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826888A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826888A8: 386ABE50  addi r3, r10, -0x41b0
	ctx.r[3].s64 = ctx.r[10].s64 + -16816;
	// 826888AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826888B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826888B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826888B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826888BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826888C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826888C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826888C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826888CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826888D0: 4BDDE551  bl 0x82466e20
	ctx.lr = 0x826888D4;
	sub_82466E20(ctx, base);
	// 826888D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826888D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826888DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826888E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826888E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826888E8 size=112
    let mut pc: u32 = 0x826888E8;
    'dispatch: loop {
        match pc {
            0x826888E8 => {
    //   block [0x826888E8..0x82688958)
	// 826888E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826888EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826888F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826888F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826888F8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826888FC: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82688900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688904: 390B2060  addi r8, r11, 0x2060
	ctx.r[8].s64 = ctx.r[11].s64 + 8288;
	// 82688908: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268890C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 82688910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688914: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688918: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268891C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688920: 386ABE80  addi r3, r10, -0x4180
	ctx.r[3].s64 = ctx.r[10].s64 + -16768;
	// 82688924: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268892C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268893C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688944: 4BDDE4DD  bl 0x82466e20
	ctx.lr = 0x82688948;
	sub_82466E20(ctx, base);
	// 82688948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268894C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688958 size=108
    let mut pc: u32 = 0x82688958;
    'dispatch: loop {
        match pc {
            0x82688958 => {
    //   block [0x82688958..0x826889C4)
	// 82688958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268895C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688964: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688968: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8268896C: 38EB2090  addi r7, r11, 0x2090
	ctx.r[7].s64 = ctx.r[11].s64 + 8336;
	// 82688970: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82688974: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 82688978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268897C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688980: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82688984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688988: 386ABEB0  addi r3, r10, -0x4150
	ctx.r[3].s64 = ctx.r[10].s64 + -16720;
	// 8268898C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268899C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826889A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826889A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826889A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826889AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826889B0: 4BDDE471  bl 0x82466e20
	ctx.lr = 0x826889B4;
	sub_82466E20(ctx, base);
	// 826889B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826889B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826889BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826889C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826889C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826889C8 size=108
    let mut pc: u32 = 0x826889C8;
    'dispatch: loop {
        match pc {
            0x826889C8 => {
    //   block [0x826889C8..0x82688A34)
	// 826889C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826889CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826889D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826889D4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826889D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826889DC: 38EB20F0  addi r7, r11, 0x20f0
	ctx.r[7].s64 = ctx.r[11].s64 + 8432;
	// 826889E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826889E4: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 826889E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826889EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826889F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826889F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826889F8: 386ABEE0  addi r3, r10, -0x4120
	ctx.r[3].s64 = ctx.r[10].s64 + -16672;
	// 826889FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688A1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82688A20: 4BDDE401  bl 0x82466e20
	ctx.lr = 0x82688A24;
	sub_82466E20(ctx, base);
	// 82688A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688A38 size=116
    let mut pc: u32 = 0x82688A38;
    'dispatch: loop {
        match pc {
            0x82688A38 => {
    //   block [0x82688A38..0x82688AAC)
	// 82688A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688A44: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82688A48: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82688A4C: 390A2138  addi r8, r10, 0x2138
	ctx.r[8].s64 = ctx.r[10].s64 + 8504;
	// 82688A50: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688A54: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82688A58: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82688A5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688A60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82688A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82688A6C: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82688A70: 396B6050  addi r11, r11, 0x6050
	ctx.r[11].s64 = ctx.r[11].s64 + 24656;
	// 82688A74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688A78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688A7C: 386ABF10  addi r3, r10, -0x40f0
	ctx.r[3].s64 = ctx.r[10].s64 + -16624;
	// 82688A80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82688A84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688A88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82688A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688A98: 4BDDE389  bl 0x82466e20
	ctx.lr = 0x82688A9C;
	sub_82466E20(ctx, base);
	// 82688A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688AB0 size=112
    let mut pc: u32 = 0x82688AB0;
    'dispatch: loop {
        match pc {
            0x82688AB0 => {
    //   block [0x82688AB0..0x82688B20)
	// 82688AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688ABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688AC0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688AC4: 38AABF70  addi r5, r10, -0x4090
	ctx.r[5].s64 = ctx.r[10].s64 + -16528;
	// 82688AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688ACC: 390B21C8  addi r8, r11, 0x21c8
	ctx.r[8].s64 = ctx.r[11].s64 + 8648;
	// 82688AD0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82688AD4: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82688AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688ADC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82688AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688AE8: 386ABF40  addi r3, r10, -0x40c0
	ctx.r[3].s64 = ctx.r[10].s64 + -16576;
	// 82688AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688B0C: 4BDDE315  bl 0x82466e20
	ctx.lr = 0x82688B10;
	sub_82466E20(ctx, base);
	// 82688B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688B20 size=100
    let mut pc: u32 = 0x82688B20;
    'dispatch: loop {
        match pc {
            0x82688B20 => {
    //   block [0x82688B20..0x82688B84)
	// 82688B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688B2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688B34: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 82688B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688B40: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 82688B44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688B54: 386ABF70  addi r3, r10, -0x4090
	ctx.r[3].s64 = ctx.r[10].s64 + -16528;
	// 82688B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688B5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688B60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82688B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82688B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688B70: 4BDDE2B1  bl 0x82466e20
	ctx.lr = 0x82688B74;
	sub_82466E20(ctx, base);
	// 82688B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82688B88 size=24
    let mut pc: u32 = 0x82688B88;
    'dispatch: loop {
        match pc {
            0x82688B88 => {
    //   block [0x82688B88..0x82688BA0)
	// 82688B88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688B8C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82688B90: 394A5190  addi r10, r10, 0x5190
	ctx.r[10].s64 = ctx.r[10].s64 + 20880;
	// 82688B94: 816B2240  lwz r11, 0x2240(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8768 as u32) ) } as u64;
	// 82688B98: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82688B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688BA0 size=116
    let mut pc: u32 = 0x82688BA0;
    'dispatch: loop {
        match pc {
            0x82688BA0 => {
    //   block [0x82688BA0..0x82688C14)
	// 82688BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688BAC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688BB0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82688BB4: 390B5190  addi r8, r11, 0x5190
	ctx.r[8].s64 = ctx.r[11].s64 + 20880;
	// 82688BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688BBC: 392A6094  addi r9, r10, 0x6094
	ctx.r[9].s64 = ctx.r[10].s64 + 24724;
	// 82688BC0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688BC4: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82688BC8: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82688BCC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82688BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688BD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688BE4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82688BE8: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82688BEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82688BF0: 386BBFA0  addi r3, r11, -0x4060
	ctx.r[3].s64 = ctx.r[11].s64 + -16480;
	// 82688BF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82688BF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688C00: 4BDDE221  bl 0x82466e20
	ctx.lr = 0x82688C04;
	sub_82466E20(ctx, base);
	// 82688C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688C18 size=112
    let mut pc: u32 = 0x82688C18;
    'dispatch: loop {
        match pc {
            0x82688C18 => {
    //   block [0x82688C18..0x82688C88)
	// 82688C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688C24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688C28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688C2C: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82688C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688C34: 390B2248  addi r8, r11, 0x2248
	ctx.r[8].s64 = ctx.r[11].s64 + 8776;
	// 82688C38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82688C3C: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 82688C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688C44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82688C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688C50: 386ABFD0  addi r3, r10, -0x4030
	ctx.r[3].s64 = ctx.r[10].s64 + -16432;
	// 82688C54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688C74: 4BDDE1AD  bl 0x82466e20
	ctx.lr = 0x82688C78;
	sub_82466E20(ctx, base);
	// 82688C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688C88 size=112
    let mut pc: u32 = 0x82688C88;
    'dispatch: loop {
        match pc {
            0x82688C88 => {
    //   block [0x82688C88..0x82688CF8)
	// 82688C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688C94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688C98: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688C9C: 38AABF10  addi r5, r10, -0x40f0
	ctx.r[5].s64 = ctx.r[10].s64 + -16624;
	// 82688CA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82688CA4: 390B2290  addi r8, r11, 0x2290
	ctx.r[8].s64 = ctx.r[11].s64 + 8848;
	// 82688CA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82688CAC: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 82688CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688CB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688CB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82688CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688CC0: 386AC000  addi r3, r10, -0x4000
	ctx.r[3].s64 = ctx.r[10].s64 + -16384;
	// 82688CC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688CE4: 4BDDE13D  bl 0x82466e20
	ctx.lr = 0x82688CE8;
	sub_82466E20(ctx, base);
	// 82688CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688CF8 size=108
    let mut pc: u32 = 0x82688CF8;
    'dispatch: loop {
        match pc {
            0x82688CF8 => {
    //   block [0x82688CF8..0x82688D64)
	// 82688CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688D04: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688D08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82688D0C: 38EB22F0  addi r7, r11, 0x22f0
	ctx.r[7].s64 = ctx.r[11].s64 + 8944;
	// 82688D10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82688D14: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 82688D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688D1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688D20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82688D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688D28: 386AC030  addi r3, r10, -0x3fd0
	ctx.r[3].s64 = ctx.r[10].s64 + -16336;
	// 82688D2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82688D50: 4BDDE0D1  bl 0x82466e20
	ctx.lr = 0x82688D54;
	sub_82466E20(ctx, base);
	// 82688D54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688D68 size=108
    let mut pc: u32 = 0x82688D68;
    'dispatch: loop {
        match pc {
            0x82688D68 => {
    //   block [0x82688D68..0x82688DD4)
	// 82688D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688D74: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688D78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82688D7C: 38EB2338  addi r7, r11, 0x2338
	ctx.r[7].s64 = ctx.r[11].s64 + 9016;
	// 82688D80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82688D84: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 82688D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688D8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688D90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82688D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688D98: 386AC060  addi r3, r10, -0x3fa0
	ctx.r[3].s64 = ctx.r[10].s64 + -16288;
	// 82688D9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82688DC0: 4BDDE061  bl 0x82466e20
	ctx.lr = 0x82688DC4;
	sub_82466E20(ctx, base);
	// 82688DC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688DD8 size=112
    let mut pc: u32 = 0x82688DD8;
    'dispatch: loop {
        match pc {
            0x82688DD8 => {
    //   block [0x82688DD8..0x82688E48)
	// 82688DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688DE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688DE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688DEC: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82688DF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688DF4: 390B2380  addi r8, r11, 0x2380
	ctx.r[8].s64 = ctx.r[11].s64 + 9088;
	// 82688DF8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82688DFC: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82688E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688E04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688E08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82688E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688E10: 386AC090  addi r3, r10, -0x3f70
	ctx.r[3].s64 = ctx.r[10].s64 + -16240;
	// 82688E14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688E34: 4BDDDFED  bl 0x82466e20
	ctx.lr = 0x82688E38;
	sub_82466E20(ctx, base);
	// 82688E38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688E48 size=112
    let mut pc: u32 = 0x82688E48;
    'dispatch: loop {
        match pc {
            0x82688E48 => {
    //   block [0x82688E48..0x82688EB8)
	// 82688E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688E54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688E58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688E5C: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82688E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688E64: 390B2428  addi r8, r11, 0x2428
	ctx.r[8].s64 = ctx.r[11].s64 + 9256;
	// 82688E68: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82688E6C: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82688E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688E74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82688E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688E80: 386AC0C0  addi r3, r10, -0x3f40
	ctx.r[3].s64 = ctx.r[10].s64 + -16192;
	// 82688E84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688EA4: 4BDDDF7D  bl 0x82466e20
	ctx.lr = 0x82688EA8;
	sub_82466E20(ctx, base);
	// 82688EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688EB8 size=108
    let mut pc: u32 = 0x82688EB8;
    'dispatch: loop {
        match pc {
            0x82688EB8 => {
    //   block [0x82688EB8..0x82688F24)
	// 82688EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688EC4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688EC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82688ECC: 38EB2470  addi r7, r11, 0x2470
	ctx.r[7].s64 = ctx.r[11].s64 + 9328;
	// 82688ED0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82688ED4: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 82688ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688EDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688EE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82688EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688EE8: 386AC0F0  addi r3, r10, -0x3f10
	ctx.r[3].s64 = ctx.r[10].s64 + -16144;
	// 82688EEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688F0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82688F10: 4BDDDF11  bl 0x82466e20
	ctx.lr = 0x82688F14;
	sub_82466E20(ctx, base);
	// 82688F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688F28 size=108
    let mut pc: u32 = 0x82688F28;
    'dispatch: loop {
        match pc {
            0x82688F28 => {
    //   block [0x82688F28..0x82688F94)
	// 82688F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688F34: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688F38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82688F3C: 38EB24A0  addi r7, r11, 0x24a0
	ctx.r[7].s64 = ctx.r[11].s64 + 9376;
	// 82688F40: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82688F44: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 82688F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688F4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688F50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82688F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688F58: 386AC120  addi r3, r10, -0x3ee0
	ctx.r[3].s64 = ctx.r[10].s64 + -16096;
	// 82688F5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82688F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82688F80: 4BDDDEA1  bl 0x82466e20
	ctx.lr = 0x82688F84;
	sub_82466E20(ctx, base);
	// 82688F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82688F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82688F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82688F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82688F98 size=112
    let mut pc: u32 = 0x82688F98;
    'dispatch: loop {
        match pc {
            0x82688F98 => {
    //   block [0x82688F98..0x82689008)
	// 82688F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82688F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82688FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82688FA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688FA8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82688FAC: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82688FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82688FB4: 390B2530  addi r8, r11, 0x2530
	ctx.r[8].s64 = ctx.r[11].s64 + 9520;
	// 82688FB8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82688FBC: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82688FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82688FC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82688FC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82688FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82688FD0: 386AC150  addi r3, r10, -0x3eb0
	ctx.r[3].s64 = ctx.r[10].s64 + -16048;
	// 82688FD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82688FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82688FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82688FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82688FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82688FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82688FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82688FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82688FF4: 4BDDDE2D  bl 0x82466e20
	ctx.lr = 0x82688FF8;
	sub_82466E20(ctx, base);
	// 82688FF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82688FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689008 size=112
    let mut pc: u32 = 0x82689008;
    'dispatch: loop {
        match pc {
            0x82689008 => {
    //   block [0x82689008..0x82689078)
	// 82689008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268900C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689014: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689018: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268901C: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82689020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689024: 390B25C0  addi r8, r11, 0x25c0
	ctx.r[8].s64 = ctx.r[11].s64 + 9664;
	// 82689028: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8268902C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 82689030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689034: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268903C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689040: 386AC180  addi r3, r10, -0x3e80
	ctx.r[3].s64 = ctx.r[10].s64 + -16000;
	// 82689044: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268904C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268905C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689064: 4BDDDDBD  bl 0x82466e20
	ctx.lr = 0x82689068;
	sub_82466E20(ctx, base);
	// 82689068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268906C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689078 size=100
    let mut pc: u32 = 0x82689078;
    'dispatch: loop {
        match pc {
            0x82689078 => {
    //   block [0x82689078..0x826890DC)
	// 82689078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268907C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268908C: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82689090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689098: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8268909C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826890A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826890A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826890A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826890AC: 386AC1B0  addi r3, r10, -0x3e50
	ctx.r[3].s64 = ctx.r[10].s64 + -15952;
	// 826890B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826890B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826890B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826890BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826890C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826890C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826890C8: 4BDDDD59  bl 0x82466e20
	ctx.lr = 0x826890CC;
	sub_82466E20(ctx, base);
	// 826890CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826890D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826890D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826890D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826890E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826890E0 size=112
    let mut pc: u32 = 0x826890E0;
    'dispatch: loop {
        match pc {
            0x826890E0 => {
    //   block [0x826890E0..0x82689150)
	// 826890E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826890E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826890E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826890EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826890F0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826890F4: 38AABD90  addi r5, r10, -0x4270
	ctx.r[5].s64 = ctx.r[10].s64 + -17008;
	// 826890F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826890FC: 390B2680  addi r8, r11, 0x2680
	ctx.r[8].s64 = ctx.r[11].s64 + 9856;
	// 82689100: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82689104: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 82689108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268910C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82689114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689118: 386AC1E0  addi r3, r10, -0x3e20
	ctx.r[3].s64 = ctx.r[10].s64 + -15904;
	// 8268911C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268912C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268913C: 4BDDDCE5  bl 0x82466e20
	ctx.lr = 0x82689140;
	sub_82466E20(ctx, base);
	// 82689140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268914C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689150 size=112
    let mut pc: u32 = 0x82689150;
    'dispatch: loop {
        match pc {
            0x82689150 => {
    //   block [0x82689150..0x826891C0)
	// 82689150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268915C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689160: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689164: 38AABBE0  addi r5, r10, -0x4420
	ctx.r[5].s64 = ctx.r[10].s64 + -17440;
	// 82689168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268916C: 390B26B0  addi r8, r11, 0x26b0
	ctx.r[8].s64 = ctx.r[11].s64 + 9904;
	// 82689170: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82689174: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 82689178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268917C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82689184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689188: 386AC210  addi r3, r10, -0x3df0
	ctx.r[3].s64 = ctx.r[10].s64 + -15856;
	// 8268918C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268919C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826891A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826891A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826891A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826891AC: 4BDDDC75  bl 0x82466e20
	ctx.lr = 0x826891B0;
	sub_82466E20(ctx, base);
	// 826891B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826891B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826891B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826891BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826891C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826891C0 size=108
    let mut pc: u32 = 0x826891C0;
    'dispatch: loop {
        match pc {
            0x826891C0 => {
    //   block [0x826891C0..0x8268922C)
	// 826891C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826891C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826891C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826891CC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826891D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826891D4: 38EB26C8  addi r7, r11, 0x26c8
	ctx.r[7].s64 = ctx.r[11].s64 + 9928;
	// 826891D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826891DC: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 826891E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826891E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826891E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826891EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826891F0: 386AC240  addi r3, r10, -0x3dc0
	ctx.r[3].s64 = ctx.r[10].s64 + -15808;
	// 826891F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826891F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826891FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268920C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82689218: 4BDDDC09  bl 0x82466e20
	ctx.lr = 0x8268921C;
	sub_82466E20(ctx, base);
	// 8268921C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689230 size=112
    let mut pc: u32 = 0x82689230;
    'dispatch: loop {
        match pc {
            0x82689230 => {
    //   block [0x82689230..0x826892A0)
	// 82689230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268923C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689240: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689244: 38AAC1B0  addi r5, r10, -0x3e50
	ctx.r[5].s64 = ctx.r[10].s64 + -15952;
	// 82689248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268924C: 390B26F8  addi r8, r11, 0x26f8
	ctx.r[8].s64 = ctx.r[11].s64 + 9976;
	// 82689250: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82689254: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 82689258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268925C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82689264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689268: 386AC270  addi r3, r10, -0x3d90
	ctx.r[3].s64 = ctx.r[10].s64 + -15760;
	// 8268926C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268927C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268928C: 4BDDDB95  bl 0x82466e20
	ctx.lr = 0x82689290;
	sub_82466E20(ctx, base);
	// 82689290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268929C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826892A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826892A0 size=108
    let mut pc: u32 = 0x826892A0;
    'dispatch: loop {
        match pc {
            0x826892A0 => {
    //   block [0x826892A0..0x8268930C)
	// 826892A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826892A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826892A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826892AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826892B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826892B4: 38EB2770  addi r7, r11, 0x2770
	ctx.r[7].s64 = ctx.r[11].s64 + 10096;
	// 826892B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826892BC: 388A23DC  addi r4, r10, 0x23dc
	ctx.r[4].s64 = ctx.r[10].s64 + 9180;
	// 826892C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826892C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826892C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826892CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826892D0: 386AC2A0  addi r3, r10, -0x3d60
	ctx.r[3].s64 = ctx.r[10].s64 + -15712;
	// 826892D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826892D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826892DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826892E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826892E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826892E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826892EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826892F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826892F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826892F8: 4BDDDB29  bl 0x82466e20
	ctx.lr = 0x826892FC;
	sub_82466E20(ctx, base);
	// 826892FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689310 size=108
    let mut pc: u32 = 0x82689310;
    'dispatch: loop {
        match pc {
            0x82689310 => {
    //   block [0x82689310..0x8268937C)
	// 82689310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268931C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689320: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82689324: 38EB27B8  addi r7, r11, 0x27b8
	ctx.r[7].s64 = ctx.r[11].s64 + 10168;
	// 82689328: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268932C: 388A2400  addi r4, r10, 0x2400
	ctx.r[4].s64 = ctx.r[10].s64 + 9216;
	// 82689330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689334: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268933C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689340: 386AC2D0  addi r3, r10, -0x3d30
	ctx.r[3].s64 = ctx.r[10].s64 + -15664;
	// 82689344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82689348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268934C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268935C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82689368: 4BDDDAB9  bl 0x82466e20
	ctx.lr = 0x8268936C;
	sub_82466E20(ctx, base);
	// 8268936C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689380 size=112
    let mut pc: u32 = 0x82689380;
    'dispatch: loop {
        match pc {
            0x82689380 => {
    //   block [0x82689380..0x826893F0)
	// 82689380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268938C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689390: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689394: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82689398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268939C: 390B27E8  addi r8, r11, 0x27e8
	ctx.r[8].s64 = ctx.r[11].s64 + 10216;
	// 826893A0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826893A4: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826893A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826893AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826893B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826893B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826893B8: 386AC300  addi r3, r10, -0x3d00
	ctx.r[3].s64 = ctx.r[10].s64 + -15616;
	// 826893BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826893C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826893C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826893C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826893CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826893D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826893D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826893D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826893DC: 4BDDDA45  bl 0x82466e20
	ctx.lr = 0x826893E0;
	sub_82466E20(ctx, base);
	// 826893E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826893E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826893E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826893EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826893F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826893F0 size=108
    let mut pc: u32 = 0x826893F0;
    'dispatch: loop {
        match pc {
            0x826893F0 => {
    //   block [0x826893F0..0x8268945C)
	// 826893F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826893F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826893F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826893FC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689404: 38EB2878  addi r7, r11, 0x2878
	ctx.r[7].s64 = ctx.r[11].s64 + 10360;
	// 82689408: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8268940C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82689410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689414: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268941C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689420: 386AC330  addi r3, r10, -0x3cd0
	ctx.r[3].s64 = ctx.r[10].s64 + -15568;
	// 82689424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82689428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268942C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268943C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82689448: 4BDDD9D9  bl 0x82466e20
	ctx.lr = 0x8268944C;
	sub_82466E20(ctx, base);
	// 8268944C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689460 size=112
    let mut pc: u32 = 0x82689460;
    'dispatch: loop {
        match pc {
            0x82689460 => {
    //   block [0x82689460..0x826894D0)
	// 82689460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268946C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689470: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689474: 38AAC1B0  addi r5, r10, -0x3e50
	ctx.r[5].s64 = ctx.r[10].s64 + -15952;
	// 82689478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268947C: 390B2908  addi r8, r11, 0x2908
	ctx.r[8].s64 = ctx.r[11].s64 + 10504;
	// 82689480: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82689484: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 82689488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268948C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82689494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689498: 386AC360  addi r3, r10, -0x3ca0
	ctx.r[3].s64 = ctx.r[10].s64 + -15520;
	// 8268949C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826894A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826894A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826894A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826894AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826894B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826894B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826894B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826894BC: 4BDDD965  bl 0x82466e20
	ctx.lr = 0x826894C0;
	sub_82466E20(ctx, base);
	// 826894C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826894C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826894C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826894CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826894D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826894D0 size=108
    let mut pc: u32 = 0x826894D0;
    'dispatch: loop {
        match pc {
            0x826894D0 => {
    //   block [0x826894D0..0x8268953C)
	// 826894D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826894D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826894D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826894DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826894E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826894E4: 38EB29C8  addi r7, r11, 0x29c8
	ctx.r[7].s64 = ctx.r[11].s64 + 10696;
	// 826894E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826894EC: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826894F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826894F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826894F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826894FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689500: 386AC390  addi r3, r10, -0x3c70
	ctx.r[3].s64 = ctx.r[10].s64 + -15472;
	// 82689504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82689508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268950C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268951C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82689528: 4BDDD8F9  bl 0x82466e20
	ctx.lr = 0x8268952C;
	sub_82466E20(ctx, base);
	// 8268952C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689540 size=112
    let mut pc: u32 = 0x82689540;
    'dispatch: loop {
        match pc {
            0x82689540 => {
    //   block [0x82689540..0x826895B0)
	// 82689540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268954C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689550: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689554: 38AAC1B0  addi r5, r10, -0x3e50
	ctx.r[5].s64 = ctx.r[10].s64 + -15952;
	// 82689558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268955C: 390B2A10  addi r8, r11, 0x2a10
	ctx.r[8].s64 = ctx.r[11].s64 + 10768;
	// 82689560: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82689564: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 82689568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268956C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82689574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689578: 386AC3C0  addi r3, r10, -0x3c40
	ctx.r[3].s64 = ctx.r[10].s64 + -15424;
	// 8268957C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268958C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268959C: 4BDDD885  bl 0x82466e20
	ctx.lr = 0x826895A0;
	sub_82466E20(ctx, base);
	// 826895A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826895A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826895A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826895AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826895B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826895B0 size=112
    let mut pc: u32 = 0x826895B0;
    'dispatch: loop {
        match pc {
            0x826895B0 => {
    //   block [0x826895B0..0x82689620)
	// 826895B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826895B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826895B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826895BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826895C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826895C4: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 826895C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826895CC: 390B2A70  addi r8, r11, 0x2a70
	ctx.r[8].s64 = ctx.r[11].s64 + 10864;
	// 826895D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826895D4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826895D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826895DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826895E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826895E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826895E8: 386AC3F0  addi r3, r10, -0x3c10
	ctx.r[3].s64 = ctx.r[10].s64 + -15376;
	// 826895EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826895F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826895F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826895F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826895FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268960C: 4BDDD815  bl 0x82466e20
	ctx.lr = 0x82689610;
	sub_82466E20(ctx, base);
	// 82689610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268961C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689620 size=108
    let mut pc: u32 = 0x82689620;
    'dispatch: loop {
        match pc {
            0x82689620 => {
    //   block [0x82689620..0x8268968C)
	// 82689620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268962C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689634: 38EB2A88  addi r7, r11, 0x2a88
	ctx.r[7].s64 = ctx.r[11].s64 + 10888;
	// 82689638: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8268963C: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82689640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689644: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268964C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689650: 386AC420  addi r3, r10, -0x3be0
	ctx.r[3].s64 = ctx.r[10].s64 + -15328;
	// 82689654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82689658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268965C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268966C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82689678: 4BDDD7A9  bl 0x82466e20
	ctx.lr = 0x8268967C;
	sub_82466E20(ctx, base);
	// 8268967C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689690 size=112
    let mut pc: u32 = 0x82689690;
    'dispatch: loop {
        match pc {
            0x82689690 => {
    //   block [0x82689690..0x82689700)
	// 82689690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268969C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826896A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826896A4: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 826896A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826896AC: 390B2B00  addi r8, r11, 0x2b00
	ctx.r[8].s64 = ctx.r[11].s64 + 11008;
	// 826896B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826896B4: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 826896B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826896BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826896C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826896C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826896C8: 386AC450  addi r3, r10, -0x3bb0
	ctx.r[3].s64 = ctx.r[10].s64 + -15280;
	// 826896CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826896D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826896D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826896D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826896DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826896E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826896E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826896E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826896EC: 4BDDD735  bl 0x82466e20
	ctx.lr = 0x826896F0;
	sub_82466E20(ctx, base);
	// 826896F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826896F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826896F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826896FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689700 size=104
    let mut pc: u32 = 0x82689700;
    'dispatch: loop {
        match pc {
            0x82689700 => {
    //   block [0x82689700..0x82689768)
	// 82689700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268970C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82689710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689714: 392A60D0  addi r9, r10, 0x60d0
	ctx.r[9].s64 = ctx.r[10].s64 + 24784;
	// 82689718: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268971C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689720: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 82689724: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268972C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689734: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82689738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268973C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82689744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268974C: 386AC480  addi r3, r10, -0x3b80
	ctx.r[3].s64 = ctx.r[10].s64 + -15232;
	// 82689750: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82689754: 4BDDD6CD  bl 0x82466e20
	ctx.lr = 0x82689758;
	sub_82466E20(ctx, base);
	// 82689758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268975C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689768 size=112
    let mut pc: u32 = 0x82689768;
    'dispatch: loop {
        match pc {
            0x82689768 => {
    //   block [0x82689768..0x826897D8)
	// 82689768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268976C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689774: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689778: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268977C: 38AAC480  addi r5, r10, -0x3b80
	ctx.r[5].s64 = ctx.r[10].s64 + -15232;
	// 82689780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689784: 390B2B30  addi r8, r11, 0x2b30
	ctx.r[8].s64 = ctx.r[11].s64 + 11056;
	// 82689788: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268978C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82689790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689794: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268979C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826897A0: 386AC4B0  addi r3, r10, -0x3b50
	ctx.r[3].s64 = ctx.r[10].s64 + -15184;
	// 826897A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826897A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826897AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826897B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826897B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826897B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826897BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826897C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826897C4: 4BDDD65D  bl 0x82466e20
	ctx.lr = 0x826897C8;
	sub_82466E20(ctx, base);
	// 826897C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826897CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826897D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826897D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826897D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826897D8 size=112
    let mut pc: u32 = 0x826897D8;
    'dispatch: loop {
        match pc {
            0x826897D8 => {
    //   block [0x826897D8..0x82689848)
	// 826897D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826897DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826897E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826897E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826897E8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826897EC: 38AAC4B0  addi r5, r10, -0x3b50
	ctx.r[5].s64 = ctx.r[10].s64 + -15184;
	// 826897F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826897F4: 390B2B60  addi r8, r11, 0x2b60
	ctx.r[8].s64 = ctx.r[11].s64 + 11104;
	// 826897F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826897FC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82689800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268980C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689810: 386AC4E0  addi r3, r10, -0x3b20
	ctx.r[3].s64 = ctx.r[10].s64 + -15136;
	// 82689814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268981C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268982C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689834: 4BDDD5ED  bl 0x82466e20
	ctx.lr = 0x82689838;
	sub_82466E20(ctx, base);
	// 82689838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268983C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689848 size=112
    let mut pc: u32 = 0x82689848;
    'dispatch: loop {
        match pc {
            0x82689848 => {
    //   block [0x82689848..0x826898B8)
	// 82689848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268984C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689854: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689858: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268985C: 38AAC4B0  addi r5, r10, -0x3b50
	ctx.r[5].s64 = ctx.r[10].s64 + -15184;
	// 82689860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689864: 390B2BC0  addi r8, r11, 0x2bc0
	ctx.r[8].s64 = ctx.r[11].s64 + 11200;
	// 82689868: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268986C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82689870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689874: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268987C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689880: 386AC510  addi r3, r10, -0x3af0
	ctx.r[3].s64 = ctx.r[10].s64 + -15088;
	// 82689884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268988C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268989C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826898A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826898A4: 4BDDD57D  bl 0x82466e20
	ctx.lr = 0x826898A8;
	sub_82466E20(ctx, base);
	// 826898A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826898AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826898B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826898B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826898B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826898B8 size=112
    let mut pc: u32 = 0x826898B8;
    'dispatch: loop {
        match pc {
            0x826898B8 => {
    //   block [0x826898B8..0x82689928)
	// 826898B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826898BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826898C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826898C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826898C8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826898CC: 38AAC4B0  addi r5, r10, -0x3b50
	ctx.r[5].s64 = ctx.r[10].s64 + -15184;
	// 826898D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826898D4: 390B2BF0  addi r8, r11, 0x2bf0
	ctx.r[8].s64 = ctx.r[11].s64 + 11248;
	// 826898D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826898DC: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826898E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826898E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826898E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826898EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826898F0: 386AC540  addi r3, r10, -0x3ac0
	ctx.r[3].s64 = ctx.r[10].s64 + -15040;
	// 826898F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826898F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826898FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268990C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689914: 4BDDD50D  bl 0x82466e20
	ctx.lr = 0x82689918;
	sub_82466E20(ctx, base);
	// 82689918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268991C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689928 size=112
    let mut pc: u32 = 0x82689928;
    'dispatch: loop {
        match pc {
            0x82689928 => {
    //   block [0x82689928..0x82689998)
	// 82689928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268992C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689938: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268993C: 38AABD30  addi r5, r10, -0x42d0
	ctx.r[5].s64 = ctx.r[10].s64 + -17104;
	// 82689940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689944: 390B2C38  addi r8, r11, 0x2c38
	ctx.r[8].s64 = ctx.r[11].s64 + 11320;
	// 82689948: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8268994C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82689950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268995C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689960: 386AC570  addi r3, r10, -0x3a90
	ctx.r[3].s64 = ctx.r[10].s64 + -14992;
	// 82689964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268996C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268997C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689984: 4BDDD49D  bl 0x82466e20
	ctx.lr = 0x82689988;
	sub_82466E20(ctx, base);
	// 82689988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268998C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689998 size=116
    let mut pc: u32 = 0x82689998;
    'dispatch: loop {
        match pc {
            0x82689998 => {
    //   block [0x82689998..0x82689A0C)
	// 82689998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268999C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826899A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826899A4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 826899A8: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 826899AC: 390A2CC8  addi r8, r10, 0x2cc8
	ctx.r[8].s64 = ctx.r[10].s64 + 11464;
	// 826899B0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826899B4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826899B8: 38AACA20  addi r5, r10, -0x35e0
	ctx.r[5].s64 = ctx.r[10].s64 + -13792;
	// 826899BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826899C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826899C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826899C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826899CC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826899D0: 396B60E8  addi r11, r11, 0x60e8
	ctx.r[11].s64 = ctx.r[11].s64 + 24808;
	// 826899D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826899D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826899DC: 386AC5A0  addi r3, r10, -0x3a60
	ctx.r[3].s64 = ctx.r[10].s64 + -14944;
	// 826899E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826899E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826899E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826899EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826899F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826899F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826899F8: 4BDDD429  bl 0x82466e20
	ctx.lr = 0x826899FC;
	sub_82466E20(ctx, base);
	// 826899FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689A10 size=100
    let mut pc: u32 = 0x82689A10;
    'dispatch: loop {
        match pc {
            0x82689A10 => {
    //   block [0x82689A10..0x82689A74)
	// 82689A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689A1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689A24: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 82689A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689A30: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82689A34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689A44: 386AC5D0  addi r3, r10, -0x3a30
	ctx.r[3].s64 = ctx.r[10].s64 + -14896;
	// 82689A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689A4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689A50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82689A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689A58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82689A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689A60: 4BDDD3C1  bl 0x82466e20
	ctx.lr = 0x82689A64;
	sub_82466E20(ctx, base);
	// 82689A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689A78 size=100
    let mut pc: u32 = 0x82689A78;
    'dispatch: loop {
        match pc {
            0x82689A78 => {
    //   block [0x82689A78..0x82689ADC)
	// 82689A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689A84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689A8C: 38AAC660  addi r5, r10, -0x39a0
	ctx.r[5].s64 = ctx.r[10].s64 + -14752;
	// 82689A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689A98: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82689A9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689AAC: 386AC600  addi r3, r10, -0x3a00
	ctx.r[3].s64 = ctx.r[10].s64 + -14848;
	// 82689AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689AB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689AB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82689ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689AC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82689AC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689AC8: 4BDDD359  bl 0x82466e20
	ctx.lr = 0x82689ACC;
	sub_82466E20(ctx, base);
	// 82689ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689AE0 size=100
    let mut pc: u32 = 0x82689AE0;
    'dispatch: loop {
        match pc {
            0x82689AE0 => {
    //   block [0x82689AE0..0x82689B44)
	// 82689AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689AEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689AF4: 38AAC5A0  addi r5, r10, -0x3a60
	ctx.r[5].s64 = ctx.r[10].s64 + -14944;
	// 82689AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689B00: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82689B04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689B14: 386AC630  addi r3, r10, -0x39d0
	ctx.r[3].s64 = ctx.r[10].s64 + -14800;
	// 82689B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689B20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82689B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689B28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82689B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689B30: 4BDDD2F1  bl 0x82466e20
	ctx.lr = 0x82689B34;
	sub_82466E20(ctx, base);
	// 82689B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689B48 size=104
    let mut pc: u32 = 0x82689B48;
    'dispatch: loop {
        match pc {
            0x82689B48 => {
    //   block [0x82689B48..0x82689BB0)
	// 82689B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689B54: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82689B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689B5C: 392A6154  addi r9, r10, 0x6154
	ctx.r[9].s64 = ctx.r[10].s64 + 24916;
	// 82689B60: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689B68: 38AAC5D0  addi r5, r10, -0x3a30
	ctx.r[5].s64 = ctx.r[10].s64 + -14896;
	// 82689B6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689B7C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82689B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689B84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689B88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82689B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689B90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82689B94: 386AC660  addi r3, r10, -0x39a0
	ctx.r[3].s64 = ctx.r[10].s64 + -14752;
	// 82689B98: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82689B9C: 4BDDD285  bl 0x82466e20
	ctx.lr = 0x82689BA0;
	sub_82466E20(ctx, base);
	// 82689BA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689BB0 size=108
    let mut pc: u32 = 0x82689BB0;
    'dispatch: loop {
        match pc {
            0x82689BB0 => {
    //   block [0x82689BB0..0x82689C1C)
	// 82689BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689BB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689BBC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689BC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689BC4: 38EB2E7C  addi r7, r11, 0x2e7c
	ctx.r[7].s64 = ctx.r[11].s64 + 11900;
	// 82689BC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82689BCC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82689BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689BD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689BD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82689BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689BE0: 386AC690  addi r3, r10, -0x3970
	ctx.r[3].s64 = ctx.r[10].s64 + -14704;
	// 82689BE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82689BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689C04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82689C08: 4BDDD219  bl 0x82466e20
	ctx.lr = 0x82689C0C;
	sub_82466E20(ctx, base);
	// 82689C0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689C20 size=112
    let mut pc: u32 = 0x82689C20;
    'dispatch: loop {
        match pc {
            0x82689C20 => {
    //   block [0x82689C20..0x82689C90)
	// 82689C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689C28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689C2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689C30: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689C34: 38AAC660  addi r5, r10, -0x39a0
	ctx.r[5].s64 = ctx.r[10].s64 + -14752;
	// 82689C38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689C3C: 390B2EB0  addi r8, r11, 0x2eb0
	ctx.r[8].s64 = ctx.r[11].s64 + 11952;
	// 82689C40: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82689C44: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82689C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689C4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689C50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82689C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689C58: 386AC6C0  addi r3, r10, -0x3940
	ctx.r[3].s64 = ctx.r[10].s64 + -14656;
	// 82689C5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689C64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689C6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689C7C: 4BDDD1A5  bl 0x82466e20
	ctx.lr = 0x82689C80;
	sub_82466E20(ctx, base);
	// 82689C80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689C8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689C90 size=116
    let mut pc: u32 = 0x82689C90;
    'dispatch: loop {
        match pc {
            0x82689C90 => {
    //   block [0x82689C90..0x82689D04)
	// 82689C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689C9C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689CA0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82689CA4: 390B2F58  addi r8, r11, 0x2f58
	ctx.r[8].s64 = ctx.r[11].s64 + 12120;
	// 82689CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689CAC: 392A61B8  addi r9, r10, 0x61b8
	ctx.r[9].s64 = ctx.r[10].s64 + 25016;
	// 82689CB0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689CB4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82689CB8: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 82689CBC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82689CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689CC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689CD4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82689CD8: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82689CDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82689CE0: 386BC6F0  addi r3, r11, -0x3910
	ctx.r[3].s64 = ctx.r[11].s64 + -14608;
	// 82689CE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82689CE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689CF0: 4BDDD131  bl 0x82466e20
	ctx.lr = 0x82689CF4;
	sub_82466E20(ctx, base);
	// 82689CF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689CF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689CFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689D00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689D08 size=112
    let mut pc: u32 = 0x82689D08;
    'dispatch: loop {
        match pc {
            0x82689D08 => {
    //   block [0x82689D08..0x82689D78)
	// 82689D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689D14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689D18: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689D1C: 38AAC7B0  addi r5, r10, -0x3850
	ctx.r[5].s64 = ctx.r[10].s64 + -14416;
	// 82689D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689D24: 390B2F70  addi r8, r11, 0x2f70
	ctx.r[8].s64 = ctx.r[11].s64 + 12144;
	// 82689D28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82689D2C: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82689D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689D34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689D38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82689D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689D40: 386AC720  addi r3, r10, -0x38e0
	ctx.r[3].s64 = ctx.r[10].s64 + -14560;
	// 82689D44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689D64: 4BDDD0BD  bl 0x82466e20
	ctx.lr = 0x82689D68;
	sub_82466E20(ctx, base);
	// 82689D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689D74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689D78 size=100
    let mut pc: u32 = 0x82689D78;
    'dispatch: loop {
        match pc {
            0x82689D78 => {
    //   block [0x82689D78..0x82689DDC)
	// 82689D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689D84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689D8C: 38AAC780  addi r5, r10, -0x3880
	ctx.r[5].s64 = ctx.r[10].s64 + -14464;
	// 82689D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689D98: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82689D9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689DAC: 386AC750  addi r3, r10, -0x38b0
	ctx.r[3].s64 = ctx.r[10].s64 + -14512;
	// 82689DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689DB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689DB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82689DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689DC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82689DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689DC8: 4BDDD059  bl 0x82466e20
	ctx.lr = 0x82689DCC;
	sub_82466E20(ctx, base);
	// 82689DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689DE0 size=112
    let mut pc: u32 = 0x82689DE0;
    'dispatch: loop {
        match pc {
            0x82689DE0 => {
    //   block [0x82689DE0..0x82689E50)
	// 82689DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689DEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689DF0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689DF4: 38AAC7B0  addi r5, r10, -0x3850
	ctx.r[5].s64 = ctx.r[10].s64 + -14416;
	// 82689DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689DFC: 390B2F88  addi r8, r11, 0x2f88
	ctx.r[8].s64 = ctx.r[11].s64 + 12168;
	// 82689E00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82689E04: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82689E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689E0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82689E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689E18: 386AC780  addi r3, r10, -0x3880
	ctx.r[3].s64 = ctx.r[10].s64 + -14464;
	// 82689E1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689E34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689E3C: 4BDDCFE5  bl 0x82466e20
	ctx.lr = 0x82689E40;
	sub_82466E20(ctx, base);
	// 82689E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689E50 size=112
    let mut pc: u32 = 0x82689E50;
    'dispatch: loop {
        match pc {
            0x82689E50 => {
    //   block [0x82689E50..0x82689EC0)
	// 82689E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689E5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689E60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82689E64: 38AAC6F0  addi r5, r10, -0x3910
	ctx.r[5].s64 = ctx.r[10].s64 + -14608;
	// 82689E68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82689E6C: 390B2FB8  addi r8, r11, 0x2fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 12216;
	// 82689E70: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82689E74: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 82689E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689E7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689E80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82689E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689E88: 386AC7B0  addi r3, r10, -0x3850
	ctx.r[3].s64 = ctx.r[10].s64 + -14416;
	// 82689E8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82689E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689EAC: 4BDDCF75  bl 0x82466e20
	ctx.lr = 0x82689EB0;
	sub_82466E20(ctx, base);
	// 82689EB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689EC0 size=100
    let mut pc: u32 = 0x82689EC0;
    'dispatch: loop {
        match pc {
            0x82689EC0 => {
    //   block [0x82689EC0..0x82689F24)
	// 82689EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689ECC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689ED4: 38AAC7B0  addi r5, r10, -0x3850
	ctx.r[5].s64 = ctx.r[10].s64 + -14416;
	// 82689ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689EE0: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82689EE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689EF4: 386AC7E0  addi r3, r10, -0x3820
	ctx.r[3].s64 = ctx.r[10].s64 + -14368;
	// 82689EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689EFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689F00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82689F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689F08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82689F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689F10: 4BDDCF11  bl 0x82466e20
	ctx.lr = 0x82689F14;
	sub_82466E20(ctx, base);
	// 82689F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689F28 size=100
    let mut pc: u32 = 0x82689F28;
    'dispatch: loop {
        match pc {
            0x82689F28 => {
    //   block [0x82689F28..0x82689F8C)
	// 82689F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689F34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689F3C: 38AAC720  addi r5, r10, -0x38e0
	ctx.r[5].s64 = ctx.r[10].s64 + -14560;
	// 82689F40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689F48: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82689F4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689F5C: 386AC810  addi r3, r10, -0x37f0
	ctx.r[3].s64 = ctx.r[10].s64 + -14320;
	// 82689F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689F64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689F68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82689F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689F70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82689F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689F78: 4BDDCEA9  bl 0x82466e20
	ctx.lr = 0x82689F7C;
	sub_82466E20(ctx, base);
	// 82689F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689F90 size=100
    let mut pc: u32 = 0x82689F90;
    'dispatch: loop {
        match pc {
            0x82689F90 => {
    //   block [0x82689F90..0x82689FF4)
	// 82689F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82689F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82689F9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82689FA4: 38AAC7E0  addi r5, r10, -0x3820
	ctx.r[5].s64 = ctx.r[10].s64 + -14368;
	// 82689FA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82689FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82689FB0: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82689FB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82689FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82689FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82689FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82689FC4: 386AC840  addi r3, r10, -0x37c0
	ctx.r[3].s64 = ctx.r[10].s64 + -14272;
	// 82689FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82689FCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82689FD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82689FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82689FD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82689FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82689FE0: 4BDDCE41  bl 0x82466e20
	ctx.lr = 0x82689FE4;
	sub_82466E20(ctx, base);
	// 82689FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82689FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82689FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82689FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82689FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82689FF8 size=100
    let mut pc: u32 = 0x82689FF8;
    'dispatch: loop {
        match pc {
            0x82689FF8 => {
    //   block [0x82689FF8..0x8268A05C)
	// 82689FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82689FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A004: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A00C: 38AAC720  addi r5, r10, -0x38e0
	ctx.r[5].s64 = ctx.r[10].s64 + -14560;
	// 8268A010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A018: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8268A01C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A02C: 386AC870  addi r3, r10, -0x3790
	ctx.r[3].s64 = ctx.r[10].s64 + -14224;
	// 8268A030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A034: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A038: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268A03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A040: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268A044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A048: 4BDDCDD9  bl 0x82466e20
	ctx.lr = 0x8268A04C;
	sub_82466E20(ctx, base);
	// 8268A04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A060 size=112
    let mut pc: u32 = 0x8268A060;
    'dispatch: loop {
        match pc {
            0x8268A060 => {
    //   block [0x8268A060..0x8268A0D0)
	// 8268A060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A06C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A070: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A074: 38AAC900  addi r5, r10, -0x3700
	ctx.r[5].s64 = ctx.r[10].s64 + -14080;
	// 8268A078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A07C: 390B3060  addi r8, r11, 0x3060
	ctx.r[8].s64 = ctx.r[11].s64 + 12384;
	// 8268A080: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268A084: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 8268A088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A08C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A098: 386AC8A0  addi r3, r10, -0x3760
	ctx.r[3].s64 = ctx.r[10].s64 + -14176;
	// 8268A09C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A0BC: 4BDDCD65  bl 0x82466e20
	ctx.lr = 0x8268A0C0;
	sub_82466E20(ctx, base);
	// 8268A0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A0D0 size=112
    let mut pc: u32 = 0x8268A0D0;
    'dispatch: loop {
        match pc {
            0x8268A0D0 => {
    //   block [0x8268A0D0..0x8268A140)
	// 8268A0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A0DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A0E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A0E4: 38AAC930  addi r5, r10, -0x36d0
	ctx.r[5].s64 = ctx.r[10].s64 + -14032;
	// 8268A0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A0EC: 390B3090  addi r8, r11, 0x3090
	ctx.r[8].s64 = ctx.r[11].s64 + 12432;
	// 8268A0F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268A0F4: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 8268A0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A0FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A108: 386AC8D0  addi r3, r10, -0x3730
	ctx.r[3].s64 = ctx.r[10].s64 + -14128;
	// 8268A10C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A12C: 4BDDCCF5  bl 0x82466e20
	ctx.lr = 0x8268A130;
	sub_82466E20(ctx, base);
	// 8268A130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A140 size=112
    let mut pc: u32 = 0x8268A140;
    'dispatch: loop {
        match pc {
            0x8268A140 => {
    //   block [0x8268A140..0x8268A1B0)
	// 8268A140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A14C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A150: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A154: 38AACA20  addi r5, r10, -0x35e0
	ctx.r[5].s64 = ctx.r[10].s64 + -13792;
	// 8268A158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A15C: 390B30A8  addi r8, r11, 0x30a8
	ctx.r[8].s64 = ctx.r[11].s64 + 12456;
	// 8268A160: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268A164: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 8268A168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A16C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A178: 386AC900  addi r3, r10, -0x3700
	ctx.r[3].s64 = ctx.r[10].s64 + -14080;
	// 8268A17C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A19C: 4BDDCC85  bl 0x82466e20
	ctx.lr = 0x8268A1A0;
	sub_82466E20(ctx, base);
	// 8268A1A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A1AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A1B0 size=112
    let mut pc: u32 = 0x8268A1B0;
    'dispatch: loop {
        match pc {
            0x8268A1B0 => {
    //   block [0x8268A1B0..0x8268A220)
	// 8268A1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A1BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A1C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A1C4: 38AAC900  addi r5, r10, -0x3700
	ctx.r[5].s64 = ctx.r[10].s64 + -14080;
	// 8268A1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A1CC: 390B30D8  addi r8, r11, 0x30d8
	ctx.r[8].s64 = ctx.r[11].s64 + 12504;
	// 8268A1D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268A1D4: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 8268A1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A1DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A1E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A1E8: 386AC930  addi r3, r10, -0x36d0
	ctx.r[3].s64 = ctx.r[10].s64 + -14032;
	// 8268A1EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A20C: 4BDDCC15  bl 0x82466e20
	ctx.lr = 0x8268A210;
	sub_82466E20(ctx, base);
	// 8268A210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A220 size=112
    let mut pc: u32 = 0x8268A220;
    'dispatch: loop {
        match pc {
            0x8268A220 => {
    //   block [0x8268A220..0x8268A290)
	// 8268A220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A22C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A230: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A234: 38AAC930  addi r5, r10, -0x36d0
	ctx.r[5].s64 = ctx.r[10].s64 + -14032;
	// 8268A238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A23C: 390B30F0  addi r8, r11, 0x30f0
	ctx.r[8].s64 = ctx.r[11].s64 + 12528;
	// 8268A240: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268A244: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 8268A248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A24C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A258: 386AC960  addi r3, r10, -0x36a0
	ctx.r[3].s64 = ctx.r[10].s64 + -13984;
	// 8268A25C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A27C: 4BDDCBA5  bl 0x82466e20
	ctx.lr = 0x8268A280;
	sub_82466E20(ctx, base);
	// 8268A280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A290 size=112
    let mut pc: u32 = 0x8268A290;
    'dispatch: loop {
        match pc {
            0x8268A290 => {
    //   block [0x8268A290..0x8268A300)
	// 8268A290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A29C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268A2A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A2A4: 392A61E4  addi r9, r10, 0x61e4
	ctx.r[9].s64 = ctx.r[10].s64 + 25060;
	// 8268A2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A2AC: 390B310C  addi r8, r11, 0x310c
	ctx.r[8].s64 = ctx.r[11].s64 + 12556;
	// 8268A2B0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8268A2B4: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 8268A2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A2BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A2C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A2C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A2C8: 386AC990  addi r3, r10, -0x3670
	ctx.r[3].s64 = ctx.r[10].s64 + -13936;
	// 8268A2CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268A2D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268A2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A2E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268A2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A2EC: 4BDDCB35  bl 0x82466e20
	ctx.lr = 0x8268A2F0;
	sub_82466E20(ctx, base);
	// 8268A2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A300 size=112
    let mut pc: u32 = 0x8268A300;
    'dispatch: loop {
        match pc {
            0x8268A300 => {
    //   block [0x8268A300..0x8268A370)
	// 8268A300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A30C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A310: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A314: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268A318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A31C: 390B3140  addi r8, r11, 0x3140
	ctx.r[8].s64 = ctx.r[11].s64 + 12608;
	// 8268A320: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8268A324: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 8268A328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A32C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A338: 386AC9C0  addi r3, r10, -0x3640
	ctx.r[3].s64 = ctx.r[10].s64 + -13888;
	// 8268A33C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8268A340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268A344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8268A348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A34C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A35C: 4BDDCAC5  bl 0x82466e20
	ctx.lr = 0x8268A360;
	sub_82466E20(ctx, base);
	// 8268A360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8268A370 size=48
    let mut pc: u32 = 0x8268A370;
    'dispatch: loop {
        match pc {
            0x8268A370 => {
    //   block [0x8268A370..0x8268A3A0)
	// 8268A370: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A374: 814B31D8  lwz r10, 0x31d8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12760 as u32) ) } as u64;
	// 8268A378: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A37C: 396B51F0  addi r11, r11, 0x51f0
	ctx.r[11].s64 = ctx.r[11].s64 + 20976;
	// 8268A380: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8268A384: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8268A388: 814A31D4  lwz r10, 0x31d4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12756 as u32) ) } as u64;
	// 8268A38C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 8268A390: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8268A394: 814A31D0  lwz r10, 0x31d0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12752 as u32) ) } as u64;
	// 8268A398: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 8268A39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A3A0 size=116
    let mut pc: u32 = 0x8268A3A0;
    'dispatch: loop {
        match pc {
            0x8268A3A0 => {
    //   block [0x8268A3A0..0x8268A414)
	// 8268A3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A3AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8268A3B0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A3B4: 392B62D8  addi r9, r11, 0x62d8
	ctx.r[9].s64 = ctx.r[11].s64 + 25304;
	// 8268A3B8: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268A3BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A3C0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8268A3C4: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 8268A3C8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A3CC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 8268A3D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A3D4: 396B51F0  addi r11, r11, 0x51f0
	ctx.r[11].s64 = ctx.r[11].s64 + 20976;
	// 8268A3D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8268A3DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A3E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8268A3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A3E8: 386AC9F0  addi r3, r10, -0x3610
	ctx.r[3].s64 = ctx.r[10].s64 + -13840;
	// 8268A3EC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8268A3F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8268A3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A3F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8268A3FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268A400: 4BDDCA21  bl 0x82466e20
	ctx.lr = 0x8268A404;
	sub_82466E20(ctx, base);
	// 8268A404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8268A418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8268A418 size=116
    let mut pc: u32 = 0x8268A418;
    'dispatch: loop {
        match pc {
            0x8268A418 => {
    //   block [0x8268A418..0x8268A48C)
	// 8268A418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268A41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8268A420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268A424: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268A428: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8268A42C: 390B31E0  addi r8, r11, 0x31e0
	ctx.r[8].s64 = ctx.r[11].s64 + 12768;
	// 8268A430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268A434: 392A63C0  addi r9, r10, 0x63c0
	ctx.r[9].s64 = ctx.r[10].s64 + 25536;
	// 8268A438: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268A43C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8268A440: 38AAA830  addi r5, r10, -0x57d0
	ctx.r[5].s64 = ctx.r[10].s64 + -22480;
	// 8268A444: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268A448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268A44C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268A450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268A454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268A458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268A45C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8268A460: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8268A464: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268A468: 386BCA20  addi r3, r11, -0x35e0
	ctx.r[3].s64 = ctx.r[11].s64 + -13792;
	// 8268A46C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268A470: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268A474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8268A478: 4BDDC9A9  bl 0x82466e20
	ctx.lr = 0x8268A47C;
	sub_82466E20(ctx, base);
	// 8268A47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268A480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268A484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268A488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


