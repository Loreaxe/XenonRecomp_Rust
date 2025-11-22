pub fn sub_82669E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669E08 size=112
    let mut pc: u32 = 0x82669E08;
    'dispatch: loop {
        match pc {
            0x82669E08 => {
    //   block [0x82669E08..0x82669E78)
	// 82669E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669E14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669E18: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669E1C: 38AADDE0  addi r5, r10, -0x2220
	ctx.r[5].s64 = ctx.r[10].s64 + -8736;
	// 82669E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669E24: 390B9BB8  addi r8, r11, -0x6448
	ctx.r[8].s64 = ctx.r[11].s64 + -25672;
	// 82669E28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82669E2C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82669E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669E34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669E40: 386AEE30  addi r3, r10, -0x11d0
	ctx.r[3].s64 = ctx.r[10].s64 + -4560;
	// 82669E44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669E64: 4BDFCFBD  bl 0x82466e20
	ctx.lr = 0x82669E68;
	sub_82466E20(ctx, base);
	// 82669E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669E78 size=112
    let mut pc: u32 = 0x82669E78;
    'dispatch: loop {
        match pc {
            0x82669E78 => {
    //   block [0x82669E78..0x82669EE8)
	// 82669E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669E88: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669E8C: 38AADDE0  addi r5, r10, -0x2220
	ctx.r[5].s64 = ctx.r[10].s64 + -8736;
	// 82669E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669E94: 390B9C18  addi r8, r11, -0x63e8
	ctx.r[8].s64 = ctx.r[11].s64 + -25576;
	// 82669E98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82669E9C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82669EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669EA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669EB0: 386AEE60  addi r3, r10, -0x11a0
	ctx.r[3].s64 = ctx.r[10].s64 + -4512;
	// 82669EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669ED4: 4BDFCF4D  bl 0x82466e20
	ctx.lr = 0x82669ED8;
	sub_82466E20(ctx, base);
	// 82669ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669EE8 size=112
    let mut pc: u32 = 0x82669EE8;
    'dispatch: loop {
        match pc {
            0x82669EE8 => {
    //   block [0x82669EE8..0x82669F58)
	// 82669EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669EF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669EF8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669EFC: 38AADDB0  addi r5, r10, -0x2250
	ctx.r[5].s64 = ctx.r[10].s64 + -8784;
	// 82669F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669F04: 390B9C78  addi r8, r11, -0x6388
	ctx.r[8].s64 = ctx.r[11].s64 + -25480;
	// 82669F08: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82669F0C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82669F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669F14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669F20: 386AEE90  addi r3, r10, -0x1170
	ctx.r[3].s64 = ctx.r[10].s64 + -4464;
	// 82669F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82669F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82669F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82669F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669F44: 4BDFCEDD  bl 0x82466e20
	ctx.lr = 0x82669F48;
	sub_82466E20(ctx, base);
	// 82669F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669F58 size=112
    let mut pc: u32 = 0x82669F58;
    'dispatch: loop {
        match pc {
            0x82669F58 => {
    //   block [0x82669F58..0x82669FC8)
	// 82669F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669F64: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 82669F68: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 82669F6C: 38EA9D38  addi r7, r10, -0x62c8
	ctx.r[7].s64 = ctx.r[10].s64 + -25288;
	// 82669F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669F74: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82669F78: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82669F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82669F80: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82669F84: 396BFF18  addi r11, r11, -0xe8
	ctx.r[11].s64 = ctx.r[11].s64 + -232;
	// 82669F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82669F8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669F90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82669F94: 386AEEC0  addi r3, r10, -0x1140
	ctx.r[3].s64 = ctx.r[10].s64 + -4416;
	// 82669F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82669F9C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82669FA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82669FA4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82669FA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82669FAC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82669FB0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82669FB4: 4BDFCE6D  bl 0x82466e20
	ctx.lr = 0x82669FB8;
	sub_82466E20(ctx, base);
	// 82669FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82669FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82669FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82669FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82669FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82669FC8 size=112
    let mut pc: u32 = 0x82669FC8;
    'dispatch: loop {
        match pc {
            0x82669FC8 => {
    //   block [0x82669FC8..0x8266A038)
	// 82669FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82669FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82669FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82669FD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669FD8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 82669FDC: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 82669FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82669FE4: 390B9F00  addi r8, r11, -0x6100
	ctx.r[8].s64 = ctx.r[11].s64 + -24832;
	// 82669FE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82669FEC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82669FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82669FF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82669FF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82669FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A000: 386AEEF0  addi r3, r10, -0x1110
	ctx.r[3].s64 = ctx.r[10].s64 + -4368;
	// 8266A004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A014: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266A018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A024: 4BDFCDFD  bl 0x82466e20
	ctx.lr = 0x8266A028;
	sub_82466E20(ctx, base);
	// 8266A028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A038 size=112
    let mut pc: u32 = 0x8266A038;
    'dispatch: loop {
        match pc {
            0x8266A038 => {
    //   block [0x8266A038..0x8266A0A8)
	// 8266A038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A044: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A048: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A04C: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 8266A050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A054: 390B9F18  addi r8, r11, -0x60e8
	ctx.r[8].s64 = ctx.r[11].s64 + -24808;
	// 8266A058: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A05C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8266A060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A064: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A070: 386AEF20  addi r3, r10, -0x10e0
	ctx.r[3].s64 = ctx.r[10].s64 + -4320;
	// 8266A074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A084: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266A088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A094: 4BDFCD8D  bl 0x82466e20
	ctx.lr = 0x8266A098;
	sub_82466E20(ctx, base);
	// 8266A098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A0A8 size=112
    let mut pc: u32 = 0x8266A0A8;
    'dispatch: loop {
        match pc {
            0x8266A0A8 => {
    //   block [0x8266A0A8..0x8266A118)
	// 8266A0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A0B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A0B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A0BC: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 8266A0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A0C4: 390B9F30  addi r8, r11, -0x60d0
	ctx.r[8].s64 = ctx.r[11].s64 + -24784;
	// 8266A0C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266A0CC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8266A0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A0D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A0D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A0E0: 386AEF50  addi r3, r10, -0x10b0
	ctx.r[3].s64 = ctx.r[10].s64 + -4272;
	// 8266A0E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A0EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A104: 4BDFCD1D  bl 0x82466e20
	ctx.lr = 0x8266A108;
	sub_82466E20(ctx, base);
	// 8266A108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A118 size=108
    let mut pc: u32 = 0x8266A118;
    'dispatch: loop {
        match pc {
            0x8266A118 => {
    //   block [0x8266A118..0x8266A184)
	// 8266A118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A124: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A12C: 38EB9F60  addi r7, r11, -0x60a0
	ctx.r[7].s64 = ctx.r[11].s64 + -24736;
	// 8266A130: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A134: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8266A138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A13C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A148: 386AEF80  addi r3, r10, -0x1080
	ctx.r[3].s64 = ctx.r[10].s64 + -4224;
	// 8266A14C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A16C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A170: 4BDFCCB1  bl 0x82466e20
	ctx.lr = 0x8266A174;
	sub_82466E20(ctx, base);
	// 8266A174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A188 size=112
    let mut pc: u32 = 0x8266A188;
    'dispatch: loop {
        match pc {
            0x8266A188 => {
    //   block [0x8266A188..0x8266A1F8)
	// 8266A188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A198: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A19C: 38AACBE0  addi r5, r10, -0x3420
	ctx.r[5].s64 = ctx.r[10].s64 + -13344;
	// 8266A1A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A1A4: 390B9F90  addi r8, r11, -0x6070
	ctx.r[8].s64 = ctx.r[11].s64 + -24688;
	// 8266A1A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A1AC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8266A1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A1B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A1B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A1C0: 386AEFB0  addi r3, r10, -0x1050
	ctx.r[3].s64 = ctx.r[10].s64 + -4176;
	// 8266A1C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A1CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A1D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266A1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A1E4: 4BDFCC3D  bl 0x82466e20
	ctx.lr = 0x8266A1E8;
	sub_82466E20(ctx, base);
	// 8266A1E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A1EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A1F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A1F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A1F8 size=108
    let mut pc: u32 = 0x8266A1F8;
    'dispatch: loop {
        match pc {
            0x8266A1F8 => {
    //   block [0x8266A1F8..0x8266A264)
	// 8266A1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A204: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A20C: 38EB9FA8  addi r7, r11, -0x6058
	ctx.r[7].s64 = ctx.r[11].s64 + -24664;
	// 8266A210: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A214: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8266A218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A21C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A220: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A228: 386AEFE0  addi r3, r10, -0x1020
	ctx.r[3].s64 = ctx.r[10].s64 + -4128;
	// 8266A22C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A23C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A24C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A250: 4BDFCBD1  bl 0x82466e20
	ctx.lr = 0x8266A254;
	sub_82466E20(ctx, base);
	// 8266A254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A268 size=108
    let mut pc: u32 = 0x8266A268;
    'dispatch: loop {
        match pc {
            0x8266A268 => {
    //   block [0x8266A268..0x8266A2D4)
	// 8266A268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A274: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A27C: 38EB9FD8  addi r7, r11, -0x6028
	ctx.r[7].s64 = ctx.r[11].s64 + -24616;
	// 8266A280: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266A284: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8266A288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A28C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A298: 386AF010  addi r3, r10, -0xff0
	ctx.r[3].s64 = ctx.r[10].s64 + -4080;
	// 8266A29C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A2A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A2BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A2C0: 4BDFCB61  bl 0x82466e20
	ctx.lr = 0x8266A2C4;
	sub_82466E20(ctx, base);
	// 8266A2C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A2D8 size=112
    let mut pc: u32 = 0x8266A2D8;
    'dispatch: loop {
        match pc {
            0x8266A2D8 => {
    //   block [0x8266A2D8..0x8266A348)
	// 8266A2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A2E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A2E8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A2EC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A2F4: 390BA020  addi r8, r11, -0x5fe0
	ctx.r[8].s64 = ctx.r[11].s64 + -24544;
	// 8266A2F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266A2FC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8266A300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A304: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A310: 386AF040  addi r3, r10, -0xfc0
	ctx.r[3].s64 = ctx.r[10].s64 + -4032;
	// 8266A314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A31C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A334: 4BDFCAED  bl 0x82466e20
	ctx.lr = 0x8266A338;
	sub_82466E20(ctx, base);
	// 8266A338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A348 size=112
    let mut pc: u32 = 0x8266A348;
    'dispatch: loop {
        match pc {
            0x8266A348 => {
    //   block [0x8266A348..0x8266A3B8)
	// 8266A348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A354: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A358: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A35C: 38AAEBC0  addi r5, r10, -0x1440
	ctx.r[5].s64 = ctx.r[10].s64 + -5184;
	// 8266A360: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266A364: 390BA068  addi r8, r11, -0x5f98
	ctx.r[8].s64 = ctx.r[11].s64 + -24472;
	// 8266A368: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A36C: 388A0E58  addi r4, r10, 0xe58
	ctx.r[4].s64 = ctx.r[10].s64 + 3672;
	// 8266A370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A374: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A380: 386AF070  addi r3, r10, -0xf90
	ctx.r[3].s64 = ctx.r[10].s64 + -3984;
	// 8266A384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A38C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A39C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A3A4: 4BDFCA7D  bl 0x82466e20
	ctx.lr = 0x8266A3A8;
	sub_82466E20(ctx, base);
	// 8266A3A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A3B8 size=108
    let mut pc: u32 = 0x8266A3B8;
    'dispatch: loop {
        match pc {
            0x8266A3B8 => {
    //   block [0x8266A3B8..0x8266A424)
	// 8266A3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A3C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A3C4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A3C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266A3CC: 38EBA080  addi r7, r11, -0x5f80
	ctx.r[7].s64 = ctx.r[11].s64 + -24448;
	// 8266A3D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266A3D4: 388A0E74  addi r4, r10, 0xe74
	ctx.r[4].s64 = ctx.r[10].s64 + 3700;
	// 8266A3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A3DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A3E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A3E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A3E8: 386AF0A0  addi r3, r10, -0xf60
	ctx.r[3].s64 = ctx.r[10].s64 + -3936;
	// 8266A3EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A3F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A3FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A40C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A410: 4BDFCA11  bl 0x82466e20
	ctx.lr = 0x8266A414;
	sub_82466E20(ctx, base);
	// 8266A414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A41C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A428 size=108
    let mut pc: u32 = 0x8266A428;
    'dispatch: loop {
        match pc {
            0x8266A428 => {
    //   block [0x8266A428..0x8266A494)
	// 8266A428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A434: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A438: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266A43C: 38EBA0C8  addi r7, r11, -0x5f38
	ctx.r[7].s64 = ctx.r[11].s64 + -24376;
	// 8266A440: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A444: 388A0E9C  addi r4, r10, 0xe9c
	ctx.r[4].s64 = ctx.r[10].s64 + 3740;
	// 8266A448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A44C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A450: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A458: 386AF0D0  addi r3, r10, -0xf30
	ctx.r[3].s64 = ctx.r[10].s64 + -3888;
	// 8266A45C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A46C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A47C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A480: 4BDFC9A1  bl 0x82466e20
	ctx.lr = 0x8266A484;
	sub_82466E20(ctx, base);
	// 8266A484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A498 size=112
    let mut pc: u32 = 0x8266A498;
    'dispatch: loop {
        match pc {
            0x8266A498 => {
    //   block [0x8266A498..0x8266A508)
	// 8266A498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A4A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A4A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A4AC: 38AAF0D0  addi r5, r10, -0xf30
	ctx.r[5].s64 = ctx.r[10].s64 + -3888;
	// 8266A4B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266A4B4: 390BA0F8  addi r8, r11, -0x5f08
	ctx.r[8].s64 = ctx.r[11].s64 + -24328;
	// 8266A4B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266A4BC: 388A0EB4  addi r4, r10, 0xeb4
	ctx.r[4].s64 = ctx.r[10].s64 + 3764;
	// 8266A4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A4C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A4C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A4CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A4D0: 386AF100  addi r3, r10, -0xf00
	ctx.r[3].s64 = ctx.r[10].s64 + -3840;
	// 8266A4D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A4DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A4F4: 4BDFC92D  bl 0x82466e20
	ctx.lr = 0x8266A4F8;
	sub_82466E20(ctx, base);
	// 8266A4F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266A508 size=24
    let mut pc: u32 = 0x8266A508;
    'dispatch: loop {
        match pc {
            0x8266A508 => {
    //   block [0x8266A508..0x8266A520)
	// 8266A508: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A50C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266A510: 394ACEA8  addi r10, r10, -0x3158
	ctx.r[10].s64 = ctx.r[10].s64 + -12632;
	// 8266A514: 816BA128  lwz r11, -0x5ed8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24280 as u32) ) } as u64;
	// 8266A518: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8266A51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A520 size=116
    let mut pc: u32 = 0x8266A520;
    'dispatch: loop {
        match pc {
            0x8266A520 => {
    //   block [0x8266A520..0x8266A594)
	// 8266A520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A52C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A530: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266A534: 390BCEA8  addi r8, r11, -0x3158
	ctx.r[8].s64 = ctx.r[11].s64 + -12632;
	// 8266A538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A53C: 392AFFB0  addi r9, r10, -0x50
	ctx.r[9].s64 = ctx.r[10].s64 + -80;
	// 8266A540: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A544: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8266A548: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A54C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A554: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8266A558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A564: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8266A568: 388A0EDC  addi r4, r10, 0xedc
	ctx.r[4].s64 = ctx.r[10].s64 + 3804;
	// 8266A56C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266A570: 386BF130  addi r3, r11, -0xed0
	ctx.r[3].s64 = ctx.r[11].s64 + -3792;
	// 8266A574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266A578: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A580: 4BDFC8A1  bl 0x82466e20
	ctx.lr = 0x8266A584;
	sub_82466E20(ctx, base);
	// 8266A584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A598 size=112
    let mut pc: u32 = 0x8266A598;
    'dispatch: loop {
        match pc {
            0x8266A598 => {
    //   block [0x8266A598..0x8266A608)
	// 8266A598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A5A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A5A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A5AC: 38AADDE0  addi r5, r10, -0x2220
	ctx.r[5].s64 = ctx.r[10].s64 + -8736;
	// 8266A5B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A5B4: 390BA130  addi r8, r11, -0x5ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -24272;
	// 8266A5B8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8266A5BC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8266A5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A5C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A5D0: 386AF160  addi r3, r10, -0xea0
	ctx.r[3].s64 = ctx.r[10].s64 + -3744;
	// 8266A5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A5F4: 4BDFC82D  bl 0x82466e20
	ctx.lr = 0x8266A5F8;
	sub_82466E20(ctx, base);
	// 8266A5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A608 size=108
    let mut pc: u32 = 0x8266A608;
    'dispatch: loop {
        match pc {
            0x8266A608 => {
    //   block [0x8266A608..0x8266A674)
	// 8266A608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A614: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A61C: 38EBA1C0  addi r7, r11, -0x5e40
	ctx.r[7].s64 = ctx.r[11].s64 + -24128;
	// 8266A620: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266A624: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8266A628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A62C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A638: 386AF190  addi r3, r10, -0xe70
	ctx.r[3].s64 = ctx.r[10].s64 + -3696;
	// 8266A63C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A65C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A660: 4BDFC7C1  bl 0x82466e20
	ctx.lr = 0x8266A664;
	sub_82466E20(ctx, base);
	// 8266A664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A678 size=108
    let mut pc: u32 = 0x8266A678;
    'dispatch: loop {
        match pc {
            0x8266A678 => {
    //   block [0x8266A678..0x8266A6E4)
	// 8266A678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A684: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A68C: 38EBA208  addi r7, r11, -0x5df8
	ctx.r[7].s64 = ctx.r[11].s64 + -24056;
	// 8266A690: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A694: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8266A698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A69C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A6A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A6A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A6A8: 386AF1C0  addi r3, r10, -0xe40
	ctx.r[3].s64 = ctx.r[10].s64 + -3648;
	// 8266A6AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A6B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A6B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A6B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A6C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A6C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A6CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A6D0: 4BDFC751  bl 0x82466e20
	ctx.lr = 0x8266A6D4;
	sub_82466E20(ctx, base);
	// 8266A6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A6E8 size=108
    let mut pc: u32 = 0x8266A6E8;
    'dispatch: loop {
        match pc {
            0x8266A6E8 => {
    //   block [0x8266A6E8..0x8266A754)
	// 8266A6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A6F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A6F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A6FC: 38EBA238  addi r7, r11, -0x5dc8
	ctx.r[7].s64 = ctx.r[11].s64 + -24008;
	// 8266A700: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A704: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8266A708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A70C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A718: 386AF1F0  addi r3, r10, -0xe10
	ctx.r[3].s64 = ctx.r[10].s64 + -3600;
	// 8266A71C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A73C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A740: 4BDFC6E1  bl 0x82466e20
	ctx.lr = 0x8266A744;
	sub_82466E20(ctx, base);
	// 8266A744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A758 size=112
    let mut pc: u32 = 0x8266A758;
    'dispatch: loop {
        match pc {
            0x8266A758 => {
    //   block [0x8266A758..0x8266A7C8)
	// 8266A758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A768: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A76C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A774: 390BA268  addi r8, r11, -0x5d98
	ctx.r[8].s64 = ctx.r[11].s64 + -23960;
	// 8266A778: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266A77C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8266A780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A790: 386AF220  addi r3, r10, -0xde0
	ctx.r[3].s64 = ctx.r[10].s64 + -3552;
	// 8266A794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A7B4: 4BDFC66D  bl 0x82466e20
	ctx.lr = 0x8266A7B8;
	sub_82466E20(ctx, base);
	// 8266A7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A7C8 size=112
    let mut pc: u32 = 0x8266A7C8;
    'dispatch: loop {
        match pc {
            0x8266A7C8 => {
    //   block [0x8266A7C8..0x8266A838)
	// 8266A7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A7D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A7D8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A7DC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A7E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A7E4: 390BA298  addi r8, r11, -0x5d68
	ctx.r[8].s64 = ctx.r[11].s64 + -23912;
	// 8266A7E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A7EC: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8266A7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A7F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A7F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A800: 386AF250  addi r3, r10, -0xdb0
	ctx.r[3].s64 = ctx.r[10].s64 + -3504;
	// 8266A804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A824: 4BDFC5FD  bl 0x82466e20
	ctx.lr = 0x8266A828;
	sub_82466E20(ctx, base);
	// 8266A828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A838 size=112
    let mut pc: u32 = 0x8266A838;
    'dispatch: loop {
        match pc {
            0x8266A838 => {
    //   block [0x8266A838..0x8266A8A8)
	// 8266A838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A844: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A848: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A84C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A854: 390BA2B0  addi r8, r11, -0x5d50
	ctx.r[8].s64 = ctx.r[11].s64 + -23888;
	// 8266A858: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A85C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8266A860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A864: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A870: 386AF280  addi r3, r10, -0xd80
	ctx.r[3].s64 = ctx.r[10].s64 + -3456;
	// 8266A874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A894: 4BDFC58D  bl 0x82466e20
	ctx.lr = 0x8266A898;
	sub_82466E20(ctx, base);
	// 8266A898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A8A8 size=108
    let mut pc: u32 = 0x8266A8A8;
    'dispatch: loop {
        match pc {
            0x8266A8A8 => {
    //   block [0x8266A8A8..0x8266A914)
	// 8266A8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A8B4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A8BC: 38EBA2C8  addi r7, r11, -0x5d38
	ctx.r[7].s64 = ctx.r[11].s64 + -23864;
	// 8266A8C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266A8C4: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8266A8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A8CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A8D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A8D8: 386AF2B0  addi r3, r10, -0xd50
	ctx.r[3].s64 = ctx.r[10].s64 + -3408;
	// 8266A8DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A8EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A8FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A900: 4BDFC521  bl 0x82466e20
	ctx.lr = 0x8266A904;
	sub_82466E20(ctx, base);
	// 8266A904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A918 size=112
    let mut pc: u32 = 0x8266A918;
    'dispatch: loop {
        match pc {
            0x8266A918 => {
    //   block [0x8266A918..0x8266A988)
	// 8266A918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A924: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A928: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A92C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266A930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A934: 390BA2F8  addi r8, r11, -0x5d08
	ctx.r[8].s64 = ctx.r[11].s64 + -23816;
	// 8266A938: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266A93C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8266A940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A944: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266A94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A950: 386AF2E0  addi r3, r10, -0xd20
	ctx.r[3].s64 = ctx.r[10].s64 + -3360;
	// 8266A954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266A958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A96C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A974: 4BDFC4AD  bl 0x82466e20
	ctx.lr = 0x8266A978;
	sub_82466E20(ctx, base);
	// 8266A978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A988 size=108
    let mut pc: u32 = 0x8266A988;
    'dispatch: loop {
        match pc {
            0x8266A988 => {
    //   block [0x8266A988..0x8266A9F4)
	// 8266A988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266A990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266A994: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266A998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266A99C: 38EBA310  addi r7, r11, -0x5cf0
	ctx.r[7].s64 = ctx.r[11].s64 + -23792;
	// 8266A9A0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8266A9A4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8266A9A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266A9AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266A9B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266A9B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266A9B8: 386AF310  addi r3, r10, -0xcf0
	ctx.r[3].s64 = ctx.r[10].s64 + -3312;
	// 8266A9BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266A9C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266A9C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266A9C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266A9CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266A9D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266A9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266A9D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266A9DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266A9E0: 4BDFC441  bl 0x82466e20
	ctx.lr = 0x8266A9E4;
	sub_82466E20(ctx, base);
	// 8266A9E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266A9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266A9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266A9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266A9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266A9F8 size=116
    let mut pc: u32 = 0x8266A9F8;
    'dispatch: loop {
        match pc {
            0x8266A9F8 => {
    //   block [0x8266A9F8..0x8266AA6C)
	// 8266A9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266A9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AA04: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266AA08: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 8266AA0C: 390AA400  addi r8, r10, -0x5c00
	ctx.r[8].s64 = ctx.r[10].s64 + -23552;
	// 8266AA10: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AA14: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8266AA18: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266AA1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AA20: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266AA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AA28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AA2C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8266AA30: 396BFFC8  addi r11, r11, -0x38
	ctx.r[11].s64 = ctx.r[11].s64 + -56;
	// 8266AA34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AA38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AA3C: 386AF340  addi r3, r10, -0xcc0
	ctx.r[3].s64 = ctx.r[10].s64 + -3264;
	// 8266AA40: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8266AA44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AA48: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8266AA4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AA54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AA58: 4BDFC3C9  bl 0x82466e20
	ctx.lr = 0x8266AA5C;
	sub_82466E20(ctx, base);
	// 8266AA5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AA70 size=108
    let mut pc: u32 = 0x8266AA70;
    'dispatch: loop {
        match pc {
            0x8266AA70 => {
    //   block [0x8266AA70..0x8266AADC)
	// 8266AA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AA7C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AA80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AA84: 38EBA5C8  addi r7, r11, -0x5a38
	ctx.r[7].s64 = ctx.r[11].s64 + -23096;
	// 8266AA88: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8266AA8C: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8266AA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AA94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AA98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266AA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AAA0: 386AF370  addi r3, r10, -0xc90
	ctx.r[3].s64 = ctx.r[10].s64 + -3216;
	// 8266AAA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266AAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AAAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AAC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266AAC8: 4BDFC359  bl 0x82466e20
	ctx.lr = 0x8266AACC;
	sub_82466E20(ctx, base);
	// 8266AACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AAD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AAD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AAD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AAE0 size=112
    let mut pc: u32 = 0x8266AAE0;
    'dispatch: loop {
        match pc {
            0x8266AAE0 => {
    //   block [0x8266AAE0..0x8266AB50)
	// 8266AAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AAEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AAF0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AAF4: 38AADDE0  addi r5, r10, -0x2220
	ctx.r[5].s64 = ctx.r[10].s64 + -8736;
	// 8266AAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AAFC: 390BA760  addi r8, r11, -0x58a0
	ctx.r[8].s64 = ctx.r[11].s64 + -22688;
	// 8266AB00: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8266AB04: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8266AB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AB0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AB10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AB14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AB18: 386AF3A0  addi r3, r10, -0xc60
	ctx.r[3].s64 = ctx.r[10].s64 + -3168;
	// 8266AB1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266AB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AB3C: 4BDFC2E5  bl 0x82466e20
	ctx.lr = 0x8266AB40;
	sub_82466E20(ctx, base);
	// 8266AB40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AB50 size=100
    let mut pc: u32 = 0x8266AB50;
    'dispatch: loop {
        match pc {
            0x8266AB50 => {
    //   block [0x8266AB50..0x8266ABB4)
	// 8266AB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AB5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AB64: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266AB68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AB70: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8266AB74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AB78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AB7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AB80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AB84: 386AF3D0  addi r3, r10, -0xc30
	ctx.r[3].s64 = ctx.r[10].s64 + -3120;
	// 8266AB88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AB8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AB90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266AB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AB98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266AB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266ABA0: 4BDFC281  bl 0x82466e20
	ctx.lr = 0x8266ABA4;
	sub_82466E20(ctx, base);
	// 8266ABA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266ABA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266ABAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266ABB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266ABB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266ABB8 size=112
    let mut pc: u32 = 0x8266ABB8;
    'dispatch: loop {
        match pc {
            0x8266ABB8 => {
    //   block [0x8266ABB8..0x8266AC28)
	// 8266ABB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266ABBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266ABC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266ABC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ABC8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266ABCC: 38AAF3D0  addi r5, r10, -0xc30
	ctx.r[5].s64 = ctx.r[10].s64 + -3120;
	// 8266ABD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266ABD4: 390BA9B8  addi r8, r11, -0x5648
	ctx.r[8].s64 = ctx.r[11].s64 + -22088;
	// 8266ABD8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8266ABDC: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8266ABE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266ABE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ABE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266ABEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266ABF0: 386AF400  addi r3, r10, -0xc00
	ctx.r[3].s64 = ctx.r[10].s64 + -3072;
	// 8266ABF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266ABF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266ABFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AC14: 4BDFC20D  bl 0x82466e20
	ctx.lr = 0x8266AC18;
	sub_82466E20(ctx, base);
	// 8266AC18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AC28 size=100
    let mut pc: u32 = 0x8266AC28;
    'dispatch: loop {
        match pc {
            0x8266AC28 => {
    //   block [0x8266AC28..0x8266AC8C)
	// 8266AC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AC34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AC3C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266AC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AC44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AC48: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8266AC4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AC5C: 386AF430  addi r3, r10, -0xbd0
	ctx.r[3].s64 = ctx.r[10].s64 + -3024;
	// 8266AC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AC64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AC68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266AC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AC70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266AC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AC78: 4BDFC1A9  bl 0x82466e20
	ctx.lr = 0x8266AC7C;
	sub_82466E20(ctx, base);
	// 8266AC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AC90 size=108
    let mut pc: u32 = 0x8266AC90;
    'dispatch: loop {
        match pc {
            0x8266AC90 => {
    //   block [0x8266AC90..0x8266ACFC)
	// 8266AC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AC9C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266ACA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266ACA4: 38EBAA30  addi r7, r11, -0x55d0
	ctx.r[7].s64 = ctx.r[11].s64 + -21968;
	// 8266ACA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266ACAC: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8266ACB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266ACB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ACB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266ACBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266ACC0: 386AF460  addi r3, r10, -0xba0
	ctx.r[3].s64 = ctx.r[10].s64 + -2976;
	// 8266ACC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266ACC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266ACCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266ACD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266ACD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266ACD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266ACDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266ACE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266ACE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266ACE8: 4BDFC139  bl 0x82466e20
	ctx.lr = 0x8266ACEC;
	sub_82466E20(ctx, base);
	// 8266ACEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266ACF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266ACF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266ACF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AD00 size=112
    let mut pc: u32 = 0x8266AD00;
    'dispatch: loop {
        match pc {
            0x8266AD00 => {
    //   block [0x8266AD00..0x8266AD70)
	// 8266AD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AD0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AD10: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AD14: 38AAF430  addi r5, r10, -0xbd0
	ctx.r[5].s64 = ctx.r[10].s64 + -3024;
	// 8266AD18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AD1C: 390BAA78  addi r8, r11, -0x5588
	ctx.r[8].s64 = ctx.r[11].s64 + -21896;
	// 8266AD20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266AD24: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8266AD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AD2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AD30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AD38: 386AF490  addi r3, r10, -0xb70
	ctx.r[3].s64 = ctx.r[10].s64 + -2928;
	// 8266AD3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266AD40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AD48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AD50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AD54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AD58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AD5C: 4BDFC0C5  bl 0x82466e20
	ctx.lr = 0x8266AD60;
	sub_82466E20(ctx, base);
	// 8266AD60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AD70 size=100
    let mut pc: u32 = 0x8266AD70;
    'dispatch: loop {
        match pc {
            0x8266AD70 => {
    //   block [0x8266AD70..0x8266ADD4)
	// 8266AD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AD7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AD84: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266AD88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AD90: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8266AD94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266ADA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266ADA4: 386AF4C0  addi r3, r10, -0xb40
	ctx.r[3].s64 = ctx.r[10].s64 + -2880;
	// 8266ADA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266ADAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266ADB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266ADB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266ADB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266ADBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266ADC0: 4BDFC061  bl 0x82466e20
	ctx.lr = 0x8266ADC4;
	sub_82466E20(ctx, base);
	// 8266ADC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266ADC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266ADCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266ADD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266ADD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266ADD8 size=100
    let mut pc: u32 = 0x8266ADD8;
    'dispatch: loop {
        match pc {
            0x8266ADD8 => {
    //   block [0x8266ADD8..0x8266AE3C)
	// 8266ADD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266ADDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266ADE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266ADE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266ADE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266ADEC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266ADF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266ADF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266ADF8: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8266ADFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AE0C: 386AF4F0  addi r3, r10, -0xb10
	ctx.r[3].s64 = ctx.r[10].s64 + -2832;
	// 8266AE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AE14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AE18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266AE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AE20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266AE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AE28: 4BDFBFF9  bl 0x82466e20
	ctx.lr = 0x8266AE2C;
	sub_82466E20(ctx, base);
	// 8266AE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AE40 size=112
    let mut pc: u32 = 0x8266AE40;
    'dispatch: loop {
        match pc {
            0x8266AE40 => {
    //   block [0x8266AE40..0x8266AEB0)
	// 8266AE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AE4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AE50: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AE54: 38AAF4C0  addi r5, r10, -0xb40
	ctx.r[5].s64 = ctx.r[10].s64 + -2880;
	// 8266AE58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AE5C: 390BAAA8  addi r8, r11, -0x5558
	ctx.r[8].s64 = ctx.r[11].s64 + -21848;
	// 8266AE60: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266AE64: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8266AE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AE6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AE78: 386AF520  addi r3, r10, -0xae0
	ctx.r[3].s64 = ctx.r[10].s64 + -2784;
	// 8266AE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266AE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AE9C: 4BDFBF85  bl 0x82466e20
	ctx.lr = 0x8266AEA0;
	sub_82466E20(ctx, base);
	// 8266AEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AEB0 size=112
    let mut pc: u32 = 0x8266AEB0;
    'dispatch: loop {
        match pc {
            0x8266AEB0 => {
    //   block [0x8266AEB0..0x8266AF20)
	// 8266AEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AEBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AEC0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AEC4: 38AAF4F0  addi r5, r10, -0xb10
	ctx.r[5].s64 = ctx.r[10].s64 + -2832;
	// 8266AEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AECC: 390BAB08  addi r8, r11, -0x54f8
	ctx.r[8].s64 = ctx.r[11].s64 + -21752;
	// 8266AED0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266AED4: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8266AED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AEDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AEE8: 386AF550  addi r3, r10, -0xab0
	ctx.r[3].s64 = ctx.r[10].s64 + -2736;
	// 8266AEEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266AEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AF0C: 4BDFBF15  bl 0x82466e20
	ctx.lr = 0x8266AF10;
	sub_82466E20(ctx, base);
	// 8266AF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AF20 size=100
    let mut pc: u32 = 0x8266AF20;
    'dispatch: loop {
        match pc {
            0x8266AF20 => {
    //   block [0x8266AF20..0x8266AF84)
	// 8266AF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AF2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AF34: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266AF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AF40: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8266AF44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AF54: 386AF580  addi r3, r10, -0xa80
	ctx.r[3].s64 = ctx.r[10].s64 + -2688;
	// 8266AF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AF5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AF60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266AF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AF68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266AF6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AF70: 4BDFBEB1  bl 0x82466e20
	ctx.lr = 0x8266AF74;
	sub_82466E20(ctx, base);
	// 8266AF74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AF88 size=112
    let mut pc: u32 = 0x8266AF88;
    'dispatch: loop {
        match pc {
            0x8266AF88 => {
    //   block [0x8266AF88..0x8266AFF8)
	// 8266AF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266AF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266AF94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AF98: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266AF9C: 38AAF580  addi r5, r10, -0xa80
	ctx.r[5].s64 = ctx.r[10].s64 + -2688;
	// 8266AFA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266AFA4: 390BAB68  addi r8, r11, -0x5498
	ctx.r[8].s64 = ctx.r[11].s64 + -21656;
	// 8266AFA8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8266AFAC: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8266AFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266AFB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266AFB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266AFBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266AFC0: 386AF5B0  addi r3, r10, -0xa50
	ctx.r[3].s64 = ctx.r[10].s64 + -2640;
	// 8266AFC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266AFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266AFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266AFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266AFD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266AFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266AFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266AFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266AFE4: 4BDFBE3D  bl 0x82466e20
	ctx.lr = 0x8266AFE8;
	sub_82466E20(ctx, base);
	// 8266AFE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266AFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266AFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266AFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266AFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266AFF8 size=108
    let mut pc: u32 = 0x8266AFF8;
    'dispatch: loop {
        match pc {
            0x8266AFF8 => {
    //   block [0x8266AFF8..0x8266B064)
	// 8266AFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266AFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B004: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B00C: 38EBAC58  addi r7, r11, -0x53a8
	ctx.r[7].s64 = ctx.r[11].s64 + -21416;
	// 8266B010: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8266B014: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8266B018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B01C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B028: 386AF5E0  addi r3, r10, -0xa20
	ctx.r[3].s64 = ctx.r[10].s64 + -2592;
	// 8266B02C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B04C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B050: 4BDFBDD1  bl 0x82466e20
	ctx.lr = 0x8266B054;
	sub_82466E20(ctx, base);
	// 8266B054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B068 size=108
    let mut pc: u32 = 0x8266B068;
    'dispatch: loop {
        match pc {
            0x8266B068 => {
    //   block [0x8266B068..0x8266B0D4)
	// 8266B068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B074: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B07C: 38EBAD48  addi r7, r11, -0x52b8
	ctx.r[7].s64 = ctx.r[11].s64 + -21176;
	// 8266B080: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266B084: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8266B088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B08C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B098: 386AF610  addi r3, r10, -0x9f0
	ctx.r[3].s64 = ctx.r[10].s64 + -2544;
	// 8266B09C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B0AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B0BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B0C0: 4BDFBD61  bl 0x82466e20
	ctx.lr = 0x8266B0C4;
	sub_82466E20(ctx, base);
	// 8266B0C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B0C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B0CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B0D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B0D8 size=108
    let mut pc: u32 = 0x8266B0D8;
    'dispatch: loop {
        match pc {
            0x8266B0D8 => {
    //   block [0x8266B0D8..0x8266B144)
	// 8266B0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B0E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B0EC: 38EBAD90  addi r7, r11, -0x5270
	ctx.r[7].s64 = ctx.r[11].s64 + -21104;
	// 8266B0F0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8266B0F4: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8266B0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B0FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B108: 386AF640  addi r3, r10, -0x9c0
	ctx.r[3].s64 = ctx.r[10].s64 + -2496;
	// 8266B10C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B12C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B130: 4BDFBCF1  bl 0x82466e20
	ctx.lr = 0x8266B134;
	sub_82466E20(ctx, base);
	// 8266B134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B148 size=108
    let mut pc: u32 = 0x8266B148;
    'dispatch: loop {
        match pc {
            0x8266B148 => {
    //   block [0x8266B148..0x8266B1B4)
	// 8266B148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B154: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B15C: 38EBAE68  addi r7, r11, -0x5198
	ctx.r[7].s64 = ctx.r[11].s64 + -20888;
	// 8266B160: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266B164: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8266B168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B16C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B178: 386AF670  addi r3, r10, -0x990
	ctx.r[3].s64 = ctx.r[10].s64 + -2448;
	// 8266B17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B1A0: 4BDFBC81  bl 0x82466e20
	ctx.lr = 0x8266B1A4;
	sub_82466E20(ctx, base);
	// 8266B1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B1B8 size=100
    let mut pc: u32 = 0x8266B1B8;
    'dispatch: loop {
        match pc {
            0x8266B1B8 => {
    //   block [0x8266B1B8..0x8266B21C)
	// 8266B1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B1C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B1CC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B1D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B1D8: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8266B1DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B1EC: 386AF6A0  addi r3, r10, -0x960
	ctx.r[3].s64 = ctx.r[10].s64 + -2400;
	// 8266B1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B1F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B1F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266B1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B200: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266B204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B208: 4BDFBC19  bl 0x82466e20
	ctx.lr = 0x8266B20C;
	sub_82466E20(ctx, base);
	// 8266B20C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B220 size=112
    let mut pc: u32 = 0x8266B220;
    'dispatch: loop {
        match pc {
            0x8266B220 => {
    //   block [0x8266B220..0x8266B290)
	// 8266B220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B22C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B230: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B234: 38AAF6A0  addi r5, r10, -0x960
	ctx.r[5].s64 = ctx.r[10].s64 + -2400;
	// 8266B238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B23C: 390BAE80  addi r8, r11, -0x5180
	ctx.r[8].s64 = ctx.r[11].s64 + -20864;
	// 8266B240: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266B244: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8266B248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B24C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B250: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B258: 386AF6D0  addi r3, r10, -0x930
	ctx.r[3].s64 = ctx.r[10].s64 + -2352;
	// 8266B25C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B27C: 4BDFBBA5  bl 0x82466e20
	ctx.lr = 0x8266B280;
	sub_82466E20(ctx, base);
	// 8266B280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B28C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B290 size=108
    let mut pc: u32 = 0x8266B290;
    'dispatch: loop {
        match pc {
            0x8266B290 => {
    //   block [0x8266B290..0x8266B2FC)
	// 8266B290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B29C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B2A4: 38EBAEC8  addi r7, r11, -0x5138
	ctx.r[7].s64 = ctx.r[11].s64 + -20792;
	// 8266B2A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266B2AC: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8266B2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B2B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B2B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B2C0: 386AF700  addi r3, r10, -0x900
	ctx.r[3].s64 = ctx.r[10].s64 + -2304;
	// 8266B2C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B2CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B2E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B2E8: 4BDFBB39  bl 0x82466e20
	ctx.lr = 0x8266B2EC;
	sub_82466E20(ctx, base);
	// 8266B2EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B300 size=112
    let mut pc: u32 = 0x8266B300;
    'dispatch: loop {
        match pc {
            0x8266B300 => {
    //   block [0x8266B300..0x8266B370)
	// 8266B300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B30C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B310: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B314: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B31C: 390BAF10  addi r8, r11, -0x50f0
	ctx.r[8].s64 = ctx.r[11].s64 + -20720;
	// 8266B320: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266B324: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8266B328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B32C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B338: 386AF730  addi r3, r10, -0x8d0
	ctx.r[3].s64 = ctx.r[10].s64 + -2256;
	// 8266B33C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B34C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B35C: 4BDFBAC5  bl 0x82466e20
	ctx.lr = 0x8266B360;
	sub_82466E20(ctx, base);
	// 8266B360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B370 size=108
    let mut pc: u32 = 0x8266B370;
    'dispatch: loop {
        match pc {
            0x8266B370 => {
    //   block [0x8266B370..0x8266B3DC)
	// 8266B370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B37C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B384: 38EBAF28  addi r7, r11, -0x50d8
	ctx.r[7].s64 = ctx.r[11].s64 + -20696;
	// 8266B388: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266B38C: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8266B390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B394: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B398: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B3A0: 386AF760  addi r3, r10, -0x8a0
	ctx.r[3].s64 = ctx.r[10].s64 + -2208;
	// 8266B3A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B3AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B3B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B3C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B3C8: 4BDFBA59  bl 0x82466e20
	ctx.lr = 0x8266B3CC;
	sub_82466E20(ctx, base);
	// 8266B3CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B3D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B3D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B3E0 size=112
    let mut pc: u32 = 0x8266B3E0;
    'dispatch: loop {
        match pc {
            0x8266B3E0 => {
    //   block [0x8266B3E0..0x8266B450)
	// 8266B3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B3EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B3F0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B3F4: 38AAF730  addi r5, r10, -0x8d0
	ctx.r[5].s64 = ctx.r[10].s64 + -2256;
	// 8266B3F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B3FC: 390BAF70  addi r8, r11, -0x5090
	ctx.r[8].s64 = ctx.r[11].s64 + -20624;
	// 8266B400: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266B404: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8266B408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B40C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B418: 386AF790  addi r3, r10, -0x870
	ctx.r[3].s64 = ctx.r[10].s64 + -2160;
	// 8266B41C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B43C: 4BDFB9E5  bl 0x82466e20
	ctx.lr = 0x8266B440;
	sub_82466E20(ctx, base);
	// 8266B440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B450 size=100
    let mut pc: u32 = 0x8266B450;
    'dispatch: loop {
        match pc {
            0x8266B450 => {
    //   block [0x8266B450..0x8266B4B4)
	// 8266B450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B45C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B464: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B468: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B46C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B470: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8266B474: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B484: 386AF7C0  addi r3, r10, -0x840
	ctx.r[3].s64 = ctx.r[10].s64 + -2112;
	// 8266B488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B48C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B490: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266B494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B498: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266B49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B4A0: 4BDFB981  bl 0x82466e20
	ctx.lr = 0x8266B4A4;
	sub_82466E20(ctx, base);
	// 8266B4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B4B8 size=112
    let mut pc: u32 = 0x8266B4B8;
    'dispatch: loop {
        match pc {
            0x8266B4B8 => {
    //   block [0x8266B4B8..0x8266B528)
	// 8266B4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B4C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B4C8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B4CC: 38AAF7C0  addi r5, r10, -0x840
	ctx.r[5].s64 = ctx.r[10].s64 + -2112;
	// 8266B4D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B4D4: 390BAF88  addi r8, r11, -0x5078
	ctx.r[8].s64 = ctx.r[11].s64 + -20600;
	// 8266B4D8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266B4DC: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 8266B4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B4E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B4F0: 386AF7F0  addi r3, r10, -0x810
	ctx.r[3].s64 = ctx.r[10].s64 + -2064;
	// 8266B4F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B514: 4BDFB90D  bl 0x82466e20
	ctx.lr = 0x8266B518;
	sub_82466E20(ctx, base);
	// 8266B518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B528 size=108
    let mut pc: u32 = 0x8266B528;
    'dispatch: loop {
        match pc {
            0x8266B528 => {
    //   block [0x8266B528..0x8266B594)
	// 8266B528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B534: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B53C: 38EBB030  addi r7, r11, -0x4fd0
	ctx.r[7].s64 = ctx.r[11].s64 + -20432;
	// 8266B540: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266B544: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 8266B548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B54C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B558: 386AF820  addi r3, r10, -0x7e0
	ctx.r[3].s64 = ctx.r[10].s64 + -2016;
	// 8266B55C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B580: 4BDFB8A1  bl 0x82466e20
	ctx.lr = 0x8266B584;
	sub_82466E20(ctx, base);
	// 8266B584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B598 size=112
    let mut pc: u32 = 0x8266B598;
    'dispatch: loop {
        match pc {
            0x8266B598 => {
    //   block [0x8266B598..0x8266B608)
	// 8266B598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B5A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B5A8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B5AC: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B5B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B5B4: 390BB060  addi r8, r11, -0x4fa0
	ctx.r[8].s64 = ctx.r[11].s64 + -20384;
	// 8266B5B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266B5BC: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 8266B5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B5C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B5D0: 386AF850  addi r3, r10, -0x7b0
	ctx.r[3].s64 = ctx.r[10].s64 + -1968;
	// 8266B5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B5F4: 4BDFB82D  bl 0x82466e20
	ctx.lr = 0x8266B5F8;
	sub_82466E20(ctx, base);
	// 8266B5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B608 size=112
    let mut pc: u32 = 0x8266B608;
    'dispatch: loop {
        match pc {
            0x8266B608 => {
    //   block [0x8266B608..0x8266B678)
	// 8266B608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B614: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B618: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B61C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B624: 390BB0A8  addi r8, r11, -0x4f58
	ctx.r[8].s64 = ctx.r[11].s64 + -20312;
	// 8266B628: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266B62C: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 8266B630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B634: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B640: 386AF880  addi r3, r10, -0x780
	ctx.r[3].s64 = ctx.r[10].s64 + -1920;
	// 8266B644: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B664: 4BDFB7BD  bl 0x82466e20
	ctx.lr = 0x8266B668;
	sub_82466E20(ctx, base);
	// 8266B668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B678 size=100
    let mut pc: u32 = 0x8266B678;
    'dispatch: loop {
        match pc {
            0x8266B678 => {
    //   block [0x8266B678..0x8266B6DC)
	// 8266B678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B684: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B68C: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B698: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8266B69C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B6AC: 386AF8B0  addi r3, r10, -0x750
	ctx.r[3].s64 = ctx.r[10].s64 + -1872;
	// 8266B6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B6B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B6B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266B6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B6C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266B6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B6C8: 4BDFB759  bl 0x82466e20
	ctx.lr = 0x8266B6CC;
	sub_82466E20(ctx, base);
	// 8266B6CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B6D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B6D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B6E0 size=112
    let mut pc: u32 = 0x8266B6E0;
    'dispatch: loop {
        match pc {
            0x8266B6E0 => {
    //   block [0x8266B6E0..0x8266B750)
	// 8266B6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B6EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B6F0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B6F4: 38AAF8B0  addi r5, r10, -0x750
	ctx.r[5].s64 = ctx.r[10].s64 + -1872;
	// 8266B6F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B6FC: 390BB0F0  addi r8, r11, -0x4f10
	ctx.r[8].s64 = ctx.r[11].s64 + -20240;
	// 8266B700: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266B704: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 8266B708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B70C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B718: 386AF8E0  addi r3, r10, -0x720
	ctx.r[3].s64 = ctx.r[10].s64 + -1824;
	// 8266B71C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B73C: 4BDFB6E5  bl 0x82466e20
	ctx.lr = 0x8266B740;
	sub_82466E20(ctx, base);
	// 8266B740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B750 size=112
    let mut pc: u32 = 0x8266B750;
    'dispatch: loop {
        match pc {
            0x8266B750 => {
    //   block [0x8266B750..0x8266B7C0)
	// 8266B750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B75C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B760: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B764: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B76C: 390BB138  addi r8, r11, -0x4ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -20168;
	// 8266B770: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266B774: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 8266B778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B77C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B788: 386AF910  addi r3, r10, -0x6f0
	ctx.r[3].s64 = ctx.r[10].s64 + -1776;
	// 8266B78C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B7AC: 4BDFB675  bl 0x82466e20
	ctx.lr = 0x8266B7B0;
	sub_82466E20(ctx, base);
	// 8266B7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B7C0 size=112
    let mut pc: u32 = 0x8266B7C0;
    'dispatch: loop {
        match pc {
            0x8266B7C0 => {
    //   block [0x8266B7C0..0x8266B830)
	// 8266B7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B7CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B7D0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B7D4: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8266B7D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B7DC: 390BB150  addi r8, r11, -0x4eb0
	ctx.r[8].s64 = ctx.r[11].s64 + -20144;
	// 8266B7E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266B7E4: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 8266B7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B7EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B7F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B7F8: 386AF940  addi r3, r10, -0x6c0
	ctx.r[3].s64 = ctx.r[10].s64 + -1728;
	// 8266B7FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B80C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266B810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B81C: 4BDFB605  bl 0x82466e20
	ctx.lr = 0x8266B820;
	sub_82466E20(ctx, base);
	// 8266B820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B830 size=112
    let mut pc: u32 = 0x8266B830;
    'dispatch: loop {
        match pc {
            0x8266B830 => {
    //   block [0x8266B830..0x8266B8A0)
	// 8266B830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B83C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B840: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B844: 38AAF910  addi r5, r10, -0x6f0
	ctx.r[5].s64 = ctx.r[10].s64 + -1776;
	// 8266B848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B84C: 390BB168  addi r8, r11, -0x4e98
	ctx.r[8].s64 = ctx.r[11].s64 + -20120;
	// 8266B850: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8266B854: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 8266B858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B85C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266B864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B868: 386AF970  addi r3, r10, -0x690
	ctx.r[3].s64 = ctx.r[10].s64 + -1680;
	// 8266B86C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266B870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B88C: 4BDFB595  bl 0x82466e20
	ctx.lr = 0x8266B890;
	sub_82466E20(ctx, base);
	// 8266B890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B8A0 size=72
    let mut pc: u32 = 0x8266B8A0;
    'dispatch: loop {
        match pc {
            0x8266B8A0 => {
    //   block [0x8266B8A0..0x8266B8E8)
	// 8266B8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B8A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B8AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8266B8B0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8266B8B4: 38CBE750  addi r6, r11, -0x18b0
	ctx.r[6].s64 = ctx.r[11].s64 + -6320;
	// 8266B8B8: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8266B8BC: 388B0018  addi r4, r11, 0x18
	ctx.r[4].s64 = ctx.r[11].s64 + 24;
	// 8266B8C0: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8266B8C4: 386BF9A0  addi r3, r11, -0x660
	ctx.r[3].s64 = ctx.r[11].s64 + -1632;
	// 8266B8C8: 4BE101C1  bl 0x8247ba88
	ctx.lr = 0x8266B8CC;
	sub_8247BA88(ctx, base);
	// 8266B8CC: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8266B8D0: 386BCDF8  addi r3, r11, -0x3208
	ctx.r[3].s64 = ctx.r[11].s64 + -12808;
	// 8266B8D4: 4BEC7265  bl 0x82532b38
	ctx.lr = 0x8266B8D8;
	sub_82532B38(ctx, base);
	// 8266B8D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8266B8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B8E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B8E8 size=108
    let mut pc: u32 = 0x8266B8E8;
    'dispatch: loop {
        match pc {
            0x8266B8E8 => {
    //   block [0x8266B8E8..0x8266B954)
	// 8266B8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B8F4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B8F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B8FC: 38EBD040  addi r7, r11, -0x2fc0
	ctx.r[7].s64 = ctx.r[11].s64 + -12224;
	// 8266B900: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8266B904: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 8266B908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266B90C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B910: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266B914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266B918: 386AF9B8  addi r3, r10, -0x648
	ctx.r[3].s64 = ctx.r[10].s64 + -1608;
	// 8266B91C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266B920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266B924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266B92C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266B934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266B93C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B940: 4BDFB4E1  bl 0x82466e20
	ctx.lr = 0x8266B944;
	sub_82466E20(ctx, base);
	// 8266B944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266B958 size=24
    let mut pc: u32 = 0x8266B958;
    'dispatch: loop {
        match pc {
            0x8266B958 => {
    //   block [0x8266B958..0x8266B970)
	// 8266B958: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B95C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266B960: 394A1FA8  addi r10, r10, 0x1fa8
	ctx.r[10].s64 = ctx.r[10].s64 + 8104;
	// 8266B964: 816BD0B8  lwz r11, -0x2f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12104 as u32) ) } as u64;
	// 8266B968: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8266B96C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B970 size=112
    let mut pc: u32 = 0x8266B970;
    'dispatch: loop {
        match pc {
            0x8266B970 => {
    //   block [0x8266B970..0x8266B9E0)
	// 8266B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B97C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8266B980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B984: 392B1274  addi r9, r11, 0x1274
	ctx.r[9].s64 = ctx.r[11].s64 + 4724;
	// 8266B988: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 8266B98C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8266B990: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B994: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 8266B998: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266B99C: 396B1FA8  addi r11, r11, 0x1fa8
	ctx.r[11].s64 = ctx.r[11].s64 + 8104;
	// 8266B9A0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8266B9A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266B9A8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8266B9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266B9B0: 386AF9E8  addi r3, r10, -0x618
	ctx.r[3].s64 = ctx.r[10].s64 + -1560;
	// 8266B9B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266B9B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8266B9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266B9C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8266B9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266B9C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266B9CC: 4BDFB455  bl 0x82466e20
	ctx.lr = 0x8266B9D0;
	sub_82466E20(ctx, base);
	// 8266B9D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266B9D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266B9D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266B9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266B9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266B9E0 size=108
    let mut pc: u32 = 0x8266B9E0;
    'dispatch: loop {
        match pc {
            0x8266B9E0 => {
    //   block [0x8266B9E0..0x8266BA4C)
	// 8266B9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266B9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266B9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266B9EC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266B9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266B9F4: 38EBD0BC  addi r7, r11, -0x2f44
	ctx.r[7].s64 = ctx.r[11].s64 + -12100;
	// 8266B9F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266B9FC: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 8266BA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BA04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BA08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BA10: 386AFA18  addi r3, r10, -0x5e8
	ctx.r[3].s64 = ctx.r[10].s64 + -1512;
	// 8266BA14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BA34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BA38: 4BDFB3E9  bl 0x82466e20
	ctx.lr = 0x8266BA3C;
	sub_82466E20(ctx, base);
	// 8266BA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BA50 size=108
    let mut pc: u32 = 0x8266BA50;
    'dispatch: loop {
        match pc {
            0x8266BA50 => {
    //   block [0x8266BA50..0x8266BABC)
	// 8266BA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BA5C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BA64: 38EBD0EC  addi r7, r11, -0x2f14
	ctx.r[7].s64 = ctx.r[11].s64 + -12052;
	// 8266BA68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266BA6C: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 8266BA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BA74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BA80: 386AFA48  addi r3, r10, -0x5b8
	ctx.r[3].s64 = ctx.r[10].s64 + -1464;
	// 8266BA84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BAA8: 4BDFB379  bl 0x82466e20
	ctx.lr = 0x8266BAAC;
	sub_82466E20(ctx, base);
	// 8266BAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BAC0 size=112
    let mut pc: u32 = 0x8266BAC0;
    'dispatch: loop {
        match pc {
            0x8266BAC0 => {
    //   block [0x8266BAC0..0x8266BB30)
	// 8266BAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BACC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BAD0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BAD4: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266BAD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BADC: 390BD120  addi r8, r11, -0x2ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -12000;
	// 8266BAE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266BAE4: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 8266BAE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BAEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BAF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266BAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BAF8: 386AFA78  addi r3, r10, -0x588
	ctx.r[3].s64 = ctx.r[10].s64 + -1416;
	// 8266BAFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266BB00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BB08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BB0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BB10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BB14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BB18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BB1C: 4BDFB305  bl 0x82466e20
	ctx.lr = 0x8266BB20;
	sub_82466E20(ctx, base);
	// 8266BB20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BB30 size=108
    let mut pc: u32 = 0x8266BB30;
    'dispatch: loop {
        match pc {
            0x8266BB30 => {
    //   block [0x8266BB30..0x8266BB9C)
	// 8266BB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BB3C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BB40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BB44: 38EBD180  addi r7, r11, -0x2e80
	ctx.r[7].s64 = ctx.r[11].s64 + -11904;
	// 8266BB48: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266BB4C: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 8266BB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BB54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BB58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BB60: 386AFAA8  addi r3, r10, -0x558
	ctx.r[3].s64 = ctx.r[10].s64 + -1368;
	// 8266BB64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BB74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BB7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BB84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BB88: 4BDFB299  bl 0x82466e20
	ctx.lr = 0x8266BB8C;
	sub_82466E20(ctx, base);
	// 8266BB8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BB90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BB94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BB98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BBA0 size=112
    let mut pc: u32 = 0x8266BBA0;
    'dispatch: loop {
        match pc {
            0x8266BBA0 => {
    //   block [0x8266BBA0..0x8266BC10)
	// 8266BBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BBA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BBAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BBB0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BBB4: 38AAFA78  addi r5, r10, -0x588
	ctx.r[5].s64 = ctx.r[10].s64 + -1416;
	// 8266BBB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BBBC: 390BD1E0  addi r8, r11, -0x2e20
	ctx.r[8].s64 = ctx.r[11].s64 + -11808;
	// 8266BBC0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8266BBC4: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 8266BBC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BBCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BBD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266BBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BBD8: 386AFAD8  addi r3, r10, -0x528
	ctx.r[3].s64 = ctx.r[10].s64 + -1320;
	// 8266BBDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266BBE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BBE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BBF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BBFC: 4BDFB225  bl 0x82466e20
	ctx.lr = 0x8266BC00;
	sub_82466E20(ctx, base);
	// 8266BC00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BC04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BC08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BC0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BC10 size=112
    let mut pc: u32 = 0x8266BC10;
    'dispatch: loop {
        match pc {
            0x8266BC10 => {
    //   block [0x8266BC10..0x8266BC80)
	// 8266BC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BC1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BC20: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BC24: 38AAFA78  addi r5, r10, -0x588
	ctx.r[5].s64 = ctx.r[10].s64 + -1416;
	// 8266BC28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BC2C: 390BD270  addi r8, r11, -0x2d90
	ctx.r[8].s64 = ctx.r[11].s64 + -11664;
	// 8266BC30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266BC34: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 8266BC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BC3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BC40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266BC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BC48: 386AFB08  addi r3, r10, -0x4f8
	ctx.r[3].s64 = ctx.r[10].s64 + -1272;
	// 8266BC4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266BC50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BC54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BC58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BC5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BC60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BC64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BC68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BC6C: 4BDFB1B5  bl 0x82466e20
	ctx.lr = 0x8266BC70;
	sub_82466E20(ctx, base);
	// 8266BC70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BC80 size=108
    let mut pc: u32 = 0x8266BC80;
    'dispatch: loop {
        match pc {
            0x8266BC80 => {
    //   block [0x8266BC80..0x8266BCEC)
	// 8266BC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BC8C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BC94: 38EBD288  addi r7, r11, -0x2d78
	ctx.r[7].s64 = ctx.r[11].s64 + -11640;
	// 8266BC98: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266BC9C: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 8266BCA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BCA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BCA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BCAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BCB0: 386AFB38  addi r3, r10, -0x4c8
	ctx.r[3].s64 = ctx.r[10].s64 + -1224;
	// 8266BCB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BCB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BCBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BCC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BCC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BCCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BCD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BCD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BCD8: 4BDFB149  bl 0x82466e20
	ctx.lr = 0x8266BCDC;
	sub_82466E20(ctx, base);
	// 8266BCDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BCF0 size=112
    let mut pc: u32 = 0x8266BCF0;
    'dispatch: loop {
        match pc {
            0x8266BCF0 => {
    //   block [0x8266BCF0..0x8266BD60)
	// 8266BCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BCFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BD00: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BD04: 38AAFA78  addi r5, r10, -0x588
	ctx.r[5].s64 = ctx.r[10].s64 + -1416;
	// 8266BD08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BD0C: 390BD2E8  addi r8, r11, -0x2d18
	ctx.r[8].s64 = ctx.r[11].s64 + -11544;
	// 8266BD10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8266BD14: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 8266BD18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BD1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BD20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266BD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BD28: 386AFB68  addi r3, r10, -0x498
	ctx.r[3].s64 = ctx.r[10].s64 + -1176;
	// 8266BD2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266BD30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BD34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BD4C: 4BDFB0D5  bl 0x82466e20
	ctx.lr = 0x8266BD50;
	sub_82466E20(ctx, base);
	// 8266BD50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BD54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BD58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BD5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BD60 size=108
    let mut pc: u32 = 0x8266BD60;
    'dispatch: loop {
        match pc {
            0x8266BD60 => {
    //   block [0x8266BD60..0x8266BDCC)
	// 8266BD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BD6C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BD74: 38EBD390  addi r7, r11, -0x2c70
	ctx.r[7].s64 = ctx.r[11].s64 + -11376;
	// 8266BD78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266BD7C: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 8266BD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BD84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BD88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BD90: 386AFB98  addi r3, r10, -0x468
	ctx.r[3].s64 = ctx.r[10].s64 + -1128;
	// 8266BD94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BDB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BDB8: 4BDFB069  bl 0x82466e20
	ctx.lr = 0x8266BDBC;
	sub_82466E20(ctx, base);
	// 8266BDBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BDD0 size=108
    let mut pc: u32 = 0x8266BDD0;
    'dispatch: loop {
        match pc {
            0x8266BDD0 => {
    //   block [0x8266BDD0..0x8266BE3C)
	// 8266BDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BDDC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BDE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BDE4: 38EBD3A8  addi r7, r11, -0x2c58
	ctx.r[7].s64 = ctx.r[11].s64 + -11352;
	// 8266BDE8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266BDEC: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 8266BDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BDF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BDF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BE00: 386AFBC8  addi r3, r10, -0x438
	ctx.r[3].s64 = ctx.r[10].s64 + -1080;
	// 8266BE04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BE28: 4BDFAFF9  bl 0x82466e20
	ctx.lr = 0x8266BE2C;
	sub_82466E20(ctx, base);
	// 8266BE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BE40 size=112
    let mut pc: u32 = 0x8266BE40;
    'dispatch: loop {
        match pc {
            0x8266BE40 => {
    //   block [0x8266BE40..0x8266BEB0)
	// 8266BE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BE4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BE50: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BE54: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266BE58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BE5C: 390BD408  addi r8, r11, -0x2bf8
	ctx.r[8].s64 = ctx.r[11].s64 + -11256;
	// 8266BE60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8266BE64: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8266BE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BE6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266BE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BE78: 386AFBF8  addi r3, r10, -0x408
	ctx.r[3].s64 = ctx.r[10].s64 + -1032;
	// 8266BE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266BE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BE9C: 4BDFAF85  bl 0x82466e20
	ctx.lr = 0x8266BEA0;
	sub_82466E20(ctx, base);
	// 8266BEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BEB0 size=108
    let mut pc: u32 = 0x8266BEB0;
    'dispatch: loop {
        match pc {
            0x8266BEB0 => {
    //   block [0x8266BEB0..0x8266BF1C)
	// 8266BEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BEBC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BEC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BEC4: 38EBD420  addi r7, r11, -0x2be0
	ctx.r[7].s64 = ctx.r[11].s64 + -11232;
	// 8266BEC8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266BECC: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 8266BED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BED4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BED8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BEE0: 386AFC28  addi r3, r10, -0x3d8
	ctx.r[3].s64 = ctx.r[10].s64 + -984;
	// 8266BEE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BEEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BF04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BF08: 4BDFAF19  bl 0x82466e20
	ctx.lr = 0x8266BF0C;
	sub_82466E20(ctx, base);
	// 8266BF0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BF20 size=108
    let mut pc: u32 = 0x8266BF20;
    'dispatch: loop {
        match pc {
            0x8266BF20 => {
    //   block [0x8266BF20..0x8266BF8C)
	// 8266BF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BF2C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BF34: 38EBD468  addi r7, r11, -0x2b98
	ctx.r[7].s64 = ctx.r[11].s64 + -11160;
	// 8266BF38: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8266BF3C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 8266BF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BF44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BF48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BF50: 386AFC58  addi r3, r10, -0x3a8
	ctx.r[3].s64 = ctx.r[10].s64 + -936;
	// 8266BF54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BF58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BF6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BF74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BF78: 4BDFAEA9  bl 0x82466e20
	ctx.lr = 0x8266BF7C;
	sub_82466E20(ctx, base);
	// 8266BF7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BF80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BF84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266BF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266BF90 size=108
    let mut pc: u32 = 0x8266BF90;
    'dispatch: loop {
        match pc {
            0x8266BF90 => {
    //   block [0x8266BF90..0x8266BFFC)
	// 8266BF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266BF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266BF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266BF9C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266BFA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266BFA4: 38EBD4F8  addi r7, r11, -0x2b08
	ctx.r[7].s64 = ctx.r[11].s64 + -11016;
	// 8266BFA8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8266BFAC: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 8266BFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266BFB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266BFB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266BFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266BFC0: 386AFC88  addi r3, r10, -0x378
	ctx.r[3].s64 = ctx.r[10].s64 + -888;
	// 8266BFC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266BFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266BFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266BFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266BFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266BFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266BFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266BFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266BFE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266BFE8: 4BDFAE39  bl 0x82466e20
	ctx.lr = 0x8266BFEC;
	sub_82466E20(ctx, base);
	// 8266BFEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266BFF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266BFF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266BFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C000 size=100
    let mut pc: u32 = 0x8266C000;
    'dispatch: loop {
        match pc {
            0x8266C000 => {
    //   block [0x8266C000..0x8266C064)
	// 8266C000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C00C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C014: 38AAFDD8  addi r5, r10, -0x228
	ctx.r[5].s64 = ctx.r[10].s64 + -552;
	// 8266C018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C020: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 8266C024: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C034: 386AFCB8  addi r3, r10, -0x348
	ctx.r[3].s64 = ctx.r[10].s64 + -840;
	// 8266C038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C03C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C040: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266C044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C048: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266C04C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C050: 4BDFADD1  bl 0x82466e20
	ctx.lr = 0x8266C054;
	sub_82466E20(ctx, base);
	// 8266C054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C05C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C068 size=112
    let mut pc: u32 = 0x8266C068;
    'dispatch: loop {
        match pc {
            0x8266C068 => {
    //   block [0x8266C068..0x8266C0D8)
	// 8266C068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C06C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C074: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C078: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C07C: 38AAFCB8  addi r5, r10, -0x348
	ctx.r[5].s64 = ctx.r[10].s64 + -840;
	// 8266C080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C084: 390BD588  addi r8, r11, -0x2a78
	ctx.r[8].s64 = ctx.r[11].s64 + -10872;
	// 8266C088: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8266C08C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 8266C090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C094: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C09C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C0A0: 386AFCE8  addi r3, r10, -0x318
	ctx.r[3].s64 = ctx.r[10].s64 + -792;
	// 8266C0A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266C0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C0BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C0C4: 4BDFAD5D  bl 0x82466e20
	ctx.lr = 0x8266C0C8;
	sub_82466E20(ctx, base);
	// 8266C0C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C0D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C0D8 size=108
    let mut pc: u32 = 0x8266C0D8;
    'dispatch: loop {
        match pc {
            0x8266C0D8 => {
    //   block [0x8266C0D8..0x8266C144)
	// 8266C0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C0E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C0E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C0E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C0EC: 38EBD5E8  addi r7, r11, -0x2a18
	ctx.r[7].s64 = ctx.r[11].s64 + -10776;
	// 8266C0F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C0F4: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 8266C0F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C0FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C108: 386AFD18  addi r3, r10, -0x2e8
	ctx.r[3].s64 = ctx.r[10].s64 + -744;
	// 8266C10C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C12C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C130: 4BDFACF1  bl 0x82466e20
	ctx.lr = 0x8266C134;
	sub_82466E20(ctx, base);
	// 8266C134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C148 size=108
    let mut pc: u32 = 0x8266C148;
    'dispatch: loop {
        match pc {
            0x8266C148 => {
    //   block [0x8266C148..0x8266C1B4)
	// 8266C148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C14C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C154: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C15C: 38EBD618  addi r7, r11, -0x29e8
	ctx.r[7].s64 = ctx.r[11].s64 + -10728;
	// 8266C160: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8266C164: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 8266C168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C16C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C178: 386AFD48  addi r3, r10, -0x2b8
	ctx.r[3].s64 = ctx.r[10].s64 + -696;
	// 8266C17C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C19C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C1A0: 4BDFAC81  bl 0x82466e20
	ctx.lr = 0x8266C1A4;
	sub_82466E20(ctx, base);
	// 8266C1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C1B8 size=108
    let mut pc: u32 = 0x8266C1B8;
    'dispatch: loop {
        match pc {
            0x8266C1B8 => {
    //   block [0x8266C1B8..0x8266C224)
	// 8266C1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C1C4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C1CC: 38EBD660  addi r7, r11, -0x29a0
	ctx.r[7].s64 = ctx.r[11].s64 + -10656;
	// 8266C1D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8266C1D4: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 8266C1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C1DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C1E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C1E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C1E8: 386AFD78  addi r3, r10, -0x288
	ctx.r[3].s64 = ctx.r[10].s64 + -648;
	// 8266C1EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C20C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C210: 4BDFAC11  bl 0x82466e20
	ctx.lr = 0x8266C214;
	sub_82466E20(ctx, base);
	// 8266C214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C228 size=96
    let mut pc: u32 = 0x8266C228;
    'dispatch: loop {
        match pc {
            0x8266C228 => {
    //   block [0x8266C228..0x8266C288)
	// 8266C228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C234: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C23C: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 8266C240: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C248: 386AFDA8  addi r3, r10, -0x258
	ctx.r[3].s64 = ctx.r[10].s64 + -600;
	// 8266C24C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C254: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266C258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C268: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8266C26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C270: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8266C274: 4BDFABAD  bl 0x82466e20
	ctx.lr = 0x8266C278;
	sub_82466E20(ctx, base);
	// 8266C278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C288 size=112
    let mut pc: u32 = 0x8266C288;
    'dispatch: loop {
        match pc {
            0x8266C288 => {
    //   block [0x8266C288..0x8266C2F8)
	// 8266C288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C294: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C298: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C29C: 38AAFDA8  addi r5, r10, -0x258
	ctx.r[5].s64 = ctx.r[10].s64 + -600;
	// 8266C2A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C2A4: 390BD6C0  addi r8, r11, -0x2940
	ctx.r[8].s64 = ctx.r[11].s64 + -10560;
	// 8266C2A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8266C2AC: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 8266C2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C2B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C2C0: 386AFDD8  addi r3, r10, -0x228
	ctx.r[3].s64 = ctx.r[10].s64 + -552;
	// 8266C2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8266C2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C2E4: 4BDFAB3D  bl 0x82466e20
	ctx.lr = 0x8266C2E8;
	sub_82466E20(ctx, base);
	// 8266C2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C2F8 size=112
    let mut pc: u32 = 0x8266C2F8;
    'dispatch: loop {
        match pc {
            0x8266C2F8 => {
    //   block [0x8266C2F8..0x8266C368)
	// 8266C2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C304: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266C308: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C30C: 392A12A0  addi r9, r10, 0x12a0
	ctx.r[9].s64 = ctx.r[10].s64 + 4768;
	// 8266C310: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C314: 390BD6F0  addi r8, r11, -0x2910
	ctx.r[8].s64 = ctx.r[11].s64 + -10512;
	// 8266C318: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8266C31C: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8266C320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C324: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C330: 386AFE08  addi r3, r10, -0x1f8
	ctx.r[3].s64 = ctx.r[10].s64 + -504;
	// 8266C334: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266C338: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266C33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C354: 4BDFAACD  bl 0x82466e20
	ctx.lr = 0x8266C358;
	sub_82466E20(ctx, base);
	// 8266C358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C368 size=108
    let mut pc: u32 = 0x8266C368;
    'dispatch: loop {
        match pc {
            0x8266C368 => {
    //   block [0x8266C368..0x8266C3D4)
	// 8266C368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C374: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C378: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C37C: 38EBD798  addi r7, r11, -0x2868
	ctx.r[7].s64 = ctx.r[11].s64 + -10344;
	// 8266C380: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C384: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8266C388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C38C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C398: 386AFE38  addi r3, r10, -0x1c8
	ctx.r[3].s64 = ctx.r[10].s64 + -456;
	// 8266C39C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C3BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C3C0: 4BDFAA61  bl 0x82466e20
	ctx.lr = 0x8266C3C4;
	sub_82466E20(ctx, base);
	// 8266C3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C3D8 size=108
    let mut pc: u32 = 0x8266C3D8;
    'dispatch: loop {
        match pc {
            0x8266C3D8 => {
    //   block [0x8266C3D8..0x8266C444)
	// 8266C3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C3E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C3E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C3EC: 38EBD7C8  addi r7, r11, -0x2838
	ctx.r[7].s64 = ctx.r[11].s64 + -10296;
	// 8266C3F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C3F4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8266C3F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C3FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C400: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C408: 386AFE68  addi r3, r10, -0x198
	ctx.r[3].s64 = ctx.r[10].s64 + -408;
	// 8266C40C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C41C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C42C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C430: 4BDFA9F1  bl 0x82466e20
	ctx.lr = 0x8266C434;
	sub_82466E20(ctx, base);
	// 8266C434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C43C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266C448 size=28
    let mut pc: u32 = 0x8266C448;
    'dispatch: loop {
        match pc {
            0x8266C448 => {
    //   block [0x8266C448..0x8266C464)
	// 8266C448: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C44C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266C450: 394A1FF0  addi r10, r10, 0x1ff0
	ctx.r[10].s64 = ctx.r[10].s64 + 8176;
	// 8266C454: 816BD7F8  lwz r11, -0x2808(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10248 as u32) ) } as u64;
	// 8266C458: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8266C45C: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8266C460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C468 size=112
    let mut pc: u32 = 0x8266C468;
    'dispatch: loop {
        match pc {
            0x8266C468 => {
    //   block [0x8266C468..0x8266C4D8)
	// 8266C468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C474: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266C478: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C47C: 392A1410  addi r9, r10, 0x1410
	ctx.r[9].s64 = ctx.r[10].s64 + 5136;
	// 8266C480: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C484: 390B1FF0  addi r8, r11, 0x1ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 8176;
	// 8266C488: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8266C48C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8266C490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C494: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C4A0: 386AFE98  addi r3, r10, -0x168
	ctx.r[3].s64 = ctx.r[10].s64 + -360;
	// 8266C4A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266C4A8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8266C4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C4BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C4C4: 4BDFA95D  bl 0x82466e20
	ctx.lr = 0x8266C4C8;
	sub_82466E20(ctx, base);
	// 8266C4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C4D8 size=108
    let mut pc: u32 = 0x8266C4D8;
    'dispatch: loop {
        match pc {
            0x8266C4D8 => {
    //   block [0x8266C4D8..0x8266C544)
	// 8266C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C4E4: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C4E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C4EC: 38EBD804  addi r7, r11, -0x27fc
	ctx.r[7].s64 = ctx.r[11].s64 + -10236;
	// 8266C4F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C4F4: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 8266C4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C4FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C508: 386AFEC8  addi r3, r10, -0x138
	ctx.r[3].s64 = ctx.r[10].s64 + -312;
	// 8266C50C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C52C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C530: 4BDFA8F1  bl 0x82466e20
	ctx.lr = 0x8266C534;
	sub_82466E20(ctx, base);
	// 8266C534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C548 size=108
    let mut pc: u32 = 0x8266C548;
    'dispatch: loop {
        match pc {
            0x8266C548 => {
    //   block [0x8266C548..0x8266C5B4)
	// 8266C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C554: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C558: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C55C: 38EBD834  addi r7, r11, -0x27cc
	ctx.r[7].s64 = ctx.r[11].s64 + -10188;
	// 8266C560: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8266C564: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 8266C568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C56C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C578: 386AFEF8  addi r3, r10, -0x108
	ctx.r[3].s64 = ctx.r[10].s64 + -264;
	// 8266C57C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C59C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C5A0: 4BDFA881  bl 0x82466e20
	ctx.lr = 0x8266C5A4;
	sub_82466E20(ctx, base);
	// 8266C5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8266C5B8 size=24
    let mut pc: u32 = 0x8266C5B8;
    'dispatch: loop {
        match pc {
            0x8266C5B8 => {
    //   block [0x8266C5B8..0x8266C5D0)
	// 8266C5B8: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C5BC: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8266C5C0: 394A20B0  addi r10, r10, 0x20b0
	ctx.r[10].s64 = ctx.r[10].s64 + 8368;
	// 8266C5C4: 816BD84C  lwz r11, -0x27b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10164 as u32) ) } as u64;
	// 8266C5C8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8266C5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C5D0 size=112
    let mut pc: u32 = 0x8266C5D0;
    'dispatch: loop {
        match pc {
            0x8266C5D0 => {
    //   block [0x8266C5D0..0x8266C640)
	// 8266C5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C5DC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266C5E0: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C5E4: 392A1464  addi r9, r10, 0x1464
	ctx.r[9].s64 = ctx.r[10].s64 + 5220;
	// 8266C5E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C5EC: 390B20B0  addi r8, r11, 0x20b0
	ctx.r[8].s64 = ctx.r[11].s64 + 8368;
	// 8266C5F0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8266C5F4: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8266C5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C5FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C608: 386AFF28  addi r3, r10, -0xd8
	ctx.r[3].s64 = ctx.r[10].s64 + -216;
	// 8266C60C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266C610: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266C614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C61C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C62C: 4BDFA7F5  bl 0x82466e20
	ctx.lr = 0x8266C630;
	sub_82466E20(ctx, base);
	// 8266C630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C640 size=108
    let mut pc: u32 = 0x8266C640;
    'dispatch: loop {
        match pc {
            0x8266C640 => {
    //   block [0x8266C640..0x8266C6AC)
	// 8266C640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C64C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C650: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C654: 38EBD850  addi r7, r11, -0x27b0
	ctx.r[7].s64 = ctx.r[11].s64 + -10160;
	// 8266C658: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C65C: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8266C660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C670: 386AFF58  addi r3, r10, -0xa8
	ctx.r[3].s64 = ctx.r[10].s64 + -168;
	// 8266C674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C68C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C698: 4BDFA789  bl 0x82466e20
	ctx.lr = 0x8266C69C;
	sub_82466E20(ctx, base);
	// 8266C69C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C6A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C6A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C6A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C6B0 size=108
    let mut pc: u32 = 0x8266C6B0;
    'dispatch: loop {
        match pc {
            0x8266C6B0 => {
    //   block [0x8266C6B0..0x8266C71C)
	// 8266C6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C6BC: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C6C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8266C6C4: 38EBD880  addi r7, r11, -0x2780
	ctx.r[7].s64 = ctx.r[11].s64 + -10112;
	// 8266C6C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8266C6CC: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8266C6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C6D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C6D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8266C6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8266C6E0: 386AFF88  addi r3, r10, -0x78
	ctx.r[3].s64 = ctx.r[10].s64 + -120;
	// 8266C6E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8266C6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8266C6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C708: 4BDFA719  bl 0x82466e20
	ctx.lr = 0x8266C70C;
	sub_82466E20(ctx, base);
	// 8266C70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8266C720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8266C720 size=112
    let mut pc: u32 = 0x8266C720;
    'dispatch: loop {
        match pc {
            0x8266C720 => {
    //   block [0x8266C720..0x8266C790)
	// 8266C720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8266C724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8266C728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8266C72C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8266C730: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8266C734: 392A1488  addi r9, r10, 0x1488
	ctx.r[9].s64 = ctx.r[10].s64 + 5256;
	// 8266C738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8266C73C: 390BD8B8  addi r8, r11, -0x2748
	ctx.r[8].s64 = ctx.r[11].s64 + -10056;
	// 8266C740: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8266C744: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8266C748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8266C74C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8266C750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8266C754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8266C758: 386AFFB8  addi r3, r10, -0x48
	ctx.r[3].s64 = ctx.r[10].s64 + -72;
	// 8266C75C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8266C760: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8266C764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8266C768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8266C76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8266C770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8266C774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8266C778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8266C77C: 4BDFA6A5  bl 0x82466e20
	ctx.lr = 0x8266C780;
	sub_82466E20(ctx, base);
	// 8266C780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8266C784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8266C788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8266C78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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


