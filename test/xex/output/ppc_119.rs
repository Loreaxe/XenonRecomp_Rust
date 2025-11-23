pub fn sub_82706D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706D78 size=92
    let mut pc: u32 = 0x82706D78;
    'dispatch: loop {
        match pc {
            0x82706D78 => {
    //   block [0x82706D78..0x82706DD4)
	// 82706D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706D80: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706D84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82706D88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82706D8C: 4BD94B45  bl 0x8249b8d0
	ctx.lr = 0x82706D90;
	sub_8249B8D0(ctx, base);
	// 82706D90: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82706D94: 3D00824A  lis r8, -0x7db6
	ctx.r[8].s64 = -2109079552;
	// 82706D98: 394BC620  addi r10, r11, -0x39e0
	ctx.r[10].s64 = ctx.r[11].s64 + -14816;
	// 82706D9C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82706DA0: 3D20824A  lis r9, -0x7db6
	ctx.r[9].s64 = -2109079552;
	// 82706DA4: 396B0E78  addi r11, r11, 0xe78
	ctx.r[11].s64 = ctx.r[11].s64 + 3704;
	// 82706DA8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82706DAC: 3948B888  addi r10, r8, -0x4778
	ctx.r[10].s64 = ctx.r[8].s64 + -18296;
	// 82706DB0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82706DB4: 3949B870  addi r10, r9, -0x4790
	ctx.r[10].s64 = ctx.r[9].s64 + -18320;
	// 82706DB8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82706DBC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82706DC0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82706DC4: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82706DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706DD8 size=112
    let mut pc: u32 = 0x82706DD8;
    'dispatch: loop {
        match pc {
            0x82706DD8 => {
    //   block [0x82706DD8..0x82706E48)
	// 82706DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706DE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706DE8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706DEC: 38AA0EB8  addi r5, r10, 0xeb8
	ctx.r[5].s64 = ctx.r[10].s64 + 3768;
	// 82706DF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706DF4: 390B0494  addi r8, r11, 0x494
	ctx.r[8].s64 = ctx.r[11].s64 + 1172;
	// 82706DF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82706DFC: 388AC620  addi r4, r10, -0x39e0
	ctx.r[4].s64 = ctx.r[10].s64 + -14816;
	// 82706E00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706E04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706E08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706E10: 386A0E88  addi r3, r10, 0xe88
	ctx.r[3].s64 = ctx.r[10].s64 + 3720;
	// 82706E14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706E18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706E2C: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 82706E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706E34: 4BD5FFED  bl 0x82466e20
	ctx.lr = 0x82706E38;
	sub_82466E20(ctx, base);
	// 82706E38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706E48 size=112
    let mut pc: u32 = 0x82706E48;
    'dispatch: loop {
        match pc {
            0x82706E48 => {
    //   block [0x82706E48..0x82706EB8)
	// 82706E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706E54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706E58: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706E5C: 38AA0DE8  addi r5, r10, 0xde8
	ctx.r[5].s64 = ctx.r[10].s64 + 3560;
	// 82706E60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706E64: 390B04FC  addi r8, r11, 0x4fc
	ctx.r[8].s64 = ctx.r[11].s64 + 1276;
	// 82706E68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82706E6C: 388AC648  addi r4, r10, -0x39b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14776;
	// 82706E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706E74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706E80: 386A0EB8  addi r3, r10, 0xeb8
	ctx.r[3].s64 = ctx.r[10].s64 + 3768;
	// 82706E84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706E9C: 38C00098  li r6, 0x98
	ctx.r[6].s64 = 152;
	// 82706EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706EA4: 4BD5FF7D  bl 0x82466e20
	ctx.lr = 0x82706EA8;
	sub_82466E20(ctx, base);
	// 82706EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82706EB8 size=12
    let mut pc: u32 = 0x82706EB8;
    'dispatch: loop {
        match pc {
            0x82706EB8 => {
    //   block [0x82706EB8..0x82706EC4)
	// 82706EB8: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82706EBC: 386BCF78  addi r3, r11, -0x3088
	ctx.r[3].s64 = ctx.r[11].s64 + -12424;
	// 82706EC0: 4BE2BC78  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706EC8 size=112
    let mut pc: u32 = 0x82706EC8;
    'dispatch: loop {
        match pc {
            0x82706EC8 => {
    //   block [0x82706EC8..0x82706F38)
	// 82706EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706ED4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82706ED8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706EDC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82706EE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706EE4: 390B1020  addi r8, r11, 0x1020
	ctx.r[8].s64 = ctx.r[11].s64 + 4128;
	// 82706EE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82706EEC: 388AC7A8  addi r4, r10, -0x3858
	ctx.r[4].s64 = ctx.r[10].s64 + -14424;
	// 82706EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706EF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706F00: 386A0F34  addi r3, r10, 0xf34
	ctx.r[3].s64 = ctx.r[10].s64 + 3892;
	// 82706F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706F1C: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 82706F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706F24: 4BD5FEFD  bl 0x82466e20
	ctx.lr = 0x82706F28;
	sub_82466E20(ctx, base);
	// 82706F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706F38 size=112
    let mut pc: u32 = 0x82706F38;
    'dispatch: loop {
        match pc {
            0x82706F38 => {
    //   block [0x82706F38..0x82706FA8)
	// 82706F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706F44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706F48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706F4C: 38AA1C30  addi r5, r10, 0x1c30
	ctx.r[5].s64 = ctx.r[10].s64 + 7216;
	// 82706F50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706F54: 390B1078  addi r8, r11, 0x1078
	ctx.r[8].s64 = ctx.r[11].s64 + 4216;
	// 82706F58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82706F5C: 388AC780  addi r4, r10, -0x3880
	ctx.r[4].s64 = ctx.r[10].s64 + -14464;
	// 82706F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706F64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706F68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82706F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706F70: 386A0F64  addi r3, r10, 0xf64
	ctx.r[3].s64 = ctx.r[10].s64 + 3940;
	// 82706F74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82706F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706F8C: 38C00058  li r6, 0x58
	ctx.r[6].s64 = 88;
	// 82706F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706F94: 4BD5FE8D  bl 0x82466e20
	ctx.lr = 0x82706F98;
	sub_82466E20(ctx, base);
	// 82706F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82706F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82706FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82706FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82706FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82706FA8 size=108
    let mut pc: u32 = 0x82706FA8;
    'dispatch: loop {
        match pc {
            0x82706FA8 => {
    //   block [0x82706FA8..0x82707014)
	// 82706FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82706FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82706FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82706FB4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82706FB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82706FBC: 38EB1090  addi r7, r11, 0x1090
	ctx.r[7].s64 = ctx.r[11].s64 + 4240;
	// 82706FC0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82706FC4: 388AC76C  addi r4, r10, -0x3894
	ctx.r[4].s64 = ctx.r[10].s64 + -14484;
	// 82706FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82706FCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82706FD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82706FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82706FD8: 386A0F94  addi r3, r10, 0xf94
	ctx.r[3].s64 = ctx.r[10].s64 + 3988;
	// 82706FDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82706FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82706FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82706FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82706FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82706FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82706FF4: 38C00200  li r6, 0x200
	ctx.r[6].s64 = 512;
	// 82706FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82706FFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707000: 4BD5FE21  bl 0x82466e20
	ctx.lr = 0x82707004;
	sub_82466E20(ctx, base);
	// 82707004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270700C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707018 size=108
    let mut pc: u32 = 0x82707018;
    'dispatch: loop {
        match pc {
            0x82707018 => {
    //   block [0x82707018..0x82707084)
	// 82707018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270701C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707024: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707028: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270702C: 38EB1108  addi r7, r11, 0x1108
	ctx.r[7].s64 = ctx.r[11].s64 + 4360;
	// 82707030: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82707034: 388AC7C4  addi r4, r10, -0x383c
	ctx.r[4].s64 = ctx.r[10].s64 + -14396;
	// 82707038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270703C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707040: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707048: 386A0FC4  addi r3, r10, 0xfc4
	ctx.r[3].s64 = ctx.r[10].s64 + 4036;
	// 8270704C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82707050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270705C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707064: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82707068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270706C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707070: 4BD5FDB1  bl 0x82466e20
	ctx.lr = 0x82707074;
	sub_82466E20(ctx, base);
	// 82707074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270707C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707088 size=108
    let mut pc: u32 = 0x82707088;
    'dispatch: loop {
        match pc {
            0x82707088 => {
    //   block [0x82707088..0x827070F4)
	// 82707088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270708C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707094: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270709C: 38EB115C  addi r7, r11, 0x115c
	ctx.r[7].s64 = ctx.r[11].s64 + 4444;
	// 827070A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827070A4: 388AC7E4  addi r4, r10, -0x381c
	ctx.r[4].s64 = ctx.r[10].s64 + -14364;
	// 827070A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827070AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827070B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827070B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827070B8: 386A0FF4  addi r3, r10, 0xff4
	ctx.r[3].s64 = ctx.r[10].s64 + 4084;
	// 827070BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827070C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827070C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827070C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827070CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827070D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827070D4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 827070D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827070DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827070E0: 4BD5FD41  bl 0x82466e20
	ctx.lr = 0x827070E4;
	sub_82466E20(ctx, base);
	// 827070E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827070E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827070EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827070F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827070F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827070F8 size=24
    let mut pc: u32 = 0x827070F8;
    'dispatch: loop {
        match pc {
            0x827070F8 => {
    //   block [0x827070F8..0x82707110)
	// 827070F8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827070FC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82707100: 394ABB50  addi r10, r10, -0x44b0
	ctx.r[10].s64 = ctx.r[10].s64 + -17584;
	// 82707104: 816BBB18  lwz r11, -0x44e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17640 as u32) ) } as u64;
	// 82707108: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8270710C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707110 size=116
    let mut pc: u32 = 0x82707110;
    'dispatch: loop {
        match pc {
            0x82707110 => {
    //   block [0x82707110..0x82707184)
	// 82707110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270711C: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82707120: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82707124: 390BBB50  addi r8, r11, -0x44b0
	ctx.r[8].s64 = ctx.r[11].s64 + -17584;
	// 82707128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270712C: 392A1148  addi r9, r10, 0x1148
	ctx.r[9].s64 = ctx.r[10].s64 + 4424;
	// 82707130: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707134: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82707138: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270713C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707144: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270714C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707154: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707158: 388AC7F8  addi r4, r10, -0x3808
	ctx.r[4].s64 = ctx.r[10].s64 + -14344;
	// 8270715C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82707160: 386B1024  addi r3, r11, 0x1024
	ctx.r[3].s64 = ctx.r[11].s64 + 4132;
	// 82707164: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82707168: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270716C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82707170: 4BD5FCB1  bl 0x82466e20
	ctx.lr = 0x82707174;
	sub_82466E20(ctx, base);
	// 82707174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270717C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707188 size=108
    let mut pc: u32 = 0x82707188;
    'dispatch: loop {
        match pc {
            0x82707188 => {
    //   block [0x82707188..0x827071F4)
	// 82707188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270718C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707194: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707198: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270719C: 38EB1184  addi r7, r11, 0x1184
	ctx.r[7].s64 = ctx.r[11].s64 + 4484;
	// 827071A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827071A4: 388AC794  addi r4, r10, -0x386c
	ctx.r[4].s64 = ctx.r[10].s64 + -14444;
	// 827071A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827071AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827071B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827071B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827071B8: 386A1054  addi r3, r10, 0x1054
	ctx.r[3].s64 = ctx.r[10].s64 + 4180;
	// 827071BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827071C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827071C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827071C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827071CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827071D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827071D4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 827071D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827071DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827071E0: 4BD5FC41  bl 0x82466e20
	ctx.lr = 0x827071E4;
	sub_82466E20(ctx, base);
	// 827071E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827071E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827071EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827071F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827071F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827071F8 size=112
    let mut pc: u32 = 0x827071F8;
    'dispatch: loop {
        match pc {
            0x827071F8 => {
    //   block [0x827071F8..0x82707268)
	// 827071F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827071FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707204: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707208: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270720C: 38AA07E4  addi r5, r10, 0x7e4
	ctx.r[5].s64 = ctx.r[10].s64 + 2020;
	// 82707210: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707214: 390B1580  addi r8, r11, 0x1580
	ctx.r[8].s64 = ctx.r[11].s64 + 5504;
	// 82707218: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270721C: 388AC804  addi r4, r10, -0x37fc
	ctx.r[4].s64 = ctx.r[10].s64 + -14332;
	// 82707220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707224: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270722C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707230: 386A1088  addi r3, r10, 0x1088
	ctx.r[3].s64 = ctx.r[10].s64 + 4232;
	// 82707234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270723C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270724C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82707250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707254: 4BD5FBCD  bl 0x82466e20
	ctx.lr = 0x82707258;
	sub_82466E20(ctx, base);
	// 82707258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270725C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707268 size=108
    let mut pc: u32 = 0x82707268;
    'dispatch: loop {
        match pc {
            0x82707268 => {
    //   block [0x82707268..0x827072D4)
	// 82707268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270726C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707274: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707278: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270727C: 38EB1610  addi r7, r11, 0x1610
	ctx.r[7].s64 = ctx.r[11].s64 + 5648;
	// 82707280: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82707284: 388AC9A0  addi r4, r10, -0x3660
	ctx.r[4].s64 = ctx.r[10].s64 + -13920;
	// 82707288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270728C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707298: 386A10F8  addi r3, r10, 0x10f8
	ctx.r[3].s64 = ctx.r[10].s64 + 4344;
	// 8270729C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827072A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827072A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827072A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827072AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827072B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827072B4: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 827072B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827072BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827072C0: 4BD5FB61  bl 0x82466e20
	ctx.lr = 0x827072C4;
	sub_82466E20(ctx, base);
	// 827072C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827072C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827072CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827072D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827072D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827072D8 size=112
    let mut pc: u32 = 0x827072D8;
    'dispatch: loop {
        match pc {
            0x827072D8 => {
    //   block [0x827072D8..0x82707348)
	// 827072D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827072DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827072E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827072E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827072E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827072EC: 38AA10F8  addi r5, r10, 0x10f8
	ctx.r[5].s64 = ctx.r[10].s64 + 4344;
	// 827072F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827072F4: 390B1640  addi r8, r11, 0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + 5696;
	// 827072F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827072FC: 388AC9BC  addi r4, r10, -0x3644
	ctx.r[4].s64 = ctx.r[10].s64 + -13892;
	// 82707300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707304: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270730C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707310: 386A10B8  addi r3, r10, 0x10b8
	ctx.r[3].s64 = ctx.r[10].s64 + 4280;
	// 82707314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270731C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270732C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82707330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707334: 4BD5FAED  bl 0x82466e20
	ctx.lr = 0x82707338;
	sub_82466E20(ctx, base);
	// 82707338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270733C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707348 size=112
    let mut pc: u32 = 0x82707348;
    'dispatch: loop {
        match pc {
            0x82707348 => {
    //   block [0x82707348..0x827073B8)
	// 82707348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270734C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707350: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707354: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707358: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270735C: 396B16D0  addi r11, r11, 0x16d0
	ctx.r[11].s64 = ctx.r[11].s64 + 5840;
	// 82707360: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82707364: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82707368: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8270736C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82707370: 4BD9BCE9  bl 0x824a3058
	ctx.lr = 0x82707374;
	sub_824A3058(ctx, base);
	// 82707374: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707378: 3D00824D  lis r8, -0x7db3
	ctx.r[8].s64 = -2108882944;
	// 8270737C: 394BCA40  addi r10, r11, -0x35c0
	ctx.r[10].s64 = ctx.r[11].s64 + -13760;
	// 82707380: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707384: 3D20824D  lis r9, -0x7db3
	ctx.r[9].s64 = -2108882944;
	// 82707388: 396B10E8  addi r11, r11, 0x10e8
	ctx.r[11].s64 = ctx.r[11].s64 + 4328;
	// 8270738C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707390: 39486B18  addi r10, r8, 0x6b18
	ctx.r[10].s64 = ctx.r[8].s64 + 27416;
	// 82707394: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707398: 39496AE0  addi r10, r9, 0x6ae0
	ctx.r[10].s64 = ctx.r[9].s64 + 27360;
	// 8270739C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827073A0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827073A4: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827073A8: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 827073AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827073B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827073B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827073B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827073B8 size=24
    let mut pc: u32 = 0x827073B8;
    'dispatch: loop {
        match pc {
            0x827073B8 => {
    //   block [0x827073B8..0x827073D0)
	// 827073B8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827073BC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827073C0: 394ABD38  addi r10, r10, -0x42c8
	ctx.r[10].s64 = ctx.r[10].s64 + -17096;
	// 827073C4: 816BBD10  lwz r11, -0x42f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17136 as u32) ) } as u64;
	// 827073C8: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 827073CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827073D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827073D0 size=116
    let mut pc: u32 = 0x827073D0;
    'dispatch: loop {
        match pc {
            0x827073D0 => {
    //   block [0x827073D0..0x82707444)
	// 827073D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827073D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827073D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827073DC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827073E0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827073E4: 390BBD38  addi r8, r11, -0x42c8
	ctx.r[8].s64 = ctx.r[11].s64 + -17096;
	// 827073E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827073EC: 392A16B8  addi r9, r10, 0x16b8
	ctx.r[9].s64 = ctx.r[10].s64 + 5816;
	// 827073F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 827073F4: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 827073F8: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 827073FC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707404: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270740C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707414: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707418: 388ACA40  addi r4, r10, -0x35c0
	ctx.r[4].s64 = ctx.r[10].s64 + -13760;
	// 8270741C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82707420: 386B1128  addi r3, r11, 0x1128
	ctx.r[3].s64 = ctx.r[11].s64 + 4392;
	// 82707424: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82707428: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270742C: 38C00120  li r6, 0x120
	ctx.r[6].s64 = 288;
	// 82707430: 4BD5F9F1  bl 0x82466e20
	ctx.lr = 0x82707434;
	sub_82466E20(ctx, base);
	// 82707434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270743C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707448 size=108
    let mut pc: u32 = 0x82707448;
    'dispatch: loop {
        match pc {
            0x82707448 => {
    //   block [0x82707448..0x827074B4)
	// 82707448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270744C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707454: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707458: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270745C: 38EB16E0  addi r7, r11, 0x16e0
	ctx.r[7].s64 = ctx.r[11].s64 + 5856;
	// 82707460: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82707464: 388ACA5C  addi r4, r10, -0x35a4
	ctx.r[4].s64 = ctx.r[10].s64 + -13732;
	// 82707468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270746C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707478: 386A1158  addi r3, r10, 0x1158
	ctx.r[3].s64 = ctx.r[10].s64 + 4440;
	// 8270747C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82707480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270748C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707494: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82707498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270749C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827074A0: 4BD5F981  bl 0x82466e20
	ctx.lr = 0x827074A4;
	sub_82466E20(ctx, base);
	// 827074A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827074A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827074AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827074B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827074B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827074B8 size=108
    let mut pc: u32 = 0x827074B8;
    'dispatch: loop {
        match pc {
            0x827074B8 => {
    //   block [0x827074B8..0x82707524)
	// 827074B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827074BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827074C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827074C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827074C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827074CC: 38EB1728  addi r7, r11, 0x1728
	ctx.r[7].s64 = ctx.r[11].s64 + 5928;
	// 827074D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827074D4: 388ACA78  addi r4, r10, -0x3588
	ctx.r[4].s64 = ctx.r[10].s64 + -13704;
	// 827074D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827074DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827074E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827074E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827074E8: 386A1188  addi r3, r10, 0x1188
	ctx.r[3].s64 = ctx.r[10].s64 + 4488;
	// 827074EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827074F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827074F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827074F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827074FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707504: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82707508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270750C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707510: 4BD5F911  bl 0x82466e20
	ctx.lr = 0x82707514;
	sub_82466E20(ctx, base);
	// 82707514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270751C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707528 size=108
    let mut pc: u32 = 0x82707528;
    'dispatch: loop {
        match pc {
            0x82707528 => {
    //   block [0x82707528..0x82707594)
	// 82707528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270752C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707534: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270753C: 38EB1758  addi r7, r11, 0x1758
	ctx.r[7].s64 = ctx.r[11].s64 + 5976;
	// 82707540: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82707544: 388ACA98  addi r4, r10, -0x3568
	ctx.r[4].s64 = ctx.r[10].s64 + -13672;
	// 82707548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270754C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707558: 386A11B8  addi r3, r10, 0x11b8
	ctx.r[3].s64 = ctx.r[10].s64 + 4536;
	// 8270755C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82707560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270756C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707574: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 82707578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270757C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707580: 4BD5F8A1  bl 0x82466e20
	ctx.lr = 0x82707584;
	sub_82466E20(ctx, base);
	// 82707584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270758C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707598 size=108
    let mut pc: u32 = 0x82707598;
    'dispatch: loop {
        match pc {
            0x82707598 => {
    //   block [0x82707598..0x82707604)
	// 82707598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270759C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827075A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827075A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827075A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827075AC: 38EB1788  addi r7, r11, 0x1788
	ctx.r[7].s64 = ctx.r[11].s64 + 6024;
	// 827075B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827075B4: 388AC8F4  addi r4, r10, -0x370c
	ctx.r[4].s64 = ctx.r[10].s64 + -14092;
	// 827075B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827075BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827075C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827075C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827075C8: 386A1248  addi r3, r10, 0x1248
	ctx.r[3].s64 = ctx.r[10].s64 + 4680;
	// 827075CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827075D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827075D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827075D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827075DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827075E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827075E4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 827075E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827075EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827075F0: 4BD5F831  bl 0x82466e20
	ctx.lr = 0x827075F4;
	sub_82466E20(ctx, base);
	// 827075F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827075F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827075FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707608 size=108
    let mut pc: u32 = 0x82707608;
    'dispatch: loop {
        match pc {
            0x82707608 => {
    //   block [0x82707608..0x82707674)
	// 82707608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270760C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707614: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707618: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270761C: 38EB17B8  addi r7, r11, 0x17b8
	ctx.r[7].s64 = ctx.r[11].s64 + 6072;
	// 82707620: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82707624: 388AC910  addi r4, r10, -0x36f0
	ctx.r[4].s64 = ctx.r[10].s64 + -14064;
	// 82707628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270762C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707638: 386A1218  addi r3, r10, 0x1218
	ctx.r[3].s64 = ctx.r[10].s64 + 4632;
	// 8270763C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82707640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270764C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707654: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82707658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270765C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707660: 4BD5F7C1  bl 0x82466e20
	ctx.lr = 0x82707664;
	sub_82466E20(ctx, base);
	// 82707664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270766C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707678 size=112
    let mut pc: u32 = 0x82707678;
    'dispatch: loop {
        match pc {
            0x82707678 => {
    //   block [0x82707678..0x827076E8)
	// 82707678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270767C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707684: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707688: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270768C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82707690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707694: 390B1800  addi r8, r11, 0x1800
	ctx.r[8].s64 = ctx.r[11].s64 + 6144;
	// 82707698: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270769C: 388AC930  addi r4, r10, -0x36d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14032;
	// 827076A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827076A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827076A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827076AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827076B0: 386A11E8  addi r3, r10, 0x11e8
	ctx.r[3].s64 = ctx.r[10].s64 + 4584;
	// 827076B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827076B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827076BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827076C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827076C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827076C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827076CC: 38C0002C  li r6, 0x2c
	ctx.r[6].s64 = 44;
	// 827076D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827076D4: 4BD5F74D  bl 0x82466e20
	ctx.lr = 0x827076D8;
	sub_82466E20(ctx, base);
	// 827076D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827076DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827076E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827076E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827076E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827076E8 size=92
    let mut pc: u32 = 0x827076E8;
    'dispatch: loop {
        match pc {
            0x827076E8 => {
    //   block [0x827076E8..0x82707744)
	// 827076E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827076EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827076F0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827076F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827076F8: 4BDF70C9  bl 0x824fe7c0
	ctx.lr = 0x827076FC;
	sub_824FE7C0(ctx, base);
	// 827076FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707700: 3CE0824D  lis r7, -0x7db3
	ctx.r[7].s64 = -2108882944;
	// 82707704: 394BC968  addi r10, r11, -0x3698
	ctx.r[10].s64 = ctx.r[11].s64 + -13976;
	// 82707708: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270770C: 3D00824D  lis r8, -0x7db3
	ctx.r[8].s64 = -2108882944;
	// 82707710: 392B18C4  addi r9, r11, 0x18c4
	ctx.r[9].s64 = ctx.r[11].s64 + 6340;
	// 82707714: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707718: 396B1278  addi r11, r11, 0x1278
	ctx.r[11].s64 = ctx.r[11].s64 + 4728;
	// 8270771C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707720: 39476E28  addi r10, r7, 0x6e28
	ctx.r[10].s64 = ctx.r[7].s64 + 28200;
	// 82707724: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707728: 39486E10  addi r10, r8, 0x6e10
	ctx.r[10].s64 = ctx.r[8].s64 + 28176;
	// 8270772C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82707730: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82707734: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82707738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270773C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707748 size=112
    let mut pc: u32 = 0x82707748;
    'dispatch: loop {
        match pc {
            0x82707748 => {
    //   block [0x82707748..0x827077B8)
	// 82707748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270774C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707754: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707758: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270775C: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82707760: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707764: 390B1858  addi r8, r11, 0x1858
	ctx.r[8].s64 = ctx.r[11].s64 + 6232;
	// 82707768: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270776C: 388AC968  addi r4, r10, -0x3698
	ctx.r[4].s64 = ctx.r[10].s64 + -13976;
	// 82707770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707774: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270777C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707780: 386A1288  addi r3, r10, 0x1288
	ctx.r[3].s64 = ctx.r[10].s64 + 4744;
	// 82707784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270778C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270779C: 38C000B4  li r6, 0xb4
	ctx.r[6].s64 = 180;
	// 827077A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827077A4: 4BD5F67D  bl 0x82466e20
	ctx.lr = 0x827077A8;
	sub_82466E20(ctx, base);
	// 827077A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827077AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827077B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827077B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827077B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827077B8 size=108
    let mut pc: u32 = 0x827077B8;
    'dispatch: loop {
        match pc {
            0x827077B8 => {
    //   block [0x827077B8..0x82707824)
	// 827077B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827077BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827077C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827077C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827077C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827077CC: 38EB18D0  addi r7, r11, 0x18d0
	ctx.r[7].s64 = ctx.r[11].s64 + 6352;
	// 827077D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 827077D4: 388AC8B0  addi r4, r10, -0x3750
	ctx.r[4].s64 = ctx.r[10].s64 + -14160;
	// 827077D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827077DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827077E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827077E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827077E8: 386A12C8  addi r3, r10, 0x12c8
	ctx.r[3].s64 = ctx.r[10].s64 + 4808;
	// 827077EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827077F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827077F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827077F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827077FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707804: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82707808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270780C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707810: 4BD5F611  bl 0x82466e20
	ctx.lr = 0x82707814;
	sub_82466E20(ctx, base);
	// 82707814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270781C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707828 size=92
    let mut pc: u32 = 0x82707828;
    'dispatch: loop {
        match pc {
            0x82707828 => {
    //   block [0x82707828..0x82707884)
	// 82707828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270782C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707830: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707834: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82707838: 4BDF6F89  bl 0x824fe7c0
	ctx.lr = 0x8270783C;
	sub_824FE7C0(ctx, base);
	// 8270783C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707840: 3CE0824D  lis r7, -0x7db3
	ctx.r[7].s64 = -2108882944;
	// 82707844: 394BC8D8  addi r10, r11, -0x3728
	ctx.r[10].s64 = ctx.r[11].s64 + -14120;
	// 82707848: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270784C: 3D00824D  lis r8, -0x7db3
	ctx.r[8].s64 = -2108882944;
	// 82707850: 392B1984  addi r9, r11, 0x1984
	ctx.r[9].s64 = ctx.r[11].s64 + 6532;
	// 82707854: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707858: 396B12B8  addi r11, r11, 0x12b8
	ctx.r[11].s64 = ctx.r[11].s64 + 4792;
	// 8270785C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707860: 39476FC0  addi r10, r7, 0x6fc0
	ctx.r[10].s64 = ctx.r[7].s64 + 28608;
	// 82707864: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707868: 39486FA8  addi r10, r8, 0x6fa8
	ctx.r[10].s64 = ctx.r[8].s64 + 28584;
	// 8270786C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82707870: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82707874: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82707878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270787C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707888 size=112
    let mut pc: u32 = 0x82707888;
    'dispatch: loop {
        match pc {
            0x82707888 => {
    //   block [0x82707888..0x827078F8)
	// 82707888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270788C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707894: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707898: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270789C: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 827078A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827078A4: 390B1900  addi r8, r11, 0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + 6400;
	// 827078A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827078AC: 388AC8D8  addi r4, r10, -0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + -14120;
	// 827078B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827078B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827078B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827078BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827078C0: 386A12F8  addi r3, r10, 0x12f8
	ctx.r[3].s64 = ctx.r[10].s64 + 4856;
	// 827078C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827078C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827078CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827078D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827078D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827078D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827078DC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827078E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827078E4: 4BD5F53D  bl 0x82466e20
	ctx.lr = 0x827078E8;
	sub_82466E20(ctx, base);
	// 827078E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827078EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827078F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827078F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827078F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827078F8 size=92
    let mut pc: u32 = 0x827078F8;
    'dispatch: loop {
        match pc {
            0x827078F8 => {
    //   block [0x827078F8..0x82707954)
	// 827078F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827078FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707900: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707904: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82707908: 4BDF6EB9  bl 0x824fe7c0
	ctx.lr = 0x8270790C;
	sub_824FE7C0(ctx, base);
	// 8270790C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707910: 3CE0824D  lis r7, -0x7db3
	ctx.r[7].s64 = -2108882944;
	// 82707914: 394BC948  addi r10, r11, -0x36b8
	ctx.r[10].s64 = ctx.r[11].s64 + -14008;
	// 82707918: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270791C: 3D00824D  lis r8, -0x7db3
	ctx.r[8].s64 = -2108882944;
	// 82707920: 392B1A00  addi r9, r11, 0x1a00
	ctx.r[9].s64 = ctx.r[11].s64 + 6656;
	// 82707924: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707928: 396B1328  addi r11, r11, 0x1328
	ctx.r[11].s64 = ctx.r[11].s64 + 4904;
	// 8270792C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707930: 39477148  addi r10, r7, 0x7148
	ctx.r[10].s64 = ctx.r[7].s64 + 29000;
	// 82707934: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707938: 39487130  addi r10, r8, 0x7130
	ctx.r[10].s64 = ctx.r[8].s64 + 28976;
	// 8270793C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82707940: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82707944: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82707948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270794C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707958 size=112
    let mut pc: u32 = 0x82707958;
    'dispatch: loop {
        match pc {
            0x82707958 => {
    //   block [0x82707958..0x827079C8)
	// 82707958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270795C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707964: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707968: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270796C: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82707970: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707974: 390B1994  addi r8, r11, 0x1994
	ctx.r[8].s64 = ctx.r[11].s64 + 6548;
	// 82707978: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270797C: 388AC948  addi r4, r10, -0x36b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14008;
	// 82707980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707984: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270798C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707990: 386A1338  addi r3, r10, 0x1338
	ctx.r[3].s64 = ctx.r[10].s64 + 4920;
	// 82707994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270799C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827079A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827079A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 827079A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827079AC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827079B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827079B4: 4BD5F46D  bl 0x82466e20
	ctx.lr = 0x827079B8;
	sub_82466E20(ctx, base);
	// 827079B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827079BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827079C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827079C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827079C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827079C8 size=112
    let mut pc: u32 = 0x827079C8;
    'dispatch: loop {
        match pc {
            0x827079C8 => {
    //   block [0x827079C8..0x82707A38)
	// 827079C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827079CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827079D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827079D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827079D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827079DC: 38AA06B4  addi r5, r10, 0x6b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1716;
	// 827079E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827079E4: 390B1A10  addi r8, r11, 0x1a10
	ctx.r[8].s64 = ctx.r[11].s64 + 6672;
	// 827079E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 827079EC: 388AC830  addi r4, r10, -0x37d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14288;
	// 827079F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827079F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827079F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827079FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707A00: 386A1368  addi r3, r10, 0x1368
	ctx.r[3].s64 = ctx.r[10].s64 + 4968;
	// 82707A04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707A08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707A1C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82707A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707A24: 4BD5F3FD  bl 0x82466e20
	ctx.lr = 0x82707A28;
	sub_82466E20(ctx, base);
	// 82707A28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707A2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707A30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707A38 size=112
    let mut pc: u32 = 0x82707A38;
    'dispatch: loop {
        match pc {
            0x82707A38 => {
    //   block [0x82707A38..0x82707AA8)
	// 82707A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707A44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707A48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707A4C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82707A50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707A54: 390B1AB8  addi r8, r11, 0x1ab8
	ctx.r[8].s64 = ctx.r[11].s64 + 6840;
	// 82707A58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82707A5C: 388ACADC  addi r4, r10, -0x3524
	ctx.r[4].s64 = ctx.r[10].s64 + -13604;
	// 82707A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707A64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707A70: 386A1398  addi r3, r10, 0x1398
	ctx.r[3].s64 = ctx.r[10].s64 + 5016;
	// 82707A74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707A8C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82707A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707A94: 4BD5F38D  bl 0x82466e20
	ctx.lr = 0x82707A98;
	sub_82466E20(ctx, base);
	// 82707A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707AA8 size=108
    let mut pc: u32 = 0x82707AA8;
    'dispatch: loop {
        match pc {
            0x82707AA8 => {
    //   block [0x82707AA8..0x82707B14)
	// 82707AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707AB4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707AB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707ABC: 38EB1AE0  addi r7, r11, 0x1ae0
	ctx.r[7].s64 = ctx.r[11].s64 + 6880;
	// 82707AC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82707AC4: 388ACAFC  addi r4, r10, -0x3504
	ctx.r[4].s64 = ctx.r[10].s64 + -13572;
	// 82707AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707ACC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707AD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82707AD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707AD8: 386A13F8  addi r3, r10, 0x13f8
	ctx.r[3].s64 = ctx.r[10].s64 + 5112;
	// 82707ADC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82707AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707AF4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82707AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707AFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707B00: 4BD5F321  bl 0x82466e20
	ctx.lr = 0x82707B04;
	sub_82466E20(ctx, base);
	// 82707B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707B10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707B18 size=112
    let mut pc: u32 = 0x82707B18;
    'dispatch: loop {
        match pc {
            0x82707B18 => {
    //   block [0x82707B18..0x82707B88)
	// 82707B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707B24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707B28: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707B2C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82707B30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707B34: 390B1B10  addi r8, r11, 0x1b10
	ctx.r[8].s64 = ctx.r[11].s64 + 6928;
	// 82707B38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82707B3C: 388ACB34  addi r4, r10, -0x34cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13516;
	// 82707B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707B44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707B50: 386A13C8  addi r3, r10, 0x13c8
	ctx.r[3].s64 = ctx.r[10].s64 + 5064;
	// 82707B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707B6C: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82707B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707B74: 4BD5F2AD  bl 0x82466e20
	ctx.lr = 0x82707B78;
	sub_82466E20(ctx, base);
	// 82707B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707B88 size=112
    let mut pc: u32 = 0x82707B88;
    'dispatch: loop {
        match pc {
            0x82707B88 => {
    //   block [0x82707B88..0x82707BF8)
	// 82707B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707B94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707B98: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707B9C: 38AA07E4  addi r5, r10, 0x7e4
	ctx.r[5].s64 = ctx.r[10].s64 + 2020;
	// 82707BA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707BA4: 390B1B38  addi r8, r11, 0x1b38
	ctx.r[8].s64 = ctx.r[11].s64 + 6968;
	// 82707BA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82707BAC: 388AC81C  addi r4, r10, -0x37e4
	ctx.r[4].s64 = ctx.r[10].s64 + -14308;
	// 82707BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707BB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707BC0: 386A1428  addi r3, r10, 0x1428
	ctx.r[3].s64 = ctx.r[10].s64 + 5160;
	// 82707BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707BDC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82707BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707BE4: 4BD5F23D  bl 0x82466e20
	ctx.lr = 0x82707BE8;
	sub_82466E20(ctx, base);
	// 82707BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707BF8 size=112
    let mut pc: u32 = 0x82707BF8;
    'dispatch: loop {
        match pc {
            0x82707BF8 => {
    //   block [0x82707BF8..0x82707C68)
	// 82707BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707C04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707C08: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707C0C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82707C10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707C14: 390B1BBC  addi r8, r11, 0x1bbc
	ctx.r[8].s64 = ctx.r[11].s64 + 7100;
	// 82707C18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82707C1C: 388ACAC0  addi r4, r10, -0x3540
	ctx.r[4].s64 = ctx.r[10].s64 + -13632;
	// 82707C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707C24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707C30: 386A1458  addi r3, r10, 0x1458
	ctx.r[3].s64 = ctx.r[10].s64 + 5208;
	// 82707C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707C4C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82707C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707C54: 4BD5F1CD  bl 0x82466e20
	ctx.lr = 0x82707C58;
	sub_82466E20(ctx, base);
	// 82707C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707C68 size=92
    let mut pc: u32 = 0x82707C68;
    'dispatch: loop {
        match pc {
            0x82707C68 => {
    //   block [0x82707C68..0x82707CC4)
	// 82707C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707C70: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707C74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82707C78: 4BDF6B49  bl 0x824fe7c0
	ctx.lr = 0x82707C7C;
	sub_824FE7C0(ctx, base);
	// 82707C7C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707C80: 3CE0824E  lis r7, -0x7db2
	ctx.r[7].s64 = -2108817408;
	// 82707C84: 394BC894  addi r10, r11, -0x376c
	ctx.r[10].s64 = ctx.r[11].s64 + -14188;
	// 82707C88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707C8C: 3D00824E  lis r8, -0x7db2
	ctx.r[8].s64 = -2108817408;
	// 82707C90: 392B1C64  addi r9, r11, 0x1c64
	ctx.r[9].s64 = ctx.r[11].s64 + 7268;
	// 82707C94: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707C98: 396B1488  addi r11, r11, 0x1488
	ctx.r[11].s64 = ctx.r[11].s64 + 5256;
	// 82707C9C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707CA0: 3947C6E8  addi r10, r7, -0x3918
	ctx.r[10].s64 = ctx.r[7].s64 + -14616;
	// 82707CA4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707CA8: 3948C6D0  addi r10, r8, -0x3930
	ctx.r[10].s64 = ctx.r[8].s64 + -14640;
	// 82707CAC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82707CB0: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82707CB4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82707CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707CC8 size=112
    let mut pc: u32 = 0x82707CC8;
    'dispatch: loop {
        match pc {
            0x82707CC8 => {
    //   block [0x82707CC8..0x82707D38)
	// 82707CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707CD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707CD8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707CDC: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82707CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707CE4: 390B1BF0  addi r8, r11, 0x1bf0
	ctx.r[8].s64 = ctx.r[11].s64 + 7152;
	// 82707CE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82707CEC: 388AC894  addi r4, r10, -0x376c
	ctx.r[4].s64 = ctx.r[10].s64 + -14188;
	// 82707CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707CF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707D00: 386A1498  addi r3, r10, 0x1498
	ctx.r[3].s64 = ctx.r[10].s64 + 5272;
	// 82707D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707D14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82707D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707D1C: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82707D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707D24: 4BD5F0FD  bl 0x82466e20
	ctx.lr = 0x82707D28;
	sub_82466E20(ctx, base);
	// 82707D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707D38 size=112
    let mut pc: u32 = 0x82707D38;
    'dispatch: loop {
        match pc {
            0x82707D38 => {
    //   block [0x82707D38..0x82707DA8)
	// 82707D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707D44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707D48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707D4C: 38AA06B4  addi r5, r10, 0x6b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1716;
	// 82707D50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707D54: 390B1C70  addi r8, r11, 0x1c70
	ctx.r[8].s64 = ctx.r[11].s64 + 7280;
	// 82707D58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82707D5C: 388AC858  addi r4, r10, -0x37a8
	ctx.r[4].s64 = ctx.r[10].s64 + -14248;
	// 82707D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707D64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707D70: 386A14C8  addi r3, r10, 0x14c8
	ctx.r[3].s64 = ctx.r[10].s64 + 5320;
	// 82707D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707D8C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82707D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707D94: 4BD5F08D  bl 0x82466e20
	ctx.lr = 0x82707D98;
	sub_82466E20(ctx, base);
	// 82707D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707DA8 size=112
    let mut pc: u32 = 0x82707DA8;
    'dispatch: loop {
        match pc {
            0x82707DA8 => {
    //   block [0x82707DA8..0x82707E18)
	// 82707DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707DB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82707DB8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707DBC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82707DC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707DC4: 390B1CF4  addi r8, r11, 0x1cf4
	ctx.r[8].s64 = ctx.r[11].s64 + 7412;
	// 82707DC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82707DCC: 388ACAB0  addi r4, r10, -0x3550
	ctx.r[4].s64 = ctx.r[10].s64 + -13648;
	// 82707DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707DD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707DE0: 386A14F8  addi r3, r10, 0x14f8
	ctx.r[3].s64 = ctx.r[10].s64 + 5368;
	// 82707DE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707DFC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 82707E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707E04: 4BD5F01D  bl 0x82466e20
	ctx.lr = 0x82707E08;
	sub_82466E20(ctx, base);
	// 82707E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707E18 size=112
    let mut pc: u32 = 0x82707E18;
    'dispatch: loop {
        match pc {
            0x82707E18 => {
    //   block [0x82707E18..0x82707E88)
	// 82707E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707E24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707E28: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707E2C: 38AA07E4  addi r5, r10, 0x7e4
	ctx.r[5].s64 = ctx.r[10].s64 + 2020;
	// 82707E30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707E34: 390B1D38  addi r8, r11, 0x1d38
	ctx.r[8].s64 = ctx.r[11].s64 + 7480;
	// 82707E38: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82707E3C: 388AC86C  addi r4, r10, -0x3794
	ctx.r[4].s64 = ctx.r[10].s64 + -14228;
	// 82707E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707E44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707E50: 386A1528  addi r3, r10, 0x1528
	ctx.r[3].s64 = ctx.r[10].s64 + 5416;
	// 82707E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707E6C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82707E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707E74: 4BD5EFAD  bl 0x82466e20
	ctx.lr = 0x82707E78;
	sub_82466E20(ctx, base);
	// 82707E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707E88 size=100
    let mut pc: u32 = 0x82707E88;
    'dispatch: loop {
        match pc {
            0x82707E88 => {
    //   block [0x82707E88..0x82707EEC)
	// 82707E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707E90: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707E94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82707E98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82707E9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82707EA0: 4BEA6B61  bl 0x825aea00
	ctx.lr = 0x82707EA4;
	sub_825AEA00(ctx, base);
	// 82707EA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82707EA8: 3CE0824E  lis r7, -0x7db2
	ctx.r[7].s64 = -2108817408;
	// 82707EAC: 394BC840  addi r10, r11, -0x37c0
	ctx.r[10].s64 = ctx.r[11].s64 + -14272;
	// 82707EB0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707EB4: 3D00824E  lis r8, -0x7db2
	ctx.r[8].s64 = -2108817408;
	// 82707EB8: 392B1EE4  addi r9, r11, 0x1ee4
	ctx.r[9].s64 = ctx.r[11].s64 + 7908;
	// 82707EBC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82707EC0: 396B1588  addi r11, r11, 0x1588
	ctx.r[11].s64 = ctx.r[11].s64 + 5512;
	// 82707EC4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82707EC8: 3947CA08  addi r10, r7, -0x35f8
	ctx.r[10].s64 = ctx.r[7].s64 + -13816;
	// 82707ECC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82707ED0: 3948C9F0  addi r10, r8, -0x3610
	ctx.r[10].s64 = ctx.r[8].s64 + -13840;
	// 82707ED4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82707ED8: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82707EDC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82707EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707EF0 size=112
    let mut pc: u32 = 0x82707EF0;
    'dispatch: loop {
        match pc {
            0x82707EF0 => {
    //   block [0x82707EF0..0x82707F60)
	// 82707EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707EFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707F00: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707F04: 38AA06B4  addi r5, r10, 0x6b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1716;
	// 82707F08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707F0C: 390B1E20  addi r8, r11, 0x1e20
	ctx.r[8].s64 = ctx.r[11].s64 + 7712;
	// 82707F10: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82707F14: 388AC840  addi r4, r10, -0x37c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14272;
	// 82707F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707F1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707F20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707F28: 386A1558  addi r3, r10, 0x1558
	ctx.r[3].s64 = ctx.r[10].s64 + 5464;
	// 82707F2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707F44: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82707F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707F4C: 4BD5EED5  bl 0x82466e20
	ctx.lr = 0x82707F50;
	sub_82466E20(ctx, base);
	// 82707F50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707F60 size=112
    let mut pc: u32 = 0x82707F60;
    'dispatch: loop {
        match pc {
            0x82707F60 => {
    //   block [0x82707F60..0x82707FD0)
	// 82707F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707F6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707F70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707F74: 38AA07B4  addi r5, r10, 0x7b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1972;
	// 82707F78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707F7C: 390B1F04  addi r8, r11, 0x1f04
	ctx.r[8].s64 = ctx.r[11].s64 + 7940;
	// 82707F80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82707F84: 388AC980  addi r4, r10, -0x3680
	ctx.r[4].s64 = ctx.r[10].s64 + -13952;
	// 82707F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82707F8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707F90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82707F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82707F98: 386A1598  addi r3, r10, 0x1598
	ctx.r[3].s64 = ctx.r[10].s64 + 5528;
	// 82707F9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82707FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82707FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82707FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82707FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82707FB4: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82707FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82707FBC: 4BD5EE65  bl 0x82466e20
	ctx.lr = 0x82707FC0;
	sub_82466E20(ctx, base);
	// 82707FC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82707FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82707FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82707FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82707FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82707FD0 size=108
    let mut pc: u32 = 0x82707FD0;
    'dispatch: loop {
        match pc {
            0x82707FD0 => {
    //   block [0x82707FD0..0x8270803C)
	// 82707FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82707FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82707FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82707FDC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82707FE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82707FE4: 396B1F38  addi r11, r11, 0x1f38
	ctx.r[11].s64 = ctx.r[11].s64 + 7992;
	// 82707FE8: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 82707FEC: 390B01C8  addi r8, r11, 0x1c8
	ctx.r[8].s64 = ctx.r[11].s64 + 456;
	// 82707FF0: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 82707FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82707FF8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82707FFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82708000: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82708004: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 82708008: 386A15C8  addi r3, r10, 0x15c8
	ctx.r[3].s64 = ctx.r[10].s64 + 5576;
	// 8270800C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82708010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708018: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8270801C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708020: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82708024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82708028: 4BD5EDF9  bl 0x82466e20
	ctx.lr = 0x8270802C;
	sub_82466E20(ctx, base);
	// 8270802C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708040 size=116
    let mut pc: u32 = 0x82708040;
    'dispatch: loop {
        match pc {
            0x82708040 => {
    //   block [0x82708040..0x827080B4)
	// 82708040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270804C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708050: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82708054: 390B28E4  addi r8, r11, 0x28e4
	ctx.r[8].s64 = ctx.r[11].s64 + 10468;
	// 82708058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270805C: 392A28D0  addi r9, r10, 0x28d0
	ctx.r[9].s64 = ctx.r[10].s64 + 10448;
	// 82708060: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708064: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82708068: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 8270806C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708074: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270807C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708084: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708088: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 8270808C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82708090: 386B1600  addi r3, r11, 0x1600
	ctx.r[3].s64 = ctx.r[11].s64 + 5632;
	// 82708094: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82708098: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270809C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827080A0: 4BD5ED81  bl 0x82466e20
	ctx.lr = 0x827080A4;
	sub_82466E20(ctx, base);
	// 827080A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827080A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827080AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827080B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827080B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827080B8 size=92
    let mut pc: u32 = 0x827080B8;
    'dispatch: loop {
        match pc {
            0x827080B8 => {
    //   block [0x827080B8..0x82708114)
	// 827080B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827080BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827080C0: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827080C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827080C8: 4BDF66F9  bl 0x824fe7c0
	ctx.lr = 0x827080CC;
	sub_824FE7C0(ctx, base);
	// 827080CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827080D0: 3CE08250  lis r7, -0x7db0
	ctx.r[7].s64 = -2108686336;
	// 827080D4: 394BB4A4  addi r10, r11, -0x4b5c
	ctx.r[10].s64 = ctx.r[11].s64 + -19292;
	// 827080D8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827080DC: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 827080E0: 392B29E4  addi r9, r11, 0x29e4
	ctx.r[9].s64 = ctx.r[11].s64 + 10724;
	// 827080E4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827080E8: 396B1630  addi r11, r11, 0x1630
	ctx.r[11].s64 = ctx.r[11].s64 + 5680;
	// 827080EC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827080F0: 39479560  addi r10, r7, -0x6aa0
	ctx.r[10].s64 = ctx.r[7].s64 + -27296;
	// 827080F4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827080F8: 39489548  addi r10, r8, -0x6ab8
	ctx.r[10].s64 = ctx.r[8].s64 + -27320;
	// 827080FC: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82708100: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82708104: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82708108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270810C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708118 size=112
    let mut pc: u32 = 0x82708118;
    'dispatch: loop {
        match pc {
            0x82708118 => {
    //   block [0x82708118..0x82708188)
	// 82708118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270811C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708124: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708128: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270812C: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82708130: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708134: 390B2960  addi r8, r11, 0x2960
	ctx.r[8].s64 = ctx.r[11].s64 + 10592;
	// 82708138: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270813C: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 82708140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708144: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270814C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708150: 386A1640  addi r3, r10, 0x1640
	ctx.r[3].s64 = ctx.r[10].s64 + 5696;
	// 82708154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270815C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270816C: 38C00100  li r6, 0x100
	ctx.r[6].s64 = 256;
	// 82708170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708174: 4BD5ECAD  bl 0x82466e20
	ctx.lr = 0x82708178;
	sub_82466E20(ctx, base);
	// 82708178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270817C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708188 size=112
    let mut pc: u32 = 0x82708188;
    'dispatch: loop {
        match pc {
            0x82708188 => {
    //   block [0x82708188..0x827081F8)
	// 82708188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270818C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708194: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708198: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270819C: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 827081A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827081A4: 390B29F8  addi r8, r11, 0x29f8
	ctx.r[8].s64 = ctx.r[11].s64 + 10744;
	// 827081A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827081AC: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 827081B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827081B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827081B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827081BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827081C0: 386A1670  addi r3, r10, 0x1670
	ctx.r[3].s64 = ctx.r[10].s64 + 5744;
	// 827081C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827081C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827081CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827081D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827081D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827081D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827081DC: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 827081E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827081E4: 4BD5EC3D  bl 0x82466e20
	ctx.lr = 0x827081E8;
	sub_82466E20(ctx, base);
	// 827081E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827081EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827081F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827081F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827081F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827081F8 size=92
    let mut pc: u32 = 0x827081F8;
    'dispatch: loop {
        match pc {
            0x827081F8 => {
    //   block [0x827081F8..0x82708254)
	// 827081F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827081FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708200: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708204: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708208: 4BDF65B9  bl 0x824fe7c0
	ctx.lr = 0x8270820C;
	sub_824FE7C0(ctx, base);
	// 8270820C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82708210: 3CE08250  lis r7, -0x7db0
	ctx.r[7].s64 = -2108686336;
	// 82708214: 394BB4B4  addi r10, r11, -0x4b4c
	ctx.r[10].s64 = ctx.r[11].s64 + -19276;
	// 82708218: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270821C: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708220: 392B2B04  addi r9, r11, 0x2b04
	ctx.r[9].s64 = ctx.r[11].s64 + 11012;
	// 82708224: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708228: 396B16A0  addi r11, r11, 0x16a0
	ctx.r[11].s64 = ctx.r[11].s64 + 5792;
	// 8270822C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708230: 39479818  addi r10, r7, -0x67e8
	ctx.r[10].s64 = ctx.r[7].s64 + -26600;
	// 82708234: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82708238: 39489800  addi r10, r8, -0x6800
	ctx.r[10].s64 = ctx.r[8].s64 + -26624;
	// 8270823C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82708240: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82708244: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82708248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270824C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708258 size=112
    let mut pc: u32 = 0x82708258;
    'dispatch: loop {
        match pc {
            0x82708258 => {
    //   block [0x82708258..0x827082C8)
	// 82708258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270825C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708264: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708268: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270826C: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82708270: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708274: 390B2AB0  addi r8, r11, 0x2ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 10928;
	// 82708278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270827C: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 82708280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270828C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708290: 386A16B0  addi r3, r10, 0x16b0
	ctx.r[3].s64 = ctx.r[10].s64 + 5808;
	// 82708294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270829C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827082A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827082A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827082A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827082AC: 38C0003C  li r6, 0x3c
	ctx.r[6].s64 = 60;
	// 827082B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827082B4: 4BD5EB6D  bl 0x82466e20
	ctx.lr = 0x827082B8;
	sub_82466E20(ctx, base);
	// 827082B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827082BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827082C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827082C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827082C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827082C8 size=36
    let mut pc: u32 = 0x827082C8;
    'dispatch: loop {
        match pc {
            0x827082C8 => {
    //   block [0x827082C8..0x827082EC)
	// 827082C8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827082CC: 814BC540  lwz r10, -0x3ac0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15040 as u32) ) } as u64;
	// 827082D0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827082D4: 396BC578  addi r11, r11, -0x3a88
	ctx.r[11].s64 = ctx.r[11].s64 + -14984;
	// 827082D8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827082DC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827082E0: 814AC538  lwz r10, -0x3ac8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-15048 as u32) ) } as u64;
	// 827082E4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 827082E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827082F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827082F0 size=108
    let mut pc: u32 = 0x827082F0;
    'dispatch: loop {
        match pc {
            0x827082F0 => {
    //   block [0x827082F0..0x8270835C)
	// 827082F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827082F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827082F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827082FC: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708304: 38EBC578  addi r7, r11, -0x3a88
	ctx.r[7].s64 = ctx.r[11].s64 + -14984;
	// 82708308: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8270830C: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 82708310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708314: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708318: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270831C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708320: 386A16E0  addi r3, r10, 0x16e0
	ctx.r[3].s64 = ctx.r[10].s64 + 5856;
	// 82708324: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82708328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270832C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270833C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82708340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708344: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82708348: 4BD5EAD9  bl 0x82466e20
	ctx.lr = 0x8270834C;
	sub_82466E20(ctx, base);
	// 8270834C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82708360 size=24
    let mut pc: u32 = 0x82708360;
    'dispatch: loop {
        match pc {
            0x82708360 => {
    //   block [0x82708360..0x82708378)
	// 82708360: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708364: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82708368: 394AC620  addi r10, r10, -0x39e0
	ctx.r[10].s64 = ctx.r[10].s64 + -14816;
	// 8270836C: 816BC538  lwz r11, -0x3ac8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15048 as u32) ) } as u64;
	// 82708370: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 82708374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708378 size=116
    let mut pc: u32 = 0x82708378;
    'dispatch: loop {
        match pc {
            0x82708378 => {
    //   block [0x82708378..0x827083EC)
	// 82708378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270837C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708384: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82708388: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8270838C: 390AC620  addi r8, r10, -0x39e0
	ctx.r[8].s64 = ctx.r[10].s64 + -14816;
	// 82708390: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708394: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708398: 38AA16E0  addi r5, r10, 0x16e0
	ctx.r[5].s64 = ctx.r[10].s64 + 5856;
	// 8270839C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827083A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827083A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827083A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827083AC: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 827083B0: 396B2B9C  addi r11, r11, 0x2b9c
	ctx.r[11].s64 = ctx.r[11].s64 + 11164;
	// 827083B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827083B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827083BC: 386A1710  addi r3, r10, 0x1710
	ctx.r[3].s64 = ctx.r[10].s64 + 5904;
	// 827083C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 827083C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827083C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 827083CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827083D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827083D4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827083D8: 4BD5EA49  bl 0x82466e20
	ctx.lr = 0x827083DC;
	sub_82466E20(ctx, base);
	// 827083DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827083E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827083E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827083E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827083F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827083F0 size=112
    let mut pc: u32 = 0x827083F0;
    'dispatch: loop {
        match pc {
            0x827083F0 => {
    //   block [0x827083F0..0x82708460)
	// 827083F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827083F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827083F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827083FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708400: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708404: 38AA16E0  addi r5, r10, 0x16e0
	ctx.r[5].s64 = ctx.r[10].s64 + 5856;
	// 82708408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270840C: 390B2BC8  addi r8, r11, 0x2bc8
	ctx.r[8].s64 = ctx.r[11].s64 + 11208;
	// 82708410: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82708414: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 82708418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270841C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708428: 386A1740  addi r3, r10, 0x1740
	ctx.r[3].s64 = ctx.r[10].s64 + 5952;
	// 8270842C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270843C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708444: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82708448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270844C: 4BD5E9D5  bl 0x82466e20
	ctx.lr = 0x82708450;
	sub_82466E20(ctx, base);
	// 82708450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270845C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708460 size=92
    let mut pc: u32 = 0x82708460;
    'dispatch: loop {
        match pc {
            0x82708460 => {
    //   block [0x82708460..0x827084BC)
	// 82708460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708468: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270846C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82708470: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708474: 4BDFCB6D  bl 0x82504fe0
	ctx.lr = 0x82708478;
	sub_82504FE0(ctx, base);
	// 82708478: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270847C: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708480: 394BB5E4  addi r10, r11, -0x4a1c
	ctx.r[10].s64 = ctx.r[11].s64 + -18972;
	// 82708484: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708488: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 8270848C: 396B1770  addi r11, r11, 0x1770
	ctx.r[11].s64 = ctx.r[11].s64 + 6000;
	// 82708490: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708494: 39489970  addi r10, r8, -0x6690
	ctx.r[10].s64 = ctx.r[8].s64 + -26256;
	// 82708498: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270849C: 39499988  addi r10, r9, -0x6678
	ctx.r[10].s64 = ctx.r[9].s64 + -26232;
	// 827084A0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827084A4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827084A8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827084AC: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 827084B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827084B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827084B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827084C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827084C0 size=24
    let mut pc: u32 = 0x827084C0;
    'dispatch: loop {
        match pc {
            0x827084C0 => {
    //   block [0x827084C0..0x827084D8)
	// 827084C0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827084C4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 827084C8: 394AC710  addi r10, r10, -0x38f0
	ctx.r[10].s64 = ctx.r[10].s64 + -14576;
	// 827084CC: 816BCE44  lwz r11, -0x31bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12732 as u32) ) } as u64;
	// 827084D0: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 827084D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827084D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827084D8 size=116
    let mut pc: u32 = 0x827084D8;
    'dispatch: loop {
        match pc {
            0x827084D8 => {
    //   block [0x827084D8..0x8270854C)
	// 827084D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827084DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827084E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827084E4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827084E8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827084EC: 392B2B60  addi r9, r11, 0x2b60
	ctx.r[9].s64 = ctx.r[11].s64 + 11104;
	// 827084F0: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 827084F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827084F8: 38E900E0  addi r7, r9, 0xe0
	ctx.r[7].s64 = ctx.r[9].s64 + 224;
	// 827084FC: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82708500: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708504: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 82708508: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270850C: 396BC710  addi r11, r11, -0x38f0
	ctx.r[11].s64 = ctx.r[11].s64 + -14576;
	// 82708510: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82708514: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708518: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8270851C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708520: 386A1780  addi r3, r10, 0x1780
	ctx.r[3].s64 = ctx.r[10].s64 + 6016;
	// 82708524: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82708528: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8270852C: 38C000C0  li r6, 0xc0
	ctx.r[6].s64 = 192;
	// 82708530: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82708534: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82708538: 4BD5E8E9  bl 0x82466e20
	ctx.lr = 0x8270853C;
	sub_82466E20(ctx, base);
	// 8270853C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708550 size=96
    let mut pc: u32 = 0x82708550;
    'dispatch: loop {
        match pc {
            0x82708550 => {
    //   block [0x82708550..0x827085B0)
	// 82708550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270855C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708564: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 82708568: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270856C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708570: 386A17B0  addi r3, r10, 0x17b0
	ctx.r[3].s64 = ctx.r[10].s64 + 6064;
	// 82708574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270857C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82708580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270858C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82708590: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82708594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82708598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270859C: 4BD5E885  bl 0x82466e20
	ctx.lr = 0x827085A0;
	sub_82466E20(ctx, base);
	// 827085A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827085A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827085A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827085AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827085B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827085B0 size=112
    let mut pc: u32 = 0x827085B0;
    'dispatch: loop {
        match pc {
            0x827085B0 => {
    //   block [0x827085B0..0x82708620)
	// 827085B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827085B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827085B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827085BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827085C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827085C4: 38AA17B0  addi r5, r10, 0x17b0
	ctx.r[5].s64 = ctx.r[10].s64 + 6064;
	// 827085C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827085CC: 390B2C70  addi r8, r11, 0x2c70
	ctx.r[8].s64 = ctx.r[11].s64 + 11376;
	// 827085D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827085D4: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 827085D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827085DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827085E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827085E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827085E8: 386A17E0  addi r3, r10, 0x17e0
	ctx.r[3].s64 = ctx.r[10].s64 + 6112;
	// 827085EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827085F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827085F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827085F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827085FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708604: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82708608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270860C: 4BD5E815  bl 0x82466e20
	ctx.lr = 0x82708610;
	sub_82466E20(ctx, base);
	// 82708610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270861C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708620 size=112
    let mut pc: u32 = 0x82708620;
    'dispatch: loop {
        match pc {
            0x82708620 => {
    //   block [0x82708620..0x82708690)
	// 82708620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270862C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708630: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708634: 38AA2210  addi r5, r10, 0x2210
	ctx.r[5].s64 = ctx.r[10].s64 + 8720;
	// 82708638: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270863C: 390B2C88  addi r8, r11, 0x2c88
	ctx.r[8].s64 = ctx.r[11].s64 + 11400;
	// 82708640: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82708644: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 82708648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270864C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708658: 386A1810  addi r3, r10, 0x1810
	ctx.r[3].s64 = ctx.r[10].s64 + 6160;
	// 8270865C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270866C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708674: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82708678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270867C: 4BD5E7A5  bl 0x82466e20
	ctx.lr = 0x82708680;
	sub_82466E20(ctx, base);
	// 82708680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270868C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708690 size=112
    let mut pc: u32 = 0x82708690;
    'dispatch: loop {
        match pc {
            0x82708690 => {
    //   block [0x82708690..0x82708700)
	// 82708690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270869C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827086A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827086A4: 38AA1810  addi r5, r10, 0x1810
	ctx.r[5].s64 = ctx.r[10].s64 + 6160;
	// 827086A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827086AC: 390B2CE8  addi r8, r11, 0x2ce8
	ctx.r[8].s64 = ctx.r[11].s64 + 11496;
	// 827086B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827086B4: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 827086B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827086BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827086C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827086C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827086C8: 386A1840  addi r3, r10, 0x1840
	ctx.r[3].s64 = ctx.r[10].s64 + 6208;
	// 827086CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827086D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827086D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827086D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827086DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827086E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827086E4: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 827086E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827086EC: 4BD5E735  bl 0x82466e20
	ctx.lr = 0x827086F0;
	sub_82466E20(ctx, base);
	// 827086F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827086F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827086F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827086FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82708700 size=24
    let mut pc: u32 = 0x82708700;
    'dispatch: loop {
        match pc {
            0x82708700 => {
    //   block [0x82708700..0x82708718)
	// 82708700: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708704: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82708708: 394AC8A8  addi r10, r10, -0x3758
	ctx.r[10].s64 = ctx.r[10].s64 + -14168;
	// 8270870C: 816BC8A0  lwz r11, -0x3760(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14176 as u32) ) } as u64;
	// 82708710: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82708714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708718 size=116
    let mut pc: u32 = 0x82708718;
    'dispatch: loop {
        match pc {
            0x82708718 => {
    //   block [0x82708718..0x8270878C)
	// 82708718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270871C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708724: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708728: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8270872C: 390BC8A8  addi r8, r11, -0x3758
	ctx.r[8].s64 = ctx.r[11].s64 + -14168;
	// 82708730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708734: 392A2DB8  addi r9, r10, 0x2db8
	ctx.r[9].s64 = ctx.r[10].s64 + 11704;
	// 82708738: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270873C: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82708740: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82708744: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270874C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708754: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82708758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270875C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708760: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 82708764: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82708768: 386B1870  addi r3, r11, 0x1870
	ctx.r[3].s64 = ctx.r[11].s64 + 6256;
	// 8270876C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82708770: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708774: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82708778: 4BD5E6A9  bl 0x82466e20
	ctx.lr = 0x8270877C;
	sub_82466E20(ctx, base);
	// 8270877C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708790 size=92
    let mut pc: u32 = 0x82708790;
    'dispatch: loop {
        match pc {
            0x82708790 => {
    //   block [0x82708790..0x827087EC)
	// 82708790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708798: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270879C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827087A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827087A4: 4BDFDDD5  bl 0x82506578
	ctx.lr = 0x827087A8;
	sub_82506578(ctx, base);
	// 827087A8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827087AC: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 827087B0: 394BB718  addi r10, r11, -0x48e8
	ctx.r[10].s64 = ctx.r[11].s64 + -18664;
	// 827087B4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827087B8: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 827087BC: 396B18A0  addi r11, r11, 0x18a0
	ctx.r[11].s64 = ctx.r[11].s64 + 6304;
	// 827087C0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827087C4: 39489C18  addi r10, r8, -0x63e8
	ctx.r[10].s64 = ctx.r[8].s64 + -25576;
	// 827087C8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827087CC: 39499C30  addi r10, r9, -0x63d0
	ctx.r[10].s64 = ctx.r[9].s64 + -25552;
	// 827087D0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827087D4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827087D8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827087DC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 827087E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827087E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827087E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827087F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827087F0 size=112
    let mut pc: u32 = 0x827087F0;
    'dispatch: loop {
        match pc {
            0x827087F0 => {
    //   block [0x827087F0..0x82708860)
	// 827087F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827087F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827087F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827087FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82708800: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708804: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82708808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270880C: 390B2DD0  addi r8, r11, 0x2dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 11728;
	// 82708810: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82708814: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 82708818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270881C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708828: 386A18B0  addi r3, r10, 0x18b0
	ctx.r[3].s64 = ctx.r[10].s64 + 6320;
	// 8270882C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270883C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708844: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82708848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270884C: 4BD5E5D5  bl 0x82466e20
	ctx.lr = 0x82708850;
	sub_82466E20(ctx, base);
	// 82708850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270885C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708860 size=92
    let mut pc: u32 = 0x82708860;
    'dispatch: loop {
        match pc {
            0x82708860 => {
    //   block [0x82708860..0x827088BC)
	// 82708860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708868: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270886C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82708870: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708874: 4BDFDD35  bl 0x825065a8
	ctx.lr = 0x82708878;
	sub_825065A8(ctx, base);
	// 82708878: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270887C: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708880: 394BB73C  addi r10, r11, -0x48c4
	ctx.r[10].s64 = ctx.r[11].s64 + -18628;
	// 82708884: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708888: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 8270888C: 396B18E0  addi r11, r11, 0x18e0
	ctx.r[11].s64 = ctx.r[11].s64 + 6368;
	// 82708890: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708894: 39489C78  addi r10, r8, -0x6388
	ctx.r[10].s64 = ctx.r[8].s64 + -25480;
	// 82708898: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270889C: 39499C90  addi r10, r9, -0x6370
	ctx.r[10].s64 = ctx.r[9].s64 + -25456;
	// 827088A0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827088A4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827088A8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827088AC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827088B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827088B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827088B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827088C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827088C0 size=112
    let mut pc: u32 = 0x827088C0;
    'dispatch: loop {
        match pc {
            0x827088C0 => {
    //   block [0x827088C0..0x82708930)
	// 827088C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827088C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827088C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827088CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827088D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827088D4: 38AA2120  addi r5, r10, 0x2120
	ctx.r[5].s64 = ctx.r[10].s64 + 8480;
	// 827088D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827088DC: 390B2E60  addi r8, r11, 0x2e60
	ctx.r[8].s64 = ctx.r[11].s64 + 11872;
	// 827088E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 827088E4: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 827088E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827088EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827088F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827088F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827088F8: 386A18F0  addi r3, r10, 0x18f0
	ctx.r[3].s64 = ctx.r[10].s64 + 6384;
	// 827088FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270890C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708914: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 82708918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270891C: 4BD5E505  bl 0x82466e20
	ctx.lr = 0x82708920;
	sub_82466E20(ctx, base);
	// 82708920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270892C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708930 size=108
    let mut pc: u32 = 0x82708930;
    'dispatch: loop {
        match pc {
            0x82708930 => {
    //   block [0x82708930..0x8270899C)
	// 82708930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270893C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708940: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708944: 38EB2E78  addi r7, r11, 0x2e78
	ctx.r[7].s64 = ctx.r[11].s64 + 11896;
	// 82708948: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8270894C: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 82708950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708954: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270895C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708960: 386A1920  addi r3, r10, 0x1920
	ctx.r[3].s64 = ctx.r[10].s64 + 6432;
	// 82708964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82708968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270896C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270897C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82708980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82708988: 4BD5E499  bl 0x82466e20
	ctx.lr = 0x8270898C;
	sub_82466E20(ctx, base);
	// 8270898C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827089A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827089A0 size=92
    let mut pc: u32 = 0x827089A0;
    'dispatch: loop {
        match pc {
            0x827089A0 => {
    //   block [0x827089A0..0x827089FC)
	// 827089A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827089A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827089A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827089AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827089B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827089B4: 4BDFE9F5  bl 0x825073a8
	ctx.lr = 0x827089B8;
	sub_825073A8(ctx, base);
	// 827089B8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827089BC: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 827089C0: 394BB628  addi r10, r11, -0x49d8
	ctx.r[10].s64 = ctx.r[11].s64 + -18904;
	// 827089C4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827089C8: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 827089CC: 396B1950  addi r11, r11, 0x1950
	ctx.r[11].s64 = ctx.r[11].s64 + 6480;
	// 827089D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827089D4: 39489CF8  addi r10, r8, -0x6308
	ctx.r[10].s64 = ctx.r[8].s64 + -25352;
	// 827089D8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827089DC: 39499CE0  addi r10, r9, -0x6320
	ctx.r[10].s64 = ctx.r[9].s64 + -25376;
	// 827089E0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827089E4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 827089E8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 827089EC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 827089F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827089F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827089F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708A00 size=112
    let mut pc: u32 = 0x82708A00;
    'dispatch: loop {
        match pc {
            0x82708A00 => {
    //   block [0x82708A00..0x82708A70)
	// 82708A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708A0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708A10: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708A14: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 82708A18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708A1C: 390B2ED8  addi r8, r11, 0x2ed8
	ctx.r[8].s64 = ctx.r[11].s64 + 11992;
	// 82708A20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82708A24: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 82708A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708A2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708A38: 386A1960  addi r3, r10, 0x1960
	ctx.r[3].s64 = ctx.r[10].s64 + 6496;
	// 82708A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708A54: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 82708A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708A5C: 4BD5E3C5  bl 0x82466e20
	ctx.lr = 0x82708A60;
	sub_82466E20(ctx, base);
	// 82708A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708A70 size=108
    let mut pc: u32 = 0x82708A70;
    'dispatch: loop {
        match pc {
            0x82708A70 => {
    //   block [0x82708A70..0x82708ADC)
	// 82708A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708A7C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708A80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708A84: 38EB2F38  addi r7, r11, 0x2f38
	ctx.r[7].s64 = ctx.r[11].s64 + 12088;
	// 82708A88: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82708A8C: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 82708A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708A94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82708A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708AA0: 386A1990  addi r3, r10, 0x1990
	ctx.r[3].s64 = ctx.r[10].s64 + 6544;
	// 82708AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82708AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708ABC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82708AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82708AC8: 4BD5E359  bl 0x82466e20
	ctx.lr = 0x82708ACC;
	sub_82466E20(ctx, base);
	// 82708ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708AE0 size=100
    let mut pc: u32 = 0x82708AE0;
    'dispatch: loop {
        match pc {
            0x82708AE0 => {
    //   block [0x82708AE0..0x82708B44)
	// 82708AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708AEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708AF4: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 82708AF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708B00: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 82708B04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708B14: 386A19C0  addi r3, r10, 0x19c0
	ctx.r[3].s64 = ctx.r[10].s64 + 6592;
	// 82708B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708B20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82708B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708B28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82708B2C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82708B30: 4BD5E2F1  bl 0x82466e20
	ctx.lr = 0x82708B34;
	sub_82466E20(ctx, base);
	// 82708B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708B48 size=92
    let mut pc: u32 = 0x82708B48;
    'dispatch: loop {
        match pc {
            0x82708B48 => {
    //   block [0x82708B48..0x82708BA4)
	// 82708B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708B50: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708B54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82708B58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708B5C: 4BDFFE65  bl 0x825089c0
	ctx.lr = 0x82708B60;
	sub_825089C0(ctx, base);
	// 82708B60: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82708B64: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708B68: 394BB69C  addi r10, r11, -0x4964
	ctx.r[10].s64 = ctx.r[11].s64 + -18788;
	// 82708B6C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708B70: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 82708B74: 396B19F0  addi r11, r11, 0x19f0
	ctx.r[11].s64 = ctx.r[11].s64 + 6640;
	// 82708B78: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708B7C: 39489D48  addi r10, r8, -0x62b8
	ctx.r[10].s64 = ctx.r[8].s64 + -25272;
	// 82708B80: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82708B84: 39499D60  addi r10, r9, -0x62a0
	ctx.r[10].s64 = ctx.r[9].s64 + -25248;
	// 82708B88: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82708B8C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82708B90: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82708B94: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82708B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708BA8 size=112
    let mut pc: u32 = 0x82708BA8;
    'dispatch: loop {
        match pc {
            0x82708BA8 => {
    //   block [0x82708BA8..0x82708C18)
	// 82708BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708BB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82708BB8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708BBC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82708BC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708BC4: 390B2F98  addi r8, r11, 0x2f98
	ctx.r[8].s64 = ctx.r[11].s64 + 12184;
	// 82708BC8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82708BCC: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 82708BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708BD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708BE0: 386A1A00  addi r3, r10, 0x1a00
	ctx.r[3].s64 = ctx.r[10].s64 + 6656;
	// 82708BE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708BE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708BFC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82708C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708C04: 4BD5E21D  bl 0x82466e20
	ctx.lr = 0x82708C08;
	sub_82466E20(ctx, base);
	// 82708C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708C18 size=92
    let mut pc: u32 = 0x82708C18;
    'dispatch: loop {
        match pc {
            0x82708C18 => {
    //   block [0x82708C18..0x82708C74)
	// 82708C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708C20: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708C24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82708C28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708C2C: 4BDFFB15  bl 0x82508740
	ctx.lr = 0x82708C30;
	sub_82508740(ctx, base);
	// 82708C30: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82708C34: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708C38: 394BB6CC  addi r10, r11, -0x4934
	ctx.r[10].s64 = ctx.r[11].s64 + -18740;
	// 82708C3C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708C40: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 82708C44: 396B1A30  addi r11, r11, 0x1a30
	ctx.r[11].s64 = ctx.r[11].s64 + 6704;
	// 82708C48: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708C4C: 39489DA8  addi r10, r8, -0x6258
	ctx.r[10].s64 = ctx.r[8].s64 + -25176;
	// 82708C50: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82708C54: 39499DC0  addi r10, r9, -0x6240
	ctx.r[10].s64 = ctx.r[9].s64 + -25152;
	// 82708C58: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82708C5C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82708C60: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82708C64: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82708C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708C78 size=112
    let mut pc: u32 = 0x82708C78;
    'dispatch: loop {
        match pc {
            0x82708C78 => {
    //   block [0x82708C78..0x82708CE8)
	// 82708C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708C84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82708C88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708C8C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82708C90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708C94: 390B3028  addi r8, r11, 0x3028
	ctx.r[8].s64 = ctx.r[11].s64 + 12328;
	// 82708C98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82708C9C: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 82708CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708CA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708CB0: 386A1A40  addi r3, r10, 0x1a40
	ctx.r[3].s64 = ctx.r[10].s64 + 6720;
	// 82708CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708CCC: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82708CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708CD4: 4BD5E14D  bl 0x82466e20
	ctx.lr = 0x82708CD8;
	sub_82466E20(ctx, base);
	// 82708CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708CE8 size=92
    let mut pc: u32 = 0x82708CE8;
    'dispatch: loop {
        match pc {
            0x82708CE8 => {
    //   block [0x82708CE8..0x82708D44)
	// 82708CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708CF0: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708CF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82708CF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82708CFC: 4BDFFCDD  bl 0x825089d8
	ctx.lr = 0x82708D00;
	sub_825089D8(ctx, base);
	// 82708D00: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82708D04: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82708D08: 394BB6FC  addi r10, r11, -0x4904
	ctx.r[10].s64 = ctx.r[11].s64 + -18692;
	// 82708D0C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82708D10: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 82708D14: 396B1A70  addi r11, r11, 0x1a70
	ctx.r[11].s64 = ctx.r[11].s64 + 6768;
	// 82708D18: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82708D1C: 39489E08  addi r10, r8, -0x61f8
	ctx.r[10].s64 = ctx.r[8].s64 + -25080;
	// 82708D20: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82708D24: 39499E20  addi r10, r9, -0x61e0
	ctx.r[10].s64 = ctx.r[9].s64 + -25056;
	// 82708D28: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82708D2C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82708D30: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82708D34: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82708D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708D48 size=112
    let mut pc: u32 = 0x82708D48;
    'dispatch: loop {
        match pc {
            0x82708D48 => {
    //   block [0x82708D48..0x82708DB8)
	// 82708D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708D54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708D58: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708D5C: 38AA1780  addi r5, r10, 0x1780
	ctx.r[5].s64 = ctx.r[10].s64 + 6016;
	// 82708D60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708D64: 390B3088  addi r8, r11, 0x3088
	ctx.r[8].s64 = ctx.r[11].s64 + 12424;
	// 82708D68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82708D6C: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 82708D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708D74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708D80: 386A1A80  addi r3, r10, 0x1a80
	ctx.r[3].s64 = ctx.r[10].s64 + 6784;
	// 82708D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708D9C: 38C000E0  li r6, 0xe0
	ctx.r[6].s64 = 224;
	// 82708DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708DA4: 4BD5E07D  bl 0x82466e20
	ctx.lr = 0x82708DA8;
	sub_82466E20(ctx, base);
	// 82708DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708DB8 size=112
    let mut pc: u32 = 0x82708DB8;
    'dispatch: loop {
        match pc {
            0x82708DB8 => {
    //   block [0x82708DB8..0x82708E28)
	// 82708DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708DC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708DC8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708DCC: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 82708DD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708DD4: 390B30B8  addi r8, r11, 0x30b8
	ctx.r[8].s64 = ctx.r[11].s64 + 12472;
	// 82708DD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82708DDC: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 82708DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708DE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708DF0: 386A1AB0  addi r3, r10, 0x1ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 6832;
	// 82708DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708E0C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82708E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708E14: 4BD5E00D  bl 0x82466e20
	ctx.lr = 0x82708E18;
	sub_82466E20(ctx, base);
	// 82708E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82708E28 size=24
    let mut pc: u32 = 0x82708E28;
    'dispatch: loop {
        match pc {
            0x82708E28 => {
    //   block [0x82708E28..0x82708E40)
	// 82708E28: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82708E2C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82708E30: 394AC970  addi r10, r10, -0x3690
	ctx.r[10].s64 = ctx.r[10].s64 + -13968;
	// 82708E34: 816BCE44  lwz r11, -0x31bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12732 as u32) ) } as u64;
	// 82708E38: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82708E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708E40 size=116
    let mut pc: u32 = 0x82708E40;
    'dispatch: loop {
        match pc {
            0x82708E40 => {
    //   block [0x82708E40..0x82708EB4)
	// 82708E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708E4C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82708E50: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82708E54: 390AC970  addi r8, r10, -0x3690
	ctx.r[8].s64 = ctx.r[10].s64 + -13968;
	// 82708E58: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708E5C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708E60: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 82708E64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708E68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82708E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708E70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708E74: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 82708E78: 396B3110  addi r11, r11, 0x3110
	ctx.r[11].s64 = ctx.r[11].s64 + 12560;
	// 82708E7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708E80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708E84: 386A1AE0  addi r3, r10, 0x1ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 6880;
	// 82708E88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82708E8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708E90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82708E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708E9C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82708EA0: 4BD5DF81  bl 0x82466e20
	ctx.lr = 0x82708EA4;
	sub_82466E20(ctx, base);
	// 82708EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708EB8 size=112
    let mut pc: u32 = 0x82708EB8;
    'dispatch: loop {
        match pc {
            0x82708EB8 => {
    //   block [0x82708EB8..0x82708F28)
	// 82708EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708EC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708EC8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708ECC: 38AA19C0  addi r5, r10, 0x19c0
	ctx.r[5].s64 = ctx.r[10].s64 + 6592;
	// 82708ED0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708ED4: 390B3178  addi r8, r11, 0x3178
	ctx.r[8].s64 = ctx.r[11].s64 + 12664;
	// 82708ED8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82708EDC: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 82708EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708EE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708EE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708EF0: 386A1B10  addi r3, r10, 0x1b10
	ctx.r[3].s64 = ctx.r[10].s64 + 6928;
	// 82708EF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708F0C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82708F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708F14: 4BD5DF0D  bl 0x82466e20
	ctx.lr = 0x82708F18;
	sub_82466E20(ctx, base);
	// 82708F18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708F28 size=100
    let mut pc: u32 = 0x82708F28;
    'dispatch: loop {
        match pc {
            0x82708F28 => {
    //   block [0x82708F28..0x82708F8C)
	// 82708F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708F34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708F3C: 38AA2254  addi r5, r10, 0x2254
	ctx.r[5].s64 = ctx.r[10].s64 + 8788;
	// 82708F40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708F48: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 82708F4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708F5C: 386A1B40  addi r3, r10, 0x1b40
	ctx.r[3].s64 = ctx.r[10].s64 + 6976;
	// 82708F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708F64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708F68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82708F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708F70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82708F74: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82708F78: 4BD5DEA9  bl 0x82466e20
	ctx.lr = 0x82708F7C;
	sub_82466E20(ctx, base);
	// 82708F7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708F80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708F84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708F88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82708F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82708F90 size=112
    let mut pc: u32 = 0x82708F90;
    'dispatch: loop {
        match pc {
            0x82708F90 => {
    //   block [0x82708F90..0x82709000)
	// 82708F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82708F94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82708F98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82708F9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708FA0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82708FA4: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 82708FA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82708FAC: 390B31F0  addi r8, r11, 0x31f0
	ctx.r[8].s64 = ctx.r[11].s64 + 12784;
	// 82708FB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82708FB4: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 82708FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82708FBC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82708FC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82708FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82708FC8: 386A1B70  addi r3, r10, 0x1b70
	ctx.r[3].s64 = ctx.r[10].s64 + 7024;
	// 82708FCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82708FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82708FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82708FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82708FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82708FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82708FE4: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82708FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82708FEC: 4BD5DE35  bl 0x82466e20
	ctx.lr = 0x82708FF0;
	sub_82466E20(ctx, base);
	// 82708FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82708FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82708FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82708FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709000 size=112
    let mut pc: u32 = 0x82709000;
    'dispatch: loop {
        match pc {
            0x82709000 => {
    //   block [0x82709000..0x82709070)
	// 82709000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270900C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709010: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709014: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 82709018: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270901C: 390B32D0  addi r8, r11, 0x32d0
	ctx.r[8].s64 = ctx.r[11].s64 + 13008;
	// 82709020: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82709024: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 82709028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270902C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709038: 386A1BA0  addi r3, r10, 0x1ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 7072;
	// 8270903C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270904C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709054: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82709058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270905C: 4BD5DDC5  bl 0x82466e20
	ctx.lr = 0x82709060;
	sub_82466E20(ctx, base);
	// 82709060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270906C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709070 size=108
    let mut pc: u32 = 0x82709070;
    'dispatch: loop {
        match pc {
            0x82709070 => {
    //   block [0x82709070..0x827090DC)
	// 82709070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270907C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709084: 38EB3364  addi r7, r11, 0x3364
	ctx.r[7].s64 = ctx.r[11].s64 + 13156;
	// 82709088: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270908C: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 82709090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709094: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270909C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827090A0: 386A1BD0  addi r3, r10, 0x1bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 7120;
	// 827090A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827090A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827090AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827090B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827090B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827090B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827090BC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 827090C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827090C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827090C8: 4BD5DD59  bl 0x82466e20
	ctx.lr = 0x827090CC;
	sub_82466E20(ctx, base);
	// 827090CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827090D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827090D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827090D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827090E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827090E0 size=108
    let mut pc: u32 = 0x827090E0;
    'dispatch: loop {
        match pc {
            0x827090E0 => {
    //   block [0x827090E0..0x8270914C)
	// 827090E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827090E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827090E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827090EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827090F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827090F4: 38EB33B0  addi r7, r11, 0x33b0
	ctx.r[7].s64 = ctx.r[11].s64 + 13232;
	// 827090F8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 827090FC: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 82709100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709104: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270910C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709110: 386A1C00  addi r3, r10, 0x1c00
	ctx.r[3].s64 = ctx.r[10].s64 + 7168;
	// 82709114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82709118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270911C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270912C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 82709130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709138: 4BD5DCE9  bl 0x82466e20
	ctx.lr = 0x8270913C;
	sub_82466E20(ctx, base);
	// 8270913C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709150 size=116
    let mut pc: u32 = 0x82709150;
    'dispatch: loop {
        match pc {
            0x82709150 => {
    //   block [0x82709150..0x827091C4)
	// 82709150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270915C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709160: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82709164: 390B3470  addi r8, r11, 0x3470
	ctx.r[8].s64 = ctx.r[11].s64 + 13424;
	// 82709168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270916C: 392A3398  addi r9, r10, 0x3398
	ctx.r[9].s64 = ctx.r[10].s64 + 13208;
	// 82709170: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709174: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82709178: 38AA2180  addi r5, r10, 0x2180
	ctx.r[5].s64 = ctx.r[10].s64 + 8576;
	// 8270917C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709184: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270918C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709194: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82709198: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 8270919C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 827091A0: 386B1C30  addi r3, r11, 0x1c30
	ctx.r[3].s64 = ctx.r[11].s64 + 7216;
	// 827091A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 827091A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827091AC: 38C0004C  li r6, 0x4c
	ctx.r[6].s64 = 76;
	// 827091B0: 4BD5DC71  bl 0x82466e20
	ctx.lr = 0x827091B4;
	sub_82466E20(ctx, base);
	// 827091B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827091B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827091BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827091C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827091C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827091C8 size=116
    let mut pc: u32 = 0x827091C8;
    'dispatch: loop {
        match pc {
            0x827091C8 => {
    //   block [0x827091C8..0x8270923C)
	// 827091C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827091CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827091D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827091D4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827091D8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 827091DC: 390B352C  addi r8, r11, 0x352c
	ctx.r[8].s64 = ctx.r[11].s64 + 13612;
	// 827091E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827091E4: 392A3518  addi r9, r10, 0x3518
	ctx.r[9].s64 = ctx.r[10].s64 + 13592;
	// 827091E8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827091EC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 827091F0: 38AA1CF0  addi r5, r10, 0x1cf0
	ctx.r[5].s64 = ctx.r[10].s64 + 7408;
	// 827091F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827091F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827091FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270920C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82709210: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 82709214: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82709218: 386B1C60  addi r3, r11, 0x1c60
	ctx.r[3].s64 = ctx.r[11].s64 + 7264;
	// 8270921C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82709220: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709224: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82709228: 4BD5DBF9  bl 0x82466e20
	ctx.lr = 0x8270922C;
	sub_82466E20(ctx, base);
	// 8270922C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709240 size=108
    let mut pc: u32 = 0x82709240;
    'dispatch: loop {
        match pc {
            0x82709240 => {
    //   block [0x82709240..0x827092AC)
	// 82709240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270924C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709250: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709254: 38EB3544  addi r7, r11, 0x3544
	ctx.r[7].s64 = ctx.r[11].s64 + 13636;
	// 82709258: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8270925C: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 82709260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709264: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270926C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709270: 386A1C90  addi r3, r10, 0x1c90
	ctx.r[3].s64 = ctx.r[10].s64 + 7312;
	// 82709274: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82709278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270927C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270928C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82709290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709298: 4BD5DB89  bl 0x82466e20
	ctx.lr = 0x8270929C;
	sub_82466E20(ctx, base);
	// 8270929C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827092A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827092A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827092A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827092B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827092B0 size=112
    let mut pc: u32 = 0x827092B0;
    'dispatch: loop {
        match pc {
            0x827092B0 => {
    //   block [0x827092B0..0x82709320)
	// 827092B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827092B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827092B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827092BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827092C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827092C4: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 827092C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827092CC: 390B3574  addi r8, r11, 0x3574
	ctx.r[8].s64 = ctx.r[11].s64 + 13684;
	// 827092D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827092D4: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 827092D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827092DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827092E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827092E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827092E8: 386A1CC0  addi r3, r10, 0x1cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 7360;
	// 827092EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827092F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827092F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827092F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827092FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709304: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82709308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270930C: 4BD5DB15  bl 0x82466e20
	ctx.lr = 0x82709310;
	sub_82466E20(ctx, base);
	// 82709310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270931C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709320 size=100
    let mut pc: u32 = 0x82709320;
    'dispatch: loop {
        match pc {
            0x82709320 => {
    //   block [0x82709320..0x82709384)
	// 82709320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270932C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709334: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 82709338: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270933C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709340: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 82709344: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270934C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709354: 386A1CF0  addi r3, r10, 0x1cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 7408;
	// 82709358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270935C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709360: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709368: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270936C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82709370: 4BD5DAB1  bl 0x82466e20
	ctx.lr = 0x82709374;
	sub_82466E20(ctx, base);
	// 82709374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270937C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709388 size=108
    let mut pc: u32 = 0x82709388;
    'dispatch: loop {
        match pc {
            0x82709388 => {
    //   block [0x82709388..0x827093F4)
	// 82709388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270938C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709394: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709398: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270939C: 38EB35D0  addi r7, r11, 0x35d0
	ctx.r[7].s64 = ctx.r[11].s64 + 13776;
	// 827093A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 827093A4: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 827093A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827093AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827093B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 827093B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827093B8: 386A1D20  addi r3, r10, 0x1d20
	ctx.r[3].s64 = ctx.r[10].s64 + 7456;
	// 827093BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 827093C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827093C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827093C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827093CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827093D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827093D4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 827093D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827093DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 827093E0: 4BD5DA41  bl 0x82466e20
	ctx.lr = 0x827093E4;
	sub_82466E20(ctx, base);
	// 827093E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827093E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827093EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827093F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827093F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x827093F8 size=24
    let mut pc: u32 = 0x827093F8;
    'dispatch: loop {
        match pc {
            0x827093F8 => {
    //   block [0x827093F8..0x82709410)
	// 827093F8: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 827093FC: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82709400: 394ACBD8  addi r10, r10, -0x3428
	ctx.r[10].s64 = ctx.r[10].s64 + -13352;
	// 82709404: 816BCE44  lwz r11, -0x31bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12732 as u32) ) } as u64;
	// 82709408: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8270940C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709410 size=116
    let mut pc: u32 = 0x82709410;
    'dispatch: loop {
        match pc {
            0x82709410 => {
    //   block [0x82709410..0x82709484)
	// 82709410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270941C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82709420: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82709424: 390ACBD8  addi r8, r10, -0x3428
	ctx.r[8].s64 = ctx.r[10].s64 + -13352;
	// 82709428: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270942C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709430: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 82709434: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709438: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8270943C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709444: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 82709448: 396B3630  addi r11, r11, 0x3630
	ctx.r[11].s64 = ctx.r[11].s64 + 13872;
	// 8270944C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709450: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709454: 386A1D50  addi r3, r10, 0x1d50
	ctx.r[3].s64 = ctx.r[10].s64 + 7504;
	// 82709458: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8270945C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709460: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82709464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270946C: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 82709470: 4BD5D9B1  bl 0x82466e20
	ctx.lr = 0x82709474;
	sub_82466E20(ctx, base);
	// 82709474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270947C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709488 size=112
    let mut pc: u32 = 0x82709488;
    'dispatch: loop {
        match pc {
            0x82709488 => {
    //   block [0x82709488..0x827094F8)
	// 82709488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270948C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709494: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709498: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270949C: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 827094A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827094A4: 390B3698  addi r8, r11, 0x3698
	ctx.r[8].s64 = ctx.r[11].s64 + 13976;
	// 827094A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 827094AC: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 827094B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827094B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827094B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827094BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827094C0: 386A1D80  addi r3, r10, 0x1d80
	ctx.r[3].s64 = ctx.r[10].s64 + 7552;
	// 827094C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827094C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827094CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827094D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827094D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827094D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827094DC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 827094E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827094E4: 4BD5D93D  bl 0x82466e20
	ctx.lr = 0x827094E8;
	sub_82466E20(ctx, base);
	// 827094E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827094EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827094F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827094F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827094F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827094F8 size=108
    let mut pc: u32 = 0x827094F8;
    'dispatch: loop {
        match pc {
            0x827094F8 => {
    //   block [0x827094F8..0x82709564)
	// 827094F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827094FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709504: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709508: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270950C: 38EB3728  addi r7, r11, 0x3728
	ctx.r[7].s64 = ctx.r[11].s64 + 14120;
	// 82709510: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82709514: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 82709518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270951C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82709524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709528: 386A1DB0  addi r3, r10, 0x1db0
	ctx.r[3].s64 = ctx.r[10].s64 + 7600;
	// 8270952C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82709530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270953C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709544: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82709548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270954C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709550: 4BD5D8D1  bl 0x82466e20
	ctx.lr = 0x82709554;
	sub_82466E20(ctx, base);
	// 82709554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270955C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709568 size=112
    let mut pc: u32 = 0x82709568;
    'dispatch: loop {
        match pc {
            0x82709568 => {
    //   block [0x82709568..0x827095D8)
	// 82709568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270956C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709574: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709578: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270957C: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 82709580: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709584: 390B3770  addi r8, r11, 0x3770
	ctx.r[8].s64 = ctx.r[11].s64 + 14192;
	// 82709588: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8270958C: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 82709590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709594: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270959C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827095A0: 386A1DE0  addi r3, r10, 0x1de0
	ctx.r[3].s64 = ctx.r[10].s64 + 7648;
	// 827095A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827095A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827095AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827095B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827095B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827095B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827095BC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 827095C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827095C4: 4BD5D85D  bl 0x82466e20
	ctx.lr = 0x827095C8;
	sub_82466E20(ctx, base);
	// 827095C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827095CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827095D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827095D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827095D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827095D8 size=112
    let mut pc: u32 = 0x827095D8;
    'dispatch: loop {
        match pc {
            0x827095D8 => {
    //   block [0x827095D8..0x82709648)
	// 827095D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827095DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827095E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827095E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827095E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827095EC: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 827095F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827095F4: 390B3848  addi r8, r11, 0x3848
	ctx.r[8].s64 = ctx.r[11].s64 + 14408;
	// 827095F8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 827095FC: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 82709600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709604: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270960C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709610: 386A1E10  addi r3, r10, 0x1e10
	ctx.r[3].s64 = ctx.r[10].s64 + 7696;
	// 82709614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270961C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709624: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82709628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270962C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82709630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709634: 4BD5D7ED  bl 0x82466e20
	ctx.lr = 0x82709638;
	sub_82466E20(ctx, base);
	// 82709638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270963C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709648 size=112
    let mut pc: u32 = 0x82709648;
    'dispatch: loop {
        match pc {
            0x82709648 => {
    //   block [0x82709648..0x827096B8)
	// 82709648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270964C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709654: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82709658: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270965C: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82709660: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709664: 390B392C  addi r8, r11, 0x392c
	ctx.r[8].s64 = ctx.r[11].s64 + 14636;
	// 82709668: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8270966C: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 82709670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709674: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270967C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709680: 386A1E40  addi r3, r10, 0x1e40
	ctx.r[3].s64 = ctx.r[10].s64 + 7744;
	// 82709684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270968C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709694: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82709698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270969C: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 827096A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827096A4: 4BD5D77D  bl 0x82466e20
	ctx.lr = 0x827096A8;
	sub_82466E20(ctx, base);
	// 827096A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827096AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827096B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827096B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827096B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827096B8 size=92
    let mut pc: u32 = 0x827096B8;
    'dispatch: loop {
        match pc {
            0x827096B8 => {
    //   block [0x827096B8..0x82709714)
	// 827096B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827096BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827096C0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827096C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 827096C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 827096CC: 4BDF85B5  bl 0x82501c80
	ctx.lr = 0x827096D0;
	sub_82501C80(ctx, base);
	// 827096D0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 827096D4: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 827096D8: 394BB85C  addi r10, r11, -0x47a4
	ctx.r[10].s64 = ctx.r[11].s64 + -18340;
	// 827096DC: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827096E0: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 827096E4: 396B1E70  addi r11, r11, 0x1e70
	ctx.r[11].s64 = ctx.r[11].s64 + 7792;
	// 827096E8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827096EC: 3948A8B8  addi r10, r8, -0x5748
	ctx.r[10].s64 = ctx.r[8].s64 + -22344;
	// 827096F0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827096F4: 3949A8A0  addi r10, r9, -0x5760
	ctx.r[10].s64 = ctx.r[9].s64 + -22368;
	// 827096F8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827096FC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82709700: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82709704: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82709708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709718 size=116
    let mut pc: u32 = 0x82709718;
    'dispatch: loop {
        match pc {
            0x82709718 => {
    //   block [0x82709718..0x8270978C)
	// 82709718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270971C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709724: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709728: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270972C: 392B39B8  addi r9, r11, 0x39b8
	ctx.r[9].s64 = ctx.r[11].s64 + 14776;
	// 82709730: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 82709734: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709738: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8270973C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82709740: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709744: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 82709748: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270974C: 396B39E8  addi r11, r11, 0x39e8
	ctx.r[11].s64 = ctx.r[11].s64 + 14824;
	// 82709750: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82709754: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709758: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8270975C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709760: 386A1E80  addi r3, r10, 0x1e80
	ctx.r[3].s64 = ctx.r[10].s64 + 7808;
	// 82709764: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82709768: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8270976C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82709770: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82709774: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709778: 4BD5D6A9  bl 0x82466e20
	ctx.lr = 0x8270977C;
	sub_82466E20(ctx, base);
	// 8270977C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709790 size=112
    let mut pc: u32 = 0x82709790;
    'dispatch: loop {
        match pc {
            0x82709790 => {
    //   block [0x82709790..0x82709800)
	// 82709790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270979C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827097A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827097A4: 38AA1054  addi r5, r10, 0x1054
	ctx.r[5].s64 = ctx.r[10].s64 + 4180;
	// 827097A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827097AC: 390B3A78  addi r8, r11, 0x3a78
	ctx.r[8].s64 = ctx.r[11].s64 + 14968;
	// 827097B0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 827097B4: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 827097B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827097BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827097C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827097C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827097C8: 386A1EB0  addi r3, r10, 0x1eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 7856;
	// 827097CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827097D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827097D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827097D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827097DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827097E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827097E4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 827097E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827097EC: 4BD5D635  bl 0x82466e20
	ctx.lr = 0x827097F0;
	sub_82466E20(ctx, base);
	// 827097F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 827097F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827097F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827097FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709800 size=112
    let mut pc: u32 = 0x82709800;
    'dispatch: loop {
        match pc {
            0x82709800 => {
    //   block [0x82709800..0x82709870)
	// 82709800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270980C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82709810: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709814: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82709818: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8270981C: 390B3AD8  addi r8, r11, 0x3ad8
	ctx.r[8].s64 = ctx.r[11].s64 + 15064;
	// 82709820: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82709824: 388AB39C  addi r4, r10, -0x4c64
	ctx.r[4].s64 = ctx.r[10].s64 + -19556;
	// 82709828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270982C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709838: 386A1EE0  addi r3, r10, 0x1ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 7904;
	// 8270983C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270984C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709854: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82709858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270985C: 4BD5D5C5  bl 0x82466e20
	ctx.lr = 0x82709860;
	sub_82466E20(ctx, base);
	// 82709860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270986C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709870 size=96
    let mut pc: u32 = 0x82709870;
    'dispatch: loop {
        match pc {
            0x82709870 => {
    //   block [0x82709870..0x827098D0)
	// 82709870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709878: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270987C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82709880: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82709884: 4BE05625  bl 0x8250eea8
	ctx.lr = 0x82709888;
	sub_8250EEA8(ctx, base);
	// 82709888: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270988C: 3CE08250  lis r7, -0x7db0
	ctx.r[7].s64 = -2108686336;
	// 82709890: 394BB5FC  addi r10, r11, -0x4a04
	ctx.r[10].s64 = ctx.r[11].s64 + -18948;
	// 82709894: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709898: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 8270989C: 392B3BA0  addi r9, r11, 0x3ba0
	ctx.r[9].s64 = ctx.r[11].s64 + 15264;
	// 827098A0: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 827098A4: 396B1F10  addi r11, r11, 0x1f10
	ctx.r[11].s64 = ctx.r[11].s64 + 7952;
	// 827098A8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 827098AC: 3947ADC0  addi r10, r7, -0x5240
	ctx.r[10].s64 = ctx.r[7].s64 + -21056;
	// 827098B0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 827098B4: 3948ADA8  addi r10, r8, -0x5258
	ctx.r[10].s64 = ctx.r[8].s64 + -21080;
	// 827098B8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 827098BC: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 827098C0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 827098C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 827098C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 827098CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827098D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827098D0 size=100
    let mut pc: u32 = 0x827098D0;
    'dispatch: loop {
        match pc {
            0x827098D0 => {
    //   block [0x827098D0..0x82709934)
	// 827098D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827098D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827098D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827098DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827098E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827098E4: 38AA2120  addi r5, r10, 0x2120
	ctx.r[5].s64 = ctx.r[10].s64 + 8480;
	// 827098E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827098EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827098F0: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 827098F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827098F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827098FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709904: 386A1F20  addi r3, r10, 0x1f20
	ctx.r[3].s64 = ctx.r[10].s64 + 7968;
	// 82709908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270990C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709910: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709918: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270991C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82709920: 4BD5D501  bl 0x82466e20
	ctx.lr = 0x82709924;
	sub_82466E20(ctx, base);
	// 82709924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270992C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709938 size=100
    let mut pc: u32 = 0x82709938;
    'dispatch: loop {
        match pc {
            0x82709938 => {
    //   block [0x82709938..0x8270999C)
	// 82709938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270993C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709944: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270994C: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 82709950: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709958: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 8270995C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270996C: 386A1F50  addi r3, r10, 0x1f50
	ctx.r[3].s64 = ctx.r[10].s64 + 8016;
	// 82709970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709974: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709978: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270997C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709980: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709984: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82709988: 4BD5D499  bl 0x82466e20
	ctx.lr = 0x8270998C;
	sub_82466E20(ctx, base);
	// 8270998C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_827099A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x827099A0 size=112
    let mut pc: u32 = 0x827099A0;
    'dispatch: loop {
        match pc {
            0x827099A0 => {
    //   block [0x827099A0..0x82709A10)
	// 827099A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 827099A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 827099A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 827099AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827099B0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 827099B4: 38AA1CF0  addi r5, r10, 0x1cf0
	ctx.r[5].s64 = ctx.r[10].s64 + 7408;
	// 827099B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 827099BC: 390B3BC8  addi r8, r11, 0x3bc8
	ctx.r[8].s64 = ctx.r[11].s64 + 15304;
	// 827099C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 827099C4: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 827099C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 827099CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 827099D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 827099D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 827099D8: 386A1F80  addi r3, r10, 0x1f80
	ctx.r[3].s64 = ctx.r[10].s64 + 8064;
	// 827099DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 827099E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 827099E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 827099E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 827099EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 827099F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 827099F4: 38C000A0  li r6, 0xa0
	ctx.r[6].s64 = 160;
	// 827099F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 827099FC: 4BD5D425  bl 0x82466e20
	ctx.lr = 0x82709A00;
	sub_82466E20(ctx, base);
	// 82709A00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709A04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709A08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709A10 size=100
    let mut pc: u32 = 0x82709A10;
    'dispatch: loop {
        match pc {
            0x82709A10 => {
    //   block [0x82709A10..0x82709A74)
	// 82709A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709A1C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82709A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709A24: 392A3C78  addi r9, r10, 0x3c78
	ctx.r[9].s64 = ctx.r[10].s64 + 15480;
	// 82709A28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709A30: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 82709A34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709A38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709A3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709A40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709A44: 386A1FB0  addi r3, r10, 0x1fb0
	ctx.r[3].s64 = ctx.r[10].s64 + 8112;
	// 82709A48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709A4C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82709A50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709A54: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82709A58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709A60: 4BD5D3C1  bl 0x82466e20
	ctx.lr = 0x82709A64;
	sub_82466E20(ctx, base);
	// 82709A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709A78 size=112
    let mut pc: u32 = 0x82709A78;
    'dispatch: loop {
        match pc {
            0x82709A78 => {
    //   block [0x82709A78..0x82709AE8)
	// 82709A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709A84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709A88: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709A8C: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 82709A90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709A94: 390B3CB4  addi r8, r11, 0x3cb4
	ctx.r[8].s64 = ctx.r[11].s64 + 15540;
	// 82709A98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82709A9C: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 82709AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709AA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709AB0: 386A1FE0  addi r3, r10, 0x1fe0
	ctx.r[3].s64 = ctx.r[10].s64 + 8160;
	// 82709AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709ACC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 82709AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709AD4: 4BD5D34D  bl 0x82466e20
	ctx.lr = 0x82709AD8;
	sub_82466E20(ctx, base);
	// 82709AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709AE8 size=96
    let mut pc: u32 = 0x82709AE8;
    'dispatch: loop {
        match pc {
            0x82709AE8 => {
    //   block [0x82709AE8..0x82709B48)
	// 82709AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709AF4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709AFC: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 82709B00: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709B08: 386A2010  addi r3, r10, 0x2010
	ctx.r[3].s64 = ctx.r[10].s64 + 8208;
	// 82709B0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709B14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82709B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709B24: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82709B28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709B30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709B34: 4BD5D2ED  bl 0x82466e20
	ctx.lr = 0x82709B38;
	sub_82466E20(ctx, base);
	// 82709B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709B48 size=92
    let mut pc: u32 = 0x82709B48;
    'dispatch: loop {
        match pc {
            0x82709B48 => {
    //   block [0x82709B48..0x82709BA4)
	// 82709B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709B50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709B54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82709B58: 4BDF4C69  bl 0x824fe7c0
	ctx.lr = 0x82709B5C;
	sub_824FE7C0(ctx, base);
	// 82709B5C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82709B60: 3CE08250  lis r7, -0x7db0
	ctx.r[7].s64 = -2108686336;
	// 82709B64: 394BB4CC  addi r10, r11, -0x4b34
	ctx.r[10].s64 = ctx.r[11].s64 + -19252;
	// 82709B68: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709B6C: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82709B70: 392BFBA4  addi r9, r11, -0x45c
	ctx.r[9].s64 = ctx.r[11].s64 + -1116;
	// 82709B74: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82709B78: 396B2040  addi r11, r11, 0x2040
	ctx.r[11].s64 = ctx.r[11].s64 + 8256;
	// 82709B7C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82709B80: 3947B028  addi r10, r7, -0x4fd8
	ctx.r[10].s64 = ctx.r[7].s64 + -20440;
	// 82709B84: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82709B88: 3948B010  addi r10, r8, -0x4ff0
	ctx.r[10].s64 = ctx.r[8].s64 + -20464;
	// 82709B8C: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82709B90: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82709B94: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82709B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709BA8 size=100
    let mut pc: u32 = 0x82709BA8;
    'dispatch: loop {
        match pc {
            0x82709BA8 => {
    //   block [0x82709BA8..0x82709C0C)
	// 82709BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709BB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709BBC: 38AA1870  addi r5, r10, 0x1870
	ctx.r[5].s64 = ctx.r[10].s64 + 6256;
	// 82709BC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709BC8: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 82709BCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709BDC: 386A2050  addi r3, r10, 0x2050
	ctx.r[3].s64 = ctx.r[10].s64 + 8272;
	// 82709BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709BE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709BE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709BF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709BF4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 82709BF8: 4BD5D229  bl 0x82466e20
	ctx.lr = 0x82709BFC;
	sub_82466E20(ctx, base);
	// 82709BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709C10 size=96
    let mut pc: u32 = 0x82709C10;
    'dispatch: loop {
        match pc {
            0x82709C10 => {
    //   block [0x82709C10..0x82709C70)
	// 82709C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709C1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709C24: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 82709C28: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709C30: 386A2080  addi r3, r10, 0x2080
	ctx.r[3].s64 = ctx.r[10].s64 + 8320;
	// 82709C34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709C3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82709C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709C4C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82709C50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709C58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709C5C: 4BD5D1C5  bl 0x82466e20
	ctx.lr = 0x82709C60;
	sub_82466E20(ctx, base);
	// 82709C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709C70 size=112
    let mut pc: u32 = 0x82709C70;
    'dispatch: loop {
        match pc {
            0x82709C70 => {
    //   block [0x82709C70..0x82709CE0)
	// 82709C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709C7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709C80: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709C84: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 82709C88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709C8C: 390B3CD0  addi r8, r11, 0x3cd0
	ctx.r[8].s64 = ctx.r[11].s64 + 15568;
	// 82709C90: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82709C94: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 82709C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709C9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709CA8: 386A20B0  addi r3, r10, 0x20b0
	ctx.r[3].s64 = ctx.r[10].s64 + 8368;
	// 82709CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709CC4: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82709CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709CCC: 4BD5D155  bl 0x82466e20
	ctx.lr = 0x82709CD0;
	sub_82466E20(ctx, base);
	// 82709CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82709CE0 size=28
    let mut pc: u32 = 0x82709CE0;
    'dispatch: loop {
        match pc {
            0x82709CE0 => {
    //   block [0x82709CE0..0x82709CFC)
	// 82709CE0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82709CE4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82709CE8: 394ACEC0  addi r10, r10, -0x3140
	ctx.r[10].s64 = ctx.r[10].s64 + -12608;
	// 82709CEC: 816BCEA8  lwz r11, -0x3158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12632 as u32) ) } as u64;
	// 82709CF0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82709CF4: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82709CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709D00 size=112
    let mut pc: u32 = 0x82709D00;
    'dispatch: loop {
        match pc {
            0x82709D00 => {
    //   block [0x82709D00..0x82709D70)
	// 82709D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709D0C: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82709D10: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 82709D14: 38EACEC0  addi r7, r10, -0x3140
	ctx.r[7].s64 = ctx.r[10].s64 + -12608;
	// 82709D18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709D1C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709D20: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 82709D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709D28: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82709D2C: 396B3DC8  addi r11, r11, 0x3dc8
	ctx.r[11].s64 = ctx.r[11].s64 + 15816;
	// 82709D30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82709D34: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709D38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709D3C: 386A20E0  addi r3, r10, 0x20e0
	ctx.r[3].s64 = ctx.r[10].s64 + 8416;
	// 82709D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709D44: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82709D48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709D4C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82709D50: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 82709D54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709D58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709D5C: 4BD5D0C5  bl 0x82466e20
	ctx.lr = 0x82709D60;
	sub_82466E20(ctx, base);
	// 82709D60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709D70 size=92
    let mut pc: u32 = 0x82709D70;
    'dispatch: loop {
        match pc {
            0x82709D70 => {
    //   block [0x82709D70..0x82709DCC)
	// 82709D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709D78: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709D7C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82709D80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82709D84: 4BE05125  bl 0x8250eea8
	ctx.lr = 0x82709D88;
	sub_8250EEA8(ctx, base);
	// 82709D88: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82709D8C: 3D008250  lis r8, -0x7db0
	ctx.r[8].s64 = -2108686336;
	// 82709D90: 394BB65C  addi r10, r11, -0x49a4
	ctx.r[10].s64 = ctx.r[11].s64 + -18852;
	// 82709D94: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 82709D98: 3D208250  lis r9, -0x7db0
	ctx.r[9].s64 = -2108686336;
	// 82709D9C: 396B2110  addi r11, r11, 0x2110
	ctx.r[11].s64 = ctx.r[11].s64 + 8464;
	// 82709DA0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82709DA4: 3948B258  addi r10, r8, -0x4da8
	ctx.r[10].s64 = ctx.r[8].s64 + -19880;
	// 82709DA8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82709DAC: 3949B270  addi r10, r9, -0x4d90
	ctx.r[10].s64 = ctx.r[9].s64 + -19856;
	// 82709DB0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82709DB4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82709DB8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82709DBC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82709DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82709DD0 size=24
    let mut pc: u32 = 0x82709DD0;
    'dispatch: loop {
        match pc {
            0x82709DD0 => {
    //   block [0x82709DD0..0x82709DE8)
	// 82709DD0: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82709DD4: 3D408282  lis r10, -0x7d7e
	ctx.r[10].s64 = -2105409536;
	// 82709DD8: 394AD028  addi r10, r10, -0x2fd8
	ctx.r[10].s64 = ctx.r[10].s64 + -12248;
	// 82709DDC: 816BCE44  lwz r11, -0x31bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12732 as u32) ) } as u64;
	// 82709DE0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82709DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709DE8 size=116
    let mut pc: u32 = 0x82709DE8;
    'dispatch: loop {
        match pc {
            0x82709DE8 => {
    //   block [0x82709DE8..0x82709E5C)
	// 82709DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709DF4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709DF8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709DFC: 392B3DA0  addi r9, r11, 0x3da0
	ctx.r[9].s64 = ctx.r[11].s64 + 15776;
	// 82709E00: 38AA22B4  addi r5, r10, 0x22b4
	ctx.r[5].s64 = ctx.r[10].s64 + 8884;
	// 82709E04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709E08: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 82709E0C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82709E10: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 82709E14: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 82709E18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709E1C: 396BD028  addi r11, r11, -0x2fd8
	ctx.r[11].s64 = ctx.r[11].s64 + -12248;
	// 82709E20: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82709E24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709E28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82709E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709E30: 386A2120  addi r3, r10, 0x2120
	ctx.r[3].s64 = ctx.r[10].s64 + 8480;
	// 82709E34: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82709E38: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82709E3C: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 82709E40: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82709E44: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709E48: 4BD5CFD9  bl 0x82466e20
	ctx.lr = 0x82709E4C;
	sub_82466E20(ctx, base);
	// 82709E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709E60 size=112
    let mut pc: u32 = 0x82709E60;
    'dispatch: loop {
        match pc {
            0x82709E60 => {
    //   block [0x82709E60..0x82709ED0)
	// 82709E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709E6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82709E70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709E74: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 82709E78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709E7C: 390B3E28  addi r8, r11, 0x3e28
	ctx.r[8].s64 = ctx.r[11].s64 + 15912;
	// 82709E80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82709E84: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 82709E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709E8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709E90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709E98: 386A2150  addi r3, r10, 0x2150
	ctx.r[3].s64 = ctx.r[10].s64 + 8528;
	// 82709E9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709EB4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82709EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709EBC: 4BD5CF65  bl 0x82466e20
	ctx.lr = 0x82709EC0;
	sub_82466E20(ctx, base);
	// 82709EC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709ECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709ED0 size=108
    let mut pc: u32 = 0x82709ED0;
    'dispatch: loop {
        match pc {
            0x82709ED0 => {
    //   block [0x82709ED0..0x82709F3C)
	// 82709ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709EDC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709EE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709EE4: 38EB3E58  addi r7, r11, 0x3e58
	ctx.r[7].s64 = ctx.r[11].s64 + 15960;
	// 82709EE8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82709EEC: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 82709EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709EF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709EF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82709EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709F00: 386A2180  addi r3, r10, 0x2180
	ctx.r[3].s64 = ctx.r[10].s64 + 8576;
	// 82709F04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82709F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709F14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709F1C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82709F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709F24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709F28: 4BD5CEF9  bl 0x82466e20
	ctx.lr = 0x82709F2C;
	sub_82466E20(ctx, base);
	// 82709F2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709F40 size=112
    let mut pc: u32 = 0x82709F40;
    'dispatch: loop {
        match pc {
            0x82709F40 => {
    //   block [0x82709F40..0x82709FB0)
	// 82709F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709F4C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709F50: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82709F54: 38AA2314  addi r5, r10, 0x2314
	ctx.r[5].s64 = ctx.r[10].s64 + 8980;
	// 82709F58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709F5C: 390B3ED0  addi r8, r11, 0x3ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 16080;
	// 82709F60: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82709F64: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 82709F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709F6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709F70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82709F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709F78: 386A21B0  addi r3, r10, 0x21b0
	ctx.r[3].s64 = ctx.r[10].s64 + 8624;
	// 82709F7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82709F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709F88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82709F90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709F94: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 82709F98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709F9C: 4BD5CE85  bl 0x82466e20
	ctx.lr = 0x82709FA0;
	sub_82466E20(ctx, base);
	// 82709FA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82709FA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82709FA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82709FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82709FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82709FB0 size=96
    let mut pc: u32 = 0x82709FB0;
    'dispatch: loop {
        match pc {
            0x82709FB0 => {
    //   block [0x82709FB0..0x8270A010)
	// 82709FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82709FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82709FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82709FBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82709FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82709FC4: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 82709FC8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 82709FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82709FD0: 386A21E0  addi r3, r10, 0x21e0
	ctx.r[3].s64 = ctx.r[10].s64 + 8672;
	// 82709FD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82709FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82709FDC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82709FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82709FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82709FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82709FEC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82709FF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82709FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82709FF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82709FFC: 4BD5CE25  bl 0x82466e20
	ctx.lr = 0x8270A000;
	sub_82466E20(ctx, base);
	// 8270A000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A010 size=100
    let mut pc: u32 = 0x8270A010;
    'dispatch: loop {
        match pc {
            0x8270A010 => {
    //   block [0x8270A010..0x8270A074)
	// 8270A010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A01C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A024: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 8270A028: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A030: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 8270A034: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A044: 386A2210  addi r3, r10, 0x2210
	ctx.r[3].s64 = ctx.r[10].s64 + 8720;
	// 8270A048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A04C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A050: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270A054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A058: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A05C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8270A060: 4BD5CDC1  bl 0x82466e20
	ctx.lr = 0x8270A064;
	sub_82466E20(ctx, base);
	// 8270A064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A078 size=104
    let mut pc: u32 = 0x8270A078;
    'dispatch: loop {
        match pc {
            0x8270A078 => {
    //   block [0x8270A078..0x8270A0E0)
	// 8270A078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A084: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8270A088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A08C: 392A3FFC  addi r9, r10, 0x3ffc
	ctx.r[9].s64 = ctx.r[10].s64 + 16380;
	// 8270A090: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A098: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A09C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A0AC: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 8270A0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A0B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A0B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270A0BC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270A0C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A0C4: 386A2254  addi r3, r10, 0x2254
	ctx.r[3].s64 = ctx.r[10].s64 + 8788;
	// 8270A0C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270A0CC: 4BD5CD55  bl 0x82466e20
	ctx.lr = 0x8270A0D0;
	sub_82466E20(ctx, base);
	// 8270A0D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A0E0 size=112
    let mut pc: u32 = 0x8270A0E0;
    'dispatch: loop {
        match pc {
            0x8270A0E0 => {
    //   block [0x8270A0E0..0x8270A150)
	// 8270A0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A0EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A0F0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A0F4: 38AA1C60  addi r5, r10, 0x1c60
	ctx.r[5].s64 = ctx.r[10].s64 + 7264;
	// 8270A0F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A0FC: 390B40FC  addi r8, r11, 0x40fc
	ctx.r[8].s64 = ctx.r[11].s64 + 16636;
	// 8270A100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270A104: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 8270A108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A10C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A118: 386A2284  addi r3, r10, 0x2284
	ctx.r[3].s64 = ctx.r[10].s64 + 8836;
	// 8270A11C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A134: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8270A138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A13C: 4BD5CCE5  bl 0x82466e20
	ctx.lr = 0x8270A140;
	sub_82466E20(ctx, base);
	// 8270A140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A150 size=112
    let mut pc: u32 = 0x8270A150;
    'dispatch: loop {
        match pc {
            0x8270A150 => {
    //   block [0x8270A150..0x8270A1C0)
	// 8270A150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A15C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A160: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A164: 38AA2150  addi r5, r10, 0x2150
	ctx.r[5].s64 = ctx.r[10].s64 + 8528;
	// 8270A168: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A16C: 390B4114  addi r8, r11, 0x4114
	ctx.r[8].s64 = ctx.r[11].s64 + 16660;
	// 8270A170: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270A174: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 8270A178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A17C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A188: 386A22B4  addi r3, r10, 0x22b4
	ctx.r[3].s64 = ctx.r[10].s64 + 8884;
	// 8270A18C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A19C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270A1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A1A4: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8270A1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A1AC: 4BD5CC75  bl 0x82466e20
	ctx.lr = 0x8270A1B0;
	sub_82466E20(ctx, base);
	// 8270A1B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A1B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A1B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A1C0 size=112
    let mut pc: u32 = 0x8270A1C0;
    'dispatch: loop {
        match pc {
            0x8270A1C0 => {
    //   block [0x8270A1C0..0x8270A230)
	// 8270A1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A1CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A1D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A1D4: 38AA2210  addi r5, r10, 0x2210
	ctx.r[5].s64 = ctx.r[10].s64 + 8720;
	// 8270A1D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A1DC: 390B4130  addi r8, r11, 0x4130
	ctx.r[8].s64 = ctx.r[11].s64 + 16688;
	// 8270A1E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270A1E4: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 8270A1E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A1EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A1F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A1F8: 386A22E4  addi r3, r10, 0x22e4
	ctx.r[3].s64 = ctx.r[10].s64 + 8932;
	// 8270A1FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A20C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A214: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 8270A218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A21C: 4BD5CC05  bl 0x82466e20
	ctx.lr = 0x8270A220;
	sub_82466E20(ctx, base);
	// 8270A220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A230 size=112
    let mut pc: u32 = 0x8270A230;
    'dispatch: loop {
        match pc {
            0x8270A230 => {
    //   block [0x8270A230..0x8270A2A0)
	// 8270A230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A23C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A240: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A244: 38AA19C0  addi r5, r10, 0x19c0
	ctx.r[5].s64 = ctx.r[10].s64 + 6592;
	// 8270A248: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A24C: 390B41C0  addi r8, r11, 0x41c0
	ctx.r[8].s64 = ctx.r[11].s64 + 16832;
	// 8270A250: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8270A254: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 8270A258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A25C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A268: 386A2314  addi r3, r10, 0x2314
	ctx.r[3].s64 = ctx.r[10].s64 + 8980;
	// 8270A26C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A27C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A284: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 8270A288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A28C: 4BD5CB95  bl 0x82466e20
	ctx.lr = 0x8270A290;
	sub_82466E20(ctx, base);
	// 8270A290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A2A0 size=96
    let mut pc: u32 = 0x8270A2A0;
    'dispatch: loop {
        match pc {
            0x8270A2A0 => {
    //   block [0x8270A2A0..0x8270A300)
	// 8270A2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A2AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A2B4: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 8270A2B8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A2C0: 386A2344  addi r3, r10, 0x2344
	ctx.r[3].s64 = ctx.r[10].s64 + 9028;
	// 8270A2C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A2C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A2CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270A2D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A2D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A2DC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8270A2E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270A2E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A2E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A2EC: 4BD5CB35  bl 0x82466e20
	ctx.lr = 0x8270A2F0;
	sub_82466E20(ctx, base);
	// 8270A2F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A2F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A2F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270A300 size=12
    let mut pc: u32 = 0x8270A300;
    'dispatch: loop {
        match pc {
            0x8270A300 => {
    //   block [0x8270A300..0x8270A30C)
	// 8270A300: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8270A304: 386BCF90  addi r3, r11, -0x3070
	ctx.r[3].s64 = ctx.r[11].s64 + -12400;
	// 8270A308: 4BE28830  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A310 size=108
    let mut pc: u32 = 0x8270A310;
    'dispatch: loop {
        match pc {
            0x8270A310 => {
    //   block [0x8270A310..0x8270A37C)
	// 8270A310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A31C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A320: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A324: 38EB5C88  addi r7, r11, 0x5c88
	ctx.r[7].s64 = ctx.r[11].s64 + 23688;
	// 8270A328: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8270A32C: 388ACB58  addi r4, r10, -0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + -13480;
	// 8270A330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A334: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A340: 386A2684  addi r3, r10, 0x2684
	ctx.r[3].s64 = ctx.r[10].s64 + 9860;
	// 8270A344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A34C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A35C: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8270A360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A368: 4BD5CAB9  bl 0x82466e20
	ctx.lr = 0x8270A36C;
	sub_82466E20(ctx, base);
	// 8270A36C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A380 size=112
    let mut pc: u32 = 0x8270A380;
    'dispatch: loop {
        match pc {
            0x8270A380 => {
    //   block [0x8270A380..0x8270A3F0)
	// 8270A380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A38C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A390: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A394: 396B5D78  addi r11, r11, 0x5d78
	ctx.r[11].s64 = ctx.r[11].s64 + 23928;
	// 8270A398: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A39C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A3A0: 390B01C8  addi r8, r11, 0x1c8
	ctx.r[8].s64 = ctx.r[11].s64 + 456;
	// 8270A3A4: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 8270A3A8: 388ACB7C  addi r4, r10, -0x3484
	ctx.r[4].s64 = ctx.r[10].s64 + -13444;
	// 8270A3AC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A3B0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A3B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A3B8: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8270A3BC: 38C001A0  li r6, 0x1a0
	ctx.r[6].s64 = 416;
	// 8270A3C0: 386A26B4  addi r3, r10, 0x26b4
	ctx.r[3].s64 = ctx.r[10].s64 + 9908;
	// 8270A3C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8270A3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A3D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8270A3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A3D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8270A3DC: 4BD5CA45  bl 0x82466e20
	ctx.lr = 0x8270A3E0;
	sub_82466E20(ctx, base);
	// 8270A3E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A3E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A3E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A3F0 size=112
    let mut pc: u32 = 0x8270A3F0;
    'dispatch: loop {
        match pc {
            0x8270A3F0 => {
    //   block [0x8270A3F0..0x8270A460)
	// 8270A3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A3FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A400: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A404: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A40C: 390B5FA8  addi r8, r11, 0x5fa8
	ctx.r[8].s64 = ctx.r[11].s64 + 24488;
	// 8270A410: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270A414: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 8270A418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A41C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A428: 386A26E4  addi r3, r10, 0x26e4
	ctx.r[3].s64 = ctx.r[10].s64 + 9956;
	// 8270A42C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A43C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270A440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A444: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 8270A448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A44C: 4BD5C9D5  bl 0x82466e20
	ctx.lr = 0x8270A450;
	sub_82466E20(ctx, base);
	// 8270A450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A45C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A460 size=108
    let mut pc: u32 = 0x8270A460;
    'dispatch: loop {
        match pc {
            0x8270A460 => {
    //   block [0x8270A460..0x8270A4CC)
	// 8270A460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A468: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A46C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A470: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8270A474: 396B602C  addi r11, r11, 0x602c
	ctx.r[11].s64 = ctx.r[11].s64 + 24620;
	// 8270A478: 38610064  addi r3, r1, 0x64
	ctx.r[3].s64 = ctx.r[1].s64 + 100;
	// 8270A47C: B1410056  sth r10, 0x56(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(86 as u32), ctx.r[10].u16 ) };
	// 8270A480: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8270A484: 4BE2307D  bl 0x8252d500
	ctx.lr = 0x8270A488;
	sub_8252D500(ctx, base);
	// 8270A488: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8270A48C: 3D008253  lis r8, -0x7dad
	ctx.r[8].s64 = -2108489728;
	// 8270A490: 394BCF24  addi r10, r11, -0x30dc
	ctx.r[10].s64 = ctx.r[11].s64 + -12508;
	// 8270A494: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 8270A498: 3D208253  lis r9, -0x7dad
	ctx.r[9].s64 = -2108489728;
	// 8270A49C: 396B2714  addi r11, r11, 0x2714
	ctx.r[11].s64 = ctx.r[11].s64 + 10004;
	// 8270A4A0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8270A4A4: 3948C5B0  addi r10, r8, -0x3a50
	ctx.r[10].s64 = ctx.r[8].s64 + -14928;
	// 8270A4A8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8270A4AC: 3949C550  addi r10, r9, -0x3ab0
	ctx.r[10].s64 = ctx.r[9].s64 + -15024;
	// 8270A4B0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8270A4B4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8270A4B8: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8270A4BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8270A4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A4D0 size=112
    let mut pc: u32 = 0x8270A4D0;
    'dispatch: loop {
        match pc {
            0x8270A4D0 => {
    //   block [0x8270A4D0..0x8270A540)
	// 8270A4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A4DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A4E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A4E4: 38AA2BD4  addi r5, r10, 0x2bd4
	ctx.r[5].s64 = ctx.r[10].s64 + 11220;
	// 8270A4E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A4EC: 390B5FC0  addi r8, r11, 0x5fc0
	ctx.r[8].s64 = ctx.r[11].s64 + 24512;
	// 8270A4F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8270A4F4: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 8270A4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A4FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A508: 386A2724  addi r3, r10, 0x2724
	ctx.r[3].s64 = ctx.r[10].s64 + 10020;
	// 8270A50C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A524: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8270A528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A52C: 4BD5C8F5  bl 0x82466e20
	ctx.lr = 0x8270A530;
	sub_82466E20(ctx, base);
	// 8270A530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A540 size=100
    let mut pc: u32 = 0x8270A540;
    'dispatch: loop {
        match pc {
            0x8270A540 => {
    //   block [0x8270A540..0x8270A5A4)
	// 8270A540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A54C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A554: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A558: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A55C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A560: 388ACC4C  addi r4, r10, -0x33b4
	ctx.r[4].s64 = ctx.r[10].s64 + -13236;
	// 8270A564: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A574: 386A2754  addi r3, r10, 0x2754
	ctx.r[3].s64 = ctx.r[10].s64 + 10068;
	// 8270A578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A57C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A580: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270A584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A588: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A58C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270A590: 4BD5C891  bl 0x82466e20
	ctx.lr = 0x8270A594;
	sub_82466E20(ctx, base);
	// 8270A594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A5A8 size=100
    let mut pc: u32 = 0x8270A5A8;
    'dispatch: loop {
        match pc {
            0x8270A5A8 => {
    //   block [0x8270A5A8..0x8270A60C)
	// 8270A5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A5B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A5BC: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A5C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A5C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A5C8: 388ACC68  addi r4, r10, -0x3398
	ctx.r[4].s64 = ctx.r[10].s64 + -13208;
	// 8270A5CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A5DC: 386A2784  addi r3, r10, 0x2784
	ctx.r[3].s64 = ctx.r[10].s64 + 10116;
	// 8270A5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A5E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A5E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8270A5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A5F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8270A5F4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8270A5F8: 4BD5C829  bl 0x82466e20
	ctx.lr = 0x8270A5FC;
	sub_82466E20(ctx, base);
	// 8270A5FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A610 size=108
    let mut pc: u32 = 0x8270A610;
    'dispatch: loop {
        match pc {
            0x8270A610 => {
    //   block [0x8270A610..0x8270A67C)
	// 8270A610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A61C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A624: 38EB6058  addi r7, r11, 0x6058
	ctx.r[7].s64 = ctx.r[11].s64 + 24664;
	// 8270A628: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8270A62C: 388ACCF4  addi r4, r10, -0x330c
	ctx.r[4].s64 = ctx.r[10].s64 + -13068;
	// 8270A630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A634: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A638: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A63C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A640: 386A27B4  addi r3, r10, 0x27b4
	ctx.r[3].s64 = ctx.r[10].s64 + 10164;
	// 8270A644: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A65C: 38C00064  li r6, 0x64
	ctx.r[6].s64 = 100;
	// 8270A660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A668: 4BD5C7B9  bl 0x82466e20
	ctx.lr = 0x8270A66C;
	sub_82466E20(ctx, base);
	// 8270A66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A680 size=108
    let mut pc: u32 = 0x8270A680;
    'dispatch: loop {
        match pc {
            0x8270A680 => {
    //   block [0x8270A680..0x8270A6EC)
	// 8270A680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A68C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A694: 38EB6148  addi r7, r11, 0x6148
	ctx.r[7].s64 = ctx.r[11].s64 + 24904;
	// 8270A698: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8270A69C: 388ACD24  addi r4, r10, -0x32dc
	ctx.r[4].s64 = ctx.r[10].s64 + -13020;
	// 8270A6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A6A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A6A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A6AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A6B0: 386A27E4  addi r3, r10, 0x27e4
	ctx.r[3].s64 = ctx.r[10].s64 + 10212;
	// 8270A6B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A6CC: 38C000D0  li r6, 0xd0
	ctx.r[6].s64 = 208;
	// 8270A6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A6D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A6D8: 4BD5C749  bl 0x82466e20
	ctx.lr = 0x8270A6DC;
	sub_82466E20(ctx, base);
	// 8270A6DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A6F0 size=108
    let mut pc: u32 = 0x8270A6F0;
    'dispatch: loop {
        match pc {
            0x8270A6F0 => {
    //   block [0x8270A6F0..0x8270A75C)
	// 8270A6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A6FC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A704: 38EB6190  addi r7, r11, 0x6190
	ctx.r[7].s64 = ctx.r[11].s64 + 24976;
	// 8270A708: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8270A70C: 388ACD44  addi r4, r10, -0x32bc
	ctx.r[4].s64 = ctx.r[10].s64 + -12988;
	// 8270A710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A714: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A718: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A71C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A720: 386A2814  addi r3, r10, 0x2814
	ctx.r[3].s64 = ctx.r[10].s64 + 10260;
	// 8270A724: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A73C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 8270A740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A748: 4BD5C6D9  bl 0x82466e20
	ctx.lr = 0x8270A74C;
	sub_82466E20(ctx, base);
	// 8270A74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A760 size=108
    let mut pc: u32 = 0x8270A760;
    'dispatch: loop {
        match pc {
            0x8270A760 => {
    //   block [0x8270A760..0x8270A7CC)
	// 8270A760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A76C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A770: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A774: 38EB6268  addi r7, r11, 0x6268
	ctx.r[7].s64 = ctx.r[11].s64 + 25192;
	// 8270A778: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8270A77C: 388ACD68  addi r4, r10, -0x3298
	ctx.r[4].s64 = ctx.r[10].s64 + -12952;
	// 8270A780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A784: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A788: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A790: 386A2844  addi r3, r10, 0x2844
	ctx.r[3].s64 = ctx.r[10].s64 + 10308;
	// 8270A794: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A7A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A7AC: 38C00048  li r6, 0x48
	ctx.r[6].s64 = 72;
	// 8270A7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A7B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A7B8: 4BD5C669  bl 0x82466e20
	ctx.lr = 0x8270A7BC;
	sub_82466E20(ctx, base);
	// 8270A7BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A7C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A7C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A7C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A7D0 size=112
    let mut pc: u32 = 0x8270A7D0;
    'dispatch: loop {
        match pc {
            0x8270A7D0 => {
    //   block [0x8270A7D0..0x8270A840)
	// 8270A7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A7DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A7E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A7E4: 38AA2754  addi r5, r10, 0x2754
	ctx.r[5].s64 = ctx.r[10].s64 + 10068;
	// 8270A7E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A7EC: 390B6280  addi r8, r11, 0x6280
	ctx.r[8].s64 = ctx.r[11].s64 + 25216;
	// 8270A7F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8270A7F4: 388ACC80  addi r4, r10, -0x3380
	ctx.r[4].s64 = ctx.r[10].s64 + -13184;
	// 8270A7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A7FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A808: 386A2874  addi r3, r10, 0x2874
	ctx.r[3].s64 = ctx.r[10].s64 + 10356;
	// 8270A80C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A824: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8270A828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A82C: 4BD5C5F5  bl 0x82466e20
	ctx.lr = 0x8270A830;
	sub_82466E20(ctx, base);
	// 8270A830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A840 size=112
    let mut pc: u32 = 0x8270A840;
    'dispatch: loop {
        match pc {
            0x8270A840 => {
    //   block [0x8270A840..0x8270A8B0)
	// 8270A840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A84C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A850: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A854: 38AA2784  addi r5, r10, 0x2784
	ctx.r[5].s64 = ctx.r[10].s64 + 10116;
	// 8270A858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A85C: 390B62E0  addi r8, r11, 0x62e0
	ctx.r[8].s64 = ctx.r[11].s64 + 25312;
	// 8270A860: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8270A864: 388ACCA4  addi r4, r10, -0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + -13148;
	// 8270A868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A86C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A878: 386A28A4  addi r3, r10, 0x28a4
	ctx.r[3].s64 = ctx.r[10].s64 + 10404;
	// 8270A87C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A894: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 8270A898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A89C: 4BD5C585  bl 0x82466e20
	ctx.lr = 0x8270A8A0;
	sub_82466E20(ctx, base);
	// 8270A8A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A8B0 size=108
    let mut pc: u32 = 0x8270A8B0;
    'dispatch: loop {
        match pc {
            0x8270A8B0 => {
    //   block [0x8270A8B0..0x8270A91C)
	// 8270A8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A8BC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A8C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A8C4: 38EB6378  addi r7, r11, 0x6378
	ctx.r[7].s64 = ctx.r[11].s64 + 25464;
	// 8270A8C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8270A8CC: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 8270A8D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A8D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A8D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8270A8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A8E0: 386A28D4  addi r3, r10, 0x28d4
	ctx.r[3].s64 = ctx.r[10].s64 + 10452;
	// 8270A8E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8270A8E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A8FC: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 8270A900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8270A908: 4BD5C519  bl 0x82466e20
	ctx.lr = 0x8270A90C;
	sub_82466E20(ctx, base);
	// 8270A90C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A920 size=112
    let mut pc: u32 = 0x8270A920;
    'dispatch: loop {
        match pc {
            0x8270A920 => {
    //   block [0x8270A920..0x8270A990)
	// 8270A920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A92C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8270A930: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A934: 38AA8DF4  addi r5, r10, -0x720c
	ctx.r[5].s64 = ctx.r[10].s64 + -29196;
	// 8270A938: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A93C: 390B63C0  addi r8, r11, 0x63c0
	ctx.r[8].s64 = ctx.r[11].s64 + 25536;
	// 8270A940: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8270A944: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 8270A948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A94C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A958: 386A2904  addi r3, r10, 0x2904
	ctx.r[3].s64 = ctx.r[10].s64 + 10500;
	// 8270A95C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A974: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8270A978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A97C: 4BD5C4A5  bl 0x82466e20
	ctx.lr = 0x8270A980;
	sub_82466E20(ctx, base);
	// 8270A980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270A990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270A990 size=112
    let mut pc: u32 = 0x8270A990;
    'dispatch: loop {
        match pc {
            0x8270A990 => {
    //   block [0x8270A990..0x8270AA00)
	// 8270A990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8270A994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8270A998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8270A99C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A9A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8270A9A4: 38AA2B44  addi r5, r10, 0x2b44
	ctx.r[5].s64 = ctx.r[10].s64 + 11076;
	// 8270A9A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8270A9AC: 390B63D8  addi r8, r11, 0x63d8
	ctx.r[8].s64 = ctx.r[11].s64 + 25560;
	// 8270A9B0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8270A9B4: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 8270A9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8270A9BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 8270A9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8270A9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8270A9C8: 386A2934  addi r3, r10, 0x2934
	ctx.r[3].s64 = ctx.r[10].s64 + 10548;
	// 8270A9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8270A9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8270A9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8270A9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8270A9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8270A9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8270A9E4: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 8270A9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8270A9EC: 4BD5C435  bl 0x82466e20
	ctx.lr = 0x8270A9F0;
	sub_82466E20(ctx, base);
	// 8270A9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8270A9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8270A9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8270A9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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
	crate::recompiler::externs::call(ctx, base, 0x82564008);
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
    // ---- function 0x8270BEC8 size=32
    let mut pc: u32 = 0x8270BEC8;
    'dispatch: loop {
        match pc {
            0x8270BEC8 => {
    //   block [0x8270BEC8..0x8270BEE8)
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
	// 8270BEE4: 4BA11174  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270BEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270BEE8 size=4
    let mut pc: u32 = 0x8270BEE8;
    'dispatch: loop {
        match pc {
            0x8270BEE8 => {
    //   block [0x8270BEE8..0x8270BEEC)
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
    //   block [0x8270BEF0..0x8270BF20)
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
	pc = 0x8270BF20; continue 'dispatch;
            }
            0x8270BF20 => {
    //   block [0x8270BF20..0x8270BF34)
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
    //   block [0x8270BF38..0x8270BF60)
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
	pc = 0x8270BF60; continue 'dispatch;
            }
            0x8270BF60 => {
    //   block [0x8270BF60..0x8270BF80)
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
    //   block [0x8270BF80..0x8270BFA4)
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
	pc = 0x8270BFA4; continue 'dispatch;
            }
            0x8270BFA4 => {
    //   block [0x8270BFA4..0x8270BFD4)
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


pub fn sub_8270BFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270BFF8 size=92
    let mut pc: u32 = 0x8270BFF8;
    'dispatch: loop {
        match pc {
            0x8270BFF8 => {
    //   block [0x8270BFF8..0x8270C01C)
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
	pc = 0x8270C01C; continue 'dispatch;
            }
            0x8270C01C => {
    //   block [0x8270C01C..0x8270C030)
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
	pc = 0x8270C030; continue 'dispatch;
            }
            0x8270C030 => {
    //   block [0x8270C030..0x8270C054)
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


pub fn sub_8270C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8270C070 size=120
    let mut pc: u32 = 0x8270C070;
    'dispatch: loop {
        match pc {
            0x8270C070 => {
    //   block [0x8270C070..0x8270C094)
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
	pc = 0x8270C094; continue 'dispatch;
            }
            0x8270C094 => {
    //   block [0x8270C094..0x8270C0B4)
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
	pc = 0x8270C0B4; continue 'dispatch;
            }
            0x8270C0B4 => {
    //   block [0x8270C0B4..0x8270C0E0)
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
	pc = 0x8270C0E0; continue 'dispatch;
            }
            0x8270C0E0 => {
    //   block [0x8270C0E0..0x8270C0E8)
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
    //   block [0x8270C0E8..0x8270C10C)
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
	pc = 0x8270C10C; continue 'dispatch;
            }
            0x8270C10C => {
    //   block [0x8270C10C..0x8270C120)
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
	pc = 0x8270C120; continue 'dispatch;
            }
            0x8270C120 => {
    //   block [0x8270C120..0x8270C144)
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
    // ---- function 0x8270C148 size=24
    let mut pc: u32 = 0x8270C148;
    'dispatch: loop {
        match pc {
            0x8270C148 => {
    //   block [0x8270C148..0x8270C160)
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
	// 8270C15C: 4BA10EFC  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C160 size=4
    let mut pc: u32 = 0x8270C160;
    'dispatch: loop {
        match pc {
            0x8270C160 => {
    //   block [0x8270C160..0x8270C164)
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
    // ---- function 0x8270C190 size=60
    let mut pc: u32 = 0x8270C190;
    'dispatch: loop {
        match pc {
            0x8270C190 => {
    //   block [0x8270C190..0x8270C1CC)
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
    //   block [0x8270C1E0..0x8270C214)
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
	pc = 0x8270C214; continue 'dispatch;
            }
            0x8270C214 => {
    //   block [0x8270C214..0x8270C224)
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
	pc = 0x8270C224; continue 'dispatch;
            }
            0x8270C224 => {
    //   block [0x8270C224..0x8270C234)
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
	pc = 0x8270C234; continue 'dispatch;
            }
            0x8270C234 => {
    //   block [0x8270C234..0x8270C258)
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
    //   block [0x8270C258..0x8270C27C)
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
	pc = 0x8270C27C; continue 'dispatch;
            }
            0x8270C27C => {
    //   block [0x8270C27C..0x8270C2C8)
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
    //   block [0x8270C2C8..0x8270C2EC)
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
	pc = 0x8270C2EC; continue 'dispatch;
            }
            0x8270C2EC => {
    //   block [0x8270C2EC..0x8270C32C)
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
    // ---- function 0x8270C348 size=24
    let mut pc: u32 = 0x8270C348;
    'dispatch: loop {
        match pc {
            0x8270C348 => {
    //   block [0x8270C348..0x8270C360)
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
	// 8270C35C: 4BA10CFC  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C360 size=4
    let mut pc: u32 = 0x8270C360;
    'dispatch: loop {
        match pc {
            0x8270C360 => {
    //   block [0x8270C360..0x8270C364)
	// 8270C360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C368 size=24
    let mut pc: u32 = 0x8270C368;
    'dispatch: loop {
        match pc {
            0x8270C368 => {
    //   block [0x8270C368..0x8270C380)
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
	// 8270C37C: 4BA10CDC  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C380 size=4
    let mut pc: u32 = 0x8270C380;
    'dispatch: loop {
        match pc {
            0x8270C380 => {
    //   block [0x8270C380..0x8270C384)
	// 8270C380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C388 size=56
    let mut pc: u32 = 0x8270C388;
    'dispatch: loop {
        match pc {
            0x8270C388 => {
    //   block [0x8270C388..0x8270C3C0)
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
    //   block [0x8270C3D0..0x8270C40C)
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
	pc = 0x8270C40C; continue 'dispatch;
            }
            0x8270C40C => {
    //   block [0x8270C40C..0x8270C420)
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
    // ---- function 0x8270C420 size=24
    let mut pc: u32 = 0x8270C420;
    'dispatch: loop {
        match pc {
            0x8270C420 => {
    //   block [0x8270C420..0x8270C438)
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
	// 8270C434: 4BA10C24  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C438 size=4
    let mut pc: u32 = 0x8270C438;
    'dispatch: loop {
        match pc {
            0x8270C438 => {
    //   block [0x8270C438..0x8270C43C)
	// 8270C438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C440 size=24
    let mut pc: u32 = 0x8270C440;
    'dispatch: loop {
        match pc {
            0x8270C440 => {
    //   block [0x8270C440..0x8270C458)
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
	// 8270C454: 4BA10C04  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C458 size=4
    let mut pc: u32 = 0x8270C458;
    'dispatch: loop {
        match pc {
            0x8270C458 => {
    //   block [0x8270C458..0x8270C45C)
	// 8270C458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C460 size=24
    let mut pc: u32 = 0x8270C460;
    'dispatch: loop {
        match pc {
            0x8270C460 => {
    //   block [0x8270C460..0x8270C478)
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
	// 8270C474: 4BA10BE4  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C478 size=4
    let mut pc: u32 = 0x8270C478;
    'dispatch: loop {
        match pc {
            0x8270C478 => {
    //   block [0x8270C478..0x8270C47C)
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
    // ---- function 0x8270C4A0 size=24
    let mut pc: u32 = 0x8270C4A0;
    'dispatch: loop {
        match pc {
            0x8270C4A0 => {
    //   block [0x8270C4A0..0x8270C4B8)
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
	// 8270C4B4: 4BA10BA4  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4B8 size=4
    let mut pc: u32 = 0x8270C4B8;
    'dispatch: loop {
        match pc {
            0x8270C4B8 => {
    //   block [0x8270C4B8..0x8270C4BC)
	// 8270C4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4C0 size=24
    let mut pc: u32 = 0x8270C4C0;
    'dispatch: loop {
        match pc {
            0x8270C4C0 => {
    //   block [0x8270C4C0..0x8270C4D8)
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
	// 8270C4D4: 4BA10B84  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4D8 size=4
    let mut pc: u32 = 0x8270C4D8;
    'dispatch: loop {
        match pc {
            0x8270C4D8 => {
    //   block [0x8270C4D8..0x8270C4DC)
	// 8270C4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4E0 size=24
    let mut pc: u32 = 0x8270C4E0;
    'dispatch: loop {
        match pc {
            0x8270C4E0 => {
    //   block [0x8270C4E0..0x8270C4F8)
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
	// 8270C4F4: 4BA10B64  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C4F8 size=4
    let mut pc: u32 = 0x8270C4F8;
    'dispatch: loop {
        match pc {
            0x8270C4F8 => {
    //   block [0x8270C4F8..0x8270C4FC)
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
    // ---- function 0x8270C508 size=24
    let mut pc: u32 = 0x8270C508;
    'dispatch: loop {
        match pc {
            0x8270C508 => {
    //   block [0x8270C508..0x8270C520)
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
	// 8270C51C: 4BA10B3C  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C520 size=4
    let mut pc: u32 = 0x8270C520;
    'dispatch: loop {
        match pc {
            0x8270C520 => {
    //   block [0x8270C520..0x8270C524)
	// 8270C520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C528 size=24
    let mut pc: u32 = 0x8270C528;
    'dispatch: loop {
        match pc {
            0x8270C528 => {
    //   block [0x8270C528..0x8270C540)
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
	// 8270C53C: 4BA10B1C  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C540 size=4
    let mut pc: u32 = 0x8270C540;
    'dispatch: loop {
        match pc {
            0x8270C540 => {
    //   block [0x8270C540..0x8270C544)
	// 8270C540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C548 size=24
    let mut pc: u32 = 0x8270C548;
    'dispatch: loop {
        match pc {
            0x8270C548 => {
    //   block [0x8270C548..0x8270C560)
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
	// 8270C55C: 4BA10AFC  b 0x8211d058
	sub_8211D058(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8270C560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8270C560 size=4
    let mut pc: u32 = 0x8270C560;
    'dispatch: loop {
        match pc {
            0x8270C560 => {
    //   block [0x8270C560..0x8270C564)
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
    // ---- function 0x8270C570 size=32
    let mut pc: u32 = 0x8270C570;
    'dispatch: loop {
        match pc {
            0x8270C570 => {
    //   block [0x8270C570..0x8270C590)
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
    //   block [0x8270C598..0x8270C5CC)
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
	pc = 0x8270C5CC; continue 'dispatch;
            }
            0x8270C5CC => {
    //   block [0x8270C5CC..0x8270C5E0)
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
	pc = 0x8270C5E0; continue 'dispatch;
            }
            0x8270C5E0 => {
    //   block [0x8270C5E0..0x8270C5F4)
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
    // ---- function 0x8270C5F8 size=32
    let mut pc: u32 = 0x8270C5F8;
    'dispatch: loop {
        match pc {
            0x8270C5F8 => {
    //   block [0x8270C5F8..0x8270C618)
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
    // ---- function 0x8270C618 size=104
    let mut pc: u32 = 0x8270C618;
    'dispatch: loop {
        match pc {
            0x8270C618 => {
    //   block [0x8270C618..0x8270C634)
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
	pc = 0x8270C634; continue 'dispatch;
            }
            0x8270C634 => {
    //   block [0x8270C634..0x8270C644)
	// 8270C634: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 8270C638: 3920000F  li r9, 0xf
	ctx.r[9].s64 = 15;
	// 8270C63C: 394B1600  addi r10, r11, 0x1600
	ctx.r[10].s64 = ctx.r[11].s64 + 5632;
	// 8270C640: 390804A4  addi r8, r8, 0x4a4
	ctx.r[8].s64 = ctx.r[8].s64 + 1188;
	pc = 0x8270C644; continue 'dispatch;
            }
            0x8270C644 => {
    //   block [0x8270C644..0x8270C668)
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
	pc = 0x8270C668; continue 'dispatch;
            }
            0x8270C668 => {
    //   block [0x8270C668..0x8270C680)
	// 8270C668: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8270C66C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8270C670: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
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
    // ---- function 0x8270C680 size=44
    let mut pc: u32 = 0x8270C680;
    'dispatch: loop {
        match pc {
            0x8270C680 => {
    //   block [0x8270C680..0x8270C6AC)
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
    //   block [0x8270C6B0..0x8270C6D8)
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
	pc = 0x8270C6D8; continue 'dispatch;
            }
            0x8270C6D8 => {
    //   block [0x8270C6D8..0x8270C718)
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
	pc = 0x8270C718; continue 'dispatch;
            }
            0x8270C718 => {
    //   block [0x8270C718..0x8270C72C)
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


