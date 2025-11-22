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


pub fn sub_82625858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625858 size=116
    let mut pc: u32 = 0x82625858;
    'dispatch: loop {
        match pc {
            0x82625858 => {
    //   block [0x82625858..0x826258CC)
	// 82625858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262585C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625864: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625868: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8262586C: 390A5490  addi r8, r10, 0x5490
	ctx.r[8].s64 = ctx.r[10].s64 + 21648;
	// 82625870: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625874: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625878: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 8262587C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625880: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82625884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262588C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82625890: 396B2DC8  addi r11, r11, 0x2dc8
	ctx.r[11].s64 = ctx.r[11].s64 + 11720;
	// 82625894: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625898: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262589C: 386A205C  addi r3, r10, 0x205c
	ctx.r[3].s64 = ctx.r[10].s64 + 8284;
	// 826258A0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826258A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826258A8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826258AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826258B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826258B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826258B8: 4BE41569  bl 0x82466e20
	ctx.lr = 0x826258BC;
	sub_82466E20(ctx, base);
	// 826258BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826258C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826258C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826258C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826258D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826258D0 size=108
    let mut pc: u32 = 0x826258D0;
    'dispatch: loop {
        match pc {
            0x826258D0 => {
    //   block [0x826258D0..0x8262593C)
	// 826258D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826258D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826258D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826258DC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826258E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826258E4: 38EB5520  addi r7, r11, 0x5520
	ctx.r[7].s64 = ctx.r[11].s64 + 21792;
	// 826258E8: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 826258EC: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826258F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826258F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826258F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826258FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625900: 386A208C  addi r3, r10, 0x208c
	ctx.r[3].s64 = ctx.r[10].s64 + 8332;
	// 82625904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262590C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262591C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625928: 4BE414F9  bl 0x82466e20
	ctx.lr = 0x8262592C;
	sub_82466E20(ctx, base);
	// 8262592C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625940 size=116
    let mut pc: u32 = 0x82625940;
    'dispatch: loop {
        match pc {
            0x82625940 => {
    //   block [0x82625940..0x826259B4)
	// 82625940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262594C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625950: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82625954: 390A5670  addi r8, r10, 0x5670
	ctx.r[8].s64 = ctx.r[10].s64 + 22128;
	// 82625958: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262595C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625960: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 82625964: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625968: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262596C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625974: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82625978: 396B2DEC  addi r11, r11, 0x2dec
	ctx.r[11].s64 = ctx.r[11].s64 + 11756;
	// 8262597C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625980: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625984: 386A20BC  addi r3, r10, 0x20bc
	ctx.r[3].s64 = ctx.r[10].s64 + 8380;
	// 82625988: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262598C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625990: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82625994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262599C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826259A0: 4BE41481  bl 0x82466e20
	ctx.lr = 0x826259A4;
	sub_82466E20(ctx, base);
	// 826259A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826259A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826259AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826259B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826259B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826259B8 size=112
    let mut pc: u32 = 0x826259B8;
    'dispatch: loop {
        match pc {
            0x826259B8 => {
    //   block [0x826259B8..0x82625A28)
	// 826259B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826259BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826259C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826259C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826259C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826259CC: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 826259D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826259D4: 390B5730  addi r8, r11, 0x5730
	ctx.r[8].s64 = ctx.r[11].s64 + 22320;
	// 826259D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826259DC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826259E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826259E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826259E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826259EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826259F0: 386A20EC  addi r3, r10, 0x20ec
	ctx.r[3].s64 = ctx.r[10].s64 + 8428;
	// 826259F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826259F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826259FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625A14: 4BE4140D  bl 0x82466e20
	ctx.lr = 0x82625A18;
	sub_82466E20(ctx, base);
	// 82625A18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625A28 size=112
    let mut pc: u32 = 0x82625A28;
    'dispatch: loop {
        match pc {
            0x82625A28 => {
    //   block [0x82625A28..0x82625A98)
	// 82625A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625A34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625A38: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625A3C: 38AA1EDC  addi r5, r10, 0x1edc
	ctx.r[5].s64 = ctx.r[10].s64 + 7900;
	// 82625A40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625A44: 390B5748  addi r8, r11, 0x5748
	ctx.r[8].s64 = ctx.r[11].s64 + 22344;
	// 82625A48: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82625A4C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 82625A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625A54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625A58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625A60: 386A211C  addi r3, r10, 0x211c
	ctx.r[3].s64 = ctx.r[10].s64 + 8476;
	// 82625A64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82625A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625A84: 4BE4139D  bl 0x82466e20
	ctx.lr = 0x82625A88;
	sub_82466E20(ctx, base);
	// 82625A88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625A8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625A90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625A94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625A98 size=112
    let mut pc: u32 = 0x82625A98;
    'dispatch: loop {
        match pc {
            0x82625A98 => {
    //   block [0x82625A98..0x82625B08)
	// 82625A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625AA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625AA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625AAC: 38AA1D8C  addi r5, r10, 0x1d8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7564;
	// 82625AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625AB4: 390B57D8  addi r8, r11, 0x57d8
	ctx.r[8].s64 = ctx.r[11].s64 + 22488;
	// 82625AB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82625ABC: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 82625AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625AC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625AD0: 386A214C  addi r3, r10, 0x214c
	ctx.r[3].s64 = ctx.r[10].s64 + 8524;
	// 82625AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82625AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625AF4: 4BE4132D  bl 0x82466e20
	ctx.lr = 0x82625AF8;
	sub_82466E20(ctx, base);
	// 82625AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625B08 size=116
    let mut pc: u32 = 0x82625B08;
    'dispatch: loop {
        match pc {
            0x82625B08 => {
    //   block [0x82625B08..0x82625B7C)
	// 82625B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625B14: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625B18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82625B1C: 390B57F4  addi r8, r11, 0x57f4
	ctx.r[8].s64 = ctx.r[11].s64 + 22516;
	// 82625B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625B24: 392A2E34  addi r9, r10, 0x2e34
	ctx.r[9].s64 = ctx.r[10].s64 + 11828;
	// 82625B28: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625B2C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82625B30: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82625B34: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625B3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625B4C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82625B50: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 82625B54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82625B58: 386B217C  addi r3, r11, 0x217c
	ctx.r[3].s64 = ctx.r[11].s64 + 8572;
	// 82625B5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82625B60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625B68: 4BE412B9  bl 0x82466e20
	ctx.lr = 0x82625B6C;
	sub_82466E20(ctx, base);
	// 82625B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625B80 size=100
    let mut pc: u32 = 0x82625B80;
    'dispatch: loop {
        match pc {
            0x82625B80 => {
    //   block [0x82625B80..0x82625BE4)
	// 82625B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625B8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625B94: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82625B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625BA0: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 82625BA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625BB4: 386A21AC  addi r3, r10, 0x21ac
	ctx.r[3].s64 = ctx.r[10].s64 + 8620;
	// 82625BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625BBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625BC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82625BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625BC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82625BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625BD0: 4BE41251  bl 0x82466e20
	ctx.lr = 0x82625BD4;
	sub_82466E20(ctx, base);
	// 82625BD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625BE8 size=112
    let mut pc: u32 = 0x82625BE8;
    'dispatch: loop {
        match pc {
            0x82625BE8 => {
    //   block [0x82625BE8..0x82625C58)
	// 82625BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625BF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625BF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625BFC: 38AA21AC  addi r5, r10, 0x21ac
	ctx.r[5].s64 = ctx.r[10].s64 + 8620;
	// 82625C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625C04: 390B580C  addi r8, r11, 0x580c
	ctx.r[8].s64 = ctx.r[11].s64 + 22540;
	// 82625C08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82625C0C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 82625C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625C14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625C20: 386A21DC  addi r3, r10, 0x21dc
	ctx.r[3].s64 = ctx.r[10].s64 + 8668;
	// 82625C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82625C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625C44: 4BE411DD  bl 0x82466e20
	ctx.lr = 0x82625C48;
	sub_82466E20(ctx, base);
	// 82625C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625C58 size=108
    let mut pc: u32 = 0x82625C58;
    'dispatch: loop {
        match pc {
            0x82625C58 => {
    //   block [0x82625C58..0x82625CC4)
	// 82625C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625C64: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625C68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82625C6C: 38EB5828  addi r7, r11, 0x5828
	ctx.r[7].s64 = ctx.r[11].s64 + 22568;
	// 82625C70: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82625C74: 388A446C  addi r4, r10, 0x446c
	ctx.r[4].s64 = ctx.r[10].s64 + 17516;
	// 82625C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625C7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82625C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625C88: 386A220C  addi r3, r10, 0x220c
	ctx.r[3].s64 = ctx.r[10].s64 + 8716;
	// 82625C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625CB0: 4BE41171  bl 0x82466e20
	ctx.lr = 0x82625CB4;
	sub_82466E20(ctx, base);
	// 82625CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625CC8 size=112
    let mut pc: u32 = 0x82625CC8;
    'dispatch: loop {
        match pc {
            0x82625CC8 => {
    //   block [0x82625CC8..0x82625D38)
	// 82625CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625CD4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625CDC: 392B2EA0  addi r9, r11, 0x2ea0
	ctx.r[9].s64 = ctx.r[11].s64 + 11936;
	// 82625CE0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82625CE4: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82625CE8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625CEC: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82625CF0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625CF4: 396B5888  addi r11, r11, 0x5888
	ctx.r[11].s64 = ctx.r[11].s64 + 22664;
	// 82625CF8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82625CFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625D00: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82625D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625D08: 386A223C  addi r3, r10, 0x223c
	ctx.r[3].s64 = ctx.r[10].s64 + 8764;
	// 82625D0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82625D10: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82625D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625D18: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82625D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625D20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82625D24: 4BE410FD  bl 0x82466e20
	ctx.lr = 0x82625D28;
	sub_82466E20(ctx, base);
	// 82625D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625D38 size=108
    let mut pc: u32 = 0x82625D38;
    'dispatch: loop {
        match pc {
            0x82625D38 => {
    //   block [0x82625D38..0x82625DA4)
	// 82625D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625D44: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625D48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625D4C: 38EB5948  addi r7, r11, 0x5948
	ctx.r[7].s64 = ctx.r[11].s64 + 22856;
	// 82625D50: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82625D54: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 82625D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625D5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625D60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82625D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625D68: 386A226C  addi r3, r10, 0x226c
	ctx.r[3].s64 = ctx.r[10].s64 + 8812;
	// 82625D6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625D90: 4BE41091  bl 0x82466e20
	ctx.lr = 0x82625D94;
	sub_82466E20(ctx, base);
	// 82625D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625DA8 size=116
    let mut pc: u32 = 0x82625DA8;
    'dispatch: loop {
        match pc {
            0x82625DA8 => {
    //   block [0x82625DA8..0x82625E1C)
	// 82625DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625DB4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625DB8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82625DBC: 390A5990  addi r8, r10, 0x5990
	ctx.r[8].s64 = ctx.r[10].s64 + 22928;
	// 82625DC0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625DC4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625DC8: 38AA1BDC  addi r5, r10, 0x1bdc
	ctx.r[5].s64 = ctx.r[10].s64 + 7132;
	// 82625DCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625DD0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82625DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82625DDC: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82625DE0: 396B2ED8  addi r11, r11, 0x2ed8
	ctx.r[11].s64 = ctx.r[11].s64 + 11992;
	// 82625DE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625DE8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625DEC: 386A229C  addi r3, r10, 0x229c
	ctx.r[3].s64 = ctx.r[10].s64 + 8860;
	// 82625DF0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82625DF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625DF8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82625DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625E08: 4BE41019  bl 0x82466e20
	ctx.lr = 0x82625E0C;
	sub_82466E20(ctx, base);
	// 82625E0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625E20 size=100
    let mut pc: u32 = 0x82625E20;
    'dispatch: loop {
        match pc {
            0x82625E20 => {
    //   block [0x82625E20..0x82625E84)
	// 82625E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625E2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625E34: 38AA1BDC  addi r5, r10, 0x1bdc
	ctx.r[5].s64 = ctx.r[10].s64 + 7132;
	// 82625E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625E40: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 82625E44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625E54: 386A22CC  addi r3, r10, 0x22cc
	ctx.r[3].s64 = ctx.r[10].s64 + 8908;
	// 82625E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625E5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625E60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82625E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625E68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82625E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625E70: 4BE40FB1  bl 0x82466e20
	ctx.lr = 0x82625E74;
	sub_82466E20(ctx, base);
	// 82625E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82625E88 size=24
    let mut pc: u32 = 0x82625E88;
    'dispatch: loop {
        match pc {
            0x82625E88 => {
    //   block [0x82625E88..0x82625EA0)
	// 82625E88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625E8C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82625E90: 394AAD38  addi r10, r10, -0x52c8
	ctx.r[10].s64 = ctx.r[10].s64 + -21192;
	// 82625E94: 816B5B14  lwz r11, 0x5b14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23316 as u32) ) } as u64;
	// 82625E98: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82625E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625EA0 size=116
    let mut pc: u32 = 0x82625EA0;
    'dispatch: loop {
        match pc {
            0x82625EA0 => {
    //   block [0x82625EA0..0x82625F14)
	// 82625EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625EAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625EB0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625EB4: 392B2F58  addi r9, r11, 0x2f58
	ctx.r[9].s64 = ctx.r[11].s64 + 12120;
	// 82625EB8: 38AA22CC  addi r5, r10, 0x22cc
	ctx.r[5].s64 = ctx.r[10].s64 + 8908;
	// 82625EBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625EC0: 38E90028  addi r7, r9, 0x28
	ctx.r[7].s64 = ctx.r[9].s64 + 40;
	// 82625EC4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 82625EC8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82625ECC: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 82625ED0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625ED4: 396BAD38  addi r11, r11, -0x52c8
	ctx.r[11].s64 = ctx.r[11].s64 + -21192;
	// 82625ED8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82625EDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625EE0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82625EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625EE8: 386A22FC  addi r3, r10, 0x22fc
	ctx.r[3].s64 = ctx.r[10].s64 + 8956;
	// 82625EEC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82625EF0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82625EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625EF8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82625EFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82625F00: 4BE40F21  bl 0x82466e20
	ctx.lr = 0x82625F04;
	sub_82466E20(ctx, base);
	// 82625F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625F18 size=112
    let mut pc: u32 = 0x82625F18;
    'dispatch: loop {
        match pc {
            0x82625F18 => {
    //   block [0x82625F18..0x82625F88)
	// 82625F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625F24: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82625F28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82625F2C: 38EA5B20  addi r7, r10, 0x5b20
	ctx.r[7].s64 = ctx.r[10].s64 + 23328;
	// 82625F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625F34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82625F38: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82625F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625F40: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625F44: 396B2FE4  addi r11, r11, 0x2fe4
	ctx.r[11].s64 = ctx.r[11].s64 + 12260;
	// 82625F48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82625F4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625F50: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625F54: 386A232C  addi r3, r10, 0x232c
	ctx.r[3].s64 = ctx.r[10].s64 + 9004;
	// 82625F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625F5C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82625F60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625F64: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82625F68: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625F6C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625F70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625F74: 4BE40EAD  bl 0x82466e20
	ctx.lr = 0x82625F78;
	sub_82466E20(ctx, base);
	// 82625F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625F88 size=108
    let mut pc: u32 = 0x82625F88;
    'dispatch: loop {
        match pc {
            0x82625F88 => {
    //   block [0x82625F88..0x82625FF4)
	// 82625F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82625F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82625F94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82625F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82625F9C: 38EB5B80  addi r7, r11, 0x5b80
	ctx.r[7].s64 = ctx.r[11].s64 + 23424;
	// 82625FA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82625FA4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 82625FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82625FAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82625FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82625FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82625FB8: 386A235C  addi r3, r10, 0x235c
	ctx.r[3].s64 = ctx.r[10].s64 + 9052;
	// 82625FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82625FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82625FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82625FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82625FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82625FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82625FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82625FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82625FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82625FE0: 4BE40E41  bl 0x82466e20
	ctx.lr = 0x82625FE4;
	sub_82466E20(ctx, base);
	// 82625FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82625FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82625FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82625FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82625FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82625FF8 size=116
    let mut pc: u32 = 0x82625FF8;
    'dispatch: loop {
        match pc {
            0x82625FF8 => {
    //   block [0x82625FF8..0x8262606C)
	// 82625FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82625FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626004: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8262600C: 390B5BB0  addi r8, r11, 0x5bb0
	ctx.r[8].s64 = ctx.r[11].s64 + 23472;
	// 82626010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626014: 392A2FD0  addi r9, r10, 0x2fd0
	ctx.r[9].s64 = ctx.r[10].s64 + 12240;
	// 82626018: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262601C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82626020: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82626024: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262602C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262603C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82626040: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82626044: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626048: 386B238C  addi r3, r11, 0x238c
	ctx.r[3].s64 = ctx.r[11].s64 + 9100;
	// 8262604C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82626050: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626058: 4BE40DC9  bl 0x82466e20
	ctx.lr = 0x8262605C;
	sub_82466E20(ctx, base);
	// 8262605C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626070 size=96
    let mut pc: u32 = 0x82626070;
    'dispatch: loop {
        match pc {
            0x82626070 => {
    //   block [0x82626070..0x826260D0)
	// 82626070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262607C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82626080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626084: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 82626088: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262608C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626090: 386A23BC  addi r3, r10, 0x23bc
	ctx.r[3].s64 = ctx.r[10].s64 + 9148;
	// 82626094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262609C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826260A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826260A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826260A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826260AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826260B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826260B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826260B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826260BC: 4BE40D65  bl 0x82466e20
	ctx.lr = 0x826260C0;
	sub_82466E20(ctx, base);
	// 826260C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826260C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826260C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826260CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826260D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826260D0 size=112
    let mut pc: u32 = 0x826260D0;
    'dispatch: loop {
        match pc {
            0x826260D0 => {
    //   block [0x826260D0..0x82626140)
	// 826260D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826260D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826260D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826260DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826260E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826260E4: 38AA23BC  addi r5, r10, 0x23bc
	ctx.r[5].s64 = ctx.r[10].s64 + 9148;
	// 826260E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826260EC: 390B5BC8  addi r8, r11, 0x5bc8
	ctx.r[8].s64 = ctx.r[11].s64 + 23496;
	// 826260F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826260F4: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826260F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826260FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626108: 386A23EC  addi r3, r10, 0x23ec
	ctx.r[3].s64 = ctx.r[10].s64 + 9196;
	// 8262610C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82626110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262611C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262612C: 4BE40CF5  bl 0x82466e20
	ctx.lr = 0x82626130;
	sub_82466E20(ctx, base);
	// 82626130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262613C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626140 size=112
    let mut pc: u32 = 0x82626140;
    'dispatch: loop {
        match pc {
            0x82626140 => {
    //   block [0x82626140..0x826261B0)
	// 82626140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262614C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626150: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626154: 392A3000  addi r9, r10, 0x3000
	ctx.r[9].s64 = ctx.r[10].s64 + 12288;
	// 82626158: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262615C: 390B5BF8  addi r8, r11, 0x5bf8
	ctx.r[8].s64 = ctx.r[11].s64 + 23544;
	// 82626160: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82626164: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82626168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262616C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626170: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626178: 386A241C  addi r3, r10, 0x241c
	ctx.r[3].s64 = ctx.r[10].s64 + 9244;
	// 8262617C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626180: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82626184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262618C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262619C: 4BE40C85  bl 0x82466e20
	ctx.lr = 0x826261A0;
	sub_82466E20(ctx, base);
	// 826261A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826261A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826261A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826261AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826261B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826261B0 size=108
    let mut pc: u32 = 0x826261B0;
    'dispatch: loop {
        match pc {
            0x826261B0 => {
    //   block [0x826261B0..0x8262621C)
	// 826261B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826261B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826261B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826261BC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826261C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826261C4: 38EB5CA0  addi r7, r11, 0x5ca0
	ctx.r[7].s64 = ctx.r[11].s64 + 23712;
	// 826261C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826261CC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826261D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826261D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826261D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826261DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826261E0: 386A244C  addi r3, r10, 0x244c
	ctx.r[3].s64 = ctx.r[10].s64 + 9292;
	// 826261E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826261E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826261EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826261F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826261F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826261F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826261FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626208: 4BE40C19  bl 0x82466e20
	ctx.lr = 0x8262620C;
	sub_82466E20(ctx, base);
	// 8262620C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626220 size=108
    let mut pc: u32 = 0x82626220;
    'dispatch: loop {
        match pc {
            0x82626220 => {
    //   block [0x82626220..0x8262628C)
	// 82626220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262622C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626230: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82626234: 38EB5CD0  addi r7, r11, 0x5cd0
	ctx.r[7].s64 = ctx.r[11].s64 + 23760;
	// 82626238: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262623C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82626240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262624C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626250: 386A247C  addi r3, r10, 0x247c
	ctx.r[3].s64 = ctx.r[10].s64 + 9340;
	// 82626254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262625C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262626C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626278: 4BE40BA9  bl 0x82466e20
	ctx.lr = 0x8262627C;
	sub_82466E20(ctx, base);
	// 8262627C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626290 size=28
    let mut pc: u32 = 0x82626290;
    'dispatch: loop {
        match pc {
            0x82626290 => {
    //   block [0x82626290..0x826262AC)
	// 82626290: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626294: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626298: 394AAE58  addi r10, r10, -0x51a8
	ctx.r[10].s64 = ctx.r[10].s64 + -20904;
	// 8262629C: 816B5D00  lwz r11, 0x5d00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23808 as u32) ) } as u64;
	// 826262A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826262A4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826262A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826262B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826262B0 size=112
    let mut pc: u32 = 0x826262B0;
    'dispatch: loop {
        match pc {
            0x826262B0 => {
    //   block [0x826262B0..0x82626320)
	// 826262B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826262B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826262B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826262BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826262C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826262C4: 392A3180  addi r9, r10, 0x3180
	ctx.r[9].s64 = ctx.r[10].s64 + 12672;
	// 826262C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826262CC: 390BAE58  addi r8, r11, -0x51a8
	ctx.r[8].s64 = ctx.r[11].s64 + -20904;
	// 826262D0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826262D4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826262D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826262DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826262E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826262E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826262E8: 386A24AC  addi r3, r10, 0x24ac
	ctx.r[3].s64 = ctx.r[10].s64 + 9388;
	// 826262EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826262F0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826262F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826262F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826262FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262630C: 4BE40B15  bl 0x82466e20
	ctx.lr = 0x82626310;
	sub_82466E20(ctx, base);
	// 82626310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262631C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626320 size=108
    let mut pc: u32 = 0x82626320;
    'dispatch: loop {
        match pc {
            0x82626320 => {
    //   block [0x82626320..0x8262638C)
	// 82626320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262632C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626330: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82626334: 38EB5D0C  addi r7, r11, 0x5d0c
	ctx.r[7].s64 = ctx.r[11].s64 + 23820;
	// 82626338: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262633C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82626340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626344: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262634C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626350: 386A24DC  addi r3, r10, 0x24dc
	ctx.r[3].s64 = ctx.r[10].s64 + 9436;
	// 82626354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262635C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262636C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626378: 4BE40AA9  bl 0x82466e20
	ctx.lr = 0x8262637C;
	sub_82466E20(ctx, base);
	// 8262637C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626390 size=108
    let mut pc: u32 = 0x82626390;
    'dispatch: loop {
        match pc {
            0x82626390 => {
    //   block [0x82626390..0x826263FC)
	// 82626390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262639C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826263A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826263A4: 38EB5D24  addi r7, r11, 0x5d24
	ctx.r[7].s64 = ctx.r[11].s64 + 23844;
	// 826263A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826263AC: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826263B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826263B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826263B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826263BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826263C0: 386A250C  addi r3, r10, 0x250c
	ctx.r[3].s64 = ctx.r[10].s64 + 9484;
	// 826263C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826263C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826263CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826263D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826263D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826263D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826263DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826263E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826263E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826263E8: 4BE40A39  bl 0x82466e20
	ctx.lr = 0x826263EC;
	sub_82466E20(ctx, base);
	// 826263EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826263F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826263F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826263F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626400 size=108
    let mut pc: u32 = 0x82626400;
    'dispatch: loop {
        match pc {
            0x82626400 => {
    //   block [0x82626400..0x8262646C)
	// 82626400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262640C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626410: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82626414: 38EB5D54  addi r7, r11, 0x5d54
	ctx.r[7].s64 = ctx.r[11].s64 + 23892;
	// 82626418: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262641C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 82626420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262642C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626430: 386A253C  addi r3, r10, 0x253c
	ctx.r[3].s64 = ctx.r[10].s64 + 9532;
	// 82626434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262643C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262644C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626458: 4BE409C9  bl 0x82466e20
	ctx.lr = 0x8262645C;
	sub_82466E20(ctx, base);
	// 8262645C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626470 size=24
    let mut pc: u32 = 0x82626470;
    'dispatch: loop {
        match pc {
            0x82626470 => {
    //   block [0x82626470..0x82626488)
	// 82626470: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626474: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626478: 394AAF18  addi r10, r10, -0x50e8
	ctx.r[10].s64 = ctx.r[10].s64 + -20712;
	// 8262647C: 816B5D6C  lwz r11, 0x5d6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23916 as u32) ) } as u64;
	// 82626480: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82626484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626488 size=112
    let mut pc: u32 = 0x82626488;
    'dispatch: loop {
        match pc {
            0x82626488 => {
    //   block [0x82626488..0x826264F8)
	// 82626488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262648C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626494: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626498: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262649C: 392A31D4  addi r9, r10, 0x31d4
	ctx.r[9].s64 = ctx.r[10].s64 + 12756;
	// 826264A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826264A4: 390BAF18  addi r8, r11, -0x50e8
	ctx.r[8].s64 = ctx.r[11].s64 + -20712;
	// 826264A8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826264AC: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826264B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826264B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826264B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826264BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826264C0: 386A256C  addi r3, r10, 0x256c
	ctx.r[3].s64 = ctx.r[10].s64 + 9580;
	// 826264C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826264C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826264CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826264D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826264D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826264D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826264DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826264E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826264E4: 4BE4093D  bl 0x82466e20
	ctx.lr = 0x826264E8;
	sub_82466E20(ctx, base);
	// 826264E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826264EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826264F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826264F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826264F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826264F8 size=112
    let mut pc: u32 = 0x826264F8;
    'dispatch: loop {
        match pc {
            0x826264F8 => {
    //   block [0x826264F8..0x82626568)
	// 826264F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826264FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626504: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626508: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262650C: 392A3210  addi r9, r10, 0x3210
	ctx.r[9].s64 = ctx.r[10].s64 + 12816;
	// 82626510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626514: 390B5D78  addi r8, r11, 0x5d78
	ctx.r[8].s64 = ctx.r[11].s64 + 23928;
	// 82626518: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8262651C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82626520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626524: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262652C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626530: 386A259C  addi r3, r10, 0x259c
	ctx.r[3].s64 = ctx.r[10].s64 + 9628;
	// 82626534: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626538: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8262653C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262654C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626554: 4BE408CD  bl 0x82466e20
	ctx.lr = 0x82626558;
	sub_82466E20(ctx, base);
	// 82626558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262655C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626568 size=108
    let mut pc: u32 = 0x82626568;
    'dispatch: loop {
        match pc {
            0x82626568 => {
    //   block [0x82626568..0x826265D4)
	// 82626568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262656C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626574: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626578: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262657C: 38EB5DC0  addi r7, r11, 0x5dc0
	ctx.r[7].s64 = ctx.r[11].s64 + 24000;
	// 82626580: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626584: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 82626588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262658C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626590: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626598: 386A25CC  addi r3, r10, 0x25cc
	ctx.r[3].s64 = ctx.r[10].s64 + 9676;
	// 8262659C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826265A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826265A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826265A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826265AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826265B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826265B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826265B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826265BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826265C0: 4BE40861  bl 0x82466e20
	ctx.lr = 0x826265C4;
	sub_82466E20(ctx, base);
	// 826265C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826265C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826265CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826265D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826265D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826265D8 size=108
    let mut pc: u32 = 0x826265D8;
    'dispatch: loop {
        match pc {
            0x826265D8 => {
    //   block [0x826265D8..0x82626644)
	// 826265D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826265DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826265E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826265E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826265E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826265EC: 38EB5DF0  addi r7, r11, 0x5df0
	ctx.r[7].s64 = ctx.r[11].s64 + 24048;
	// 826265F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826265F4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826265F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826265FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626608: 386A25FC  addi r3, r10, 0x25fc
	ctx.r[3].s64 = ctx.r[10].s64 + 9724;
	// 8262660C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262661C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262662C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626630: 4BE407F1  bl 0x82466e20
	ctx.lr = 0x82626634;
	sub_82466E20(ctx, base);
	// 82626634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262663C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626648 size=112
    let mut pc: u32 = 0x82626648;
    'dispatch: loop {
        match pc {
            0x82626648 => {
    //   block [0x82626648..0x826266B8)
	// 82626648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262664C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626658: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262665C: 392A3248  addi r9, r10, 0x3248
	ctx.r[9].s64 = ctx.r[10].s64 + 12872;
	// 82626660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626664: 390B5E28  addi r8, r11, 0x5e28
	ctx.r[8].s64 = ctx.r[11].s64 + 24104;
	// 82626668: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8262666C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 82626670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626674: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262667C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626680: 386A262C  addi r3, r10, 0x262c
	ctx.r[3].s64 = ctx.r[10].s64 + 9772;
	// 82626684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626688: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262668C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262669C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826266A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826266A4: 4BE4077D  bl 0x82466e20
	ctx.lr = 0x826266A8;
	sub_82466E20(ctx, base);
	// 826266A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826266AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826266B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826266B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826266B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826266B8 size=108
    let mut pc: u32 = 0x826266B8;
    'dispatch: loop {
        match pc {
            0x826266B8 => {
    //   block [0x826266B8..0x82626724)
	// 826266B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826266BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826266C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826266C4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826266C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826266CC: 38EB5E88  addi r7, r11, 0x5e88
	ctx.r[7].s64 = ctx.r[11].s64 + 24200;
	// 826266D0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826266D4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826266D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826266DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826266E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826266E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826266E8: 386A265C  addi r3, r10, 0x265c
	ctx.r[3].s64 = ctx.r[10].s64 + 9820;
	// 826266EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826266F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826266F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826266F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826266FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262670C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626710: 4BE40711  bl 0x82466e20
	ctx.lr = 0x82626714;
	sub_82466E20(ctx, base);
	// 82626714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262671C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626728 size=108
    let mut pc: u32 = 0x82626728;
    'dispatch: loop {
        match pc {
            0x82626728 => {
    //   block [0x82626728..0x82626794)
	// 82626728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262672C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626734: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626738: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8262673C: 38EB5F90  addi r7, r11, 0x5f90
	ctx.r[7].s64 = ctx.r[11].s64 + 24464;
	// 82626740: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82626744: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82626748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262674C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626758: 386A268C  addi r3, r10, 0x268c
	ctx.r[3].s64 = ctx.r[10].s64 + 9868;
	// 8262675C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262676C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262677C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626780: 4BE406A1  bl 0x82466e20
	ctx.lr = 0x82626784;
	sub_82466E20(ctx, base);
	// 82626784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262678C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626798 size=108
    let mut pc: u32 = 0x82626798;
    'dispatch: loop {
        match pc {
            0x82626798 => {
    //   block [0x82626798..0x82626804)
	// 82626798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262679C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826267A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826267A4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826267A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826267AC: 38EB5FA8  addi r7, r11, 0x5fa8
	ctx.r[7].s64 = ctx.r[11].s64 + 24488;
	// 826267B0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826267B4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826267B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826267BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826267C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826267C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826267C8: 386A26BC  addi r3, r10, 0x26bc
	ctx.r[3].s64 = ctx.r[10].s64 + 9916;
	// 826267CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826267D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826267D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826267D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826267DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826267E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826267E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826267E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826267EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826267F0: 4BE40631  bl 0x82466e20
	ctx.lr = 0x826267F4;
	sub_82466E20(ctx, base);
	// 826267F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826267F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826267FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626808 size=24
    let mut pc: u32 = 0x82626808;
    'dispatch: loop {
        match pc {
            0x82626808 => {
    //   block [0x82626808..0x82626820)
	// 82626808: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262680C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626810: 394AAFA8  addi r10, r10, -0x5058
	ctx.r[10].s64 = ctx.r[10].s64 + -20568;
	// 82626814: 816B5E24  lwz r11, 0x5e24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24100 as u32) ) } as u64;
	// 82626818: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8262681C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626820 size=108
    let mut pc: u32 = 0x82626820;
    'dispatch: loop {
        match pc {
            0x82626820 => {
    //   block [0x82626820..0x8262688C)
	// 82626820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262682C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82626830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626834: 38EBAFA8  addi r7, r11, -0x5058
	ctx.r[7].s64 = ctx.r[11].s64 + -20568;
	// 82626838: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8262683C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82626840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626844: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262684C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626850: 386A26EC  addi r3, r10, 0x26ec
	ctx.r[3].s64 = ctx.r[10].s64 + 9964;
	// 82626854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262685C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262686C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626878: 4BE405A9  bl 0x82466e20
	ctx.lr = 0x8262687C;
	sub_82466E20(ctx, base);
	// 8262687C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626890 size=24
    let mut pc: u32 = 0x82626890;
    'dispatch: loop {
        match pc {
            0x82626890 => {
    //   block [0x82626890..0x826268A8)
	// 82626890: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626894: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626898: 394AAFD8  addi r10, r10, -0x5028
	ctx.r[10].s64 = ctx.r[10].s64 + -20520;
	// 8262689C: 816B5E24  lwz r11, 0x5e24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24100 as u32) ) } as u64;
	// 826268A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826268A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826268A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826268A8 size=108
    let mut pc: u32 = 0x826268A8;
    'dispatch: loop {
        match pc {
            0x826268A8 => {
    //   block [0x826268A8..0x82626914)
	// 826268A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826268AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826268B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826268B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826268B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826268BC: 38EBAFD8  addi r7, r11, -0x5028
	ctx.r[7].s64 = ctx.r[11].s64 + -20520;
	// 826268C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826268C4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826268C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826268CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826268D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826268D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826268D8: 386A271C  addi r3, r10, 0x271c
	ctx.r[3].s64 = ctx.r[10].s64 + 10012;
	// 826268DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826268E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826268E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826268E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826268EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826268F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826268F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826268F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826268FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626900: 4BE40521  bl 0x82466e20
	ctx.lr = 0x82626904;
	sub_82466E20(ctx, base);
	// 82626904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262690C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626918 size=108
    let mut pc: u32 = 0x82626918;
    'dispatch: loop {
        match pc {
            0x82626918 => {
    //   block [0x82626918..0x82626984)
	// 82626918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262691C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626924: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262692C: 38EB6020  addi r7, r11, 0x6020
	ctx.r[7].s64 = ctx.r[11].s64 + 24608;
	// 82626930: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82626934: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82626938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262693C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626940: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626948: 386A274C  addi r3, r10, 0x274c
	ctx.r[3].s64 = ctx.r[10].s64 + 10060;
	// 8262694C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262695C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262696C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626970: 4BE404B1  bl 0x82466e20
	ctx.lr = 0x82626974;
	sub_82466E20(ctx, base);
	// 82626974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262697C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626988 size=24
    let mut pc: u32 = 0x82626988;
    'dispatch: loop {
        match pc {
            0x82626988 => {
    //   block [0x82626988..0x826269A0)
	// 82626988: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262698C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626990: 394AB008  addi r10, r10, -0x4ff8
	ctx.r[10].s64 = ctx.r[10].s64 + -20472;
	// 82626994: 816B5E24  lwz r11, 0x5e24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24100 as u32) ) } as u64;
	// 82626998: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8262699C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826269A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826269A0 size=108
    let mut pc: u32 = 0x826269A0;
    'dispatch: loop {
        match pc {
            0x826269A0 => {
    //   block [0x826269A0..0x82626A0C)
	// 826269A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826269A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826269A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826269AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826269B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826269B4: 38EBB008  addi r7, r11, -0x4ff8
	ctx.r[7].s64 = ctx.r[11].s64 + -20472;
	// 826269B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826269BC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826269C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826269C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826269C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826269CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826269D0: 386A277C  addi r3, r10, 0x277c
	ctx.r[3].s64 = ctx.r[10].s64 + 10108;
	// 826269D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826269D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826269DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826269E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826269E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826269E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826269EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826269F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826269F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826269F8: 4BE40429  bl 0x82466e20
	ctx.lr = 0x826269FC;
	sub_82466E20(ctx, base);
	// 826269FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626A10 size=112
    let mut pc: u32 = 0x82626A10;
    'dispatch: loop {
        match pc {
            0x82626A10 => {
    //   block [0x82626A10..0x82626A80)
	// 82626A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626A1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626A20: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626A24: 392A328C  addi r9, r10, 0x328c
	ctx.r[9].s64 = ctx.r[10].s64 + 12940;
	// 82626A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626A2C: 390B6038  addi r8, r11, 0x6038
	ctx.r[8].s64 = ctx.r[11].s64 + 24632;
	// 82626A30: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82626A34: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82626A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626A3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626A48: 386A27AC  addi r3, r10, 0x27ac
	ctx.r[3].s64 = ctx.r[10].s64 + 10156;
	// 82626A4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626A50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82626A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626A6C: 4BE403B5  bl 0x82466e20
	ctx.lr = 0x82626A70;
	sub_82466E20(ctx, base);
	// 82626A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626A80 size=108
    let mut pc: u32 = 0x82626A80;
    'dispatch: loop {
        match pc {
            0x82626A80 => {
    //   block [0x82626A80..0x82626AEC)
	// 82626A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626A8C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626A94: 38EB6068  addi r7, r11, 0x6068
	ctx.r[7].s64 = ctx.r[11].s64 + 24680;
	// 82626A98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626A9C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 82626AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626AA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626AB0: 386A27DC  addi r3, r10, 0x27dc
	ctx.r[3].s64 = ctx.r[10].s64 + 10204;
	// 82626AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626AD8: 4BE40349  bl 0x82466e20
	ctx.lr = 0x82626ADC;
	sub_82466E20(ctx, base);
	// 82626ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626AF0 size=108
    let mut pc: u32 = 0x82626AF0;
    'dispatch: loop {
        match pc {
            0x82626AF0 => {
    //   block [0x82626AF0..0x82626B5C)
	// 82626AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626AFC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626B04: 38EB6098  addi r7, r11, 0x6098
	ctx.r[7].s64 = ctx.r[11].s64 + 24728;
	// 82626B08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626B0C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 82626B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626B14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626B20: 386A280C  addi r3, r10, 0x280c
	ctx.r[3].s64 = ctx.r[10].s64 + 10252;
	// 82626B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626B48: 4BE402D9  bl 0x82466e20
	ctx.lr = 0x82626B4C;
	sub_82466E20(ctx, base);
	// 82626B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626B60 size=112
    let mut pc: u32 = 0x82626B60;
    'dispatch: loop {
        match pc {
            0x82626B60 => {
    //   block [0x82626B60..0x82626BD0)
	// 82626B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626B6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626B70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626B74: 38AA286C  addi r5, r10, 0x286c
	ctx.r[5].s64 = ctx.r[10].s64 + 10348;
	// 82626B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626B7C: 390B60C8  addi r8, r11, 0x60c8
	ctx.r[8].s64 = ctx.r[11].s64 + 24776;
	// 82626B80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82626B84: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82626B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626B8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626B94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626B98: 386A283C  addi r3, r10, 0x283c
	ctx.r[3].s64 = ctx.r[10].s64 + 10300;
	// 82626B9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82626BA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626BA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626BB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626BBC: 4BE40265  bl 0x82466e20
	ctx.lr = 0x82626BC0;
	sub_82466E20(ctx, base);
	// 82626BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626BD0 size=108
    let mut pc: u32 = 0x82626BD0;
    'dispatch: loop {
        match pc {
            0x82626BD0 => {
    //   block [0x82626BD0..0x82626C3C)
	// 82626BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626BDC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626BE4: 38EB60E0  addi r7, r11, 0x60e0
	ctx.r[7].s64 = ctx.r[11].s64 + 24800;
	// 82626BE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626BEC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82626BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626BF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626C00: 386A286C  addi r3, r10, 0x286c
	ctx.r[3].s64 = ctx.r[10].s64 + 10348;
	// 82626C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626C28: 4BE401F9  bl 0x82466e20
	ctx.lr = 0x82626C2C;
	sub_82466E20(ctx, base);
	// 82626C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626C40 size=108
    let mut pc: u32 = 0x82626C40;
    'dispatch: loop {
        match pc {
            0x82626C40 => {
    //   block [0x82626C40..0x82626CAC)
	// 82626C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626C4C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626C54: 38EB6110  addi r7, r11, 0x6110
	ctx.r[7].s64 = ctx.r[11].s64 + 24848;
	// 82626C58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82626C5C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82626C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626C64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626C70: 386A289C  addi r3, r10, 0x289c
	ctx.r[3].s64 = ctx.r[10].s64 + 10396;
	// 82626C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626C98: 4BE40189  bl 0x82466e20
	ctx.lr = 0x82626C9C;
	sub_82466E20(ctx, base);
	// 82626C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626CB0 size=108
    let mut pc: u32 = 0x82626CB0;
    'dispatch: loop {
        match pc {
            0x82626CB0 => {
    //   block [0x82626CB0..0x82626D1C)
	// 82626CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626CBC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626CC4: 38EB6128  addi r7, r11, 0x6128
	ctx.r[7].s64 = ctx.r[11].s64 + 24872;
	// 82626CC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626CCC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82626CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626CD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626CE0: 386A28CC  addi r3, r10, 0x28cc
	ctx.r[3].s64 = ctx.r[10].s64 + 10444;
	// 82626CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626D08: 4BE40119  bl 0x82466e20
	ctx.lr = 0x82626D0C;
	sub_82466E20(ctx, base);
	// 82626D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626D20 size=108
    let mut pc: u32 = 0x82626D20;
    'dispatch: loop {
        match pc {
            0x82626D20 => {
    //   block [0x82626D20..0x82626D8C)
	// 82626D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626D2C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626D34: 38EB6158  addi r7, r11, 0x6158
	ctx.r[7].s64 = ctx.r[11].s64 + 24920;
	// 82626D38: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82626D3C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82626D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626D44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626D50: 386A28FC  addi r3, r10, 0x28fc
	ctx.r[3].s64 = ctx.r[10].s64 + 10492;
	// 82626D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626D78: 4BE400A9  bl 0x82466e20
	ctx.lr = 0x82626D7C;
	sub_82466E20(ctx, base);
	// 82626D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626D90 size=108
    let mut pc: u32 = 0x82626D90;
    'dispatch: loop {
        match pc {
            0x82626D90 => {
    //   block [0x82626D90..0x82626DFC)
	// 82626D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626D9C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626DA4: 38EB6200  addi r7, r11, 0x6200
	ctx.r[7].s64 = ctx.r[11].s64 + 25088;
	// 82626DA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626DAC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82626DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626DB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626DC0: 386A292C  addi r3, r10, 0x292c
	ctx.r[3].s64 = ctx.r[10].s64 + 10540;
	// 82626DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626DE8: 4BE40039  bl 0x82466e20
	ctx.lr = 0x82626DEC;
	sub_82466E20(ctx, base);
	// 82626DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626E00 size=108
    let mut pc: u32 = 0x82626E00;
    'dispatch: loop {
        match pc {
            0x82626E00 => {
    //   block [0x82626E00..0x82626E6C)
	// 82626E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626E0C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626E14: 38EB6230  addi r7, r11, 0x6230
	ctx.r[7].s64 = ctx.r[11].s64 + 25136;
	// 82626E18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82626E1C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 82626E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626E24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626E30: 386A295C  addi r3, r10, 0x295c
	ctx.r[3].s64 = ctx.r[10].s64 + 10588;
	// 82626E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626E58: 4BE3FFC9  bl 0x82466e20
	ctx.lr = 0x82626E5C;
	sub_82466E20(ctx, base);
	// 82626E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626E70 size=108
    let mut pc: u32 = 0x82626E70;
    'dispatch: loop {
        match pc {
            0x82626E70 => {
    //   block [0x82626E70..0x82626EDC)
	// 82626E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626E7C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626E84: 38EB6248  addi r7, r11, 0x6248
	ctx.r[7].s64 = ctx.r[11].s64 + 25160;
	// 82626E88: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626E8C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82626E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626E94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626EA0: 386A298C  addi r3, r10, 0x298c
	ctx.r[3].s64 = ctx.r[10].s64 + 10636;
	// 82626EA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626EC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626EC8: 4BE3FF59  bl 0x82466e20
	ctx.lr = 0x82626ECC;
	sub_82466E20(ctx, base);
	// 82626ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626EE0 size=108
    let mut pc: u32 = 0x82626EE0;
    'dispatch: loop {
        match pc {
            0x82626EE0 => {
    //   block [0x82626EE0..0x82626F4C)
	// 82626EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626EEC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626EF4: 38EB6278  addi r7, r11, 0x6278
	ctx.r[7].s64 = ctx.r[11].s64 + 25208;
	// 82626EF8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82626EFC: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82626F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626F04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82626F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82626F10: 386A29BC  addi r3, r10, 0x29bc
	ctx.r[3].s64 = ctx.r[10].s64 + 10684;
	// 82626F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82626F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82626F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626F38: 4BE3FEE9  bl 0x82466e20
	ctx.lr = 0x82626F3C;
	sub_82466E20(ctx, base);
	// 82626F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82626F50 size=24
    let mut pc: u32 = 0x82626F50;
    'dispatch: loop {
        match pc {
            0x82626F50 => {
    //   block [0x82626F50..0x82626F68)
	// 82626F50: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626F54: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82626F58: 394AB038  addi r10, r10, -0x4fc8
	ctx.r[10].s64 = ctx.r[10].s64 + -20424;
	// 82626F5C: 816B6338  lwz r11, 0x6338(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25400 as u32) ) } as u64;
	// 82626F60: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82626F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626F68 size=112
    let mut pc: u32 = 0x82626F68;
    'dispatch: loop {
        match pc {
            0x82626F68 => {
    //   block [0x82626F68..0x82626FD8)
	// 82626F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626F74: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82626F78: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82626F7C: 392A32B8  addi r9, r10, 0x32b8
	ctx.r[9].s64 = ctx.r[10].s64 + 12984;
	// 82626F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626F84: 390BB038  addi r8, r11, -0x4fc8
	ctx.r[8].s64 = ctx.r[11].s64 + -20424;
	// 82626F88: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82626F8C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82626F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626F94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82626F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82626F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82626FA0: 386A29EC  addi r3, r10, 0x29ec
	ctx.r[3].s64 = ctx.r[10].s64 + 10732;
	// 82626FA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82626FA8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82626FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82626FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82626FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82626FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82626FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82626FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82626FC4: 4BE3FE5D  bl 0x82466e20
	ctx.lr = 0x82626FC8;
	sub_82466E20(ctx, base);
	// 82626FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82626FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82626FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82626FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82626FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82626FD8 size=108
    let mut pc: u32 = 0x82626FD8;
    'dispatch: loop {
        match pc {
            0x82626FD8 => {
    //   block [0x82626FD8..0x82627044)
	// 82626FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82626FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82626FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82626FE4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82626FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82626FEC: 38EB6340  addi r7, r11, 0x6340
	ctx.r[7].s64 = ctx.r[11].s64 + 25408;
	// 82626FF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82626FF4: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82626FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82626FFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627008: 386A2A1C  addi r3, r10, 0x2a1c
	ctx.r[3].s64 = ctx.r[10].s64 + 10780;
	// 8262700C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262701C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262702C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627030: 4BE3FDF1  bl 0x82466e20
	ctx.lr = 0x82627034;
	sub_82466E20(ctx, base);
	// 82627034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262703C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627048 size=112
    let mut pc: u32 = 0x82627048;
    'dispatch: loop {
        match pc {
            0x82627048 => {
    //   block [0x82627048..0x826270B8)
	// 82627048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262704C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627054: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82627058: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262705C: 392A32FC  addi r9, r10, 0x32fc
	ctx.r[9].s64 = ctx.r[10].s64 + 13052;
	// 82627060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627064: 390B6370  addi r8, r11, 0x6370
	ctx.r[8].s64 = ctx.r[11].s64 + 25456;
	// 82627068: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8262706C: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82627070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627074: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627078: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262707C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627080: 386A2A4C  addi r3, r10, 0x2a4c
	ctx.r[3].s64 = ctx.r[10].s64 + 10828;
	// 82627084: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82627088: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262708C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262709C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826270A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826270A4: 4BE3FD7D  bl 0x82466e20
	ctx.lr = 0x826270A8;
	sub_82466E20(ctx, base);
	// 826270A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826270AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826270B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826270B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826270B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826270B8 size=24
    let mut pc: u32 = 0x826270B8;
    'dispatch: loop {
        match pc {
            0x826270B8 => {
    //   block [0x826270B8..0x826270D0)
	// 826270B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826270BC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 826270C0: 394AB0B0  addi r10, r10, -0x4f50
	ctx.r[10].s64 = ctx.r[10].s64 + -20304;
	// 826270C4: 816B6430  lwz r11, 0x6430(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25648 as u32) ) } as u64;
	// 826270C8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826270CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826270D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826270D0 size=112
    let mut pc: u32 = 0x826270D0;
    'dispatch: loop {
        match pc {
            0x826270D0 => {
    //   block [0x826270D0..0x82627140)
	// 826270D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826270D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826270D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826270DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826270E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826270E4: 392A3338  addi r9, r10, 0x3338
	ctx.r[9].s64 = ctx.r[10].s64 + 13112;
	// 826270E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826270EC: 390BB0B0  addi r8, r11, -0x4f50
	ctx.r[8].s64 = ctx.r[11].s64 + -20304;
	// 826270F0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826270F4: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826270F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826270FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627108: 386A2A7C  addi r3, r10, 0x2a7c
	ctx.r[3].s64 = ctx.r[10].s64 + 10876;
	// 8262710C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82627110: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82627114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262711C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262712C: 4BE3FCF5  bl 0x82466e20
	ctx.lr = 0x82627130;
	sub_82466E20(ctx, base);
	// 82627130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262713C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627140 size=108
    let mut pc: u32 = 0x82627140;
    'dispatch: loop {
        match pc {
            0x82627140 => {
    //   block [0x82627140..0x826271AC)
	// 82627140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262714C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627154: 38EB6434  addi r7, r11, 0x6434
	ctx.r[7].s64 = ctx.r[11].s64 + 25652;
	// 82627158: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262715C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82627160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262716C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627170: 386A2AAC  addi r3, r10, 0x2aac
	ctx.r[3].s64 = ctx.r[10].s64 + 10924;
	// 82627174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262717C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262718C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627198: 4BE3FC89  bl 0x82466e20
	ctx.lr = 0x8262719C;
	sub_82466E20(ctx, base);
	// 8262719C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826271A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826271A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826271A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826271B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826271B0 size=108
    let mut pc: u32 = 0x826271B0;
    'dispatch: loop {
        match pc {
            0x826271B0 => {
    //   block [0x826271B0..0x8262721C)
	// 826271B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826271B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826271B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826271BC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826271C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826271C4: 38EB644C  addi r7, r11, 0x644c
	ctx.r[7].s64 = ctx.r[11].s64 + 25676;
	// 826271C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826271CC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826271D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826271D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826271D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826271DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826271E0: 386A2ADC  addi r3, r10, 0x2adc
	ctx.r[3].s64 = ctx.r[10].s64 + 10972;
	// 826271E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826271E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826271EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826271F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826271F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826271F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826271FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627208: 4BE3FC19  bl 0x82466e20
	ctx.lr = 0x8262720C;
	sub_82466E20(ctx, base);
	// 8262720C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82627220 size=24
    let mut pc: u32 = 0x82627220;
    'dispatch: loop {
        match pc {
            0x82627220 => {
    //   block [0x82627220..0x82627238)
	// 82627220: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627224: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82627228: 394AB0F8  addi r10, r10, -0x4f08
	ctx.r[10].s64 = ctx.r[10].s64 + -20232;
	// 8262722C: 816B647C  lwz r11, 0x647c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25724 as u32) ) } as u64;
	// 82627230: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82627234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627238 size=112
    let mut pc: u32 = 0x82627238;
    'dispatch: loop {
        match pc {
            0x82627238 => {
    //   block [0x82627238..0x826272A8)
	// 82627238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262723C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627244: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82627248: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262724C: 392A3374  addi r9, r10, 0x3374
	ctx.r[9].s64 = ctx.r[10].s64 + 13172;
	// 82627250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627254: 390BB0F8  addi r8, r11, -0x4f08
	ctx.r[8].s64 = ctx.r[11].s64 + -20232;
	// 82627258: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8262725C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82627260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262726C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627270: 386A2B0C  addi r3, r10, 0x2b0c
	ctx.r[3].s64 = ctx.r[10].s64 + 11020;
	// 82627274: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82627278: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8262727C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262728C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627294: 4BE3FB8D  bl 0x82466e20
	ctx.lr = 0x82627298;
	sub_82466E20(ctx, base);
	// 82627298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262729C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826272A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826272A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826272A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826272A8 size=108
    let mut pc: u32 = 0x826272A8;
    'dispatch: loop {
        match pc {
            0x826272A8 => {
    //   block [0x826272A8..0x82627314)
	// 826272A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826272AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826272B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826272B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826272B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826272BC: 38EB6480  addi r7, r11, 0x6480
	ctx.r[7].s64 = ctx.r[11].s64 + 25728;
	// 826272C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826272C4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826272C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826272CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826272D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826272D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826272D8: 386A2B3C  addi r3, r10, 0x2b3c
	ctx.r[3].s64 = ctx.r[10].s64 + 11068;
	// 826272DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826272E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826272E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826272E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826272EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826272F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826272F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826272F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826272FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627300: 4BE3FB21  bl 0x82466e20
	ctx.lr = 0x82627304;
	sub_82466E20(ctx, base);
	// 82627304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262730C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627318 size=108
    let mut pc: u32 = 0x82627318;
    'dispatch: loop {
        match pc {
            0x82627318 => {
    //   block [0x82627318..0x82627384)
	// 82627318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627324: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262732C: 38EB6498  addi r7, r11, 0x6498
	ctx.r[7].s64 = ctx.r[11].s64 + 25752;
	// 82627330: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82627334: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82627338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262733C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627348: 386A2B6C  addi r3, r10, 0x2b6c
	ctx.r[3].s64 = ctx.r[10].s64 + 11116;
	// 8262734C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262735C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262736C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627370: 4BE3FAB1  bl 0x82466e20
	ctx.lr = 0x82627374;
	sub_82466E20(ctx, base);
	// 82627374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262737C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627388 size=108
    let mut pc: u32 = 0x82627388;
    'dispatch: loop {
        match pc {
            0x82627388 => {
    //   block [0x82627388..0x826273F4)
	// 82627388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262738C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627394: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262739C: 38EB64E0  addi r7, r11, 0x64e0
	ctx.r[7].s64 = ctx.r[11].s64 + 25824;
	// 826273A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826273A4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826273A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826273AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826273B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826273B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826273B8: 386A2B9C  addi r3, r10, 0x2b9c
	ctx.r[3].s64 = ctx.r[10].s64 + 11164;
	// 826273BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826273C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826273C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826273C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826273CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826273D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826273D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826273D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826273DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826273E0: 4BE3FA41  bl 0x82466e20
	ctx.lr = 0x826273E4;
	sub_82466E20(ctx, base);
	// 826273E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826273E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826273EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826273F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826273F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826273F8 size=108
    let mut pc: u32 = 0x826273F8;
    'dispatch: loop {
        match pc {
            0x826273F8 => {
    //   block [0x826273F8..0x82627464)
	// 826273F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826273FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627404: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262740C: 38EB6510  addi r7, r11, 0x6510
	ctx.r[7].s64 = ctx.r[11].s64 + 25872;
	// 82627410: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82627414: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82627418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262741C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627428: 386A2BCC  addi r3, r10, 0x2bcc
	ctx.r[3].s64 = ctx.r[10].s64 + 11212;
	// 8262742C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262743C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262744C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627450: 4BE3F9D1  bl 0x82466e20
	ctx.lr = 0x82627454;
	sub_82466E20(ctx, base);
	// 82627454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262745C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627468 size=108
    let mut pc: u32 = 0x82627468;
    'dispatch: loop {
        match pc {
            0x82627468 => {
    //   block [0x82627468..0x826274D4)
	// 82627468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262746C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627474: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262747C: 38EB6630  addi r7, r11, 0x6630
	ctx.r[7].s64 = ctx.r[11].s64 + 26160;
	// 82627480: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82627484: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82627488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262748C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627490: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627498: 386A2BFC  addi r3, r10, 0x2bfc
	ctx.r[3].s64 = ctx.r[10].s64 + 11260;
	// 8262749C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826274A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826274A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826274A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826274AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826274B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826274B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826274B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826274BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826274C0: 4BE3F961  bl 0x82466e20
	ctx.lr = 0x826274C4;
	sub_82466E20(ctx, base);
	// 826274C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826274C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826274CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826274D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826274D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826274D8 size=108
    let mut pc: u32 = 0x826274D8;
    'dispatch: loop {
        match pc {
            0x826274D8 => {
    //   block [0x826274D8..0x82627544)
	// 826274D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826274DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826274E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826274E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826274E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826274EC: 38EB66C0  addi r7, r11, 0x66c0
	ctx.r[7].s64 = ctx.r[11].s64 + 26304;
	// 826274F0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826274F4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826274F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826274FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627508: 386A2C2C  addi r3, r10, 0x2c2c
	ctx.r[3].s64 = ctx.r[10].s64 + 11308;
	// 8262750C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262751C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262752C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627530: 4BE3F8F1  bl 0x82466e20
	ctx.lr = 0x82627534;
	sub_82466E20(ctx, base);
	// 82627534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262753C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627548 size=108
    let mut pc: u32 = 0x82627548;
    'dispatch: loop {
        match pc {
            0x82627548 => {
    //   block [0x82627548..0x826275B4)
	// 82627548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262754C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627554: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262755C: 38EB6780  addi r7, r11, 0x6780
	ctx.r[7].s64 = ctx.r[11].s64 + 26496;
	// 82627560: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82627564: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82627568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262756C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627578: 386A2C5C  addi r3, r10, 0x2c5c
	ctx.r[3].s64 = ctx.r[10].s64 + 11356;
	// 8262757C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262758C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262759C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826275A0: 4BE3F881  bl 0x82466e20
	ctx.lr = 0x826275A4;
	sub_82466E20(ctx, base);
	// 826275A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826275A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826275AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826275B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826275B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826275B8 size=108
    let mut pc: u32 = 0x826275B8;
    'dispatch: loop {
        match pc {
            0x826275B8 => {
    //   block [0x826275B8..0x82627624)
	// 826275B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826275BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826275C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826275C4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826275C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826275CC: 38EB6858  addi r7, r11, 0x6858
	ctx.r[7].s64 = ctx.r[11].s64 + 26712;
	// 826275D0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826275D4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826275D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826275DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826275E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826275E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826275E8: 386A2C8C  addi r3, r10, 0x2c8c
	ctx.r[3].s64 = ctx.r[10].s64 + 11404;
	// 826275EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826275F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826275F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826275F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826275FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262760C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627610: 4BE3F811  bl 0x82466e20
	ctx.lr = 0x82627614;
	sub_82466E20(ctx, base);
	// 82627614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262761C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627628 size=108
    let mut pc: u32 = 0x82627628;
    'dispatch: loop {
        match pc {
            0x82627628 => {
    //   block [0x82627628..0x82627694)
	// 82627628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262762C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627634: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262763C: 38EB6918  addi r7, r11, 0x6918
	ctx.r[7].s64 = ctx.r[11].s64 + 26904;
	// 82627640: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82627644: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82627648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262764C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627658: 386A2CBC  addi r3, r10, 0x2cbc
	ctx.r[3].s64 = ctx.r[10].s64 + 11452;
	// 8262765C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262766C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262767C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627680: 4BE3F7A1  bl 0x82466e20
	ctx.lr = 0x82627684;
	sub_82466E20(ctx, base);
	// 82627684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262768C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627698 size=112
    let mut pc: u32 = 0x82627698;
    'dispatch: loop {
        match pc {
            0x82627698 => {
    //   block [0x82627698..0x82627708)
	// 82627698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262769C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826276A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826276A4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826276A8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826276AC: 38EA69C0  addi r7, r10, 0x69c0
	ctx.r[7].s64 = ctx.r[10].s64 + 27072;
	// 826276B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826276B4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826276B8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826276BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826276C0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826276C4: 396B3388  addi r11, r11, 0x3388
	ctx.r[11].s64 = ctx.r[11].s64 + 13192;
	// 826276C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826276CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826276D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826276D4: 386A2CEC  addi r3, r10, 0x2cec
	ctx.r[3].s64 = ctx.r[10].s64 + 11500;
	// 826276D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826276DC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826276E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826276E4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826276E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826276EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826276F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826276F4: 4BE3F72D  bl 0x82466e20
	ctx.lr = 0x826276F8;
	sub_82466E20(ctx, base);
	// 826276F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826276FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627708 size=108
    let mut pc: u32 = 0x82627708;
    'dispatch: loop {
        match pc {
            0x82627708 => {
    //   block [0x82627708..0x82627774)
	// 82627708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262770C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627714: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262771C: 38EB6AE0  addi r7, r11, 0x6ae0
	ctx.r[7].s64 = ctx.r[11].s64 + 27360;
	// 82627720: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82627724: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82627728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262772C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627738: 386A2D1C  addi r3, r10, 0x2d1c
	ctx.r[3].s64 = ctx.r[10].s64 + 11548;
	// 8262773C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262774C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262775C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627760: 4BE3F6C1  bl 0x82466e20
	ctx.lr = 0x82627764;
	sub_82466E20(ctx, base);
	// 82627764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262776C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627778 size=108
    let mut pc: u32 = 0x82627778;
    'dispatch: loop {
        match pc {
            0x82627778 => {
    //   block [0x82627778..0x826277E4)
	// 82627778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262777C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627784: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262778C: 38EB6B40  addi r7, r11, 0x6b40
	ctx.r[7].s64 = ctx.r[11].s64 + 27456;
	// 82627790: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82627794: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82627798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262779C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826277A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826277A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826277A8: 386A2D4C  addi r3, r10, 0x2d4c
	ctx.r[3].s64 = ctx.r[10].s64 + 11596;
	// 826277AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826277B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826277B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826277B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826277BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826277C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826277C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826277C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826277CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826277D0: 4BE3F651  bl 0x82466e20
	ctx.lr = 0x826277D4;
	sub_82466E20(ctx, base);
	// 826277D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826277D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826277DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826277E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826277E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826277E8 size=108
    let mut pc: u32 = 0x826277E8;
    'dispatch: loop {
        match pc {
            0x826277E8 => {
    //   block [0x826277E8..0x82627854)
	// 826277E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826277EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826277F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826277F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826277F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826277FC: 38EB6C48  addi r7, r11, 0x6c48
	ctx.r[7].s64 = ctx.r[11].s64 + 27720;
	// 82627800: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82627804: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 82627808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262780C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627818: 386A2D7C  addi r3, r10, 0x2d7c
	ctx.r[3].s64 = ctx.r[10].s64 + 11644;
	// 8262781C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262782C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262783C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627840: 4BE3F5E1  bl 0x82466e20
	ctx.lr = 0x82627844;
	sub_82466E20(ctx, base);
	// 82627844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262784C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627858 size=108
    let mut pc: u32 = 0x82627858;
    'dispatch: loop {
        match pc {
            0x82627858 => {
    //   block [0x82627858..0x826278C4)
	// 82627858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262785C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627864: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262786C: 38EB6D20  addi r7, r11, 0x6d20
	ctx.r[7].s64 = ctx.r[11].s64 + 27936;
	// 82627870: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82627874: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82627878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262787C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627888: 386A2DAC  addi r3, r10, 0x2dac
	ctx.r[3].s64 = ctx.r[10].s64 + 11692;
	// 8262788C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262789C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826278A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826278A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826278A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826278AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826278B0: 4BE3F571  bl 0x82466e20
	ctx.lr = 0x826278B4;
	sub_82466E20(ctx, base);
	// 826278B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826278B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826278BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826278C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826278C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826278C8 size=108
    let mut pc: u32 = 0x826278C8;
    'dispatch: loop {
        match pc {
            0x826278C8 => {
    //   block [0x826278C8..0x82627934)
	// 826278C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826278CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826278D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826278D4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826278D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826278DC: 38EB6D68  addi r7, r11, 0x6d68
	ctx.r[7].s64 = ctx.r[11].s64 + 28008;
	// 826278E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826278E4: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826278E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826278EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826278F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826278F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826278F8: 386A2DDC  addi r3, r10, 0x2ddc
	ctx.r[3].s64 = ctx.r[10].s64 + 11740;
	// 826278FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262790C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262791C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627920: 4BE3F501  bl 0x82466e20
	ctx.lr = 0x82627924;
	sub_82466E20(ctx, base);
	// 82627924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262792C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627938 size=108
    let mut pc: u32 = 0x82627938;
    'dispatch: loop {
        match pc {
            0x82627938 => {
    //   block [0x82627938..0x826279A4)
	// 82627938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262793C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627944: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262794C: 38EB6D80  addi r7, r11, 0x6d80
	ctx.r[7].s64 = ctx.r[11].s64 + 28032;
	// 82627950: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82627954: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 82627958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262795C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627968: 386A2E0C  addi r3, r10, 0x2e0c
	ctx.r[3].s64 = ctx.r[10].s64 + 11788;
	// 8262796C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262797C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262798C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627990: 4BE3F491  bl 0x82466e20
	ctx.lr = 0x82627994;
	sub_82466E20(ctx, base);
	// 82627994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262799C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826279A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826279A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826279A8 size=108
    let mut pc: u32 = 0x826279A8;
    'dispatch: loop {
        match pc {
            0x826279A8 => {
    //   block [0x826279A8..0x82627A14)
	// 826279A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826279AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826279B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826279B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826279B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826279BC: 38EB6DC8  addi r7, r11, 0x6dc8
	ctx.r[7].s64 = ctx.r[11].s64 + 28104;
	// 826279C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826279C4: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826279C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826279CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826279D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826279D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826279D8: 386A2E3C  addi r3, r10, 0x2e3c
	ctx.r[3].s64 = ctx.r[10].s64 + 11836;
	// 826279DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826279E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826279E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826279E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826279EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826279F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826279F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826279F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826279FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627A00: 4BE3F421  bl 0x82466e20
	ctx.lr = 0x82627A04;
	sub_82466E20(ctx, base);
	// 82627A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627A18 size=112
    let mut pc: u32 = 0x82627A18;
    'dispatch: loop {
        match pc {
            0x82627A18 => {
    //   block [0x82627A18..0x82627A88)
	// 82627A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627A24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627A28: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627A2C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82627A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627A34: 390B6DE0  addi r8, r11, 0x6de0
	ctx.r[8].s64 = ctx.r[11].s64 + 28128;
	// 82627A38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82627A3C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82627A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627A44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627A50: 386A2E6C  addi r3, r10, 0x2e6c
	ctx.r[3].s64 = ctx.r[10].s64 + 11884;
	// 82627A54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82627A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627A74: 4BE3F3AD  bl 0x82466e20
	ctx.lr = 0x82627A78;
	sub_82466E20(ctx, base);
	// 82627A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627A88 size=108
    let mut pc: u32 = 0x82627A88;
    'dispatch: loop {
        match pc {
            0x82627A88 => {
    //   block [0x82627A88..0x82627AF4)
	// 82627A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627A94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627A9C: 38EB6E28  addi r7, r11, 0x6e28
	ctx.r[7].s64 = ctx.r[11].s64 + 28200;
	// 82627AA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82627AA4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82627AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627AAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82627AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627AB8: 386A2E9C  addi r3, r10, 0x2e9c
	ctx.r[3].s64 = ctx.r[10].s64 + 11932;
	// 82627ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82627AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627AE0: 4BE3F341  bl 0x82466e20
	ctx.lr = 0x82627AE4;
	sub_82466E20(ctx, base);
	// 82627AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627AF8 size=112
    let mut pc: u32 = 0x82627AF8;
    'dispatch: loop {
        match pc {
            0x82627AF8 => {
    //   block [0x82627AF8..0x82627B68)
	// 82627AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627B04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627B08: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627B0C: 38AA2E9C  addi r5, r10, 0x2e9c
	ctx.r[5].s64 = ctx.r[10].s64 + 11932;
	// 82627B10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627B14: 390B6E88  addi r8, r11, 0x6e88
	ctx.r[8].s64 = ctx.r[11].s64 + 28296;
	// 82627B18: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82627B1C: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82627B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627B24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627B30: 386A2ECC  addi r3, r10, 0x2ecc
	ctx.r[3].s64 = ctx.r[10].s64 + 11980;
	// 82627B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82627B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627B54: 4BE3F2CD  bl 0x82466e20
	ctx.lr = 0x82627B58;
	sub_82466E20(ctx, base);
	// 82627B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627B68 size=96
    let mut pc: u32 = 0x82627B68;
    'dispatch: loop {
        match pc {
            0x82627B68 => {
    //   block [0x82627B68..0x82627BC8)
	// 82627B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627B74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627B7C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82627B80: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627B88: 386A2EFC  addi r3, r10, 0x2efc
	ctx.r[3].s64 = ctx.r[10].s64 + 12028;
	// 82627B8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627B94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82627B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627BA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627BB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627BB4: 4BE3F26D  bl 0x82466e20
	ctx.lr = 0x82627BB8;
	sub_82466E20(ctx, base);
	// 82627BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627BC8 size=112
    let mut pc: u32 = 0x82627BC8;
    'dispatch: loop {
        match pc {
            0x82627BC8 => {
    //   block [0x82627BC8..0x82627C38)
	// 82627BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627BD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627BD8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627BDC: 38AA4CFC  addi r5, r10, 0x4cfc
	ctx.r[5].s64 = ctx.r[10].s64 + 19708;
	// 82627BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627BE4: 390B6ED0  addi r8, r11, 0x6ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 28368;
	// 82627BE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82627BEC: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82627BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627BF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627C00: 386A2F2C  addi r3, r10, 0x2f2c
	ctx.r[3].s64 = ctx.r[10].s64 + 12076;
	// 82627C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82627C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627C24: 4BE3F1FD  bl 0x82466e20
	ctx.lr = 0x82627C28;
	sub_82466E20(ctx, base);
	// 82627C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627C38 size=96
    let mut pc: u32 = 0x82627C38;
    'dispatch: loop {
        match pc {
            0x82627C38 => {
    //   block [0x82627C38..0x82627C98)
	// 82627C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627C44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627C4C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82627C50: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627C58: 386A2F5C  addi r3, r10, 0x2f5c
	ctx.r[3].s64 = ctx.r[10].s64 + 12124;
	// 82627C5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627C64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82627C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627C78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627C80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627C84: 4BE3F19D  bl 0x82466e20
	ctx.lr = 0x82627C88;
	sub_82466E20(ctx, base);
	// 82627C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627C98 size=100
    let mut pc: u32 = 0x82627C98;
    'dispatch: loop {
        match pc {
            0x82627C98 => {
    //   block [0x82627C98..0x82627CFC)
	// 82627C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627CA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627CAC: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82627CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627CB8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82627CBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627CC4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82627CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627CCC: 386A2F8C  addi r3, r10, 0x2f8c
	ctx.r[3].s64 = ctx.r[10].s64 + 12172;
	// 82627CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627CD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627CD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627CE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627CE8: 4BE3F139  bl 0x82466e20
	ctx.lr = 0x82627CEC;
	sub_82466E20(ctx, base);
	// 82627CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627D00 size=104
    let mut pc: u32 = 0x82627D00;
    'dispatch: loop {
        match pc {
            0x82627D00 => {
    //   block [0x82627D00..0x82627D68)
	// 82627D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627D0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82627D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627D14: 392A3400  addi r9, r10, 0x3400
	ctx.r[9].s64 = ctx.r[10].s64 + 13312;
	// 82627D18: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627D20: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82627D24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627D34: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 82627D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627D3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627D40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627D48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627D4C: 386A2FBC  addi r3, r10, 0x2fbc
	ctx.r[3].s64 = ctx.r[10].s64 + 12220;
	// 82627D50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82627D54: 4BE3F0CD  bl 0x82466e20
	ctx.lr = 0x82627D58;
	sub_82466E20(ctx, base);
	// 82627D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627D5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627D60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627D64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627D68 size=96
    let mut pc: u32 = 0x82627D68;
    'dispatch: loop {
        match pc {
            0x82627D68 => {
    //   block [0x82627D68..0x82627DC8)
	// 82627D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627D74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627D7C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82627D80: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627D88: 386A2FEC  addi r3, r10, 0x2fec
	ctx.r[3].s64 = ctx.r[10].s64 + 12268;
	// 82627D8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627D94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82627D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627DA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627DB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627DB4: 4BE3F06D  bl 0x82466e20
	ctx.lr = 0x82627DB8;
	sub_82466E20(ctx, base);
	// 82627DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627DC8 size=100
    let mut pc: u32 = 0x82627DC8;
    'dispatch: loop {
        match pc {
            0x82627DC8 => {
    //   block [0x82627DC8..0x82627E2C)
	// 82627DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627DD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627DDC: 38AA2FBC  addi r5, r10, 0x2fbc
	ctx.r[5].s64 = ctx.r[10].s64 + 12220;
	// 82627DE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627DE8: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 82627DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627DFC: 386A301C  addi r3, r10, 0x301c
	ctx.r[3].s64 = ctx.r[10].s64 + 12316;
	// 82627E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627E04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627E08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627E10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627E18: 4BE3F009  bl 0x82466e20
	ctx.lr = 0x82627E1C;
	sub_82466E20(ctx, base);
	// 82627E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627E30 size=112
    let mut pc: u32 = 0x82627E30;
    'dispatch: loop {
        match pc {
            0x82627E30 => {
    //   block [0x82627E30..0x82627EA0)
	// 82627E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627E40: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627E44: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 82627E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627E4C: 390B6F34  addi r8, r11, 0x6f34
	ctx.r[8].s64 = ctx.r[11].s64 + 28468;
	// 82627E50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82627E54: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82627E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627E5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627E68: 386A304C  addi r3, r10, 0x304c
	ctx.r[3].s64 = ctx.r[10].s64 + 12364;
	// 82627E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82627E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627E8C: 4BE3EF95  bl 0x82466e20
	ctx.lr = 0x82627E90;
	sub_82466E20(ctx, base);
	// 82627E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627EA0 size=112
    let mut pc: u32 = 0x82627EA0;
    'dispatch: loop {
        match pc {
            0x82627EA0 => {
    //   block [0x82627EA0..0x82627F10)
	// 82627EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627EAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627EB0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627EB4: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 82627EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627EBC: 390B6F64  addi r8, r11, 0x6f64
	ctx.r[8].s64 = ctx.r[11].s64 + 28516;
	// 82627EC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82627EC4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82627EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627ECC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82627ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627ED8: 386A307C  addi r3, r10, 0x307c
	ctx.r[3].s64 = ctx.r[10].s64 + 12412;
	// 82627EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82627EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627EFC: 4BE3EF25  bl 0x82466e20
	ctx.lr = 0x82627F00;
	sub_82466E20(ctx, base);
	// 82627F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627F10 size=100
    let mut pc: u32 = 0x82627F10;
    'dispatch: loop {
        match pc {
            0x82627F10 => {
    //   block [0x82627F10..0x82627F74)
	// 82627F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627F1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627F24: 38AA2F8C  addi r5, r10, 0x2f8c
	ctx.r[5].s64 = ctx.r[10].s64 + 12172;
	// 82627F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627F30: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82627F34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82627F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627F44: 386A30AC  addi r3, r10, 0x30ac
	ctx.r[3].s64 = ctx.r[10].s64 + 12460;
	// 82627F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627F4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627F50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627F58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627F60: 4BE3EEC1  bl 0x82466e20
	ctx.lr = 0x82627F64;
	sub_82466E20(ctx, base);
	// 82627F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627F78 size=96
    let mut pc: u32 = 0x82627F78;
    'dispatch: loop {
        match pc {
            0x82627F78 => {
    //   block [0x82627F78..0x82627FD8)
	// 82627F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627F84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82627F8C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82627F90: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82627F98: 386A30DC  addi r3, r10, 0x30dc
	ctx.r[3].s64 = ctx.r[10].s64 + 12508;
	// 82627F9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82627FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82627FA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82627FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82627FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82627FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82627FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82627FB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82627FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82627FC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82627FC4: 4BE3EE5D  bl 0x82466e20
	ctx.lr = 0x82627FC8;
	sub_82466E20(ctx, base);
	// 82627FC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82627FCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82627FD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82627FD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82627FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82627FD8 size=112
    let mut pc: u32 = 0x82627FD8;
    'dispatch: loop {
        match pc {
            0x82627FD8 => {
    //   block [0x82627FD8..0x82628048)
	// 82627FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82627FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82627FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82627FE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82627FE8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82627FEC: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82627FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82627FF4: 390B6F7C  addi r8, r11, 0x6f7c
	ctx.r[8].s64 = ctx.r[11].s64 + 28540;
	// 82627FF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82627FFC: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 82628000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628004: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262800C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628010: 386A310C  addi r3, r10, 0x310c
	ctx.r[3].s64 = ctx.r[10].s64 + 12556;
	// 82628014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262801C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262802C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628034: 4BE3EDED  bl 0x82466e20
	ctx.lr = 0x82628038;
	sub_82466E20(ctx, base);
	// 82628038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262803C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628048 size=96
    let mut pc: u32 = 0x82628048;
    'dispatch: loop {
        match pc {
            0x82628048 => {
    //   block [0x82628048..0x826280A8)
	// 82628048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262804C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628054: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262805C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82628060: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628068: 386A313C  addi r3, r10, 0x313c
	ctx.r[3].s64 = ctx.r[10].s64 + 12604;
	// 8262806C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628074: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82628078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262807C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628088: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262808C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628090: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82628094: 4BE3ED8D  bl 0x82466e20
	ctx.lr = 0x82628098;
	sub_82466E20(ctx, base);
	// 82628098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262809C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826280A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826280A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826280A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826280A8 size=112
    let mut pc: u32 = 0x826280A8;
    'dispatch: loop {
        match pc {
            0x826280A8 => {
    //   block [0x826280A8..0x82628118)
	// 826280A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826280AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826280B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826280B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826280B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826280BC: 38AA313C  addi r5, r10, 0x313c
	ctx.r[5].s64 = ctx.r[10].s64 + 12604;
	// 826280C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826280C4: 390B6F94  addi r8, r11, 0x6f94
	ctx.r[8].s64 = ctx.r[11].s64 + 28564;
	// 826280C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826280CC: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826280D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826280D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826280D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826280DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826280E0: 386A316C  addi r3, r10, 0x316c
	ctx.r[3].s64 = ctx.r[10].s64 + 12652;
	// 826280E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826280E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826280EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826280F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826280F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826280F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826280FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628104: 4BE3ED1D  bl 0x82466e20
	ctx.lr = 0x82628108;
	sub_82466E20(ctx, base);
	// 82628108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262810C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628118 size=108
    let mut pc: u32 = 0x82628118;
    'dispatch: loop {
        match pc {
            0x82628118 => {
    //   block [0x82628118..0x82628184)
	// 82628118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262811C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628124: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262812C: 38EB6FB0  addi r7, r11, 0x6fb0
	ctx.r[7].s64 = ctx.r[11].s64 + 28592;
	// 82628130: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82628134: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82628138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262813C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628148: 386A319C  addi r3, r10, 0x319c
	ctx.r[3].s64 = ctx.r[10].s64 + 12700;
	// 8262814C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262815C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262816C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628170: 4BE3ECB1  bl 0x82466e20
	ctx.lr = 0x82628174;
	sub_82466E20(ctx, base);
	// 82628174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262817C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628188 size=112
    let mut pc: u32 = 0x82628188;
    'dispatch: loop {
        match pc {
            0x82628188 => {
    //   block [0x82628188..0x826281F8)
	// 82628188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262818C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628194: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628198: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262819C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 826281A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826281A4: 390B7010  addi r8, r11, 0x7010
	ctx.r[8].s64 = ctx.r[11].s64 + 28688;
	// 826281A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826281AC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826281B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826281B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826281B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826281BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826281C0: 386A31CC  addi r3, r10, 0x31cc
	ctx.r[3].s64 = ctx.r[10].s64 + 12748;
	// 826281C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826281C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826281CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826281D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826281D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826281D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826281DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826281E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826281E4: 4BE3EC3D  bl 0x82466e20
	ctx.lr = 0x826281E8;
	sub_82466E20(ctx, base);
	// 826281E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826281EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826281F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826281F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826281F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826281F8 size=112
    let mut pc: u32 = 0x826281F8;
    'dispatch: loop {
        match pc {
            0x826281F8 => {
    //   block [0x826281F8..0x82628268)
	// 826281F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826281FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628204: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628208: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262820C: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 82628210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628214: 390B7028  addi r8, r11, 0x7028
	ctx.r[8].s64 = ctx.r[11].s64 + 28712;
	// 82628218: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262821C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 82628220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262822C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628230: 386A31FC  addi r3, r10, 0x31fc
	ctx.r[3].s64 = ctx.r[10].s64 + 12796;
	// 82628234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262823C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262824C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628254: 4BE3EBCD  bl 0x82466e20
	ctx.lr = 0x82628258;
	sub_82466E20(ctx, base);
	// 82628258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262825C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628268 size=112
    let mut pc: u32 = 0x82628268;
    'dispatch: loop {
        match pc {
            0x82628268 => {
    //   block [0x82628268..0x826282D8)
	// 82628268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262826C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628278: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262827C: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 82628280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628284: 390B7058  addi r8, r11, 0x7058
	ctx.r[8].s64 = ctx.r[11].s64 + 28760;
	// 82628288: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262828C: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 82628290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262829C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826282A0: 386A322C  addi r3, r10, 0x322c
	ctx.r[3].s64 = ctx.r[10].s64 + 12844;
	// 826282A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826282A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826282AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826282B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826282B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826282B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826282BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826282C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826282C4: 4BE3EB5D  bl 0x82466e20
	ctx.lr = 0x826282C8;
	sub_82466E20(ctx, base);
	// 826282C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826282CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826282D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826282D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826282D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826282D8 size=116
    let mut pc: u32 = 0x826282D8;
    'dispatch: loop {
        match pc {
            0x826282D8 => {
    //   block [0x826282D8..0x8262834C)
	// 826282D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826282DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826282E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826282E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826282E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826282EC: 390B7070  addi r8, r11, 0x7070
	ctx.r[8].s64 = ctx.r[11].s64 + 28784;
	// 826282F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826282F4: 392A342C  addi r9, r10, 0x342c
	ctx.r[9].s64 = ctx.r[10].s64 + 13356;
	// 826282F8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826282FC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82628300: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82628304: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262830C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262831C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82628320: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 82628324: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82628328: 386B325C  addi r3, r11, 0x325c
	ctx.r[3].s64 = ctx.r[11].s64 + 12892;
	// 8262832C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82628330: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628338: 4BE3EAE9  bl 0x82466e20
	ctx.lr = 0x8262833C;
	sub_82466E20(ctx, base);
	// 8262833C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628350 size=112
    let mut pc: u32 = 0x82628350;
    'dispatch: loop {
        match pc {
            0x82628350 => {
    //   block [0x82628350..0x826283C0)
	// 82628350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262835C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628360: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628364: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 82628368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262836C: 390B70A0  addi r8, r11, 0x70a0
	ctx.r[8].s64 = ctx.r[11].s64 + 28832;
	// 82628370: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82628374: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82628378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262837C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628388: 386A328C  addi r3, r10, 0x328c
	ctx.r[3].s64 = ctx.r[10].s64 + 12940;
	// 8262838C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262839C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826283A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826283A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826283A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826283AC: 4BE3EA75  bl 0x82466e20
	ctx.lr = 0x826283B0;
	sub_82466E20(ctx, base);
	// 826283B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826283B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826283B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826283BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826283C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826283C0 size=112
    let mut pc: u32 = 0x826283C0;
    'dispatch: loop {
        match pc {
            0x826283C0 => {
    //   block [0x826283C0..0x82628430)
	// 826283C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826283C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826283C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826283CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826283D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826283D4: 38AA382C  addi r5, r10, 0x382c
	ctx.r[5].s64 = ctx.r[10].s64 + 14380;
	// 826283D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826283DC: 390B70B8  addi r8, r11, 0x70b8
	ctx.r[8].s64 = ctx.r[11].s64 + 28856;
	// 826283E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826283E4: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826283E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826283EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826283F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826283F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826283F8: 386A32BC  addi r3, r10, 0x32bc
	ctx.r[3].s64 = ctx.r[10].s64 + 12988;
	// 826283FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262840C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262841C: 4BE3EA05  bl 0x82466e20
	ctx.lr = 0x82628420;
	sub_82466E20(ctx, base);
	// 82628420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262842C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628430 size=116
    let mut pc: u32 = 0x82628430;
    'dispatch: loop {
        match pc {
            0x82628430 => {
    //   block [0x82628430..0x826284A4)
	// 82628430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262843C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82628440: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82628444: 390A70D0  addi r8, r10, 0x70d0
	ctx.r[8].s64 = ctx.r[10].s64 + 28880;
	// 82628448: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8262844C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82628450: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82628454: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628458: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8262845C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628460: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628464: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82628468: 396B3440  addi r11, r11, 0x3440
	ctx.r[11].s64 = ctx.r[11].s64 + 13376;
	// 8262846C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628470: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82628474: 386A32EC  addi r3, r10, 0x32ec
	ctx.r[3].s64 = ctx.r[10].s64 + 13036;
	// 82628478: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8262847C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628480: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82628484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262848C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628490: 4BE3E991  bl 0x82466e20
	ctx.lr = 0x82628494;
	sub_82466E20(ctx, base);
	// 82628494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262849C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826284A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826284A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826284A8 size=112
    let mut pc: u32 = 0x826284A8;
    'dispatch: loop {
        match pc {
            0x826284A8 => {
    //   block [0x826284A8..0x82628518)
	// 826284A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826284AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826284B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826284B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826284B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826284BC: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 826284C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826284C4: 390B7148  addi r8, r11, 0x7148
	ctx.r[8].s64 = ctx.r[11].s64 + 29000;
	// 826284C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826284CC: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826284D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826284D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826284D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826284DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826284E0: 386A331C  addi r3, r10, 0x331c
	ctx.r[3].s64 = ctx.r[10].s64 + 13084;
	// 826284E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826284E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826284EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826284F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826284F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826284F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826284FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628504: 4BE3E91D  bl 0x82466e20
	ctx.lr = 0x82628508;
	sub_82466E20(ctx, base);
	// 82628508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262850C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628518 size=112
    let mut pc: u32 = 0x82628518;
    'dispatch: loop {
        match pc {
            0x82628518 => {
    //   block [0x82628518..0x82628588)
	// 82628518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262851C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628524: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628528: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262852C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82628530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628534: 390B7190  addi r8, r11, 0x7190
	ctx.r[8].s64 = ctx.r[11].s64 + 29072;
	// 82628538: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262853C: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82628540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628544: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262854C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628550: 386A334C  addi r3, r10, 0x334c
	ctx.r[3].s64 = ctx.r[10].s64 + 13132;
	// 82628554: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262855C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262856C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628574: 4BE3E8AD  bl 0x82466e20
	ctx.lr = 0x82628578;
	sub_82466E20(ctx, base);
	// 82628578: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262857C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628588 size=112
    let mut pc: u32 = 0x82628588;
    'dispatch: loop {
        match pc {
            0x82628588 => {
    //   block [0x82628588..0x826285F8)
	// 82628588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262858C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628594: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628598: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262859C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 826285A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826285A4: 390B71C0  addi r8, r11, 0x71c0
	ctx.r[8].s64 = ctx.r[11].s64 + 29120;
	// 826285A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826285AC: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826285B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826285B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826285B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826285BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826285C0: 386A337C  addi r3, r10, 0x337c
	ctx.r[3].s64 = ctx.r[10].s64 + 13180;
	// 826285C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826285C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826285CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826285D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826285D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826285D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826285DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826285E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826285E4: 4BE3E83D  bl 0x82466e20
	ctx.lr = 0x826285E8;
	sub_82466E20(ctx, base);
	// 826285E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826285EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826285F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826285F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826285F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826285F8 size=108
    let mut pc: u32 = 0x826285F8;
    'dispatch: loop {
        match pc {
            0x826285F8 => {
    //   block [0x826285F8..0x82628664)
	// 826285F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826285FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628604: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262860C: 38EB71F0  addi r7, r11, 0x71f0
	ctx.r[7].s64 = ctx.r[11].s64 + 29168;
	// 82628610: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82628614: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 82628618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262861C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628628: 386A33AC  addi r3, r10, 0x33ac
	ctx.r[3].s64 = ctx.r[10].s64 + 13228;
	// 8262862C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262863C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262864C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628650: 4BE3E7D1  bl 0x82466e20
	ctx.lr = 0x82628654;
	sub_82466E20(ctx, base);
	// 82628654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262865C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628668 size=112
    let mut pc: u32 = 0x82628668;
    'dispatch: loop {
        match pc {
            0x82628668 => {
    //   block [0x82628668..0x826286D8)
	// 82628668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262866C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628674: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628678: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262867C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82628680: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628684: 390B7238  addi r8, r11, 0x7238
	ctx.r[8].s64 = ctx.r[11].s64 + 29240;
	// 82628688: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8262868C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82628690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628694: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628698: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262869C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826286A0: 386A33DC  addi r3, r10, 0x33dc
	ctx.r[3].s64 = ctx.r[10].s64 + 13276;
	// 826286A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826286A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826286AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826286B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826286B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826286B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826286BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826286C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826286C4: 4BE3E75D  bl 0x82466e20
	ctx.lr = 0x826286C8;
	sub_82466E20(ctx, base);
	// 826286C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826286CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826286D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826286D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826286D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826286D8 size=116
    let mut pc: u32 = 0x826286D8;
    'dispatch: loop {
        match pc {
            0x826286D8 => {
    //   block [0x826286D8..0x8262874C)
	// 826286D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826286DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826286E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826286E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826286E8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826286EC: 392B3480  addi r9, r11, 0x3480
	ctx.r[9].s64 = ctx.r[11].s64 + 13440;
	// 826286F0: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 826286F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826286F8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826286FC: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82628700: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628704: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 82628708: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8262870C: 396B72B8  addi r11, r11, 0x72b8
	ctx.r[11].s64 = ctx.r[11].s64 + 29368;
	// 82628710: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82628714: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628718: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8262871C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628720: 386A340C  addi r3, r10, 0x340c
	ctx.r[3].s64 = ctx.r[10].s64 + 13324;
	// 82628724: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82628728: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8262872C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628730: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82628734: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82628738: 4BE3E6E9  bl 0x82466e20
	ctx.lr = 0x8262873C;
	sub_82466E20(ctx, base);
	// 8262873C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82628750 size=36
    let mut pc: u32 = 0x82628750;
    'dispatch: loop {
        match pc {
            0x82628750 => {
    //   block [0x82628750..0x82628774)
	// 82628750: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628754: 814B734C  lwz r10, 0x734c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29516 as u32) ) } as u64;
	// 82628758: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8262875C: 396BB170  addi r11, r11, -0x4e90
	ctx.r[11].s64 = ctx.r[11].s64 + -20112;
	// 82628760: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82628764: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82628768: 814A72B4  lwz r10, 0x72b4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29364 as u32) ) } as u64;
	// 8262876C: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82628770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628778 size=108
    let mut pc: u32 = 0x82628778;
    'dispatch: loop {
        match pc {
            0x82628778 => {
    //   block [0x82628778..0x826287E4)
	// 82628778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262877C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628784: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82628788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262878C: 38EBB170  addi r7, r11, -0x4e90
	ctx.r[7].s64 = ctx.r[11].s64 + -20112;
	// 82628790: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 82628794: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 82628798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262879C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826287A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826287A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826287A8: 386A343C  addi r3, r10, 0x343c
	ctx.r[3].s64 = ctx.r[10].s64 + 13372;
	// 826287AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826287B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826287B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826287B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826287BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826287C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826287C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826287C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826287CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826287D0: 4BE3E651  bl 0x82466e20
	ctx.lr = 0x826287D4;
	sub_82466E20(ctx, base);
	// 826287D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826287D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826287DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826287E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826287E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826287E8 size=24
    let mut pc: u32 = 0x826287E8;
    'dispatch: loop {
        match pc {
            0x826287E8 => {
    //   block [0x826287E8..0x82628800)
	// 826287E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826287EC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 826287F0: 394AB230  addi r10, r10, -0x4dd0
	ctx.r[10].s64 = ctx.r[10].s64 + -19920;
	// 826287F4: 816B72B4  lwz r11, 0x72b4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29364 as u32) ) } as u64;
	// 826287F8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826287FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628800 size=112
    let mut pc: u32 = 0x82628800;
    'dispatch: loop {
        match pc {
            0x82628800 => {
    //   block [0x82628800..0x82628870)
	// 82628800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262880C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628810: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82628814: 38AA343C  addi r5, r10, 0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + 13372;
	// 82628818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262881C: 390BB230  addi r8, r11, -0x4dd0
	ctx.r[8].s64 = ctx.r[11].s64 + -19920;
	// 82628820: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82628824: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 82628828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262882C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628838: 386A346C  addi r3, r10, 0x346c
	ctx.r[3].s64 = ctx.r[10].s64 + 13420;
	// 8262883C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262884C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262885C: 4BE3E5C5  bl 0x82466e20
	ctx.lr = 0x82628860;
	sub_82466E20(ctx, base);
	// 82628860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262886C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628870 size=112
    let mut pc: u32 = 0x82628870;
    'dispatch: loop {
        match pc {
            0x82628870 => {
    //   block [0x82628870..0x826288E0)
	// 82628870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262887C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628880: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628884: 38AA343C  addi r5, r10, 0x343c
	ctx.r[5].s64 = ctx.r[10].s64 + 13372;
	// 82628888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262888C: 390B7350  addi r8, r11, 0x7350
	ctx.r[8].s64 = ctx.r[11].s64 + 29520;
	// 82628890: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82628894: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 82628898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262889C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826288A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826288A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826288A8: 386A349C  addi r3, r10, 0x349c
	ctx.r[3].s64 = ctx.r[10].s64 + 13468;
	// 826288AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826288B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826288B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826288B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826288BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826288C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826288C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826288C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826288CC: 4BE3E555  bl 0x82466e20
	ctx.lr = 0x826288D0;
	sub_82466E20(ctx, base);
	// 826288D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826288D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826288D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826288DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826288E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826288E0 size=116
    let mut pc: u32 = 0x826288E0;
    'dispatch: loop {
        match pc {
            0x826288E0 => {
    //   block [0x826288E0..0x82628954)
	// 826288E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826288E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826288E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826288EC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826288F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826288F4: 390B73B0  addi r8, r11, 0x73b0
	ctx.r[8].s64 = ctx.r[11].s64 + 29616;
	// 826288F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826288FC: 392A3500  addi r9, r10, 0x3500
	ctx.r[9].s64 = ctx.r[10].s64 + 13568;
	// 82628900: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628904: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82628908: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 8262890C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628914: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262891C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628924: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82628928: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 8262892C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82628930: 386B34CC  addi r3, r11, 0x34cc
	ctx.r[3].s64 = ctx.r[11].s64 + 13516;
	// 82628934: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82628938: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8262893C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628940: 4BE3E4E1  bl 0x82466e20
	ctx.lr = 0x82628944;
	sub_82466E20(ctx, base);
	// 82628944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262894C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628958 size=100
    let mut pc: u32 = 0x82628958;
    'dispatch: loop {
        match pc {
            0x82628958 => {
    //   block [0x82628958..0x826289BC)
	// 82628958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262895C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628964: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262896C: 38AA361C  addi r5, r10, 0x361c
	ctx.r[5].s64 = ctx.r[10].s64 + 13852;
	// 82628970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628978: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8262897C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262898C: 386A34FC  addi r3, r10, 0x34fc
	ctx.r[3].s64 = ctx.r[10].s64 + 13564;
	// 82628990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628994: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628998: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8262899C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826289A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826289A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826289A8: 4BE3E479  bl 0x82466e20
	ctx.lr = 0x826289AC;
	sub_82466E20(ctx, base);
	// 826289AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826289B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826289B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826289B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826289C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826289C0 size=100
    let mut pc: u32 = 0x826289C0;
    'dispatch: loop {
        match pc {
            0x826289C0 => {
    //   block [0x826289C0..0x82628A24)
	// 826289C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826289C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826289C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826289CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826289D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826289D4: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 826289D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826289DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826289E0: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826289E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826289E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826289EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826289F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826289F4: 386A352C  addi r3, r10, 0x352c
	ctx.r[3].s64 = ctx.r[10].s64 + 13612;
	// 826289F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826289FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628A00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82628A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628A08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82628A0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628A10: 4BE3E411  bl 0x82466e20
	ctx.lr = 0x82628A14;
	sub_82466E20(ctx, base);
	// 82628A14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628A28 size=108
    let mut pc: u32 = 0x82628A28;
    'dispatch: loop {
        match pc {
            0x82628A28 => {
    //   block [0x82628A28..0x82628A94)
	// 82628A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628A34: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628A3C: 38EB7440  addi r7, r11, 0x7440
	ctx.r[7].s64 = ctx.r[11].s64 + 29760;
	// 82628A40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82628A44: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 82628A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628A50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628A58: 386A355C  addi r3, r10, 0x355c
	ctx.r[3].s64 = ctx.r[10].s64 + 13660;
	// 82628A5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628A7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628A80: 4BE3E3A1  bl 0x82466e20
	ctx.lr = 0x82628A84;
	sub_82466E20(ctx, base);
	// 82628A84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628A90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628A98 size=112
    let mut pc: u32 = 0x82628A98;
    'dispatch: loop {
        match pc {
            0x82628A98 => {
    //   block [0x82628A98..0x82628B08)
	// 82628A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628AA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628AA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628AA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628AAC: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 82628AB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628AB4: 390B7470  addi r8, r11, 0x7470
	ctx.r[8].s64 = ctx.r[11].s64 + 29808;
	// 82628AB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82628ABC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82628AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628AC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628AC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628AD0: 386A358C  addi r3, r10, 0x358c
	ctx.r[3].s64 = ctx.r[10].s64 + 13708;
	// 82628AD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628AD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628AE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628AE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628AEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628AF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628AF4: 4BE3E32D  bl 0x82466e20
	ctx.lr = 0x82628AF8;
	sub_82466E20(ctx, base);
	// 82628AF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628B04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628B08 size=108
    let mut pc: u32 = 0x82628B08;
    'dispatch: loop {
        match pc {
            0x82628B08 => {
    //   block [0x82628B08..0x82628B74)
	// 82628B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628B10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628B14: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628B1C: 38EB7488  addi r7, r11, 0x7488
	ctx.r[7].s64 = ctx.r[11].s64 + 29832;
	// 82628B20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82628B24: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82628B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628B2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628B30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628B34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628B38: 386A35BC  addi r3, r10, 0x35bc
	ctx.r[3].s64 = ctx.r[10].s64 + 13756;
	// 82628B3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628B5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628B60: 4BE3E2C1  bl 0x82466e20
	ctx.lr = 0x82628B64;
	sub_82466E20(ctx, base);
	// 82628B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82628B78 size=28
    let mut pc: u32 = 0x82628B78;
    'dispatch: loop {
        match pc {
            0x82628B78 => {
    //   block [0x82628B78..0x82628B94)
	// 82628B78: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628B7C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82628B80: 394AB2C0  addi r10, r10, -0x4d40
	ctx.r[10].s64 = ctx.r[10].s64 + -19776;
	// 82628B84: 816B74A0  lwz r11, 0x74a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29856 as u32) ) } as u64;
	// 82628B88: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82628B8C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82628B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628B98 size=108
    let mut pc: u32 = 0x82628B98;
    'dispatch: loop {
        match pc {
            0x82628B98 => {
    //   block [0x82628B98..0x82628C04)
	// 82628B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628BA4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82628BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628BAC: 38EBB2C0  addi r7, r11, -0x4d40
	ctx.r[7].s64 = ctx.r[11].s64 + -19776;
	// 82628BB0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82628BB4: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82628BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628BBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628BC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628BC8: 386A35EC  addi r3, r10, 0x35ec
	ctx.r[3].s64 = ctx.r[10].s64 + 13804;
	// 82628BCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628BF0: 4BE3E231  bl 0x82466e20
	ctx.lr = 0x82628BF4;
	sub_82466E20(ctx, base);
	// 82628BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628C08 size=116
    let mut pc: u32 = 0x82628C08;
    'dispatch: loop {
        match pc {
            0x82628C08 => {
    //   block [0x82628C08..0x82628C7C)
	// 82628C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628C14: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628C18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82628C1C: 390B74A8  addi r8, r11, 0x74a8
	ctx.r[8].s64 = ctx.r[11].s64 + 29864;
	// 82628C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628C24: 392A357C  addi r9, r10, 0x357c
	ctx.r[9].s64 = ctx.r[10].s64 + 13692;
	// 82628C28: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628C2C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82628C30: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 82628C34: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628C3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628C4C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82628C50: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 82628C54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82628C58: 386B361C  addi r3, r11, 0x361c
	ctx.r[3].s64 = ctx.r[11].s64 + 13852;
	// 82628C5C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82628C60: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628C68: 4BE3E1B9  bl 0x82466e20
	ctx.lr = 0x82628C6C;
	sub_82466E20(ctx, base);
	// 82628C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628C80 size=112
    let mut pc: u32 = 0x82628C80;
    'dispatch: loop {
        match pc {
            0x82628C80 => {
    //   block [0x82628C80..0x82628CF0)
	// 82628C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628C8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628C90: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628C94: 38AA322C  addi r5, r10, 0x322c
	ctx.r[5].s64 = ctx.r[10].s64 + 12844;
	// 82628C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628C9C: 390B7520  addi r8, r11, 0x7520
	ctx.r[8].s64 = ctx.r[11].s64 + 29984;
	// 82628CA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82628CA4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82628CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628CAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628CB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628CB8: 386A364C  addi r3, r10, 0x364c
	ctx.r[3].s64 = ctx.r[10].s64 + 13900;
	// 82628CBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628CDC: 4BE3E145  bl 0x82466e20
	ctx.lr = 0x82628CE0;
	sub_82466E20(ctx, base);
	// 82628CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628CF0 size=108
    let mut pc: u32 = 0x82628CF0;
    'dispatch: loop {
        match pc {
            0x82628CF0 => {
    //   block [0x82628CF0..0x82628D5C)
	// 82628CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628CFC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628D04: 38EB7538  addi r7, r11, 0x7538
	ctx.r[7].s64 = ctx.r[11].s64 + 30008;
	// 82628D08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82628D0C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82628D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628D14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628D18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628D1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628D20: 386A367C  addi r3, r10, 0x367c
	ctx.r[3].s64 = ctx.r[10].s64 + 13948;
	// 82628D24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628D44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628D48: 4BE3E0D9  bl 0x82466e20
	ctx.lr = 0x82628D4C;
	sub_82466E20(ctx, base);
	// 82628D4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628D60 size=112
    let mut pc: u32 = 0x82628D60;
    'dispatch: loop {
        match pc {
            0x82628D60 => {
    //   block [0x82628D60..0x82628DD0)
	// 82628D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628D68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628D6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628D70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628D74: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 82628D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628D7C: 390B7568  addi r8, r11, 0x7568
	ctx.r[8].s64 = ctx.r[11].s64 + 30056;
	// 82628D80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82628D84: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82628D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628D8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628D90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628D98: 386A36AC  addi r3, r10, 0x36ac
	ctx.r[3].s64 = ctx.r[10].s64 + 13996;
	// 82628D9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628DAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628DBC: 4BE3E065  bl 0x82466e20
	ctx.lr = 0x82628DC0;
	sub_82466E20(ctx, base);
	// 82628DC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628DD0 size=112
    let mut pc: u32 = 0x82628DD0;
    'dispatch: loop {
        match pc {
            0x82628DD0 => {
    //   block [0x82628DD0..0x82628E40)
	// 82628DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628DDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628DE0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628DE4: 38AA382C  addi r5, r10, 0x382c
	ctx.r[5].s64 = ctx.r[10].s64 + 14380;
	// 82628DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628DEC: 390B7598  addi r8, r11, 0x7598
	ctx.r[8].s64 = ctx.r[11].s64 + 30104;
	// 82628DF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82628DF4: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82628DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628DFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628E00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628E08: 386A36DC  addi r3, r10, 0x36dc
	ctx.r[3].s64 = ctx.r[10].s64 + 14044;
	// 82628E0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628E1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628E2C: 4BE3DFF5  bl 0x82466e20
	ctx.lr = 0x82628E30;
	sub_82466E20(ctx, base);
	// 82628E30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628E3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628E40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628E40 size=100
    let mut pc: u32 = 0x82628E40;
    'dispatch: loop {
        match pc {
            0x82628E40 => {
    //   block [0x82628E40..0x82628EA4)
	// 82628E40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628E44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628E48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628E4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628E54: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 82628E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628E60: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 82628E64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628E74: 386A370C  addi r3, r10, 0x370c
	ctx.r[3].s64 = ctx.r[10].s64 + 14092;
	// 82628E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628E7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628E80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82628E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628E88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82628E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628E90: 4BE3DF91  bl 0x82466e20
	ctx.lr = 0x82628E94;
	sub_82466E20(ctx, base);
	// 82628E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628EA8 size=112
    let mut pc: u32 = 0x82628EA8;
    'dispatch: loop {
        match pc {
            0x82628EA8 => {
    //   block [0x82628EA8..0x82628F18)
	// 82628EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628EB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628EB8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628EBC: 38AA352C  addi r5, r10, 0x352c
	ctx.r[5].s64 = ctx.r[10].s64 + 13612;
	// 82628EC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628EC4: 390B75C8  addi r8, r11, 0x75c8
	ctx.r[8].s64 = ctx.r[11].s64 + 30152;
	// 82628EC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82628ECC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82628ED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628ED4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628ED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628EE0: 386A373C  addi r3, r10, 0x373c
	ctx.r[3].s64 = ctx.r[10].s64 + 14140;
	// 82628EE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628EE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628EF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628EF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628EFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628F00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628F04: 4BE3DF1D  bl 0x82466e20
	ctx.lr = 0x82628F08;
	sub_82466E20(ctx, base);
	// 82628F08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628F18 size=112
    let mut pc: u32 = 0x82628F18;
    'dispatch: loop {
        match pc {
            0x82628F18 => {
    //   block [0x82628F18..0x82628F88)
	// 82628F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628F24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628F28: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628F2C: 38AA352C  addi r5, r10, 0x352c
	ctx.r[5].s64 = ctx.r[10].s64 + 13612;
	// 82628F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628F34: 390B7610  addi r8, r11, 0x7610
	ctx.r[8].s64 = ctx.r[11].s64 + 30224;
	// 82628F38: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82628F3C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82628F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628F44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82628F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628F50: 386A376C  addi r3, r10, 0x376c
	ctx.r[3].s64 = ctx.r[10].s64 + 14188;
	// 82628F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82628F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628F74: 4BE3DEAD  bl 0x82466e20
	ctx.lr = 0x82628F78;
	sub_82466E20(ctx, base);
	// 82628F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628F88 size=108
    let mut pc: u32 = 0x82628F88;
    'dispatch: loop {
        match pc {
            0x82628F88 => {
    //   block [0x82628F88..0x82628FF4)
	// 82628F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82628F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82628F94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82628F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82628F9C: 38EB76B8  addi r7, r11, 0x76b8
	ctx.r[7].s64 = ctx.r[11].s64 + 30392;
	// 82628FA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82628FA4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 82628FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82628FAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82628FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82628FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82628FB8: 386A379C  addi r3, r10, 0x379c
	ctx.r[3].s64 = ctx.r[10].s64 + 14236;
	// 82628FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82628FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82628FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82628FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82628FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82628FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82628FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82628FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82628FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82628FE0: 4BE3DE41  bl 0x82466e20
	ctx.lr = 0x82628FE4;
	sub_82466E20(ctx, base);
	// 82628FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82628FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82628FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82628FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82628FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82628FF8 size=112
    let mut pc: u32 = 0x82628FF8;
    'dispatch: loop {
        match pc {
            0x82628FF8 => {
    //   block [0x82628FF8..0x82629068)
	// 82628FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82628FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629004: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629008: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262900C: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 82629010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629014: 390B7700  addi r8, r11, 0x7700
	ctx.r[8].s64 = ctx.r[11].s64 + 30464;
	// 82629018: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8262901C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82629020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629024: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262902C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629030: 386A37CC  addi r3, r10, 0x37cc
	ctx.r[3].s64 = ctx.r[10].s64 + 14284;
	// 82629034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262903C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262904C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629054: 4BE3DDCD  bl 0x82466e20
	ctx.lr = 0x82629058;
	sub_82466E20(ctx, base);
	// 82629058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262905C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629068 size=100
    let mut pc: u32 = 0x82629068;
    'dispatch: loop {
        match pc {
            0x82629068 => {
    //   block [0x82629068..0x826290CC)
	// 82629068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629074: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262907C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82629080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629088: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8262908C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262909C: 386A37FC  addi r3, r10, 0x37fc
	ctx.r[3].s64 = ctx.r[10].s64 + 14332;
	// 826290A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826290A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826290A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826290AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826290B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826290B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826290B8: 4BE3DD69  bl 0x82466e20
	ctx.lr = 0x826290BC;
	sub_82466E20(ctx, base);
	// 826290BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826290C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826290C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826290C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826290D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826290D0 size=100
    let mut pc: u32 = 0x826290D0;
    'dispatch: loop {
        match pc {
            0x826290D0 => {
    //   block [0x826290D0..0x82629134)
	// 826290D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826290D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826290D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826290DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826290E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826290E4: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 826290E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826290EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826290F0: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826290F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826290F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826290FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629104: 386A382C  addi r3, r10, 0x382c
	ctx.r[3].s64 = ctx.r[10].s64 + 14380;
	// 82629108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262910C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629110: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82629114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629118: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8262911C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629120: 4BE3DD01  bl 0x82466e20
	ctx.lr = 0x82629124;
	sub_82466E20(ctx, base);
	// 82629124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8262912C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629138 size=112
    let mut pc: u32 = 0x82629138;
    'dispatch: loop {
        match pc {
            0x82629138 => {
    //   block [0x82629138..0x826291A8)
	// 82629138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262913C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629144: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629148: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262914C: 38AA23EC  addi r5, r10, 0x23ec
	ctx.r[5].s64 = ctx.r[10].s64 + 9196;
	// 82629150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629154: 390B7760  addi r8, r11, 0x7760
	ctx.r[8].s64 = ctx.r[11].s64 + 30560;
	// 82629158: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8262915C: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82629160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262916C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629170: 386A385C  addi r3, r10, 0x385c
	ctx.r[3].s64 = ctx.r[10].s64 + 14428;
	// 82629174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262917C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262918C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629194: 4BE3DC8D  bl 0x82466e20
	ctx.lr = 0x82629198;
	sub_82466E20(ctx, base);
	// 82629198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262919C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826291A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826291A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826291A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826291A8 size=112
    let mut pc: u32 = 0x826291A8;
    'dispatch: loop {
        match pc {
            0x826291A8 => {
    //   block [0x826291A8..0x82629218)
	// 826291A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826291AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826291B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826291B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826291B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826291BC: 38AA361C  addi r5, r10, 0x361c
	ctx.r[5].s64 = ctx.r[10].s64 + 13852;
	// 826291C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826291C4: 390B77F0  addi r8, r11, 0x77f0
	ctx.r[8].s64 = ctx.r[11].s64 + 30704;
	// 826291C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826291CC: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826291D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826291D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826291D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826291DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826291E0: 386A388C  addi r3, r10, 0x388c
	ctx.r[3].s64 = ctx.r[10].s64 + 14476;
	// 826291E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826291E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826291EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826291F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826291F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826291F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826291FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629204: 4BE3DC1D  bl 0x82466e20
	ctx.lr = 0x82629208;
	sub_82466E20(ctx, base);
	// 82629208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262920C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629218 size=112
    let mut pc: u32 = 0x82629218;
    'dispatch: loop {
        match pc {
            0x82629218 => {
    //   block [0x82629218..0x82629288)
	// 82629218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262921C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629228: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262922C: 38AA376C  addi r5, r10, 0x376c
	ctx.r[5].s64 = ctx.r[10].s64 + 14188;
	// 82629230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629234: 390B7808  addi r8, r11, 0x7808
	ctx.r[8].s64 = ctx.r[11].s64 + 30728;
	// 82629238: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8262923C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 82629240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629244: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262924C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629250: 386A38BC  addi r3, r10, 0x38bc
	ctx.r[3].s64 = ctx.r[10].s64 + 14524;
	// 82629254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262925C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262926C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629274: 4BE3DBAD  bl 0x82466e20
	ctx.lr = 0x82629278;
	sub_82466E20(ctx, base);
	// 82629278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262927C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629288 size=112
    let mut pc: u32 = 0x82629288;
    'dispatch: loop {
        match pc {
            0x82629288 => {
    //   block [0x82629288..0x826292F8)
	// 82629288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262928C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629298: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262929C: 38AA310C  addi r5, r10, 0x310c
	ctx.r[5].s64 = ctx.r[10].s64 + 12556;
	// 826292A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826292A4: 390B7838  addi r8, r11, 0x7838
	ctx.r[8].s64 = ctx.r[11].s64 + 30776;
	// 826292A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826292AC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826292B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826292B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826292B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826292BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826292C0: 386A38EC  addi r3, r10, 0x38ec
	ctx.r[3].s64 = ctx.r[10].s64 + 14572;
	// 826292C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826292C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826292CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826292D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826292D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826292D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826292DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826292E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826292E4: 4BE3DB3D  bl 0x82466e20
	ctx.lr = 0x826292E8;
	sub_82466E20(ctx, base);
	// 826292E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826292EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826292F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826292F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826292F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826292F8 size=112
    let mut pc: u32 = 0x826292F8;
    'dispatch: loop {
        match pc {
            0x826292F8 => {
    //   block [0x826292F8..0x82629368)
	// 826292F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826292FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629304: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629308: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262930C: 38AA32BC  addi r5, r10, 0x32bc
	ctx.r[5].s64 = ctx.r[10].s64 + 12988;
	// 82629310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629314: 390B7880  addi r8, r11, 0x7880
	ctx.r[8].s64 = ctx.r[11].s64 + 30848;
	// 82629318: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8262931C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82629320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629324: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262932C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629330: 386A391C  addi r3, r10, 0x391c
	ctx.r[3].s64 = ctx.r[10].s64 + 14620;
	// 82629334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262933C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262934C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629354: 4BE3DACD  bl 0x82466e20
	ctx.lr = 0x82629358;
	sub_82466E20(ctx, base);
	// 82629358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262935C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629368 size=112
    let mut pc: u32 = 0x82629368;
    'dispatch: loop {
        match pc {
            0x82629368 => {
    //   block [0x82629368..0x826293D8)
	// 82629368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8262936C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82629374: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629378: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262937C: 38AA322C  addi r5, r10, 0x322c
	ctx.r[5].s64 = ctx.r[10].s64 + 12844;
	// 82629380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629384: 390B78C8  addi r8, r11, 0x78c8
	ctx.r[8].s64 = ctx.r[11].s64 + 30920;
	// 82629388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8262938C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82629390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629394: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262939C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826293A0: 386A394C  addi r3, r10, 0x394c
	ctx.r[3].s64 = ctx.r[10].s64 + 14668;
	// 826293A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826293A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826293AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826293B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826293B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826293B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826293BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826293C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826293C4: 4BE3DA5D  bl 0x82466e20
	ctx.lr = 0x826293C8;
	sub_82466E20(ctx, base);
	// 826293C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826293CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826293D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826293D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826293D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826293D8 size=112
    let mut pc: u32 = 0x826293D8;
    'dispatch: loop {
        match pc {
            0x826293D8 => {
    //   block [0x826293D8..0x82629448)
	// 826293D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826293DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826293E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826293E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826293E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826293EC: 38AA328C  addi r5, r10, 0x328c
	ctx.r[5].s64 = ctx.r[10].s64 + 12940;
	// 826293F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826293F4: 390B78E0  addi r8, r11, 0x78e0
	ctx.r[8].s64 = ctx.r[11].s64 + 30944;
	// 826293F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826293FC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 82629400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629404: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8262940C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629410: 386A397C  addi r3, r10, 0x397c
	ctx.r[3].s64 = ctx.r[10].s64 + 14716;
	// 82629414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262941C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262942C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629434: 4BE3D9ED  bl 0x82466e20
	ctx.lr = 0x82629438;
	sub_82466E20(ctx, base);
	// 82629438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8262943C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82629444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82629448 size=24
    let mut pc: u32 = 0x82629448;
    'dispatch: loop {
        match pc {
            0x82629448 => {
    //   block [0x82629448..0x82629460)
	// 82629448: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8262944C: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 82629450: 394AB3F8  addi r10, r10, -0x4c08
	ctx.r[10].s64 = ctx.r[10].s64 + -19464;
	// 82629454: 816B7910  lwz r11, 0x7910(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30992 as u32) ) } as u64;
	// 82629458: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8262945C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629460 size=112
    let mut pc: u32 = 0x82629460;
    'dispatch: loop {
        match pc {
            0x82629460 => {
    //   block [0x82629460..0x826294D0)
	// 82629460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262946C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82629470: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82629474: 392A36A0  addi r9, r10, 0x36a0
	ctx.r[9].s64 = ctx.r[10].s64 + 13984;
	// 82629478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8262947C: 390BB3F8  addi r8, r11, -0x4c08
	ctx.r[8].s64 = ctx.r[11].s64 + -19464;
	// 82629480: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82629484: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 82629488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8262948C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629498: 386A39AC  addi r3, r10, 0x39ac
	ctx.r[3].s64 = ctx.r[10].s64 + 14764;
	// 8262949C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826294A0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826294A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826294A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826294AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826294B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826294B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826294B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826294BC: 4BE3D965  bl 0x82466e20
	ctx.lr = 0x826294C0;
	sub_82466E20(ctx, base);
	// 826294C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826294C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826294C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826294CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826294D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826294D0 size=112
    let mut pc: u32 = 0x826294D0;
    'dispatch: loop {
        match pc {
            0x826294D0 => {
    //   block [0x826294D0..0x82629540)
	// 826294D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826294D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826294D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826294DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826294E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826294E4: 38AA39AC  addi r5, r10, 0x39ac
	ctx.r[5].s64 = ctx.r[10].s64 + 14764;
	// 826294E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826294EC: 390B7918  addi r8, r11, 0x7918
	ctx.r[8].s64 = ctx.r[11].s64 + 31000;
	// 826294F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826294F4: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826294F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826294FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82629504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629508: 386A39DC  addi r3, r10, 0x39dc
	ctx.r[3].s64 = ctx.r[10].s64 + 14812;
	// 8262950C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82629510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82629514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8262951C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82629524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8262952C: 4BE3D8F5  bl 0x82466e20
	ctx.lr = 0x82629530;
	sub_82466E20(ctx, base);
	// 82629530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82629534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82629538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8262953C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82629540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82629540 size=108
    let mut pc: u32 = 0x82629540;
    'dispatch: loop {
        match pc {
            0x82629540 => {
    //   block [0x82629540..0x826295AC)
	// 82629540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82629544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82629548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8262954C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82629550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82629554: 38EB7948  addi r7, r11, 0x7948
	ctx.r[7].s64 = ctx.r[11].s64 + 31048;
	// 82629558: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8262955C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 82629560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82629564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82629568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8262956C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82629570: 386A3A0C  addi r3, r10, 0x3a0c
	ctx.r[3].s64 = ctx.r[10].s64 + 14860;
	// 82629574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82629578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8262957C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82629580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82629584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82629588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8262958C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82629590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82629594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82629598: 4BE3D889  bl 0x82466e20
	ctx.lr = 0x8262959C;
	sub_82466E20(ctx, base);
	// 8262959C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826295A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826295A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826295A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


