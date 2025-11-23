pub fn sub_8261EEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EEC8 size=112
    let mut pc: u32 = 0x8261EEC8;
    'dispatch: loop {
        match pc {
            0x8261EEC8 => {
    //   block [0x8261EEC8..0x8261EF38)
	// 8261EEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EED4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EED8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261EEDC: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 8261EEE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EEE4: 390B03A8  addi r8, r11, 0x3a8
	ctx.r[8].s64 = ctx.r[11].s64 + 936;
	// 8261EEE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261EEEC: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 8261EEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EEF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EEF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261EEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EF00: 386AF374  addi r3, r10, -0xc8c
	ctx.r[3].s64 = ctx.r[10].s64 + -3212;
	// 8261EF04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261EF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EF24: 4BE47EFD  bl 0x82466e20
	ctx.lr = 0x8261EF28;
	sub_82466E20(ctx, base);
	// 8261EF28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EF38 size=100
    let mut pc: u32 = 0x8261EF38;
    'dispatch: loop {
        match pc {
            0x8261EF38 => {
    //   block [0x8261EF38..0x8261EF9C)
	// 8261EF38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EF3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EF40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EF44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EF4C: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 8261EF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EF58: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8261EF5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261EF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EF6C: 386AF3A4  addi r3, r10, -0xc5c
	ctx.r[3].s64 = ctx.r[10].s64 + -3164;
	// 8261EF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EF74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EF78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261EF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EF80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261EF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EF88: 4BE47E99  bl 0x82466e20
	ctx.lr = 0x8261EF8C;
	sub_82466E20(ctx, base);
	// 8261EF8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261EFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261EFA0 size=96
    let mut pc: u32 = 0x8261EFA0;
    'dispatch: loop {
        match pc {
            0x8261EFA0 => {
    //   block [0x8261EFA0..0x8261F000)
	// 8261EFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261EFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261EFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261EFAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261EFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261EFB4: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 8261EFB8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261EFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261EFC0: 386AF3D4  addi r3, r10, -0xc2c
	ctx.r[3].s64 = ctx.r[10].s64 + -3116;
	// 8261EFC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261EFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261EFCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261EFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261EFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261EFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261EFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261EFE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261EFE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261EFE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261EFEC: 4BE47E35  bl 0x82466e20
	ctx.lr = 0x8261EFF0;
	sub_82466E20(ctx, base);
	// 8261EFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261EFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261EFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261EFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F000 size=112
    let mut pc: u32 = 0x8261F000;
    'dispatch: loop {
        match pc {
            0x8261F000 => {
    //   block [0x8261F000..0x8261F070)
	// 8261F000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F00C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F010: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F014: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8261F018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F01C: 390B03C0  addi r8, r11, 0x3c0
	ctx.r[8].s64 = ctx.r[11].s64 + 960;
	// 8261F020: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F024: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 8261F028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F02C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F038: 386AF404  addi r3, r10, -0xbfc
	ctx.r[3].s64 = ctx.r[10].s64 + -3068;
	// 8261F03C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F05C: 4BE47DC5  bl 0x82466e20
	ctx.lr = 0x8261F060;
	sub_82466E20(ctx, base);
	// 8261F060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F070 size=108
    let mut pc: u32 = 0x8261F070;
    'dispatch: loop {
        match pc {
            0x8261F070 => {
    //   block [0x8261F070..0x8261F0DC)
	// 8261F070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F07C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F084: 38EB03D8  addi r7, r11, 0x3d8
	ctx.r[7].s64 = ctx.r[11].s64 + 984;
	// 8261F088: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8261F08C: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8261F090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F094: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261F09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F0A0: 386AF434  addi r3, r10, -0xbcc
	ctx.r[3].s64 = ctx.r[10].s64 + -3020;
	// 8261F0A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261F0A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F0B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F0BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F0C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261F0C8: 4BE47D59  bl 0x82466e20
	ctx.lr = 0x8261F0CC;
	sub_82466E20(ctx, base);
	// 8261F0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F0E0 size=112
    let mut pc: u32 = 0x8261F0E0;
    'dispatch: loop {
        match pc {
            0x8261F0E0 => {
    //   block [0x8261F0E0..0x8261F150)
	// 8261F0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F0EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F0F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F0F4: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F0F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F0FC: 390B0438  addi r8, r11, 0x438
	ctx.r[8].s64 = ctx.r[11].s64 + 1080;
	// 8261F100: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F104: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8261F108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F10C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F118: 386AF464  addi r3, r10, -0xb9c
	ctx.r[3].s64 = ctx.r[10].s64 + -2972;
	// 8261F11C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F12C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F13C: 4BE47CE5  bl 0x82466e20
	ctx.lr = 0x8261F140;
	sub_82466E20(ctx, base);
	// 8261F140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F150 size=112
    let mut pc: u32 = 0x8261F150;
    'dispatch: loop {
        match pc {
            0x8261F150 => {
    //   block [0x8261F150..0x8261F1C0)
	// 8261F150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F15C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F160: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F164: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261F168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F16C: 390B0450  addi r8, r11, 0x450
	ctx.r[8].s64 = ctx.r[11].s64 + 1104;
	// 8261F170: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261F174: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8261F178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F17C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F188: 386AF494  addi r3, r10, -0xb6c
	ctx.r[3].s64 = ctx.r[10].s64 + -2924;
	// 8261F18C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F19C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F1AC: 4BE47C75  bl 0x82466e20
	ctx.lr = 0x8261F1B0;
	sub_82466E20(ctx, base);
	// 8261F1B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F1B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F1B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F1C0 size=112
    let mut pc: u32 = 0x8261F1C0;
    'dispatch: loop {
        match pc {
            0x8261F1C0 => {
    //   block [0x8261F1C0..0x8261F230)
	// 8261F1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F1CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F1D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F1D4: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261F1D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F1DC: 390B0480  addi r8, r11, 0x480
	ctx.r[8].s64 = ctx.r[11].s64 + 1152;
	// 8261F1E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F1E4: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8261F1E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F1EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F1F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F1F8: 386AF4C4  addi r3, r10, -0xb3c
	ctx.r[3].s64 = ctx.r[10].s64 + -2876;
	// 8261F1FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F20C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F21C: 4BE47C05  bl 0x82466e20
	ctx.lr = 0x8261F220;
	sub_82466E20(ctx, base);
	// 8261F220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F230 size=112
    let mut pc: u32 = 0x8261F230;
    'dispatch: loop {
        match pc {
            0x8261F230 => {
    //   block [0x8261F230..0x8261F2A0)
	// 8261F230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F23C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F240: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F244: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F248: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F24C: 390B0498  addi r8, r11, 0x498
	ctx.r[8].s64 = ctx.r[11].s64 + 1176;
	// 8261F250: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261F254: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8261F258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F25C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F260: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F268: 386AF4F4  addi r3, r10, -0xb0c
	ctx.r[3].s64 = ctx.r[10].s64 + -2828;
	// 8261F26C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F278: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F27C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F280: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F288: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F28C: 4BE47B95  bl 0x82466e20
	ctx.lr = 0x8261F290;
	sub_82466E20(ctx, base);
	// 8261F290: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F29C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F2A0 size=112
    let mut pc: u32 = 0x8261F2A0;
    'dispatch: loop {
        match pc {
            0x8261F2A0 => {
    //   block [0x8261F2A0..0x8261F310)
	// 8261F2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F2AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F2B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F2B4: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261F2B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F2BC: 390B04C8  addi r8, r11, 0x4c8
	ctx.r[8].s64 = ctx.r[11].s64 + 1224;
	// 8261F2C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F2C4: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8261F2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F2CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F2D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F2D8: 386AF524  addi r3, r10, -0xadc
	ctx.r[3].s64 = ctx.r[10].s64 + -2780;
	// 8261F2DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F2E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F2E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F2EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F2F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F2F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F2FC: 4BE47B25  bl 0x82466e20
	ctx.lr = 0x8261F300;
	sub_82466E20(ctx, base);
	// 8261F300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F310 size=112
    let mut pc: u32 = 0x8261F310;
    'dispatch: loop {
        match pc {
            0x8261F310 => {
    //   block [0x8261F310..0x8261F380)
	// 8261F310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F31C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F320: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F324: 38AAFA04  addi r5, r10, -0x5fc
	ctx.r[5].s64 = ctx.r[10].s64 + -1532;
	// 8261F328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F32C: 390B04E0  addi r8, r11, 0x4e0
	ctx.r[8].s64 = ctx.r[11].s64 + 1248;
	// 8261F330: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F334: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8261F338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F33C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F348: 386AF554  addi r3, r10, -0xaac
	ctx.r[3].s64 = ctx.r[10].s64 + -2732;
	// 8261F34C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F35C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F36C: 4BE47AB5  bl 0x82466e20
	ctx.lr = 0x8261F370;
	sub_82466E20(ctx, base);
	// 8261F370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F380 size=112
    let mut pc: u32 = 0x8261F380;
    'dispatch: loop {
        match pc {
            0x8261F380 => {
    //   block [0x8261F380..0x8261F3F0)
	// 8261F380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F38C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F390: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F394: 38AAF764  addi r5, r10, -0x89c
	ctx.r[5].s64 = ctx.r[10].s64 + -2204;
	// 8261F398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F39C: 390B04F8  addi r8, r11, 0x4f8
	ctx.r[8].s64 = ctx.r[11].s64 + 1272;
	// 8261F3A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F3A4: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8261F3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F3AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F3B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F3B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F3B8: 386AF584  addi r3, r10, -0xa7c
	ctx.r[3].s64 = ctx.r[10].s64 + -2684;
	// 8261F3BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F3C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F3C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F3C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F3D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F3D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F3DC: 4BE47A45  bl 0x82466e20
	ctx.lr = 0x8261F3E0;
	sub_82466E20(ctx, base);
	// 8261F3E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F3E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F3E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F3EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F3F0 size=112
    let mut pc: u32 = 0x8261F3F0;
    'dispatch: loop {
        match pc {
            0x8261F3F0 => {
    //   block [0x8261F3F0..0x8261F460)
	// 8261F3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F3FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F400: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F404: 38AAF524  addi r5, r10, -0xadc
	ctx.r[5].s64 = ctx.r[10].s64 + -2780;
	// 8261F408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F40C: 390B0510  addi r8, r11, 0x510
	ctx.r[8].s64 = ctx.r[11].s64 + 1296;
	// 8261F410: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261F414: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8261F418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F41C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F420: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F428: 386AF5B4  addi r3, r10, -0xa4c
	ctx.r[3].s64 = ctx.r[10].s64 + -2636;
	// 8261F42C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F43C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F44C: 4BE479D5  bl 0x82466e20
	ctx.lr = 0x8261F450;
	sub_82466E20(ctx, base);
	// 8261F450: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F45C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F460 size=112
    let mut pc: u32 = 0x8261F460;
    'dispatch: loop {
        match pc {
            0x8261F460 => {
    //   block [0x8261F460..0x8261F4D0)
	// 8261F460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F46C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F470: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F474: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F47C: 390B0558  addi r8, r11, 0x558
	ctx.r[8].s64 = ctx.r[11].s64 + 1368;
	// 8261F480: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261F484: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8261F488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F48C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F498: 386AF5E4  addi r3, r10, -0xa1c
	ctx.r[3].s64 = ctx.r[10].s64 + -2588;
	// 8261F49C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F4A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F4A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F4B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F4B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F4B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F4BC: 4BE47965  bl 0x82466e20
	ctx.lr = 0x8261F4C0;
	sub_82466E20(ctx, base);
	// 8261F4C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F4D0 size=112
    let mut pc: u32 = 0x8261F4D0;
    'dispatch: loop {
        match pc {
            0x8261F4D0 => {
    //   block [0x8261F4D0..0x8261F540)
	// 8261F4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F4DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F4E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F4E4: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F4E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F4EC: 390B0588  addi r8, r11, 0x588
	ctx.r[8].s64 = ctx.r[11].s64 + 1416;
	// 8261F4F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261F4F4: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8261F4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F4FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F508: 386AF614  addi r3, r10, -0x9ec
	ctx.r[3].s64 = ctx.r[10].s64 + -2540;
	// 8261F50C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F52C: 4BE478F5  bl 0x82466e20
	ctx.lr = 0x8261F530;
	sub_82466E20(ctx, base);
	// 8261F530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F540 size=108
    let mut pc: u32 = 0x8261F540;
    'dispatch: loop {
        match pc {
            0x8261F540 => {
    //   block [0x8261F540..0x8261F5AC)
	// 8261F540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F54C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F554: 38EB05B8  addi r7, r11, 0x5b8
	ctx.r[7].s64 = ctx.r[11].s64 + 1464;
	// 8261F558: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261F55C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8261F560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261F56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F570: 386AF644  addi r3, r10, -0x9bc
	ctx.r[3].s64 = ctx.r[10].s64 + -2492;
	// 8261F574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261F578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261F598: 4BE47889  bl 0x82466e20
	ctx.lr = 0x8261F59C;
	sub_82466E20(ctx, base);
	// 8261F59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F5B0 size=112
    let mut pc: u32 = 0x8261F5B0;
    'dispatch: loop {
        match pc {
            0x8261F5B0 => {
    //   block [0x8261F5B0..0x8261F620)
	// 8261F5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F5BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F5C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F5C4: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F5CC: 390B0600  addi r8, r11, 0x600
	ctx.r[8].s64 = ctx.r[11].s64 + 1536;
	// 8261F5D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8261F5D4: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8261F5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F5DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F5E8: 386AF674  addi r3, r10, -0x98c
	ctx.r[3].s64 = ctx.r[10].s64 + -2444;
	// 8261F5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F60C: 4BE47815  bl 0x82466e20
	ctx.lr = 0x8261F610;
	sub_82466E20(ctx, base);
	// 8261F610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F620 size=116
    let mut pc: u32 = 0x8261F620;
    'dispatch: loop {
        match pc {
            0x8261F620 => {
    //   block [0x8261F620..0x8261F694)
	// 8261F620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F62C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261F634: 390B0680  addi r8, r11, 0x680
	ctx.r[8].s64 = ctx.r[11].s64 + 1664;
	// 8261F638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F63C: 392A1810  addi r9, r10, 0x1810
	ctx.r[9].s64 = ctx.r[10].s64 + 6160;
	// 8261F640: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F644: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8261F648: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261F64C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F654: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F664: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261F668: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8261F66C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261F670: 386BF6A4  addi r3, r11, -0x95c
	ctx.r[3].s64 = ctx.r[11].s64 + -2396;
	// 8261F674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261F678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F680: 4BE477A1  bl 0x82466e20
	ctx.lr = 0x8261F684;
	sub_82466E20(ctx, base);
	// 8261F684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F698 size=100
    let mut pc: u32 = 0x8261F698;
    'dispatch: loop {
        match pc {
            0x8261F698 => {
    //   block [0x8261F698..0x8261F6FC)
	// 8261F698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F6A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F6AC: 38AAF7F4  addi r5, r10, -0x80c
	ctx.r[5].s64 = ctx.r[10].s64 + -2060;
	// 8261F6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F6B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F6B8: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8261F6BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F6CC: 386AF6D4  addi r3, r10, -0x92c
	ctx.r[3].s64 = ctx.r[10].s64 + -2348;
	// 8261F6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F6D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F6D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261F6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F6E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261F6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F6E8: 4BE47739  bl 0x82466e20
	ctx.lr = 0x8261F6EC;
	sub_82466E20(ctx, base);
	// 8261F6EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F6F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F6F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F6F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F700 size=100
    let mut pc: u32 = 0x8261F700;
    'dispatch: loop {
        match pc {
            0x8261F700 => {
    //   block [0x8261F700..0x8261F764)
	// 8261F700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F70C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F714: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261F718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F71C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F720: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8261F724: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F734: 386AF704  addi r3, r10, -0x8fc
	ctx.r[3].s64 = ctx.r[10].s64 + -2300;
	// 8261F738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F73C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261F744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261F74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F750: 4BE476D1  bl 0x82466e20
	ctx.lr = 0x8261F754;
	sub_82466E20(ctx, base);
	// 8261F754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F768 size=108
    let mut pc: u32 = 0x8261F768;
    'dispatch: loop {
        match pc {
            0x8261F768 => {
    //   block [0x8261F768..0x8261F7D4)
	// 8261F768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F774: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F77C: 38EB06F8  addi r7, r11, 0x6f8
	ctx.r[7].s64 = ctx.r[11].s64 + 1784;
	// 8261F780: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261F784: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8261F788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F78C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F790: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261F794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F798: 386AF734  addi r3, r10, -0x8cc
	ctx.r[3].s64 = ctx.r[10].s64 + -2252;
	// 8261F79C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261F7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F7BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261F7C0: 4BE47661  bl 0x82466e20
	ctx.lr = 0x8261F7C4;
	sub_82466E20(ctx, base);
	// 8261F7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F7D8 size=112
    let mut pc: u32 = 0x8261F7D8;
    'dispatch: loop {
        match pc {
            0x8261F7D8 => {
    //   block [0x8261F7D8..0x8261F848)
	// 8261F7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F7E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F7E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F7EC: 38AAF524  addi r5, r10, -0xadc
	ctx.r[5].s64 = ctx.r[10].s64 + -2780;
	// 8261F7F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F7F4: 390B0728  addi r8, r11, 0x728
	ctx.r[8].s64 = ctx.r[11].s64 + 1832;
	// 8261F7F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F7FC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8261F800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F804: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F810: 386AF764  addi r3, r10, -0x89c
	ctx.r[3].s64 = ctx.r[10].s64 + -2204;
	// 8261F814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261F818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F834: 4BE475ED  bl 0x82466e20
	ctx.lr = 0x8261F838;
	sub_82466E20(ctx, base);
	// 8261F838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F83C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F848 size=108
    let mut pc: u32 = 0x8261F848;
    'dispatch: loop {
        match pc {
            0x8261F848 => {
    //   block [0x8261F848..0x8261F8B4)
	// 8261F848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F854: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F85C: 38EB0740  addi r7, r11, 0x740
	ctx.r[7].s64 = ctx.r[11].s64 + 1856;
	// 8261F860: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261F864: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8261F868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F86C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F870: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261F874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F878: 386AF794  addi r3, r10, -0x86c
	ctx.r[3].s64 = ctx.r[10].s64 + -2156;
	// 8261F87C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261F880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F89C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261F8A0: 4BE47581  bl 0x82466e20
	ctx.lr = 0x8261F8A4;
	sub_82466E20(ctx, base);
	// 8261F8A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F8A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F8AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F8B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261F8B8 size=28
    let mut pc: u32 = 0x8261F8B8;
    'dispatch: loop {
        match pc {
            0x8261F8B8 => {
    //   block [0x8261F8B8..0x8261F8D4)
	// 8261F8B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F8BC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261F8C0: 394A3CF0  addi r10, r10, 0x3cf0
	ctx.r[10].s64 = ctx.r[10].s64 + 15600;
	// 8261F8C4: 816B067C  lwz r11, 0x67c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1660 as u32) ) } as u64;
	// 8261F8C8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8261F8CC: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8261F8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F8D8 size=108
    let mut pc: u32 = 0x8261F8D8;
    'dispatch: loop {
        match pc {
            0x8261F8D8 => {
    //   block [0x8261F8D8..0x8261F944)
	// 8261F8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F8E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F8E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F8EC: 38EB3CF0  addi r7, r11, 0x3cf0
	ctx.r[7].s64 = ctx.r[11].s64 + 15600;
	// 8261F8F0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8261F8F4: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8261F8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F8FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261F904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261F908: 386AF7C4  addi r3, r10, -0x83c
	ctx.r[3].s64 = ctx.r[10].s64 + -2108;
	// 8261F90C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261F910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261F914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F92C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261F930: 4BE474F1  bl 0x82466e20
	ctx.lr = 0x8261F934;
	sub_82466E20(ctx, base);
	// 8261F934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F948 size=116
    let mut pc: u32 = 0x8261F948;
    'dispatch: loop {
        match pc {
            0x8261F948 => {
    //   block [0x8261F948..0x8261F9BC)
	// 8261F948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F954: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261F95C: 390B0760  addi r8, r11, 0x760
	ctx.r[8].s64 = ctx.r[11].s64 + 1888;
	// 8261F960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F964: 392A1864  addi r9, r10, 0x1864
	ctx.r[9].s64 = ctx.r[10].s64 + 6244;
	// 8261F968: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F96C: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8261F970: 38AAF524  addi r5, r10, -0xadc
	ctx.r[5].s64 = ctx.r[10].s64 + -2780;
	// 8261F974: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261F97C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261F984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261F988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261F98C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261F990: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8261F994: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261F998: 386BF7F4  addi r3, r11, -0x80c
	ctx.r[3].s64 = ctx.r[11].s64 + -2060;
	// 8261F99C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8261F9A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261F9A8: 4BE47479  bl 0x82466e20
	ctx.lr = 0x8261F9AC;
	sub_82466E20(ctx, base);
	// 8261F9AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261F9B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261F9B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261F9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261F9C0 size=112
    let mut pc: u32 = 0x8261F9C0;
    'dispatch: loop {
        match pc {
            0x8261F9C0 => {
    //   block [0x8261F9C0..0x8261FA30)
	// 8261F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261F9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261F9CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F9D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261F9D4: 38AAF4C4  addi r5, r10, -0xb3c
	ctx.r[5].s64 = ctx.r[10].s64 + -2876;
	// 8261F9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261F9DC: 390B07C0  addi r8, r11, 0x7c0
	ctx.r[8].s64 = ctx.r[11].s64 + 1984;
	// 8261F9E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261F9E4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8261F9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261F9EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261F9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261F9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261F9F8: 386AF824  addi r3, r10, -0x7dc
	ctx.r[3].s64 = ctx.r[10].s64 + -2012;
	// 8261F9FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FA1C: 4BE47405  bl 0x82466e20
	ctx.lr = 0x8261FA20;
	sub_82466E20(ctx, base);
	// 8261FA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FA30 size=108
    let mut pc: u32 = 0x8261FA30;
    'dispatch: loop {
        match pc {
            0x8261FA30 => {
    //   block [0x8261FA30..0x8261FA9C)
	// 8261FA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FA3C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FA40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FA44: 38EB07D8  addi r7, r11, 0x7d8
	ctx.r[7].s64 = ctx.r[11].s64 + 2008;
	// 8261FA48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261FA4C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8261FA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FA54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FA58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261FA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FA60: 386AF854  addi r3, r10, -0x7ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1964;
	// 8261FA64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261FA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FA84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261FA88: 4BE47399  bl 0x82466e20
	ctx.lr = 0x8261FA8C;
	sub_82466E20(ctx, base);
	// 8261FA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FAA0 size=112
    let mut pc: u32 = 0x8261FAA0;
    'dispatch: loop {
        match pc {
            0x8261FAA0 => {
    //   block [0x8261FAA0..0x8261FB10)
	// 8261FAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FAAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FAB0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FAB4: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261FAB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FABC: 390B0808  addi r8, r11, 0x808
	ctx.r[8].s64 = ctx.r[11].s64 + 2056;
	// 8261FAC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261FAC4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8261FAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FACC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FAD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FAD8: 386AF884  addi r3, r10, -0x77c
	ctx.r[3].s64 = ctx.r[10].s64 + -1916;
	// 8261FADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FAF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FAFC: 4BE47325  bl 0x82466e20
	ctx.lr = 0x8261FB00;
	sub_82466E20(ctx, base);
	// 8261FB00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FB10 size=112
    let mut pc: u32 = 0x8261FB10;
    'dispatch: loop {
        match pc {
            0x8261FB10 => {
    //   block [0x8261FB10..0x8261FB80)
	// 8261FB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FB1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FB20: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FB24: 38AAFA04  addi r5, r10, -0x5fc
	ctx.r[5].s64 = ctx.r[10].s64 + -1532;
	// 8261FB28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FB2C: 390B0838  addi r8, r11, 0x838
	ctx.r[8].s64 = ctx.r[11].s64 + 2104;
	// 8261FB30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261FB34: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8261FB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FB3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FB40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FB48: 386AF8B4  addi r3, r10, -0x74c
	ctx.r[3].s64 = ctx.r[10].s64 + -1868;
	// 8261FB4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FB6C: 4BE472B5  bl 0x82466e20
	ctx.lr = 0x8261FB70;
	sub_82466E20(ctx, base);
	// 8261FB70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FB80 size=100
    let mut pc: u32 = 0x8261FB80;
    'dispatch: loop {
        match pc {
            0x8261FB80 => {
    //   block [0x8261FB80..0x8261FBE4)
	// 8261FB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FB8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FB94: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261FB98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FBA0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8261FBA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FBB4: 386AF8E4  addi r3, r10, -0x71c
	ctx.r[3].s64 = ctx.r[10].s64 + -1820;
	// 8261FBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FBBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FBC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261FBC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FBC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261FBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FBD0: 4BE47251  bl 0x82466e20
	ctx.lr = 0x8261FBD4;
	sub_82466E20(ctx, base);
	// 8261FBD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FBE8 size=112
    let mut pc: u32 = 0x8261FBE8;
    'dispatch: loop {
        match pc {
            0x8261FBE8 => {
    //   block [0x8261FBE8..0x8261FC58)
	// 8261FBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FBF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FBF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FBFC: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 8261FC00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FC04: 390B0868  addi r8, r11, 0x868
	ctx.r[8].s64 = ctx.r[11].s64 + 2152;
	// 8261FC08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261FC0C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8261FC10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FC14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FC18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FC20: 386AF914  addi r3, r10, -0x6ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1772;
	// 8261FC24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FC28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FC30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FC34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FC38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FC3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FC40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FC44: 4BE471DD  bl 0x82466e20
	ctx.lr = 0x8261FC48;
	sub_82466E20(ctx, base);
	// 8261FC48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FC54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FC58 size=112
    let mut pc: u32 = 0x8261FC58;
    'dispatch: loop {
        match pc {
            0x8261FC58 => {
    //   block [0x8261FC58..0x8261FCC8)
	// 8261FC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FC64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FC68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FC6C: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 8261FC70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FC74: 390B08B0  addi r8, r11, 0x8b0
	ctx.r[8].s64 = ctx.r[11].s64 + 2224;
	// 8261FC78: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261FC7C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8261FC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FC84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FC88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FC8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FC90: 386AF944  addi r3, r10, -0x6bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1724;
	// 8261FC94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FC98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FC9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FCA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FCA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FCAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FCB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FCB4: 4BE4716D  bl 0x82466e20
	ctx.lr = 0x8261FCB8;
	sub_82466E20(ctx, base);
	// 8261FCB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FCC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FCC8 size=108
    let mut pc: u32 = 0x8261FCC8;
    'dispatch: loop {
        match pc {
            0x8261FCC8 => {
    //   block [0x8261FCC8..0x8261FD34)
	// 8261FCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FCD4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FCD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FCDC: 38EB0958  addi r7, r11, 0x958
	ctx.r[7].s64 = ctx.r[11].s64 + 2392;
	// 8261FCE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261FCE4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8261FCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FCEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FCF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261FCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FCF8: 386AF974  addi r3, r10, -0x68c
	ctx.r[3].s64 = ctx.r[10].s64 + -1676;
	// 8261FCFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261FD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261FD20: 4BE47101  bl 0x82466e20
	ctx.lr = 0x8261FD24;
	sub_82466E20(ctx, base);
	// 8261FD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FD38 size=112
    let mut pc: u32 = 0x8261FD38;
    'dispatch: loop {
        match pc {
            0x8261FD38 => {
    //   block [0x8261FD38..0x8261FDA8)
	// 8261FD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FD44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FD48: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FD4C: 38AAF524  addi r5, r10, -0xadc
	ctx.r[5].s64 = ctx.r[10].s64 + -2780;
	// 8261FD50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FD54: 390B09A0  addi r8, r11, 0x9a0
	ctx.r[8].s64 = ctx.r[11].s64 + 2464;
	// 8261FD58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261FD5C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8261FD60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FD64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FD68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FD70: 386AF9A4  addi r3, r10, -0x65c
	ctx.r[3].s64 = ctx.r[10].s64 + -1628;
	// 8261FD74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FD78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FD7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FD80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FD88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FD8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FD90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FD94: 4BE4708D  bl 0x82466e20
	ctx.lr = 0x8261FD98;
	sub_82466E20(ctx, base);
	// 8261FD98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FDA8 size=100
    let mut pc: u32 = 0x8261FDA8;
    'dispatch: loop {
        match pc {
            0x8261FDA8 => {
    //   block [0x8261FDA8..0x8261FE0C)
	// 8261FDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FDB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FDBC: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 8261FDC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FDC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FDC8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8261FDCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FDDC: 386AF9D4  addi r3, r10, -0x62c
	ctx.r[3].s64 = ctx.r[10].s64 + -1580;
	// 8261FDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FDE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FDE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261FDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FDF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261FDF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FDF8: 4BE47029  bl 0x82466e20
	ctx.lr = 0x8261FDFC;
	sub_82466E20(ctx, base);
	// 8261FDFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FE10 size=100
    let mut pc: u32 = 0x8261FE10;
    'dispatch: loop {
        match pc {
            0x8261FE10 => {
    //   block [0x8261FE10..0x8261FE74)
	// 8261FE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FE1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FE24: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261FE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FE30: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8261FE34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FE44: 386AFA04  addi r3, r10, -0x5fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1532;
	// 8261FE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FE4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FE50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261FE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FE58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261FE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FE60: 4BE46FC1  bl 0x82466e20
	ctx.lr = 0x8261FE64;
	sub_82466E20(ctx, base);
	// 8261FE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FE78 size=108
    let mut pc: u32 = 0x8261FE78;
    'dispatch: loop {
        match pc {
            0x8261FE78 => {
    //   block [0x8261FE78..0x8261FEE4)
	// 8261FE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FE84: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FE88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FE8C: 38EB0A00  addi r7, r11, 0xa00
	ctx.r[7].s64 = ctx.r[11].s64 + 2560;
	// 8261FE90: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8261FE94: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8261FE98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FE9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FEA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261FEA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FEA8: 386AFA34  addi r3, r10, -0x5cc
	ctx.r[3].s64 = ctx.r[10].s64 + -1484;
	// 8261FEAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261FEB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FEB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FEC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FEC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FEC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261FED0: 4BE46F51  bl 0x82466e20
	ctx.lr = 0x8261FED4;
	sub_82466E20(ctx, base);
	// 8261FED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FEDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FEE8 size=112
    let mut pc: u32 = 0x8261FEE8;
    'dispatch: loop {
        match pc {
            0x8261FEE8 => {
    //   block [0x8261FEE8..0x8261FF58)
	// 8261FEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FEF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FEF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FEFC: 38AAF7F4  addi r5, r10, -0x80c
	ctx.r[5].s64 = ctx.r[10].s64 + -2060;
	// 8261FF00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FF04: 390B0A90  addi r8, r11, 0xa90
	ctx.r[8].s64 = ctx.r[11].s64 + 2704;
	// 8261FF08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261FF0C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8261FF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FF14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FF18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FF20: 386AFA64  addi r3, r10, -0x59c
	ctx.r[3].s64 = ctx.r[10].s64 + -1436;
	// 8261FF24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FF28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FF2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FF3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FF44: 4BE46EDD  bl 0x82466e20
	ctx.lr = 0x8261FF48;
	sub_82466E20(ctx, base);
	// 8261FF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FF58 size=112
    let mut pc: u32 = 0x8261FF58;
    'dispatch: loop {
        match pc {
            0x8261FF58 => {
    //   block [0x8261FF58..0x8261FFC8)
	// 8261FF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FF64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FF68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FF6C: 38AAF944  addi r5, r10, -0x6bc
	ctx.r[5].s64 = ctx.r[10].s64 + -1724;
	// 8261FF70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FF74: 390B0AA8  addi r8, r11, 0xaa8
	ctx.r[8].s64 = ctx.r[11].s64 + 2728;
	// 8261FF78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261FF7C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8261FF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FF84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FF88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261FF90: 386AFA94  addi r3, r10, -0x56c
	ctx.r[3].s64 = ctx.r[10].s64 + -1388;
	// 8261FF94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261FF98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261FF9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261FFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261FFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261FFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261FFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261FFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261FFB4: 4BE46E6D  bl 0x82466e20
	ctx.lr = 0x8261FFB8;
	sub_82466E20(ctx, base);
	// 8261FFB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261FFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261FFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261FFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261FFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261FFC8 size=112
    let mut pc: u32 = 0x8261FFC8;
    'dispatch: loop {
        match pc {
            0x8261FFC8 => {
    //   block [0x8261FFC8..0x82620038)
	// 8261FFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261FFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261FFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261FFD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FFD8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261FFDC: 38AAF404  addi r5, r10, -0xbfc
	ctx.r[5].s64 = ctx.r[10].s64 + -3068;
	// 8261FFE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261FFE4: 390B0AD8  addi r8, r11, 0xad8
	ctx.r[8].s64 = ctx.r[11].s64 + 2776;
	// 8261FFE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261FFEC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8261FFF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261FFF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261FFF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261FFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620000: 386AFAC4  addi r3, r10, -0x53c
	ctx.r[3].s64 = ctx.r[10].s64 + -1340;
	// 82620004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262000C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262001C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620024: 4BE46DFD  bl 0x82466e20
	ctx.lr = 0x82620028;
	sub_82466E20(ctx, base);
	// 82620028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262002C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620038 size=112
    let mut pc: u32 = 0x82620038;
    'dispatch: loop {
        match pc {
            0x82620038 => {
    //   block [0x82620038..0x826200A8)
	// 82620038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262003C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620044: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620048: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262004C: 38AAF554  addi r5, r10, -0xaac
	ctx.r[5].s64 = ctx.r[10].s64 + -2732;
	// 82620050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620054: 390B0B20  addi r8, r11, 0xb20
	ctx.r[8].s64 = ctx.r[11].s64 + 2848;
	// 82620058: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262005C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82620060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620064: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262006C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620070: 386AFAF4  addi r3, r10, -0x50c
	ctx.r[3].s64 = ctx.r[10].s64 + -1292;
	// 82620074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262007C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262008C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620094: 4BE46D8D  bl 0x82466e20
	ctx.lr = 0x82620098;
	sub_82466E20(ctx, base);
	// 82620098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262009C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826200A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826200A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826200A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826200A8 size=108
    let mut pc: u32 = 0x826200A8;
    'dispatch: loop {
        match pc {
            0x826200A8 => {
    //   block [0x826200A8..0x82620114)
	// 826200A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826200AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826200B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826200B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826200B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826200BC: 38EB0B68  addi r7, r11, 0xb68
	ctx.r[7].s64 = ctx.r[11].s64 + 2920;
	// 826200C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826200C4: 388A21D8  addi r4, r10, 0x21d8
	ctx.r[4].s64 = ctx.r[10].s64 + 8664;
	// 826200C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826200CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826200D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826200D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826200D8: 386AFB24  addi r3, r10, -0x4dc
	ctx.r[3].s64 = ctx.r[10].s64 + -1244;
	// 826200DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826200E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826200E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826200E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826200EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826200F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826200F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826200F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826200FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620100: 4BE46D21  bl 0x82466e20
	ctx.lr = 0x82620104;
	sub_82466E20(ctx, base);
	// 82620104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262010C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620118 size=112
    let mut pc: u32 = 0x82620118;
    'dispatch: loop {
        match pc {
            0x82620118 => {
    //   block [0x82620118..0x82620188)
	// 82620118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262011C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620124: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620128: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262012C: 38AAF4C4  addi r5, r10, -0xb3c
	ctx.r[5].s64 = ctx.r[10].s64 + -2876;
	// 82620130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620134: 390B0BB0  addi r8, r11, 0xbb0
	ctx.r[8].s64 = ctx.r[11].s64 + 2992;
	// 82620138: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262013C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82620140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620144: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262014C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620150: 386AFB54  addi r3, r10, -0x4ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1196;
	// 82620154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262015C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262016C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620174: 4BE46CAD  bl 0x82466e20
	ctx.lr = 0x82620178;
	sub_82466E20(ctx, base);
	// 82620178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262017C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620188 size=112
    let mut pc: u32 = 0x82620188;
    'dispatch: loop {
        match pc {
            0x82620188 => {
    //   block [0x82620188..0x826201F8)
	// 82620188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262018C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620194: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620198: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262019C: 38AAF524  addi r5, r10, -0xadc
	ctx.r[5].s64 = ctx.r[10].s64 + -2780;
	// 826201A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826201A4: 390B0BC8  addi r8, r11, 0xbc8
	ctx.r[8].s64 = ctx.r[11].s64 + 3016;
	// 826201A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826201AC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826201B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826201B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826201B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826201BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826201C0: 386AFB84  addi r3, r10, -0x47c
	ctx.r[3].s64 = ctx.r[10].s64 + -1148;
	// 826201C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826201C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826201CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826201D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826201D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826201D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826201DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826201E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826201E4: 4BE46C3D  bl 0x82466e20
	ctx.lr = 0x826201E8;
	sub_82466E20(ctx, base);
	// 826201E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826201EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826201F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826201F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826201F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826201F8 size=112
    let mut pc: u32 = 0x826201F8;
    'dispatch: loop {
        match pc {
            0x826201F8 => {
    //   block [0x826201F8..0x82620268)
	// 826201F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826201FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620204: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620208: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262020C: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82620210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620214: 390B0BF8  addi r8, r11, 0xbf8
	ctx.r[8].s64 = ctx.r[11].s64 + 3064;
	// 82620218: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262021C: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 82620220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262022C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620230: 386AFBB4  addi r3, r10, -0x44c
	ctx.r[3].s64 = ctx.r[10].s64 + -1100;
	// 82620234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262023C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262024C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620254: 4BE46BCD  bl 0x82466e20
	ctx.lr = 0x82620258;
	sub_82466E20(ctx, base);
	// 82620258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262025C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620268 size=112
    let mut pc: u32 = 0x82620268;
    'dispatch: loop {
        match pc {
            0x82620268 => {
    //   block [0x82620268..0x826202D8)
	// 82620268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262026C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620278: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262027C: 38AAFBB4  addi r5, r10, -0x44c
	ctx.r[5].s64 = ctx.r[10].s64 + -1100;
	// 82620280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620284: 390B0C58  addi r8, r11, 0xc58
	ctx.r[8].s64 = ctx.r[11].s64 + 3160;
	// 82620288: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262028C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82620290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262029C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826202A0: 386AFBE4  addi r3, r10, -0x41c
	ctx.r[3].s64 = ctx.r[10].s64 + -1052;
	// 826202A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826202A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826202AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826202B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826202B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826202B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826202BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826202C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826202C4: 4BE46B5D  bl 0x82466e20
	ctx.lr = 0x826202C8;
	sub_82466E20(ctx, base);
	// 826202C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826202CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826202D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826202D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826202D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826202D8 size=112
    let mut pc: u32 = 0x826202D8;
    'dispatch: loop {
        match pc {
            0x826202D8 => {
    //   block [0x826202D8..0x82620348)
	// 826202D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826202DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826202E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826202E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826202E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826202EC: 38AAFBB4  addi r5, r10, -0x44c
	ctx.r[5].s64 = ctx.r[10].s64 + -1100;
	// 826202F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826202F4: 390B0C70  addi r8, r11, 0xc70
	ctx.r[8].s64 = ctx.r[11].s64 + 3184;
	// 826202F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826202FC: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 82620300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620304: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262030C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620310: 386AFC14  addi r3, r10, -0x3ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1004;
	// 82620314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262031C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262032C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620334: 4BE46AED  bl 0x82466e20
	ctx.lr = 0x82620338;
	sub_82466E20(ctx, base);
	// 82620338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262033C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620348 size=112
    let mut pc: u32 = 0x82620348;
    'dispatch: loop {
        match pc {
            0x82620348 => {
    //   block [0x82620348..0x826203B8)
	// 82620348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262034C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620354: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620358: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262035C: 38AAFBB4  addi r5, r10, -0x44c
	ctx.r[5].s64 = ctx.r[10].s64 + -1100;
	// 82620360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620364: 390B0CA0  addi r8, r11, 0xca0
	ctx.r[8].s64 = ctx.r[11].s64 + 3232;
	// 82620368: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262036C: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82620370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620374: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262037C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620380: 386AFC44  addi r3, r10, -0x3bc
	ctx.r[3].s64 = ctx.r[10].s64 + -956;
	// 82620384: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262038C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262039C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826203A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826203A4: 4BE46A7D  bl 0x82466e20
	ctx.lr = 0x826203A8;
	sub_82466E20(ctx, base);
	// 826203A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826203AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826203B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826203B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826203B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826203B8 size=24
    let mut pc: u32 = 0x826203B8;
    'dispatch: loop {
        match pc {
            0x826203B8 => {
    //   block [0x826203B8..0x826203D0)
	// 826203B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826203BC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826203C0: 394A3E28  addi r10, r10, 0x3e28
	ctx.r[10].s64 = ctx.r[10].s64 + 15912;
	// 826203C4: 816B075C  lwz r11, 0x75c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1884 as u32) ) } as u64;
	// 826203C8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826203CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826203D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826203D0 size=112
    let mut pc: u32 = 0x826203D0;
    'dispatch: loop {
        match pc {
            0x826203D0 => {
    //   block [0x826203D0..0x82620440)
	// 826203D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826203D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826203D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826203DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826203E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826203E4: 392A18B4  addi r9, r10, 0x18b4
	ctx.r[9].s64 = ctx.r[10].s64 + 6324;
	// 826203E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826203EC: 390B3E28  addi r8, r11, 0x3e28
	ctx.r[8].s64 = ctx.r[11].s64 + 15912;
	// 826203F0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826203F4: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826203F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826203FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620408: 386AFC74  addi r3, r10, -0x38c
	ctx.r[3].s64 = ctx.r[10].s64 + -908;
	// 8262040C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82620410: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82620414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262041C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262042C: 4BE469F5  bl 0x82466e20
	ctx.lr = 0x82620430;
	sub_82466E20(ctx, base);
	// 82620430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262043C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620440 size=108
    let mut pc: u32 = 0x82620440;
    'dispatch: loop {
        match pc {
            0x82620440 => {
    //   block [0x82620440..0x826204AC)
	// 82620440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262044C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620454: 38EB0CB8  addi r7, r11, 0xcb8
	ctx.r[7].s64 = ctx.r[11].s64 + 3256;
	// 82620458: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262045C: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82620460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620464: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262046C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620470: 386AFCA4  addi r3, r10, -0x35c
	ctx.r[3].s64 = ctx.r[10].s64 + -860;
	// 82620474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262047C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262048C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620498: 4BE46989  bl 0x82466e20
	ctx.lr = 0x8262049C;
	sub_82466E20(ctx, base);
	// 8262049C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826204A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826204A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826204A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826204B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826204B0 size=108
    let mut pc: u32 = 0x826204B0;
    'dispatch: loop {
        match pc {
            0x826204B0 => {
    //   block [0x826204B0..0x8262051C)
	// 826204B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826204B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826204B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826204BC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826204C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826204C4: 38EB0CD0  addi r7, r11, 0xcd0
	ctx.r[7].s64 = ctx.r[11].s64 + 3280;
	// 826204C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826204CC: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826204D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826204D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826204D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826204DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826204E0: 386AFCD4  addi r3, r10, -0x32c
	ctx.r[3].s64 = ctx.r[10].s64 + -812;
	// 826204E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826204E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826204EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826204F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826204F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826204F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826204FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620508: 4BE46919  bl 0x82466e20
	ctx.lr = 0x8262050C;
	sub_82466E20(ctx, base);
	// 8262050C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620520 size=116
    let mut pc: u32 = 0x82620520;
    'dispatch: loop {
        match pc {
            0x82620520 => {
    //   block [0x82620520..0x82620594)
	// 82620520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262052C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620530: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620534: 390B0D1C  addi r8, r11, 0xd1c
	ctx.r[8].s64 = ctx.r[11].s64 + 3356;
	// 82620538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262053C: 392A1988  addi r9, r10, 0x1988
	ctx.r[9].s64 = ctx.r[10].s64 + 6536;
	// 82620540: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620544: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82620548: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8262054C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620554: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262055C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620564: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82620568: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8262056C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82620570: 386BFD04  addi r3, r11, -0x2fc
	ctx.r[3].s64 = ctx.r[11].s64 + -764;
	// 82620574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82620578: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262057C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620580: 4BE468A1  bl 0x82466e20
	ctx.lr = 0x82620584;
	sub_82466E20(ctx, base);
	// 82620584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262058C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620598 size=108
    let mut pc: u32 = 0x82620598;
    'dispatch: loop {
        match pc {
            0x82620598 => {
    //   block [0x82620598..0x82620604)
	// 82620598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262059C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826205A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826205A4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826205A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826205AC: 38EB0D38  addi r7, r11, 0xd38
	ctx.r[7].s64 = ctx.r[11].s64 + 3384;
	// 826205B0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826205B4: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 826205B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826205BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826205C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826205C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826205C8: 386AFD34  addi r3, r10, -0x2cc
	ctx.r[3].s64 = ctx.r[10].s64 + -716;
	// 826205CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826205D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826205D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826205D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826205DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826205E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826205E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826205E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826205EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826205F0: 4BE46831  bl 0x82466e20
	ctx.lr = 0x826205F4;
	sub_82466E20(ctx, base);
	// 826205F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826205F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826205FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82620608 size=24
    let mut pc: u32 = 0x82620608;
    'dispatch: loop {
        match pc {
            0x82620608 => {
    //   block [0x82620608..0x82620620)
	// 82620608: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262060C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82620610: 394A3E70  addi r10, r10, 0x3e70
	ctx.r[10].s64 = ctx.r[10].s64 + 15984;
	// 82620614: 816B0D34  lwz r11, 0xd34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3380 as u32) ) } as u64;
	// 82620618: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262061C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620620 size=116
    let mut pc: u32 = 0x82620620;
    'dispatch: loop {
        match pc {
            0x82620620 => {
    //   block [0x82620620..0x82620694)
	// 82620620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262062C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620634: 390B3E70  addi r8, r11, 0x3e70
	ctx.r[8].s64 = ctx.r[11].s64 + 15984;
	// 82620638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262063C: 392A19E4  addi r9, r10, 0x19e4
	ctx.r[9].s64 = ctx.r[10].s64 + 6628;
	// 82620640: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620644: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82620648: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8262064C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620654: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262065C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620664: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82620668: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8262066C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82620670: 386BFD64  addi r3, r11, -0x29c
	ctx.r[3].s64 = ctx.r[11].s64 + -668;
	// 82620674: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82620678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262067C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620680: 4BE467A1  bl 0x82466e20
	ctx.lr = 0x82620684;
	sub_82466E20(ctx, base);
	// 82620684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262068C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620698 size=112
    let mut pc: u32 = 0x82620698;
    'dispatch: loop {
        match pc {
            0x82620698 => {
    //   block [0x82620698..0x82620708)
	// 82620698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262069C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826206A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826206A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826206A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826206AC: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 826206B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826206B4: 390B0DA0  addi r8, r11, 0xda0
	ctx.r[8].s64 = ctx.r[11].s64 + 3488;
	// 826206B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826206BC: 388A2200  addi r4, r10, 0x2200
	ctx.r[4].s64 = ctx.r[10].s64 + 8704;
	// 826206C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826206C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826206C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826206CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826206D0: 386AFD94  addi r3, r10, -0x26c
	ctx.r[3].s64 = ctx.r[10].s64 + -620;
	// 826206D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826206D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826206DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826206E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826206E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826206E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826206EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826206F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826206F4: 4BE4672D  bl 0x82466e20
	ctx.lr = 0x826206F8;
	sub_82466E20(ctx, base);
	// 826206F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826206FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620708 size=112
    let mut pc: u32 = 0x82620708;
    'dispatch: loop {
        match pc {
            0x82620708 => {
    //   block [0x82620708..0x82620778)
	// 82620708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262070C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620714: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620718: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262071C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620724: 390B0DB8  addi r8, r11, 0xdb8
	ctx.r[8].s64 = ctx.r[11].s64 + 3512;
	// 82620728: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262072C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82620730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620734: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262073C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620740: 386AFDC4  addi r3, r10, -0x23c
	ctx.r[3].s64 = ctx.r[10].s64 + -572;
	// 82620744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262074C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262075C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620764: 4BE466BD  bl 0x82466e20
	ctx.lr = 0x82620768;
	sub_82466E20(ctx, base);
	// 82620768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262076C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620778 size=108
    let mut pc: u32 = 0x82620778;
    'dispatch: loop {
        match pc {
            0x82620778 => {
    //   block [0x82620778..0x826207E4)
	// 82620778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262077C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620784: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620788: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262078C: 38EB0DE8  addi r7, r11, 0xde8
	ctx.r[7].s64 = ctx.r[11].s64 + 3560;
	// 82620790: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82620794: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 82620798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262079C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826207A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826207A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826207A8: 386AFDF4  addi r3, r10, -0x20c
	ctx.r[3].s64 = ctx.r[10].s64 + -524;
	// 826207AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826207B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826207B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826207B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826207BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826207C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826207C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826207C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826207CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826207D0: 4BE46651  bl 0x82466e20
	ctx.lr = 0x826207D4;
	sub_82466E20(ctx, base);
	// 826207D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826207D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826207DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826207E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826207E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826207E8 size=108
    let mut pc: u32 = 0x826207E8;
    'dispatch: loop {
        match pc {
            0x826207E8 => {
    //   block [0x826207E8..0x82620854)
	// 826207E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826207EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826207F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826207F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826207F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826207FC: 38EB0E30  addi r7, r11, 0xe30
	ctx.r[7].s64 = ctx.r[11].s64 + 3632;
	// 82620800: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82620804: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 82620808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262080C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620818: 386AFE24  addi r3, r10, -0x1dc
	ctx.r[3].s64 = ctx.r[10].s64 + -476;
	// 8262081C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262082C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262083C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620840: 4BE465E1  bl 0x82466e20
	ctx.lr = 0x82620844;
	sub_82466E20(ctx, base);
	// 82620844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262084C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620858 size=112
    let mut pc: u32 = 0x82620858;
    'dispatch: loop {
        match pc {
            0x82620858 => {
    //   block [0x82620858..0x826208C8)
	// 82620858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262085C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620864: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620868: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262086C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620874: 390B0E60  addi r8, r11, 0xe60
	ctx.r[8].s64 = ctx.r[11].s64 + 3680;
	// 82620878: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262087C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 82620880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620884: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262088C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620890: 386AFE54  addi r3, r10, -0x1ac
	ctx.r[3].s64 = ctx.r[10].s64 + -428;
	// 82620894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262089C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826208A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826208A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826208A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826208AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826208B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826208B4: 4BE4656D  bl 0x82466e20
	ctx.lr = 0x826208B8;
	sub_82466E20(ctx, base);
	// 826208B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826208BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826208C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826208C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826208C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826208C8 size=108
    let mut pc: u32 = 0x826208C8;
    'dispatch: loop {
        match pc {
            0x826208C8 => {
    //   block [0x826208C8..0x82620934)
	// 826208C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826208CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826208D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826208D4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826208D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826208DC: 38EB0E90  addi r7, r11, 0xe90
	ctx.r[7].s64 = ctx.r[11].s64 + 3728;
	// 826208E0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826208E4: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 826208E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826208EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826208F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826208F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826208F8: 386AFE84  addi r3, r10, -0x17c
	ctx.r[3].s64 = ctx.r[10].s64 + -380;
	// 826208FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262090C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262091C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620920: 4BE46501  bl 0x82466e20
	ctx.lr = 0x82620924;
	sub_82466E20(ctx, base);
	// 82620924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262092C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620938 size=108
    let mut pc: u32 = 0x82620938;
    'dispatch: loop {
        match pc {
            0x82620938 => {
    //   block [0x82620938..0x826209A4)
	// 82620938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262093C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620944: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620948: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262094C: 38EB0EF0  addi r7, r11, 0xef0
	ctx.r[7].s64 = ctx.r[11].s64 + 3824;
	// 82620950: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82620954: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 82620958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262095C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620968: 386AFEB4  addi r3, r10, -0x14c
	ctx.r[3].s64 = ctx.r[10].s64 + -332;
	// 8262096C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262097C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262098C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620990: 4BE46491  bl 0x82466e20
	ctx.lr = 0x82620994;
	sub_82466E20(ctx, base);
	// 82620994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262099C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826209A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826209A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826209A8 size=116
    let mut pc: u32 = 0x826209A8;
    'dispatch: loop {
        match pc {
            0x826209A8 => {
    //   block [0x826209A8..0x82620A1C)
	// 826209A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826209AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826209B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826209B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826209B8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826209BC: 390A0F38  addi r8, r10, 0xf38
	ctx.r[8].s64 = ctx.r[10].s64 + 3896;
	// 826209C0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826209C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826209C8: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 826209CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826209D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826209D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826209D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826209DC: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826209E0: 396B1A20  addi r11, r11, 0x1a20
	ctx.r[11].s64 = ctx.r[11].s64 + 6688;
	// 826209E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826209E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826209EC: 386AFEE4  addi r3, r10, -0x11c
	ctx.r[3].s64 = ctx.r[10].s64 + -284;
	// 826209F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826209F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826209F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826209FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620A08: 4BE46419  bl 0x82466e20
	ctx.lr = 0x82620A0C;
	sub_82466E20(ctx, base);
	// 82620A0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620A18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620A20 size=112
    let mut pc: u32 = 0x82620A20;
    'dispatch: loop {
        match pc {
            0x82620A20 => {
    //   block [0x82620A20..0x82620A90)
	// 82620A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620A2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620A30: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620A34: 38AAFF44  addi r5, r10, -0xbc
	ctx.r[5].s64 = ctx.r[10].s64 + -188;
	// 82620A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620A3C: 390B0FC8  addi r8, r11, 0xfc8
	ctx.r[8].s64 = ctx.r[11].s64 + 4040;
	// 82620A40: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82620A44: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82620A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620A50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620A58: 386AFF14  addi r3, r10, -0xec
	ctx.r[3].s64 = ctx.r[10].s64 + -236;
	// 82620A5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620A7C: 4BE463A5  bl 0x82466e20
	ctx.lr = 0x82620A80;
	sub_82466E20(ctx, base);
	// 82620A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620A90 size=100
    let mut pc: u32 = 0x82620A90;
    'dispatch: loop {
        match pc {
            0x82620A90 => {
    //   block [0x82620A90..0x82620AF4)
	// 82620A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620A9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620AA4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82620AA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620AB0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 82620AB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620AB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620AC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620AC4: 386AFF44  addi r3, r10, -0xbc
	ctx.r[3].s64 = ctx.r[10].s64 + -188;
	// 82620AC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620ACC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620AD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82620AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620AD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82620ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620AE0: 4BE46341  bl 0x82466e20
	ctx.lr = 0x82620AE4;
	sub_82466E20(ctx, base);
	// 82620AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82620AF8 size=24
    let mut pc: u32 = 0x82620AF8;
    'dispatch: loop {
        match pc {
            0x82620AF8 => {
    //   block [0x82620AF8..0x82620B10)
	// 82620AF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620AFC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82620B00: 394A3F30  addi r10, r10, 0x3f30
	ctx.r[10].s64 = ctx.r[10].s64 + 16176;
	// 82620B04: 816B1040  lwz r11, 0x1040(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4160 as u32) ) } as u64;
	// 82620B08: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82620B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620B10 size=116
    let mut pc: u32 = 0x82620B10;
    'dispatch: loop {
        match pc {
            0x82620B10 => {
    //   block [0x82620B10..0x82620B84)
	// 82620B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620B1C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620B20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620B24: 390B3F30  addi r8, r11, 0x3f30
	ctx.r[8].s64 = ctx.r[11].s64 + 16176;
	// 82620B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620B2C: 392A1A64  addi r9, r10, 0x1a64
	ctx.r[9].s64 = ctx.r[10].s64 + 6756;
	// 82620B30: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620B34: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82620B38: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620B3C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620B44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620B54: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82620B58: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82620B5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82620B60: 386BFF74  addi r3, r11, -0x8c
	ctx.r[3].s64 = ctx.r[11].s64 + -140;
	// 82620B64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82620B68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620B70: 4BE462B1  bl 0x82466e20
	ctx.lr = 0x82620B74;
	sub_82466E20(ctx, base);
	// 82620B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620B88 size=112
    let mut pc: u32 = 0x82620B88;
    'dispatch: loop {
        match pc {
            0x82620B88 => {
    //   block [0x82620B88..0x82620BF8)
	// 82620B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620B94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620B98: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620B9C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620BA4: 390B1048  addi r8, r11, 0x1048
	ctx.r[8].s64 = ctx.r[11].s64 + 4168;
	// 82620BA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82620BAC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 82620BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620BB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620BC0: 386AFFA4  addi r3, r10, -0x5c
	ctx.r[3].s64 = ctx.r[10].s64 + -92;
	// 82620BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620BE4: 4BE4623D  bl 0x82466e20
	ctx.lr = 0x82620BE8;
	sub_82466E20(ctx, base);
	// 82620BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620BF8 size=112
    let mut pc: u32 = 0x82620BF8;
    'dispatch: loop {
        match pc {
            0x82620BF8 => {
    //   block [0x82620BF8..0x82620C68)
	// 82620BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620C04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620C08: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620C0C: 38AAFEE4  addi r5, r10, -0x11c
	ctx.r[5].s64 = ctx.r[10].s64 + -284;
	// 82620C10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620C14: 390B1090  addi r8, r11, 0x1090
	ctx.r[8].s64 = ctx.r[11].s64 + 4240;
	// 82620C18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82620C1C: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 82620C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620C24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620C30: 386AFFD4  addi r3, r10, -0x2c
	ctx.r[3].s64 = ctx.r[10].s64 + -44;
	// 82620C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620C54: 4BE461CD  bl 0x82466e20
	ctx.lr = 0x82620C58;
	sub_82466E20(ctx, base);
	// 82620C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620C68 size=108
    let mut pc: u32 = 0x82620C68;
    'dispatch: loop {
        match pc {
            0x82620C68 => {
    //   block [0x82620C68..0x82620CD4)
	// 82620C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620C74: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620C78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620C7C: 38EB10F0  addi r7, r11, 0x10f0
	ctx.r[7].s64 = ctx.r[11].s64 + 4336;
	// 82620C80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82620C84: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 82620C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620C8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620C90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620C98: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82620C9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620CC0: 4BE46161  bl 0x82466e20
	ctx.lr = 0x82620CC4;
	sub_82466E20(ctx, base);
	// 82620CC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620CD8 size=108
    let mut pc: u32 = 0x82620CD8;
    'dispatch: loop {
        match pc {
            0x82620CD8 => {
    //   block [0x82620CD8..0x82620D44)
	// 82620CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620CE4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620CE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620CEC: 38EB1138  addi r7, r11, 0x1138
	ctx.r[7].s64 = ctx.r[11].s64 + 4408;
	// 82620CF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82620CF4: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 82620CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620CFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620D00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620D08: 386A0034  addi r3, r10, 0x34
	ctx.r[3].s64 = ctx.r[10].s64 + 52;
	// 82620D0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620D2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620D30: 4BE460F1  bl 0x82466e20
	ctx.lr = 0x82620D34;
	sub_82466E20(ctx, base);
	// 82620D34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620D38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620D3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620D48 size=112
    let mut pc: u32 = 0x82620D48;
    'dispatch: loop {
        match pc {
            0x82620D48 => {
    //   block [0x82620D48..0x82620DB8)
	// 82620D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620D54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620D58: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620D5C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620D64: 390B1180  addi r8, r11, 0x1180
	ctx.r[8].s64 = ctx.r[11].s64 + 4480;
	// 82620D68: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82620D6C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82620D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620D74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620D80: 386A0064  addi r3, r10, 0x64
	ctx.r[3].s64 = ctx.r[10].s64 + 100;
	// 82620D84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620D8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620DA4: 4BE4607D  bl 0x82466e20
	ctx.lr = 0x82620DA8;
	sub_82466E20(ctx, base);
	// 82620DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620DB8 size=112
    let mut pc: u32 = 0x82620DB8;
    'dispatch: loop {
        match pc {
            0x82620DB8 => {
    //   block [0x82620DB8..0x82620E28)
	// 82620DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620DC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620DC8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620DCC: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620DD4: 390B1228  addi r8, r11, 0x1228
	ctx.r[8].s64 = ctx.r[11].s64 + 4648;
	// 82620DD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82620DDC: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82620DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620DE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620DE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620DF0: 386A0094  addi r3, r10, 0x94
	ctx.r[3].s64 = ctx.r[10].s64 + 148;
	// 82620DF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620E14: 4BE4600D  bl 0x82466e20
	ctx.lr = 0x82620E18;
	sub_82466E20(ctx, base);
	// 82620E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620E24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620E28 size=108
    let mut pc: u32 = 0x82620E28;
    'dispatch: loop {
        match pc {
            0x82620E28 => {
    //   block [0x82620E28..0x82620E94)
	// 82620E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620E34: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620E38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620E3C: 38EB1270  addi r7, r11, 0x1270
	ctx.r[7].s64 = ctx.r[11].s64 + 4720;
	// 82620E40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82620E44: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 82620E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620E4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620E50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620E58: 386A00C4  addi r3, r10, 0xc4
	ctx.r[3].s64 = ctx.r[10].s64 + 196;
	// 82620E5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620E80: 4BE45FA1  bl 0x82466e20
	ctx.lr = 0x82620E84;
	sub_82466E20(ctx, base);
	// 82620E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620E98 size=108
    let mut pc: u32 = 0x82620E98;
    'dispatch: loop {
        match pc {
            0x82620E98 => {
    //   block [0x82620E98..0x82620F04)
	// 82620E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620EA4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620EA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82620EAC: 38EB12A0  addi r7, r11, 0x12a0
	ctx.r[7].s64 = ctx.r[11].s64 + 4768;
	// 82620EB0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82620EB4: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 82620EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620EBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620EC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82620EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620EC8: 386A00F4  addi r3, r10, 0xf4
	ctx.r[3].s64 = ctx.r[10].s64 + 244;
	// 82620ECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82620ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620EEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82620EF0: 4BE45F31  bl 0x82466e20
	ctx.lr = 0x82620EF4;
	sub_82466E20(ctx, base);
	// 82620EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620F08 size=112
    let mut pc: u32 = 0x82620F08;
    'dispatch: loop {
        match pc {
            0x82620F08 => {
    //   block [0x82620F08..0x82620F78)
	// 82620F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620F18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620F1C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620F24: 390B1330  addi r8, r11, 0x1330
	ctx.r[8].s64 = ctx.r[11].s64 + 4912;
	// 82620F28: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82620F2C: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82620F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620F34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620F38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620F40: 386A0124  addi r3, r10, 0x124
	ctx.r[3].s64 = ctx.r[10].s64 + 292;
	// 82620F44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620F64: 4BE45EBD  bl 0x82466e20
	ctx.lr = 0x82620F68;
	sub_82466E20(ctx, base);
	// 82620F68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620F6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620F70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620F78 size=112
    let mut pc: u32 = 0x82620F78;
    'dispatch: loop {
        match pc {
            0x82620F78 => {
    //   block [0x82620F78..0x82620FE8)
	// 82620F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620F84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620F88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82620F8C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82620F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82620F94: 390B13C0  addi r8, r11, 0x13c0
	ctx.r[8].s64 = ctx.r[11].s64 + 5056;
	// 82620F98: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82620F9C: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 82620FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620FA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82620FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82620FB0: 386A0154  addi r3, r10, 0x154
	ctx.r[3].s64 = ctx.r[10].s64 + 340;
	// 82620FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82620FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82620FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82620FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82620FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82620FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82620FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82620FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82620FD4: 4BE45E4D  bl 0x82466e20
	ctx.lr = 0x82620FD8;
	sub_82466E20(ctx, base);
	// 82620FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82620FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82620FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82620FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82620FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82620FE8 size=100
    let mut pc: u32 = 0x82620FE8;
    'dispatch: loop {
        match pc {
            0x82620FE8 => {
    //   block [0x82620FE8..0x8262104C)
	// 82620FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82620FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82620FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82620FF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82620FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82620FFC: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82621000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621008: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8262100C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262101C: 386A0184  addi r3, r10, 0x184
	ctx.r[3].s64 = ctx.r[10].s64 + 388;
	// 82621020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621024: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621028: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262102C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621030: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621038: 4BE45DE9  bl 0x82466e20
	ctx.lr = 0x8262103C;
	sub_82466E20(ctx, base);
	// 8262103C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621050 size=112
    let mut pc: u32 = 0x82621050;
    'dispatch: loop {
        match pc {
            0x82621050 => {
    //   block [0x82621050..0x826210C0)
	// 82621050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262105C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621060: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621064: 38AAFD64  addi r5, r10, -0x29c
	ctx.r[5].s64 = ctx.r[10].s64 + -668;
	// 82621068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262106C: 390B1480  addi r8, r11, 0x1480
	ctx.r[8].s64 = ctx.r[11].s64 + 5248;
	// 82621070: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82621074: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 82621078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262107C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621088: 386A01B4  addi r3, r10, 0x1b4
	ctx.r[3].s64 = ctx.r[10].s64 + 436;
	// 8262108C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621090: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262109C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826210A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826210A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826210A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826210AC: 4BE45D75  bl 0x82466e20
	ctx.lr = 0x826210B0;
	sub_82466E20(ctx, base);
	// 826210B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826210B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826210B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826210BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826210C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826210C0 size=112
    let mut pc: u32 = 0x826210C0;
    'dispatch: loop {
        match pc {
            0x826210C0 => {
    //   block [0x826210C0..0x82621130)
	// 826210C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826210C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826210C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826210CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826210D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826210D4: 38AAFBB4  addi r5, r10, -0x44c
	ctx.r[5].s64 = ctx.r[10].s64 + -1100;
	// 826210D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826210DC: 390B14B0  addi r8, r11, 0x14b0
	ctx.r[8].s64 = ctx.r[11].s64 + 5296;
	// 826210E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826210E4: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 826210E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826210EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826210F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826210F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826210F8: 386A01E4  addi r3, r10, 0x1e4
	ctx.r[3].s64 = ctx.r[10].s64 + 484;
	// 826210FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262110C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262111C: 4BE45D05  bl 0x82466e20
	ctx.lr = 0x82621120;
	sub_82466E20(ctx, base);
	// 82621120: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262112C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621130 size=108
    let mut pc: u32 = 0x82621130;
    'dispatch: loop {
        match pc {
            0x82621130 => {
    //   block [0x82621130..0x8262119C)
	// 82621130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262113C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621144: 38EB14C8  addi r7, r11, 0x14c8
	ctx.r[7].s64 = ctx.r[11].s64 + 5320;
	// 82621148: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262114C: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82621150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621154: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621158: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262115C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621160: 386A0214  addi r3, r10, 0x214
	ctx.r[3].s64 = ctx.r[10].s64 + 532;
	// 82621164: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262116C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262117C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621184: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621188: 4BE45C99  bl 0x82466e20
	ctx.lr = 0x8262118C;
	sub_82466E20(ctx, base);
	// 8262118C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826211A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826211A0 size=112
    let mut pc: u32 = 0x826211A0;
    'dispatch: loop {
        match pc {
            0x826211A0 => {
    //   block [0x826211A0..0x82621210)
	// 826211A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826211A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826211A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826211AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826211B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826211B4: 38AA0184  addi r5, r10, 0x184
	ctx.r[5].s64 = ctx.r[10].s64 + 388;
	// 826211B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826211BC: 390B14F8  addi r8, r11, 0x14f8
	ctx.r[8].s64 = ctx.r[11].s64 + 5368;
	// 826211C0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826211C4: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826211C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826211CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826211D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826211D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826211D8: 386A0244  addi r3, r10, 0x244
	ctx.r[3].s64 = ctx.r[10].s64 + 580;
	// 826211DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826211E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826211E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826211E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826211EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826211F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826211F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826211F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826211FC: 4BE45C25  bl 0x82466e20
	ctx.lr = 0x82621200;
	sub_82466E20(ctx, base);
	// 82621200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262120C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621210 size=108
    let mut pc: u32 = 0x82621210;
    'dispatch: loop {
        match pc {
            0x82621210 => {
    //   block [0x82621210..0x8262127C)
	// 82621210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262121C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621224: 38EB1570  addi r7, r11, 0x1570
	ctx.r[7].s64 = ctx.r[11].s64 + 5488;
	// 82621228: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262122C: 388A239C  addi r4, r10, 0x239c
	ctx.r[4].s64 = ctx.r[10].s64 + 9116;
	// 82621230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621234: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262123C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621240: 386A0274  addi r3, r10, 0x274
	ctx.r[3].s64 = ctx.r[10].s64 + 628;
	// 82621244: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262124C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262125C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621268: 4BE45BB9  bl 0x82466e20
	ctx.lr = 0x8262126C;
	sub_82466E20(ctx, base);
	// 8262126C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621280 size=112
    let mut pc: u32 = 0x82621280;
    'dispatch: loop {
        match pc {
            0x82621280 => {
    //   block [0x82621280..0x826212F0)
	// 82621280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262128C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621290: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621294: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82621298: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262129C: 390B15A0  addi r8, r11, 0x15a0
	ctx.r[8].s64 = ctx.r[11].s64 + 5536;
	// 826212A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826212A4: 388A23C0  addi r4, r10, 0x23c0
	ctx.r[4].s64 = ctx.r[10].s64 + 9152;
	// 826212A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826212AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826212B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826212B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826212B8: 386A02A4  addi r3, r10, 0x2a4
	ctx.r[3].s64 = ctx.r[10].s64 + 676;
	// 826212BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826212C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826212C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826212C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826212CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826212D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826212D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826212D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826212DC: 4BE45B45  bl 0x82466e20
	ctx.lr = 0x826212E0;
	sub_82466E20(ctx, base);
	// 826212E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826212E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826212E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826212EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826212F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826212F0 size=108
    let mut pc: u32 = 0x826212F0;
    'dispatch: loop {
        match pc {
            0x826212F0 => {
    //   block [0x826212F0..0x8262135C)
	// 826212F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826212F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826212F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826212FC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621304: 38EB15E8  addi r7, r11, 0x15e8
	ctx.r[7].s64 = ctx.r[11].s64 + 5608;
	// 82621308: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262130C: 388A23DC  addi r4, r10, 0x23dc
	ctx.r[4].s64 = ctx.r[10].s64 + 9180;
	// 82621310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621314: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621318: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262131C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621320: 386A02D4  addi r3, r10, 0x2d4
	ctx.r[3].s64 = ctx.r[10].s64 + 724;
	// 82621324: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262132C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262133C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621344: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621348: 4BE45AD9  bl 0x82466e20
	ctx.lr = 0x8262134C;
	sub_82466E20(ctx, base);
	// 8262134C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621360 size=108
    let mut pc: u32 = 0x82621360;
    'dispatch: loop {
        match pc {
            0x82621360 => {
    //   block [0x82621360..0x826213CC)
	// 82621360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262136C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621370: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621374: 38EB1630  addi r7, r11, 0x1630
	ctx.r[7].s64 = ctx.r[11].s64 + 5680;
	// 82621378: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262137C: 388A2400  addi r4, r10, 0x2400
	ctx.r[4].s64 = ctx.r[10].s64 + 9216;
	// 82621380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621384: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621388: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262138C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621390: 386A0304  addi r3, r10, 0x304
	ctx.r[3].s64 = ctx.r[10].s64 + 772;
	// 82621394: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262139C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826213A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826213A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826213A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826213AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826213B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826213B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826213B8: 4BE45A69  bl 0x82466e20
	ctx.lr = 0x826213BC;
	sub_82466E20(ctx, base);
	// 826213BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826213C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826213C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826213C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826213D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826213D0 size=112
    let mut pc: u32 = 0x826213D0;
    'dispatch: loop {
        match pc {
            0x826213D0 => {
    //   block [0x826213D0..0x82621440)
	// 826213D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826213D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826213D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826213DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826213E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826213E4: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 826213E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826213EC: 390B1660  addi r8, r11, 0x1660
	ctx.r[8].s64 = ctx.r[11].s64 + 5728;
	// 826213F0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826213F4: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826213F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826213FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621408: 386A0334  addi r3, r10, 0x334
	ctx.r[3].s64 = ctx.r[10].s64 + 820;
	// 8262140C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262141C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262142C: 4BE459F5  bl 0x82466e20
	ctx.lr = 0x82621430;
	sub_82466E20(ctx, base);
	// 82621430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262143C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621440 size=108
    let mut pc: u32 = 0x82621440;
    'dispatch: loop {
        match pc {
            0x82621440 => {
    //   block [0x82621440..0x826214AC)
	// 82621440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262144C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621454: 38EB16F0  addi r7, r11, 0x16f0
	ctx.r[7].s64 = ctx.r[11].s64 + 5872;
	// 82621458: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8262145C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82621460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621464: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262146C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621470: 386A0364  addi r3, r10, 0x364
	ctx.r[3].s64 = ctx.r[10].s64 + 868;
	// 82621474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262147C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262148C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621498: 4BE45989  bl 0x82466e20
	ctx.lr = 0x8262149C;
	sub_82466E20(ctx, base);
	// 8262149C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826214A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826214A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826214A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826214B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826214B0 size=112
    let mut pc: u32 = 0x826214B0;
    'dispatch: loop {
        match pc {
            0x826214B0 => {
    //   block [0x826214B0..0x82621520)
	// 826214B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826214B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826214B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826214BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826214C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826214C4: 38AA0184  addi r5, r10, 0x184
	ctx.r[5].s64 = ctx.r[10].s64 + 388;
	// 826214C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826214CC: 390B1780  addi r8, r11, 0x1780
	ctx.r[8].s64 = ctx.r[11].s64 + 6016;
	// 826214D0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826214D4: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826214D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826214DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826214E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826214E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826214E8: 386A0394  addi r3, r10, 0x394
	ctx.r[3].s64 = ctx.r[10].s64 + 916;
	// 826214EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826214F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826214F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826214F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826214FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262150C: 4BE45915  bl 0x82466e20
	ctx.lr = 0x82621510;
	sub_82466E20(ctx, base);
	// 82621510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262151C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621520 size=108
    let mut pc: u32 = 0x82621520;
    'dispatch: loop {
        match pc {
            0x82621520 => {
    //   block [0x82621520..0x8262158C)
	// 82621520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262152C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621534: 38EB17F8  addi r7, r11, 0x17f8
	ctx.r[7].s64 = ctx.r[11].s64 + 6136;
	// 82621538: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262153C: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 82621540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621544: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621548: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262154C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621550: 386A03C4  addi r3, r10, 0x3c4
	ctx.r[3].s64 = ctx.r[10].s64 + 964;
	// 82621554: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262155C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262156C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621574: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621578: 4BE458A9  bl 0x82466e20
	ctx.lr = 0x8262157C;
	sub_82466E20(ctx, base);
	// 8262157C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621590 size=112
    let mut pc: u32 = 0x82621590;
    'dispatch: loop {
        match pc {
            0x82621590 => {
    //   block [0x82621590..0x82621600)
	// 82621590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262159C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826215A0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826215A4: 38AA0184  addi r5, r10, 0x184
	ctx.r[5].s64 = ctx.r[10].s64 + 388;
	// 826215A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826215AC: 390B1840  addi r8, r11, 0x1840
	ctx.r[8].s64 = ctx.r[11].s64 + 6208;
	// 826215B0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826215B4: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 826215B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826215BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826215C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826215C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826215C8: 386A03F4  addi r3, r10, 0x3f4
	ctx.r[3].s64 = ctx.r[10].s64 + 1012;
	// 826215CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826215D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826215D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826215D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826215DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826215E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826215E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826215E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826215EC: 4BE45835  bl 0x82466e20
	ctx.lr = 0x826215F0;
	sub_82466E20(ctx, base);
	// 826215F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826215F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826215F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826215FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621600 size=112
    let mut pc: u32 = 0x82621600;
    'dispatch: loop {
        match pc {
            0x82621600 => {
    //   block [0x82621600..0x82621670)
	// 82621600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262160C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621610: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621614: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82621618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262161C: 390B18A0  addi r8, r11, 0x18a0
	ctx.r[8].s64 = ctx.r[11].s64 + 6304;
	// 82621620: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82621624: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82621628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262162C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621638: 386A0424  addi r3, r10, 0x424
	ctx.r[3].s64 = ctx.r[10].s64 + 1060;
	// 8262163C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262164C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262165C: 4BE457C5  bl 0x82466e20
	ctx.lr = 0x82621660;
	sub_82466E20(ctx, base);
	// 82621660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262166C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621670 size=108
    let mut pc: u32 = 0x82621670;
    'dispatch: loop {
        match pc {
            0x82621670 => {
    //   block [0x82621670..0x826216DC)
	// 82621670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262167C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621684: 38EB18B8  addi r7, r11, 0x18b8
	ctx.r[7].s64 = ctx.r[11].s64 + 6328;
	// 82621688: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262168C: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82621690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621698: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262169C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826216A0: 386A0454  addi r3, r10, 0x454
	ctx.r[3].s64 = ctx.r[10].s64 + 1108;
	// 826216A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826216A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826216AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826216B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826216B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826216B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826216BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826216C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826216C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826216C8: 4BE45759  bl 0x82466e20
	ctx.lr = 0x826216CC;
	sub_82466E20(ctx, base);
	// 826216CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826216D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826216D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826216D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826216E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826216E0 size=112
    let mut pc: u32 = 0x826216E0;
    'dispatch: loop {
        match pc {
            0x826216E0 => {
    //   block [0x826216E0..0x82621750)
	// 826216E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826216E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826216E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826216EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826216F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826216F4: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 826216F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826216FC: 390B1930  addi r8, r11, 0x1930
	ctx.r[8].s64 = ctx.r[11].s64 + 6448;
	// 82621700: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82621704: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82621708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262170C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621718: 386A0484  addi r3, r10, 0x484
	ctx.r[3].s64 = ctx.r[10].s64 + 1156;
	// 8262171C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262172C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262173C: 4BE456E5  bl 0x82466e20
	ctx.lr = 0x82621740;
	sub_82466E20(ctx, base);
	// 82621740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262174C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621750 size=100
    let mut pc: u32 = 0x82621750;
    'dispatch: loop {
        match pc {
            0x82621750 => {
    //   block [0x82621750..0x826217B4)
	// 82621750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262175C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621764: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82621768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262176C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621770: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82621774: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262177C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621784: 386A04B4  addi r3, r10, 0x4b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1204;
	// 82621788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262178C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621790: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621798: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262179C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826217A0: 4BE45681  bl 0x82466e20
	ctx.lr = 0x826217A4;
	sub_82466E20(ctx, base);
	// 826217A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826217A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826217AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826217B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826217B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826217B8 size=112
    let mut pc: u32 = 0x826217B8;
    'dispatch: loop {
        match pc {
            0x826217B8 => {
    //   block [0x826217B8..0x82621828)
	// 826217B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826217BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826217C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826217C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826217C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826217CC: 38AA04B4  addi r5, r10, 0x4b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1204;
	// 826217D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826217D4: 390B1960  addi r8, r11, 0x1960
	ctx.r[8].s64 = ctx.r[11].s64 + 6496;
	// 826217D8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826217DC: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826217E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826217E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826217E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826217EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826217F0: 386A04E4  addi r3, r10, 0x4e4
	ctx.r[3].s64 = ctx.r[10].s64 + 1252;
	// 826217F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826217F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826217FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262180C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621814: 4BE4560D  bl 0x82466e20
	ctx.lr = 0x82621818;
	sub_82466E20(ctx, base);
	// 82621818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262181C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621828 size=112
    let mut pc: u32 = 0x82621828;
    'dispatch: loop {
        match pc {
            0x82621828 => {
    //   block [0x82621828..0x82621898)
	// 82621828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262182C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621834: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621838: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262183C: 38AA04B4  addi r5, r10, 0x4b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1204;
	// 82621840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621844: 390B19D8  addi r8, r11, 0x19d8
	ctx.r[8].s64 = ctx.r[11].s64 + 6616;
	// 82621848: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262184C: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82621850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621854: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262185C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621860: 386A0514  addi r3, r10, 0x514
	ctx.r[3].s64 = ctx.r[10].s64 + 1300;
	// 82621864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262186C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262187C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621884: 4BE4559D  bl 0x82466e20
	ctx.lr = 0x82621888;
	sub_82466E20(ctx, base);
	// 82621888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262188C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621898 size=112
    let mut pc: u32 = 0x82621898;
    'dispatch: loop {
        match pc {
            0x82621898 => {
    //   block [0x82621898..0x82621908)
	// 82621898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262189C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826218A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826218A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826218A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826218AC: 38AA04B4  addi r5, r10, 0x4b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1204;
	// 826218B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826218B4: 390B1A38  addi r8, r11, 0x1a38
	ctx.r[8].s64 = ctx.r[11].s64 + 6712;
	// 826218B8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826218BC: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826218C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826218C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826218C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826218CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826218D0: 386A0544  addi r3, r10, 0x544
	ctx.r[3].s64 = ctx.r[10].s64 + 1348;
	// 826218D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826218D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826218DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826218E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826218E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826218E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826218EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826218F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826218F4: 4BE4552D  bl 0x82466e20
	ctx.lr = 0x826218F8;
	sub_82466E20(ctx, base);
	// 826218F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826218FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621908 size=112
    let mut pc: u32 = 0x82621908;
    'dispatch: loop {
        match pc {
            0x82621908 => {
    //   block [0x82621908..0x82621978)
	// 82621908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262190C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621914: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621918: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262191C: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82621920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621924: 390B1AB0  addi r8, r11, 0x1ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 6832;
	// 82621928: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8262192C: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82621930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262193C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621940: 386A0574  addi r3, r10, 0x574
	ctx.r[3].s64 = ctx.r[10].s64 + 1396;
	// 82621944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262194C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262195C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621964: 4BE454BD  bl 0x82466e20
	ctx.lr = 0x82621968;
	sub_82466E20(ctx, base);
	// 82621968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262196C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621978 size=116
    let mut pc: u32 = 0x82621978;
    'dispatch: loop {
        match pc {
            0x82621978 => {
    //   block [0x82621978..0x826219EC)
	// 82621978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262197C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621984: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82621988: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 8262198C: 390A1B40  addi r8, r10, 0x1b40
	ctx.r[8].s64 = ctx.r[10].s64 + 6976;
	// 82621990: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621994: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82621998: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 8262199C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826219A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826219A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826219A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826219AC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826219B0: 396B1A78  addi r11, r11, 0x1a78
	ctx.r[11].s64 = ctx.r[11].s64 + 6776;
	// 826219B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826219B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826219BC: 386A05A4  addi r3, r10, 0x5a4
	ctx.r[3].s64 = ctx.r[10].s64 + 1444;
	// 826219C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826219C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826219C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826219CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826219D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826219D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826219D8: 4BE45449  bl 0x82466e20
	ctx.lr = 0x826219DC;
	sub_82466E20(ctx, base);
	// 826219DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826219E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826219E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826219E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826219F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826219F0 size=100
    let mut pc: u32 = 0x826219F0;
    'dispatch: loop {
        match pc {
            0x826219F0 => {
    //   block [0x826219F0..0x82621A54)
	// 826219F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826219F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826219F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826219FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621A04: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82621A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621A10: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 82621A14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621A24: 386A05D4  addi r3, r10, 0x5d4
	ctx.r[3].s64 = ctx.r[10].s64 + 1492;
	// 82621A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621A2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621A30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621A38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621A40: 4BE453E1  bl 0x82466e20
	ctx.lr = 0x82621A44;
	sub_82466E20(ctx, base);
	// 82621A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621A58 size=100
    let mut pc: u32 = 0x82621A58;
    'dispatch: loop {
        match pc {
            0x82621A58 => {
    //   block [0x82621A58..0x82621ABC)
	// 82621A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621A64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621A6C: 38AA0664  addi r5, r10, 0x664
	ctx.r[5].s64 = ctx.r[10].s64 + 1636;
	// 82621A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621A78: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82621A7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621A8C: 386A0604  addi r3, r10, 0x604
	ctx.r[3].s64 = ctx.r[10].s64 + 1540;
	// 82621A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621A94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621A98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621AA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621AA8: 4BE45379  bl 0x82466e20
	ctx.lr = 0x82621AAC;
	sub_82466E20(ctx, base);
	// 82621AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621AC0 size=100
    let mut pc: u32 = 0x82621AC0;
    'dispatch: loop {
        match pc {
            0x82621AC0 => {
    //   block [0x82621AC0..0x82621B24)
	// 82621AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621ACC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621AD4: 38AA05A4  addi r5, r10, 0x5a4
	ctx.r[5].s64 = ctx.r[10].s64 + 1444;
	// 82621AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621AE0: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 82621AE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621AF4: 386A0634  addi r3, r10, 0x634
	ctx.r[3].s64 = ctx.r[10].s64 + 1588;
	// 82621AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621AFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621B00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621B08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621B10: 4BE45311  bl 0x82466e20
	ctx.lr = 0x82621B14;
	sub_82466E20(ctx, base);
	// 82621B14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621B18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621B1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621B28 size=104
    let mut pc: u32 = 0x82621B28;
    'dispatch: loop {
        match pc {
            0x82621B28 => {
    //   block [0x82621B28..0x82621B90)
	// 82621B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621B34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621B3C: 392A1AE4  addi r9, r10, 0x1ae4
	ctx.r[9].s64 = ctx.r[10].s64 + 6884;
	// 82621B40: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621B48: 38AA05D4  addi r5, r10, 0x5d4
	ctx.r[5].s64 = ctx.r[10].s64 + 1492;
	// 82621B4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621B5C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82621B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621B64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621B68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621B70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621B74: 386A0664  addi r3, r10, 0x664
	ctx.r[3].s64 = ctx.r[10].s64 + 1636;
	// 82621B78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82621B7C: 4BE452A5  bl 0x82466e20
	ctx.lr = 0x82621B80;
	sub_82466E20(ctx, base);
	// 82621B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621B90 size=108
    let mut pc: u32 = 0x82621B90;
    'dispatch: loop {
        match pc {
            0x82621B90 => {
    //   block [0x82621B90..0x82621BFC)
	// 82621B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621B9C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621BA4: 38EB1CF0  addi r7, r11, 0x1cf0
	ctx.r[7].s64 = ctx.r[11].s64 + 7408;
	// 82621BA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82621BAC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82621BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621BB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621BB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82621BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621BC0: 386A0694  addi r3, r10, 0x694
	ctx.r[3].s64 = ctx.r[10].s64 + 1684;
	// 82621BC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82621BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621BE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82621BE8: 4BE45239  bl 0x82466e20
	ctx.lr = 0x82621BEC;
	sub_82466E20(ctx, base);
	// 82621BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621C00 size=112
    let mut pc: u32 = 0x82621C00;
    'dispatch: loop {
        match pc {
            0x82621C00 => {
    //   block [0x82621C00..0x82621C70)
	// 82621C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621C0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621C10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621C14: 38AA0664  addi r5, r10, 0x664
	ctx.r[5].s64 = ctx.r[10].s64 + 1636;
	// 82621C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621C1C: 390B1D20  addi r8, r11, 0x1d20
	ctx.r[8].s64 = ctx.r[11].s64 + 7456;
	// 82621C20: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82621C24: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 82621C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621C2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621C30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621C38: 386A06C4  addi r3, r10, 0x6c4
	ctx.r[3].s64 = ctx.r[10].s64 + 1732;
	// 82621C3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621C5C: 4BE451C5  bl 0x82466e20
	ctx.lr = 0x82621C60;
	sub_82466E20(ctx, base);
	// 82621C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621C70 size=116
    let mut pc: u32 = 0x82621C70;
    'dispatch: loop {
        match pc {
            0x82621C70 => {
    //   block [0x82621C70..0x82621CE4)
	// 82621C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621C7C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621C80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621C84: 390B1DCC  addi r8, r11, 0x1dcc
	ctx.r[8].s64 = ctx.r[11].s64 + 7628;
	// 82621C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621C8C: 392A1B48  addi r9, r10, 0x1b48
	ctx.r[9].s64 = ctx.r[10].s64 + 6984;
	// 82621C90: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621C94: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82621C98: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82621C9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621CA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621CB4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82621CB8: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 82621CBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82621CC0: 386B06F4  addi r3, r11, 0x6f4
	ctx.r[3].s64 = ctx.r[11].s64 + 1780;
	// 82621CC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82621CC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621CD0: 4BE45151  bl 0x82466e20
	ctx.lr = 0x82621CD4;
	sub_82466E20(ctx, base);
	// 82621CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621CE8 size=112
    let mut pc: u32 = 0x82621CE8;
    'dispatch: loop {
        match pc {
            0x82621CE8 => {
    //   block [0x82621CE8..0x82621D58)
	// 82621CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621CF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621CF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621CFC: 38AA07B4  addi r5, r10, 0x7b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1972;
	// 82621D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621D04: 390B1DE4  addi r8, r11, 0x1de4
	ctx.r[8].s64 = ctx.r[11].s64 + 7652;
	// 82621D08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82621D0C: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 82621D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621D14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621D20: 386A0724  addi r3, r10, 0x724
	ctx.r[3].s64 = ctx.r[10].s64 + 1828;
	// 82621D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621D44: 4BE450DD  bl 0x82466e20
	ctx.lr = 0x82621D48;
	sub_82466E20(ctx, base);
	// 82621D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621D58 size=100
    let mut pc: u32 = 0x82621D58;
    'dispatch: loop {
        match pc {
            0x82621D58 => {
    //   block [0x82621D58..0x82621DBC)
	// 82621D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621D64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621D6C: 38AA0784  addi r5, r10, 0x784
	ctx.r[5].s64 = ctx.r[10].s64 + 1924;
	// 82621D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621D74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621D78: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82621D7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621D8C: 386A0754  addi r3, r10, 0x754
	ctx.r[3].s64 = ctx.r[10].s64 + 1876;
	// 82621D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621D94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621D98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621DA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621DA8: 4BE45079  bl 0x82466e20
	ctx.lr = 0x82621DAC;
	sub_82466E20(ctx, base);
	// 82621DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621DC0 size=112
    let mut pc: u32 = 0x82621DC0;
    'dispatch: loop {
        match pc {
            0x82621DC0 => {
    //   block [0x82621DC0..0x82621E30)
	// 82621DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621DCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621DD0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621DD4: 38AA07B4  addi r5, r10, 0x7b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1972;
	// 82621DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621DDC: 390B1DFC  addi r8, r11, 0x1dfc
	ctx.r[8].s64 = ctx.r[11].s64 + 7676;
	// 82621DE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82621DE4: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 82621DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621DF8: 386A0784  addi r3, r10, 0x784
	ctx.r[3].s64 = ctx.r[10].s64 + 1924;
	// 82621DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621E1C: 4BE45005  bl 0x82466e20
	ctx.lr = 0x82621E20;
	sub_82466E20(ctx, base);
	// 82621E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621E30 size=112
    let mut pc: u32 = 0x82621E30;
    'dispatch: loop {
        match pc {
            0x82621E30 => {
    //   block [0x82621E30..0x82621EA0)
	// 82621E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621E40: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82621E44: 38AA06F4  addi r5, r10, 0x6f4
	ctx.r[5].s64 = ctx.r[10].s64 + 1780;
	// 82621E48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82621E4C: 390B1E30  addi r8, r11, 0x1e30
	ctx.r[8].s64 = ctx.r[11].s64 + 7728;
	// 82621E50: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82621E54: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 82621E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621E5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82621E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621E68: 386A07B4  addi r3, r10, 0x7b4
	ctx.r[3].s64 = ctx.r[10].s64 + 1972;
	// 82621E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82621E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621E8C: 4BE44F95  bl 0x82466e20
	ctx.lr = 0x82621E90;
	sub_82466E20(ctx, base);
	// 82621E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621EA0 size=100
    let mut pc: u32 = 0x82621EA0;
    'dispatch: loop {
        match pc {
            0x82621EA0 => {
    //   block [0x82621EA0..0x82621F04)
	// 82621EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621EAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621EB4: 38AA07B4  addi r5, r10, 0x7b4
	ctx.r[5].s64 = ctx.r[10].s64 + 1972;
	// 82621EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621EC0: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82621EC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621ED4: 386A07E4  addi r3, r10, 0x7e4
	ctx.r[3].s64 = ctx.r[10].s64 + 2020;
	// 82621ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621EDC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621EE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621EE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621EF0: 4BE44F31  bl 0x82466e20
	ctx.lr = 0x82621EF4;
	sub_82466E20(ctx, base);
	// 82621EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621F08 size=100
    let mut pc: u32 = 0x82621F08;
    'dispatch: loop {
        match pc {
            0x82621F08 => {
    //   block [0x82621F08..0x82621F6C)
	// 82621F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621F1C: 38AA0724  addi r5, r10, 0x724
	ctx.r[5].s64 = ctx.r[10].s64 + 1828;
	// 82621F20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621F28: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 82621F2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621F3C: 386A0814  addi r3, r10, 0x814
	ctx.r[3].s64 = ctx.r[10].s64 + 2068;
	// 82621F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621F44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621F48: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621F50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621F58: 4BE44EC9  bl 0x82466e20
	ctx.lr = 0x82621F5C;
	sub_82466E20(ctx, base);
	// 82621F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621F70 size=100
    let mut pc: u32 = 0x82621F70;
    'dispatch: loop {
        match pc {
            0x82621F70 => {
    //   block [0x82621F70..0x82621FD4)
	// 82621F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621F7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621F84: 38AA07E4  addi r5, r10, 0x7e4
	ctx.r[5].s64 = ctx.r[10].s64 + 2020;
	// 82621F88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621F90: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82621F94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82621F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82621FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82621FA4: 386A0844  addi r3, r10, 0x844
	ctx.r[3].s64 = ctx.r[10].s64 + 2116;
	// 82621FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82621FAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82621FB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82621FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82621FB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82621FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82621FC0: 4BE44E61  bl 0x82466e20
	ctx.lr = 0x82621FC4;
	sub_82466E20(ctx, base);
	// 82621FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82621FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82621FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82621FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82621FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82621FD8 size=100
    let mut pc: u32 = 0x82621FD8;
    'dispatch: loop {
        match pc {
            0x82621FD8 => {
    //   block [0x82621FD8..0x8262203C)
	// 82621FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82621FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82621FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82621FE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82621FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82621FEC: 38AA0724  addi r5, r10, 0x724
	ctx.r[5].s64 = ctx.r[10].s64 + 1828;
	// 82621FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82621FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82621FF8: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 82621FFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262200C: 386A0874  addi r3, r10, 0x874
	ctx.r[3].s64 = ctx.r[10].s64 + 2164;
	// 82622010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622014: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622018: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262201C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622020: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82622024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622028: 4BE44DF9  bl 0x82466e20
	ctx.lr = 0x8262202C;
	sub_82466E20(ctx, base);
	// 8262202C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622040 size=112
    let mut pc: u32 = 0x82622040;
    'dispatch: loop {
        match pc {
            0x82622040 => {
    //   block [0x82622040..0x826220B0)
	// 82622040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262204C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622050: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622054: 38AA0904  addi r5, r10, 0x904
	ctx.r[5].s64 = ctx.r[10].s64 + 2308;
	// 82622058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262205C: 390B1ED8  addi r8, r11, 0x1ed8
	ctx.r[8].s64 = ctx.r[11].s64 + 7896;
	// 82622060: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82622064: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82622068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262206C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622078: 386A08A4  addi r3, r10, 0x8a4
	ctx.r[3].s64 = ctx.r[10].s64 + 2212;
	// 8262207C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262208C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262209C: 4BE44D85  bl 0x82466e20
	ctx.lr = 0x826220A0;
	sub_82466E20(ctx, base);
	// 826220A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826220A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826220A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826220AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826220B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826220B0 size=112
    let mut pc: u32 = 0x826220B0;
    'dispatch: loop {
        match pc {
            0x826220B0 => {
    //   block [0x826220B0..0x82622120)
	// 826220B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826220B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826220B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826220BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826220C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826220C4: 38AA0934  addi r5, r10, 0x934
	ctx.r[5].s64 = ctx.r[10].s64 + 2356;
	// 826220C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826220CC: 390B1F08  addi r8, r11, 0x1f08
	ctx.r[8].s64 = ctx.r[11].s64 + 7944;
	// 826220D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826220D4: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826220D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826220DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826220E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826220E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826220E8: 386A08D4  addi r3, r10, 0x8d4
	ctx.r[3].s64 = ctx.r[10].s64 + 2260;
	// 826220EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826220F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826220F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826220F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826220FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262210C: 4BE44D15  bl 0x82466e20
	ctx.lr = 0x82622110;
	sub_82466E20(ctx, base);
	// 82622110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262211C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622120 size=112
    let mut pc: u32 = 0x82622120;
    'dispatch: loop {
        match pc {
            0x82622120 => {
    //   block [0x82622120..0x82622190)
	// 82622120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262212C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622130: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622134: 38AA0A24  addi r5, r10, 0xa24
	ctx.r[5].s64 = ctx.r[10].s64 + 2596;
	// 82622138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262213C: 390B1F20  addi r8, r11, 0x1f20
	ctx.r[8].s64 = ctx.r[11].s64 + 7968;
	// 82622140: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82622144: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82622148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262214C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622158: 386A0904  addi r3, r10, 0x904
	ctx.r[3].s64 = ctx.r[10].s64 + 2308;
	// 8262215C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262216C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262217C: 4BE44CA5  bl 0x82466e20
	ctx.lr = 0x82622180;
	sub_82466E20(ctx, base);
	// 82622180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262218C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622190 size=112
    let mut pc: u32 = 0x82622190;
    'dispatch: loop {
        match pc {
            0x82622190 => {
    //   block [0x82622190..0x82622200)
	// 82622190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262219C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826221A0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826221A4: 38AA0904  addi r5, r10, 0x904
	ctx.r[5].s64 = ctx.r[10].s64 + 2308;
	// 826221A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826221AC: 390B1F50  addi r8, r11, 0x1f50
	ctx.r[8].s64 = ctx.r[11].s64 + 8016;
	// 826221B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826221B4: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826221B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826221BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826221C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826221C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826221C8: 386A0934  addi r3, r10, 0x934
	ctx.r[3].s64 = ctx.r[10].s64 + 2356;
	// 826221CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826221D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826221D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826221D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826221DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826221E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826221E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826221E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826221EC: 4BE44C35  bl 0x82466e20
	ctx.lr = 0x826221F0;
	sub_82466E20(ctx, base);
	// 826221F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826221F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826221F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826221FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622200 size=112
    let mut pc: u32 = 0x82622200;
    'dispatch: loop {
        match pc {
            0x82622200 => {
    //   block [0x82622200..0x82622270)
	// 82622200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262220C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622210: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622214: 38AA0934  addi r5, r10, 0x934
	ctx.r[5].s64 = ctx.r[10].s64 + 2356;
	// 82622218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262221C: 390B1F68  addi r8, r11, 0x1f68
	ctx.r[8].s64 = ctx.r[11].s64 + 8040;
	// 82622220: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622224: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 82622228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262222C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622238: 386A0964  addi r3, r10, 0x964
	ctx.r[3].s64 = ctx.r[10].s64 + 2404;
	// 8262223C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262224C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262225C: 4BE44BC5  bl 0x82466e20
	ctx.lr = 0x82622260;
	sub_82466E20(ctx, base);
	// 82622260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262226C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622270 size=112
    let mut pc: u32 = 0x82622270;
    'dispatch: loop {
        match pc {
            0x82622270 => {
    //   block [0x82622270..0x826222E0)
	// 82622270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262227C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82622280: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622284: 392A1B74  addi r9, r10, 0x1b74
	ctx.r[9].s64 = ctx.r[10].s64 + 7028;
	// 82622288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262228C: 390B1F80  addi r8, r11, 0x1f80
	ctx.r[8].s64 = ctx.r[11].s64 + 8064;
	// 82622290: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82622294: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82622298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262229C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826222A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826222A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826222A8: 386A0994  addi r3, r10, 0x994
	ctx.r[3].s64 = ctx.r[10].s64 + 2452;
	// 826222AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826222B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826222B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826222B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826222BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826222C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826222C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826222C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826222CC: 4BE44B55  bl 0x82466e20
	ctx.lr = 0x826222D0;
	sub_82466E20(ctx, base);
	// 826222D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826222D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826222D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826222DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826222E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826222E0 size=112
    let mut pc: u32 = 0x826222E0;
    'dispatch: loop {
        match pc {
            0x826222E0 => {
    //   block [0x826222E0..0x82622350)
	// 826222E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826222E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826222E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826222EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826222F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826222F4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826222F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826222FC: 390B1FB0  addi r8, r11, 0x1fb0
	ctx.r[8].s64 = ctx.r[11].s64 + 8112;
	// 82622300: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82622304: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82622308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262230C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622318: 386A09C4  addi r3, r10, 0x9c4
	ctx.r[3].s64 = ctx.r[10].s64 + 2500;
	// 8262231C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262232C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262233C: 4BE44AE5  bl 0x82466e20
	ctx.lr = 0x82622340;
	sub_82466E20(ctx, base);
	// 82622340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262234C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82622350 size=48
    let mut pc: u32 = 0x82622350;
    'dispatch: loop {
        match pc {
            0x82622350 => {
    //   block [0x82622350..0x82622380)
	// 82622350: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622354: 814B204C  lwz r10, 0x204c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8268 as u32) ) } as u64;
	// 82622358: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262235C: 396B3F90  addi r11, r11, 0x3f90
	ctx.r[11].s64 = ctx.r[11].s64 + 16272;
	// 82622360: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82622364: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82622368: 814A2048  lwz r10, 0x2048(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8264 as u32) ) } as u64;
	// 8262236C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82622370: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82622374: 814A2044  lwz r10, 0x2044(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8260 as u32) ) } as u64;
	// 82622378: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 8262237C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622380 size=116
    let mut pc: u32 = 0x82622380;
    'dispatch: loop {
        match pc {
            0x82622380 => {
    //   block [0x82622380..0x826223F4)
	// 82622380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262238C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82622390: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622394: 392B1C68  addi r9, r11, 0x1c68
	ctx.r[9].s64 = ctx.r[11].s64 + 7272;
	// 82622398: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 8262239C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826223A0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826223A4: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 826223A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826223AC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826223B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826223B4: 396B3F90  addi r11, r11, 0x3f90
	ctx.r[11].s64 = ctx.r[11].s64 + 16272;
	// 826223B8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826223BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826223C0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826223C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826223C8: 386A09F4  addi r3, r10, 0x9f4
	ctx.r[3].s64 = ctx.r[10].s64 + 2548;
	// 826223CC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826223D0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826223D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826223D8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826223DC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826223E0: 4BE44A41  bl 0x82466e20
	ctx.lr = 0x826223E4;
	sub_82466E20(ctx, base);
	// 826223E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826223E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826223EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826223F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826223F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826223F8 size=116
    let mut pc: u32 = 0x826223F8;
    'dispatch: loop {
        match pc {
            0x826223F8 => {
    //   block [0x826223F8..0x8262246C)
	// 826223F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826223FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622404: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262240C: 390B2058  addi r8, r11, 0x2058
	ctx.r[8].s64 = ctx.r[11].s64 + 8280;
	// 82622410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622414: 392A1D50  addi r9, r10, 0x1d50
	ctx.r[9].s64 = ctx.r[10].s64 + 7504;
	// 82622418: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262241C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82622420: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82622424: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262242C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262243C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82622440: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82622444: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82622448: 386B0A24  addi r3, r11, 0xa24
	ctx.r[3].s64 = ctx.r[11].s64 + 2596;
	// 8262244C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82622450: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622458: 4BE449C9  bl 0x82466e20
	ctx.lr = 0x8262245C;
	sub_82466E20(ctx, base);
	// 8262245C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622470 size=112
    let mut pc: u32 = 0x82622470;
    'dispatch: loop {
        match pc {
            0x82622470 => {
    //   block [0x82622470..0x826224E0)
	// 82622470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262247C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622480: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622484: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82622488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262248C: 390B20D0  addi r8, r11, 0x20d0
	ctx.r[8].s64 = ctx.r[11].s64 + 8400;
	// 82622490: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622494: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82622498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262249C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826224A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826224A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826224A8: 386A0A54  addi r3, r10, 0xa54
	ctx.r[3].s64 = ctx.r[10].s64 + 2644;
	// 826224AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826224B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826224B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826224B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826224BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826224C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826224C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826224C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826224CC: 4BE44955  bl 0x82466e20
	ctx.lr = 0x826224D0;
	sub_82466E20(ctx, base);
	// 826224D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826224D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826224D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826224DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826224E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826224E0 size=112
    let mut pc: u32 = 0x826224E0;
    'dispatch: loop {
        match pc {
            0x826224E0 => {
    //   block [0x826224E0..0x82622550)
	// 826224E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826224E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826224E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826224EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826224F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826224F4: 38AAF224  addi r5, r10, -0xddc
	ctx.r[5].s64 = ctx.r[10].s64 + -3548;
	// 826224F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826224FC: 390B20E8  addi r8, r11, 0x20e8
	ctx.r[8].s64 = ctx.r[11].s64 + 8424;
	// 82622500: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622504: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82622508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262250C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622518: 386A0A84  addi r3, r10, 0xa84
	ctx.r[3].s64 = ctx.r[10].s64 + 2692;
	// 8262251C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262252C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262253C: 4BE448E5  bl 0x82466e20
	ctx.lr = 0x82622540;
	sub_82466E20(ctx, base);
	// 82622540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262254C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622550 size=108
    let mut pc: u32 = 0x82622550;
    'dispatch: loop {
        match pc {
            0x82622550 => {
    //   block [0x82622550..0x826225BC)
	// 82622550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262255C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622564: 38EB2100  addi r7, r11, 0x2100
	ctx.r[7].s64 = ctx.r[11].s64 + 8448;
	// 82622568: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262256C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82622570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622574: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262257C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622580: 386A0AB4  addi r3, r10, 0xab4
	ctx.r[3].s64 = ctx.r[10].s64 + 2740;
	// 82622584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262258C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262259C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826225A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826225A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826225A8: 4BE44879  bl 0x82466e20
	ctx.lr = 0x826225AC;
	sub_82466E20(ctx, base);
	// 826225AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826225B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826225B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826225B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826225C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826225C0 size=112
    let mut pc: u32 = 0x826225C0;
    'dispatch: loop {
        match pc {
            0x826225C0 => {
    //   block [0x826225C0..0x82622630)
	// 826225C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826225C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826225C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826225CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826225D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826225D4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826225D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826225DC: 390B2118  addi r8, r11, 0x2118
	ctx.r[8].s64 = ctx.r[11].s64 + 8472;
	// 826225E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826225E4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826225E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826225EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826225F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826225F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826225F8: 386A0AE4  addi r3, r10, 0xae4
	ctx.r[3].s64 = ctx.r[10].s64 + 2788;
	// 826225FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262260C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262261C: 4BE44805  bl 0x82466e20
	ctx.lr = 0x82622620;
	sub_82466E20(ctx, base);
	// 82622620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262262C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622630 size=108
    let mut pc: u32 = 0x82622630;
    'dispatch: loop {
        match pc {
            0x82622630 => {
    //   block [0x82622630..0x8262269C)
	// 82622630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262263C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622644: 38EB2160  addi r7, r11, 0x2160
	ctx.r[7].s64 = ctx.r[11].s64 + 8544;
	// 82622648: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262264C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82622650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622654: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262265C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622660: 386A0B14  addi r3, r10, 0xb14
	ctx.r[3].s64 = ctx.r[10].s64 + 2836;
	// 82622664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262266C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262267C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622688: 4BE44799  bl 0x82466e20
	ctx.lr = 0x8262268C;
	sub_82466E20(ctx, base);
	// 8262268C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826226A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826226A0 size=112
    let mut pc: u32 = 0x826226A0;
    'dispatch: loop {
        match pc {
            0x826226A0 => {
    //   block [0x826226A0..0x82622710)
	// 826226A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826226A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826226A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826226AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826226B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826226B4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826226B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826226BC: 390B2178  addi r8, r11, 0x2178
	ctx.r[8].s64 = ctx.r[11].s64 + 8568;
	// 826226C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826226C4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826226C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826226CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826226D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826226D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826226D8: 386A0B44  addi r3, r10, 0xb44
	ctx.r[3].s64 = ctx.r[10].s64 + 2884;
	// 826226DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826226E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826226E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826226E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826226EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826226F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826226F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826226F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826226FC: 4BE44725  bl 0x82466e20
	ctx.lr = 0x82622700;
	sub_82466E20(ctx, base);
	// 82622700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262270C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622710 size=112
    let mut pc: u32 = 0x82622710;
    'dispatch: loop {
        match pc {
            0x82622710 => {
    //   block [0x82622710..0x82622780)
	// 82622710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262271C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622720: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622724: 38AA0C04  addi r5, r10, 0xc04
	ctx.r[5].s64 = ctx.r[10].s64 + 3076;
	// 82622728: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262272C: 390B21A8  addi r8, r11, 0x21a8
	ctx.r[8].s64 = ctx.r[11].s64 + 8616;
	// 82622730: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82622734: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 82622738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262273C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622748: 386A0B74  addi r3, r10, 0xb74
	ctx.r[3].s64 = ctx.r[10].s64 + 2932;
	// 8262274C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262275C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262276C: 4BE446B5  bl 0x82466e20
	ctx.lr = 0x82622770;
	sub_82466E20(ctx, base);
	// 82622770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262277C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622780 size=108
    let mut pc: u32 = 0x82622780;
    'dispatch: loop {
        match pc {
            0x82622780 => {
    //   block [0x82622780..0x826227EC)
	// 82622780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262278C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622790: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82622794: 38EB2220  addi r7, r11, 0x2220
	ctx.r[7].s64 = ctx.r[11].s64 + 8736;
	// 82622798: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262279C: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 826227A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826227A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826227A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826227AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826227B0: 386A0BA4  addi r3, r10, 0xba4
	ctx.r[3].s64 = ctx.r[10].s64 + 2980;
	// 826227B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826227B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826227BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826227C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826227C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826227C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826227CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826227D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826227D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826227D8: 4BE44649  bl 0x82466e20
	ctx.lr = 0x826227DC;
	sub_82466E20(ctx, base);
	// 826227DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826227E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826227E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826227E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826227F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826227F0 size=108
    let mut pc: u32 = 0x826227F0;
    'dispatch: loop {
        match pc {
            0x826227F0 => {
    //   block [0x826227F0..0x8262285C)
	// 826227F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826227F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826227F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826227FC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622800: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82622804: 38EB2268  addi r7, r11, 0x2268
	ctx.r[7].s64 = ctx.r[11].s64 + 8808;
	// 82622808: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262280C: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 82622810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622818: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262281C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622820: 386A0BD4  addi r3, r10, 0xbd4
	ctx.r[3].s64 = ctx.r[10].s64 + 3028;
	// 82622824: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262282C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262283C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622848: 4BE445D9  bl 0x82466e20
	ctx.lr = 0x8262284C;
	sub_82466E20(ctx, base);
	// 8262284C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622860 size=116
    let mut pc: u32 = 0x82622860;
    'dispatch: loop {
        match pc {
            0x82622860 => {
    //   block [0x82622860..0x826228D4)
	// 82622860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262286C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82622870: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82622874: 390A22B0  addi r8, r10, 0x22b0
	ctx.r[8].s64 = ctx.r[10].s64 + 8880;
	// 82622878: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262287C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82622880: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 82622884: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622888: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262288C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622894: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82622898: 396B1D64  addi r11, r11, 0x1d64
	ctx.r[11].s64 = ctx.r[11].s64 + 7524;
	// 8262289C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826228A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826228A4: 386A0C04  addi r3, r10, 0xc04
	ctx.r[3].s64 = ctx.r[10].s64 + 3076;
	// 826228A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826228AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826228B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826228B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826228B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826228BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826228C0: 4BE44561  bl 0x82466e20
	ctx.lr = 0x826228C4;
	sub_82466E20(ctx, base);
	// 826228C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826228C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826228CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826228D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826228D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826228D8 size=108
    let mut pc: u32 = 0x826228D8;
    'dispatch: loop {
        match pc {
            0x826228D8 => {
    //   block [0x826228D8..0x82622944)
	// 826228D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826228DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826228E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826228E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826228E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826228EC: 38EB2388  addi r7, r11, 0x2388
	ctx.r[7].s64 = ctx.r[11].s64 + 9096;
	// 826228F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826228F4: 388A24D4  addi r4, r10, 0x24d4
	ctx.r[4].s64 = ctx.r[10].s64 + 9428;
	// 826228F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826228FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82622904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622908: 386A0C34  addi r3, r10, 0xc34
	ctx.r[3].s64 = ctx.r[10].s64 + 3124;
	// 8262290C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262291C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262292C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622930: 4BE444F1  bl 0x82466e20
	ctx.lr = 0x82622934;
	sub_82466E20(ctx, base);
	// 82622934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262293C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622948 size=108
    let mut pc: u32 = 0x82622948;
    'dispatch: loop {
        match pc {
            0x82622948 => {
    //   block [0x82622948..0x826229B4)
	// 82622948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262294C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622954: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262295C: 38EB23B8  addi r7, r11, 0x23b8
	ctx.r[7].s64 = ctx.r[11].s64 + 9144;
	// 82622960: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82622964: 388A24F8  addi r4, r10, 0x24f8
	ctx.r[4].s64 = ctx.r[10].s64 + 9464;
	// 82622968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262296C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622970: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82622974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622978: 386A0C64  addi r3, r10, 0xc64
	ctx.r[3].s64 = ctx.r[10].s64 + 3172;
	// 8262297C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262298C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262299C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826229A0: 4BE44481  bl 0x82466e20
	ctx.lr = 0x826229A4;
	sub_82466E20(ctx, base);
	// 826229A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826229A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826229AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826229B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826229B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826229B8 size=112
    let mut pc: u32 = 0x826229B8;
    'dispatch: loop {
        match pc {
            0x826229B8 => {
    //   block [0x826229B8..0x82622A28)
	// 826229B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826229BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826229C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826229C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826229C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826229CC: 38AAFD04  addi r5, r10, -0x2fc
	ctx.r[5].s64 = ctx.r[10].s64 + -764;
	// 826229D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826229D4: 390B23E8  addi r8, r11, 0x23e8
	ctx.r[8].s64 = ctx.r[11].s64 + 9192;
	// 826229D8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826229DC: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826229E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826229E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826229E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826229EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826229F0: 386A0C94  addi r3, r10, 0xc94
	ctx.r[3].s64 = ctx.r[10].s64 + 3220;
	// 826229F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826229F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826229FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622A14: 4BE4440D  bl 0x82466e20
	ctx.lr = 0x82622A18;
	sub_82466E20(ctx, base);
	// 82622A18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622A28 size=112
    let mut pc: u32 = 0x82622A28;
    'dispatch: loop {
        match pc {
            0x82622A28 => {
    //   block [0x82622A28..0x82622A98)
	// 82622A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622A34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622A38: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622A3C: 38AAFC14  addi r5, r10, -0x3ec
	ctx.r[5].s64 = ctx.r[10].s64 + -1004;
	// 82622A40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622A44: 390B24C0  addi r8, r11, 0x24c0
	ctx.r[8].s64 = ctx.r[11].s64 + 9408;
	// 82622A48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82622A4C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82622A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622A54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622A58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622A60: 386A0CC4  addi r3, r10, 0xcc4
	ctx.r[3].s64 = ctx.r[10].s64 + 3268;
	// 82622A64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622A84: 4BE4439D  bl 0x82466e20
	ctx.lr = 0x82622A88;
	sub_82466E20(ctx, base);
	// 82622A88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622A98 size=112
    let mut pc: u32 = 0x82622A98;
    'dispatch: loop {
        match pc {
            0x82622A98 => {
    //   block [0x82622A98..0x82622B08)
	// 82622A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622AA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622AA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622AAC: 38AAFC14  addi r5, r10, -0x3ec
	ctx.r[5].s64 = ctx.r[10].s64 + -1004;
	// 82622AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622AB4: 390B2508  addi r8, r11, 0x2508
	ctx.r[8].s64 = ctx.r[11].s64 + 9480;
	// 82622AB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82622ABC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 82622AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622AC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622AD0: 386A0CF4  addi r3, r10, 0xcf4
	ctx.r[3].s64 = ctx.r[10].s64 + 3316;
	// 82622AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622AF4: 4BE4432D  bl 0x82466e20
	ctx.lr = 0x82622AF8;
	sub_82466E20(ctx, base);
	// 82622AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622B08 size=112
    let mut pc: u32 = 0x82622B08;
    'dispatch: loop {
        match pc {
            0x82622B08 => {
    //   block [0x82622B08..0x82622B78)
	// 82622B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622B14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622B18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622B1C: 38AAFC44  addi r5, r10, -0x3bc
	ctx.r[5].s64 = ctx.r[10].s64 + -956;
	// 82622B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622B24: 390B2568  addi r8, r11, 0x2568
	ctx.r[8].s64 = ctx.r[11].s64 + 9576;
	// 82622B28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82622B2C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82622B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622B34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622B38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622B40: 386A0D24  addi r3, r10, 0xd24
	ctx.r[3].s64 = ctx.r[10].s64 + 3364;
	// 82622B44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622B54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622B64: 4BE442BD  bl 0x82466e20
	ctx.lr = 0x82622B68;
	sub_82466E20(ctx, base);
	// 82622B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622B78 size=112
    let mut pc: u32 = 0x82622B78;
    'dispatch: loop {
        match pc {
            0x82622B78 => {
    //   block [0x82622B78..0x82622BE8)
	// 82622B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622B84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622B88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622B8C: 38AAFC44  addi r5, r10, -0x3bc
	ctx.r[5].s64 = ctx.r[10].s64 + -956;
	// 82622B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622B94: 390B25C8  addi r8, r11, 0x25c8
	ctx.r[8].s64 = ctx.r[11].s64 + 9672;
	// 82622B98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82622B9C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82622BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622BA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622BB0: 386A0D54  addi r3, r10, 0xd54
	ctx.r[3].s64 = ctx.r[10].s64 + 3412;
	// 82622BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622BD4: 4BE4424D  bl 0x82466e20
	ctx.lr = 0x82622BD8;
	sub_82466E20(ctx, base);
	// 82622BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622BE8 size=112
    let mut pc: u32 = 0x82622BE8;
    'dispatch: loop {
        match pc {
            0x82622BE8 => {
    //   block [0x82622BE8..0x82622C58)
	// 82622BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622BF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622BF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622BFC: 38AAFC14  addi r5, r10, -0x3ec
	ctx.r[5].s64 = ctx.r[10].s64 + -1004;
	// 82622C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622C04: 390B2628  addi r8, r11, 0x2628
	ctx.r[8].s64 = ctx.r[11].s64 + 9768;
	// 82622C08: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82622C0C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 82622C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622C14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622C20: 386A0D84  addi r3, r10, 0xd84
	ctx.r[3].s64 = ctx.r[10].s64 + 3460;
	// 82622C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622C44: 4BE441DD  bl 0x82466e20
	ctx.lr = 0x82622C48;
	sub_82466E20(ctx, base);
	// 82622C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622C58 size=108
    let mut pc: u32 = 0x82622C58;
    'dispatch: loop {
        match pc {
            0x82622C58 => {
    //   block [0x82622C58..0x82622CC4)
	// 82622C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622C64: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622C6C: 38EB26E8  addi r7, r11, 0x26e8
	ctx.r[7].s64 = ctx.r[11].s64 + 9960;
	// 82622C70: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82622C74: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82622C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622C7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82622C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622C88: 386A0DB4  addi r3, r10, 0xdb4
	ctx.r[3].s64 = ctx.r[10].s64 + 3508;
	// 82622C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622CB0: 4BE44171  bl 0x82466e20
	ctx.lr = 0x82622CB4;
	sub_82466E20(ctx, base);
	// 82622CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622CC8 size=112
    let mut pc: u32 = 0x82622CC8;
    'dispatch: loop {
        match pc {
            0x82622CC8 => {
    //   block [0x82622CC8..0x82622D38)
	// 82622CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622CD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622CD8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622CDC: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 82622CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622CE4: 390B2880  addi r8, r11, 0x2880
	ctx.r[8].s64 = ctx.r[11].s64 + 10368;
	// 82622CE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622CEC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 82622CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622CF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622D00: 386A0DE4  addi r3, r10, 0xde4
	ctx.r[3].s64 = ctx.r[10].s64 + 3556;
	// 82622D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622D24: 4BE440FD  bl 0x82466e20
	ctx.lr = 0x82622D28;
	sub_82466E20(ctx, base);
	// 82622D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622D38 size=112
    let mut pc: u32 = 0x82622D38;
    'dispatch: loop {
        match pc {
            0x82622D38 => {
    //   block [0x82622D38..0x82622DA8)
	// 82622D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622D44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622D48: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622D4C: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 82622D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622D54: 390B2898  addi r8, r11, 0x2898
	ctx.r[8].s64 = ctx.r[11].s64 + 10392;
	// 82622D58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622D5C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82622D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622D64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622D70: 386A0E14  addi r3, r10, 0xe14
	ctx.r[3].s64 = ctx.r[10].s64 + 3604;
	// 82622D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622D84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82622D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622D94: 4BE4408D  bl 0x82466e20
	ctx.lr = 0x82622D98;
	sub_82466E20(ctx, base);
	// 82622D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622DA8 size=112
    let mut pc: u32 = 0x82622DA8;
    'dispatch: loop {
        match pc {
            0x82622DA8 => {
    //   block [0x82622DA8..0x82622E18)
	// 82622DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622DB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622DB8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622DBC: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 82622DC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622DC4: 390B28B0  addi r8, r11, 0x28b0
	ctx.r[8].s64 = ctx.r[11].s64 + 10416;
	// 82622DC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82622DCC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 82622DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622DD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622DE0: 386A0E44  addi r3, r10, 0xe44
	ctx.r[3].s64 = ctx.r[10].s64 + 3652;
	// 82622DE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622E04: 4BE4401D  bl 0x82466e20
	ctx.lr = 0x82622E08;
	sub_82466E20(ctx, base);
	// 82622E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622E18 size=108
    let mut pc: u32 = 0x82622E18;
    'dispatch: loop {
        match pc {
            0x82622E18 => {
    //   block [0x82622E18..0x82622E84)
	// 82622E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622E24: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622E2C: 38EB28E0  addi r7, r11, 0x28e0
	ctx.r[7].s64 = ctx.r[11].s64 + 10464;
	// 82622E30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82622E34: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82622E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622E40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82622E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622E48: 386A0E74  addi r3, r10, 0xe74
	ctx.r[3].s64 = ctx.r[10].s64 + 3700;
	// 82622E4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622E6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622E70: 4BE43FB1  bl 0x82466e20
	ctx.lr = 0x82622E74;
	sub_82466E20(ctx, base);
	// 82622E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622E88 size=112
    let mut pc: u32 = 0x82622E88;
    'dispatch: loop {
        match pc {
            0x82622E88 => {
    //   block [0x82622E88..0x82622EF8)
	// 82622E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622E94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622E98: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622E9C: 38AAF2E4  addi r5, r10, -0xd1c
	ctx.r[5].s64 = ctx.r[10].s64 + -3356;
	// 82622EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622EA4: 390B2910  addi r8, r11, 0x2910
	ctx.r[8].s64 = ctx.r[11].s64 + 10512;
	// 82622EA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82622EAC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82622EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622EB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622EC0: 386A0EA4  addi r3, r10, 0xea4
	ctx.r[3].s64 = ctx.r[10].s64 + 3748;
	// 82622EC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622ED4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82622ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622EE4: 4BE43F3D  bl 0x82466e20
	ctx.lr = 0x82622EE8;
	sub_82466E20(ctx, base);
	// 82622EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622EF8 size=112
    let mut pc: u32 = 0x82622EF8;
    'dispatch: loop {
        match pc {
            0x82622EF8 => {
    //   block [0x82622EF8..0x82622F68)
	// 82622EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622F04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622F08: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622F0C: 38AAFC44  addi r5, r10, -0x3bc
	ctx.r[5].s64 = ctx.r[10].s64 + -956;
	// 82622F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622F14: 390B2928  addi r8, r11, 0x2928
	ctx.r[8].s64 = ctx.r[11].s64 + 10536;
	// 82622F18: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82622F1C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82622F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622F24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622F28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82622F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622F30: 386A0ED4  addi r3, r10, 0xed4
	ctx.r[3].s64 = ctx.r[10].s64 + 3796;
	// 82622F34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82622F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622F54: 4BE43ECD  bl 0x82466e20
	ctx.lr = 0x82622F58;
	sub_82466E20(ctx, base);
	// 82622F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622F68 size=108
    let mut pc: u32 = 0x82622F68;
    'dispatch: loop {
        match pc {
            0x82622F68 => {
    //   block [0x82622F68..0x82622FD4)
	// 82622F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622F74: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622F78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622F7C: 38EB29B8  addi r7, r11, 0x29b8
	ctx.r[7].s64 = ctx.r[11].s64 + 10680;
	// 82622F80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82622F84: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 82622F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622F8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82622F90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82622F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82622F98: 386A0F04  addi r3, r10, 0xf04
	ctx.r[3].s64 = ctx.r[10].s64 + 3844;
	// 82622F9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82622FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82622FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82622FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82622FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82622FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82622FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82622FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82622FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82622FC0: 4BE43E61  bl 0x82466e20
	ctx.lr = 0x82622FC4;
	sub_82466E20(ctx, base);
	// 82622FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82622FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82622FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82622FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82622FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82622FD8 size=108
    let mut pc: u32 = 0x82622FD8;
    'dispatch: loop {
        match pc {
            0x82622FD8 => {
    //   block [0x82622FD8..0x82623044)
	// 82622FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82622FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82622FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82622FE4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82622FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82622FEC: 38EB2A00  addi r7, r11, 0x2a00
	ctx.r[7].s64 = ctx.r[11].s64 + 10752;
	// 82622FF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82622FF4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82622FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82622FFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82623004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623008: 386A0F34  addi r3, r10, 0xf34
	ctx.r[3].s64 = ctx.r[10].s64 + 3892;
	// 8262300C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262301C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262302C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623030: 4BE43DF1  bl 0x82466e20
	ctx.lr = 0x82623034;
	sub_82466E20(ctx, base);
	// 82623034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262303C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623048 size=108
    let mut pc: u32 = 0x82623048;
    'dispatch: loop {
        match pc {
            0x82623048 => {
    //   block [0x82623048..0x826230B4)
	// 82623048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262304C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623054: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262305C: 38EB2A30  addi r7, r11, 0x2a30
	ctx.r[7].s64 = ctx.r[11].s64 + 10800;
	// 82623060: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82623064: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 82623068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262306C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82623074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623078: 386A0F64  addi r3, r10, 0xf64
	ctx.r[3].s64 = ctx.r[10].s64 + 3940;
	// 8262307C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262308C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262309C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826230A0: 4BE43D81  bl 0x82466e20
	ctx.lr = 0x826230A4;
	sub_82466E20(ctx, base);
	// 826230A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826230A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826230AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826230B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826230B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826230B8 size=112
    let mut pc: u32 = 0x826230B8;
    'dispatch: loop {
        match pc {
            0x826230B8 => {
    //   block [0x826230B8..0x82623128)
	// 826230B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826230BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826230C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826230C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826230C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826230CC: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826230D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826230D4: 390B2A60  addi r8, r11, 0x2a60
	ctx.r[8].s64 = ctx.r[11].s64 + 10848;
	// 826230D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826230DC: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826230E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826230E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826230E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826230EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826230F0: 386A0F94  addi r3, r10, 0xf94
	ctx.r[3].s64 = ctx.r[10].s64 + 3988;
	// 826230F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826230F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826230FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262310C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623114: 4BE43D0D  bl 0x82466e20
	ctx.lr = 0x82623118;
	sub_82466E20(ctx, base);
	// 82623118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262311C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623128 size=112
    let mut pc: u32 = 0x82623128;
    'dispatch: loop {
        match pc {
            0x82623128 => {
    //   block [0x82623128..0x82623198)
	// 82623128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262312C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623134: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623138: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262313C: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623140: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623144: 390B2A90  addi r8, r11, 0x2a90
	ctx.r[8].s64 = ctx.r[11].s64 + 10896;
	// 82623148: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262314C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82623150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623154: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623158: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262315C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623160: 386A0FC4  addi r3, r10, 0xfc4
	ctx.r[3].s64 = ctx.r[10].s64 + 4036;
	// 82623164: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262316C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262317C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623184: 4BE43C9D  bl 0x82466e20
	ctx.lr = 0x82623188;
	sub_82466E20(ctx, base);
	// 82623188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262318C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623198 size=112
    let mut pc: u32 = 0x82623198;
    'dispatch: loop {
        match pc {
            0x82623198 => {
    //   block [0x82623198..0x82623208)
	// 82623198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262319C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826231A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826231A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826231A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826231AC: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826231B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826231B4: 390B2AA8  addi r8, r11, 0x2aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 10920;
	// 826231B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826231BC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826231C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826231C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826231C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826231CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826231D0: 386A0FF4  addi r3, r10, 0xff4
	ctx.r[3].s64 = ctx.r[10].s64 + 4084;
	// 826231D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826231D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826231DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826231E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826231E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826231E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826231EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826231F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826231F4: 4BE43C2D  bl 0x82466e20
	ctx.lr = 0x826231F8;
	sub_82466E20(ctx, base);
	// 826231F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826231FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623208 size=108
    let mut pc: u32 = 0x82623208;
    'dispatch: loop {
        match pc {
            0x82623208 => {
    //   block [0x82623208..0x82623274)
	// 82623208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262320C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623214: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262321C: 38EB2AC0  addi r7, r11, 0x2ac0
	ctx.r[7].s64 = ctx.r[11].s64 + 10944;
	// 82623220: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82623224: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82623228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262322C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623230: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82623234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623238: 386A1024  addi r3, r10, 0x1024
	ctx.r[3].s64 = ctx.r[10].s64 + 4132;
	// 8262323C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262324C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262325C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623260: 4BE43BC1  bl 0x82466e20
	ctx.lr = 0x82623264;
	sub_82466E20(ctx, base);
	// 82623264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262326C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623278 size=112
    let mut pc: u32 = 0x82623278;
    'dispatch: loop {
        match pc {
            0x82623278 => {
    //   block [0x82623278..0x826232E8)
	// 82623278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262327C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623288: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262328C: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623294: 390B2AF0  addi r8, r11, 0x2af0
	ctx.r[8].s64 = ctx.r[11].s64 + 10992;
	// 82623298: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262329C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826232A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826232A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826232A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826232AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826232B0: 386A1054  addi r3, r10, 0x1054
	ctx.r[3].s64 = ctx.r[10].s64 + 4180;
	// 826232B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826232B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826232BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826232C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826232C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826232C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826232CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826232D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826232D4: 4BE43B4D  bl 0x82466e20
	ctx.lr = 0x826232D8;
	sub_82466E20(ctx, base);
	// 826232D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826232DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826232E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826232E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826232E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826232E8 size=108
    let mut pc: u32 = 0x826232E8;
    'dispatch: loop {
        match pc {
            0x826232E8 => {
    //   block [0x826232E8..0x82623354)
	// 826232E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826232EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826232F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826232F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826232F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826232FC: 38EB2B08  addi r7, r11, 0x2b08
	ctx.r[7].s64 = ctx.r[11].s64 + 11016;
	// 82623300: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82623304: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82623308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262330C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623310: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82623314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623318: 386A1084  addi r3, r10, 0x1084
	ctx.r[3].s64 = ctx.r[10].s64 + 4228;
	// 8262331C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262332C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262333C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623340: 4BE43AE1  bl 0x82466e20
	ctx.lr = 0x82623344;
	sub_82466E20(ctx, base);
	// 82623344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262334C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623358 size=112
    let mut pc: u32 = 0x82623358;
    'dispatch: loop {
        match pc {
            0x82623358 => {
    //   block [0x82623358..0x826233C8)
	// 82623358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262335C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623364: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623368: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262336C: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623374: 390B2BE0  addi r8, r11, 0x2be0
	ctx.r[8].s64 = ctx.r[11].s64 + 11232;
	// 82623378: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 8262337C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82623380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623384: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262338C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623390: 386A10B4  addi r3, r10, 0x10b4
	ctx.r[3].s64 = ctx.r[10].s64 + 4276;
	// 82623394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262339C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826233A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826233A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826233A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826233AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826233B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826233B4: 4BE43A6D  bl 0x82466e20
	ctx.lr = 0x826233B8;
	sub_82466E20(ctx, base);
	// 826233B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826233BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826233C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826233C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826233C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826233C8 size=108
    let mut pc: u32 = 0x826233C8;
    'dispatch: loop {
        match pc {
            0x826233C8 => {
    //   block [0x826233C8..0x82623434)
	// 826233C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826233CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826233D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826233D4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826233D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826233DC: 38EB2D90  addi r7, r11, 0x2d90
	ctx.r[7].s64 = ctx.r[11].s64 + 11664;
	// 826233E0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826233E4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826233E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826233EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826233F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826233F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826233F8: 386A10E4  addi r3, r10, 0x10e4
	ctx.r[3].s64 = ctx.r[10].s64 + 4324;
	// 826233FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262340C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262341C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623420: 4BE43A01  bl 0x82466e20
	ctx.lr = 0x82623424;
	sub_82466E20(ctx, base);
	// 82623424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262342C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623438 size=112
    let mut pc: u32 = 0x82623438;
    'dispatch: loop {
        match pc {
            0x82623438 => {
    //   block [0x82623438..0x826234A8)
	// 82623438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262343C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623444: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623448: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262344C: 38AAFC44  addi r5, r10, -0x3bc
	ctx.r[5].s64 = ctx.r[10].s64 + -956;
	// 82623450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623454: 390B2F28  addi r8, r11, 0x2f28
	ctx.r[8].s64 = ctx.r[11].s64 + 12072;
	// 82623458: 3920001A  li r9, 0x1a
	ctx.r[9].s64 = 26;
	// 8262345C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82623460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623464: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262346C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623470: 386A1114  addi r3, r10, 0x1114
	ctx.r[3].s64 = ctx.r[10].s64 + 4372;
	// 82623474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262347C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262348C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623494: 4BE4398D  bl 0x82466e20
	ctx.lr = 0x82623498;
	sub_82466E20(ctx, base);
	// 82623498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262349C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826234A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826234A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826234A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826234A8 size=100
    let mut pc: u32 = 0x826234A8;
    'dispatch: loop {
        match pc {
            0x826234A8 => {
    //   block [0x826234A8..0x8262350C)
	// 826234A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826234AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826234B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826234B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826234B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826234BC: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826234C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826234C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826234C8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826234CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826234D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826234D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826234D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826234DC: 386A1144  addi r3, r10, 0x1144
	ctx.r[3].s64 = ctx.r[10].s64 + 4420;
	// 826234E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826234E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826234E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826234EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826234F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826234F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826234F8: 4BE43929  bl 0x82466e20
	ctx.lr = 0x826234FC;
	sub_82466E20(ctx, base);
	// 826234FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623510 size=112
    let mut pc: u32 = 0x82623510;
    'dispatch: loop {
        match pc {
            0x82623510 => {
    //   block [0x82623510..0x82623580)
	// 82623510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262351C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623520: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623524: 38AA1144  addi r5, r10, 0x1144
	ctx.r[5].s64 = ctx.r[10].s64 + 4420;
	// 82623528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262352C: 390B3198  addi r8, r11, 0x3198
	ctx.r[8].s64 = ctx.r[11].s64 + 12696;
	// 82623530: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82623534: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 82623538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262353C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623540: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82623544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623548: 386A1174  addi r3, r10, 0x1174
	ctx.r[3].s64 = ctx.r[10].s64 + 4468;
	// 8262354C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262355C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262356C: 4BE438B5  bl 0x82466e20
	ctx.lr = 0x82623570;
	sub_82466E20(ctx, base);
	// 82623570: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623574: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623578: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262357C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623580 size=100
    let mut pc: u32 = 0x82623580;
    'dispatch: loop {
        match pc {
            0x82623580 => {
    //   block [0x82623580..0x826235E4)
	// 82623580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262358C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623594: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623598: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262359C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826235A0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826235A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826235A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826235AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826235B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826235B4: 386A11A4  addi r3, r10, 0x11a4
	ctx.r[3].s64 = ctx.r[10].s64 + 4516;
	// 826235B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826235BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826235C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826235C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826235C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826235CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826235D0: 4BE43851  bl 0x82466e20
	ctx.lr = 0x826235D4;
	sub_82466E20(ctx, base);
	// 826235D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826235D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826235DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826235E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826235E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826235E8 size=108
    let mut pc: u32 = 0x826235E8;
    'dispatch: loop {
        match pc {
            0x826235E8 => {
    //   block [0x826235E8..0x82623654)
	// 826235E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826235EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826235F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826235F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826235F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826235FC: 38EB3210  addi r7, r11, 0x3210
	ctx.r[7].s64 = ctx.r[11].s64 + 12816;
	// 82623600: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82623604: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 82623608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262360C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82623614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623618: 386A11D4  addi r3, r10, 0x11d4
	ctx.r[3].s64 = ctx.r[10].s64 + 4564;
	// 8262361C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262362C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262363C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623640: 4BE437E1  bl 0x82466e20
	ctx.lr = 0x82623644;
	sub_82466E20(ctx, base);
	// 82623644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262364C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623658 size=112
    let mut pc: u32 = 0x82623658;
    'dispatch: loop {
        match pc {
            0x82623658 => {
    //   block [0x82623658..0x826236C8)
	// 82623658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262365C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623664: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623668: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262366C: 38AA11A4  addi r5, r10, 0x11a4
	ctx.r[5].s64 = ctx.r[10].s64 + 4516;
	// 82623670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623674: 390B3258  addi r8, r11, 0x3258
	ctx.r[8].s64 = ctx.r[11].s64 + 12888;
	// 82623678: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262367C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 82623680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623684: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262368C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623690: 386A1204  addi r3, r10, 0x1204
	ctx.r[3].s64 = ctx.r[10].s64 + 4612;
	// 82623694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262369C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826236A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826236A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826236A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826236AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826236B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826236B4: 4BE4376D  bl 0x82466e20
	ctx.lr = 0x826236B8;
	sub_82466E20(ctx, base);
	// 826236B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826236BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826236C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826236C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826236C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826236C8 size=100
    let mut pc: u32 = 0x826236C8;
    'dispatch: loop {
        match pc {
            0x826236C8 => {
    //   block [0x826236C8..0x8262372C)
	// 826236C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826236CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826236D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826236D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826236D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826236DC: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826236E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826236E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826236E8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826236EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826236F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826236F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826236F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826236FC: 386A1234  addi r3, r10, 0x1234
	ctx.r[3].s64 = ctx.r[10].s64 + 4660;
	// 82623700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623704: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623708: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262370C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623710: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82623714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623718: 4BE43709  bl 0x82466e20
	ctx.lr = 0x8262371C;
	sub_82466E20(ctx, base);
	// 8262371C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623730 size=100
    let mut pc: u32 = 0x82623730;
    'dispatch: loop {
        match pc {
            0x82623730 => {
    //   block [0x82623730..0x82623794)
	// 82623730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262373C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623744: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262374C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623750: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 82623754: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262375C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623764: 386A1264  addi r3, r10, 0x1264
	ctx.r[3].s64 = ctx.r[10].s64 + 4708;
	// 82623768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262376C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623770: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82623774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623778: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262377C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623780: 4BE436A1  bl 0x82466e20
	ctx.lr = 0x82623784;
	sub_82466E20(ctx, base);
	// 82623784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262378C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623798 size=112
    let mut pc: u32 = 0x82623798;
    'dispatch: loop {
        match pc {
            0x82623798 => {
    //   block [0x82623798..0x82623808)
	// 82623798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262379C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826237A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826237A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826237A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826237AC: 38AA1234  addi r5, r10, 0x1234
	ctx.r[5].s64 = ctx.r[10].s64 + 4660;
	// 826237B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826237B4: 390B3288  addi r8, r11, 0x3288
	ctx.r[8].s64 = ctx.r[11].s64 + 12936;
	// 826237B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826237BC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826237C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826237C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826237C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826237CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826237D0: 386A1294  addi r3, r10, 0x1294
	ctx.r[3].s64 = ctx.r[10].s64 + 4756;
	// 826237D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826237D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826237DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826237E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826237E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826237E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826237EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826237F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826237F4: 4BE4362D  bl 0x82466e20
	ctx.lr = 0x826237F8;
	sub_82466E20(ctx, base);
	// 826237F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826237FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623808 size=112
    let mut pc: u32 = 0x82623808;
    'dispatch: loop {
        match pc {
            0x82623808 => {
    //   block [0x82623808..0x82623878)
	// 82623808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262380C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623818: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262381C: 38AA1264  addi r5, r10, 0x1264
	ctx.r[5].s64 = ctx.r[10].s64 + 4708;
	// 82623820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623824: 390B32E8  addi r8, r11, 0x32e8
	ctx.r[8].s64 = ctx.r[11].s64 + 13032;
	// 82623828: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262382C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 82623830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623834: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623838: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262383C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623840: 386A12C4  addi r3, r10, 0x12c4
	ctx.r[3].s64 = ctx.r[10].s64 + 4804;
	// 82623844: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262384C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262385C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623864: 4BE435BD  bl 0x82466e20
	ctx.lr = 0x82623868;
	sub_82466E20(ctx, base);
	// 82623868: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262386C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623870: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623878 size=100
    let mut pc: u32 = 0x82623878;
    'dispatch: loop {
        match pc {
            0x82623878 => {
    //   block [0x82623878..0x826238DC)
	// 82623878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262387C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623884: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262388C: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623898: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8262389C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826238A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826238A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826238A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826238AC: 386A12F4  addi r3, r10, 0x12f4
	ctx.r[3].s64 = ctx.r[10].s64 + 4852;
	// 826238B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826238B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826238B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826238BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826238C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826238C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826238C8: 4BE43559  bl 0x82466e20
	ctx.lr = 0x826238CC;
	sub_82466E20(ctx, base);
	// 826238CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826238D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826238D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826238D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826238E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826238E0 size=112
    let mut pc: u32 = 0x826238E0;
    'dispatch: loop {
        match pc {
            0x826238E0 => {
    //   block [0x826238E0..0x82623950)
	// 826238E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826238E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826238E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826238EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826238F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826238F4: 38AA12F4  addi r5, r10, 0x12f4
	ctx.r[5].s64 = ctx.r[10].s64 + 4852;
	// 826238F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826238FC: 390B3348  addi r8, r11, 0x3348
	ctx.r[8].s64 = ctx.r[11].s64 + 13128;
	// 82623900: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82623904: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 82623908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262390C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82623914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623918: 386A1324  addi r3, r10, 0x1324
	ctx.r[3].s64 = ctx.r[10].s64 + 4900;
	// 8262391C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262392C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262393C: 4BE434E5  bl 0x82466e20
	ctx.lr = 0x82623940;
	sub_82466E20(ctx, base);
	// 82623940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262394C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623950 size=108
    let mut pc: u32 = 0x82623950;
    'dispatch: loop {
        match pc {
            0x82623950 => {
    //   block [0x82623950..0x826239BC)
	// 82623950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262395C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623964: 38EB3438  addi r7, r11, 0x3438
	ctx.r[7].s64 = ctx.r[11].s64 + 13368;
	// 82623968: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8262396C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 82623970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623974: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623978: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262397C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623980: 386A1354  addi r3, r10, 0x1354
	ctx.r[3].s64 = ctx.r[10].s64 + 4948;
	// 82623984: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262398C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262399C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826239A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826239A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826239A8: 4BE43479  bl 0x82466e20
	ctx.lr = 0x826239AC;
	sub_82466E20(ctx, base);
	// 826239AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826239B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826239B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826239B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826239C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826239C0 size=108
    let mut pc: u32 = 0x826239C0;
    'dispatch: loop {
        match pc {
            0x826239C0 => {
    //   block [0x826239C0..0x82623A2C)
	// 826239C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826239C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826239C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826239CC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826239D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826239D4: 38EB3528  addi r7, r11, 0x3528
	ctx.r[7].s64 = ctx.r[11].s64 + 13608;
	// 826239D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826239DC: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826239E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826239E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826239E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826239EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826239F0: 386A1384  addi r3, r10, 0x1384
	ctx.r[3].s64 = ctx.r[10].s64 + 4996;
	// 826239F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826239F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826239FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623A14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623A18: 4BE43409  bl 0x82466e20
	ctx.lr = 0x82623A1C;
	sub_82466E20(ctx, base);
	// 82623A1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623A30 size=108
    let mut pc: u32 = 0x82623A30;
    'dispatch: loop {
        match pc {
            0x82623A30 => {
    //   block [0x82623A30..0x82623A9C)
	// 82623A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623A3C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623A40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623A44: 38EB3570  addi r7, r11, 0x3570
	ctx.r[7].s64 = ctx.r[11].s64 + 13680;
	// 82623A48: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82623A4C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 82623A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623A54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623A58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82623A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623A60: 386A13B4  addi r3, r10, 0x13b4
	ctx.r[3].s64 = ctx.r[10].s64 + 5044;
	// 82623A64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623A84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623A88: 4BE43399  bl 0x82466e20
	ctx.lr = 0x82623A8C;
	sub_82466E20(ctx, base);
	// 82623A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623AA0 size=108
    let mut pc: u32 = 0x82623AA0;
    'dispatch: loop {
        match pc {
            0x82623AA0 => {
    //   block [0x82623AA0..0x82623B0C)
	// 82623AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623AAC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623AB4: 38EB3648  addi r7, r11, 0x3648
	ctx.r[7].s64 = ctx.r[11].s64 + 13896;
	// 82623AB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82623ABC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 82623AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623AC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623AC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82623ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623AD0: 386A13E4  addi r3, r10, 0x13e4
	ctx.r[3].s64 = ctx.r[10].s64 + 5092;
	// 82623AD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623ADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623AF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623AF8: 4BE43329  bl 0x82466e20
	ctx.lr = 0x82623AFC;
	sub_82466E20(ctx, base);
	// 82623AFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623B10 size=100
    let mut pc: u32 = 0x82623B10;
    'dispatch: loop {
        match pc {
            0x82623B10 => {
    //   block [0x82623B10..0x82623B74)
	// 82623B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623B1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623B24: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623B28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623B30: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 82623B34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623B3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623B44: 386A1414  addi r3, r10, 0x1414
	ctx.r[3].s64 = ctx.r[10].s64 + 5140;
	// 82623B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623B4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623B50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82623B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623B58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82623B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623B60: 4BE432C1  bl 0x82466e20
	ctx.lr = 0x82623B64;
	sub_82466E20(ctx, base);
	// 82623B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623B78 size=112
    let mut pc: u32 = 0x82623B78;
    'dispatch: loop {
        match pc {
            0x82623B78 => {
    //   block [0x82623B78..0x82623BE8)
	// 82623B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623B80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623B84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623B88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623B8C: 38AA1414  addi r5, r10, 0x1414
	ctx.r[5].s64 = ctx.r[10].s64 + 5140;
	// 82623B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623B94: 390B3660  addi r8, r11, 0x3660
	ctx.r[8].s64 = ctx.r[11].s64 + 13920;
	// 82623B98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82623B9C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 82623BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623BA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623BA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82623BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623BB0: 386A1444  addi r3, r10, 0x1444
	ctx.r[3].s64 = ctx.r[10].s64 + 5188;
	// 82623BB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623BD4: 4BE4324D  bl 0x82466e20
	ctx.lr = 0x82623BD8;
	sub_82466E20(ctx, base);
	// 82623BD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623BDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623BE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623BE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623BE8 size=108
    let mut pc: u32 = 0x82623BE8;
    'dispatch: loop {
        match pc {
            0x82623BE8 => {
    //   block [0x82623BE8..0x82623C54)
	// 82623BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623BF4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623BFC: 38EB36A8  addi r7, r11, 0x36a8
	ctx.r[7].s64 = ctx.r[11].s64 + 13992;
	// 82623C00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82623C04: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 82623C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623C0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623C10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82623C14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623C18: 386A1474  addi r3, r10, 0x1474
	ctx.r[3].s64 = ctx.r[10].s64 + 5236;
	// 82623C1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623C20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623C3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623C40: 4BE431E1  bl 0x82466e20
	ctx.lr = 0x82623C44;
	sub_82466E20(ctx, base);
	// 82623C44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623C48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623C4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623C58 size=112
    let mut pc: u32 = 0x82623C58;
    'dispatch: loop {
        match pc {
            0x82623C58 => {
    //   block [0x82623C58..0x82623CC8)
	// 82623C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623C64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623C68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623C6C: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623C74: 390B36F0  addi r8, r11, 0x36f0
	ctx.r[8].s64 = ctx.r[11].s64 + 14064;
	// 82623C78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82623C7C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 82623C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623C84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623C88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82623C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623C90: 386A14A4  addi r3, r10, 0x14a4
	ctx.r[3].s64 = ctx.r[10].s64 + 5284;
	// 82623C94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623CA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623CB4: 4BE4316D  bl 0x82466e20
	ctx.lr = 0x82623CB8;
	sub_82466E20(ctx, base);
	// 82623CB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623CBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623CC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623CC8 size=108
    let mut pc: u32 = 0x82623CC8;
    'dispatch: loop {
        match pc {
            0x82623CC8 => {
    //   block [0x82623CC8..0x82623D34)
	// 82623CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623CD4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623CDC: 38EB3708  addi r7, r11, 0x3708
	ctx.r[7].s64 = ctx.r[11].s64 + 14088;
	// 82623CE0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82623CE4: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 82623CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623CEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623CF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82623CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623CF8: 386A14D4  addi r3, r10, 0x14d4
	ctx.r[3].s64 = ctx.r[10].s64 + 5332;
	// 82623CFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623D0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623D20: 4BE43101  bl 0x82466e20
	ctx.lr = 0x82623D24;
	sub_82466E20(ctx, base);
	// 82623D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623D38 size=112
    let mut pc: u32 = 0x82623D38;
    'dispatch: loop {
        match pc {
            0x82623D38 => {
    //   block [0x82623D38..0x82623DA8)
	// 82623D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623D44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623D48: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623D4C: 38AA14A4  addi r5, r10, 0x14a4
	ctx.r[5].s64 = ctx.r[10].s64 + 5284;
	// 82623D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623D54: 390B3750  addi r8, r11, 0x3750
	ctx.r[8].s64 = ctx.r[11].s64 + 14160;
	// 82623D58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82623D5C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 82623D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623D64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82623D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623D70: 386A1504  addi r3, r10, 0x1504
	ctx.r[3].s64 = ctx.r[10].s64 + 5380;
	// 82623D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623D94: 4BE4308D  bl 0x82466e20
	ctx.lr = 0x82623D98;
	sub_82466E20(ctx, base);
	// 82623D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623DA8 size=100
    let mut pc: u32 = 0x82623DA8;
    'dispatch: loop {
        match pc {
            0x82623DA8 => {
    //   block [0x82623DA8..0x82623E0C)
	// 82623DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623DB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623DBC: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623DC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623DC8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 82623DCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623DDC: 386A1534  addi r3, r10, 0x1534
	ctx.r[3].s64 = ctx.r[10].s64 + 5428;
	// 82623DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623DE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623DE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82623DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623DF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82623DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623DF8: 4BE43029  bl 0x82466e20
	ctx.lr = 0x82623DFC;
	sub_82466E20(ctx, base);
	// 82623DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623E10 size=112
    let mut pc: u32 = 0x82623E10;
    'dispatch: loop {
        match pc {
            0x82623E10 => {
    //   block [0x82623E10..0x82623E80)
	// 82623E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623E1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623E20: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623E24: 38AA1534  addi r5, r10, 0x1534
	ctx.r[5].s64 = ctx.r[10].s64 + 5428;
	// 82623E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623E2C: 390B3768  addi r8, r11, 0x3768
	ctx.r[8].s64 = ctx.r[11].s64 + 14184;
	// 82623E30: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82623E34: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 82623E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82623E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623E48: 386A1564  addi r3, r10, 0x1564
	ctx.r[3].s64 = ctx.r[10].s64 + 5476;
	// 82623E4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623E6C: 4BE42FB5  bl 0x82466e20
	ctx.lr = 0x82623E70;
	sub_82466E20(ctx, base);
	// 82623E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623E80 size=108
    let mut pc: u32 = 0x82623E80;
    'dispatch: loop {
        match pc {
            0x82623E80 => {
    //   block [0x82623E80..0x82623EEC)
	// 82623E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623E8C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623E94: 38EB3810  addi r7, r11, 0x3810
	ctx.r[7].s64 = ctx.r[11].s64 + 14352;
	// 82623E98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82623E9C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 82623EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623EA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82623EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623EB0: 386A1594  addi r3, r10, 0x1594
	ctx.r[3].s64 = ctx.r[10].s64 + 5524;
	// 82623EB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82623EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623ED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82623ED8: 4BE42F49  bl 0x82466e20
	ctx.lr = 0x82623EDC;
	sub_82466E20(ctx, base);
	// 82623EDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623EF0 size=112
    let mut pc: u32 = 0x82623EF0;
    'dispatch: loop {
        match pc {
            0x82623EF0 => {
    //   block [0x82623EF0..0x82623F60)
	// 82623EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623EFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623F00: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623F04: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623F08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623F0C: 390B3840  addi r8, r11, 0x3840
	ctx.r[8].s64 = ctx.r[11].s64 + 14400;
	// 82623F10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82623F14: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 82623F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623F1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623F20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82623F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623F28: 386A15C4  addi r3, r10, 0x15c4
	ctx.r[3].s64 = ctx.r[10].s64 + 5572;
	// 82623F2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623F4C: 4BE42ED5  bl 0x82466e20
	ctx.lr = 0x82623F50;
	sub_82466E20(ctx, base);
	// 82623F50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623F54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623F58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623F60 size=112
    let mut pc: u32 = 0x82623F60;
    'dispatch: loop {
        match pc {
            0x82623F60 => {
    //   block [0x82623F60..0x82623FD0)
	// 82623F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623F6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623F70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82623F74: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623F78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623F7C: 390B3888  addi r8, r11, 0x3888
	ctx.r[8].s64 = ctx.r[11].s64 + 14472;
	// 82623F80: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82623F84: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 82623F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623F8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623F90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82623F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82623F98: 386A15F4  addi r3, r10, 0x15f4
	ctx.r[3].s64 = ctx.r[10].s64 + 5620;
	// 82623F9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82623FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82623FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82623FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82623FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82623FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82623FBC: 4BE42E65  bl 0x82466e20
	ctx.lr = 0x82623FC0;
	sub_82466E20(ctx, base);
	// 82623FC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82623FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82623FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82623FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82623FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82623FD0 size=100
    let mut pc: u32 = 0x82623FD0;
    'dispatch: loop {
        match pc {
            0x82623FD0 => {
    //   block [0x82623FD0..0x82624034)
	// 82623FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82623FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82623FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82623FDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82623FE4: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82623FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82623FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82623FF0: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 82623FF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82623FF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82623FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624004: 386A1624  addi r3, r10, 0x1624
	ctx.r[3].s64 = ctx.r[10].s64 + 5668;
	// 82624008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262400C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624010: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82624014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624018: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262401C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624020: 4BE42E01  bl 0x82466e20
	ctx.lr = 0x82624024;
	sub_82466E20(ctx, base);
	// 82624024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262402C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624038 size=112
    let mut pc: u32 = 0x82624038;
    'dispatch: loop {
        match pc {
            0x82624038 => {
    //   block [0x82624038..0x826240A8)
	// 82624038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262403C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624044: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624048: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262404C: 38AA1624  addi r5, r10, 0x1624
	ctx.r[5].s64 = ctx.r[10].s64 + 5668;
	// 82624050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624054: 390B38D0  addi r8, r11, 0x38d0
	ctx.r[8].s64 = ctx.r[11].s64 + 14544;
	// 82624058: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262405C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 82624060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624064: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262406C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624070: 386A1654  addi r3, r10, 0x1654
	ctx.r[3].s64 = ctx.r[10].s64 + 5716;
	// 82624074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82624078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262407C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262408C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624094: 4BE42D8D  bl 0x82466e20
	ctx.lr = 0x82624098;
	sub_82466E20(ctx, base);
	// 82624098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262409C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826240A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826240A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826240A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826240A8 size=112
    let mut pc: u32 = 0x826240A8;
    'dispatch: loop {
        match pc {
            0x826240A8 => {
    //   block [0x826240A8..0x82624118)
	// 826240A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826240AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826240B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826240B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826240B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826240BC: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 826240C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826240C4: 390B3918  addi r8, r11, 0x3918
	ctx.r[8].s64 = ctx.r[11].s64 + 14616;
	// 826240C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826240CC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826240D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826240D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826240D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826240DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826240E0: 386A1684  addi r3, r10, 0x1684
	ctx.r[3].s64 = ctx.r[10].s64 + 5764;
	// 826240E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826240E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826240EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826240F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826240F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826240F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826240FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624104: 4BE42D1D  bl 0x82466e20
	ctx.lr = 0x82624108;
	sub_82466E20(ctx, base);
	// 82624108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262410C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624118 size=112
    let mut pc: u32 = 0x82624118;
    'dispatch: loop {
        match pc {
            0x82624118 => {
    //   block [0x82624118..0x82624188)
	// 82624118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262411C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624124: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624128: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262412C: 38AAE804  addi r5, r10, -0x17fc
	ctx.r[5].s64 = ctx.r[10].s64 + -6140;
	// 82624130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624134: 390B3930  addi r8, r11, 0x3930
	ctx.r[8].s64 = ctx.r[11].s64 + 14640;
	// 82624138: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262413C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 82624140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624144: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262414C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624150: 386A16B4  addi r3, r10, 0x16b4
	ctx.r[3].s64 = ctx.r[10].s64 + 5812;
	// 82624154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82624158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262415C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624164: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82624168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262416C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624174: 4BE42CAD  bl 0x82466e20
	ctx.lr = 0x82624178;
	sub_82466E20(ctx, base);
	// 82624178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262417C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624188 size=112
    let mut pc: u32 = 0x82624188;
    'dispatch: loop {
        match pc {
            0x82624188 => {
    //   block [0x82624188..0x826241F8)
	// 82624188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262418C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624194: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624198: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262419C: 38AA1684  addi r5, r10, 0x1684
	ctx.r[5].s64 = ctx.r[10].s64 + 5764;
	// 826241A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826241A4: 390B3948  addi r8, r11, 0x3948
	ctx.r[8].s64 = ctx.r[11].s64 + 14664;
	// 826241A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826241AC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826241B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826241B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826241B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826241BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826241C0: 386A16E4  addi r3, r10, 0x16e4
	ctx.r[3].s64 = ctx.r[10].s64 + 5860;
	// 826241C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826241C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826241CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826241D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826241D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826241D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826241DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826241E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826241E4: 4BE42C3D  bl 0x82466e20
	ctx.lr = 0x826241E8;
	sub_82466E20(ctx, base);
	// 826241E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826241EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826241F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826241F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826241F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826241F8 size=72
    let mut pc: u32 = 0x826241F8;
    'dispatch: loop {
        match pc {
            0x826241F8 => {
    //   block [0x826241F8..0x82624240)
	// 826241F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826241FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624200: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624204: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82624208: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8262420C: 38CB1420  addi r6, r11, 0x1420
	ctx.r[6].s64 = ctx.r[11].s64 + 5152;
	// 82624210: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82624214: 388B1D90  addi r4, r11, 0x1d90
	ctx.r[4].s64 = ctx.r[11].s64 + 7568;
	// 82624218: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8262421C: 386B1714  addi r3, r11, 0x1714
	ctx.r[3].s64 = ctx.r[11].s64 + 5908;
	// 82624220: 4BE57869  bl 0x8247ba88
	ctx.lr = 0x82624224;
	sub_8247BA88(ctx, base);
	// 82624224: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 82624228: 386BCD50  addi r3, r11, -0x32b0
	ctx.r[3].s64 = ctx.r[11].s64 + -12976;
	// 8262422C: 4BF0E90D  bl 0x82532b38
	ctx.lr = 0x82624230;
	sub_82532B38(ctx, base);
	// 82624230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82624234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262423C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624240 size=108
    let mut pc: u32 = 0x82624240;
    'dispatch: loop {
        match pc {
            0x82624240 => {
    //   block [0x82624240..0x826242AC)
	// 82624240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262424C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624254: 38EB4258  addi r7, r11, 0x4258
	ctx.r[7].s64 = ctx.r[11].s64 + 16984;
	// 82624258: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8262425C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 82624260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262426C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624270: 386A172C  addi r3, r10, 0x172c
	ctx.r[3].s64 = ctx.r[10].s64 + 5932;
	// 82624274: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262427C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262428C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624298: 4BE42B89  bl 0x82466e20
	ctx.lr = 0x8262429C;
	sub_82466E20(ctx, base);
	// 8262429C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826242A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826242A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826242A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826242B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826242B0 size=24
    let mut pc: u32 = 0x826242B0;
    'dispatch: loop {
        match pc {
            0x826242B0 => {
    //   block [0x826242B0..0x826242C8)
	// 826242B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826242B4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 826242B8: 394AA870  addi r10, r10, -0x5790
	ctx.r[10].s64 = ctx.r[10].s64 + -22416;
	// 826242BC: 816B42D0  lwz r11, 0x42d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17104 as u32) ) } as u64;
	// 826242C0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826242C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826242C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826242C8 size=112
    let mut pc: u32 = 0x826242C8;
    'dispatch: loop {
        match pc {
            0x826242C8 => {
    //   block [0x826242C8..0x82624338)
	// 826242C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826242CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826242D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826242D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826242D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826242DC: 392B2A94  addi r9, r11, 0x2a94
	ctx.r[9].s64 = ctx.r[11].s64 + 10900;
	// 826242E0: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 826242E4: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826242E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826242EC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826242F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826242F4: 396BA870  addi r11, r11, -0x5790
	ctx.r[11].s64 = ctx.r[11].s64 + -22416;
	// 826242F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826242FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624300: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82624304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624308: 386A175C  addi r3, r10, 0x175c
	ctx.r[3].s64 = ctx.r[10].s64 + 5980;
	// 8262430C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82624310: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82624314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624318: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262431C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624320: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82624324: 4BE42AFD  bl 0x82466e20
	ctx.lr = 0x82624328;
	sub_82466E20(ctx, base);
	// 82624328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262432C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624338 size=108
    let mut pc: u32 = 0x82624338;
    'dispatch: loop {
        match pc {
            0x82624338 => {
    //   block [0x82624338..0x826243A4)
	// 82624338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262433C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624344: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262434C: 38EB42D4  addi r7, r11, 0x42d4
	ctx.r[7].s64 = ctx.r[11].s64 + 17108;
	// 82624350: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82624354: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 82624358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262435C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82624364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624368: 386A178C  addi r3, r10, 0x178c
	ctx.r[3].s64 = ctx.r[10].s64 + 6028;
	// 8262436C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262437C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262438C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624390: 4BE42A91  bl 0x82466e20
	ctx.lr = 0x82624394;
	sub_82466E20(ctx, base);
	// 82624394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262439C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826243A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826243A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826243A8 size=108
    let mut pc: u32 = 0x826243A8;
    'dispatch: loop {
        match pc {
            0x826243A8 => {
    //   block [0x826243A8..0x82624414)
	// 826243A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826243AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826243B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826243B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826243B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826243BC: 38EB4304  addi r7, r11, 0x4304
	ctx.r[7].s64 = ctx.r[11].s64 + 17156;
	// 826243C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826243C4: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826243C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826243CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826243D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826243D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826243D8: 386A17BC  addi r3, r10, 0x17bc
	ctx.r[3].s64 = ctx.r[10].s64 + 6076;
	// 826243DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826243E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826243E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826243E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826243EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826243F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826243F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826243F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826243FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624400: 4BE42A21  bl 0x82466e20
	ctx.lr = 0x82624404;
	sub_82466E20(ctx, base);
	// 82624404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262440C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82624418 size=24
    let mut pc: u32 = 0x82624418;
    'dispatch: loop {
        match pc {
            0x82624418 => {
    //   block [0x82624418..0x82624430)
	// 82624418: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262441C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82624420: 394AA8B8  addi r10, r10, -0x5748
	ctx.r[10].s64 = ctx.r[10].s64 + -22344;
	// 82624424: 816B4334  lwz r11, 0x4334(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17204 as u32) ) } as u64;
	// 82624428: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262442C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624430 size=116
    let mut pc: u32 = 0x82624430;
    'dispatch: loop {
        match pc {
            0x82624430 => {
    //   block [0x82624430..0x826244A4)
	// 82624430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262443C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82624440: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82624444: 390BA8B8  addi r8, r11, -0x5748
	ctx.r[8].s64 = ctx.r[11].s64 + -22344;
	// 82624448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262444C: 392A2AD8  addi r9, r10, 0x2ad8
	ctx.r[9].s64 = ctx.r[10].s64 + 10968;
	// 82624450: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624454: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82624458: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 8262445C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82624460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624464: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262446C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624474: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82624478: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 8262447C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82624480: 386B17EC  addi r3, r11, 0x17ec
	ctx.r[3].s64 = ctx.r[11].s64 + 6124;
	// 82624484: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82624488: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262448C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624490: 4BE42991  bl 0x82466e20
	ctx.lr = 0x82624494;
	sub_82466E20(ctx, base);
	// 82624494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262449C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826244A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826244A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826244A8 size=108
    let mut pc: u32 = 0x826244A8;
    'dispatch: loop {
        match pc {
            0x826244A8 => {
    //   block [0x826244A8..0x82624514)
	// 826244A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826244AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826244B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826244B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826244B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826244BC: 38EB4338  addi r7, r11, 0x4338
	ctx.r[7].s64 = ctx.r[11].s64 + 17208;
	// 826244C0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826244C4: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826244C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826244CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826244D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826244D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826244D8: 386A181C  addi r3, r10, 0x181c
	ctx.r[3].s64 = ctx.r[10].s64 + 6172;
	// 826244DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826244E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826244E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826244E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826244EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826244F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826244F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826244F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826244FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624500: 4BE42921  bl 0x82466e20
	ctx.lr = 0x82624504;
	sub_82466E20(ctx, base);
	// 82624504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262450C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624518 size=112
    let mut pc: u32 = 0x82624518;
    'dispatch: loop {
        match pc {
            0x82624518 => {
    //   block [0x82624518..0x82624588)
	// 82624518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262451C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624524: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624528: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262452C: 38AA17EC  addi r5, r10, 0x17ec
	ctx.r[5].s64 = ctx.r[10].s64 + 6124;
	// 82624530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624534: 390B43C8  addi r8, r11, 0x43c8
	ctx.r[8].s64 = ctx.r[11].s64 + 17352;
	// 82624538: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8262453C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 82624540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624544: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262454C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624550: 386A184C  addi r3, r10, 0x184c
	ctx.r[3].s64 = ctx.r[10].s64 + 6220;
	// 82624554: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82624558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262455C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262456C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624574: 4BE428AD  bl 0x82466e20
	ctx.lr = 0x82624578;
	sub_82466E20(ctx, base);
	// 82624578: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262457C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624588 size=112
    let mut pc: u32 = 0x82624588;
    'dispatch: loop {
        match pc {
            0x82624588 => {
    //   block [0x82624588..0x826245F8)
	// 82624588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262458C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624594: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624598: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262459C: 38AA17EC  addi r5, r10, 0x17ec
	ctx.r[5].s64 = ctx.r[10].s64 + 6124;
	// 826245A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826245A4: 390B44E8  addi r8, r11, 0x44e8
	ctx.r[8].s64 = ctx.r[11].s64 + 17640;
	// 826245A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826245AC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826245B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826245B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826245B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826245BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826245C0: 386A187C  addi r3, r10, 0x187c
	ctx.r[3].s64 = ctx.r[10].s64 + 6268;
	// 826245C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826245C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826245CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826245D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826245D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826245D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826245DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826245E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826245E4: 4BE4283D  bl 0x82466e20
	ctx.lr = 0x826245E8;
	sub_82466E20(ctx, base);
	// 826245E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826245EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826245F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826245F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826245F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826245F8 size=108
    let mut pc: u32 = 0x826245F8;
    'dispatch: loop {
        match pc {
            0x826245F8 => {
    //   block [0x826245F8..0x82624664)
	// 826245F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826245FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624604: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262460C: 38EB4500  addi r7, r11, 0x4500
	ctx.r[7].s64 = ctx.r[11].s64 + 17664;
	// 82624610: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82624614: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 82624618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262461C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82624624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624628: 386A18AC  addi r3, r10, 0x18ac
	ctx.r[3].s64 = ctx.r[10].s64 + 6316;
	// 8262462C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262463C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262464C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624650: 4BE427D1  bl 0x82466e20
	ctx.lr = 0x82624654;
	sub_82466E20(ctx, base);
	// 82624654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262465C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624668 size=112
    let mut pc: u32 = 0x82624668;
    'dispatch: loop {
        match pc {
            0x82624668 => {
    //   block [0x82624668..0x826246D8)
	// 82624668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262466C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624674: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624678: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262467C: 38AA17EC  addi r5, r10, 0x17ec
	ctx.r[5].s64 = ctx.r[10].s64 + 6124;
	// 82624680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624684: 390B4590  addi r8, r11, 0x4590
	ctx.r[8].s64 = ctx.r[11].s64 + 17808;
	// 82624688: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8262468C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 82624690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262469C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826246A0: 386A18DC  addi r3, r10, 0x18dc
	ctx.r[3].s64 = ctx.r[10].s64 + 6364;
	// 826246A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826246A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826246AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826246B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826246B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826246B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826246BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826246C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826246C4: 4BE4275D  bl 0x82466e20
	ctx.lr = 0x826246C8;
	sub_82466E20(ctx, base);
	// 826246C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826246CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826246D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826246D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826246D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826246D8 size=108
    let mut pc: u32 = 0x826246D8;
    'dispatch: loop {
        match pc {
            0x826246D8 => {
    //   block [0x826246D8..0x82624744)
	// 826246D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826246DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826246E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826246E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826246E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826246EC: 38EB4680  addi r7, r11, 0x4680
	ctx.r[7].s64 = ctx.r[11].s64 + 18048;
	// 826246F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826246F4: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826246F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826246FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82624704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624708: 386A190C  addi r3, r10, 0x190c
	ctx.r[3].s64 = ctx.r[10].s64 + 6412;
	// 8262470C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262471C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262472C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624730: 4BE426F1  bl 0x82466e20
	ctx.lr = 0x82624734;
	sub_82466E20(ctx, base);
	// 82624734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262473C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624748 size=108
    let mut pc: u32 = 0x82624748;
    'dispatch: loop {
        match pc {
            0x82624748 => {
    //   block [0x82624748..0x826247B4)
	// 82624748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262474C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624754: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262475C: 38EB4698  addi r7, r11, 0x4698
	ctx.r[7].s64 = ctx.r[11].s64 + 18072;
	// 82624760: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82624764: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 82624768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262476C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82624774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624778: 386A193C  addi r3, r10, 0x193c
	ctx.r[3].s64 = ctx.r[10].s64 + 6460;
	// 8262477C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262478C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262479C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826247A0: 4BE42681  bl 0x82466e20
	ctx.lr = 0x826247A4;
	sub_82466E20(ctx, base);
	// 826247A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826247A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826247AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826247B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826247B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826247B8 size=116
    let mut pc: u32 = 0x826247B8;
    'dispatch: loop {
        match pc {
            0x826247B8 => {
    //   block [0x826247B8..0x8262482C)
	// 826247B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826247BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826247C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826247C4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826247C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826247CC: 390B46FC  addi r8, r11, 0x46fc
	ctx.r[8].s64 = ctx.r[11].s64 + 18172;
	// 826247D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826247D4: 392A2B04  addi r9, r10, 0x2b04
	ctx.r[9].s64 = ctx.r[10].s64 + 11012;
	// 826247D8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826247DC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826247E0: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 826247E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826247E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826247EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826247F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826247F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826247F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826247FC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82624800: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 82624804: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82624808: 386B196C  addi r3, r11, 0x196c
	ctx.r[3].s64 = ctx.r[11].s64 + 6508;
	// 8262480C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82624810: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624818: 4BE42609  bl 0x82466e20
	ctx.lr = 0x8262481C;
	sub_82466E20(ctx, base);
	// 8262481C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624830 size=108
    let mut pc: u32 = 0x82624830;
    'dispatch: loop {
        match pc {
            0x82624830 => {
    //   block [0x82624830..0x8262489C)
	// 82624830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262483C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624844: 38EB4718  addi r7, r11, 0x4718
	ctx.r[7].s64 = ctx.r[11].s64 + 18200;
	// 82624848: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8262484C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 82624850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624854: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624858: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262485C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624860: 386A199C  addi r3, r10, 0x199c
	ctx.r[3].s64 = ctx.r[10].s64 + 6556;
	// 82624864: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262486C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262487C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624884: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624888: 4BE42599  bl 0x82466e20
	ctx.lr = 0x8262488C;
	sub_82466E20(ctx, base);
	// 8262488C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826248A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826248A0 size=108
    let mut pc: u32 = 0x826248A0;
    'dispatch: loop {
        match pc {
            0x826248A0 => {
    //   block [0x826248A0..0x8262490C)
	// 826248A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826248A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826248A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826248AC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826248B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826248B4: 38EB4760  addi r7, r11, 0x4760
	ctx.r[7].s64 = ctx.r[11].s64 + 18272;
	// 826248B8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826248BC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826248C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826248C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826248C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826248CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826248D0: 386A19CC  addi r3, r10, 0x19cc
	ctx.r[3].s64 = ctx.r[10].s64 + 6604;
	// 826248D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826248D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826248DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826248E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826248E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826248E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826248EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826248F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826248F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826248F8: 4BE42529  bl 0x82466e20
	ctx.lr = 0x826248FC;
	sub_82466E20(ctx, base);
	// 826248FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624910 size=108
    let mut pc: u32 = 0x82624910;
    'dispatch: loop {
        match pc {
            0x82624910 => {
    //   block [0x82624910..0x8262497C)
	// 82624910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262491C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624924: 38EB47F0  addi r7, r11, 0x47f0
	ctx.r[7].s64 = ctx.r[11].s64 + 18416;
	// 82624928: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8262492C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82624930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624934: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624938: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262493C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624940: 386A19FC  addi r3, r10, 0x19fc
	ctx.r[3].s64 = ctx.r[10].s64 + 6652;
	// 82624944: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262494C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262495C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624968: 4BE424B9  bl 0x82466e20
	ctx.lr = 0x8262496C;
	sub_82466E20(ctx, base);
	// 8262496C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624980 size=100
    let mut pc: u32 = 0x82624980;
    'dispatch: loop {
        match pc {
            0x82624980 => {
    //   block [0x82624980..0x826249E4)
	// 82624980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262498C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624994: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82624998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262499C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826249A0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826249A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826249A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826249AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826249B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826249B4: 386A1A2C  addi r3, r10, 0x1a2c
	ctx.r[3].s64 = ctx.r[10].s64 + 6700;
	// 826249B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826249BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826249C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826249C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826249C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826249CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826249D0: 4BE42451  bl 0x82466e20
	ctx.lr = 0x826249D4;
	sub_82466E20(ctx, base);
	// 826249D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826249D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826249DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826249E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826249E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826249E8 size=112
    let mut pc: u32 = 0x826249E8;
    'dispatch: loop {
        match pc {
            0x826249E8 => {
    //   block [0x826249E8..0x82624A58)
	// 826249E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826249EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826249F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826249F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826249F8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826249FC: 38AA1A2C  addi r5, r10, 0x1a2c
	ctx.r[5].s64 = ctx.r[10].s64 + 6700;
	// 82624A00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624A04: 390B4880  addi r8, r11, 0x4880
	ctx.r[8].s64 = ctx.r[11].s64 + 18560;
	// 82624A08: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82624A0C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82624A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624A14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624A18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82624A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624A20: 386A1A5C  addi r3, r10, 0x1a5c
	ctx.r[3].s64 = ctx.r[10].s64 + 6748;
	// 82624A24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82624A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624A34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624A3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624A44: 4BE423DD  bl 0x82466e20
	ctx.lr = 0x82624A48;
	sub_82466E20(ctx, base);
	// 82624A48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624A54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624A58 size=108
    let mut pc: u32 = 0x82624A58;
    'dispatch: loop {
        match pc {
            0x82624A58 => {
    //   block [0x82624A58..0x82624AC4)
	// 82624A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624A64: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624A6C: 38EB48E0  addi r7, r11, 0x48e0
	ctx.r[7].s64 = ctx.r[11].s64 + 18656;
	// 82624A70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82624A74: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 82624A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624A7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624A80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82624A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624A88: 386A1A8C  addi r3, r10, 0x1a8c
	ctx.r[3].s64 = ctx.r[10].s64 + 6796;
	// 82624A8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624AAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624AB0: 4BE42371  bl 0x82466e20
	ctx.lr = 0x82624AB4;
	sub_82466E20(ctx, base);
	// 82624AB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624AC8 size=108
    let mut pc: u32 = 0x82624AC8;
    'dispatch: loop {
        match pc {
            0x82624AC8 => {
    //   block [0x82624AC8..0x82624B34)
	// 82624AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624AD4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624ADC: 38EB4910  addi r7, r11, 0x4910
	ctx.r[7].s64 = ctx.r[11].s64 + 18704;
	// 82624AE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82624AE4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82624AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624AEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82624AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624AF8: 386A1ABC  addi r3, r10, 0x1abc
	ctx.r[3].s64 = ctx.r[10].s64 + 6844;
	// 82624AFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624B20: 4BE42301  bl 0x82466e20
	ctx.lr = 0x82624B24;
	sub_82466E20(ctx, base);
	// 82624B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624B38 size=108
    let mut pc: u32 = 0x82624B38;
    'dispatch: loop {
        match pc {
            0x82624B38 => {
    //   block [0x82624B38..0x82624BA4)
	// 82624B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624B44: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624B4C: 38EB4970  addi r7, r11, 0x4970
	ctx.r[7].s64 = ctx.r[11].s64 + 18800;
	// 82624B50: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82624B54: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 82624B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624B5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624B60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82624B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624B68: 386A1AEC  addi r3, r10, 0x1aec
	ctx.r[3].s64 = ctx.r[10].s64 + 6892;
	// 82624B6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624B90: 4BE42291  bl 0x82466e20
	ctx.lr = 0x82624B94;
	sub_82466E20(ctx, base);
	// 82624B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624BA8 size=108
    let mut pc: u32 = 0x82624BA8;
    'dispatch: loop {
        match pc {
            0x82624BA8 => {
    //   block [0x82624BA8..0x82624C14)
	// 82624BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624BB4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624BBC: 38EB49D0  addi r7, r11, 0x49d0
	ctx.r[7].s64 = ctx.r[11].s64 + 18896;
	// 82624BC0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82624BC4: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 82624BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624BCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82624BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624BD8: 386A1B1C  addi r3, r10, 0x1b1c
	ctx.r[3].s64 = ctx.r[10].s64 + 6940;
	// 82624BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624C00: 4BE42221  bl 0x82466e20
	ctx.lr = 0x82624C04;
	sub_82466E20(ctx, base);
	// 82624C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624C18 size=112
    let mut pc: u32 = 0x82624C18;
    'dispatch: loop {
        match pc {
            0x82624C18 => {
    //   block [0x82624C18..0x82624C88)
	// 82624C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624C24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82624C28: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624C2C: 392A2B38  addi r9, r10, 0x2b38
	ctx.r[9].s64 = ctx.r[10].s64 + 11064;
	// 82624C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624C34: 390B4A48  addi r8, r11, 0x4a48
	ctx.r[8].s64 = ctx.r[11].s64 + 19016;
	// 82624C38: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82624C3C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 82624C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624C44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82624C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624C50: 386A1B4C  addi r3, r10, 0x1b4c
	ctx.r[3].s64 = ctx.r[10].s64 + 6988;
	// 82624C54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82624C58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82624C5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624C64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624C6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624C74: 4BE421AD  bl 0x82466e20
	ctx.lr = 0x82624C78;
	sub_82466E20(ctx, base);
	// 82624C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624C88 size=112
    let mut pc: u32 = 0x82624C88;
    'dispatch: loop {
        match pc {
            0x82624C88 => {
    //   block [0x82624C88..0x82624CF8)
	// 82624C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624C94: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82624C98: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82624C9C: 38EA4B50  addi r7, r10, 0x4b50
	ctx.r[7].s64 = ctx.r[10].s64 + 19280;
	// 82624CA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624CA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82624CA8: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 82624CAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624CB0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624CB4: 396B2B4C  addi r11, r11, 0x2b4c
	ctx.r[11].s64 = ctx.r[11].s64 + 11084;
	// 82624CB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82624CBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624CC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624CC4: 386A1B7C  addi r3, r10, 0x1b7c
	ctx.r[3].s64 = ctx.r[10].s64 + 7036;
	// 82624CC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624CCC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82624CD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624CD4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82624CD8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624CDC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624CE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624CE4: 4BE4213D  bl 0x82466e20
	ctx.lr = 0x82624CE8;
	sub_82466E20(ctx, base);
	// 82624CE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624CEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624CF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624CF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624CF8 size=112
    let mut pc: u32 = 0x82624CF8;
    'dispatch: loop {
        match pc {
            0x82624CF8 => {
    //   block [0x82624CF8..0x82624D68)
	// 82624CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624D04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82624D08: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624D0C: 392A2B90  addi r9, r10, 0x2b90
	ctx.r[9].s64 = ctx.r[10].s64 + 11152;
	// 82624D10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624D14: 390B4C5C  addi r8, r11, 0x4c5c
	ctx.r[8].s64 = ctx.r[11].s64 + 19548;
	// 82624D18: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82624D1C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82624D20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624D24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624D28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82624D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624D30: 386A1BAC  addi r3, r10, 0x1bac
	ctx.r[3].s64 = ctx.r[10].s64 + 7084;
	// 82624D34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82624D38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82624D3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624D40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624D48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624D50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624D54: 4BE420CD  bl 0x82466e20
	ctx.lr = 0x82624D58;
	sub_82466E20(ctx, base);
	// 82624D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624D68 size=100
    let mut pc: u32 = 0x82624D68;
    'dispatch: loop {
        match pc {
            0x82624D68 => {
    //   block [0x82624D68..0x82624DCC)
	// 82624D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624D74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624D7C: 38AA217C  addi r5, r10, 0x217c
	ctx.r[5].s64 = ctx.r[10].s64 + 8572;
	// 82624D80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624D88: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 82624D8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624D9C: 386A1BDC  addi r3, r10, 0x1bdc
	ctx.r[3].s64 = ctx.r[10].s64 + 7132;
	// 82624DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624DA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624DA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82624DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624DB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82624DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624DB8: 4BE42069  bl 0x82466e20
	ctx.lr = 0x82624DBC;
	sub_82466E20(ctx, base);
	// 82624DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624DD0 size=116
    let mut pc: u32 = 0x82624DD0;
    'dispatch: loop {
        match pc {
            0x82624DD0 => {
    //   block [0x82624DD0..0x82624E44)
	// 82624DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624DDC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82624DE0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82624DE4: 390A4C90  addi r8, r10, 0x4c90
	ctx.r[8].s64 = ctx.r[10].s64 + 19600;
	// 82624DE8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624DEC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82624DF0: 38AA1BDC  addi r5, r10, 0x1bdc
	ctx.r[5].s64 = ctx.r[10].s64 + 7132;
	// 82624DF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624DF8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82624DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82624E04: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82624E08: 396B2BA4  addi r11, r11, 0x2ba4
	ctx.r[11].s64 = ctx.r[11].s64 + 11172;
	// 82624E0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624E10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624E14: 386A1C0C  addi r3, r10, 0x1c0c
	ctx.r[3].s64 = ctx.r[10].s64 + 7180;
	// 82624E18: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82624E1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624E20: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82624E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624E30: 4BE41FF1  bl 0x82466e20
	ctx.lr = 0x82624E34;
	sub_82466E20(ctx, base);
	// 82624E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624E48 size=100
    let mut pc: u32 = 0x82624E48;
    'dispatch: loop {
        match pc {
            0x82624E48 => {
    //   block [0x82624E48..0x82624EAC)
	// 82624E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624E54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624E5C: 38AA1C0C  addi r5, r10, 0x1c0c
	ctx.r[5].s64 = ctx.r[10].s64 + 7180;
	// 82624E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624E64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624E68: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 82624E6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624E74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624E7C: 386A1C3C  addi r3, r10, 0x1c3c
	ctx.r[3].s64 = ctx.r[10].s64 + 7228;
	// 82624E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624E84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624E88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82624E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624E90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82624E94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624E98: 4BE41F89  bl 0x82466e20
	ctx.lr = 0x82624E9C;
	sub_82466E20(ctx, base);
	// 82624E9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624EA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624EA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624EA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624EB0 size=112
    let mut pc: u32 = 0x82624EB0;
    'dispatch: loop {
        match pc {
            0x82624EB0 => {
    //   block [0x82624EB0..0x82624F20)
	// 82624EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624EBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624EC0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624EC4: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82624EC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624ECC: 390B4D38  addi r8, r11, 0x4d38
	ctx.r[8].s64 = ctx.r[11].s64 + 19768;
	// 82624ED0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82624ED4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82624ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624EDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624EE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82624EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624EE8: 386A1C6C  addi r3, r10, 0x1c6c
	ctx.r[3].s64 = ctx.r[10].s64 + 7276;
	// 82624EEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82624EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624EF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624F0C: 4BE41F15  bl 0x82466e20
	ctx.lr = 0x82624F10;
	sub_82466E20(ctx, base);
	// 82624F10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624F14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624F18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624F1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624F20 size=116
    let mut pc: u32 = 0x82624F20;
    'dispatch: loop {
        match pc {
            0x82624F20 => {
    //   block [0x82624F20..0x82624F94)
	// 82624F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624F28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624F2C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82624F30: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82624F34: 390A4D80  addi r8, r10, 0x4d80
	ctx.r[8].s64 = ctx.r[10].s64 + 19840;
	// 82624F38: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624F3C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82624F40: 38AA1BDC  addi r5, r10, 0x1bdc
	ctx.r[5].s64 = ctx.r[10].s64 + 7132;
	// 82624F44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624F48: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82624F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624F50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82624F54: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 82624F58: 396B2BD0  addi r11, r11, 0x2bd0
	ctx.r[11].s64 = ctx.r[11].s64 + 11216;
	// 82624F5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624F60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624F64: 386A1C9C  addi r3, r10, 0x1c9c
	ctx.r[3].s64 = ctx.r[10].s64 + 7324;
	// 82624F68: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82624F6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624F70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82624F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624F80: 4BE41EA1  bl 0x82466e20
	ctx.lr = 0x82624F84;
	sub_82466E20(ctx, base);
	// 82624F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82624F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82624F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82624F98 size=108
    let mut pc: u32 = 0x82624F98;
    'dispatch: loop {
        match pc {
            0x82624F98 => {
    //   block [0x82624F98..0x82625004)
	// 82624F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82624F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82624FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82624FA4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82624FA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82624FAC: 38EB4E40  addi r7, r11, 0x4e40
	ctx.r[7].s64 = ctx.r[11].s64 + 20032;
	// 82624FB0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82624FB4: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82624FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82624FBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82624FC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82624FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82624FC8: 386A1CCC  addi r3, r10, 0x1ccc
	ctx.r[3].s64 = ctx.r[10].s64 + 7372;
	// 82624FCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82624FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82624FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82624FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82624FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82624FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82624FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82624FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82624FEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82624FF0: 4BE41E31  bl 0x82466e20
	ctx.lr = 0x82624FF4;
	sub_82466E20(ctx, base);
	// 82624FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82624FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82624FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82625008 size=24
    let mut pc: u32 = 0x82625008;
    'dispatch: loop {
        match pc {
            0x82625008 => {
    //   block [0x82625008..0x82625020)
	// 82625008: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262500C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82625010: 394AA930  addi r10, r10, -0x56d0
	ctx.r[10].s64 = ctx.r[10].s64 + -22224;
	// 82625014: 816B4C8C  lwz r11, 0x4c8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19596 as u32) ) } as u64;
	// 82625018: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262501C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625020 size=116
    let mut pc: u32 = 0x82625020;
    'dispatch: loop {
        match pc {
            0x82625020 => {
    //   block [0x82625020..0x82625094)
	// 82625020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262502C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625030: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625034: 392B2C18  addi r9, r11, 0x2c18
	ctx.r[9].s64 = ctx.r[11].s64 + 11288;
	// 82625038: 38AA1BDC  addi r5, r10, 0x1bdc
	ctx.r[5].s64 = ctx.r[10].s64 + 7132;
	// 8262503C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625040: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 82625044: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 82625048: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262504C: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 82625050: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625054: 396BA930  addi r11, r11, -0x56d0
	ctx.r[11].s64 = ctx.r[11].s64 + -22224;
	// 82625058: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8262505C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625060: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82625064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625068: 386A1CFC  addi r3, r10, 0x1cfc
	ctx.r[3].s64 = ctx.r[10].s64 + 7420;
	// 8262506C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82625070: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82625074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625078: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8262507C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82625080: 4BE41DA1  bl 0x82466e20
	ctx.lr = 0x82625084;
	sub_82466E20(ctx, base);
	// 82625084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262508C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625098 size=112
    let mut pc: u32 = 0x82625098;
    'dispatch: loop {
        match pc {
            0x82625098 => {
    //   block [0x82625098..0x82625108)
	// 82625098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262509C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826250A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826250A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826250A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826250AC: 38AA1BDC  addi r5, r10, 0x1bdc
	ctx.r[5].s64 = ctx.r[10].s64 + 7132;
	// 826250B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826250B4: 390B4E88  addi r8, r11, 0x4e88
	ctx.r[8].s64 = ctx.r[11].s64 + 20104;
	// 826250B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826250BC: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826250C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826250C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826250C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826250CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826250D0: 386A1D2C  addi r3, r10, 0x1d2c
	ctx.r[3].s64 = ctx.r[10].s64 + 7468;
	// 826250D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826250D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826250DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826250E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826250E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826250E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826250EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826250F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826250F4: 4BE41D2D  bl 0x82466e20
	ctx.lr = 0x826250F8;
	sub_82466E20(ctx, base);
	// 826250F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826250FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625108 size=112
    let mut pc: u32 = 0x82625108;
    'dispatch: loop {
        match pc {
            0x82625108 => {
    //   block [0x82625108..0x82625178)
	// 82625108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262510C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625118: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262511C: 38AA1BDC  addi r5, r10, 0x1bdc
	ctx.r[5].s64 = ctx.r[10].s64 + 7132;
	// 82625120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625124: 390B4EB8  addi r8, r11, 0x4eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 20152;
	// 82625128: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262512C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 82625130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625134: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262513C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625140: 386A1D5C  addi r3, r10, 0x1d5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7516;
	// 82625144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82625148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262514C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262515C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625164: 4BE41CBD  bl 0x82466e20
	ctx.lr = 0x82625168;
	sub_82466E20(ctx, base);
	// 82625168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262516C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625178 size=100
    let mut pc: u32 = 0x82625178;
    'dispatch: loop {
        match pc {
            0x82625178 => {
    //   block [0x82625178..0x826251DC)
	// 82625178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262517C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262518C: 38AA217C  addi r5, r10, 0x217c
	ctx.r[5].s64 = ctx.r[10].s64 + 8572;
	// 82625190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625198: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 8262519C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826251A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826251A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826251A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826251AC: 386A1D8C  addi r3, r10, 0x1d8c
	ctx.r[3].s64 = ctx.r[10].s64 + 7564;
	// 826251B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826251B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826251B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826251BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826251C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826251C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826251C8: 4BE41C59  bl 0x82466e20
	ctx.lr = 0x826251CC;
	sub_82466E20(ctx, base);
	// 826251CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826251D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826251D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826251D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826251E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826251E0 size=112
    let mut pc: u32 = 0x826251E0;
    'dispatch: loop {
        match pc {
            0x826251E0 => {
    //   block [0x826251E0..0x82625250)
	// 826251E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826251E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826251E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826251EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826251F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826251F4: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 826251F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826251FC: 390B4ED0  addi r8, r11, 0x4ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 20176;
	// 82625200: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82625204: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 82625208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262520C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625210: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625218: 386A1DBC  addi r3, r10, 0x1dbc
	ctx.r[3].s64 = ctx.r[10].s64 + 7612;
	// 8262521C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82625220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262522C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262523C: 4BE41BE5  bl 0x82466e20
	ctx.lr = 0x82625240;
	sub_82466E20(ctx, base);
	// 82625240: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262524C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625250 size=108
    let mut pc: u32 = 0x82625250;
    'dispatch: loop {
        match pc {
            0x82625250 => {
    //   block [0x82625250..0x826252BC)
	// 82625250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262525C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625264: 38EB4F00  addi r7, r11, 0x4f00
	ctx.r[7].s64 = ctx.r[11].s64 + 20224;
	// 82625268: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8262526C: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 82625270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262527C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625280: 386A1DEC  addi r3, r10, 0x1dec
	ctx.r[3].s64 = ctx.r[10].s64 + 7660;
	// 82625284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262528C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262529C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826252A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826252A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826252A8: 4BE41B79  bl 0x82466e20
	ctx.lr = 0x826252AC;
	sub_82466E20(ctx, base);
	// 826252AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826252B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826252B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826252B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826252C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826252C0 size=108
    let mut pc: u32 = 0x826252C0;
    'dispatch: loop {
        match pc {
            0x826252C0 => {
    //   block [0x826252C0..0x8262532C)
	// 826252C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826252C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826252C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826252CC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826252D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826252D4: 38EB4F60  addi r7, r11, 0x4f60
	ctx.r[7].s64 = ctx.r[11].s64 + 20320;
	// 826252D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826252DC: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826252E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826252E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826252E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826252EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826252F0: 386A1E1C  addi r3, r10, 0x1e1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7708;
	// 826252F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826252F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826252FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262530C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625318: 4BE41B09  bl 0x82466e20
	ctx.lr = 0x8262531C;
	sub_82466E20(ctx, base);
	// 8262531C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625330 size=116
    let mut pc: u32 = 0x82625330;
    'dispatch: loop {
        match pc {
            0x82625330 => {
    //   block [0x82625330..0x826253A4)
	// 82625330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262533C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625340: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82625344: 390A4F90  addi r8, r10, 0x4f90
	ctx.r[8].s64 = ctx.r[10].s64 + 20368;
	// 82625348: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262534C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625350: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 82625354: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625358: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262535C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625364: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82625368: 396B2C70  addi r11, r11, 0x2c70
	ctx.r[11].s64 = ctx.r[11].s64 + 11376;
	// 8262536C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625370: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625374: 386A1E4C  addi r3, r10, 0x1e4c
	ctx.r[3].s64 = ctx.r[10].s64 + 7756;
	// 82625378: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262537C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625380: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82625384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262538C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625390: 4BE41A91  bl 0x82466e20
	ctx.lr = 0x82625394;
	sub_82466E20(ctx, base);
	// 82625394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262539C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826253A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826253A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826253A8 size=112
    let mut pc: u32 = 0x826253A8;
    'dispatch: loop {
        match pc {
            0x826253A8 => {
    //   block [0x826253A8..0x82625418)
	// 826253A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826253AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826253B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826253B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826253B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826253BC: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 826253C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826253C4: 390B5110  addi r8, r11, 0x5110
	ctx.r[8].s64 = ctx.r[11].s64 + 20752;
	// 826253C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826253CC: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826253D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826253D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826253D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826253DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826253E0: 386A1E7C  addi r3, r10, 0x1e7c
	ctx.r[3].s64 = ctx.r[10].s64 + 7804;
	// 826253E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826253E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826253EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826253F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826253F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826253F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826253FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625404: 4BE41A1D  bl 0x82466e20
	ctx.lr = 0x82625408;
	sub_82466E20(ctx, base);
	// 82625408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262540C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625418 size=116
    let mut pc: u32 = 0x82625418;
    'dispatch: loop {
        match pc {
            0x82625418 => {
    //   block [0x82625418..0x8262548C)
	// 82625418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262541C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625424: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625428: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8262542C: 390A5128  addi r8, r10, 0x5128
	ctx.r[8].s64 = ctx.r[10].s64 + 20776;
	// 82625430: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625434: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625438: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 8262543C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625440: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82625444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262544C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82625450: 396B2CBC  addi r11, r11, 0x2cbc
	ctx.r[11].s64 = ctx.r[11].s64 + 11452;
	// 82625454: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625458: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262545C: 386A1EAC  addi r3, r10, 0x1eac
	ctx.r[3].s64 = ctx.r[10].s64 + 7852;
	// 82625460: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82625464: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625468: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8262546C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625478: 4BE419A9  bl 0x82466e20
	ctx.lr = 0x8262547C;
	sub_82466E20(ctx, base);
	// 8262547C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625490 size=112
    let mut pc: u32 = 0x82625490;
    'dispatch: loop {
        match pc {
            0x82625490 => {
    //   block [0x82625490..0x82625500)
	// 82625490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262549C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826254A0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826254A4: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 826254A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826254AC: 390B5188  addi r8, r11, 0x5188
	ctx.r[8].s64 = ctx.r[11].s64 + 20872;
	// 826254B0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826254B4: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826254B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826254BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826254C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826254C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826254C8: 386A1EDC  addi r3, r10, 0x1edc
	ctx.r[3].s64 = ctx.r[10].s64 + 7900;
	// 826254CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826254D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826254D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826254D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826254DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826254E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826254E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826254E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826254EC: 4BE41935  bl 0x82466e20
	ctx.lr = 0x826254F0;
	sub_82466E20(ctx, base);
	// 826254F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826254F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826254F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826254FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625500 size=112
    let mut pc: u32 = 0x82625500;
    'dispatch: loop {
        match pc {
            0x82625500 => {
    //   block [0x82625500..0x82625570)
	// 82625500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262550C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625510: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625514: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 82625518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262551C: 390B5260  addi r8, r11, 0x5260
	ctx.r[8].s64 = ctx.r[11].s64 + 21088;
	// 82625520: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 82625524: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 82625528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262552C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625538: 386A1F0C  addi r3, r10, 0x1f0c
	ctx.r[3].s64 = ctx.r[10].s64 + 7948;
	// 8262553C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82625540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262554C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262555C: 4BE418C5  bl 0x82466e20
	ctx.lr = 0x82625560;
	sub_82466E20(ctx, base);
	// 82625560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262556C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625570 size=112
    let mut pc: u32 = 0x82625570;
    'dispatch: loop {
        match pc {
            0x82625570 => {
    //   block [0x82625570..0x826255E0)
	// 82625570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262557C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625580: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625584: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 82625588: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262558C: 390B5368  addi r8, r11, 0x5368
	ctx.r[8].s64 = ctx.r[11].s64 + 21352;
	// 82625590: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82625594: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 82625598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262559C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826255A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826255A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826255A8: 386A1F3C  addi r3, r10, 0x1f3c
	ctx.r[3].s64 = ctx.r[10].s64 + 7996;
	// 826255AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826255B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826255B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826255B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826255BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826255C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826255C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826255C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826255CC: 4BE41855  bl 0x82466e20
	ctx.lr = 0x826255D0;
	sub_82466E20(ctx, base);
	// 826255D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826255D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826255D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826255DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826255E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826255E0 size=24
    let mut pc: u32 = 0x826255E0;
    'dispatch: loop {
        match pc {
            0x826255E0 => {
    //   block [0x826255E0..0x826255F8)
	// 826255E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826255E4: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 826255E8: 394AAA98  addi r10, r10, -0x5568
	ctx.r[10].s64 = ctx.r[10].s64 + -21864;
	// 826255EC: 816B5398  lwz r11, 0x5398(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21400 as u32) ) } as u64;
	// 826255F0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826255F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826255F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826255F8 size=116
    let mut pc: u32 = 0x826255F8;
    'dispatch: loop {
        match pc {
            0x826255F8 => {
    //   block [0x826255F8..0x8262566C)
	// 826255F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826255FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625604: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625608: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262560C: 392B2CEC  addi r9, r11, 0x2cec
	ctx.r[9].s64 = ctx.r[11].s64 + 11500;
	// 82625610: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 82625614: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82625618: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8262561C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 82625620: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82625624: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 82625628: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262562C: 396BAA98  addi r11, r11, -0x5568
	ctx.r[11].s64 = ctx.r[11].s64 + -21864;
	// 82625630: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82625634: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625638: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262563C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625640: 386A1F6C  addi r3, r10, 0x1f6c
	ctx.r[3].s64 = ctx.r[10].s64 + 8044;
	// 82625644: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82625648: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262564C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625650: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82625654: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82625658: 4BE417C9  bl 0x82466e20
	ctx.lr = 0x8262565C;
	sub_82466E20(ctx, base);
	// 8262565C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625670 size=116
    let mut pc: u32 = 0x82625670;
    'dispatch: loop {
        match pc {
            0x82625670 => {
    //   block [0x82625670..0x826256E4)
	// 82625670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262567C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625680: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82625684: 390A539C  addi r8, r10, 0x539c
	ctx.r[8].s64 = ctx.r[10].s64 + 21404;
	// 82625688: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262568C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625690: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82625694: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625698: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262569C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826256A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826256A4: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826256A8: 396B2D5C  addi r11, r11, 0x2d5c
	ctx.r[11].s64 = ctx.r[11].s64 + 11612;
	// 826256AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826256B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826256B4: 386A1F9C  addi r3, r10, 0x1f9c
	ctx.r[3].s64 = ctx.r[10].s64 + 8092;
	// 826256B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826256BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826256C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826256C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826256C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826256CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826256D0: 4BE41751  bl 0x82466e20
	ctx.lr = 0x826256D4;
	sub_82466E20(ctx, base);
	// 826256D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826256D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826256DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826256E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826256E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826256E8 size=108
    let mut pc: u32 = 0x826256E8;
    'dispatch: loop {
        match pc {
            0x826256E8 => {
    //   block [0x826256E8..0x82625754)
	// 826256E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826256EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826256F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826256F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826256F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826256FC: 38EB53D0  addi r7, r11, 0x53d0
	ctx.r[7].s64 = ctx.r[11].s64 + 21456;
	// 82625700: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82625704: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 82625708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262570C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82625714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625718: 386A1FCC  addi r3, r10, 0x1fcc
	ctx.r[3].s64 = ctx.r[10].s64 + 8140;
	// 8262571C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262572C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262573C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625740: 4BE416E1  bl 0x82466e20
	ctx.lr = 0x82625744;
	sub_82466E20(ctx, base);
	// 82625744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262574C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82625758 size=24
    let mut pc: u32 = 0x82625758;
    'dispatch: loop {
        match pc {
            0x82625758 => {
    //   block [0x82625758..0x82625770)
	// 82625758: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262575C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82625760: 394AAC30  addi r10, r10, -0x53d0
	ctx.r[10].s64 = ctx.r[10].s64 + -21456;
	// 82625764: 816B53CC  lwz r11, 0x53cc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21452 as u32) ) } as u64;
	// 82625768: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8262576C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625770 size=116
    let mut pc: u32 = 0x82625770;
    'dispatch: loop {
        match pc {
            0x82625770 => {
    //   block [0x82625770..0x826257E4)
	// 82625770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262577C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625780: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625784: 392B2D80  addi r9, r11, 0x2d80
	ctx.r[9].s64 = ctx.r[11].s64 + 11648;
	// 82625788: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 8262578C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625790: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82625794: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 82625798: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262579C: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826257A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826257A4: 396BAC30  addi r11, r11, -0x53d0
	ctx.r[11].s64 = ctx.r[11].s64 + -21456;
	// 826257A8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826257AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826257B0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826257B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826257B8: 386A1FFC  addi r3, r10, 0x1ffc
	ctx.r[3].s64 = ctx.r[10].s64 + 8188;
	// 826257BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826257C0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826257C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826257C8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826257CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826257D0: 4BE41651  bl 0x82466e20
	ctx.lr = 0x826257D4;
	sub_82466E20(ctx, base);
	// 826257D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826257D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826257DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826257E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826257E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826257E8 size=112
    let mut pc: u32 = 0x826257E8;
    'dispatch: loop {
        match pc {
            0x826257E8 => {
    //   block [0x826257E8..0x82625858)
	// 826257E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826257EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826257F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826257F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826257F8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826257FC: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 82625800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625804: 390B5460  addi r8, r11, 0x5460
	ctx.r[8].s64 = ctx.r[11].s64 + 21600;
	// 82625808: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262580C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82625810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262581C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625820: 386A202C  addi r3, r10, 0x202c
	ctx.r[3].s64 = ctx.r[10].s64 + 8236;
	// 82625824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82625828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262582C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262583C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625844: 4BE415DD  bl 0x82466e20
	ctx.lr = 0x82625848;
	sub_82466E20(ctx, base);
	// 82625848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262584C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


