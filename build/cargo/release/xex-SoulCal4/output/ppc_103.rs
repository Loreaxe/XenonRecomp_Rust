pub fn sub_82649D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649D78 size=100
    let mut pc: u32 = 0x82649D78;
    'dispatch: loop {
        match pc {
            0x82649D78 => {
    //   block [0x82649D78..0x82649DDC)
	// 82649D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649D84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649D8C: 38AA1660  addi r5, r10, 0x1660
	ctx.r[5].s64 = ctx.r[10].s64 + 5728;
	// 82649D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649D98: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 82649D9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649DAC: 386A1690  addi r3, r10, 0x1690
	ctx.r[3].s64 = ctx.r[10].s64 + 5776;
	// 82649DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649DB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649DB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82649DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649DC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82649DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649DC8: 4BE1D059  bl 0x82466e20
	ctx.lr = 0x82649DCC;
	sub_82466E20(ctx, base);
	// 82649DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649DE0 size=112
    let mut pc: u32 = 0x82649DE0;
    'dispatch: loop {
        match pc {
            0x82649DE0 => {
    //   block [0x82649DE0..0x82649E50)
	// 82649DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649DEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649DF0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649DF4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 82649DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649DFC: 390B1B88  addi r8, r11, 0x1b88
	ctx.r[8].s64 = ctx.r[11].s64 + 7048;
	// 82649E00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82649E04: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82649E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649E0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649E18: 386A16C0  addi r3, r10, 0x16c0
	ctx.r[3].s64 = ctx.r[10].s64 + 5824;
	// 82649E1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82649E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649E34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649E3C: 4BE1CFE5  bl 0x82466e20
	ctx.lr = 0x82649E40;
	sub_82466E20(ctx, base);
	// 82649E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649E50 size=116
    let mut pc: u32 = 0x82649E50;
    'dispatch: loop {
        match pc {
            0x82649E50 => {
    //   block [0x82649E50..0x82649EC4)
	// 82649E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649E5C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82649E60: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82649E64: 390A1BD0  addi r8, r10, 0x1bd0
	ctx.r[8].s64 = ctx.r[10].s64 + 7120;
	// 82649E68: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649E6C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649E70: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 82649E74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649E78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82649E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649E80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649E84: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 82649E88: 396B9640  addi r11, r11, -0x69c0
	ctx.r[11].s64 = ctx.r[11].s64 + -27072;
	// 82649E8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649E90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649E94: 386A16F0  addi r3, r10, 0x16f0
	ctx.r[3].s64 = ctx.r[10].s64 + 5872;
	// 82649E98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82649E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649EA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82649EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649EB0: 4BE1CF71  bl 0x82466e20
	ctx.lr = 0x82649EB4;
	sub_82466E20(ctx, base);
	// 82649EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649EC8 size=108
    let mut pc: u32 = 0x82649EC8;
    'dispatch: loop {
        match pc {
            0x82649EC8 => {
    //   block [0x82649EC8..0x82649F34)
	// 82649EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649ED4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649EDC: 38EB1C90  addi r7, r11, 0x1c90
	ctx.r[7].s64 = ctx.r[11].s64 + 7312;
	// 82649EE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82649EE4: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82649EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649EEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649EF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82649EF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82649EF8: 386A1720  addi r3, r10, 0x1720
	ctx.r[3].s64 = ctx.r[10].s64 + 5920;
	// 82649EFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82649F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82649F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82649F0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82649F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82649F1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82649F20: 4BE1CF01  bl 0x82466e20
	ctx.lr = 0x82649F24;
	sub_82466E20(ctx, base);
	// 82649F24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82649F38 size=24
    let mut pc: u32 = 0x82649F38;
    'dispatch: loop {
        match pc {
            0x82649F38 => {
    //   block [0x82649F38..0x82649F50)
	// 82649F38: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649F3C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82649F40: 394A7780  addi r10, r10, 0x7780
	ctx.r[10].s64 = ctx.r[10].s64 + 30592;
	// 82649F44: 816B1ADC  lwz r11, 0x1adc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6876 as u32) ) } as u64;
	// 82649F48: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82649F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649F50 size=116
    let mut pc: u32 = 0x82649F50;
    'dispatch: loop {
        match pc {
            0x82649F50 => {
    //   block [0x82649F50..0x82649FC4)
	// 82649F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649F5C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82649F60: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649F64: 392B9688  addi r9, r11, -0x6978
	ctx.r[9].s64 = ctx.r[11].s64 + -27000;
	// 82649F68: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 82649F6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649F70: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 82649F74: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 82649F78: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649F7C: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 82649F80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82649F84: 396B7780  addi r11, r11, 0x7780
	ctx.r[11].s64 = ctx.r[11].s64 + 30592;
	// 82649F88: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82649F8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649F90: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82649F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82649F98: 386A1750  addi r3, r10, 0x1750
	ctx.r[3].s64 = ctx.r[10].s64 + 5968;
	// 82649F9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82649FA0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82649FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82649FA8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82649FAC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82649FB0: 4BE1CE71  bl 0x82466e20
	ctx.lr = 0x82649FB4;
	sub_82466E20(ctx, base);
	// 82649FB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82649FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82649FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82649FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82649FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82649FC8 size=112
    let mut pc: u32 = 0x82649FC8;
    'dispatch: loop {
        match pc {
            0x82649FC8 => {
    //   block [0x82649FC8..0x8264A038)
	// 82649FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82649FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82649FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82649FD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649FD8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82649FDC: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 82649FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82649FE4: 390B1CD8  addi r8, r11, 0x1cd8
	ctx.r[8].s64 = ctx.r[11].s64 + 7384;
	// 82649FE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82649FEC: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82649FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82649FF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82649FF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82649FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A000: 386A1780  addi r3, r10, 0x1780
	ctx.r[3].s64 = ctx.r[10].s64 + 6016;
	// 8264A004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A024: 4BE1CDFD  bl 0x82466e20
	ctx.lr = 0x8264A028;
	sub_82466E20(ctx, base);
	// 8264A028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A038 size=112
    let mut pc: u32 = 0x8264A038;
    'dispatch: loop {
        match pc {
            0x8264A038 => {
    //   block [0x8264A038..0x8264A0A8)
	// 8264A038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A044: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A048: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A04C: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 8264A050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A054: 390B1D08  addi r8, r11, 0x1d08
	ctx.r[8].s64 = ctx.r[11].s64 + 7432;
	// 8264A058: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264A05C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 8264A060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A064: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A070: 386A17B0  addi r3, r10, 0x17b0
	ctx.r[3].s64 = ctx.r[10].s64 + 6064;
	// 8264A074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A094: 4BE1CD8D  bl 0x82466e20
	ctx.lr = 0x8264A098;
	sub_82466E20(ctx, base);
	// 8264A098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A0A8 size=100
    let mut pc: u32 = 0x8264A0A8;
    'dispatch: loop {
        match pc {
            0x8264A0A8 => {
    //   block [0x8264A0A8..0x8264A10C)
	// 8264A0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A0B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A0BC: 38AA1BD0  addi r5, r10, 0x1bd0
	ctx.r[5].s64 = ctx.r[10].s64 + 7120;
	// 8264A0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A0C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A0C8: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8264A0CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A0D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A0DC: 386A17E0  addi r3, r10, 0x17e0
	ctx.r[3].s64 = ctx.r[10].s64 + 6112;
	// 8264A0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A0E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A0E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264A0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A0F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264A0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A0F8: 4BE1CD29  bl 0x82466e20
	ctx.lr = 0x8264A0FC;
	sub_82466E20(ctx, base);
	// 8264A0FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A110 size=112
    let mut pc: u32 = 0x8264A110;
    'dispatch: loop {
        match pc {
            0x8264A110 => {
    //   block [0x8264A110..0x8264A180)
	// 8264A110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A11C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A120: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A124: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A12C: 390B1D20  addi r8, r11, 0x1d20
	ctx.r[8].s64 = ctx.r[11].s64 + 7456;
	// 8264A130: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264A134: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 8264A138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A13C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A148: 386A1810  addi r3, r10, 0x1810
	ctx.r[3].s64 = ctx.r[10].s64 + 6160;
	// 8264A14C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A16C: 4BE1CCB5  bl 0x82466e20
	ctx.lr = 0x8264A170;
	sub_82466E20(ctx, base);
	// 8264A170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A180 size=108
    let mut pc: u32 = 0x8264A180;
    'dispatch: loop {
        match pc {
            0x8264A180 => {
    //   block [0x8264A180..0x8264A1EC)
	// 8264A180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A18C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A194: 38EB1D50  addi r7, r11, 0x1d50
	ctx.r[7].s64 = ctx.r[11].s64 + 7504;
	// 8264A198: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264A19C: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 8264A1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A1A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264A1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A1B0: 386A1840  addi r3, r10, 0x1840
	ctx.r[3].s64 = ctx.r[10].s64 + 6208;
	// 8264A1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264A1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264A1D8: 4BE1CC49  bl 0x82466e20
	ctx.lr = 0x8264A1DC;
	sub_82466E20(ctx, base);
	// 8264A1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A1F0 size=108
    let mut pc: u32 = 0x8264A1F0;
    'dispatch: loop {
        match pc {
            0x8264A1F0 => {
    //   block [0x8264A1F0..0x8264A25C)
	// 8264A1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A1FC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A204: 38EB1DB0  addi r7, r11, 0x1db0
	ctx.r[7].s64 = ctx.r[11].s64 + 7600;
	// 8264A208: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264A20C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 8264A210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A214: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264A21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A220: 386A1870  addi r3, r10, 0x1870
	ctx.r[3].s64 = ctx.r[10].s64 + 6256;
	// 8264A224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264A228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264A248: 4BE1CBD9  bl 0x82466e20
	ctx.lr = 0x8264A24C;
	sub_82466E20(ctx, base);
	// 8264A24C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A260 size=116
    let mut pc: u32 = 0x8264A260;
    'dispatch: loop {
        match pc {
            0x8264A260 => {
    //   block [0x8264A260..0x8264A2D4)
	// 8264A260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A26C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A270: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8264A274: 390A1DE0  addi r8, r10, 0x1de0
	ctx.r[8].s64 = ctx.r[10].s64 + 7648;
	// 8264A278: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A27C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A280: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A284: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A288: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264A28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A294: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 8264A298: 396B96E0  addi r11, r11, -0x6920
	ctx.r[11].s64 = ctx.r[11].s64 + -26912;
	// 8264A29C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A2A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A2A4: 386A18A0  addi r3, r10, 0x18a0
	ctx.r[3].s64 = ctx.r[10].s64 + 6304;
	// 8264A2A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264A2AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A2B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264A2B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A2C0: 4BE1CB61  bl 0x82466e20
	ctx.lr = 0x8264A2C4;
	sub_82466E20(ctx, base);
	// 8264A2C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A2D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A2D8 size=112
    let mut pc: u32 = 0x8264A2D8;
    'dispatch: loop {
        match pc {
            0x8264A2D8 => {
    //   block [0x8264A2D8..0x8264A348)
	// 8264A2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A2E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A2E8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A2EC: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A2F4: 390B1F60  addi r8, r11, 0x1f60
	ctx.r[8].s64 = ctx.r[11].s64 + 8032;
	// 8264A2F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264A2FC: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 8264A300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A304: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A310: 386A18D0  addi r3, r10, 0x18d0
	ctx.r[3].s64 = ctx.r[10].s64 + 6352;
	// 8264A314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A31C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A334: 4BE1CAED  bl 0x82466e20
	ctx.lr = 0x8264A338;
	sub_82466E20(ctx, base);
	// 8264A338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A348 size=116
    let mut pc: u32 = 0x8264A348;
    'dispatch: loop {
        match pc {
            0x8264A348 => {
    //   block [0x8264A348..0x8264A3BC)
	// 8264A348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A354: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A358: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8264A35C: 390A1F78  addi r8, r10, 0x1f78
	ctx.r[8].s64 = ctx.r[10].s64 + 8056;
	// 8264A360: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A364: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A368: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A36C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A370: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264A374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A37C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 8264A380: 396B972C  addi r11, r11, -0x68d4
	ctx.r[11].s64 = ctx.r[11].s64 + -26836;
	// 8264A384: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A388: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A38C: 386A1900  addi r3, r10, 0x1900
	ctx.r[3].s64 = ctx.r[10].s64 + 6400;
	// 8264A390: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264A394: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A398: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264A39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A3A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A3A8: 4BE1CA79  bl 0x82466e20
	ctx.lr = 0x8264A3AC;
	sub_82466E20(ctx, base);
	// 8264A3AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A3B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A3C0 size=112
    let mut pc: u32 = 0x8264A3C0;
    'dispatch: loop {
        match pc {
            0x8264A3C0 => {
    //   block [0x8264A3C0..0x8264A430)
	// 8264A3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A3CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A3D0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A3D4: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A3DC: 390B1FD8  addi r8, r11, 0x1fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 8152;
	// 8264A3E0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8264A3E4: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 8264A3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A3EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A3F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A3F8: 386A1930  addi r3, r10, 0x1930
	ctx.r[3].s64 = ctx.r[10].s64 + 6448;
	// 8264A3FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A40C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A41C: 4BE1CA05  bl 0x82466e20
	ctx.lr = 0x8264A420;
	sub_82466E20(ctx, base);
	// 8264A420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A430 size=112
    let mut pc: u32 = 0x8264A430;
    'dispatch: loop {
        match pc {
            0x8264A430 => {
    //   block [0x8264A430..0x8264A4A0)
	// 8264A430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A43C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A440: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A444: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A44C: 390B20B0  addi r8, r11, 0x20b0
	ctx.r[8].s64 = ctx.r[11].s64 + 8368;
	// 8264A450: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8264A454: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 8264A458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A45C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A468: 386A1960  addi r3, r10, 0x1960
	ctx.r[3].s64 = ctx.r[10].s64 + 6496;
	// 8264A46C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A48C: 4BE1C995  bl 0x82466e20
	ctx.lr = 0x8264A490;
	sub_82466E20(ctx, base);
	// 8264A490: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A494: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A498: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A49C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A4A0 size=112
    let mut pc: u32 = 0x8264A4A0;
    'dispatch: loop {
        match pc {
            0x8264A4A0 => {
    //   block [0x8264A4A0..0x8264A510)
	// 8264A4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A4AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A4B0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A4B4: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A4B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264A4BC: 390B21B8  addi r8, r11, 0x21b8
	ctx.r[8].s64 = ctx.r[11].s64 + 8632;
	// 8264A4C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264A4C4: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 8264A4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A4CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A4D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A4D8: 386A1990  addi r3, r10, 0x1990
	ctx.r[3].s64 = ctx.r[10].s64 + 6544;
	// 8264A4DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A4EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A4FC: 4BE1C925  bl 0x82466e20
	ctx.lr = 0x8264A500;
	sub_82466E20(ctx, base);
	// 8264A500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A50C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264A510 size=24
    let mut pc: u32 = 0x8264A510;
    'dispatch: loop {
        match pc {
            0x8264A510 => {
    //   block [0x8264A510..0x8264A528)
	// 8264A510: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A514: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A518: 394A78E8  addi r10, r10, 0x78e8
	ctx.r[10].s64 = ctx.r[10].s64 + 30952;
	// 8264A51C: 816B21E8  lwz r11, 0x21e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8680 as u32) ) } as u64;
	// 8264A520: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8264A524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A528 size=116
    let mut pc: u32 = 0x8264A528;
    'dispatch: loop {
        match pc {
            0x8264A528 => {
    //   block [0x8264A528..0x8264A59C)
	// 8264A528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A534: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A538: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A53C: 392B975C  addi r9, r11, -0x68a4
	ctx.r[9].s64 = ctx.r[11].s64 + -26788;
	// 8264A540: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A544: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264A548: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8264A54C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8264A550: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A554: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 8264A558: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A55C: 396B78E8  addi r11, r11, 0x78e8
	ctx.r[11].s64 = ctx.r[11].s64 + 30952;
	// 8264A560: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8264A564: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A568: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264A56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A570: 386A19C0  addi r3, r10, 0x19c0
	ctx.r[3].s64 = ctx.r[10].s64 + 6592;
	// 8264A574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264A578: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264A57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A580: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264A584: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264A588: 4BE1C899  bl 0x82466e20
	ctx.lr = 0x8264A58C;
	sub_82466E20(ctx, base);
	// 8264A58C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A5A0 size=116
    let mut pc: u32 = 0x8264A5A0;
    'dispatch: loop {
        match pc {
            0x8264A5A0 => {
    //   block [0x8264A5A0..0x8264A614)
	// 8264A5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A5AC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A5B0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8264A5B4: 390A21EC  addi r8, r10, 0x21ec
	ctx.r[8].s64 = ctx.r[10].s64 + 8684;
	// 8264A5B8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A5BC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A5C0: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264A5C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A5C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264A5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A5D4: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 8264A5D8: 396B97CC  addi r11, r11, -0x6834
	ctx.r[11].s64 = ctx.r[11].s64 + -26676;
	// 8264A5DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A5E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A5E4: 386A19F0  addi r3, r10, 0x19f0
	ctx.r[3].s64 = ctx.r[10].s64 + 6640;
	// 8264A5E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264A5EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A5F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264A5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A600: 4BE1C821  bl 0x82466e20
	ctx.lr = 0x8264A604;
	sub_82466E20(ctx, base);
	// 8264A604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A618 size=108
    let mut pc: u32 = 0x8264A618;
    'dispatch: loop {
        match pc {
            0x8264A618 => {
    //   block [0x8264A618..0x8264A684)
	// 8264A618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A624: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A62C: 38EB2220  addi r7, r11, 0x2220
	ctx.r[7].s64 = ctx.r[11].s64 + 8736;
	// 8264A630: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8264A634: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 8264A638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A63C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264A644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A648: 386A1A20  addi r3, r10, 0x1a20
	ctx.r[3].s64 = ctx.r[10].s64 + 6688;
	// 8264A64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264A650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264A670: 4BE1C7B1  bl 0x82466e20
	ctx.lr = 0x8264A674;
	sub_82466E20(ctx, base);
	// 8264A674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264A688 size=24
    let mut pc: u32 = 0x8264A688;
    'dispatch: loop {
        match pc {
            0x8264A688 => {
    //   block [0x8264A688..0x8264A6A0)
	// 8264A688: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A68C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A690: 394A7A80  addi r10, r10, 0x7a80
	ctx.r[10].s64 = ctx.r[10].s64 + 31360;
	// 8264A694: 816B221C  lwz r11, 0x221c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8732 as u32) ) } as u64;
	// 8264A698: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8264A69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A6A0 size=116
    let mut pc: u32 = 0x8264A6A0;
    'dispatch: loop {
        match pc {
            0x8264A6A0 => {
    //   block [0x8264A6A0..0x8264A714)
	// 8264A6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A6AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A6B0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A6B4: 392B97F0  addi r9, r11, -0x6810
	ctx.r[9].s64 = ctx.r[11].s64 + -26640;
	// 8264A6B8: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A6BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A6C0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8264A6C4: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8264A6C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A6CC: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 8264A6D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A6D4: 396B7A80  addi r11, r11, 0x7a80
	ctx.r[11].s64 = ctx.r[11].s64 + 31360;
	// 8264A6D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8264A6DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A6E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264A6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A6E8: 386A1A50  addi r3, r10, 0x1a50
	ctx.r[3].s64 = ctx.r[10].s64 + 6736;
	// 8264A6EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264A6F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264A6F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A6F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264A6FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264A700: 4BE1C721  bl 0x82466e20
	ctx.lr = 0x8264A704;
	sub_82466E20(ctx, base);
	// 8264A704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A70C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A718 size=112
    let mut pc: u32 = 0x8264A718;
    'dispatch: loop {
        match pc {
            0x8264A718 => {
    //   block [0x8264A718..0x8264A788)
	// 8264A718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A724: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A728: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A72C: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A734: 390B22B0  addi r8, r11, 0x22b0
	ctx.r[8].s64 = ctx.r[11].s64 + 8880;
	// 8264A738: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264A73C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 8264A740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A744: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A750: 386A1A80  addi r3, r10, 0x1a80
	ctx.r[3].s64 = ctx.r[10].s64 + 6784;
	// 8264A754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A774: 4BE1C6AD  bl 0x82466e20
	ctx.lr = 0x8264A778;
	sub_82466E20(ctx, base);
	// 8264A778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A788 size=116
    let mut pc: u32 = 0x8264A788;
    'dispatch: loop {
        match pc {
            0x8264A788 => {
    //   block [0x8264A788..0x8264A7FC)
	// 8264A788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A794: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A798: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8264A79C: 390A22E0  addi r8, r10, 0x22e0
	ctx.r[8].s64 = ctx.r[10].s64 + 8928;
	// 8264A7A0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A7A4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A7A8: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A7AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A7B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264A7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A7B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A7BC: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 8264A7C0: 396B9838  addi r11, r11, -0x67c8
	ctx.r[11].s64 = ctx.r[11].s64 + -26568;
	// 8264A7C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A7C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A7CC: 386A1AB0  addi r3, r10, 0x1ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 6832;
	// 8264A7D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264A7D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A7D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264A7DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A7E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A7E8: 4BE1C639  bl 0x82466e20
	ctx.lr = 0x8264A7EC;
	sub_82466E20(ctx, base);
	// 8264A7EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A7F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A7F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A7F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A800 size=108
    let mut pc: u32 = 0x8264A800;
    'dispatch: loop {
        match pc {
            0x8264A800 => {
    //   block [0x8264A800..0x8264A86C)
	// 8264A800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A80C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A814: 38EB2370  addi r7, r11, 0x2370
	ctx.r[7].s64 = ctx.r[11].s64 + 9072;
	// 8264A818: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 8264A81C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 8264A820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A824: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A828: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264A82C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A830: 386A1AE0  addi r3, r10, 0x1ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 6880;
	// 8264A834: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264A838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A83C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A854: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264A858: 4BE1C5C9  bl 0x82466e20
	ctx.lr = 0x8264A85C;
	sub_82466E20(ctx, base);
	// 8264A85C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A870 size=116
    let mut pc: u32 = 0x8264A870;
    'dispatch: loop {
        match pc {
            0x8264A870 => {
    //   block [0x8264A870..0x8264A8E4)
	// 8264A870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A87C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264A880: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8264A884: 390A24C0  addi r8, r10, 0x24c0
	ctx.r[8].s64 = ctx.r[10].s64 + 9408;
	// 8264A888: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A88C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264A890: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A894: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A898: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264A89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A8A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A8A4: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 8264A8A8: 396B985C  addi r11, r11, -0x67a4
	ctx.r[11].s64 = ctx.r[11].s64 + -26532;
	// 8264A8AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A8B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A8B4: 386A1B10  addi r3, r10, 0x1b10
	ctx.r[3].s64 = ctx.r[10].s64 + 6928;
	// 8264A8B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264A8BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A8C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264A8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A8D0: 4BE1C551  bl 0x82466e20
	ctx.lr = 0x8264A8D4;
	sub_82466E20(ctx, base);
	// 8264A8D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A8E8 size=112
    let mut pc: u32 = 0x8264A8E8;
    'dispatch: loop {
        match pc {
            0x8264A8E8 => {
    //   block [0x8264A8E8..0x8264A958)
	// 8264A8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A8F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A8F8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A8FC: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A904: 390B2580  addi r8, r11, 0x2580
	ctx.r[8].s64 = ctx.r[11].s64 + 9600;
	// 8264A908: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264A90C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 8264A910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A914: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A918: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A920: 386A1B40  addi r3, r10, 0x1b40
	ctx.r[3].s64 = ctx.r[10].s64 + 6976;
	// 8264A924: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A93C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A944: 4BE1C4DD  bl 0x82466e20
	ctx.lr = 0x8264A948;
	sub_82466E20(ctx, base);
	// 8264A948: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A94C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A950: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A958 size=112
    let mut pc: u32 = 0x8264A958;
    'dispatch: loop {
        match pc {
            0x8264A958 => {
    //   block [0x8264A958..0x8264A9C8)
	// 8264A958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A968: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A96C: 38AA1930  addi r5, r10, 0x1930
	ctx.r[5].s64 = ctx.r[10].s64 + 6448;
	// 8264A970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A974: 390B2598  addi r8, r11, 0x2598
	ctx.r[8].s64 = ctx.r[11].s64 + 9624;
	// 8264A978: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8264A97C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 8264A980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A984: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A988: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264A990: 386A1B70  addi r3, r10, 0x1b70
	ctx.r[3].s64 = ctx.r[10].s64 + 7024;
	// 8264A994: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264A998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264A99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264A9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264A9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264A9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264A9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264A9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264A9B4: 4BE1C46D  bl 0x82466e20
	ctx.lr = 0x8264A9B8;
	sub_82466E20(ctx, base);
	// 8264A9B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264A9BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264A9C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264A9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264A9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264A9C8 size=112
    let mut pc: u32 = 0x8264A9C8;
    'dispatch: loop {
        match pc {
            0x8264A9C8 => {
    //   block [0x8264A9C8..0x8264AA38)
	// 8264A9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264A9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264A9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264A9D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A9D8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264A9DC: 38AA17E0  addi r5, r10, 0x17e0
	ctx.r[5].s64 = ctx.r[10].s64 + 6112;
	// 8264A9E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264A9E4: 390B2628  addi r8, r11, 0x2628
	ctx.r[8].s64 = ctx.r[11].s64 + 9768;
	// 8264A9E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264A9EC: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 8264A9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264A9F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264A9F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264A9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AA00: 386A1BA0  addi r3, r10, 0x1ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 7072;
	// 8264AA04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264AA08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AA10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AA14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AA18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AA20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AA24: 4BE1C3FD  bl 0x82466e20
	ctx.lr = 0x8264AA28;
	sub_82466E20(ctx, base);
	// 8264AA28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AA38 size=116
    let mut pc: u32 = 0x8264AA38;
    'dispatch: loop {
        match pc {
            0x8264AA38 => {
    //   block [0x8264AA38..0x8264AAAC)
	// 8264AA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AA44: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AA48: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264AA4C: 390B2644  addi r8, r11, 0x2644
	ctx.r[8].s64 = ctx.r[11].s64 + 9796;
	// 8264AA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AA54: 392A98A4  addi r9, r10, -0x675c
	ctx.r[9].s64 = ctx.r[10].s64 + -26460;
	// 8264AA58: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AA5C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8264AA60: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264AA64: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264AA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AA6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AA7C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264AA80: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 8264AA84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264AA88: 386B1BD0  addi r3, r11, 0x1bd0
	ctx.r[3].s64 = ctx.r[11].s64 + 7120;
	// 8264AA8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264AA90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AA98: 4BE1C389  bl 0x82466e20
	ctx.lr = 0x8264AA9C;
	sub_82466E20(ctx, base);
	// 8264AA9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AAA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AAB0 size=100
    let mut pc: u32 = 0x8264AAB0;
    'dispatch: loop {
        match pc {
            0x8264AAB0 => {
    //   block [0x8264AAB0..0x8264AB14)
	// 8264AAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AABC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AAC4: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264AAC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AAD0: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 8264AAD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AAE4: 386A1C00  addi r3, r10, 0x1c00
	ctx.r[3].s64 = ctx.r[10].s64 + 7168;
	// 8264AAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AAEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AAF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264AAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AAF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264AAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AB00: 4BE1C321  bl 0x82466e20
	ctx.lr = 0x8264AB04;
	sub_82466E20(ctx, base);
	// 8264AB04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AB08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AB0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AB10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AB18 size=112
    let mut pc: u32 = 0x8264AB18;
    'dispatch: loop {
        match pc {
            0x8264AB18 => {
    //   block [0x8264AB18..0x8264AB88)
	// 8264AB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AB24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AB28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AB2C: 38AA1C00  addi r5, r10, 0x1c00
	ctx.r[5].s64 = ctx.r[10].s64 + 7168;
	// 8264AB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AB34: 390B265C  addi r8, r11, 0x265c
	ctx.r[8].s64 = ctx.r[11].s64 + 9820;
	// 8264AB38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264AB3C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 8264AB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AB44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264AB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AB50: 386A1C30  addi r3, r10, 0x1c30
	ctx.r[3].s64 = ctx.r[10].s64 + 7216;
	// 8264AB54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264AB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AB74: 4BE1C2AD  bl 0x82466e20
	ctx.lr = 0x8264AB78;
	sub_82466E20(ctx, base);
	// 8264AB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AB88 size=108
    let mut pc: u32 = 0x8264AB88;
    'dispatch: loop {
        match pc {
            0x8264AB88 => {
    //   block [0x8264AB88..0x8264ABF4)
	// 8264AB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AB94: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AB98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8264AB9C: 38EB2678  addi r7, r11, 0x2678
	ctx.r[7].s64 = ctx.r[11].s64 + 9848;
	// 8264ABA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264ABA4: 388A446C  addi r4, r10, 0x446c
	ctx.r[4].s64 = ctx.r[10].s64 + 17516;
	// 8264ABA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264ABAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ABB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264ABB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264ABB8: 386A1C60  addi r3, r10, 0x1c60
	ctx.r[3].s64 = ctx.r[10].s64 + 7264;
	// 8264ABBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264ABC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264ABC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264ABC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264ABCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264ABD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264ABD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264ABD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264ABDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264ABE0: 4BE1C241  bl 0x82466e20
	ctx.lr = 0x8264ABE4;
	sub_82466E20(ctx, base);
	// 8264ABE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264ABE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264ABEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264ABF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264ABF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264ABF8 size=112
    let mut pc: u32 = 0x8264ABF8;
    'dispatch: loop {
        match pc {
            0x8264ABF8 => {
    //   block [0x8264ABF8..0x8264AC68)
	// 8264ABF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264ABFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AC04: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264AC08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AC0C: 392B9910  addi r9, r11, -0x66f0
	ctx.r[9].s64 = ctx.r[11].s64 + -26352;
	// 8264AC10: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8264AC14: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8264AC18: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AC1C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 8264AC20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AC24: 396B26D8  addi r11, r11, 0x26d8
	ctx.r[11].s64 = ctx.r[11].s64 + 9944;
	// 8264AC28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264AC2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AC30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8264AC34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AC38: 386A1C90  addi r3, r10, 0x1c90
	ctx.r[3].s64 = ctx.r[10].s64 + 7312;
	// 8264AC3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264AC40: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264AC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AC48: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264AC4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264AC50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264AC54: 4BE1C1CD  bl 0x82466e20
	ctx.lr = 0x8264AC58;
	sub_82466E20(ctx, base);
	// 8264AC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AC68 size=108
    let mut pc: u32 = 0x8264AC68;
    'dispatch: loop {
        match pc {
            0x8264AC68 => {
    //   block [0x8264AC68..0x8264ACD4)
	// 8264AC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AC74: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AC78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AC7C: 38EB2798  addi r7, r11, 0x2798
	ctx.r[7].s64 = ctx.r[11].s64 + 10136;
	// 8264AC80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264AC84: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 8264AC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AC8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AC90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264AC94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AC98: 386A1CC0  addi r3, r10, 0x1cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 7360;
	// 8264AC9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264ACA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264ACA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264ACA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264ACAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264ACB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264ACB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264ACB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264ACBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264ACC0: 4BE1C161  bl 0x82466e20
	ctx.lr = 0x8264ACC4;
	sub_82466E20(ctx, base);
	// 8264ACC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264ACC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264ACCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264ACD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264ACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264ACD8 size=116
    let mut pc: u32 = 0x8264ACD8;
    'dispatch: loop {
        match pc {
            0x8264ACD8 => {
    //   block [0x8264ACD8..0x8264AD4C)
	// 8264ACD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264ACDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264ACE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264ACE4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264ACE8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 8264ACEC: 390A27E0  addi r8, r10, 0x27e0
	ctx.r[8].s64 = ctx.r[10].s64 + 10208;
	// 8264ACF0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ACF4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264ACF8: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 8264ACFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AD00: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264AD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AD08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264AD0C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 8264AD10: 396B9948  addi r11, r11, -0x66b8
	ctx.r[11].s64 = ctx.r[11].s64 + -26296;
	// 8264AD14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AD18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AD1C: 386A1CF0  addi r3, r10, 0x1cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 7408;
	// 8264AD20: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264AD24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AD28: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264AD2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AD38: 4BE1C0E9  bl 0x82466e20
	ctx.lr = 0x8264AD3C;
	sub_82466E20(ctx, base);
	// 8264AD3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AD40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AD44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AD50 size=100
    let mut pc: u32 = 0x8264AD50;
    'dispatch: loop {
        match pc {
            0x8264AD50 => {
    //   block [0x8264AD50..0x8264ADB4)
	// 8264AD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AD5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AD64: 38AA1630  addi r5, r10, 0x1630
	ctx.r[5].s64 = ctx.r[10].s64 + 5680;
	// 8264AD68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AD6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AD70: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 8264AD74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AD7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AD84: 386A1D20  addi r3, r10, 0x1d20
	ctx.r[3].s64 = ctx.r[10].s64 + 7456;
	// 8264AD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AD8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AD90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264AD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AD98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264AD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264ADA0: 4BE1C081  bl 0x82466e20
	ctx.lr = 0x8264ADA4;
	sub_82466E20(ctx, base);
	// 8264ADA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264ADA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264ADAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264ADB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264ADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264ADB8 size=24
    let mut pc: u32 = 0x8264ADB8;
    'dispatch: loop {
        match pc {
            0x8264ADB8 => {
    //   block [0x8264ADB8..0x8264ADD0)
	// 8264ADB8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264ADBC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264ADC0: 394A7B88  addi r10, r10, 0x7b88
	ctx.r[10].s64 = ctx.r[10].s64 + 31624;
	// 8264ADC4: 816B2964  lwz r11, 0x2964(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10596 as u32) ) } as u64;
	// 8264ADC8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8264ADCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264ADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264ADD0 size=116
    let mut pc: u32 = 0x8264ADD0;
    'dispatch: loop {
        match pc {
            0x8264ADD0 => {
    //   block [0x8264ADD0..0x8264AE44)
	// 8264ADD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264ADD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264ADD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264ADDC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264ADE0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264ADE4: 392B99C8  addi r9, r11, -0x6638
	ctx.r[9].s64 = ctx.r[11].s64 + -26168;
	// 8264ADE8: 38AA1D20  addi r5, r10, 0x1d20
	ctx.r[5].s64 = ctx.r[10].s64 + 7456;
	// 8264ADEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264ADF0: 38E90028  addi r7, r9, 0x28
	ctx.r[7].s64 = ctx.r[9].s64 + 40;
	// 8264ADF4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 8264ADF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264ADFC: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 8264AE00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AE04: 396B7B88  addi r11, r11, 0x7b88
	ctx.r[11].s64 = ctx.r[11].s64 + 31624;
	// 8264AE08: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8264AE0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AE10: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8264AE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AE18: 386A1D50  addi r3, r10, 0x1d50
	ctx.r[3].s64 = ctx.r[10].s64 + 7504;
	// 8264AE1C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8264AE20: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8264AE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AE28: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8264AE2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264AE30: 4BE1BFF1  bl 0x82466e20
	ctx.lr = 0x8264AE34;
	sub_82466E20(ctx, base);
	// 8264AE34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AE48 size=112
    let mut pc: u32 = 0x8264AE48;
    'dispatch: loop {
        match pc {
            0x8264AE48 => {
    //   block [0x8264AE48..0x8264AEB8)
	// 8264AE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AE54: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264AE58: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8264AE5C: 38EA2970  addi r7, r10, 0x2970
	ctx.r[7].s64 = ctx.r[10].s64 + 10608;
	// 8264AE60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AE64: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8264AE68: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 8264AE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AE70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264AE74: 396B9A54  addi r11, r11, -0x65ac
	ctx.r[11].s64 = ctx.r[11].s64 + -26028;
	// 8264AE78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264AE7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AE80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AE84: 386A1D80  addi r3, r10, 0x1d80
	ctx.r[3].s64 = ctx.r[10].s64 + 7552;
	// 8264AE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AE8C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8264AE90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AE94: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8264AE98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AE9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AEA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264AEA4: 4BE1BF7D  bl 0x82466e20
	ctx.lr = 0x8264AEA8;
	sub_82466E20(ctx, base);
	// 8264AEA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AEB8 size=108
    let mut pc: u32 = 0x8264AEB8;
    'dispatch: loop {
        match pc {
            0x8264AEB8 => {
    //   block [0x8264AEB8..0x8264AF24)
	// 8264AEB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AEBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AEC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AEC4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AECC: 38EB29D0  addi r7, r11, 0x29d0
	ctx.r[7].s64 = ctx.r[11].s64 + 10704;
	// 8264AED0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264AED4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 8264AED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AEDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AEE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264AEE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AEE8: 386A1DB0  addi r3, r10, 0x1db0
	ctx.r[3].s64 = ctx.r[10].s64 + 7600;
	// 8264AEEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264AEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AF0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264AF10: 4BE1BF11  bl 0x82466e20
	ctx.lr = 0x8264AF14;
	sub_82466E20(ctx, base);
	// 8264AF14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AF28 size=116
    let mut pc: u32 = 0x8264AF28;
    'dispatch: loop {
        match pc {
            0x8264AF28 => {
    //   block [0x8264AF28..0x8264AF9C)
	// 8264AF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AF34: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264AF38: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264AF3C: 390B2A00  addi r8, r11, 0x2a00
	ctx.r[8].s64 = ctx.r[11].s64 + 10752;
	// 8264AF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AF44: 392A9A40  addi r9, r10, -0x65c0
	ctx.r[9].s64 = ctx.r[10].s64 + -26048;
	// 8264AF48: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AF4C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8264AF50: 38AA1E40  addi r5, r10, 0x1e40
	ctx.r[5].s64 = ctx.r[10].s64 + 7744;
	// 8264AF54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264AF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AF5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264AF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264AF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AF6C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8264AF70: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 8264AF74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264AF78: 386B1DE0  addi r3, r11, 0x1de0
	ctx.r[3].s64 = ctx.r[11].s64 + 7648;
	// 8264AF7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264AF80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AF88: 4BE1BE99  bl 0x82466e20
	ctx.lr = 0x8264AF8C;
	sub_82466E20(ctx, base);
	// 8264AF8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264AFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264AFA0 size=96
    let mut pc: u32 = 0x8264AFA0;
    'dispatch: loop {
        match pc {
            0x8264AFA0 => {
    //   block [0x8264AFA0..0x8264B000)
	// 8264AFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264AFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264AFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264AFAC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264AFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264AFB4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 8264AFB8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264AFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264AFC0: 386A1E10  addi r3, r10, 0x1e10
	ctx.r[3].s64 = ctx.r[10].s64 + 7696;
	// 8264AFC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264AFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264AFCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264AFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264AFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264AFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264AFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264AFE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8264AFE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264AFE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8264AFEC: 4BE1BE35  bl 0x82466e20
	ctx.lr = 0x8264AFF0;
	sub_82466E20(ctx, base);
	// 8264AFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264AFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264AFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264AFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B000 size=112
    let mut pc: u32 = 0x8264B000;
    'dispatch: loop {
        match pc {
            0x8264B000 => {
    //   block [0x8264B000..0x8264B070)
	// 8264B000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B00C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B010: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B014: 38AA1E10  addi r5, r10, 0x1e10
	ctx.r[5].s64 = ctx.r[10].s64 + 7696;
	// 8264B018: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B01C: 390B2A18  addi r8, r11, 0x2a18
	ctx.r[8].s64 = ctx.r[11].s64 + 10776;
	// 8264B020: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8264B024: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 8264B028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B02C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B038: 386A1E40  addi r3, r10, 0x1e40
	ctx.r[3].s64 = ctx.r[10].s64 + 7744;
	// 8264B03C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264B040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B05C: 4BE1BDC5  bl 0x82466e20
	ctx.lr = 0x8264B060;
	sub_82466E20(ctx, base);
	// 8264B060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B070 size=112
    let mut pc: u32 = 0x8264B070;
    'dispatch: loop {
        match pc {
            0x8264B070 => {
    //   block [0x8264B070..0x8264B0E0)
	// 8264B070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B07C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B080: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B084: 392A9A70  addi r9, r10, -0x6590
	ctx.r[9].s64 = ctx.r[10].s64 + -26000;
	// 8264B088: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B08C: 390B2A48  addi r8, r11, 0x2a48
	ctx.r[8].s64 = ctx.r[11].s64 + 10824;
	// 8264B090: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8264B094: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 8264B098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B09C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B0A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B0A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B0A8: 386A1E70  addi r3, r10, 0x1e70
	ctx.r[3].s64 = ctx.r[10].s64 + 7792;
	// 8264B0AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B0B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264B0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B0C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B0CC: 4BE1BD55  bl 0x82466e20
	ctx.lr = 0x8264B0D0;
	sub_82466E20(ctx, base);
	// 8264B0D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B0E0 size=108
    let mut pc: u32 = 0x8264B0E0;
    'dispatch: loop {
        match pc {
            0x8264B0E0 => {
    //   block [0x8264B0E0..0x8264B14C)
	// 8264B0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B0EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B0F0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B0F4: 38EB2AF0  addi r7, r11, 0x2af0
	ctx.r[7].s64 = ctx.r[11].s64 + 10992;
	// 8264B0F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B0FC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 8264B100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B104: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B110: 386A1EA0  addi r3, r10, 0x1ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 7840;
	// 8264B114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B138: 4BE1BCE9  bl 0x82466e20
	ctx.lr = 0x8264B13C;
	sub_82466E20(ctx, base);
	// 8264B13C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B150 size=108
    let mut pc: u32 = 0x8264B150;
    'dispatch: loop {
        match pc {
            0x8264B150 => {
    //   block [0x8264B150..0x8264B1BC)
	// 8264B150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B15C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B160: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B164: 38EB2B20  addi r7, r11, 0x2b20
	ctx.r[7].s64 = ctx.r[11].s64 + 11040;
	// 8264B168: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B16C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 8264B170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B174: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B180: 386A1ED0  addi r3, r10, 0x1ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 7888;
	// 8264B184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B1A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B1A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B1A8: 4BE1BC79  bl 0x82466e20
	ctx.lr = 0x8264B1AC;
	sub_82466E20(ctx, base);
	// 8264B1AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264B1C0 size=28
    let mut pc: u32 = 0x8264B1C0;
    'dispatch: loop {
        match pc {
            0x8264B1C0 => {
    //   block [0x8264B1C0..0x8264B1DC)
	// 8264B1C0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B1C4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264B1C8: 394A7CA8  addi r10, r10, 0x7ca8
	ctx.r[10].s64 = ctx.r[10].s64 + 31912;
	// 8264B1CC: 816B2B50  lwz r11, 0x2b50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11088 as u32) ) } as u64;
	// 8264B1D0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8264B1D4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8264B1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B1E0 size=112
    let mut pc: u32 = 0x8264B1E0;
    'dispatch: loop {
        match pc {
            0x8264B1E0 => {
    //   block [0x8264B1E0..0x8264B250)
	// 8264B1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B1EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B1F0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B1F4: 392A9BF0  addi r9, r10, -0x6410
	ctx.r[9].s64 = ctx.r[10].s64 + -25616;
	// 8264B1F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B1FC: 390B7CA8  addi r8, r11, 0x7ca8
	ctx.r[8].s64 = ctx.r[11].s64 + 31912;
	// 8264B200: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8264B204: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 8264B208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B20C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B218: 386A1F00  addi r3, r10, 0x1f00
	ctx.r[3].s64 = ctx.r[10].s64 + 7936;
	// 8264B21C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B220: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8264B224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B22C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B23C: 4BE1BBE5  bl 0x82466e20
	ctx.lr = 0x8264B240;
	sub_82466E20(ctx, base);
	// 8264B240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B250 size=108
    let mut pc: u32 = 0x8264B250;
    'dispatch: loop {
        match pc {
            0x8264B250 => {
    //   block [0x8264B250..0x8264B2BC)
	// 8264B250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B25C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B260: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B264: 38EB2B5C  addi r7, r11, 0x2b5c
	ctx.r[7].s64 = ctx.r[11].s64 + 11100;
	// 8264B268: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264B26C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 8264B270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B274: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B280: 386A1F30  addi r3, r10, 0x1f30
	ctx.r[3].s64 = ctx.r[10].s64 + 7984;
	// 8264B284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B2A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B2A8: 4BE1BB79  bl 0x82466e20
	ctx.lr = 0x8264B2AC;
	sub_82466E20(ctx, base);
	// 8264B2AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B2C0 size=108
    let mut pc: u32 = 0x8264B2C0;
    'dispatch: loop {
        match pc {
            0x8264B2C0 => {
    //   block [0x8264B2C0..0x8264B32C)
	// 8264B2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B2CC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B2D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B2D4: 38EB2B74  addi r7, r11, 0x2b74
	ctx.r[7].s64 = ctx.r[11].s64 + 11124;
	// 8264B2D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B2DC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 8264B2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B2E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B2E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B2F0: 386A1F60  addi r3, r10, 0x1f60
	ctx.r[3].s64 = ctx.r[10].s64 + 8032;
	// 8264B2F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B2FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B30C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B318: 4BE1BB09  bl 0x82466e20
	ctx.lr = 0x8264B31C;
	sub_82466E20(ctx, base);
	// 8264B31C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B330 size=108
    let mut pc: u32 = 0x8264B330;
    'dispatch: loop {
        match pc {
            0x8264B330 => {
    //   block [0x8264B330..0x8264B39C)
	// 8264B330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B33C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B340: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B344: 38EB2BA4  addi r7, r11, 0x2ba4
	ctx.r[7].s64 = ctx.r[11].s64 + 11172;
	// 8264B348: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264B34C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 8264B350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B354: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B360: 386A1F90  addi r3, r10, 0x1f90
	ctx.r[3].s64 = ctx.r[10].s64 + 8080;
	// 8264B364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B388: 4BE1BA99  bl 0x82466e20
	ctx.lr = 0x8264B38C;
	sub_82466E20(ctx, base);
	// 8264B38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264B3A0 size=24
    let mut pc: u32 = 0x8264B3A0;
    'dispatch: loop {
        match pc {
            0x8264B3A0 => {
    //   block [0x8264B3A0..0x8264B3B8)
	// 8264B3A0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B3A4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264B3A8: 394A7D68  addi r10, r10, 0x7d68
	ctx.r[10].s64 = ctx.r[10].s64 + 32104;
	// 8264B3AC: 816B2BBC  lwz r11, 0x2bbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11196 as u32) ) } as u64;
	// 8264B3B0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8264B3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B3B8 size=112
    let mut pc: u32 = 0x8264B3B8;
    'dispatch: loop {
        match pc {
            0x8264B3B8 => {
    //   block [0x8264B3B8..0x8264B428)
	// 8264B3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B3C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B3C4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B3C8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B3CC: 392A9C44  addi r9, r10, -0x63bc
	ctx.r[9].s64 = ctx.r[10].s64 + -25532;
	// 8264B3D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B3D4: 390B7D68  addi r8, r11, 0x7d68
	ctx.r[8].s64 = ctx.r[11].s64 + 32104;
	// 8264B3D8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8264B3DC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8264B3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B3E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B3E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B3F0: 386A1FC0  addi r3, r10, 0x1fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 8128;
	// 8264B3F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B3F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264B3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B40C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B414: 4BE1BA0D  bl 0x82466e20
	ctx.lr = 0x8264B418;
	sub_82466E20(ctx, base);
	// 8264B418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B428 size=112
    let mut pc: u32 = 0x8264B428;
    'dispatch: loop {
        match pc {
            0x8264B428 => {
    //   block [0x8264B428..0x8264B498)
	// 8264B428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B434: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B438: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B43C: 392A9C80  addi r9, r10, -0x6380
	ctx.r[9].s64 = ctx.r[10].s64 + -25472;
	// 8264B440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B444: 390B2BC8  addi r8, r11, 0x2bc8
	ctx.r[8].s64 = ctx.r[11].s64 + 11208;
	// 8264B448: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8264B44C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 8264B450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B458: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B460: 386A1FF0  addi r3, r10, 0x1ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 8176;
	// 8264B464: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B468: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8264B46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B47C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B484: 4BE1B99D  bl 0x82466e20
	ctx.lr = 0x8264B488;
	sub_82466E20(ctx, base);
	// 8264B488: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B48C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B490: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B498 size=108
    let mut pc: u32 = 0x8264B498;
    'dispatch: loop {
        match pc {
            0x8264B498 => {
    //   block [0x8264B498..0x8264B504)
	// 8264B498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B4A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B4A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B4AC: 38EB2C10  addi r7, r11, 0x2c10
	ctx.r[7].s64 = ctx.r[11].s64 + 11280;
	// 8264B4B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B4B4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8264B4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B4BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B4C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B4C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B4C8: 386A2020  addi r3, r10, 0x2020
	ctx.r[3].s64 = ctx.r[10].s64 + 8224;
	// 8264B4CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B4D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B4DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B4E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B4EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B4F0: 4BE1B931  bl 0x82466e20
	ctx.lr = 0x8264B4F4;
	sub_82466E20(ctx, base);
	// 8264B4F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B4F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B4FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B508 size=108
    let mut pc: u32 = 0x8264B508;
    'dispatch: loop {
        match pc {
            0x8264B508 => {
    //   block [0x8264B508..0x8264B574)
	// 8264B508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B514: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B518: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B51C: 38EB2C40  addi r7, r11, 0x2c40
	ctx.r[7].s64 = ctx.r[11].s64 + 11328;
	// 8264B520: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B524: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8264B528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B52C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B538: 386A2050  addi r3, r10, 0x2050
	ctx.r[3].s64 = ctx.r[10].s64 + 8272;
	// 8264B53C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B55C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B560: 4BE1B8C1  bl 0x82466e20
	ctx.lr = 0x8264B564;
	sub_82466E20(ctx, base);
	// 8264B564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B578 size=112
    let mut pc: u32 = 0x8264B578;
    'dispatch: loop {
        match pc {
            0x8264B578 => {
    //   block [0x8264B578..0x8264B5E8)
	// 8264B578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B584: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B588: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B58C: 392A9CB8  addi r9, r10, -0x6348
	ctx.r[9].s64 = ctx.r[10].s64 + -25416;
	// 8264B590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B594: 390B2C78  addi r8, r11, 0x2c78
	ctx.r[8].s64 = ctx.r[11].s64 + 11384;
	// 8264B598: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8264B59C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8264B5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B5A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B5B0: 386A2080  addi r3, r10, 0x2080
	ctx.r[3].s64 = ctx.r[10].s64 + 8320;
	// 8264B5B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B5B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264B5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B5CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B5D4: 4BE1B84D  bl 0x82466e20
	ctx.lr = 0x8264B5D8;
	sub_82466E20(ctx, base);
	// 8264B5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B5E8 size=108
    let mut pc: u32 = 0x8264B5E8;
    'dispatch: loop {
        match pc {
            0x8264B5E8 => {
    //   block [0x8264B5E8..0x8264B654)
	// 8264B5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B5F4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B5F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B5FC: 38EB2CD8  addi r7, r11, 0x2cd8
	ctx.r[7].s64 = ctx.r[11].s64 + 11480;
	// 8264B600: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8264B604: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8264B608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B60C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B618: 386A20B0  addi r3, r10, 0x20b0
	ctx.r[3].s64 = ctx.r[10].s64 + 8368;
	// 8264B61C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B63C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B640: 4BE1B7E1  bl 0x82466e20
	ctx.lr = 0x8264B644;
	sub_82466E20(ctx, base);
	// 8264B644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B658 size=108
    let mut pc: u32 = 0x8264B658;
    'dispatch: loop {
        match pc {
            0x8264B658 => {
    //   block [0x8264B658..0x8264B6C4)
	// 8264B658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B664: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B668: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8264B66C: 38EB2DE0  addi r7, r11, 0x2de0
	ctx.r[7].s64 = ctx.r[11].s64 + 11744;
	// 8264B670: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264B674: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 8264B678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B67C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B688: 386A20E0  addi r3, r10, 0x20e0
	ctx.r[3].s64 = ctx.r[10].s64 + 8416;
	// 8264B68C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B69C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B6AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B6B0: 4BE1B771  bl 0x82466e20
	ctx.lr = 0x8264B6B4;
	sub_82466E20(ctx, base);
	// 8264B6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B6C8 size=108
    let mut pc: u32 = 0x8264B6C8;
    'dispatch: loop {
        match pc {
            0x8264B6C8 => {
    //   block [0x8264B6C8..0x8264B734)
	// 8264B6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B6D4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B6DC: 38EB2DF8  addi r7, r11, 0x2df8
	ctx.r[7].s64 = ctx.r[11].s64 + 11768;
	// 8264B6E0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8264B6E4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 8264B6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B6EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B6F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B6F8: 386A2110  addi r3, r10, 0x2110
	ctx.r[3].s64 = ctx.r[10].s64 + 8464;
	// 8264B6FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B71C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B720: 4BE1B701  bl 0x82466e20
	ctx.lr = 0x8264B724;
	sub_82466E20(ctx, base);
	// 8264B724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264B738 size=24
    let mut pc: u32 = 0x8264B738;
    'dispatch: loop {
        match pc {
            0x8264B738 => {
    //   block [0x8264B738..0x8264B750)
	// 8264B738: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B73C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264B740: 394A7DF8  addi r10, r10, 0x7df8
	ctx.r[10].s64 = ctx.r[10].s64 + 32248;
	// 8264B744: 816B2C74  lwz r11, 0x2c74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11380 as u32) ) } as u64;
	// 8264B748: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8264B74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B750 size=108
    let mut pc: u32 = 0x8264B750;
    'dispatch: loop {
        match pc {
            0x8264B750 => {
    //   block [0x8264B750..0x8264B7BC)
	// 8264B750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B75C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B764: 38EB7DF8  addi r7, r11, 0x7df8
	ctx.r[7].s64 = ctx.r[11].s64 + 32248;
	// 8264B768: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B76C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 8264B770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B774: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B778: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B77C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B780: 386A2140  addi r3, r10, 0x2140
	ctx.r[3].s64 = ctx.r[10].s64 + 8512;
	// 8264B784: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B78C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B79C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B7A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B7A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B7A8: 4BE1B679  bl 0x82466e20
	ctx.lr = 0x8264B7AC;
	sub_82466E20(ctx, base);
	// 8264B7AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B7B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B7B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B7B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264B7C0 size=24
    let mut pc: u32 = 0x8264B7C0;
    'dispatch: loop {
        match pc {
            0x8264B7C0 => {
    //   block [0x8264B7C0..0x8264B7D8)
	// 8264B7C0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B7C4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264B7C8: 394A7E28  addi r10, r10, 0x7e28
	ctx.r[10].s64 = ctx.r[10].s64 + 32296;
	// 8264B7CC: 816B2C74  lwz r11, 0x2c74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11380 as u32) ) } as u64;
	// 8264B7D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8264B7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B7D8 size=108
    let mut pc: u32 = 0x8264B7D8;
    'dispatch: loop {
        match pc {
            0x8264B7D8 => {
    //   block [0x8264B7D8..0x8264B844)
	// 8264B7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B7E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B7E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B7EC: 38EB7E28  addi r7, r11, 0x7e28
	ctx.r[7].s64 = ctx.r[11].s64 + 32296;
	// 8264B7F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B7F4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 8264B7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B7FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B808: 386A2170  addi r3, r10, 0x2170
	ctx.r[3].s64 = ctx.r[10].s64 + 8560;
	// 8264B80C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B82C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B830: 4BE1B5F1  bl 0x82466e20
	ctx.lr = 0x8264B834;
	sub_82466E20(ctx, base);
	// 8264B834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B848 size=108
    let mut pc: u32 = 0x8264B848;
    'dispatch: loop {
        match pc {
            0x8264B848 => {
    //   block [0x8264B848..0x8264B8B4)
	// 8264B848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B854: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B85C: 38EB2E70  addi r7, r11, 0x2e70
	ctx.r[7].s64 = ctx.r[11].s64 + 11888;
	// 8264B860: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264B864: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 8264B868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B86C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B878: 386A21A0  addi r3, r10, 0x21a0
	ctx.r[3].s64 = ctx.r[10].s64 + 8608;
	// 8264B87C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B89C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B8A0: 4BE1B581  bl 0x82466e20
	ctx.lr = 0x8264B8A4;
	sub_82466E20(ctx, base);
	// 8264B8A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264B8B8 size=24
    let mut pc: u32 = 0x8264B8B8;
    'dispatch: loop {
        match pc {
            0x8264B8B8 => {
    //   block [0x8264B8B8..0x8264B8D0)
	// 8264B8B8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B8BC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264B8C0: 394A7E58  addi r10, r10, 0x7e58
	ctx.r[10].s64 = ctx.r[10].s64 + 32344;
	// 8264B8C4: 816B2C74  lwz r11, 0x2c74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11380 as u32) ) } as u64;
	// 8264B8C8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8264B8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B8D0 size=108
    let mut pc: u32 = 0x8264B8D0;
    'dispatch: loop {
        match pc {
            0x8264B8D0 => {
    //   block [0x8264B8D0..0x8264B93C)
	// 8264B8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B8DC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B8E4: 38EB7E58  addi r7, r11, 0x7e58
	ctx.r[7].s64 = ctx.r[11].s64 + 32344;
	// 8264B8E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B8EC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 8264B8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B8F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B8F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B900: 386A21D0  addi r3, r10, 0x21d0
	ctx.r[3].s64 = ctx.r[10].s64 + 8656;
	// 8264B904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B928: 4BE1B4F9  bl 0x82466e20
	ctx.lr = 0x8264B92C;
	sub_82466E20(ctx, base);
	// 8264B92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B940 size=112
    let mut pc: u32 = 0x8264B940;
    'dispatch: loop {
        match pc {
            0x8264B940 => {
    //   block [0x8264B940..0x8264B9B0)
	// 8264B940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B94C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264B950: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B954: 392A9CFC  addi r9, r10, -0x6304
	ctx.r[9].s64 = ctx.r[10].s64 + -25348;
	// 8264B958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B95C: 390B2E88  addi r8, r11, 0x2e88
	ctx.r[8].s64 = ctx.r[11].s64 + 11912;
	// 8264B960: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8264B964: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 8264B968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B96C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264B974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264B978: 386A2200  addi r3, r10, 0x2200
	ctx.r[3].s64 = ctx.r[10].s64 + 8704;
	// 8264B97C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264B980: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264B984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264B998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264B99C: 4BE1B485  bl 0x82466e20
	ctx.lr = 0x8264B9A0;
	sub_82466E20(ctx, base);
	// 8264B9A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264B9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264B9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264B9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264B9B0 size=108
    let mut pc: u32 = 0x8264B9B0;
    'dispatch: loop {
        match pc {
            0x8264B9B0 => {
    //   block [0x8264B9B0..0x8264BA1C)
	// 8264B9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264B9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264B9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264B9BC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264B9C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264B9C4: 38EB2EB8  addi r7, r11, 0x2eb8
	ctx.r[7].s64 = ctx.r[11].s64 + 11960;
	// 8264B9C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264B9CC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 8264B9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264B9D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264B9D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264B9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264B9E0: 386A2230  addi r3, r10, 0x2230
	ctx.r[3].s64 = ctx.r[10].s64 + 8752;
	// 8264B9E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264B9E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264B9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264B9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264B9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264B9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264B9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BA04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BA08: 4BE1B419  bl 0x82466e20
	ctx.lr = 0x8264BA0C;
	sub_82466E20(ctx, base);
	// 8264BA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BA20 size=108
    let mut pc: u32 = 0x8264BA20;
    'dispatch: loop {
        match pc {
            0x8264BA20 => {
    //   block [0x8264BA20..0x8264BA8C)
	// 8264BA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BA2C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BA34: 38EB2EE8  addi r7, r11, 0x2ee8
	ctx.r[7].s64 = ctx.r[11].s64 + 12008;
	// 8264BA38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BA3C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 8264BA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BA44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BA48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BA50: 386A2260  addi r3, r10, 0x2260
	ctx.r[3].s64 = ctx.r[10].s64 + 8800;
	// 8264BA54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BA74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BA78: 4BE1B3A9  bl 0x82466e20
	ctx.lr = 0x8264BA7C;
	sub_82466E20(ctx, base);
	// 8264BA7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BA90 size=112
    let mut pc: u32 = 0x8264BA90;
    'dispatch: loop {
        match pc {
            0x8264BA90 => {
    //   block [0x8264BA90..0x8264BB00)
	// 8264BA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BA9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BAA0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BAA4: 38AA22C0  addi r5, r10, 0x22c0
	ctx.r[5].s64 = ctx.r[10].s64 + 8896;
	// 8264BAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BAAC: 390B2F18  addi r8, r11, 0x2f18
	ctx.r[8].s64 = ctx.r[11].s64 + 12056;
	// 8264BAB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8264BAB4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 8264BAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BABC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BAC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264BAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BAC8: 386A2290  addi r3, r10, 0x2290
	ctx.r[3].s64 = ctx.r[10].s64 + 8848;
	// 8264BACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8264BAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BAD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BAEC: 4BE1B335  bl 0x82466e20
	ctx.lr = 0x8264BAF0;
	sub_82466E20(ctx, base);
	// 8264BAF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BB00 size=108
    let mut pc: u32 = 0x8264BB00;
    'dispatch: loop {
        match pc {
            0x8264BB00 => {
    //   block [0x8264BB00..0x8264BB6C)
	// 8264BB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BB0C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BB14: 38EB2F30  addi r7, r11, 0x2f30
	ctx.r[7].s64 = ctx.r[11].s64 + 12080;
	// 8264BB18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BB1C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 8264BB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BB24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BB28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BB2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BB30: 386A22C0  addi r3, r10, 0x22c0
	ctx.r[3].s64 = ctx.r[10].s64 + 8896;
	// 8264BB34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BB54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BB58: 4BE1B2C9  bl 0x82466e20
	ctx.lr = 0x8264BB5C;
	sub_82466E20(ctx, base);
	// 8264BB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BB70 size=108
    let mut pc: u32 = 0x8264BB70;
    'dispatch: loop {
        match pc {
            0x8264BB70 => {
    //   block [0x8264BB70..0x8264BBDC)
	// 8264BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BB7C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BB84: 38EB2F60  addi r7, r11, 0x2f60
	ctx.r[7].s64 = ctx.r[11].s64 + 12128;
	// 8264BB88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264BB8C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 8264BB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BB94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BB98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BBA0: 386A22F0  addi r3, r10, 0x22f0
	ctx.r[3].s64 = ctx.r[10].s64 + 8944;
	// 8264BBA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BBA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BBB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BBC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BBC8: 4BE1B259  bl 0x82466e20
	ctx.lr = 0x8264BBCC;
	sub_82466E20(ctx, base);
	// 8264BBCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BBD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BBD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BBE0 size=108
    let mut pc: u32 = 0x8264BBE0;
    'dispatch: loop {
        match pc {
            0x8264BBE0 => {
    //   block [0x8264BBE0..0x8264BC4C)
	// 8264BBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BBEC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BBF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BBF4: 38EB2F78  addi r7, r11, 0x2f78
	ctx.r[7].s64 = ctx.r[11].s64 + 12152;
	// 8264BBF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BBFC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 8264BC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BC04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BC08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BC10: 386A2320  addi r3, r10, 0x2320
	ctx.r[3].s64 = ctx.r[10].s64 + 8992;
	// 8264BC14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BC38: 4BE1B1E9  bl 0x82466e20
	ctx.lr = 0x8264BC3C;
	sub_82466E20(ctx, base);
	// 8264BC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BC50 size=108
    let mut pc: u32 = 0x8264BC50;
    'dispatch: loop {
        match pc {
            0x8264BC50 => {
    //   block [0x8264BC50..0x8264BCBC)
	// 8264BC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BC5C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BC60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BC64: 38EB2FA8  addi r7, r11, 0x2fa8
	ctx.r[7].s64 = ctx.r[11].s64 + 12200;
	// 8264BC68: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8264BC6C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 8264BC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BC74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BC78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BC80: 386A2350  addi r3, r10, 0x2350
	ctx.r[3].s64 = ctx.r[10].s64 + 9040;
	// 8264BC84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BC88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BC90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BC98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BCA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BCA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BCA8: 4BE1B179  bl 0x82466e20
	ctx.lr = 0x8264BCAC;
	sub_82466E20(ctx, base);
	// 8264BCAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BCB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BCB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BCC0 size=108
    let mut pc: u32 = 0x8264BCC0;
    'dispatch: loop {
        match pc {
            0x8264BCC0 => {
    //   block [0x8264BCC0..0x8264BD2C)
	// 8264BCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BCC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BCCC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BCD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BCD4: 38EB3050  addi r7, r11, 0x3050
	ctx.r[7].s64 = ctx.r[11].s64 + 12368;
	// 8264BCD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BCDC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 8264BCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BCE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BCE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BCEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BCF0: 386A2380  addi r3, r10, 0x2380
	ctx.r[3].s64 = ctx.r[10].s64 + 9088;
	// 8264BCF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BCF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BD04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BD14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BD18: 4BE1B109  bl 0x82466e20
	ctx.lr = 0x8264BD1C;
	sub_82466E20(ctx, base);
	// 8264BD1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BD30 size=108
    let mut pc: u32 = 0x8264BD30;
    'dispatch: loop {
        match pc {
            0x8264BD30 => {
    //   block [0x8264BD30..0x8264BD9C)
	// 8264BD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BD3C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BD40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BD44: 38EB3080  addi r7, r11, 0x3080
	ctx.r[7].s64 = ctx.r[11].s64 + 12416;
	// 8264BD48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264BD4C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 8264BD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BD54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BD58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BD60: 386A23B0  addi r3, r10, 0x23b0
	ctx.r[3].s64 = ctx.r[10].s64 + 9136;
	// 8264BD64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BD68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BD70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BD78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BD80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BD84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BD88: 4BE1B099  bl 0x82466e20
	ctx.lr = 0x8264BD8C;
	sub_82466E20(ctx, base);
	// 8264BD8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BD90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BD94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BD98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BDA0 size=108
    let mut pc: u32 = 0x8264BDA0;
    'dispatch: loop {
        match pc {
            0x8264BDA0 => {
    //   block [0x8264BDA0..0x8264BE0C)
	// 8264BDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BDA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BDAC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BDB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BDB4: 38EB3098  addi r7, r11, 0x3098
	ctx.r[7].s64 = ctx.r[11].s64 + 12440;
	// 8264BDB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BDBC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 8264BDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BDC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BDC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BDD0: 386A23E0  addi r3, r10, 0x23e0
	ctx.r[3].s64 = ctx.r[10].s64 + 9184;
	// 8264BDD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BDDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BDF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BDF8: 4BE1B029  bl 0x82466e20
	ctx.lr = 0x8264BDFC;
	sub_82466E20(ctx, base);
	// 8264BDFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BE10 size=108
    let mut pc: u32 = 0x8264BE10;
    'dispatch: loop {
        match pc {
            0x8264BE10 => {
    //   block [0x8264BE10..0x8264BE7C)
	// 8264BE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BE1C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BE20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BE24: 38EB30C8  addi r7, r11, 0x30c8
	ctx.r[7].s64 = ctx.r[11].s64 + 12488;
	// 8264BE28: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8264BE2C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 8264BE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BE34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BE38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BE40: 386A2410  addi r3, r10, 0x2410
	ctx.r[3].s64 = ctx.r[10].s64 + 9232;
	// 8264BE44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BE48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BE64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BE68: 4BE1AFB9  bl 0x82466e20
	ctx.lr = 0x8264BE6C;
	sub_82466E20(ctx, base);
	// 8264BE6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264BE80 size=24
    let mut pc: u32 = 0x8264BE80;
    'dispatch: loop {
        match pc {
            0x8264BE80 => {
    //   block [0x8264BE80..0x8264BE98)
	// 8264BE80: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BE84: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264BE88: 394A7E88  addi r10, r10, 0x7e88
	ctx.r[10].s64 = ctx.r[10].s64 + 32392;
	// 8264BE8C: 816B3188  lwz r11, 0x3188(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12680 as u32) ) } as u64;
	// 8264BE90: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8264BE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BE98 size=112
    let mut pc: u32 = 0x8264BE98;
    'dispatch: loop {
        match pc {
            0x8264BE98 => {
    //   block [0x8264BE98..0x8264BF08)
	// 8264BE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BEA4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264BEA8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BEAC: 392A9D28  addi r9, r10, -0x62d8
	ctx.r[9].s64 = ctx.r[10].s64 + -25304;
	// 8264BEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BEB4: 390B7E88  addi r8, r11, 0x7e88
	ctx.r[8].s64 = ctx.r[11].s64 + 32392;
	// 8264BEB8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8264BEBC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 8264BEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BEC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264BECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BED0: 386A2440  addi r3, r10, 0x2440
	ctx.r[3].s64 = ctx.r[10].s64 + 9280;
	// 8264BED4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264BED8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264BEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BEF4: 4BE1AF2D  bl 0x82466e20
	ctx.lr = 0x8264BEF8;
	sub_82466E20(ctx, base);
	// 8264BEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BF08 size=108
    let mut pc: u32 = 0x8264BF08;
    'dispatch: loop {
        match pc {
            0x8264BF08 => {
    //   block [0x8264BF08..0x8264BF74)
	// 8264BF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BF14: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BF1C: 38EB3190  addi r7, r11, 0x3190
	ctx.r[7].s64 = ctx.r[11].s64 + 12688;
	// 8264BF20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264BF24: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 8264BF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BF2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264BF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264BF38: 386A2470  addi r3, r10, 0x2470
	ctx.r[3].s64 = ctx.r[10].s64 + 9328;
	// 8264BF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264BF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264BF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BF60: 4BE1AEC1  bl 0x82466e20
	ctx.lr = 0x8264BF64;
	sub_82466E20(ctx, base);
	// 8264BF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264BF78 size=112
    let mut pc: u32 = 0x8264BF78;
    'dispatch: loop {
        match pc {
            0x8264BF78 => {
    //   block [0x8264BF78..0x8264BFE8)
	// 8264BF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264BF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264BF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264BF84: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264BF88: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BF8C: 392A9D6C  addi r9, r10, -0x6294
	ctx.r[9].s64 = ctx.r[10].s64 + -25236;
	// 8264BF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264BF94: 390B31C0  addi r8, r11, 0x31c0
	ctx.r[8].s64 = ctx.r[11].s64 + 12736;
	// 8264BF98: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8264BF9C: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 8264BFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264BFA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264BFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264BFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264BFB0: 386A24A0  addi r3, r10, 0x24a0
	ctx.r[3].s64 = ctx.r[10].s64 + 9376;
	// 8264BFB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264BFB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264BFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264BFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264BFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264BFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264BFCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264BFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264BFD4: 4BE1AE4D  bl 0x82466e20
	ctx.lr = 0x8264BFD8;
	sub_82466E20(ctx, base);
	// 8264BFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264BFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264BFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264BFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264BFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264BFE8 size=24
    let mut pc: u32 = 0x8264BFE8;
    'dispatch: loop {
        match pc {
            0x8264BFE8 => {
    //   block [0x8264BFE8..0x8264C000)
	// 8264BFE8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264BFEC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264BFF0: 394A7F00  addi r10, r10, 0x7f00
	ctx.r[10].s64 = ctx.r[10].s64 + 32512;
	// 8264BFF4: 816B3280  lwz r11, 0x3280(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12928 as u32) ) } as u64;
	// 8264BFF8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8264BFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C000 size=112
    let mut pc: u32 = 0x8264C000;
    'dispatch: loop {
        match pc {
            0x8264C000 => {
    //   block [0x8264C000..0x8264C070)
	// 8264C000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C00C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264C010: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C014: 392A9DA8  addi r9, r10, -0x6258
	ctx.r[9].s64 = ctx.r[10].s64 + -25176;
	// 8264C018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C01C: 390B7F00  addi r8, r11, 0x7f00
	ctx.r[8].s64 = ctx.r[11].s64 + 32512;
	// 8264C020: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8264C024: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 8264C028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C02C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264C034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C038: 386A24D0  addi r3, r10, 0x24d0
	ctx.r[3].s64 = ctx.r[10].s64 + 9424;
	// 8264C03C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264C040: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264C044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C04C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C05C: 4BE1ADC5  bl 0x82466e20
	ctx.lr = 0x8264C060;
	sub_82466E20(ctx, base);
	// 8264C060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C070 size=108
    let mut pc: u32 = 0x8264C070;
    'dispatch: loop {
        match pc {
            0x8264C070 => {
    //   block [0x8264C070..0x8264C0DC)
	// 8264C070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C07C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C084: 38EB3284  addi r7, r11, 0x3284
	ctx.r[7].s64 = ctx.r[11].s64 + 12932;
	// 8264C088: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264C08C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 8264C090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C094: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C0A0: 386A2500  addi r3, r10, 0x2500
	ctx.r[3].s64 = ctx.r[10].s64 + 9472;
	// 8264C0A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C0BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C0C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C0C8: 4BE1AD59  bl 0x82466e20
	ctx.lr = 0x8264C0CC;
	sub_82466E20(ctx, base);
	// 8264C0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C0E0 size=108
    let mut pc: u32 = 0x8264C0E0;
    'dispatch: loop {
        match pc {
            0x8264C0E0 => {
    //   block [0x8264C0E0..0x8264C14C)
	// 8264C0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C0EC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C0F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C0F4: 38EB329C  addi r7, r11, 0x329c
	ctx.r[7].s64 = ctx.r[11].s64 + 12956;
	// 8264C0F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264C0FC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8264C100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C104: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C110: 386A2530  addi r3, r10, 0x2530
	ctx.r[3].s64 = ctx.r[10].s64 + 9520;
	// 8264C114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C12C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C138: 4BE1ACE9  bl 0x82466e20
	ctx.lr = 0x8264C13C;
	sub_82466E20(ctx, base);
	// 8264C13C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8264C150 size=24
    let mut pc: u32 = 0x8264C150;
    'dispatch: loop {
        match pc {
            0x8264C150 => {
    //   block [0x8264C150..0x8264C168)
	// 8264C150: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C154: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8264C158: 394A7F48  addi r10, r10, 0x7f48
	ctx.r[10].s64 = ctx.r[10].s64 + 32584;
	// 8264C15C: 816B32CC  lwz r11, 0x32cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(13004 as u32) ) } as u64;
	// 8264C160: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8264C164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C168 size=112
    let mut pc: u32 = 0x8264C168;
    'dispatch: loop {
        match pc {
            0x8264C168 => {
    //   block [0x8264C168..0x8264C1D8)
	// 8264C168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C174: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8264C178: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C17C: 392A9DE4  addi r9, r10, -0x621c
	ctx.r[9].s64 = ctx.r[10].s64 + -25116;
	// 8264C180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C184: 390B7F48  addi r8, r11, 0x7f48
	ctx.r[8].s64 = ctx.r[11].s64 + 32584;
	// 8264C188: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8264C18C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 8264C190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C194: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8264C19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C1A0: 386A2560  addi r3, r10, 0x2560
	ctx.r[3].s64 = ctx.r[10].s64 + 9568;
	// 8264C1A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8264C1A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8264C1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C1B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C1B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C1BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C1C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C1C4: 4BE1AC5D  bl 0x82466e20
	ctx.lr = 0x8264C1C8;
	sub_82466E20(ctx, base);
	// 8264C1C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C1D8 size=108
    let mut pc: u32 = 0x8264C1D8;
    'dispatch: loop {
        match pc {
            0x8264C1D8 => {
    //   block [0x8264C1D8..0x8264C244)
	// 8264C1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C1E4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C1E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C1EC: 38EB32D0  addi r7, r11, 0x32d0
	ctx.r[7].s64 = ctx.r[11].s64 + 13008;
	// 8264C1F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8264C1F4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 8264C1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C1FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C208: 386A2590  addi r3, r10, 0x2590
	ctx.r[3].s64 = ctx.r[10].s64 + 9616;
	// 8264C20C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C230: 4BE1ABF1  bl 0x82466e20
	ctx.lr = 0x8264C234;
	sub_82466E20(ctx, base);
	// 8264C234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C248 size=108
    let mut pc: u32 = 0x8264C248;
    'dispatch: loop {
        match pc {
            0x8264C248 => {
    //   block [0x8264C248..0x8264C2B4)
	// 8264C248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C254: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C25C: 38EB32E8  addi r7, r11, 0x32e8
	ctx.r[7].s64 = ctx.r[11].s64 + 13032;
	// 8264C260: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8264C264: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 8264C268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C26C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C278: 386A25C0  addi r3, r10, 0x25c0
	ctx.r[3].s64 = ctx.r[10].s64 + 9664;
	// 8264C27C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C29C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C2A0: 4BE1AB81  bl 0x82466e20
	ctx.lr = 0x8264C2A4;
	sub_82466E20(ctx, base);
	// 8264C2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C2B8 size=108
    let mut pc: u32 = 0x8264C2B8;
    'dispatch: loop {
        match pc {
            0x8264C2B8 => {
    //   block [0x8264C2B8..0x8264C324)
	// 8264C2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C2C4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C2CC: 38EB3330  addi r7, r11, 0x3330
	ctx.r[7].s64 = ctx.r[11].s64 + 13104;
	// 8264C2D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8264C2D4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 8264C2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C2DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C2E8: 386A25F0  addi r3, r10, 0x25f0
	ctx.r[3].s64 = ctx.r[10].s64 + 9712;
	// 8264C2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C310: 4BE1AB11  bl 0x82466e20
	ctx.lr = 0x8264C314;
	sub_82466E20(ctx, base);
	// 8264C314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C328 size=108
    let mut pc: u32 = 0x8264C328;
    'dispatch: loop {
        match pc {
            0x8264C328 => {
    //   block [0x8264C328..0x8264C394)
	// 8264C328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C334: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C33C: 38EB3360  addi r7, r11, 0x3360
	ctx.r[7].s64 = ctx.r[11].s64 + 13152;
	// 8264C340: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8264C344: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 8264C348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C34C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C350: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C358: 386A2620  addi r3, r10, 0x2620
	ctx.r[3].s64 = ctx.r[10].s64 + 9760;
	// 8264C35C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C37C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C380: 4BE1AAA1  bl 0x82466e20
	ctx.lr = 0x8264C384;
	sub_82466E20(ctx, base);
	// 8264C384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C38C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C398 size=108
    let mut pc: u32 = 0x8264C398;
    'dispatch: loop {
        match pc {
            0x8264C398 => {
    //   block [0x8264C398..0x8264C404)
	// 8264C398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C3A4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C3A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C3AC: 38EB3480  addi r7, r11, 0x3480
	ctx.r[7].s64 = ctx.r[11].s64 + 13440;
	// 8264C3B0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8264C3B4: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8264C3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C3BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C3C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C3C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C3C8: 386A2650  addi r3, r10, 0x2650
	ctx.r[3].s64 = ctx.r[10].s64 + 9808;
	// 8264C3CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C3D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C3D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C3E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C3E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C3E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C3EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C3F0: 4BE1AA31  bl 0x82466e20
	ctx.lr = 0x8264C3F4;
	sub_82466E20(ctx, base);
	// 8264C3F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C3F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C3FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C408 size=108
    let mut pc: u32 = 0x8264C408;
    'dispatch: loop {
        match pc {
            0x8264C408 => {
    //   block [0x8264C408..0x8264C474)
	// 8264C408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C414: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C41C: 38EB3510  addi r7, r11, 0x3510
	ctx.r[7].s64 = ctx.r[11].s64 + 13584;
	// 8264C420: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8264C424: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8264C428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C42C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C438: 386A2680  addi r3, r10, 0x2680
	ctx.r[3].s64 = ctx.r[10].s64 + 9856;
	// 8264C43C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C45C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C460: 4BE1A9C1  bl 0x82466e20
	ctx.lr = 0x8264C464;
	sub_82466E20(ctx, base);
	// 8264C464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C46C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8264C478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8264C478 size=108
    let mut pc: u32 = 0x8264C478;
    'dispatch: loop {
        match pc {
            0x8264C478 => {
    //   block [0x8264C478..0x8264C4E4)
	// 8264C478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8264C47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8264C480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8264C484: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8264C488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8264C48C: 38EB35D0  addi r7, r11, 0x35d0
	ctx.r[7].s64 = ctx.r[11].s64 + 13776;
	// 8264C490: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8264C494: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8264C498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8264C49C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8264C4A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8264C4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8264C4A8: 386A26B0  addi r3, r10, 0x26b0
	ctx.r[3].s64 = ctx.r[10].s64 + 9904;
	// 8264C4AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8264C4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8264C4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8264C4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8264C4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8264C4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8264C4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8264C4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8264C4CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8264C4D0: 4BE1A951  bl 0x82466e20
	ctx.lr = 0x8264C4D4;
	sub_82466E20(ctx, base);
	// 8264C4D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8264C4D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8264C4DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8264C4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


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


