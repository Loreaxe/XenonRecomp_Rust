pub fn sub_826F0FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F0FD8 size=108
    let mut pc: u32 = 0x826F0FD8;
    'dispatch: loop {
        match pc {
            0x826F0FD8 => {
    //   block [0x826F0FD8..0x826F1044)
	// 826F0FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F0FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F0FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F0FE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F0FE8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826F0FEC: 38EBB5A8  addi r7, r11, -0x4a58
	ctx.r[7].s64 = ctx.r[11].s64 + -19032;
	// 826F0FF0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F0FF4: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826F0FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F0FFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1008: 386A7BF4  addi r3, r10, 0x7bf4
	ctx.r[3].s64 = ctx.r[10].s64 + 31732;
	// 826F100C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F101C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F102C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1030: 4BD75DF1  bl 0x82466e20
	ctx.lr = 0x826F1034;
	sub_82466E20(ctx, base);
	// 826F1034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F103C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1048 size=108
    let mut pc: u32 = 0x826F1048;
    'dispatch: loop {
        match pc {
            0x826F1048 => {
    //   block [0x826F1048..0x826F10B4)
	// 826F1048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F104C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1054: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F105C: 38EBB5C0  addi r7, r11, -0x4a40
	ctx.r[7].s64 = ctx.r[11].s64 + -19008;
	// 826F1060: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F1064: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826F1068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F106C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1078: 386A7C24  addi r3, r10, 0x7c24
	ctx.r[3].s64 = ctx.r[10].s64 + 31780;
	// 826F107C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F108C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F109C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F10A0: 4BD75D81  bl 0x82466e20
	ctx.lr = 0x826F10A4;
	sub_82466E20(ctx, base);
	// 826F10A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F10A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F10AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F10B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F10B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F10B8 size=24
    let mut pc: u32 = 0x826F10B8;
    'dispatch: loop {
        match pc {
            0x826F10B8 => {
    //   block [0x826F10B8..0x826F10D0)
	// 826F10B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F10BC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F10C0: 394A1928  addi r10, r10, 0x1928
	ctx.r[10].s64 = ctx.r[10].s64 + 6440;
	// 826F10C4: 816BB650  lwz r11, -0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18864 as u32) ) } as u64;
	// 826F10C8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F10CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F10D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F10D0 size=108
    let mut pc: u32 = 0x826F10D0;
    'dispatch: loop {
        match pc {
            0x826F10D0 => {
    //   block [0x826F10D0..0x826F113C)
	// 826F10D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F10D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F10D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F10DC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F10E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F10E4: 38EB1928  addi r7, r11, 0x1928
	ctx.r[7].s64 = ctx.r[11].s64 + 6440;
	// 826F10E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F10EC: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826F10F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F10F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F10F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F10FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1100: 386A7C54  addi r3, r10, 0x7c54
	ctx.r[3].s64 = ctx.r[10].s64 + 31828;
	// 826F1104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F110C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F111C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1128: 4BD75CF9  bl 0x82466e20
	ctx.lr = 0x826F112C;
	sub_82466E20(ctx, base);
	// 826F112C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F1140 size=24
    let mut pc: u32 = 0x826F1140;
    'dispatch: loop {
        match pc {
            0x826F1140 => {
    //   block [0x826F1140..0x826F1158)
	// 826F1140: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1144: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F1148: 394A1958  addi r10, r10, 0x1958
	ctx.r[10].s64 = ctx.r[10].s64 + 6488;
	// 826F114C: 816BB650  lwz r11, -0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18864 as u32) ) } as u64;
	// 826F1150: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F1154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1158 size=108
    let mut pc: u32 = 0x826F1158;
    'dispatch: loop {
        match pc {
            0x826F1158 => {
    //   block [0x826F1158..0x826F11C4)
	// 826F1158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F115C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1164: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F116C: 38EB1958  addi r7, r11, 0x1958
	ctx.r[7].s64 = ctx.r[11].s64 + 6488;
	// 826F1170: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F1174: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826F1178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F117C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1188: 386A7C84  addi r3, r10, 0x7c84
	ctx.r[3].s64 = ctx.r[10].s64 + 31876;
	// 826F118C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F119C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F11A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F11A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F11A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F11AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F11B0: 4BD75C71  bl 0x82466e20
	ctx.lr = 0x826F11B4;
	sub_82466E20(ctx, base);
	// 826F11B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F11B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F11BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F11C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F11C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F11C8 size=108
    let mut pc: u32 = 0x826F11C8;
    'dispatch: loop {
        match pc {
            0x826F11C8 => {
    //   block [0x826F11C8..0x826F1234)
	// 826F11C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F11CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F11D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F11D4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F11D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F11DC: 38EBB638  addi r7, r11, -0x49c8
	ctx.r[7].s64 = ctx.r[11].s64 + -18888;
	// 826F11E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F11E4: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826F11E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F11EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F11F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F11F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F11F8: 386A7CB4  addi r3, r10, 0x7cb4
	ctx.r[3].s64 = ctx.r[10].s64 + 31924;
	// 826F11FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F120C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F121C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1220: 4BD75C01  bl 0x82466e20
	ctx.lr = 0x826F1224;
	sub_82466E20(ctx, base);
	// 826F1224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F122C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F1238 size=24
    let mut pc: u32 = 0x826F1238;
    'dispatch: loop {
        match pc {
            0x826F1238 => {
    //   block [0x826F1238..0x826F1250)
	// 826F1238: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F123C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F1240: 394A1988  addi r10, r10, 0x1988
	ctx.r[10].s64 = ctx.r[10].s64 + 6536;
	// 826F1244: 816BB650  lwz r11, -0x49b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18864 as u32) ) } as u64;
	// 826F1248: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F124C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1250 size=108
    let mut pc: u32 = 0x826F1250;
    'dispatch: loop {
        match pc {
            0x826F1250 => {
    //   block [0x826F1250..0x826F12BC)
	// 826F1250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F125C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1264: 38EB1988  addi r7, r11, 0x1988
	ctx.r[7].s64 = ctx.r[11].s64 + 6536;
	// 826F1268: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F126C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826F1270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1274: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F127C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1280: 386A7CE4  addi r3, r10, 0x7ce4
	ctx.r[3].s64 = ctx.r[10].s64 + 31972;
	// 826F1284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F128C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F129C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F12A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F12A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F12A8: 4BD75B79  bl 0x82466e20
	ctx.lr = 0x826F12AC;
	sub_82466E20(ctx, base);
	// 826F12AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F12B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F12B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F12B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F12C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F12C0 size=112
    let mut pc: u32 = 0x826F12C0;
    'dispatch: loop {
        match pc {
            0x826F12C0 => {
    //   block [0x826F12C0..0x826F1330)
	// 826F12C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F12C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F12C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F12CC: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F12D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F12D4: 392A9CCC  addi r9, r10, -0x6334
	ctx.r[9].s64 = ctx.r[10].s64 + -25396;
	// 826F12D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F12DC: 390BB654  addi r8, r11, -0x49ac
	ctx.r[8].s64 = ctx.r[11].s64 + -18860;
	// 826F12E0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826F12E4: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826F12E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F12EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F12F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F12F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F12F8: 386A7D14  addi r3, r10, 0x7d14
	ctx.r[3].s64 = ctx.r[10].s64 + 32020;
	// 826F12FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F1300: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F1304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F130C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F131C: 4BD75B05  bl 0x82466e20
	ctx.lr = 0x826F1320;
	sub_82466E20(ctx, base);
	// 826F1320: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F132C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1330 size=108
    let mut pc: u32 = 0x826F1330;
    'dispatch: loop {
        match pc {
            0x826F1330 => {
    //   block [0x826F1330..0x826F139C)
	// 826F1330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F133C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1344: 38EBB684  addi r7, r11, -0x497c
	ctx.r[7].s64 = ctx.r[11].s64 + -18812;
	// 826F1348: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F134C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826F1350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1358: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F135C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1360: 386A7D44  addi r3, r10, 0x7d44
	ctx.r[3].s64 = ctx.r[10].s64 + 32068;
	// 826F1364: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F136C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F137C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1388: 4BD75A99  bl 0x82466e20
	ctx.lr = 0x826F138C;
	sub_82466E20(ctx, base);
	// 826F138C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F13A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F13A0 size=108
    let mut pc: u32 = 0x826F13A0;
    'dispatch: loop {
        match pc {
            0x826F13A0 => {
    //   block [0x826F13A0..0x826F140C)
	// 826F13A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F13A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F13A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F13AC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F13B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F13B4: 38EBB6B4  addi r7, r11, -0x494c
	ctx.r[7].s64 = ctx.r[11].s64 + -18764;
	// 826F13B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F13BC: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826F13C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F13C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F13C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F13CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F13D0: 386A7D74  addi r3, r10, 0x7d74
	ctx.r[3].s64 = ctx.r[10].s64 + 32116;
	// 826F13D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F13D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F13DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F13E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F13E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F13E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F13EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F13F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F13F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F13F8: 4BD75A29  bl 0x82466e20
	ctx.lr = 0x826F13FC;
	sub_82466E20(ctx, base);
	// 826F13FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1410 size=108
    let mut pc: u32 = 0x826F1410;
    'dispatch: loop {
        match pc {
            0x826F1410 => {
    //   block [0x826F1410..0x826F147C)
	// 826F1410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F141C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1424: 38EBB6CC  addi r7, r11, -0x4934
	ctx.r[7].s64 = ctx.r[11].s64 + -18740;
	// 826F1428: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F142C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826F1430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1434: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1438: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F143C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1440: 386A7DA4  addi r3, r10, 0x7da4
	ctx.r[3].s64 = ctx.r[10].s64 + 32164;
	// 826F1444: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F144C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F145C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1468: 4BD759B9  bl 0x82466e20
	ctx.lr = 0x826F146C;
	sub_82466E20(ctx, base);
	// 826F146C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1480 size=112
    let mut pc: u32 = 0x826F1480;
    'dispatch: loop {
        match pc {
            0x826F1480 => {
    //   block [0x826F1480..0x826F14F0)
	// 826F1480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F148C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1490: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1494: 38AA7E04  addi r5, r10, 0x7e04
	ctx.r[5].s64 = ctx.r[10].s64 + 32260;
	// 826F1498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F149C: 390BB6FC  addi r8, r11, -0x4904
	ctx.r[8].s64 = ctx.r[11].s64 + -18692;
	// 826F14A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F14A4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826F14A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F14AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F14B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F14B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F14B8: 386A7DD4  addi r3, r10, 0x7dd4
	ctx.r[3].s64 = ctx.r[10].s64 + 32212;
	// 826F14BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F14C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F14C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F14C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F14CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F14D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F14D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F14D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F14DC: 4BD75945  bl 0x82466e20
	ctx.lr = 0x826F14E0;
	sub_82466E20(ctx, base);
	// 826F14E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F14E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F14E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F14EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F14F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F14F0 size=108
    let mut pc: u32 = 0x826F14F0;
    'dispatch: loop {
        match pc {
            0x826F14F0 => {
    //   block [0x826F14F0..0x826F155C)
	// 826F14F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F14F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F14F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F14FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1504: 38EBB714  addi r7, r11, -0x48ec
	ctx.r[7].s64 = ctx.r[11].s64 + -18668;
	// 826F1508: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F150C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826F1510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1514: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F151C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1520: 386A7E04  addi r3, r10, 0x7e04
	ctx.r[3].s64 = ctx.r[10].s64 + 32260;
	// 826F1524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F152C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F153C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1548: 4BD758D9  bl 0x82466e20
	ctx.lr = 0x826F154C;
	sub_82466E20(ctx, base);
	// 826F154C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1560 size=108
    let mut pc: u32 = 0x826F1560;
    'dispatch: loop {
        match pc {
            0x826F1560 => {
    //   block [0x826F1560..0x826F15CC)
	// 826F1560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F156C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1574: 38EBB744  addi r7, r11, -0x48bc
	ctx.r[7].s64 = ctx.r[11].s64 + -18620;
	// 826F1578: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F157C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826F1580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1584: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F158C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1590: 386A7E34  addi r3, r10, 0x7e34
	ctx.r[3].s64 = ctx.r[10].s64 + 32308;
	// 826F1594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F159C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F15A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F15A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F15A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F15AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F15B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F15B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F15B8: 4BD75869  bl 0x82466e20
	ctx.lr = 0x826F15BC;
	sub_82466E20(ctx, base);
	// 826F15BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F15C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F15C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F15C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F15D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F15D0 size=108
    let mut pc: u32 = 0x826F15D0;
    'dispatch: loop {
        match pc {
            0x826F15D0 => {
    //   block [0x826F15D0..0x826F163C)
	// 826F15D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F15D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F15D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F15DC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F15E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F15E4: 38EBB75C  addi r7, r11, -0x48a4
	ctx.r[7].s64 = ctx.r[11].s64 + -18596;
	// 826F15E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F15EC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826F15F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F15F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F15F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F15FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1600: 386A7E64  addi r3, r10, 0x7e64
	ctx.r[3].s64 = ctx.r[10].s64 + 32356;
	// 826F1604: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F160C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F161C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1624: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1628: 4BD757F9  bl 0x82466e20
	ctx.lr = 0x826F162C;
	sub_82466E20(ctx, base);
	// 826F162C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1640 size=108
    let mut pc: u32 = 0x826F1640;
    'dispatch: loop {
        match pc {
            0x826F1640 => {
    //   block [0x826F1640..0x826F16AC)
	// 826F1640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F164C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1654: 38EBB790  addi r7, r11, -0x4870
	ctx.r[7].s64 = ctx.r[11].s64 + -18544;
	// 826F1658: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826F165C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826F1660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1668: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F166C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1670: 386A7E94  addi r3, r10, 0x7e94
	ctx.r[3].s64 = ctx.r[10].s64 + 32404;
	// 826F1674: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F167C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F168C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1694: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1698: 4BD75789  bl 0x82466e20
	ctx.lr = 0x826F169C;
	sub_82466E20(ctx, base);
	// 826F169C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F16A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F16A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F16A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F16B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F16B0 size=108
    let mut pc: u32 = 0x826F16B0;
    'dispatch: loop {
        match pc {
            0x826F16B0 => {
    //   block [0x826F16B0..0x826F171C)
	// 826F16B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F16B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F16B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F16BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F16C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F16C4: 38EBB838  addi r7, r11, -0x47c8
	ctx.r[7].s64 = ctx.r[11].s64 + -18376;
	// 826F16C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F16CC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826F16D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F16D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F16D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F16DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F16E0: 386A7EC4  addi r3, r10, 0x7ec4
	ctx.r[3].s64 = ctx.r[10].s64 + 32452;
	// 826F16E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F16E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F16EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F16F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F16F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F16F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F16FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1708: 4BD75719  bl 0x82466e20
	ctx.lr = 0x826F170C;
	sub_82466E20(ctx, base);
	// 826F170C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1720 size=108
    let mut pc: u32 = 0x826F1720;
    'dispatch: loop {
        match pc {
            0x826F1720 => {
    //   block [0x826F1720..0x826F178C)
	// 826F1720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F172C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1734: 38EBB868  addi r7, r11, -0x4798
	ctx.r[7].s64 = ctx.r[11].s64 + -18328;
	// 826F1738: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F173C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826F1740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F174C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1750: 386A7EF4  addi r3, r10, 0x7ef4
	ctx.r[3].s64 = ctx.r[10].s64 + 32500;
	// 826F1754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F175C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F176C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1778: 4BD756A9  bl 0x82466e20
	ctx.lr = 0x826F177C;
	sub_82466E20(ctx, base);
	// 826F177C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1790 size=108
    let mut pc: u32 = 0x826F1790;
    'dispatch: loop {
        match pc {
            0x826F1790 => {
    //   block [0x826F1790..0x826F17FC)
	// 826F1790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F179C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F17A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F17A4: 38EBB880  addi r7, r11, -0x4780
	ctx.r[7].s64 = ctx.r[11].s64 + -18304;
	// 826F17A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F17AC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826F17B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F17B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F17B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F17BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F17C0: 386A7F24  addi r3, r10, 0x7f24
	ctx.r[3].s64 = ctx.r[10].s64 + 32548;
	// 826F17C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F17C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F17CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F17D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F17D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F17D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F17DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F17E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F17E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F17E8: 4BD75639  bl 0x82466e20
	ctx.lr = 0x826F17EC;
	sub_82466E20(ctx, base);
	// 826F17EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F17F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F17F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F17F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1800 size=112
    let mut pc: u32 = 0x826F1800;
    'dispatch: loop {
        match pc {
            0x826F1800 => {
    //   block [0x826F1800..0x826F1870)
	// 826F1800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F180C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1810: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1814: 38AA7D74  addi r5, r10, 0x7d74
	ctx.r[5].s64 = ctx.r[10].s64 + 32116;
	// 826F1818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F181C: 390BB8B0  addi r8, r11, -0x4750
	ctx.r[8].s64 = ctx.r[11].s64 + -18256;
	// 826F1820: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826F1824: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826F1828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F182C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1830: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F1834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1838: 386A7F54  addi r3, r10, 0x7f54
	ctx.r[3].s64 = ctx.r[10].s64 + 32596;
	// 826F183C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F1840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F184C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F185C: 4BD755C5  bl 0x82466e20
	ctx.lr = 0x826F1860;
	sub_82466E20(ctx, base);
	// 826F1860: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F186C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F1870 size=24
    let mut pc: u32 = 0x826F1870;
    'dispatch: loop {
        match pc {
            0x826F1870 => {
    //   block [0x826F1870..0x826F1888)
	// 826F1870: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1874: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F1878: 394A19B8  addi r10, r10, 0x19b8
	ctx.r[10].s64 = ctx.r[10].s64 + 6584;
	// 826F187C: 816BB78C  lwz r11, -0x4874(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18548 as u32) ) } as u64;
	// 826F1880: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F1884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1888 size=112
    let mut pc: u32 = 0x826F1888;
    'dispatch: loop {
        match pc {
            0x826F1888 => {
    //   block [0x826F1888..0x826F18F8)
	// 826F1888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F188C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1894: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F1898: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F189C: 392A9CF8  addi r9, r10, -0x6308
	ctx.r[9].s64 = ctx.r[10].s64 + -25352;
	// 826F18A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F18A4: 390B19B8  addi r8, r11, 0x19b8
	ctx.r[8].s64 = ctx.r[11].s64 + 6584;
	// 826F18A8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F18AC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826F18B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F18B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F18B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F18BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F18C0: 386A7F84  addi r3, r10, 0x7f84
	ctx.r[3].s64 = ctx.r[10].s64 + 32644;
	// 826F18C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F18C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F18CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F18D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F18D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F18D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F18DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F18E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F18E4: 4BD7553D  bl 0x82466e20
	ctx.lr = 0x826F18E8;
	sub_82466E20(ctx, base);
	// 826F18E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F18EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F18F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F18F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F18F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F18F8 size=108
    let mut pc: u32 = 0x826F18F8;
    'dispatch: loop {
        match pc {
            0x826F18F8 => {
    //   block [0x826F18F8..0x826F1964)
	// 826F18F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F18FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1904: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F190C: 38EBB95C  addi r7, r11, -0x46a4
	ctx.r[7].s64 = ctx.r[11].s64 + -18084;
	// 826F1910: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F1914: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826F1918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F191C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1928: 386A7FB4  addi r3, r10, 0x7fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 32692;
	// 826F192C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F193C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F194C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1950: 4BD754D1  bl 0x82466e20
	ctx.lr = 0x826F1954;
	sub_82466E20(ctx, base);
	// 826F1954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F195C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1968 size=116
    let mut pc: u32 = 0x826F1968;
    'dispatch: loop {
        match pc {
            0x826F1968 => {
    //   block [0x826F1968..0x826F19DC)
	// 826F1968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F196C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1974: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1978: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F197C: 390BB990  addi r8, r11, -0x4670
	ctx.r[8].s64 = ctx.r[11].s64 + -18032;
	// 826F1980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1984: 392A9D3C  addi r9, r10, -0x62c4
	ctx.r[9].s64 = ctx.r[10].s64 + -25284;
	// 826F1988: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F198C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826F1990: 38AA7D74  addi r5, r10, 0x7d74
	ctx.r[5].s64 = ctx.r[10].s64 + 32116;
	// 826F1994: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F1998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F199C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F19A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F19A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F19A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F19AC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826F19B0: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826F19B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F19B8: 386B7FE4  addi r3, r11, 0x7fe4
	ctx.r[3].s64 = ctx.r[11].s64 + 32740;
	// 826F19BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F19C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F19C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F19C8: 4BD75459  bl 0x82466e20
	ctx.lr = 0x826F19CC;
	sub_82466E20(ctx, base);
	// 826F19CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F19D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F19D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F19D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F19E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F19E0 size=24
    let mut pc: u32 = 0x826F19E0;
    'dispatch: loop {
        match pc {
            0x826F19E0 => {
    //   block [0x826F19E0..0x826F19F8)
	// 826F19E0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F19E4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F19E8: 394A1A30  addi r10, r10, 0x1a30
	ctx.r[10].s64 = ctx.r[10].s64 + 6704;
	// 826F19EC: 816BB98C  lwz r11, -0x4674(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18036 as u32) ) } as u64;
	// 826F19F0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F19F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F19F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F19F8 size=112
    let mut pc: u32 = 0x826F19F8;
    'dispatch: loop {
        match pc {
            0x826F19F8 => {
    //   block [0x826F19F8..0x826F1A68)
	// 826F19F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F19FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1A04: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F1A08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1A0C: 392A9D78  addi r9, r10, -0x6288
	ctx.r[9].s64 = ctx.r[10].s64 + -25224;
	// 826F1A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1A14: 390B1A30  addi r8, r11, 0x1a30
	ctx.r[8].s64 = ctx.r[11].s64 + 6704;
	// 826F1A18: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826F1A1C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826F1A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1A24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F1A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1A30: 386A8014  addi r3, r10, -0x7fec
	ctx.r[3].s64 = ctx.r[10].s64 + -32748;
	// 826F1A34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F1A38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F1A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1A54: 4BD753CD  bl 0x82466e20
	ctx.lr = 0x826F1A58;
	sub_82466E20(ctx, base);
	// 826F1A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1A68 size=108
    let mut pc: u32 = 0x826F1A68;
    'dispatch: loop {
        match pc {
            0x826F1A68 => {
    //   block [0x826F1A68..0x826F1AD4)
	// 826F1A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1A74: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1A7C: 38EBBA50  addi r7, r11, -0x45b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17840;
	// 826F1A80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1A84: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826F1A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1A8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1A98: 386A8044  addi r3, r10, -0x7fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -32700;
	// 826F1A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1AC0: 4BD75361  bl 0x82466e20
	ctx.lr = 0x826F1AC4;
	sub_82466E20(ctx, base);
	// 826F1AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1AD8 size=108
    let mut pc: u32 = 0x826F1AD8;
    'dispatch: loop {
        match pc {
            0x826F1AD8 => {
    //   block [0x826F1AD8..0x826F1B44)
	// 826F1AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1AE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1AEC: 38EBBA68  addi r7, r11, -0x4598
	ctx.r[7].s64 = ctx.r[11].s64 + -17816;
	// 826F1AF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F1AF4: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826F1AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1AFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1B08: 386A8074  addi r3, r10, -0x7f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -32652;
	// 826F1B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1B30: 4BD752F1  bl 0x82466e20
	ctx.lr = 0x826F1B34;
	sub_82466E20(ctx, base);
	// 826F1B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F1B48 size=24
    let mut pc: u32 = 0x826F1B48;
    'dispatch: loop {
        match pc {
            0x826F1B48 => {
    //   block [0x826F1B48..0x826F1B60)
	// 826F1B48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1B4C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F1B50: 394A1A78  addi r10, r10, 0x1a78
	ctx.r[10].s64 = ctx.r[10].s64 + 6776;
	// 826F1B54: 816BBA98  lwz r11, -0x4568(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17768 as u32) ) } as u64;
	// 826F1B58: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F1B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1B60 size=112
    let mut pc: u32 = 0x826F1B60;
    'dispatch: loop {
        match pc {
            0x826F1B60 => {
    //   block [0x826F1B60..0x826F1BD0)
	// 826F1B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1B6C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F1B70: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1B74: 392A9DB4  addi r9, r10, -0x624c
	ctx.r[9].s64 = ctx.r[10].s64 + -25164;
	// 826F1B78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1B7C: 390B1A78  addi r8, r11, 0x1a78
	ctx.r[8].s64 = ctx.r[11].s64 + 6776;
	// 826F1B80: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F1B84: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826F1B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1B8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F1B94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1B98: 386A80A4  addi r3, r10, -0x7f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -32604;
	// 826F1B9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F1BA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F1BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1BA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1BAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1BB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1BB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1BBC: 4BD75265  bl 0x82466e20
	ctx.lr = 0x826F1BC0;
	sub_82466E20(ctx, base);
	// 826F1BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1BCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1BD0 size=112
    let mut pc: u32 = 0x826F1BD0;
    'dispatch: loop {
        match pc {
            0x826F1BD0 => {
    //   block [0x826F1BD0..0x826F1C40)
	// 826F1BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1BDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F1BE0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1BE4: 38AA7D74  addi r5, r10, 0x7d74
	ctx.r[5].s64 = ctx.r[10].s64 + 32116;
	// 826F1BE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1BEC: 390BBA9C  addi r8, r11, -0x4564
	ctx.r[8].s64 = ctx.r[11].s64 + -17764;
	// 826F1BF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F1BF4: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 826F1BF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1BFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1C00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F1C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1C08: 386A80D4  addi r3, r10, -0x7f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -32556;
	// 826F1C0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F1C10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1C14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1C18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1C20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1C28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1C2C: 4BD751F5  bl 0x82466e20
	ctx.lr = 0x826F1C30;
	sub_82466E20(ctx, base);
	// 826F1C30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1C40 size=108
    let mut pc: u32 = 0x826F1C40;
    'dispatch: loop {
        match pc {
            0x826F1C40 => {
    //   block [0x826F1C40..0x826F1CAC)
	// 826F1C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1C4C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1C54: 38EBBACC  addi r7, r11, -0x4534
	ctx.r[7].s64 = ctx.r[11].s64 + -17716;
	// 826F1C58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F1C5C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826F1C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1C64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1C70: 386A8104  addi r3, r10, -0x7efc
	ctx.r[3].s64 = ctx.r[10].s64 + -32508;
	// 826F1C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1C98: 4BD75189  bl 0x82466e20
	ctx.lr = 0x826F1C9C;
	sub_82466E20(ctx, base);
	// 826F1C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1CB0 size=108
    let mut pc: u32 = 0x826F1CB0;
    'dispatch: loop {
        match pc {
            0x826F1CB0 => {
    //   block [0x826F1CB0..0x826F1D1C)
	// 826F1CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1CBC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1CC4: 38EBBB00  addi r7, r11, -0x4500
	ctx.r[7].s64 = ctx.r[11].s64 + -17664;
	// 826F1CC8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F1CCC: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826F1CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1CD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1CD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1CE0: 386A8134  addi r3, r10, -0x7ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -32460;
	// 826F1CE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1D08: 4BD75119  bl 0x82466e20
	ctx.lr = 0x826F1D0C;
	sub_82466E20(ctx, base);
	// 826F1D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1D20 size=108
    let mut pc: u32 = 0x826F1D20;
    'dispatch: loop {
        match pc {
            0x826F1D20 => {
    //   block [0x826F1D20..0x826F1D8C)
	// 826F1D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1D28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1D2C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1D34: 38EBBB60  addi r7, r11, -0x44a0
	ctx.r[7].s64 = ctx.r[11].s64 + -17568;
	// 826F1D38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F1D3C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826F1D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1D44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1D48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1D4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1D50: 386A8164  addi r3, r10, -0x7e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -32412;
	// 826F1D54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1D74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1D78: 4BD750A9  bl 0x82466e20
	ctx.lr = 0x826F1D7C;
	sub_82466E20(ctx, base);
	// 826F1D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1D90 size=108
    let mut pc: u32 = 0x826F1D90;
    'dispatch: loop {
        match pc {
            0x826F1D90 => {
    //   block [0x826F1D90..0x826F1DFC)
	// 826F1D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1D9C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1DA4: 38EBBB90  addi r7, r11, -0x4470
	ctx.r[7].s64 = ctx.r[11].s64 + -17520;
	// 826F1DA8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826F1DAC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826F1DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1DB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1DB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1DC0: 386A8194  addi r3, r10, -0x7e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -32364;
	// 826F1DC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1DE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1DE8: 4BD75039  bl 0x82466e20
	ctx.lr = 0x826F1DEC;
	sub_82466E20(ctx, base);
	// 826F1DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1E00 size=108
    let mut pc: u32 = 0x826F1E00;
    'dispatch: loop {
        match pc {
            0x826F1E00 => {
    //   block [0x826F1E00..0x826F1E6C)
	// 826F1E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1E0C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1E14: 38EBBCB0  addi r7, r11, -0x4350
	ctx.r[7].s64 = ctx.r[11].s64 + -17232;
	// 826F1E18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1E1C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 826F1E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1E24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1E30: 386A81C4  addi r3, r10, -0x7e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -32316;
	// 826F1E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1E58: 4BD74FC9  bl 0x82466e20
	ctx.lr = 0x826F1E5C;
	sub_82466E20(ctx, base);
	// 826F1E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1E70 size=108
    let mut pc: u32 = 0x826F1E70;
    'dispatch: loop {
        match pc {
            0x826F1E70 => {
    //   block [0x826F1E70..0x826F1EDC)
	// 826F1E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1E7C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1E80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1E84: 38EBBCC8  addi r7, r11, -0x4338
	ctx.r[7].s64 = ctx.r[11].s64 + -17208;
	// 826F1E88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1E8C: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 826F1E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1E94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1E98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1E9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1EA0: 386A81F4  addi r3, r10, -0x7e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -32268;
	// 826F1EA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1EA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1EAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1EBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1EC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1EC8: 4BD74F59  bl 0x82466e20
	ctx.lr = 0x826F1ECC;
	sub_82466E20(ctx, base);
	// 826F1ECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1ED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1EE0 size=108
    let mut pc: u32 = 0x826F1EE0;
    'dispatch: loop {
        match pc {
            0x826F1EE0 => {
    //   block [0x826F1EE0..0x826F1F4C)
	// 826F1EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1EEC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1EF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1EF4: 38EBBCE0  addi r7, r11, -0x4320
	ctx.r[7].s64 = ctx.r[11].s64 + -17184;
	// 826F1EF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1EFC: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 826F1F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1F04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1F08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1F10: 386A8224  addi r3, r10, -0x7ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -32220;
	// 826F1F14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1F18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1F24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1F2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1F38: 4BD74EE9  bl 0x82466e20
	ctx.lr = 0x826F1F3C;
	sub_82466E20(ctx, base);
	// 826F1F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1F50 size=108
    let mut pc: u32 = 0x826F1F50;
    'dispatch: loop {
        match pc {
            0x826F1F50 => {
    //   block [0x826F1F50..0x826F1FBC)
	// 826F1F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1F5C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1F64: 38EBBCF8  addi r7, r11, -0x4308
	ctx.r[7].s64 = ctx.r[11].s64 + -17160;
	// 826F1F68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1F6C: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 826F1F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1F74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1F78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1F80: 386A8254  addi r3, r10, -0x7dac
	ctx.r[3].s64 = ctx.r[10].s64 + -32172;
	// 826F1F84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F1F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F1F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F1F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F1F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F1FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F1FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F1FA8: 4BD74E79  bl 0x82466e20
	ctx.lr = 0x826F1FAC;
	sub_82466E20(ctx, base);
	// 826F1FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F1FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F1FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F1FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F1FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F1FC0 size=108
    let mut pc: u32 = 0x826F1FC0;
    'dispatch: loop {
        match pc {
            0x826F1FC0 => {
    //   block [0x826F1FC0..0x826F202C)
	// 826F1FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F1FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F1FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F1FCC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F1FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F1FD4: 38EBBD10  addi r7, r11, -0x42f0
	ctx.r[7].s64 = ctx.r[11].s64 + -17136;
	// 826F1FD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F1FDC: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826F1FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F1FE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F1FE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F1FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F1FF0: 386A8284  addi r3, r10, -0x7d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -32124;
	// 826F1FF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F1FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F1FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F200C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2018: 4BD74E09  bl 0x82466e20
	ctx.lr = 0x826F201C;
	sub_82466E20(ctx, base);
	// 826F201C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2030 size=108
    let mut pc: u32 = 0x826F2030;
    'dispatch: loop {
        match pc {
            0x826F2030 => {
    //   block [0x826F2030..0x826F209C)
	// 826F2030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F203C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2040: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2044: 38EBBD28  addi r7, r11, -0x42d8
	ctx.r[7].s64 = ctx.r[11].s64 + -17112;
	// 826F2048: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F204C: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 826F2050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2054: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F205C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2060: 386A82B4  addi r3, r10, -0x7d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -32076;
	// 826F2064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F206C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F207C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2088: 4BD74D99  bl 0x82466e20
	ctx.lr = 0x826F208C;
	sub_82466E20(ctx, base);
	// 826F208C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F20A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F20A0 size=108
    let mut pc: u32 = 0x826F20A0;
    'dispatch: loop {
        match pc {
            0x826F20A0 => {
    //   block [0x826F20A0..0x826F210C)
	// 826F20A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F20A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F20A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F20AC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F20B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F20B4: 38EBBD40  addi r7, r11, -0x42c0
	ctx.r[7].s64 = ctx.r[11].s64 + -17088;
	// 826F20B8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826F20BC: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826F20C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F20C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F20C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F20CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F20D0: 386A82E4  addi r3, r10, -0x7d1c
	ctx.r[3].s64 = ctx.r[10].s64 + -32028;
	// 826F20D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F20D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F20DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F20E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F20E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F20E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F20EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F20F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F20F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F20F8: 4BD74D29  bl 0x82466e20
	ctx.lr = 0x826F20FC;
	sub_82466E20(ctx, base);
	// 826F20FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2110 size=108
    let mut pc: u32 = 0x826F2110;
    'dispatch: loop {
        match pc {
            0x826F2110 => {
    //   block [0x826F2110..0x826F217C)
	// 826F2110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F211C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2124: 38EBBDD0  addi r7, r11, -0x4230
	ctx.r[7].s64 = ctx.r[11].s64 + -16944;
	// 826F2128: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826F212C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826F2130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2134: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F213C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2140: 386A8314  addi r3, r10, -0x7cec
	ctx.r[3].s64 = ctx.r[10].s64 + -31980;
	// 826F2144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F214C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F215C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2168: 4BD74CB9  bl 0x82466e20
	ctx.lr = 0x826F216C;
	sub_82466E20(ctx, base);
	// 826F216C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2180 size=108
    let mut pc: u32 = 0x826F2180;
    'dispatch: loop {
        match pc {
            0x826F2180 => {
    //   block [0x826F2180..0x826F21EC)
	// 826F2180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F218C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2194: 38EBBE90  addi r7, r11, -0x4170
	ctx.r[7].s64 = ctx.r[11].s64 + -16752;
	// 826F2198: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826F219C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826F21A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F21A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F21A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F21AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F21B0: 386A8344  addi r3, r10, -0x7cbc
	ctx.r[3].s64 = ctx.r[10].s64 + -31932;
	// 826F21B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F21B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F21BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F21C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F21C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F21C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F21CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F21D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F21D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F21D8: 4BD74C49  bl 0x82466e20
	ctx.lr = 0x826F21DC;
	sub_82466E20(ctx, base);
	// 826F21DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F21E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F21E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F21E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F21F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F21F0 size=108
    let mut pc: u32 = 0x826F21F0;
    'dispatch: loop {
        match pc {
            0x826F21F0 => {
    //   block [0x826F21F0..0x826F225C)
	// 826F21F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F21F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F21F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F21FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2204: 38EBBF68  addi r7, r11, -0x4098
	ctx.r[7].s64 = ctx.r[11].s64 + -16536;
	// 826F2208: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826F220C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826F2210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2214: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F221C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2220: 386A8374  addi r3, r10, -0x7c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -31884;
	// 826F2224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F222C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F223C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2248: 4BD74BD9  bl 0x82466e20
	ctx.lr = 0x826F224C;
	sub_82466E20(ctx, base);
	// 826F224C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2260 size=108
    let mut pc: u32 = 0x826F2260;
    'dispatch: loop {
        match pc {
            0x826F2260 => {
    //   block [0x826F2260..0x826F22CC)
	// 826F2260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F226C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2274: 38EBC028  addi r7, r11, -0x3fd8
	ctx.r[7].s64 = ctx.r[11].s64 + -16344;
	// 826F2278: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826F227C: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826F2280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2284: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F228C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2290: 386A83A4  addi r3, r10, -0x7c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -31836;
	// 826F2294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F229C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F22A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F22A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F22A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F22AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F22B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F22B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F22B8: 4BD74B69  bl 0x82466e20
	ctx.lr = 0x826F22BC;
	sub_82466E20(ctx, base);
	// 826F22BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F22C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F22C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F22C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F22D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F22D0 size=112
    let mut pc: u32 = 0x826F22D0;
    'dispatch: loop {
        match pc {
            0x826F22D0 => {
    //   block [0x826F22D0..0x826F2340)
	// 826F22D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F22D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F22D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F22DC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F22E0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826F22E4: 38EAC0D0  addi r7, r10, -0x3f30
	ctx.r[7].s64 = ctx.r[10].s64 + -16176;
	// 826F22E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F22EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F22F0: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826F22F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F22F8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F22FC: 396B9DC8  addi r11, r11, -0x6238
	ctx.r[11].s64 = ctx.r[11].s64 + -25144;
	// 826F2300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2304: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2308: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F230C: 386A83D4  addi r3, r10, -0x7c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -31788;
	// 826F2310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2314: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F2318: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F231C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F2320: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2324: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2328: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F232C: 4BD74AF5  bl 0x82466e20
	ctx.lr = 0x826F2330;
	sub_82466E20(ctx, base);
	// 826F2330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F233C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2340 size=108
    let mut pc: u32 = 0x826F2340;
    'dispatch: loop {
        match pc {
            0x826F2340 => {
    //   block [0x826F2340..0x826F23AC)
	// 826F2340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F234C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2354: 38EBC1F0  addi r7, r11, -0x3e10
	ctx.r[7].s64 = ctx.r[11].s64 + -15888;
	// 826F2358: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F235C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826F2360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2364: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F236C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2370: 386A8404  addi r3, r10, -0x7bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -31740;
	// 826F2374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F237C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F238C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2398: 4BD74A89  bl 0x82466e20
	ctx.lr = 0x826F239C;
	sub_82466E20(ctx, base);
	// 826F239C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F23A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F23A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F23A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F23B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F23B0 size=108
    let mut pc: u32 = 0x826F23B0;
    'dispatch: loop {
        match pc {
            0x826F23B0 => {
    //   block [0x826F23B0..0x826F241C)
	// 826F23B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F23B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F23B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F23BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F23C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F23C4: 38EBC250  addi r7, r11, -0x3db0
	ctx.r[7].s64 = ctx.r[11].s64 + -15792;
	// 826F23C8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826F23CC: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826F23D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F23D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F23D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F23DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F23E0: 386A8434  addi r3, r10, -0x7bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -31692;
	// 826F23E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F23E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F23EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F23F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F23F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F23F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F23FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2408: 4BD74A19  bl 0x82466e20
	ctx.lr = 0x826F240C;
	sub_82466E20(ctx, base);
	// 826F240C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2420 size=108
    let mut pc: u32 = 0x826F2420;
    'dispatch: loop {
        match pc {
            0x826F2420 => {
    //   block [0x826F2420..0x826F248C)
	// 826F2420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F242C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2434: 38EBC358  addi r7, r11, -0x3ca8
	ctx.r[7].s64 = ctx.r[11].s64 + -15528;
	// 826F2438: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826F243C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826F2440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2444: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F244C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2450: 386A8464  addi r3, r10, -0x7b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -31644;
	// 826F2454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F245C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F246C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2478: 4BD749A9  bl 0x82466e20
	ctx.lr = 0x826F247C;
	sub_82466E20(ctx, base);
	// 826F247C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2490 size=108
    let mut pc: u32 = 0x826F2490;
    'dispatch: loop {
        match pc {
            0x826F2490 => {
    //   block [0x826F2490..0x826F24FC)
	// 826F2490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F249C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F24A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F24A4: 38EBC430  addi r7, r11, -0x3bd0
	ctx.r[7].s64 = ctx.r[11].s64 + -15312;
	// 826F24A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F24AC: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826F24B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F24B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F24B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F24BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F24C0: 386A8494  addi r3, r10, -0x7b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -31596;
	// 826F24C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F24C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F24CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F24D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F24D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F24D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F24DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F24E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F24E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F24E8: 4BD74939  bl 0x82466e20
	ctx.lr = 0x826F24EC;
	sub_82466E20(ctx, base);
	// 826F24EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F24F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F24F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F24F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2500 size=108
    let mut pc: u32 = 0x826F2500;
    'dispatch: loop {
        match pc {
            0x826F2500 => {
    //   block [0x826F2500..0x826F256C)
	// 826F2500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F250C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2514: 38EBC478  addi r7, r11, -0x3b88
	ctx.r[7].s64 = ctx.r[11].s64 + -15240;
	// 826F2518: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F251C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826F2520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2524: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F252C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2530: 386A84C4  addi r3, r10, -0x7b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -31548;
	// 826F2534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F253C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F254C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2558: 4BD748C9  bl 0x82466e20
	ctx.lr = 0x826F255C;
	sub_82466E20(ctx, base);
	// 826F255C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2570 size=112
    let mut pc: u32 = 0x826F2570;
    'dispatch: loop {
        match pc {
            0x826F2570 => {
    //   block [0x826F2570..0x826F25E0)
	// 826F2570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F257C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F2580: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2584: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F2588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F258C: 390BC490  addi r8, r11, -0x3b70
	ctx.r[8].s64 = ctx.r[11].s64 + -15216;
	// 826F2590: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F2594: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826F2598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F259C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F25A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F25A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F25A8: 386A84F4  addi r3, r10, -0x7b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -31500;
	// 826F25AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F25B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F25B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F25B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F25BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F25C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F25C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F25C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F25CC: 4BD74855  bl 0x82466e20
	ctx.lr = 0x826F25D0;
	sub_82466E20(ctx, base);
	// 826F25D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F25D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F25D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F25DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F25E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F25E0 size=112
    let mut pc: u32 = 0x826F25E0;
    'dispatch: loop {
        match pc {
            0x826F25E0 => {
    //   block [0x826F25E0..0x826F2650)
	// 826F25E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F25E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F25E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F25EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F25F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F25F4: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F25F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F25FC: 390BC4F0  addi r8, r11, -0x3b10
	ctx.r[8].s64 = ctx.r[11].s64 + -15120;
	// 826F2600: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F2604: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 826F2608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F260C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F2614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2618: 386A8524  addi r3, r10, -0x7adc
	ctx.r[3].s64 = ctx.r[10].s64 + -31452;
	// 826F261C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F2620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F262C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F263C: 4BD747E5  bl 0x82466e20
	ctx.lr = 0x826F2640;
	sub_82466E20(ctx, base);
	// 826F2640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F264C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2650 size=108
    let mut pc: u32 = 0x826F2650;
    'dispatch: loop {
        match pc {
            0x826F2650 => {
    //   block [0x826F2650..0x826F26BC)
	// 826F2650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F265C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2664: 38EBC538  addi r7, r11, -0x3ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -15048;
	// 826F2668: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F266C: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 826F2670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2674: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F267C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2680: 386A8554  addi r3, r10, -0x7aac
	ctx.r[3].s64 = ctx.r[10].s64 + -31404;
	// 826F2684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F268C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F269C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F26A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F26A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F26A8: 4BD74779  bl 0x82466e20
	ctx.lr = 0x826F26AC;
	sub_82466E20(ctx, base);
	// 826F26AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F26B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F26B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F26B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F26C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F26C0 size=24
    let mut pc: u32 = 0x826F26C0;
    'dispatch: loop {
        match pc {
            0x826F26C0 => {
    //   block [0x826F26C0..0x826F26D8)
	// 826F26C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F26C4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F26C8: 394A1AF0  addi r10, r10, 0x1af0
	ctx.r[10].s64 = ctx.r[10].s64 + 6896;
	// 826F26CC: 816BBAFC  lwz r11, -0x4504(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17668 as u32) ) } as u64;
	// 826F26D0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F26D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F26D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F26D8 size=112
    let mut pc: u32 = 0x826F26D8;
    'dispatch: loop {
        match pc {
            0x826F26D8 => {
    //   block [0x826F26D8..0x826F2748)
	// 826F26D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F26DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F26E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F26E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F26E8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F26EC: 38AA8764  addi r5, r10, -0x789c
	ctx.r[5].s64 = ctx.r[10].s64 + -30876;
	// 826F26F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F26F4: 390B1AF0  addi r8, r11, 0x1af0
	ctx.r[8].s64 = ctx.r[11].s64 + 6896;
	// 826F26F8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826F26FC: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 826F2700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2704: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F270C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2710: 386A8584  addi r3, r10, -0x7a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -31356;
	// 826F2714: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F2718: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F271C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F272C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2734: 4BD746ED  bl 0x82466e20
	ctx.lr = 0x826F2738;
	sub_82466E20(ctx, base);
	// 826F2738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F273C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2748 size=108
    let mut pc: u32 = 0x826F2748;
    'dispatch: loop {
        match pc {
            0x826F2748 => {
    //   block [0x826F2748..0x826F27B4)
	// 826F2748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F274C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2754: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F275C: 38EBC550  addi r7, r11, -0x3ab0
	ctx.r[7].s64 = ctx.r[11].s64 + -15024;
	// 826F2760: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F2764: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 826F2768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F276C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2778: 386A85B4  addi r3, r10, -0x7a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -31308;
	// 826F277C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F278C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F279C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F27A0: 4BD74681  bl 0x82466e20
	ctx.lr = 0x826F27A4;
	sub_82466E20(ctx, base);
	// 826F27A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F27A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F27AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F27B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F27B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F27B8 size=112
    let mut pc: u32 = 0x826F27B8;
    'dispatch: loop {
        match pc {
            0x826F27B8 => {
    //   block [0x826F27B8..0x826F2828)
	// 826F27B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F27BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F27C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F27C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F27C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F27CC: 38AA8764  addi r5, r10, -0x789c
	ctx.r[5].s64 = ctx.r[10].s64 + -30876;
	// 826F27D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F27D4: 390BC5B0  addi r8, r11, -0x3a50
	ctx.r[8].s64 = ctx.r[11].s64 + -14928;
	// 826F27D8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826F27DC: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 826F27E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F27E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F27E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F27EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F27F0: 386A85E4  addi r3, r10, -0x7a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -31260;
	// 826F27F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F27F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F27FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F280C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2814: 4BD7460D  bl 0x82466e20
	ctx.lr = 0x826F2818;
	sub_82466E20(ctx, base);
	// 826F2818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F281C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2828 size=108
    let mut pc: u32 = 0x826F2828;
    'dispatch: loop {
        match pc {
            0x826F2828 => {
    //   block [0x826F2828..0x826F2894)
	// 826F2828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F282C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2834: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F283C: 38EBC670  addi r7, r11, -0x3990
	ctx.r[7].s64 = ctx.r[11].s64 + -14736;
	// 826F2840: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F2844: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 826F2848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F284C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2858: 386A8614  addi r3, r10, -0x79ec
	ctx.r[3].s64 = ctx.r[10].s64 + -31212;
	// 826F285C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F286C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F287C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2880: 4BD745A1  bl 0x82466e20
	ctx.lr = 0x826F2884;
	sub_82466E20(ctx, base);
	// 826F2884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F288C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2898 size=108
    let mut pc: u32 = 0x826F2898;
    'dispatch: loop {
        match pc {
            0x826F2898 => {
    //   block [0x826F2898..0x826F2904)
	// 826F2898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F289C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F28A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F28A4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F28A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F28AC: 38EBC6E8  addi r7, r11, -0x3918
	ctx.r[7].s64 = ctx.r[11].s64 + -14616;
	// 826F28B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F28B4: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 826F28B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F28BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F28C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F28C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F28C8: 386A8644  addi r3, r10, -0x79bc
	ctx.r[3].s64 = ctx.r[10].s64 + -31164;
	// 826F28CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F28D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F28D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F28D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F28DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F28E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F28E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F28E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F28EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F28F0: 4BD74531  bl 0x82466e20
	ctx.lr = 0x826F28F4;
	sub_82466E20(ctx, base);
	// 826F28F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F28F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F28FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2908 size=108
    let mut pc: u32 = 0x826F2908;
    'dispatch: loop {
        match pc {
            0x826F2908 => {
    //   block [0x826F2908..0x826F2974)
	// 826F2908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F290C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2914: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F291C: 38EBC730  addi r7, r11, -0x38d0
	ctx.r[7].s64 = ctx.r[11].s64 + -14544;
	// 826F2920: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F2924: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 826F2928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F292C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2938: 386A8674  addi r3, r10, -0x798c
	ctx.r[3].s64 = ctx.r[10].s64 + -31116;
	// 826F293C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F294C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F295C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2960: 4BD744C1  bl 0x82466e20
	ctx.lr = 0x826F2964;
	sub_82466E20(ctx, base);
	// 826F2964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F296C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2978 size=112
    let mut pc: u32 = 0x826F2978;
    'dispatch: loop {
        match pc {
            0x826F2978 => {
    //   block [0x826F2978..0x826F29E8)
	// 826F2978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F297C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2984: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2988: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F298C: 38AA8674  addi r5, r10, -0x798c
	ctx.r[5].s64 = ctx.r[10].s64 + -31116;
	// 826F2990: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2994: 390BC778  addi r8, r11, -0x3888
	ctx.r[8].s64 = ctx.r[11].s64 + -14472;
	// 826F2998: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F299C: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 826F29A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F29A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F29A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F29AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F29B0: 386A86A4  addi r3, r10, -0x795c
	ctx.r[3].s64 = ctx.r[10].s64 + -31068;
	// 826F29B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F29B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F29BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F29C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F29C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F29C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F29CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F29D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F29D4: 4BD7444D  bl 0x82466e20
	ctx.lr = 0x826F29D8;
	sub_82466E20(ctx, base);
	// 826F29D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F29DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F29E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F29E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F29E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F29E8 size=108
    let mut pc: u32 = 0x826F29E8;
    'dispatch: loop {
        match pc {
            0x826F29E8 => {
    //   block [0x826F29E8..0x826F2A54)
	// 826F29E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F29EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F29F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F29F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F29F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F29FC: 38EBC7F0  addi r7, r11, -0x3810
	ctx.r[7].s64 = ctx.r[11].s64 + -14352;
	// 826F2A00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F2A04: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 826F2A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2A0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2A18: 386A86D4  addi r3, r10, -0x792c
	ctx.r[3].s64 = ctx.r[10].s64 + -31020;
	// 826F2A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2A40: 4BD743E1  bl 0x82466e20
	ctx.lr = 0x826F2A44;
	sub_82466E20(ctx, base);
	// 826F2A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2A58 size=108
    let mut pc: u32 = 0x826F2A58;
    'dispatch: loop {
        match pc {
            0x826F2A58 => {
    //   block [0x826F2A58..0x826F2AC4)
	// 826F2A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2A64: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2A6C: 38EBC838  addi r7, r11, -0x37c8
	ctx.r[7].s64 = ctx.r[11].s64 + -14280;
	// 826F2A70: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826F2A74: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 826F2A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2A7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2A80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2A88: 386A8704  addi r3, r10, -0x78fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30972;
	// 826F2A8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2AAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2AB0: 4BD74371  bl 0x82466e20
	ctx.lr = 0x826F2AB4;
	sub_82466E20(ctx, base);
	// 826F2AB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2AC8 size=108
    let mut pc: u32 = 0x826F2AC8;
    'dispatch: loop {
        match pc {
            0x826F2AC8 => {
    //   block [0x826F2AC8..0x826F2B34)
	// 826F2AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2AD4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2ADC: 38EBC8F8  addi r7, r11, -0x3708
	ctx.r[7].s64 = ctx.r[11].s64 + -14088;
	// 826F2AE0: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 826F2AE4: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 826F2AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2AEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2AF8: 386A8734  addi r3, r10, -0x78cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30924;
	// 826F2AFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2B20: 4BD74301  bl 0x82466e20
	ctx.lr = 0x826F2B24;
	sub_82466E20(ctx, base);
	// 826F2B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2B38 size=112
    let mut pc: u32 = 0x826F2B38;
    'dispatch: loop {
        match pc {
            0x826F2B38 => {
    //   block [0x826F2B38..0x826F2BA8)
	// 826F2B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2B44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F2B48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2B4C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F2B50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2B54: 390BCA78  addi r8, r11, -0x3588
	ctx.r[8].s64 = ctx.r[11].s64 + -13704;
	// 826F2B58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F2B5C: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 826F2B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2B64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2B68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F2B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2B70: 386A8764  addi r3, r10, -0x789c
	ctx.r[3].s64 = ctx.r[10].s64 + -30876;
	// 826F2B74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F2B78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2B7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2B84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2B8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2B94: 4BD7428D  bl 0x82466e20
	ctx.lr = 0x826F2B98;
	sub_82466E20(ctx, base);
	// 826F2B98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2BA8 size=108
    let mut pc: u32 = 0x826F2BA8;
    'dispatch: loop {
        match pc {
            0x826F2BA8 => {
    //   block [0x826F2BA8..0x826F2C14)
	// 826F2BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2BB4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2BBC: 38EBCAD8  addi r7, r11, -0x3528
	ctx.r[7].s64 = ctx.r[11].s64 + -13608;
	// 826F2BC0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F2BC4: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 826F2BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2BCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2BD8: 386A8794  addi r3, r10, -0x786c
	ctx.r[3].s64 = ctx.r[10].s64 + -30828;
	// 826F2BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2C00: 4BD74221  bl 0x82466e20
	ctx.lr = 0x826F2C04;
	sub_82466E20(ctx, base);
	// 826F2C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2C18 size=112
    let mut pc: u32 = 0x826F2C18;
    'dispatch: loop {
        match pc {
            0x826F2C18 => {
    //   block [0x826F2C18..0x826F2C88)
	// 826F2C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2C24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F2C28: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2C2C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F2C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2C34: 390BCB50  addi r8, r11, -0x34b0
	ctx.r[8].s64 = ctx.r[11].s64 + -13488;
	// 826F2C38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F2C3C: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 826F2C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2C44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F2C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2C50: 386A87C4  addi r3, r10, -0x783c
	ctx.r[3].s64 = ctx.r[10].s64 + -30780;
	// 826F2C54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F2C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2C74: 4BD741AD  bl 0x82466e20
	ctx.lr = 0x826F2C78;
	sub_82466E20(ctx, base);
	// 826F2C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2C88 size=108
    let mut pc: u32 = 0x826F2C88;
    'dispatch: loop {
        match pc {
            0x826F2C88 => {
    //   block [0x826F2C88..0x826F2CF4)
	// 826F2C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2C94: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2C9C: 38EBCB98  addi r7, r11, -0x3468
	ctx.r[7].s64 = ctx.r[11].s64 + -13416;
	// 826F2CA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F2CA4: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 826F2CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2CAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2CB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2CB8: 386A87F4  addi r3, r10, -0x780c
	ctx.r[3].s64 = ctx.r[10].s64 + -30732;
	// 826F2CBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2CDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2CE0: 4BD74141  bl 0x82466e20
	ctx.lr = 0x826F2CE4;
	sub_82466E20(ctx, base);
	// 826F2CE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2CF8 size=108
    let mut pc: u32 = 0x826F2CF8;
    'dispatch: loop {
        match pc {
            0x826F2CF8 => {
    //   block [0x826F2CF8..0x826F2D64)
	// 826F2CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2D04: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2D08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F2D0C: 38EBCBF8  addi r7, r11, -0x3408
	ctx.r[7].s64 = ctx.r[11].s64 + -13320;
	// 826F2D10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F2D14: 388A0DBC  addi r4, r10, 0xdbc
	ctx.r[4].s64 = ctx.r[10].s64 + 3516;
	// 826F2D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2D1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2D20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2D28: 386A8824  addi r3, r10, -0x77dc
	ctx.r[3].s64 = ctx.r[10].s64 + -30684;
	// 826F2D2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2D50: 4BD740D1  bl 0x82466e20
	ctx.lr = 0x826F2D54;
	sub_82466E20(ctx, base);
	// 826F2D54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2D68 size=108
    let mut pc: u32 = 0x826F2D68;
    'dispatch: loop {
        match pc {
            0x826F2D68 => {
    //   block [0x826F2D68..0x826F2DD4)
	// 826F2D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2D74: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2D7C: 38EBCC40  addi r7, r11, -0x33c0
	ctx.r[7].s64 = ctx.r[11].s64 + -13248;
	// 826F2D80: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826F2D84: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 826F2D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2D8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2D90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2D98: 386A8854  addi r3, r10, -0x77ac
	ctx.r[3].s64 = ctx.r[10].s64 + -30636;
	// 826F2D9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2DC0: 4BD74061  bl 0x82466e20
	ctx.lr = 0x826F2DC4;
	sub_82466E20(ctx, base);
	// 826F2DC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2DD8 size=108
    let mut pc: u32 = 0x826F2DD8;
    'dispatch: loop {
        match pc {
            0x826F2DD8 => {
    //   block [0x826F2DD8..0x826F2E44)
	// 826F2DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2DE4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2DEC: 38EBCD00  addi r7, r11, -0x3300
	ctx.r[7].s64 = ctx.r[11].s64 + -13056;
	// 826F2DF0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826F2DF4: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 826F2DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2DFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2E00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2E08: 386A8884  addi r3, r10, -0x777c
	ctx.r[3].s64 = ctx.r[10].s64 + -30588;
	// 826F2E0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2E30: 4BD73FF1  bl 0x82466e20
	ctx.lr = 0x826F2E34;
	sub_82466E20(ctx, base);
	// 826F2E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2E48 size=108
    let mut pc: u32 = 0x826F2E48;
    'dispatch: loop {
        match pc {
            0x826F2E48 => {
    //   block [0x826F2E48..0x826F2EB4)
	// 826F2E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2E54: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2E58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2E5C: 38EBCD90  addi r7, r11, -0x3270
	ctx.r[7].s64 = ctx.r[11].s64 + -12912;
	// 826F2E60: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826F2E64: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 826F2E68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2E6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2E70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2E78: 386A88B4  addi r3, r10, -0x774c
	ctx.r[3].s64 = ctx.r[10].s64 + -30540;
	// 826F2E7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2E80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2E8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2E94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2E9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2EA0: 4BD73F81  bl 0x82466e20
	ctx.lr = 0x826F2EA4;
	sub_82466E20(ctx, base);
	// 826F2EA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2EB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2EB8 size=108
    let mut pc: u32 = 0x826F2EB8;
    'dispatch: loop {
        match pc {
            0x826F2EB8 => {
    //   block [0x826F2EB8..0x826F2F24)
	// 826F2EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2EC4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2EC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2ECC: 38EBCEC8  addi r7, r11, -0x3138
	ctx.r[7].s64 = ctx.r[11].s64 + -12600;
	// 826F2ED0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F2ED4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826F2ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2EDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2EE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F2EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2EE8: 386A88E4  addi r3, r10, -0x771c
	ctx.r[3].s64 = ctx.r[10].s64 + -30492;
	// 826F2EEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F2EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2EF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2F04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2F0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2F10: 4BD73F11  bl 0x82466e20
	ctx.lr = 0x826F2F14;
	sub_82466E20(ctx, base);
	// 826F2F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2F28 size=116
    let mut pc: u32 = 0x826F2F28;
    'dispatch: loop {
        match pc {
            0x826F2F28 => {
    //   block [0x826F2F28..0x826F2F9C)
	// 826F2F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2F34: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F2F38: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F2F3C: 390BCF30  addi r8, r11, -0x30d0
	ctx.r[8].s64 = ctx.r[11].s64 + -12496;
	// 826F2F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2F44: 392A9E7C  addi r9, r10, -0x6184
	ctx.r[9].s64 = ctx.r[10].s64 + -24964;
	// 826F2F48: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2F4C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F2F50: 38AA88E4  addi r5, r10, -0x771c
	ctx.r[5].s64 = ctx.r[10].s64 + -30492;
	// 826F2F54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F2F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2F5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F2F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2F6C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F2F70: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826F2F74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F2F78: 386B8914  addi r3, r11, -0x76ec
	ctx.r[3].s64 = ctx.r[11].s64 + -30444;
	// 826F2F7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F2F80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2F88: 4BD73E99  bl 0x82466e20
	ctx.lr = 0x826F2F8C;
	sub_82466E20(ctx, base);
	// 826F2F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F2FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F2FA0 size=96
    let mut pc: u32 = 0x826F2FA0;
    'dispatch: loop {
        match pc {
            0x826F2FA0 => {
    //   block [0x826F2FA0..0x826F3000)
	// 826F2FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F2FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F2FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F2FAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F2FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F2FB4: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826F2FB8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F2FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F2FC0: 386A8944  addi r3, r10, -0x76bc
	ctx.r[3].s64 = ctx.r[10].s64 + -30396;
	// 826F2FC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F2FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F2FCC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F2FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F2FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F2FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F2FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F2FE0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F2FE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F2FE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F2FEC: 4BD73E35  bl 0x82466e20
	ctx.lr = 0x826F2FF0;
	sub_82466E20(ctx, base);
	// 826F2FF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F2FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F2FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F2FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3000 size=112
    let mut pc: u32 = 0x826F3000;
    'dispatch: loop {
        match pc {
            0x826F3000 => {
    //   block [0x826F3000..0x826F3070)
	// 826F3000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F300C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3010: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3014: 38AAAA14  addi r5, r10, -0x55ec
	ctx.r[5].s64 = ctx.r[10].s64 + -21996;
	// 826F3018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F301C: 390BCFA8  addi r8, r11, -0x3058
	ctx.r[8].s64 = ctx.r[11].s64 + -12376;
	// 826F3020: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F3024: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826F3028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F302C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3038: 386A8974  addi r3, r10, -0x768c
	ctx.r[3].s64 = ctx.r[10].s64 + -30348;
	// 826F303C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F304C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F305C: 4BD73DC5  bl 0x82466e20
	ctx.lr = 0x826F3060;
	sub_82466E20(ctx, base);
	// 826F3060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F306C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3070 size=96
    let mut pc: u32 = 0x826F3070;
    'dispatch: loop {
        match pc {
            0x826F3070 => {
    //   block [0x826F3070..0x826F30D0)
	// 826F3070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F307C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3084: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 826F3088: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F308C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3090: 386A89A4  addi r3, r10, -0x765c
	ctx.r[3].s64 = ctx.r[10].s64 + -30300;
	// 826F3094: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F309C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F30A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F30A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F30A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F30AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F30B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F30B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F30B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F30BC: 4BD73D65  bl 0x82466e20
	ctx.lr = 0x826F30C0;
	sub_82466E20(ctx, base);
	// 826F30C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F30C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F30C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F30CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F30D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F30D0 size=24
    let mut pc: u32 = 0x826F30D0;
    'dispatch: loop {
        match pc {
            0x826F30D0 => {
    //   block [0x826F30D0..0x826F30E8)
	// 826F30D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F30D4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F30D8: 394A1BB0  addi r10, r10, 0x1bb0
	ctx.r[10].s64 = ctx.r[10].s64 + 7088;
	// 826F30DC: 816BCF2C  lwz r11, -0x30d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12500 as u32) ) } as u64;
	// 826F30E0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F30E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F30E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F30E8 size=116
    let mut pc: u32 = 0x826F30E8;
    'dispatch: loop {
        match pc {
            0x826F30E8 => {
    //   block [0x826F30E8..0x826F315C)
	// 826F30E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F30EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F30F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F30F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F30F8: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F30FC: 390B1BB0  addi r8, r11, 0x1bb0
	ctx.r[8].s64 = ctx.r[11].s64 + 7088;
	// 826F3100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3104: 392A9EB8  addi r9, r10, -0x6148
	ctx.r[9].s64 = ctx.r[10].s64 + -24904;
	// 826F3108: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F310C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826F3110: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F3114: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F311C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3124: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F3128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F312C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F3130: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 826F3134: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F3138: 386B89D4  addi r3, r11, -0x762c
	ctx.r[3].s64 = ctx.r[11].s64 + -30252;
	// 826F313C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F3140: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3148: 4BD73CD9  bl 0x82466e20
	ctx.lr = 0x826F314C;
	sub_82466E20(ctx, base);
	// 826F314C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3160 size=104
    let mut pc: u32 = 0x826F3160;
    'dispatch: loop {
        match pc {
            0x826F3160 => {
    //   block [0x826F3160..0x826F31C8)
	// 826F3160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F316C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F3170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3174: 392A9EE4  addi r9, r10, -0x611c
	ctx.r[9].s64 = ctx.r[10].s64 + -24860;
	// 826F3178: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F317C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3180: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F3184: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F318C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3194: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 826F3198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F319C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F31A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F31A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F31A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F31AC: 386A8A04  addi r3, r10, -0x75fc
	ctx.r[3].s64 = ctx.r[10].s64 + -30204;
	// 826F31B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F31B4: 4BD73C6D  bl 0x82466e20
	ctx.lr = 0x826F31B8;
	sub_82466E20(ctx, base);
	// 826F31B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F31BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F31C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F31C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F31C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F31C8 size=96
    let mut pc: u32 = 0x826F31C8;
    'dispatch: loop {
        match pc {
            0x826F31C8 => {
    //   block [0x826F31C8..0x826F3228)
	// 826F31C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F31CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F31D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F31D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F31D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F31DC: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826F31E0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F31E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F31E8: 386A8A34  addi r3, r10, -0x75cc
	ctx.r[3].s64 = ctx.r[10].s64 + -30156;
	// 826F31EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F31F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F31F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F31F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F31FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3208: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F320C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3210: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3214: 4BD73C0D  bl 0x82466e20
	ctx.lr = 0x826F3218;
	sub_82466E20(ctx, base);
	// 826F3218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F321C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3228 size=100
    let mut pc: u32 = 0x826F3228;
    'dispatch: loop {
        match pc {
            0x826F3228 => {
    //   block [0x826F3228..0x826F328C)
	// 826F3228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F322C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3234: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F323C: 38AA8A04  addi r5, r10, -0x75fc
	ctx.r[5].s64 = ctx.r[10].s64 + -30204;
	// 826F3240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3248: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 826F324C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F325C: 386A8A64  addi r3, r10, -0x759c
	ctx.r[3].s64 = ctx.r[10].s64 + -30108;
	// 826F3260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3264: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3268: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F326C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3270: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3278: 4BD73BA9  bl 0x82466e20
	ctx.lr = 0x826F327C;
	sub_82466E20(ctx, base);
	// 826F327C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3290 size=112
    let mut pc: u32 = 0x826F3290;
    'dispatch: loop {
        match pc {
            0x826F3290 => {
    //   block [0x826F3290..0x826F3300)
	// 826F3290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F329C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F32A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F32A4: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F32A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F32AC: 390BD010  addi r8, r11, -0x2ff0
	ctx.r[8].s64 = ctx.r[11].s64 + -12272;
	// 826F32B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F32B4: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826F32B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F32BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F32C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F32C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F32C8: 386A8A94  addi r3, r10, -0x756c
	ctx.r[3].s64 = ctx.r[10].s64 + -30060;
	// 826F32CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F32D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F32D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F32D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F32DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F32E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F32E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F32E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F32EC: 4BD73B35  bl 0x82466e20
	ctx.lr = 0x826F32F0;
	sub_82466E20(ctx, base);
	// 826F32F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F32F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F32F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F32FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3300 size=112
    let mut pc: u32 = 0x826F3300;
    'dispatch: loop {
        match pc {
            0x826F3300 => {
    //   block [0x826F3300..0x826F3370)
	// 826F3300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F330C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3310: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3314: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F3318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F331C: 390BD058  addi r8, r11, -0x2fa8
	ctx.r[8].s64 = ctx.r[11].s64 + -12200;
	// 826F3320: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F3324: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826F3328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F332C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3330: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3338: 386A8AC4  addi r3, r10, -0x753c
	ctx.r[3].s64 = ctx.r[10].s64 + -30012;
	// 826F333C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F334C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F335C: 4BD73AC5  bl 0x82466e20
	ctx.lr = 0x826F3360;
	sub_82466E20(ctx, base);
	// 826F3360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F336C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3370 size=100
    let mut pc: u32 = 0x826F3370;
    'dispatch: loop {
        match pc {
            0x826F3370 => {
    //   block [0x826F3370..0x826F33D4)
	// 826F3370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F337C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3384: 38AA89D4  addi r5, r10, -0x762c
	ctx.r[5].s64 = ctx.r[10].s64 + -30252;
	// 826F3388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F338C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3390: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826F3394: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F339C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F33A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F33A4: 386A8AF4  addi r3, r10, -0x750c
	ctx.r[3].s64 = ctx.r[10].s64 + -29964;
	// 826F33A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F33AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F33B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F33B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F33B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F33BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F33C0: 4BD73A61  bl 0x82466e20
	ctx.lr = 0x826F33C4;
	sub_82466E20(ctx, base);
	// 826F33C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F33C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F33CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F33D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F33D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F33D8 size=96
    let mut pc: u32 = 0x826F33D8;
    'dispatch: loop {
        match pc {
            0x826F33D8 => {
    //   block [0x826F33D8..0x826F3438)
	// 826F33D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F33DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F33E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F33E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F33E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F33EC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 826F33F0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F33F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F33F8: 386A8B24  addi r3, r10, -0x74dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29916;
	// 826F33FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3404: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F3408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F340C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3418: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F341C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3420: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3424: 4BD739FD  bl 0x82466e20
	ctx.lr = 0x826F3428;
	sub_82466E20(ctx, base);
	// 826F3428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F342C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3438 size=112
    let mut pc: u32 = 0x826F3438;
    'dispatch: loop {
        match pc {
            0x826F3438 => {
    //   block [0x826F3438..0x826F34A8)
	// 826F3438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F343C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3444: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F3448: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F344C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F3450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3454: 390BD070  addi r8, r11, -0x2f90
	ctx.r[8].s64 = ctx.r[11].s64 + -12176;
	// 826F3458: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F345C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826F3460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3464: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F346C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3470: 386A8B54  addi r3, r10, -0x74ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29868;
	// 826F3474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F347C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F348C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3494: 4BD7398D  bl 0x82466e20
	ctx.lr = 0x826F3498;
	sub_82466E20(ctx, base);
	// 826F3498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F349C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F34A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F34A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F34A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F34A8 size=96
    let mut pc: u32 = 0x826F34A8;
    'dispatch: loop {
        match pc {
            0x826F34A8 => {
    //   block [0x826F34A8..0x826F3508)
	// 826F34A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F34AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F34B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F34B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F34B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F34BC: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 826F34C0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F34C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F34C8: 386A8B84  addi r3, r10, -0x747c
	ctx.r[3].s64 = ctx.r[10].s64 + -29820;
	// 826F34CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F34D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F34D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F34D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F34DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F34E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F34E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F34E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F34EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F34F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F34F4: 4BD7392D  bl 0x82466e20
	ctx.lr = 0x826F34F8;
	sub_82466E20(ctx, base);
	// 826F34F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F34FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3508 size=112
    let mut pc: u32 = 0x826F3508;
    'dispatch: loop {
        match pc {
            0x826F3508 => {
    //   block [0x826F3508..0x826F3578)
	// 826F3508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F350C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3514: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3518: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F351C: 38AA8B84  addi r5, r10, -0x747c
	ctx.r[5].s64 = ctx.r[10].s64 + -29820;
	// 826F3520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3524: 390BD0A0  addi r8, r11, -0x2f60
	ctx.r[8].s64 = ctx.r[11].s64 + -12128;
	// 826F3528: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F352C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826F3530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3534: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F353C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3540: 386A8BB4  addi r3, r10, -0x744c
	ctx.r[3].s64 = ctx.r[10].s64 + -29772;
	// 826F3544: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F354C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F355C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3564: 4BD738BD  bl 0x82466e20
	ctx.lr = 0x826F3568;
	sub_82466E20(ctx, base);
	// 826F3568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F356C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3578 size=108
    let mut pc: u32 = 0x826F3578;
    'dispatch: loop {
        match pc {
            0x826F3578 => {
    //   block [0x826F3578..0x826F35E4)
	// 826F3578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F357C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3584: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F358C: 38EBD0B8  addi r7, r11, -0x2f48
	ctx.r[7].s64 = ctx.r[11].s64 + -12104;
	// 826F3590: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F3594: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 826F3598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F359C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F35A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F35A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F35A8: 386A8BE4  addi r3, r10, -0x741c
	ctx.r[3].s64 = ctx.r[10].s64 + -29724;
	// 826F35AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F35B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F35B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F35B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F35BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F35C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F35C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F35C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F35CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F35D0: 4BD73851  bl 0x82466e20
	ctx.lr = 0x826F35D4;
	sub_82466E20(ctx, base);
	// 826F35D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F35D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F35DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F35E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F35E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F35E8 size=112
    let mut pc: u32 = 0x826F35E8;
    'dispatch: loop {
        match pc {
            0x826F35E8 => {
    //   block [0x826F35E8..0x826F3658)
	// 826F35E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F35EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F35F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F35F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F35F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F35FC: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F3600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3604: 390BD118  addi r8, r11, -0x2ee8
	ctx.r[8].s64 = ctx.r[11].s64 + -12008;
	// 826F3608: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F360C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826F3610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3614: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F361C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3620: 386A8C14  addi r3, r10, -0x73ec
	ctx.r[3].s64 = ctx.r[10].s64 + -29676;
	// 826F3624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F362C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F363C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3644: 4BD737DD  bl 0x82466e20
	ctx.lr = 0x826F3648;
	sub_82466E20(ctx, base);
	// 826F3648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F364C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3658 size=112
    let mut pc: u32 = 0x826F3658;
    'dispatch: loop {
        match pc {
            0x826F3658 => {
    //   block [0x826F3658..0x826F36C8)
	// 826F3658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F365C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3664: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3668: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F366C: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F3670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3674: 390BD130  addi r8, r11, -0x2ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -11984;
	// 826F3678: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F367C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826F3680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3684: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F368C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3690: 386A8C44  addi r3, r10, -0x73bc
	ctx.r[3].s64 = ctx.r[10].s64 + -29628;
	// 826F3694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F369C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F36A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F36A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F36A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F36AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F36B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F36B4: 4BD7376D  bl 0x82466e20
	ctx.lr = 0x826F36B8;
	sub_82466E20(ctx, base);
	// 826F36B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F36BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F36C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F36C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F36C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F36C8 size=100
    let mut pc: u32 = 0x826F36C8;
    'dispatch: loop {
        match pc {
            0x826F36C8 => {
    //   block [0x826F36C8..0x826F372C)
	// 826F36C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F36CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F36D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F36D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F36D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F36DC: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F36E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F36E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F36E8: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 826F36EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F36F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F36F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F36F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F36FC: 386A8C74  addi r3, r10, -0x738c
	ctx.r[3].s64 = ctx.r[10].s64 + -29580;
	// 826F3700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3704: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3708: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F370C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3710: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3718: 4BD73709  bl 0x82466e20
	ctx.lr = 0x826F371C;
	sub_82466E20(ctx, base);
	// 826F371C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3730 size=116
    let mut pc: u32 = 0x826F3730;
    'dispatch: loop {
        match pc {
            0x826F3730 => {
    //   block [0x826F3730..0x826F37A4)
	// 826F3730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F373C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3740: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F3744: 390BD160  addi r8, r11, -0x2ea0
	ctx.r[8].s64 = ctx.r[11].s64 + -11936;
	// 826F3748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F374C: 392A9F10  addi r9, r10, -0x60f0
	ctx.r[9].s64 = ctx.r[10].s64 + -24816;
	// 826F3750: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3754: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826F3758: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F375C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3764: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F376C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3774: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F3778: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826F377C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F3780: 386B8CA4  addi r3, r11, -0x735c
	ctx.r[3].s64 = ctx.r[11].s64 + -29532;
	// 826F3784: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F3788: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F378C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3790: 4BD73691  bl 0x82466e20
	ctx.lr = 0x826F3794;
	sub_82466E20(ctx, base);
	// 826F3794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F379C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F37A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F37A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F37A8 size=112
    let mut pc: u32 = 0x826F37A8;
    'dispatch: loop {
        match pc {
            0x826F37A8 => {
    //   block [0x826F37A8..0x826F3818)
	// 826F37A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F37AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F37B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F37B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F37B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F37BC: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F37C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F37C4: 390BD190  addi r8, r11, -0x2e70
	ctx.r[8].s64 = ctx.r[11].s64 + -11888;
	// 826F37C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F37CC: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 826F37D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F37D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F37D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F37DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F37E0: 386A8CD4  addi r3, r10, -0x732c
	ctx.r[3].s64 = ctx.r[10].s64 + -29484;
	// 826F37E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F37E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F37EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F37F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F37F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F37F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F37FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3804: 4BD7361D  bl 0x82466e20
	ctx.lr = 0x826F3808;
	sub_82466E20(ctx, base);
	// 826F3808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F380C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3818 size=116
    let mut pc: u32 = 0x826F3818;
    'dispatch: loop {
        match pc {
            0x826F3818 => {
    //   block [0x826F3818..0x826F388C)
	// 826F3818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F381C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3824: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3828: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F382C: 390BD1AC  addi r8, r11, -0x2e54
	ctx.r[8].s64 = ctx.r[11].s64 + -11860;
	// 826F3830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3834: 392A9F3C  addi r9, r10, -0x60c4
	ctx.r[9].s64 = ctx.r[10].s64 + -24772;
	// 826F3838: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F383C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F3840: 38AA9364  addi r5, r10, -0x6c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -27804;
	// 826F3844: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F384C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F385C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F3860: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826F3864: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F3868: 386B8D04  addi r3, r11, -0x72fc
	ctx.r[3].s64 = ctx.r[11].s64 + -29436;
	// 826F386C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F3870: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3878: 4BD735A9  bl 0x82466e20
	ctx.lr = 0x826F387C;
	sub_82466E20(ctx, base);
	// 826F387C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3890 size=112
    let mut pc: u32 = 0x826F3890;
    'dispatch: loop {
        match pc {
            0x826F3890 => {
    //   block [0x826F3890..0x826F3900)
	// 826F3890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F389C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F38A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F38A4: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F38A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F38AC: 390BD1C8  addi r8, r11, -0x2e38
	ctx.r[8].s64 = ctx.r[11].s64 + -11832;
	// 826F38B0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F38B4: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826F38B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F38BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F38C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F38C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F38C8: 386A8D34  addi r3, r10, -0x72cc
	ctx.r[3].s64 = ctx.r[10].s64 + -29388;
	// 826F38CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F38D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F38D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F38D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F38DC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F38E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F38E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F38E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F38EC: 4BD73535  bl 0x82466e20
	ctx.lr = 0x826F38F0;
	sub_82466E20(ctx, base);
	// 826F38F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F38F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F38F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F38FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3900 size=112
    let mut pc: u32 = 0x826F3900;
    'dispatch: loop {
        match pc {
            0x826F3900 => {
    //   block [0x826F3900..0x826F3970)
	// 826F3900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F390C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3910: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3914: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F3918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F391C: 390BD240  addi r8, r11, -0x2dc0
	ctx.r[8].s64 = ctx.r[11].s64 + -11712;
	// 826F3920: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F3924: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826F3928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F392C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3938: 386A8D64  addi r3, r10, -0x729c
	ctx.r[3].s64 = ctx.r[10].s64 + -29340;
	// 826F393C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F394C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F395C: 4BD734C5  bl 0x82466e20
	ctx.lr = 0x826F3960;
	sub_82466E20(ctx, base);
	// 826F3960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F396C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3970 size=112
    let mut pc: u32 = 0x826F3970;
    'dispatch: loop {
        match pc {
            0x826F3970 => {
    //   block [0x826F3970..0x826F39E0)
	// 826F3970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F397C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3980: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3984: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F3988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F398C: 390BD288  addi r8, r11, -0x2d78
	ctx.r[8].s64 = ctx.r[11].s64 + -11640;
	// 826F3990: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F3994: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826F3998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F399C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F39A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F39A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F39A8: 386A8D94  addi r3, r10, -0x726c
	ctx.r[3].s64 = ctx.r[10].s64 + -29292;
	// 826F39AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F39B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F39B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F39B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F39BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F39C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F39C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F39C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F39CC: 4BD73455  bl 0x82466e20
	ctx.lr = 0x826F39D0;
	sub_82466E20(ctx, base);
	// 826F39D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F39D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F39D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F39DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F39E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F39E0 size=112
    let mut pc: u32 = 0x826F39E0;
    'dispatch: loop {
        match pc {
            0x826F39E0 => {
    //   block [0x826F39E0..0x826F3A50)
	// 826F39E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F39E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F39E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F39EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F39F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F39F4: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F39F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F39FC: 390BD2D0  addi r8, r11, -0x2d30
	ctx.r[8].s64 = ctx.r[11].s64 + -11568;
	// 826F3A00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F3A04: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826F3A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3A0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3A18: 386A8DC4  addi r3, r10, -0x723c
	ctx.r[3].s64 = ctx.r[10].s64 + -29244;
	// 826F3A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3A3C: 4BD733E5  bl 0x82466e20
	ctx.lr = 0x826F3A40;
	sub_82466E20(ctx, base);
	// 826F3A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3A50 size=108
    let mut pc: u32 = 0x826F3A50;
    'dispatch: loop {
        match pc {
            0x826F3A50 => {
    //   block [0x826F3A50..0x826F3ABC)
	// 826F3A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3A5C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3A64: 38EBD318  addi r7, r11, -0x2ce8
	ctx.r[7].s64 = ctx.r[11].s64 + -11496;
	// 826F3A68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F3A6C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826F3A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3A74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F3A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3A80: 386A8DF4  addi r3, r10, -0x720c
	ctx.r[3].s64 = ctx.r[10].s64 + -29196;
	// 826F3A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F3A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3AA8: 4BD73379  bl 0x82466e20
	ctx.lr = 0x826F3AAC;
	sub_82466E20(ctx, base);
	// 826F3AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3AC0 size=112
    let mut pc: u32 = 0x826F3AC0;
    'dispatch: loop {
        match pc {
            0x826F3AC0 => {
    //   block [0x826F3AC0..0x826F3B30)
	// 826F3AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3ACC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3AD0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3AD4: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F3AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3ADC: 390BD360  addi r8, r11, -0x2ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -11424;
	// 826F3AE0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F3AE4: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826F3AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3AEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3AF8: 386A8E24  addi r3, r10, -0x71dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29148;
	// 826F3AFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3B1C: 4BD73305  bl 0x82466e20
	ctx.lr = 0x826F3B20;
	sub_82466E20(ctx, base);
	// 826F3B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3B30 size=116
    let mut pc: u32 = 0x826F3B30;
    'dispatch: loop {
        match pc {
            0x826F3B30 => {
    //   block [0x826F3B30..0x826F3BA4)
	// 826F3B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3B3C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F3B40: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3B44: 392B9F78  addi r9, r11, -0x6088
	ctx.r[9].s64 = ctx.r[11].s64 + -24712;
	// 826F3B48: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F3B4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3B50: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826F3B54: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826F3B58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3B5C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826F3B60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3B64: 396BD3D8  addi r11, r11, -0x2c28
	ctx.r[11].s64 = ctx.r[11].s64 + -11304;
	// 826F3B68: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F3B6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3B70: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F3B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3B78: 386A8E54  addi r3, r10, -0x71ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29100;
	// 826F3B7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F3B80: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F3B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3B88: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F3B8C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3B90: 4BD73291  bl 0x82466e20
	ctx.lr = 0x826F3B94;
	sub_82466E20(ctx, base);
	// 826F3B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F3BA8 size=36
    let mut pc: u32 = 0x826F3BA8;
    'dispatch: loop {
        match pc {
            0x826F3BA8 => {
    //   block [0x826F3BA8..0x826F3BCC)
	// 826F3BA8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3BAC: 814BD470  lwz r10, -0x2b90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11152 as u32) ) } as u64;
	// 826F3BB0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3BB4: 396B1BE0  addi r11, r11, 0x1be0
	ctx.r[11].s64 = ctx.r[11].s64 + 7136;
	// 826F3BB8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826F3BBC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F3BC0: 814AD468  lwz r10, -0x2b98(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-11160 as u32) ) } as u64;
	// 826F3BC4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826F3BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3BD0 size=108
    let mut pc: u32 = 0x826F3BD0;
    'dispatch: loop {
        match pc {
            0x826F3BD0 => {
    //   block [0x826F3BD0..0x826F3C3C)
	// 826F3BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3BDC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3BE4: 38EB1BE0  addi r7, r11, 0x1be0
	ctx.r[7].s64 = ctx.r[11].s64 + 7136;
	// 826F3BE8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826F3BEC: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 826F3BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3BF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F3BFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3C00: 386A8E84  addi r3, r10, -0x717c
	ctx.r[3].s64 = ctx.r[10].s64 + -29052;
	// 826F3C04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F3C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3C24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3C28: 4BD731F9  bl 0x82466e20
	ctx.lr = 0x826F3C2C;
	sub_82466E20(ctx, base);
	// 826F3C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F3C40 size=24
    let mut pc: u32 = 0x826F3C40;
    'dispatch: loop {
        match pc {
            0x826F3C40 => {
    //   block [0x826F3C40..0x826F3C58)
	// 826F3C40: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3C44: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F3C48: 394A1C88  addi r10, r10, 0x1c88
	ctx.r[10].s64 = ctx.r[10].s64 + 7304;
	// 826F3C4C: 816BD468  lwz r11, -0x2b98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11160 as u32) ) } as u64;
	// 826F3C50: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826F3C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3C58 size=116
    let mut pc: u32 = 0x826F3C58;
    'dispatch: loop {
        match pc {
            0x826F3C58 => {
    //   block [0x826F3C58..0x826F3CCC)
	// 826F3C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3C64: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F3C68: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826F3C6C: 390A1C88  addi r8, r10, 0x1c88
	ctx.r[8].s64 = ctx.r[10].s64 + 7304;
	// 826F3C70: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3C74: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F3C78: 38AA8E84  addi r5, r10, -0x717c
	ctx.r[5].s64 = ctx.r[10].s64 + -29052;
	// 826F3C7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3C80: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F3C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3C88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3C8C: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 826F3C90: 396BA034  addi r11, r11, -0x5fcc
	ctx.r[11].s64 = ctx.r[11].s64 + -24524;
	// 826F3C94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3C98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3C9C: 386A8EB4  addi r3, r10, -0x714c
	ctx.r[3].s64 = ctx.r[10].s64 + -29004;
	// 826F3CA0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F3CA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3CA8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F3CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3CB8: 4BD73169  bl 0x82466e20
	ctx.lr = 0x826F3CBC;
	sub_82466E20(ctx, base);
	// 826F3CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3CD0 size=112
    let mut pc: u32 = 0x826F3CD0;
    'dispatch: loop {
        match pc {
            0x826F3CD0 => {
    //   block [0x826F3CD0..0x826F3D40)
	// 826F3CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3CDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3CE0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3CE4: 38AA8E84  addi r5, r10, -0x717c
	ctx.r[5].s64 = ctx.r[10].s64 + -29052;
	// 826F3CE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3CEC: 390BD478  addi r8, r11, -0x2b88
	ctx.r[8].s64 = ctx.r[11].s64 + -11144;
	// 826F3CF0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F3CF4: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 826F3CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3CFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3D00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3D08: 386A8EE4  addi r3, r10, -0x711c
	ctx.r[3].s64 = ctx.r[10].s64 + -28956;
	// 826F3D0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3D2C: 4BD730F5  bl 0x82466e20
	ctx.lr = 0x826F3D30;
	sub_82466E20(ctx, base);
	// 826F3D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F3D40 size=24
    let mut pc: u32 = 0x826F3D40;
    'dispatch: loop {
        match pc {
            0x826F3D40 => {
    //   block [0x826F3D40..0x826F3D58)
	// 826F3D40: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3D44: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F3D48: 394A1D78  addi r10, r10, 0x1d78
	ctx.r[10].s64 = ctx.r[10].s64 + 7544;
	// 826F3D4C: 816BD5CC  lwz r11, -0x2a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10804 as u32) ) } as u64;
	// 826F3D50: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826F3D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3D58 size=116
    let mut pc: u32 = 0x826F3D58;
    'dispatch: loop {
        match pc {
            0x826F3D58 => {
    //   block [0x826F3D58..0x826F3DCC)
	// 826F3D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3D64: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F3D68: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3D6C: 392B9FF8  addi r9, r11, -0x6008
	ctx.r[9].s64 = ctx.r[11].s64 + -24584;
	// 826F3D70: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F3D74: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3D78: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826F3D7C: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 826F3D80: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3D84: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 826F3D88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3D8C: 396B1D78  addi r11, r11, 0x1d78
	ctx.r[11].s64 = ctx.r[11].s64 + 7544;
	// 826F3D90: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F3D94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3D98: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F3D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3DA0: 386A8F14  addi r3, r10, -0x70ec
	ctx.r[3].s64 = ctx.r[10].s64 + -28908;
	// 826F3DA4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826F3DA8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F3DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3DB0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F3DB4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3DB8: 4BD73069  bl 0x82466e20
	ctx.lr = 0x826F3DBC;
	sub_82466E20(ctx, base);
	// 826F3DBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3DC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3DC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3DD0 size=100
    let mut pc: u32 = 0x826F3DD0;
    'dispatch: loop {
        match pc {
            0x826F3DD0 => {
    //   block [0x826F3DD0..0x826F3E34)
	// 826F3DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3DD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3DDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3DE4: 38AA9064  addi r5, r10, -0x6f9c
	ctx.r[5].s64 = ctx.r[10].s64 + -28572;
	// 826F3DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3DF0: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826F3DF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3E04: 386A8F44  addi r3, r10, -0x70bc
	ctx.r[3].s64 = ctx.r[10].s64 + -28860;
	// 826F3E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3E0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3E10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F3E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3E18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3E1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3E20: 4BD73001  bl 0x82466e20
	ctx.lr = 0x826F3E24;
	sub_82466E20(ctx, base);
	// 826F3E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3E38 size=100
    let mut pc: u32 = 0x826F3E38;
    'dispatch: loop {
        match pc {
            0x826F3E38 => {
    //   block [0x826F3E38..0x826F3E9C)
	// 826F3E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3E44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3E4C: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F3E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3E58: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826F3E5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3E6C: 386A8F74  addi r3, r10, -0x708c
	ctx.r[3].s64 = ctx.r[10].s64 + -28812;
	// 826F3E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3E74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3E78: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F3E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3E80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F3E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3E88: 4BD72F99  bl 0x82466e20
	ctx.lr = 0x826F3E8C;
	sub_82466E20(ctx, base);
	// 826F3E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3EA0 size=108
    let mut pc: u32 = 0x826F3EA0;
    'dispatch: loop {
        match pc {
            0x826F3EA0 => {
    //   block [0x826F3EA0..0x826F3F0C)
	// 826F3EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3EAC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3EB4: 38EBD4F0  addi r7, r11, -0x2b10
	ctx.r[7].s64 = ctx.r[11].s64 + -11024;
	// 826F3EB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F3EBC: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826F3EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3EC4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F3ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3ED0: 386A8FA4  addi r3, r10, -0x705c
	ctx.r[3].s64 = ctx.r[10].s64 + -28764;
	// 826F3ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F3ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3EF8: 4BD72F29  bl 0x82466e20
	ctx.lr = 0x826F3EFC;
	sub_82466E20(ctx, base);
	// 826F3EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3F10 size=112
    let mut pc: u32 = 0x826F3F10;
    'dispatch: loop {
        match pc {
            0x826F3F10 => {
    //   block [0x826F3F10..0x826F3F80)
	// 826F3F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3F1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3F20: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3F24: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F3F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3F2C: 390BD550  addi r8, r11, -0x2ab0
	ctx.r[8].s64 = ctx.r[11].s64 + -10928;
	// 826F3F30: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F3F34: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826F3F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3F3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F3F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3F48: 386A8FD4  addi r3, r10, -0x702c
	ctx.r[3].s64 = ctx.r[10].s64 + -28716;
	// 826F3F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F3F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3F6C: 4BD72EB5  bl 0x82466e20
	ctx.lr = 0x826F3F70;
	sub_82466E20(ctx, base);
	// 826F3F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F3F80 size=108
    let mut pc: u32 = 0x826F3F80;
    'dispatch: loop {
        match pc {
            0x826F3F80 => {
    //   block [0x826F3F80..0x826F3FEC)
	// 826F3F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F3F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F3F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F3F8C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F3F94: 38EBD5B0  addi r7, r11, -0x2a50
	ctx.r[7].s64 = ctx.r[11].s64 + -10832;
	// 826F3F98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F3F9C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826F3FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F3FA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F3FA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F3FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F3FB0: 386A9004  addi r3, r10, -0x6ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -28668;
	// 826F3FB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F3FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F3FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F3FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F3FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F3FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F3FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F3FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F3FD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F3FD8: 4BD72E49  bl 0x82466e20
	ctx.lr = 0x826F3FDC;
	sub_82466E20(ctx, base);
	// 826F3FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F3FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F3FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F3FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F3FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F3FF0 size=28
    let mut pc: u32 = 0x826F3FF0;
    'dispatch: loop {
        match pc {
            0x826F3FF0 => {
    //   block [0x826F3FF0..0x826F400C)
	// 826F3FF0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F3FF4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F3FF8: 394A1E50  addi r10, r10, 0x1e50
	ctx.r[10].s64 = ctx.r[10].s64 + 7760;
	// 826F3FFC: 816BD474  lwz r11, -0x2b8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11148 as u32) ) } as u64;
	// 826F4000: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F4004: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826F4008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4010 size=112
    let mut pc: u32 = 0x826F4010;
    'dispatch: loop {
        match pc {
            0x826F4010 => {
    //   block [0x826F4010..0x826F4080)
	// 826F4010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F401C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4020: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 826F4024: 38EA1E50  addi r7, r10, 0x1e50
	ctx.r[7].s64 = ctx.r[10].s64 + 7760;
	// 826F4028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F402C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F4030: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826F4034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4038: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F403C: 396BA0F0  addi r11, r11, -0x5f10
	ctx.r[11].s64 = ctx.r[11].s64 + -24336;
	// 826F4040: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F4044: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4048: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F404C: 386A9034  addi r3, r10, -0x6fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -28620;
	// 826F4050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4054: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F4058: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F405C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F4060: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4064: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4068: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F406C: 4BD72DB5  bl 0x82466e20
	ctx.lr = 0x826F4070;
	sub_82466E20(ctx, base);
	// 826F4070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F407C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F4080 size=24
    let mut pc: u32 = 0x826F4080;
    'dispatch: loop {
        match pc {
            0x826F4080 => {
    //   block [0x826F4080..0x826F4098)
	// 826F4080: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4084: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4088: 394A1FB8  addi r10, r10, 0x1fb8
	ctx.r[10].s64 = ctx.r[10].s64 + 8120;
	// 826F408C: 816BD5CC  lwz r11, -0x2a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10804 as u32) ) } as u64;
	// 826F4090: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F4094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4098 size=116
    let mut pc: u32 = 0x826F4098;
    'dispatch: loop {
        match pc {
            0x826F4098 => {
    //   block [0x826F4098..0x826F410C)
	// 826F4098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F409C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F40A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F40A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F40A8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F40AC: 392BA0C8  addi r9, r11, -0x5f38
	ctx.r[9].s64 = ctx.r[11].s64 + -24376;
	// 826F40B0: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F40B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F40B8: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826F40BC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826F40C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F40C4: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826F40C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F40CC: 396B1FB8  addi r11, r11, 0x1fb8
	ctx.r[11].s64 = ctx.r[11].s64 + 8120;
	// 826F40D0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F40D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F40D8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F40DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F40E0: 386A9064  addi r3, r10, -0x6f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -28572;
	// 826F40E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826F40E8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F40EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F40F0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F40F4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F40F8: 4BD72D29  bl 0x82466e20
	ctx.lr = 0x826F40FC;
	sub_82466E20(ctx, base);
	// 826F40FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4110 size=112
    let mut pc: u32 = 0x826F4110;
    'dispatch: loop {
        match pc {
            0x826F4110 => {
    //   block [0x826F4110..0x826F4180)
	// 826F4110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F411C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4120: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4124: 38AA8C74  addi r5, r10, -0x738c
	ctx.r[5].s64 = ctx.r[10].s64 + -29580;
	// 826F4128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F412C: 390BD5D0  addi r8, r11, -0x2a30
	ctx.r[8].s64 = ctx.r[11].s64 + -10800;
	// 826F4130: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F4134: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826F4138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F413C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4148: 386A9094  addi r3, r10, -0x6f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -28524;
	// 826F414C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F415C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F416C: 4BD72CB5  bl 0x82466e20
	ctx.lr = 0x826F4170;
	sub_82466E20(ctx, base);
	// 826F4170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F417C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F4180 size=24
    let mut pc: u32 = 0x826F4180;
    'dispatch: loop {
        match pc {
            0x826F4180 => {
    //   block [0x826F4180..0x826F4198)
	// 826F4180: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4184: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4188: 394A2060  addi r10, r10, 0x2060
	ctx.r[10].s64 = ctx.r[10].s64 + 8288;
	// 826F418C: 816BD5CC  lwz r11, -0x2a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10804 as u32) ) } as u64;
	// 826F4190: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826F4194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4198 size=116
    let mut pc: u32 = 0x826F4198;
    'dispatch: loop {
        match pc {
            0x826F4198 => {
    //   block [0x826F4198..0x826F420C)
	// 826F4198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F419C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F41A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F41A4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F41A8: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826F41AC: 390A2060  addi r8, r10, 0x2060
	ctx.r[8].s64 = ctx.r[10].s64 + 8288;
	// 826F41B0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F41B4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F41B8: 38AA8C74  addi r5, r10, -0x738c
	ctx.r[5].s64 = ctx.r[10].s64 + -29580;
	// 826F41BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F41C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F41C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F41C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F41CC: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 826F41D0: 396BA150  addi r11, r11, -0x5eb0
	ctx.r[11].s64 = ctx.r[11].s64 + -24240;
	// 826F41D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F41D8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F41DC: 386A90C4  addi r3, r10, -0x6f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -28476;
	// 826F41E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F41E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F41E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F41EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F41F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F41F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F41F8: 4BD72C29  bl 0x82466e20
	ctx.lr = 0x826F41FC;
	sub_82466E20(ctx, base);
	// 826F41FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4210 size=112
    let mut pc: u32 = 0x826F4210;
    'dispatch: loop {
        match pc {
            0x826F4210 => {
    //   block [0x826F4210..0x826F4280)
	// 826F4210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F421C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F4220: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4224: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F4228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F422C: 390BD660  addi r8, r11, -0x29a0
	ctx.r[8].s64 = ctx.r[11].s64 + -10656;
	// 826F4230: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4234: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 826F4238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F423C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4240: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4248: 386A90F4  addi r3, r10, -0x6f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -28428;
	// 826F424C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F425C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F4260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F426C: 4BD72BB5  bl 0x82466e20
	ctx.lr = 0x826F4270;
	sub_82466E20(ctx, base);
	// 826F4270: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4274: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4278: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F427C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4280 size=108
    let mut pc: u32 = 0x826F4280;
    'dispatch: loop {
        match pc {
            0x826F4280 => {
    //   block [0x826F4280..0x826F42EC)
	// 826F4280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F428C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4294: 38EBD690  addi r7, r11, -0x2970
	ctx.r[7].s64 = ctx.r[11].s64 + -10608;
	// 826F4298: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F429C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826F42A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F42A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F42A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F42AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F42B0: 386A9124  addi r3, r10, -0x6edc
	ctx.r[3].s64 = ctx.r[10].s64 + -28380;
	// 826F42B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F42B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F42BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F42C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F42C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F42C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F42CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F42D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F42D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F42D8: 4BD72B49  bl 0x82466e20
	ctx.lr = 0x826F42DC;
	sub_82466E20(ctx, base);
	// 826F42DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F42E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F42E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F42E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F42F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F42F0 size=112
    let mut pc: u32 = 0x826F42F0;
    'dispatch: loop {
        match pc {
            0x826F42F0 => {
    //   block [0x826F42F0..0x826F4360)
	// 826F42F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F42F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F42F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F42FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4300: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4304: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F4308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F430C: 390BD6C0  addi r8, r11, -0x2940
	ctx.r[8].s64 = ctx.r[11].s64 + -10560;
	// 826F4310: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4314: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826F4318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F431C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4320: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4328: 386A9154  addi r3, r10, -0x6eac
	ctx.r[3].s64 = ctx.r[10].s64 + -28332;
	// 826F432C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F433C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F434C: 4BD72AD5  bl 0x82466e20
	ctx.lr = 0x826F4350;
	sub_82466E20(ctx, base);
	// 826F4350: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F435C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4360 size=112
    let mut pc: u32 = 0x826F4360;
    'dispatch: loop {
        match pc {
            0x826F4360 => {
    //   block [0x826F4360..0x826F43D0)
	// 826F4360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F436C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4370: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4374: 38AA9364  addi r5, r10, -0x6c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -27804;
	// 826F4378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F437C: 390BD6F0  addi r8, r11, -0x2910
	ctx.r[8].s64 = ctx.r[11].s64 + -10512;
	// 826F4380: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4384: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826F4388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F438C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4398: 386A9184  addi r3, r10, -0x6e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -28284;
	// 826F439C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F43A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F43A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F43A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F43AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F43B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F43B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F43B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F43BC: 4BD72A65  bl 0x82466e20
	ctx.lr = 0x826F43C0;
	sub_82466E20(ctx, base);
	// 826F43C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F43C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F43C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F43CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F43D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F43D0 size=108
    let mut pc: u32 = 0x826F43D0;
    'dispatch: loop {
        match pc {
            0x826F43D0 => {
    //   block [0x826F43D0..0x826F443C)
	// 826F43D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F43D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F43D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F43DC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F43E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F43E4: 38EBD720  addi r7, r11, -0x28e0
	ctx.r[7].s64 = ctx.r[11].s64 + -10464;
	// 826F43E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F43EC: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 826F43F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F43F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F43F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F43FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4400: 386A91B4  addi r3, r10, -0x6e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -28236;
	// 826F4404: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F4408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F440C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F441C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F4428: 4BD729F9  bl 0x82466e20
	ctx.lr = 0x826F442C;
	sub_82466E20(ctx, base);
	// 826F442C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4440 size=108
    let mut pc: u32 = 0x826F4440;
    'dispatch: loop {
        match pc {
            0x826F4440 => {
    //   block [0x826F4440..0x826F44AC)
	// 826F4440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F444C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F4454: 38EBD768  addi r7, r11, -0x2898
	ctx.r[7].s64 = ctx.r[11].s64 + -10392;
	// 826F4458: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F445C: 388A0DDC  addi r4, r10, 0xddc
	ctx.r[4].s64 = ctx.r[10].s64 + 3548;
	// 826F4460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4464: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4468: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F446C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4470: 386A91E4  addi r3, r10, -0x6e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -28188;
	// 826F4474: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F4478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F447C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F448C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4494: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F4498: 4BD72989  bl 0x82466e20
	ctx.lr = 0x826F449C;
	sub_82466E20(ctx, base);
	// 826F449C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F44A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F44A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F44A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F44B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F44B0 size=112
    let mut pc: u32 = 0x826F44B0;
    'dispatch: loop {
        match pc {
            0x826F44B0 => {
    //   block [0x826F44B0..0x826F4520)
	// 826F44B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F44B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F44B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F44BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F44C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F44C4: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F44C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F44CC: 390BD7C8  addi r8, r11, -0x2838
	ctx.r[8].s64 = ctx.r[11].s64 + -10296;
	// 826F44D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F44D4: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 826F44D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F44DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F44E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F44E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F44E8: 386A9214  addi r3, r10, -0x6dec
	ctx.r[3].s64 = ctx.r[10].s64 + -28140;
	// 826F44EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F44F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F44F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F44F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F44FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F450C: 4BD72915  bl 0x82466e20
	ctx.lr = 0x826F4510;
	sub_82466E20(ctx, base);
	// 826F4510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F451C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4520 size=100
    let mut pc: u32 = 0x826F4520;
    'dispatch: loop {
        match pc {
            0x826F4520 => {
    //   block [0x826F4520..0x826F4584)
	// 826F4520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F452C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4534: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F4538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F453C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4540: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826F4544: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F454C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4554: 386A9244  addi r3, r10, -0x6dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -28092;
	// 826F4558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F455C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4560: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F4564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4568: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F456C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4570: 4BD728B1  bl 0x82466e20
	ctx.lr = 0x826F4574;
	sub_82466E20(ctx, base);
	// 826F4574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F457C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4588 size=112
    let mut pc: u32 = 0x826F4588;
    'dispatch: loop {
        match pc {
            0x826F4588 => {
    //   block [0x826F4588..0x826F45F8)
	// 826F4588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F458C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4594: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4598: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F459C: 38AA8F74  addi r5, r10, -0x708c
	ctx.r[5].s64 = ctx.r[10].s64 + -28812;
	// 826F45A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F45A4: 390BD828  addi r8, r11, -0x27d8
	ctx.r[8].s64 = ctx.r[11].s64 + -10200;
	// 826F45A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F45AC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826F45B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F45B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F45B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F45BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F45C0: 386A9274  addi r3, r10, -0x6d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -28044;
	// 826F45C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F45C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F45CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F45D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F45D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F45D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F45DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F45E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F45E4: 4BD7283D  bl 0x82466e20
	ctx.lr = 0x826F45E8;
	sub_82466E20(ctx, base);
	// 826F45E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F45EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F45F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F45F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F45F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F45F8 size=112
    let mut pc: u32 = 0x826F45F8;
    'dispatch: loop {
        match pc {
            0x826F45F8 => {
    //   block [0x826F45F8..0x826F4668)
	// 826F45F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F45FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4604: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4608: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F460C: 38AA8F74  addi r5, r10, -0x708c
	ctx.r[5].s64 = ctx.r[10].s64 + -28812;
	// 826F4610: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4614: 390BD870  addi r8, r11, -0x2790
	ctx.r[8].s64 = ctx.r[11].s64 + -10128;
	// 826F4618: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826F461C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826F4620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4624: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F462C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4630: 386A92A4  addi r3, r10, -0x6d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -27996;
	// 826F4634: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4638: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F463C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F464C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4654: 4BD727CD  bl 0x82466e20
	ctx.lr = 0x826F4658;
	sub_82466E20(ctx, base);
	// 826F4658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F465C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4668 size=108
    let mut pc: u32 = 0x826F4668;
    'dispatch: loop {
        match pc {
            0x826F4668 => {
    //   block [0x826F4668..0x826F46D4)
	// 826F4668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F466C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4674: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F467C: 38EBD918  addi r7, r11, -0x26e8
	ctx.r[7].s64 = ctx.r[11].s64 + -9960;
	// 826F4680: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F4684: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826F4688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F468C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F4694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4698: 386A92D4  addi r3, r10, -0x6d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -27948;
	// 826F469C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F46A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F46A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F46A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F46AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F46B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F46B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F46B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F46BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F46C0: 4BD72761  bl 0x82466e20
	ctx.lr = 0x826F46C4;
	sub_82466E20(ctx, base);
	// 826F46C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F46C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F46CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F46D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F46D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F46D8 size=24
    let mut pc: u32 = 0x826F46D8;
    'dispatch: loop {
        match pc {
            0x826F46D8 => {
    //   block [0x826F46D8..0x826F46F0)
	// 826F46D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F46DC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F46E0: 394A2198  addi r10, r10, 0x2198
	ctx.r[10].s64 = ctx.r[10].s64 + 8600;
	// 826F46E4: 816BD5CC  lwz r11, -0x2a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10804 as u32) ) } as u64;
	// 826F46E8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F46EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F46F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F46F0 size=116
    let mut pc: u32 = 0x826F46F0;
    'dispatch: loop {
        match pc {
            0x826F46F0 => {
    //   block [0x826F46F0..0x826F4764)
	// 826F46F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F46F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F46F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F46FC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4700: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F4704: 390A2198  addi r8, r10, 0x2198
	ctx.r[8].s64 = ctx.r[10].s64 + 8600;
	// 826F4708: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F470C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F4710: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F4714: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4718: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F471C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4724: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826F4728: 396BA188  addi r11, r11, -0x5e78
	ctx.r[11].s64 = ctx.r[11].s64 + -24184;
	// 826F472C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4730: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4734: 386A9304  addi r3, r10, -0x6cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -27900;
	// 826F4738: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F473C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4740: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F4744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F474C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4750: 4BD726D1  bl 0x82466e20
	ctx.lr = 0x826F4754;
	sub_82466E20(ctx, base);
	// 826F4754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F475C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4768 size=100
    let mut pc: u32 = 0x826F4768;
    'dispatch: loop {
        match pc {
            0x826F4768 => {
    //   block [0x826F4768..0x826F47CC)
	// 826F4768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4774: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F477C: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F4780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4788: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826F478C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F479C: 386A9334  addi r3, r10, -0x6ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -27852;
	// 826F47A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F47A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F47A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F47AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F47B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F47B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F47B8: 4BD72669  bl 0x82466e20
	ctx.lr = 0x826F47BC;
	sub_82466E20(ctx, base);
	// 826F47BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F47C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F47C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F47C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F47D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F47D0 size=100
    let mut pc: u32 = 0x826F47D0;
    'dispatch: loop {
        match pc {
            0x826F47D0 => {
    //   block [0x826F47D0..0x826F4834)
	// 826F47D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F47D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F47D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F47DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F47E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F47E4: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F47E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F47EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F47F0: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826F47F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F47F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F47FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4804: 386A9364  addi r3, r10, -0x6c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -27804;
	// 826F4808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F480C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4810: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F4814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4818: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F481C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4820: 4BD72601  bl 0x82466e20
	ctx.lr = 0x826F4824;
	sub_82466E20(ctx, base);
	// 826F4824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F482C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4838 size=112
    let mut pc: u32 = 0x826F4838;
    'dispatch: loop {
        match pc {
            0x826F4838 => {
    //   block [0x826F4838..0x826F48A8)
	// 826F4838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F483C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4844: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F4848: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F484C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F4850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4854: 390BD978  addi r8, r11, -0x2688
	ctx.r[8].s64 = ctx.r[11].s64 + -9864;
	// 826F4858: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F485C: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 826F4860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4864: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F486C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4870: 386A9394  addi r3, r10, -0x6c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -27756;
	// 826F4874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F487C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F488C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4894: 4BD7258D  bl 0x82466e20
	ctx.lr = 0x826F4898;
	sub_82466E20(ctx, base);
	// 826F4898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F489C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F48A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F48A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F48A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F48A8 size=112
    let mut pc: u32 = 0x826F48A8;
    'dispatch: loop {
        match pc {
            0x826F48A8 => {
    //   block [0x826F48A8..0x826F4918)
	// 826F48A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F48AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F48B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F48B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F48B8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F48BC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F48C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F48C4: 390BDA08  addi r8, r11, -0x25f8
	ctx.r[8].s64 = ctx.r[11].s64 + -9720;
	// 826F48C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F48CC: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 826F48D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F48D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F48D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F48DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F48E0: 386A93C4  addi r3, r10, -0x6c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -27708;
	// 826F48E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F48E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F48EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F48F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F48F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F48F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F48FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4904: 4BD7251D  bl 0x82466e20
	ctx.lr = 0x826F4908;
	sub_82466E20(ctx, base);
	// 826F4908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F490C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4918 size=112
    let mut pc: u32 = 0x826F4918;
    'dispatch: loop {
        match pc {
            0x826F4918 => {
    //   block [0x826F4918..0x826F4988)
	// 826F4918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4924: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4928: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F492C: 38AA8F14  addi r5, r10, -0x70ec
	ctx.r[5].s64 = ctx.r[10].s64 + -28908;
	// 826F4930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4934: 390BDA68  addi r8, r11, -0x2598
	ctx.r[8].s64 = ctx.r[11].s64 + -9624;
	// 826F4938: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F493C: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 826F4940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4944: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F494C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4950: 386A93F4  addi r3, r10, -0x6c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -27660;
	// 826F4954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F495C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F496C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4974: 4BD724AD  bl 0x82466e20
	ctx.lr = 0x826F4978;
	sub_82466E20(ctx, base);
	// 826F4978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F497C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4988 size=112
    let mut pc: u32 = 0x826F4988;
    'dispatch: loop {
        match pc {
            0x826F4988 => {
    //   block [0x826F4988..0x826F49F8)
	// 826F4988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F498C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4994: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F4998: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F499C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F49A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F49A4: 390BDA98  addi r8, r11, -0x2568
	ctx.r[8].s64 = ctx.r[11].s64 + -9576;
	// 826F49A8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F49AC: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826F49B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F49B4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F49B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F49BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F49C0: 386A9424  addi r3, r10, -0x6bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -27612;
	// 826F49C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F49C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F49CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F49D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F49D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F49D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F49DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F49E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F49E4: 4BD7243D  bl 0x82466e20
	ctx.lr = 0x826F49E8;
	sub_82466E20(ctx, base);
	// 826F49E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F49EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F49F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F49F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F49F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F49F8 size=112
    let mut pc: u32 = 0x826F49F8;
    'dispatch: loop {
        match pc {
            0x826F49F8 => {
    //   block [0x826F49F8..0x826F4A68)
	// 826F49F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F49FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4A04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4A08: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4A0C: 38AA9064  addi r5, r10, -0x6f9c
	ctx.r[5].s64 = ctx.r[10].s64 + -28572;
	// 826F4A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4A14: 390BDB28  addi r8, r11, -0x24d8
	ctx.r[8].s64 = ctx.r[11].s64 + -9432;
	// 826F4A18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F4A1C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826F4A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4A24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4A30: 386A9454  addi r3, r10, -0x6bac
	ctx.r[3].s64 = ctx.r[10].s64 + -27564;
	// 826F4A34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4A54: 4BD723CD  bl 0x82466e20
	ctx.lr = 0x826F4A58;
	sub_82466E20(ctx, base);
	// 826F4A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4A68 size=112
    let mut pc: u32 = 0x826F4A68;
    'dispatch: loop {
        match pc {
            0x826F4A68 => {
    //   block [0x826F4A68..0x826F4AD8)
	// 826F4A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4A74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4A78: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4A7C: 38AA92A4  addi r5, r10, -0x6d5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27996;
	// 826F4A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4A84: 390BDB40  addi r8, r11, -0x24c0
	ctx.r[8].s64 = ctx.r[11].s64 + -9408;
	// 826F4A88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4A8C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826F4A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4A94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4A98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4AA0: 386A9484  addi r3, r10, -0x6b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -27516;
	// 826F4AA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4AB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4AC4: 4BD7235D  bl 0x82466e20
	ctx.lr = 0x826F4AC8;
	sub_82466E20(ctx, base);
	// 826F4AC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4ACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4AD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4AD8 size=112
    let mut pc: u32 = 0x826F4AD8;
    'dispatch: loop {
        match pc {
            0x826F4AD8 => {
    //   block [0x826F4AD8..0x826F4B48)
	// 826F4AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4AE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4AE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4AEC: 38AA8B54  addi r5, r10, -0x74ac
	ctx.r[5].s64 = ctx.r[10].s64 + -29868;
	// 826F4AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4AF4: 390BDB70  addi r8, r11, -0x2490
	ctx.r[8].s64 = ctx.r[11].s64 + -9360;
	// 826F4AF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F4AFC: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826F4B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4B04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4B08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4B10: 386A94B4  addi r3, r10, -0x6b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -27468;
	// 826F4B14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4B34: 4BD722ED  bl 0x82466e20
	ctx.lr = 0x826F4B38;
	sub_82466E20(ctx, base);
	// 826F4B38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F4B48 size=24
    let mut pc: u32 = 0x826F4B48;
    'dispatch: loop {
        match pc {
            0x826F4B48 => {
    //   block [0x826F4B48..0x826F4B60)
	// 826F4B48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4B4C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4B50: 394A2210  addi r10, r10, 0x2210
	ctx.r[10].s64 = ctx.r[10].s64 + 8720;
	// 826F4B54: 816BD5CC  lwz r11, -0x2a34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10804 as u32) ) } as u64;
	// 826F4B58: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F4B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4B60 size=116
    let mut pc: u32 = 0x826F4B60;
    'dispatch: loop {
        match pc {
            0x826F4B60 => {
    //   block [0x826F4B60..0x826F4BD4)
	// 826F4B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4B6C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4B70: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F4B74: 390A2210  addi r8, r10, 0x2210
	ctx.r[8].s64 = ctx.r[10].s64 + 8720;
	// 826F4B78: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4B7C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F4B80: 38AA8D04  addi r5, r10, -0x72fc
	ctx.r[5].s64 = ctx.r[10].s64 + -29436;
	// 826F4B84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4B88: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F4B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4B90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4B94: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826F4B98: 396BA1A0  addi r11, r11, -0x5e60
	ctx.r[11].s64 = ctx.r[11].s64 + -24160;
	// 826F4B9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4BA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4BA4: 386A94E4  addi r3, r10, -0x6b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -27420;
	// 826F4BA8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F4BAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4BB0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F4BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4BB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4BC0: 4BD72261  bl 0x82466e20
	ctx.lr = 0x826F4BC4;
	sub_82466E20(ctx, base);
	// 826F4BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4BD8 size=112
    let mut pc: u32 = 0x826F4BD8;
    'dispatch: loop {
        match pc {
            0x826F4BD8 => {
    //   block [0x826F4BD8..0x826F4C48)
	// 826F4BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4BE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4BE8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4BEC: 38AA8C74  addi r5, r10, -0x738c
	ctx.r[5].s64 = ctx.r[10].s64 + -29580;
	// 826F4BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4BF4: 390BDBB8  addi r8, r11, -0x2448
	ctx.r[8].s64 = ctx.r[11].s64 + -9288;
	// 826F4BF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4BFC: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826F4C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4C04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4C10: 386A9514  addi r3, r10, -0x6aec
	ctx.r[3].s64 = ctx.r[10].s64 + -27372;
	// 826F4C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4C34: 4BD721ED  bl 0x82466e20
	ctx.lr = 0x826F4C38;
	sub_82466E20(ctx, base);
	// 826F4C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4C48 size=112
    let mut pc: u32 = 0x826F4C48;
    'dispatch: loop {
        match pc {
            0x826F4C48 => {
    //   block [0x826F4C48..0x826F4CB8)
	// 826F4C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4C54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4C58: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4C5C: 38AA8CD4  addi r5, r10, -0x732c
	ctx.r[5].s64 = ctx.r[10].s64 + -29484;
	// 826F4C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4C64: 390BDBE8  addi r8, r11, -0x2418
	ctx.r[8].s64 = ctx.r[11].s64 + -9240;
	// 826F4C68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4C6C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826F4C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4C74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4C80: 386A9544  addi r3, r10, -0x6abc
	ctx.r[3].s64 = ctx.r[10].s64 + -27324;
	// 826F4C84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4CA4: 4BD7217D  bl 0x82466e20
	ctx.lr = 0x826F4CA8;
	sub_82466E20(ctx, base);
	// 826F4CA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4CB8 size=100
    let mut pc: u32 = 0x826F4CB8;
    'dispatch: loop {
        match pc {
            0x826F4CB8 => {
    //   block [0x826F4CB8..0x826F4D1C)
	// 826F4CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4CC4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F4CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4CCC: 392AA210  addi r9, r10, -0x5df0
	ctx.r[9].s64 = ctx.r[10].s64 + -24048;
	// 826F4CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4CD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4CD8: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 826F4CDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4CE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4CEC: 386A9574  addi r3, r10, -0x6a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -27276;
	// 826F4CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4CF4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826F4CF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F4CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4D00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F4D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F4D08: 4BD72119  bl 0x82466e20
	ctx.lr = 0x826F4D0C;
	sub_82466E20(ctx, base);
	// 826F4D0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4D10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4D14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F4D20 size=24
    let mut pc: u32 = 0x826F4D20;
    'dispatch: loop {
        match pc {
            0x826F4D20 => {
    //   block [0x826F4D20..0x826F4D38)
	// 826F4D20: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4D24: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F4D28: 394A22B8  addi r10, r10, 0x22b8
	ctx.r[10].s64 = ctx.r[10].s64 + 8888;
	// 826F4D2C: 816BDC20  lwz r11, -0x23e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9184 as u32) ) } as u64;
	// 826F4D30: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F4D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4D38 size=112
    let mut pc: u32 = 0x826F4D38;
    'dispatch: loop {
        match pc {
            0x826F4D38 => {
    //   block [0x826F4D38..0x826F4DA8)
	// 826F4D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4D44: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F4D48: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4D4C: 392AA350  addi r9, r10, -0x5cb0
	ctx.r[9].s64 = ctx.r[10].s64 + -23728;
	// 826F4D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4D54: 390B22B8  addi r8, r11, 0x22b8
	ctx.r[8].s64 = ctx.r[11].s64 + 8888;
	// 826F4D58: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F4D5C: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826F4D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4D64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4D70: 386A95A4  addi r3, r10, -0x6a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -27228;
	// 826F4D74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F4D78: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826F4D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F4D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4D94: 4BD7208D  bl 0x82466e20
	ctx.lr = 0x826F4D98;
	sub_82466E20(ctx, base);
	// 826F4D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4DA8 size=112
    let mut pc: u32 = 0x826F4DA8;
    'dispatch: loop {
        match pc {
            0x826F4DA8 => {
    //   block [0x826F4DA8..0x826F4E18)
	// 826F4DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4DB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4DB8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4DBC: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F4DC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4DC4: 390BDC28  addi r8, r11, -0x23d8
	ctx.r[8].s64 = ctx.r[11].s64 + -9176;
	// 826F4DC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4DCC: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826F4DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4DD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4DE0: 386A95D4  addi r3, r10, -0x6a2c
	ctx.r[3].s64 = ctx.r[10].s64 + -27180;
	// 826F4DE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4E04: 4BD7201D  bl 0x82466e20
	ctx.lr = 0x826F4E08;
	sub_82466E20(ctx, base);
	// 826F4E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4E18 size=108
    let mut pc: u32 = 0x826F4E18;
    'dispatch: loop {
        match pc {
            0x826F4E18 => {
    //   block [0x826F4E18..0x826F4E84)
	// 826F4E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4E24: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4E28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4E2C: 38EBDC58  addi r7, r11, -0x23a8
	ctx.r[7].s64 = ctx.r[11].s64 + -9128;
	// 826F4E30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F4E34: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826F4E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4E3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4E40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F4E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4E48: 386A9604  addi r3, r10, -0x69fc
	ctx.r[3].s64 = ctx.r[10].s64 + -27132;
	// 826F4E4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F4E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4E6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F4E70: 4BD71FB1  bl 0x82466e20
	ctx.lr = 0x826F4E74;
	sub_82466E20(ctx, base);
	// 826F4E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4E88 size=112
    let mut pc: u32 = 0x826F4E88;
    'dispatch: loop {
        match pc {
            0x826F4E88 => {
    //   block [0x826F4E88..0x826F4EF8)
	// 826F4E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4E94: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4E98: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4E9C: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F4EA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F4EA4: 390BDC70  addi r8, r11, -0x2390
	ctx.r[8].s64 = ctx.r[11].s64 + -9104;
	// 826F4EA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F4EAC: 388A0E04  addi r4, r10, 0xe04
	ctx.r[4].s64 = ctx.r[10].s64 + 3588;
	// 826F4EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4EB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4EC0: 386A9634  addi r3, r10, -0x69cc
	ctx.r[3].s64 = ctx.r[10].s64 + -27084;
	// 826F4EC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4EE4: 4BD71F3D  bl 0x82466e20
	ctx.lr = 0x826F4EE8;
	sub_82466E20(ctx, base);
	// 826F4EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4EF8 size=100
    let mut pc: u32 = 0x826F4EF8;
    'dispatch: loop {
        match pc {
            0x826F4EF8 => {
    //   block [0x826F4EF8..0x826F4F5C)
	// 826F4EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4F04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4F0C: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F4F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4F18: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826F4F1C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4F20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4F28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4F2C: 386A9664  addi r3, r10, -0x699c
	ctx.r[3].s64 = ctx.r[10].s64 + -27036;
	// 826F4F30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4F34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4F38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F4F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4F40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F4F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4F48: 4BD71ED9  bl 0x82466e20
	ctx.lr = 0x826F4F4C;
	sub_82466E20(ctx, base);
	// 826F4F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4F60 size=112
    let mut pc: u32 = 0x826F4F60;
    'dispatch: loop {
        match pc {
            0x826F4F60 => {
    //   block [0x826F4F60..0x826F4FD0)
	// 826F4F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4F6C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4F70: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4F74: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F4F78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4F7C: 390BDCD0  addi r8, r11, -0x2330
	ctx.r[8].s64 = ctx.r[11].s64 + -9008;
	// 826F4F80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F4F84: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 826F4F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4F8C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4F90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F4F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F4F98: 386A9694  addi r3, r10, -0x696c
	ctx.r[3].s64 = ctx.r[10].s64 + -26988;
	// 826F4F9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F4FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F4FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F4FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F4FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F4FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F4FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F4FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F4FBC: 4BD71E65  bl 0x82466e20
	ctx.lr = 0x826F4FC0;
	sub_82466E20(ctx, base);
	// 826F4FC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F4FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F4FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F4FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F4FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F4FD0 size=112
    let mut pc: u32 = 0x826F4FD0;
    'dispatch: loop {
        match pc {
            0x826F4FD0 => {
    //   block [0x826F4FD0..0x826F5040)
	// 826F4FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F4FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F4FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F4FDC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F4FE0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F4FE4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F4FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F4FEC: 390BDCE8  addi r8, r11, -0x2318
	ctx.r[8].s64 = ctx.r[11].s64 + -8984;
	// 826F4FF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F4FF4: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 826F4FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F4FFC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5000: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5008: 386A96C4  addi r3, r10, -0x693c
	ctx.r[3].s64 = ctx.r[10].s64 + -26940;
	// 826F500C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F501C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F502C: 4BD71DF5  bl 0x82466e20
	ctx.lr = 0x826F5030;
	sub_82466E20(ctx, base);
	// 826F5030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F503C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5040 size=112
    let mut pc: u32 = 0x826F5040;
    'dispatch: loop {
        match pc {
            0x826F5040 => {
    //   block [0x826F5040..0x826F50B0)
	// 826F5040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F504C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5050: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5054: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F505C: 390BDD18  addi r8, r11, -0x22e8
	ctx.r[8].s64 = ctx.r[11].s64 + -8936;
	// 826F5060: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F5064: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 826F5068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F506C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5078: 386A96F4  addi r3, r10, -0x690c
	ctx.r[3].s64 = ctx.r[10].s64 + -26892;
	// 826F507C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F508C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F509C: 4BD71D85  bl 0x82466e20
	ctx.lr = 0x826F50A0;
	sub_82466E20(ctx, base);
	// 826F50A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F50A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F50A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F50AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F50B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F50B0 size=112
    let mut pc: u32 = 0x826F50B0;
    'dispatch: loop {
        match pc {
            0x826F50B0 => {
    //   block [0x826F50B0..0x826F5120)
	// 826F50B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F50B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F50B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F50BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F50C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F50C4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F50C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F50CC: 390BDD48  addi r8, r11, -0x22b8
	ctx.r[8].s64 = ctx.r[11].s64 + -8888;
	// 826F50D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F50D4: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 826F50D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F50DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F50E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F50E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F50E8: 386A9724  addi r3, r10, -0x68dc
	ctx.r[3].s64 = ctx.r[10].s64 + -26844;
	// 826F50EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F50F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F50F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F50F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F50FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F510C: 4BD71D15  bl 0x82466e20
	ctx.lr = 0x826F5110;
	sub_82466E20(ctx, base);
	// 826F5110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F511C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5120 size=112
    let mut pc: u32 = 0x826F5120;
    'dispatch: loop {
        match pc {
            0x826F5120 => {
    //   block [0x826F5120..0x826F5190)
	// 826F5120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F512C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5130: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5134: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F513C: 390BDD78  addi r8, r11, -0x2288
	ctx.r[8].s64 = ctx.r[11].s64 + -8840;
	// 826F5140: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F5144: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 826F5148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F514C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5158: 386A9754  addi r3, r10, -0x68ac
	ctx.r[3].s64 = ctx.r[10].s64 + -26796;
	// 826F515C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F516C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F517C: 4BD71CA5  bl 0x82466e20
	ctx.lr = 0x826F5180;
	sub_82466E20(ctx, base);
	// 826F5180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F518C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5190 size=112
    let mut pc: u32 = 0x826F5190;
    'dispatch: loop {
        match pc {
            0x826F5190 => {
    //   block [0x826F5190..0x826F5200)
	// 826F5190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F519C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F51A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F51A4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F51A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F51AC: 390BDD90  addi r8, r11, -0x2270
	ctx.r[8].s64 = ctx.r[11].s64 + -8816;
	// 826F51B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F51B4: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 826F51B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F51BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F51C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F51C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F51C8: 386A9784  addi r3, r10, -0x687c
	ctx.r[3].s64 = ctx.r[10].s64 + -26748;
	// 826F51CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F51D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F51D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F51D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F51DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F51E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F51E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F51E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F51EC: 4BD71C35  bl 0x82466e20
	ctx.lr = 0x826F51F0;
	sub_82466E20(ctx, base);
	// 826F51F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F51F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F51F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F51FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5200 size=112
    let mut pc: u32 = 0x826F5200;
    'dispatch: loop {
        match pc {
            0x826F5200 => {
    //   block [0x826F5200..0x826F5270)
	// 826F5200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F520C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5210: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5214: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F521C: 390BDDA8  addi r8, r11, -0x2258
	ctx.r[8].s64 = ctx.r[11].s64 + -8792;
	// 826F5220: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F5224: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826F5228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F522C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5238: 386A97B4  addi r3, r10, -0x684c
	ctx.r[3].s64 = ctx.r[10].s64 + -26700;
	// 826F523C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F524C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F525C: 4BD71BC5  bl 0x82466e20
	ctx.lr = 0x826F5260;
	sub_82466E20(ctx, base);
	// 826F5260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F526C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5270 size=112
    let mut pc: u32 = 0x826F5270;
    'dispatch: loop {
        match pc {
            0x826F5270 => {
    //   block [0x826F5270..0x826F52E0)
	// 826F5270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F527C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5280: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5284: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F528C: 390BDDF0  addi r8, r11, -0x2210
	ctx.r[8].s64 = ctx.r[11].s64 + -8720;
	// 826F5290: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F5294: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 826F5298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F529C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F52A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F52A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F52A8: 386A97E4  addi r3, r10, -0x681c
	ctx.r[3].s64 = ctx.r[10].s64 + -26652;
	// 826F52AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F52B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F52B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F52B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F52BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F52C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F52C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F52C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F52CC: 4BD71B55  bl 0x82466e20
	ctx.lr = 0x826F52D0;
	sub_82466E20(ctx, base);
	// 826F52D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F52D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F52D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F52DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F52E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F52E0 size=112
    let mut pc: u32 = 0x826F52E0;
    'dispatch: loop {
        match pc {
            0x826F52E0 => {
    //   block [0x826F52E0..0x826F5350)
	// 826F52E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F52E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F52E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F52EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F52F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F52F4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F52F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F52FC: 390BDE38  addi r8, r11, -0x21c8
	ctx.r[8].s64 = ctx.r[11].s64 + -8648;
	// 826F5300: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F5304: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826F5308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F530C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5318: 386A9814  addi r3, r10, -0x67ec
	ctx.r[3].s64 = ctx.r[10].s64 + -26604;
	// 826F531C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F532C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F533C: 4BD71AE5  bl 0x82466e20
	ctx.lr = 0x826F5340;
	sub_82466E20(ctx, base);
	// 826F5340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F534C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5350 size=112
    let mut pc: u32 = 0x826F5350;
    'dispatch: loop {
        match pc {
            0x826F5350 => {
    //   block [0x826F5350..0x826F53C0)
	// 826F5350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F535C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5360: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5364: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F536C: 390BDE50  addi r8, r11, -0x21b0
	ctx.r[8].s64 = ctx.r[11].s64 + -8624;
	// 826F5370: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F5374: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 826F5378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F537C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5388: 386A9844  addi r3, r10, -0x67bc
	ctx.r[3].s64 = ctx.r[10].s64 + -26556;
	// 826F538C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F539C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F53A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F53A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F53A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F53AC: 4BD71A75  bl 0x82466e20
	ctx.lr = 0x826F53B0;
	sub_82466E20(ctx, base);
	// 826F53B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F53B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F53B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F53BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F53C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F53C0 size=116
    let mut pc: u32 = 0x826F53C0;
    'dispatch: loop {
        match pc {
            0x826F53C0 => {
    //   block [0x826F53C0..0x826F5434)
	// 826F53C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F53C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F53C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F53CC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F53D0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F53D4: 390ADE80  addi r8, r10, -0x2180
	ctx.r[8].s64 = ctx.r[10].s64 + -8576;
	// 826F53D8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F53DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F53E0: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F53E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F53E8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F53EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F53F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F53F4: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 826F53F8: 396BA378  addi r11, r11, -0x5c88
	ctx.r[11].s64 = ctx.r[11].s64 + -23688;
	// 826F53FC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5400: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5404: 386A9874  addi r3, r10, -0x678c
	ctx.r[3].s64 = ctx.r[10].s64 + -26508;
	// 826F5408: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F540C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5410: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F5414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F541C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5420: 4BD71A01  bl 0x82466e20
	ctx.lr = 0x826F5424;
	sub_82466E20(ctx, base);
	// 826F5424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F542C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5438 size=116
    let mut pc: u32 = 0x826F5438;
    'dispatch: loop {
        match pc {
            0x826F5438 => {
    //   block [0x826F5438..0x826F54AC)
	// 826F5438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F543C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5444: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F5448: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826F544C: 390ADEF8  addi r8, r10, -0x2108
	ctx.r[8].s64 = ctx.r[10].s64 + -8456;
	// 826F5450: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5454: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F5458: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F545C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5460: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F5464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F546C: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 826F5470: 396BA390  addi r11, r11, -0x5c70
	ctx.r[11].s64 = ctx.r[11].s64 + -23664;
	// 826F5474: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5478: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F547C: 386A98A4  addi r3, r10, -0x675c
	ctx.r[3].s64 = ctx.r[10].s64 + -26460;
	// 826F5480: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F5484: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5488: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F548C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5498: 4BD71989  bl 0x82466e20
	ctx.lr = 0x826F549C;
	sub_82466E20(ctx, base);
	// 826F549C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F54A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F54A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F54A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F54B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F54B0 size=24
    let mut pc: u32 = 0x826F54B0;
    'dispatch: loop {
        match pc {
            0x826F54B0 => {
    //   block [0x826F54B0..0x826F54C8)
	// 826F54B0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F54B4: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F54B8: 394A22D0  addi r10, r10, 0x22d0
	ctx.r[10].s64 = ctx.r[10].s64 + 8912;
	// 826F54BC: 816BDF88  lwz r11, -0x2078(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8312 as u32) ) } as u64;
	// 826F54C0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826F54C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F54C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F54C8 size=116
    let mut pc: u32 = 0x826F54C8;
    'dispatch: loop {
        match pc {
            0x826F54C8 => {
    //   block [0x826F54C8..0x826F553C)
	// 826F54C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F54CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F54D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F54D4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F54D8: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F54DC: 392BA3BC  addi r9, r11, -0x5c44
	ctx.r[9].s64 = ctx.r[11].s64 + -23620;
	// 826F54E0: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F54E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F54E8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826F54EC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826F54F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F54F4: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 826F54F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F54FC: 396B22D0  addi r11, r11, 0x22d0
	ctx.r[11].s64 = ctx.r[11].s64 + 8912;
	// 826F5500: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F5504: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5508: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F550C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5510: 386A98D4  addi r3, r10, -0x672c
	ctx.r[3].s64 = ctx.r[10].s64 + -26412;
	// 826F5514: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F5518: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F551C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5520: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F5524: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F5528: 4BD718F9  bl 0x82466e20
	ctx.lr = 0x826F552C;
	sub_82466E20(ctx, base);
	// 826F552C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5540 size=112
    let mut pc: u32 = 0x826F5540;
    'dispatch: loop {
        match pc {
            0x826F5540 => {
    //   block [0x826F5540..0x826F55B0)
	// 826F5540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F554C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5550: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5554: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F555C: 390BDF90  addi r8, r11, -0x2070
	ctx.r[8].s64 = ctx.r[11].s64 + -8304;
	// 826F5560: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F5564: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826F5568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F556C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5578: 386A9904  addi r3, r10, -0x66fc
	ctx.r[3].s64 = ctx.r[10].s64 + -26364;
	// 826F557C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F558C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F559C: 4BD71885  bl 0x82466e20
	ctx.lr = 0x826F55A0;
	sub_82466E20(ctx, base);
	// 826F55A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F55A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F55A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F55AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F55B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F55B0 size=112
    let mut pc: u32 = 0x826F55B0;
    'dispatch: loop {
        match pc {
            0x826F55B0 => {
    //   block [0x826F55B0..0x826F5620)
	// 826F55B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F55B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F55B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F55BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F55C0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F55C4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F55C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F55CC: 390BDFF0  addi r8, r11, -0x2010
	ctx.r[8].s64 = ctx.r[11].s64 + -8208;
	// 826F55D0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826F55D4: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 826F55D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F55DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F55E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F55E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F55E8: 386A9934  addi r3, r10, -0x66cc
	ctx.r[3].s64 = ctx.r[10].s64 + -26316;
	// 826F55EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F55F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F55F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F55F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F55FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F560C: 4BD71815  bl 0x82466e20
	ctx.lr = 0x826F5610;
	sub_82466E20(ctx, base);
	// 826F5610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F561C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5620 size=112
    let mut pc: u32 = 0x826F5620;
    'dispatch: loop {
        match pc {
            0x826F5620 => {
    //   block [0x826F5620..0x826F5690)
	// 826F5620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F562C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5630: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5634: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F563C: 390BE098  addi r8, r11, -0x1f68
	ctx.r[8].s64 = ctx.r[11].s64 + -8040;
	// 826F5640: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F5644: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826F5648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F564C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5658: 386A9964  addi r3, r10, -0x669c
	ctx.r[3].s64 = ctx.r[10].s64 + -26268;
	// 826F565C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F566C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F567C: 4BD717A5  bl 0x82466e20
	ctx.lr = 0x826F5680;
	sub_82466E20(ctx, base);
	// 826F5680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F568C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5690 size=112
    let mut pc: u32 = 0x826F5690;
    'dispatch: loop {
        match pc {
            0x826F5690 => {
    //   block [0x826F5690..0x826F5700)
	// 826F5690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F569C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F56A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F56A4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F56A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F56AC: 390BE110  addi r8, r11, -0x1ef0
	ctx.r[8].s64 = ctx.r[11].s64 + -7920;
	// 826F56B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F56B4: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 826F56B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F56BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F56C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F56C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F56C8: 386A9994  addi r3, r10, -0x666c
	ctx.r[3].s64 = ctx.r[10].s64 + -26220;
	// 826F56CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F56D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F56D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F56D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F56DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F56E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F56E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F56E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F56EC: 4BD71735  bl 0x82466e20
	ctx.lr = 0x826F56F0;
	sub_82466E20(ctx, base);
	// 826F56F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F56F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F56F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F56FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5700 size=112
    let mut pc: u32 = 0x826F5700;
    'dispatch: loop {
        match pc {
            0x826F5700 => {
    //   block [0x826F5700..0x826F5770)
	// 826F5700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F570C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5710: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5714: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F571C: 390BE158  addi r8, r11, -0x1ea8
	ctx.r[8].s64 = ctx.r[11].s64 + -7848;
	// 826F5720: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F5724: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826F5728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F572C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5738: 386A99C4  addi r3, r10, -0x663c
	ctx.r[3].s64 = ctx.r[10].s64 + -26172;
	// 826F573C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F574C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F575C: 4BD716C5  bl 0x82466e20
	ctx.lr = 0x826F5760;
	sub_82466E20(ctx, base);
	// 826F5760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F576C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5770 size=112
    let mut pc: u32 = 0x826F5770;
    'dispatch: loop {
        match pc {
            0x826F5770 => {
    //   block [0x826F5770..0x826F57E0)
	// 826F5770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F577C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5780: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5784: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F5788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F578C: 390BE1E8  addi r8, r11, -0x1e18
	ctx.r[8].s64 = ctx.r[11].s64 + -7704;
	// 826F5790: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F5794: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 826F5798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F579C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F57A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F57A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F57A8: 386A99F4  addi r3, r10, -0x660c
	ctx.r[3].s64 = ctx.r[10].s64 + -26124;
	// 826F57AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F57B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F57B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F57B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F57BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F57C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F57C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F57C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F57CC: 4BD71655  bl 0x82466e20
	ctx.lr = 0x826F57D0;
	sub_82466E20(ctx, base);
	// 826F57D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F57D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F57D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F57DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F57E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F57E0 size=112
    let mut pc: u32 = 0x826F57E0;
    'dispatch: loop {
        match pc {
            0x826F57E0 => {
    //   block [0x826F57E0..0x826F5850)
	// 826F57E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F57E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F57E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F57EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F57F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F57F4: 38AA95A4  addi r5, r10, -0x6a5c
	ctx.r[5].s64 = ctx.r[10].s64 + -27228;
	// 826F57F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F57FC: 390BE248  addi r8, r11, -0x1db8
	ctx.r[8].s64 = ctx.r[11].s64 + -7608;
	// 826F5800: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F5804: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 826F5808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F580C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5818: 386A9A24  addi r3, r10, -0x65dc
	ctx.r[3].s64 = ctx.r[10].s64 + -26076;
	// 826F581C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F582C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F583C: 4BD715E5  bl 0x82466e20
	ctx.lr = 0x826F5840;
	sub_82466E20(ctx, base);
	// 826F5840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F584C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5850 size=112
    let mut pc: u32 = 0x826F5850;
    'dispatch: loop {
        match pc {
            0x826F5850 => {
    //   block [0x826F5850..0x826F58C0)
	// 826F5850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F585C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5860: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5864: 38AA9A24  addi r5, r10, -0x65dc
	ctx.r[5].s64 = ctx.r[10].s64 + -26076;
	// 826F5868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F586C: 390BE2A8  addi r8, r11, -0x1d58
	ctx.r[8].s64 = ctx.r[11].s64 + -7512;
	// 826F5870: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F5874: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826F5878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F587C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5888: 386A9A54  addi r3, r10, -0x65ac
	ctx.r[3].s64 = ctx.r[10].s64 + -26028;
	// 826F588C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F589C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F58A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F58A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F58A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F58AC: 4BD71575  bl 0x82466e20
	ctx.lr = 0x826F58B0;
	sub_82466E20(ctx, base);
	// 826F58B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F58B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F58B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F58BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F58C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F58C0 size=112
    let mut pc: u32 = 0x826F58C0;
    'dispatch: loop {
        match pc {
            0x826F58C0 => {
    //   block [0x826F58C0..0x826F5930)
	// 826F58C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F58C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F58C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F58CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F58D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F58D4: 38AA9A24  addi r5, r10, -0x65dc
	ctx.r[5].s64 = ctx.r[10].s64 + -26076;
	// 826F58D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F58DC: 390BE2D8  addi r8, r11, -0x1d28
	ctx.r[8].s64 = ctx.r[11].s64 + -7464;
	// 826F58E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F58E4: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 826F58E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F58EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F58F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F58F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F58F8: 386A9A84  addi r3, r10, -0x657c
	ctx.r[3].s64 = ctx.r[10].s64 + -25980;
	// 826F58FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F590C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F591C: 4BD71505  bl 0x82466e20
	ctx.lr = 0x826F5920;
	sub_82466E20(ctx, base);
	// 826F5920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F592C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5930 size=100
    let mut pc: u32 = 0x826F5930;
    'dispatch: loop {
        match pc {
            0x826F5930 => {
    //   block [0x826F5930..0x826F5994)
	// 826F5930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F593C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5944: 38AA9A24  addi r5, r10, -0x65dc
	ctx.r[5].s64 = ctx.r[10].s64 + -26076;
	// 826F5948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F594C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5950: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826F5954: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F595C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5964: 386A9AB4  addi r3, r10, -0x654c
	ctx.r[3].s64 = ctx.r[10].s64 + -25932;
	// 826F5968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F596C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F5974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F597C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5980: 4BD714A1  bl 0x82466e20
	ctx.lr = 0x826F5984;
	sub_82466E20(ctx, base);
	// 826F5984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F598C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5998 size=112
    let mut pc: u32 = 0x826F5998;
    'dispatch: loop {
        match pc {
            0x826F5998 => {
    //   block [0x826F5998..0x826F5A08)
	// 826F5998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F599C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F59A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F59A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F59A8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F59AC: 38AA9A24  addi r5, r10, -0x65dc
	ctx.r[5].s64 = ctx.r[10].s64 + -26076;
	// 826F59B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F59B4: 390BE308  addi r8, r11, -0x1cf8
	ctx.r[8].s64 = ctx.r[11].s64 + -7416;
	// 826F59B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F59BC: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 826F59C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F59C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F59C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F59CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F59D0: 386A9AE4  addi r3, r10, -0x651c
	ctx.r[3].s64 = ctx.r[10].s64 + -25884;
	// 826F59D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F59D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F59DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F59E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F59E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F59E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F59EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F59F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F59F4: 4BD7142D  bl 0x82466e20
	ctx.lr = 0x826F59F8;
	sub_82466E20(ctx, base);
	// 826F59F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F59FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5A08 size=108
    let mut pc: u32 = 0x826F5A08;
    'dispatch: loop {
        match pc {
            0x826F5A08 => {
    //   block [0x826F5A08..0x826F5A74)
	// 826F5A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5A10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5A14: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5A18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826F5A1C: 38EBE320  addi r7, r11, -0x1ce0
	ctx.r[7].s64 = ctx.r[11].s64 + -7392;
	// 826F5A20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F5A24: 388A0E24  addi r4, r10, 0xe24
	ctx.r[4].s64 = ctx.r[10].s64 + 3620;
	// 826F5A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5A2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5A30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F5A34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5A38: 386A9B14  addi r3, r10, -0x64ec
	ctx.r[3].s64 = ctx.r[10].s64 + -25836;
	// 826F5A3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F5A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5A60: 4BD713C1  bl 0x82466e20
	ctx.lr = 0x826F5A64;
	sub_82466E20(ctx, base);
	// 826F5A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5A78 size=112
    let mut pc: u32 = 0x826F5A78;
    'dispatch: loop {
        match pc {
            0x826F5A78 => {
    //   block [0x826F5A78..0x826F5AE8)
	// 826F5A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5A84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F5A88: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5A8C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F5A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5A94: 390BE368  addi r8, r11, -0x1c98
	ctx.r[8].s64 = ctx.r[11].s64 + -7320;
	// 826F5A98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F5A9C: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826F5AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5AA4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5AB0: 386A9B44  addi r3, r10, -0x64bc
	ctx.r[3].s64 = ctx.r[10].s64 + -25788;
	// 826F5AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5AD4: 4BD7134D  bl 0x82466e20
	ctx.lr = 0x826F5AD8;
	sub_82466E20(ctx, base);
	// 826F5AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5AE8 size=112
    let mut pc: u32 = 0x826F5AE8;
    'dispatch: loop {
        match pc {
            0x826F5AE8 => {
    //   block [0x826F5AE8..0x826F5B58)
	// 826F5AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5AF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5AF8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5AFC: 38AA9B44  addi r5, r10, -0x64bc
	ctx.r[5].s64 = ctx.r[10].s64 + -25788;
	// 826F5B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5B04: 390BE3C8  addi r8, r11, -0x1c38
	ctx.r[8].s64 = ctx.r[11].s64 + -7224;
	// 826F5B08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F5B0C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 826F5B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5B14: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5B20: 386A9B74  addi r3, r10, -0x648c
	ctx.r[3].s64 = ctx.r[10].s64 + -25740;
	// 826F5B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5B44: 4BD712DD  bl 0x82466e20
	ctx.lr = 0x826F5B48;
	sub_82466E20(ctx, base);
	// 826F5B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5B58 size=112
    let mut pc: u32 = 0x826F5B58;
    'dispatch: loop {
        match pc {
            0x826F5B58 => {
    //   block [0x826F5B58..0x826F5BC8)
	// 826F5B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5B64: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5B68: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5B6C: 38AA9B44  addi r5, r10, -0x64bc
	ctx.r[5].s64 = ctx.r[10].s64 + -25788;
	// 826F5B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5B74: 390BE3E0  addi r8, r11, -0x1c20
	ctx.r[8].s64 = ctx.r[11].s64 + -7200;
	// 826F5B78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F5B7C: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826F5B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5B84: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5B90: 386A9BA4  addi r3, r10, -0x645c
	ctx.r[3].s64 = ctx.r[10].s64 + -25692;
	// 826F5B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5BB4: 4BD7126D  bl 0x82466e20
	ctx.lr = 0x826F5BB8;
	sub_82466E20(ctx, base);
	// 826F5BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5BC8 size=112
    let mut pc: u32 = 0x826F5BC8;
    'dispatch: loop {
        match pc {
            0x826F5BC8 => {
    //   block [0x826F5BC8..0x826F5C38)
	// 826F5BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5BD4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5BD8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5BDC: 38AA9B44  addi r5, r10, -0x64bc
	ctx.r[5].s64 = ctx.r[10].s64 + -25788;
	// 826F5BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5BE4: 390BE410  addi r8, r11, -0x1bf0
	ctx.r[8].s64 = ctx.r[11].s64 + -7152;
	// 826F5BE8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F5BEC: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 826F5BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5BF4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5C00: 386A9BD4  addi r3, r10, -0x642c
	ctx.r[3].s64 = ctx.r[10].s64 + -25644;
	// 826F5C04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5C14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5C24: 4BD711FD  bl 0x82466e20
	ctx.lr = 0x826F5C28;
	sub_82466E20(ctx, base);
	// 826F5C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F5C38 size=24
    let mut pc: u32 = 0x826F5C38;
    'dispatch: loop {
        match pc {
            0x826F5C38 => {
    //   block [0x826F5C38..0x826F5C50)
	// 826F5C38: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5C3C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F5C40: 394A2378  addi r10, r10, 0x2378
	ctx.r[10].s64 = ctx.r[10].s64 + 9080;
	// 826F5C44: 816BDF8C  lwz r11, -0x2074(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8308 as u32) ) } as u64;
	// 826F5C48: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F5C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5C50 size=112
    let mut pc: u32 = 0x826F5C50;
    'dispatch: loop {
        match pc {
            0x826F5C50 => {
    //   block [0x826F5C50..0x826F5CC0)
	// 826F5C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5C5C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F5C60: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5C64: 392AA418  addi r9, r10, -0x5be8
	ctx.r[9].s64 = ctx.r[10].s64 + -23528;
	// 826F5C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5C6C: 390B2378  addi r8, r11, 0x2378
	ctx.r[8].s64 = ctx.r[11].s64 + 9080;
	// 826F5C70: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826F5C74: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826F5C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5C7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5C84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5C88: 386A9C04  addi r3, r10, -0x63fc
	ctx.r[3].s64 = ctx.r[10].s64 + -25596;
	// 826F5C8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F5C90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F5C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5CA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5CAC: 4BD71175  bl 0x82466e20
	ctx.lr = 0x826F5CB0;
	sub_82466E20(ctx, base);
	// 826F5CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5CC0 size=108
    let mut pc: u32 = 0x826F5CC0;
    'dispatch: loop {
        match pc {
            0x826F5CC0 => {
    //   block [0x826F5CC0..0x826F5D2C)
	// 826F5CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5CCC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5CD4: 38EBE428  addi r7, r11, -0x1bd8
	ctx.r[7].s64 = ctx.r[11].s64 + -7128;
	// 826F5CD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826F5CDC: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826F5CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5CE4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5CE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F5CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5CF0: 386A9C34  addi r3, r10, -0x63cc
	ctx.r[3].s64 = ctx.r[10].s64 + -25548;
	// 826F5CF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F5CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5D18: 4BD71109  bl 0x82466e20
	ctx.lr = 0x826F5D1C;
	sub_82466E20(ctx, base);
	// 826F5D1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5D30 size=108
    let mut pc: u32 = 0x826F5D30;
    'dispatch: loop {
        match pc {
            0x826F5D30 => {
    //   block [0x826F5D30..0x826F5D9C)
	// 826F5D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5D3C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5D44: 38EBE440  addi r7, r11, -0x1bc0
	ctx.r[7].s64 = ctx.r[11].s64 + -7104;
	// 826F5D48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F5D4C: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826F5D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5D54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5D58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F5D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5D60: 386A9C64  addi r3, r10, -0x639c
	ctx.r[3].s64 = ctx.r[10].s64 + -25500;
	// 826F5D64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F5D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5D88: 4BD71099  bl 0x82466e20
	ctx.lr = 0x826F5D8C;
	sub_82466E20(ctx, base);
	// 826F5D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5DA0 size=116
    let mut pc: u32 = 0x826F5DA0;
    'dispatch: loop {
        match pc {
            0x826F5DA0 => {
    //   block [0x826F5DA0..0x826F5E14)
	// 826F5DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5DAC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5DB0: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F5DB4: 390BE48C  addi r8, r11, -0x1b74
	ctx.r[8].s64 = ctx.r[11].s64 + -7028;
	// 826F5DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5DBC: 392AA4D0  addi r9, r10, -0x5b30
	ctx.r[9].s64 = ctx.r[10].s64 + -23344;
	// 826F5DC0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F5DC4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F5DC8: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F5DCC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5DD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5DE4: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F5DE8: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826F5DEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F5DF0: 386B9C94  addi r3, r11, -0x636c
	ctx.r[3].s64 = ctx.r[11].s64 + -25452;
	// 826F5DF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F5DF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5E00: 4BD71021  bl 0x82466e20
	ctx.lr = 0x826F5E04;
	sub_82466E20(ctx, base);
	// 826F5E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F5E18 size=24
    let mut pc: u32 = 0x826F5E18;
    'dispatch: loop {
        match pc {
            0x826F5E18 => {
    //   block [0x826F5E18..0x826F5E30)
	// 826F5E18: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5E1C: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F5E20: 394A23C0  addi r10, r10, 0x23c0
	ctx.r[10].s64 = ctx.r[10].s64 + 9152;
	// 826F5E24: 816BE4A4  lwz r11, -0x1b5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7004 as u32) ) } as u64;
	// 826F5E28: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826F5E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5E30 size=116
    let mut pc: u32 = 0x826F5E30;
    'dispatch: loop {
        match pc {
            0x826F5E30 => {
    //   block [0x826F5E30..0x826F5EA4)
	// 826F5E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5E3C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5E40: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F5E44: 390B23C0  addi r8, r11, 0x23c0
	ctx.r[8].s64 = ctx.r[11].s64 + 9152;
	// 826F5E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5E4C: 392AA540  addi r9, r10, -0x5ac0
	ctx.r[9].s64 = ctx.r[10].s64 + -23232;
	// 826F5E50: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F5E54: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826F5E58: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F5E5C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5E64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5E74: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F5E78: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826F5E7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F5E80: 386B9CC4  addi r3, r11, -0x633c
	ctx.r[3].s64 = ctx.r[11].s64 + -25404;
	// 826F5E84: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826F5E88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5E90: 4BD70F91  bl 0x82466e20
	ctx.lr = 0x826F5E94;
	sub_82466E20(ctx, base);
	// 826F5E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5EA8 size=108
    let mut pc: u32 = 0x826F5EA8;
    'dispatch: loop {
        match pc {
            0x826F5EA8 => {
    //   block [0x826F5EA8..0x826F5F14)
	// 826F5EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5EB4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5EBC: 38EBE4B4  addi r7, r11, -0x1b4c
	ctx.r[7].s64 = ctx.r[11].s64 + -6988;
	// 826F5EC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F5EC4: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 826F5EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5ECC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F5ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5ED8: 386A9CF4  addi r3, r10, -0x630c
	ctx.r[3].s64 = ctx.r[10].s64 + -25356;
	// 826F5EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F5EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5F00: 4BD70F21  bl 0x82466e20
	ctx.lr = 0x826F5F04;
	sub_82466E20(ctx, base);
	// 826F5F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5F18 size=112
    let mut pc: u32 = 0x826F5F18;
    'dispatch: loop {
        match pc {
            0x826F5F18 => {
    //   block [0x826F5F18..0x826F5F88)
	// 826F5F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5F24: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5F28: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5F2C: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F5F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5F34: 390BE4E4  addi r8, r11, -0x1b1c
	ctx.r[8].s64 = ctx.r[11].s64 + -6940;
	// 826F5F38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F5F3C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826F5F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5F44: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5F50: 386A9D24  addi r3, r10, -0x62dc
	ctx.r[3].s64 = ctx.r[10].s64 + -25308;
	// 826F5F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F5F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F5F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F5F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5F74: 4BD70EAD  bl 0x82466e20
	ctx.lr = 0x826F5F78;
	sub_82466E20(ctx, base);
	// 826F5F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5F88 size=112
    let mut pc: u32 = 0x826F5F88;
    'dispatch: loop {
        match pc {
            0x826F5F88 => {
    //   block [0x826F5F88..0x826F5FF8)
	// 826F5F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F5F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F5F94: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F5F98: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F5F9C: 392AA598  addi r9, r10, -0x5a68
	ctx.r[9].s64 = ctx.r[10].s64 + -23144;
	// 826F5FA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F5FA4: 390BE500  addi r8, r11, -0x1b00
	ctx.r[8].s64 = ctx.r[11].s64 + -6912;
	// 826F5FA8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826F5FAC: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 826F5FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F5FB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F5FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F5FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F5FC0: 386A9D54  addi r3, r10, -0x62ac
	ctx.r[3].s64 = ctx.r[10].s64 + -25260;
	// 826F5FC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F5FC8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F5FCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F5FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F5FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F5FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F5FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F5FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F5FE4: 4BD70E3D  bl 0x82466e20
	ctx.lr = 0x826F5FE8;
	sub_82466E20(ctx, base);
	// 826F5FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F5FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F5FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F5FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F5FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F5FF8 size=112
    let mut pc: u32 = 0x826F5FF8;
    'dispatch: loop {
        match pc {
            0x826F5FF8 => {
    //   block [0x826F5FF8..0x826F6068)
	// 826F5FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F5FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6004: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6008: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F600C: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6014: 390BE548  addi r8, r11, -0x1ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -6840;
	// 826F6018: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F601C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826F6020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6024: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F602C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6030: 386A9D84  addi r3, r10, -0x627c
	ctx.r[3].s64 = ctx.r[10].s64 + -25212;
	// 826F6034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F603C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F604C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6054: 4BD70DCD  bl 0x82466e20
	ctx.lr = 0x826F6058;
	sub_82466E20(ctx, base);
	// 826F6058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F605C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6068 size=112
    let mut pc: u32 = 0x826F6068;
    'dispatch: loop {
        match pc {
            0x826F6068 => {
    //   block [0x826F6068..0x826F60D8)
	// 826F6068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F606C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6074: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F6078: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F607C: 392AA5C4  addi r9, r10, -0x5a3c
	ctx.r[9].s64 = ctx.r[10].s64 + -23100;
	// 826F6080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6084: 390BE568  addi r8, r11, -0x1a98
	ctx.r[8].s64 = ctx.r[11].s64 + -6808;
	// 826F6088: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826F608C: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 826F6090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6094: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F609C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F60A0: 386A9DB4  addi r3, r10, -0x624c
	ctx.r[3].s64 = ctx.r[10].s64 + -25164;
	// 826F60A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F60A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F60AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F60B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F60B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F60B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F60BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F60C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F60C4: 4BD70D5D  bl 0x82466e20
	ctx.lr = 0x826F60C8;
	sub_82466E20(ctx, base);
	// 826F60C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F60CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F60D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F60D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F60D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F60D8 size=112
    let mut pc: u32 = 0x826F60D8;
    'dispatch: loop {
        match pc {
            0x826F60D8 => {
    //   block [0x826F60D8..0x826F6148)
	// 826F60D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F60DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F60E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F60E4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F60E8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F60EC: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F60F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F60F4: 390BE5F8  addi r8, r11, -0x1a08
	ctx.r[8].s64 = ctx.r[11].s64 + -6664;
	// 826F60F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F60FC: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826F6100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6104: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F610C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6110: 386A9DE4  addi r3, r10, -0x621c
	ctx.r[3].s64 = ctx.r[10].s64 + -25116;
	// 826F6114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F611C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F612C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6134: 4BD70CED  bl 0x82466e20
	ctx.lr = 0x826F6138;
	sub_82466E20(ctx, base);
	// 826F6138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F613C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6148 size=112
    let mut pc: u32 = 0x826F6148;
    'dispatch: loop {
        match pc {
            0x826F6148 => {
    //   block [0x826F6148..0x826F61B8)
	// 826F6148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F614C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6154: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6158: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F615C: 38AA9E44  addi r5, r10, -0x61bc
	ctx.r[5].s64 = ctx.r[10].s64 + -25020;
	// 826F6160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6164: 390BE610  addi r8, r11, -0x19f0
	ctx.r[8].s64 = ctx.r[11].s64 + -6640;
	// 826F6168: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F616C: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826F6170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6174: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6178: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F617C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6180: 386A9E14  addi r3, r10, -0x61ec
	ctx.r[3].s64 = ctx.r[10].s64 + -25068;
	// 826F6184: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F618C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F619C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F61A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F61A4: 4BD70C7D  bl 0x82466e20
	ctx.lr = 0x826F61A8;
	sub_82466E20(ctx, base);
	// 826F61A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F61AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F61B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F61B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F61B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F61B8 size=100
    let mut pc: u32 = 0x826F61B8;
    'dispatch: loop {
        match pc {
            0x826F61B8 => {
    //   block [0x826F61B8..0x826F621C)
	// 826F61B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F61BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F61C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F61C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F61C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F61CC: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F61D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F61D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F61D8: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826F61DC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F61E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F61E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F61E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F61EC: 386A9E44  addi r3, r10, -0x61bc
	ctx.r[3].s64 = ctx.r[10].s64 + -25020;
	// 826F61F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F61F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F61F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F61FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6200: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F6204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6208: 4BD70C19  bl 0x82466e20
	ctx.lr = 0x826F620C;
	sub_82466E20(ctx, base);
	// 826F620C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F6220 size=24
    let mut pc: u32 = 0x826F6220;
    'dispatch: loop {
        match pc {
            0x826F6220 => {
    //   block [0x826F6220..0x826F6238)
	// 826F6220: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6224: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F6228: 394A2498  addi r10, r10, 0x2498
	ctx.r[10].s64 = ctx.r[10].s64 + 9368;
	// 826F622C: 816BE564  lwz r11, -0x1a9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6812 as u32) ) } as u64;
	// 826F6230: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826F6234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6238 size=116
    let mut pc: u32 = 0x826F6238;
    'dispatch: loop {
        match pc {
            0x826F6238 => {
    //   block [0x826F6238..0x826F62AC)
	// 826F6238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F623C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6244: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6248: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F624C: 390B2498  addi r8, r11, 0x2498
	ctx.r[8].s64 = ctx.r[11].s64 + 9368;
	// 826F6250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6254: 392AA600  addi r9, r10, -0x5a00
	ctx.r[9].s64 = ctx.r[10].s64 + -23040;
	// 826F6258: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F625C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F6260: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6264: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F626C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F627C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F6280: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826F6284: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6288: 386B9E74  addi r3, r11, -0x618c
	ctx.r[3].s64 = ctx.r[11].s64 + -24972;
	// 826F628C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F6290: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6298: 4BD70B89  bl 0x82466e20
	ctx.lr = 0x826F629C;
	sub_82466E20(ctx, base);
	// 826F629C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F62A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F62A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F62A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F62B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F62B0 size=108
    let mut pc: u32 = 0x826F62B0;
    'dispatch: loop {
        match pc {
            0x826F62B0 => {
    //   block [0x826F62B0..0x826F631C)
	// 826F62B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F62B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F62B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F62BC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F62C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F62C4: 38EBE688  addi r7, r11, -0x1978
	ctx.r[7].s64 = ctx.r[11].s64 + -6520;
	// 826F62C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F62CC: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 826F62D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F62D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F62D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F62DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F62E0: 386A9EA4  addi r3, r10, -0x615c
	ctx.r[3].s64 = ctx.r[10].s64 + -24924;
	// 826F62E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F62E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F62EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F62F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F62F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F62F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F62FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6308: 4BD70B19  bl 0x82466e20
	ctx.lr = 0x826F630C;
	sub_82466E20(ctx, base);
	// 826F630C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6320 size=112
    let mut pc: u32 = 0x826F6320;
    'dispatch: loop {
        match pc {
            0x826F6320 => {
    //   block [0x826F6320..0x826F6390)
	// 826F6320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F632C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6330: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6334: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F633C: 390BE6B8  addi r8, r11, -0x1948
	ctx.r[8].s64 = ctx.r[11].s64 + -6472;
	// 826F6340: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F6344: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826F6348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F634C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6358: 386A9ED4  addi r3, r10, -0x612c
	ctx.r[3].s64 = ctx.r[10].s64 + -24876;
	// 826F635C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F636C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F637C: 4BD70AA5  bl 0x82466e20
	ctx.lr = 0x826F6380;
	sub_82466E20(ctx, base);
	// 826F6380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F638C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6390 size=112
    let mut pc: u32 = 0x826F6390;
    'dispatch: loop {
        match pc {
            0x826F6390 => {
    //   block [0x826F6390..0x826F6400)
	// 826F6390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F639C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F63A0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F63A4: 392AA624  addi r9, r10, -0x59dc
	ctx.r[9].s64 = ctx.r[10].s64 + -23004;
	// 826F63A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F63AC: 390BE6D8  addi r8, r11, -0x1928
	ctx.r[8].s64 = ctx.r[11].s64 + -6440;
	// 826F63B0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F63B4: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 826F63B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F63BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F63C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F63C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F63C8: 386A9F04  addi r3, r10, -0x60fc
	ctx.r[3].s64 = ctx.r[10].s64 + -24828;
	// 826F63CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F63D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F63D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F63D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F63DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F63E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F63E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F63E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F63EC: 4BD70A35  bl 0x82466e20
	ctx.lr = 0x826F63F0;
	sub_82466E20(ctx, base);
	// 826F63F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F63F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F63F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F63FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6400 size=112
    let mut pc: u32 = 0x826F6400;
    'dispatch: loop {
        match pc {
            0x826F6400 => {
    //   block [0x826F6400..0x826F6470)
	// 826F6400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F640C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6410: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6414: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F641C: 390BE780  addi r8, r11, -0x1880
	ctx.r[8].s64 = ctx.r[11].s64 + -6272;
	// 826F6420: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F6424: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826F6428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F642C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6438: 386A9F34  addi r3, r10, -0x60cc
	ctx.r[3].s64 = ctx.r[10].s64 + -24780;
	// 826F643C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F644C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F645C: 4BD709C5  bl 0x82466e20
	ctx.lr = 0x826F6460;
	sub_82466E20(ctx, base);
	// 826F6460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F646C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6470 size=108
    let mut pc: u32 = 0x826F6470;
    'dispatch: loop {
        match pc {
            0x826F6470 => {
    //   block [0x826F6470..0x826F64DC)
	// 826F6470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F647C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6484: 38EBE798  addi r7, r11, -0x1868
	ctx.r[7].s64 = ctx.r[11].s64 + -6248;
	// 826F6488: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F648C: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826F6490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6494: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F649C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F64A0: 386A9F64  addi r3, r10, -0x609c
	ctx.r[3].s64 = ctx.r[10].s64 + -24732;
	// 826F64A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F64A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F64AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F64B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F64B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F64B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F64BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F64C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F64C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F64C8: 4BD70959  bl 0x82466e20
	ctx.lr = 0x826F64CC;
	sub_82466E20(ctx, base);
	// 826F64CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F64D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F64D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F64D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F64E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F64E0 size=112
    let mut pc: u32 = 0x826F64E0;
    'dispatch: loop {
        match pc {
            0x826F64E0 => {
    //   block [0x826F64E0..0x826F6550)
	// 826F64E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F64E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F64E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F64EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F64F0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F64F4: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F64F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F64FC: 390BE7C8  addi r8, r11, -0x1838
	ctx.r[8].s64 = ctx.r[11].s64 + -6200;
	// 826F6500: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F6504: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826F6508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F650C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6518: 386A9F94  addi r3, r10, -0x606c
	ctx.r[3].s64 = ctx.r[10].s64 + -24684;
	// 826F651C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F652C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F653C: 4BD708E5  bl 0x82466e20
	ctx.lr = 0x826F6540;
	sub_82466E20(ctx, base);
	// 826F6540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F654C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6550 size=112
    let mut pc: u32 = 0x826F6550;
    'dispatch: loop {
        match pc {
            0x826F6550 => {
    //   block [0x826F6550..0x826F65C0)
	// 826F6550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F655C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F6560: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6564: 392AA658  addi r9, r10, -0x59a8
	ctx.r[9].s64 = ctx.r[10].s64 + -22952;
	// 826F6568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F656C: 390BE7E0  addi r8, r11, -0x1820
	ctx.r[8].s64 = ctx.r[11].s64 + -6176;
	// 826F6570: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826F6574: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826F6578: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F657C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6580: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6588: 386A9FC4  addi r3, r10, -0x603c
	ctx.r[3].s64 = ctx.r[10].s64 + -24636;
	// 826F658C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6590: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F6594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F659C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F65A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F65A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F65A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F65AC: 4BD70875  bl 0x82466e20
	ctx.lr = 0x826F65B0;
	sub_82466E20(ctx, base);
	// 826F65B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F65B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F65B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F65BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F65C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F65C0 size=112
    let mut pc: u32 = 0x826F65C0;
    'dispatch: loop {
        match pc {
            0x826F65C0 => {
    //   block [0x826F65C0..0x826F6630)
	// 826F65C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F65C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F65C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F65CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F65D0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F65D4: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F65D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F65DC: 390BE888  addi r8, r11, -0x1778
	ctx.r[8].s64 = ctx.r[11].s64 + -6008;
	// 826F65E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F65E4: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 826F65E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F65EC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F65F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F65F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F65F8: 386A9FF4  addi r3, r10, -0x600c
	ctx.r[3].s64 = ctx.r[10].s64 + -24588;
	// 826F65FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F660C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F661C: 4BD70805  bl 0x82466e20
	ctx.lr = 0x826F6620;
	sub_82466E20(ctx, base);
	// 826F6620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F662C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6630 size=112
    let mut pc: u32 = 0x826F6630;
    'dispatch: loop {
        match pc {
            0x826F6630 => {
    //   block [0x826F6630..0x826F66A0)
	// 826F6630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F663C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6640: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6644: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F664C: 390BE8D0  addi r8, r11, -0x1730
	ctx.r[8].s64 = ctx.r[11].s64 + -5936;
	// 826F6650: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826F6654: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826F6658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F665C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6660: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6668: 386AA024  addi r3, r10, -0x5fdc
	ctx.r[3].s64 = ctx.r[10].s64 + -24540;
	// 826F666C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F667C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F668C: 4BD70795  bl 0x82466e20
	ctx.lr = 0x826F6690;
	sub_82466E20(ctx, base);
	// 826F6690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F669C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F66A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F66A0 size=100
    let mut pc: u32 = 0x826F66A0;
    'dispatch: loop {
        match pc {
            0x826F66A0 => {
    //   block [0x826F66A0..0x826F6704)
	// 826F66A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F66A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F66A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F66AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F66B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F66B4: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F66B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F66BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F66C0: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 826F66C4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F66C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F66CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F66D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F66D4: 386AA054  addi r3, r10, -0x5fac
	ctx.r[3].s64 = ctx.r[10].s64 + -24492;
	// 826F66D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F66DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F66E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F66E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F66E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F66EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F66F0: 4BD70731  bl 0x82466e20
	ctx.lr = 0x826F66F4;
	sub_82466E20(ctx, base);
	// 826F66F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F66F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F66FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6708 size=112
    let mut pc: u32 = 0x826F6708;
    'dispatch: loop {
        match pc {
            0x826F6708 => {
    //   block [0x826F6708..0x826F6778)
	// 826F6708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F670C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6714: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6718: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F671C: 38AA9CC4  addi r5, r10, -0x633c
	ctx.r[5].s64 = ctx.r[10].s64 + -25404;
	// 826F6720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6724: 390BE990  addi r8, r11, -0x1670
	ctx.r[8].s64 = ctx.r[11].s64 + -5744;
	// 826F6728: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F672C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 826F6730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6734: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F673C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6740: 386AA084  addi r3, r10, -0x5f7c
	ctx.r[3].s64 = ctx.r[10].s64 + -24444;
	// 826F6744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F674C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F675C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6764: 4BD706BD  bl 0x82466e20
	ctx.lr = 0x826F6768;
	sub_82466E20(ctx, base);
	// 826F6768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F676C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6778 size=112
    let mut pc: u32 = 0x826F6778;
    'dispatch: loop {
        match pc {
            0x826F6778 => {
    //   block [0x826F6778..0x826F67E8)
	// 826F6778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F677C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6784: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6788: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F678C: 38AA9B44  addi r5, r10, -0x64bc
	ctx.r[5].s64 = ctx.r[10].s64 + -25788;
	// 826F6790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6794: 390BE9C0  addi r8, r11, -0x1640
	ctx.r[8].s64 = ctx.r[11].s64 + -5696;
	// 826F6798: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F679C: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 826F67A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F67A4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F67A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F67AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F67B0: 386AA0B4  addi r3, r10, -0x5f4c
	ctx.r[3].s64 = ctx.r[10].s64 + -24396;
	// 826F67B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F67B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F67BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F67C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F67C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F67C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F67CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F67D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F67D4: 4BD7064D  bl 0x82466e20
	ctx.lr = 0x826F67D8;
	sub_82466E20(ctx, base);
	// 826F67D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F67DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F67E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F67E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F67E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F67E8 size=108
    let mut pc: u32 = 0x826F67E8;
    'dispatch: loop {
        match pc {
            0x826F67E8 => {
    //   block [0x826F67E8..0x826F6854)
	// 826F67E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F67EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F67F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F67F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F67F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F67FC: 38EBE9D8  addi r7, r11, -0x1628
	ctx.r[7].s64 = ctx.r[11].s64 + -5672;
	// 826F6800: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F6804: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 826F6808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F680C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F6814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6818: 386AA0E4  addi r3, r10, -0x5f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -24348;
	// 826F681C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F6820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F682C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F683C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6840: 4BD705E1  bl 0x82466e20
	ctx.lr = 0x826F6844;
	sub_82466E20(ctx, base);
	// 826F6844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F684C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6858 size=112
    let mut pc: u32 = 0x826F6858;
    'dispatch: loop {
        match pc {
            0x826F6858 => {
    //   block [0x826F6858..0x826F68C8)
	// 826F6858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F685C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6864: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6868: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F686C: 38AAA054  addi r5, r10, -0x5fac
	ctx.r[5].s64 = ctx.r[10].s64 + -24492;
	// 826F6870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6874: 390BEA08  addi r8, r11, -0x15f8
	ctx.r[8].s64 = ctx.r[11].s64 + -5624;
	// 826F6878: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826F687C: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826F6880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6884: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F688C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6890: 386AA114  addi r3, r10, -0x5eec
	ctx.r[3].s64 = ctx.r[10].s64 + -24300;
	// 826F6894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F689C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F68A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F68A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F68A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F68AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F68B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F68B4: 4BD7056D  bl 0x82466e20
	ctx.lr = 0x826F68B8;
	sub_82466E20(ctx, base);
	// 826F68B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F68BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F68C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F68C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F68C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F68C8 size=112
    let mut pc: u32 = 0x826F68C8;
    'dispatch: loop {
        match pc {
            0x826F68C8 => {
    //   block [0x826F68C8..0x826F6938)
	// 826F68C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F68CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F68D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F68D4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F68D8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F68DC: 392AA684  addi r9, r10, -0x597c
	ctx.r[9].s64 = ctx.r[10].s64 + -22908;
	// 826F68E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F68E4: 390BEAA0  addi r8, r11, -0x1560
	ctx.r[8].s64 = ctx.r[11].s64 + -5472;
	// 826F68E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826F68EC: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 826F68F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F68F4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F68F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F68FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6900: 386AA144  addi r3, r10, -0x5ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -24252;
	// 826F6904: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6908: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F690C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F691C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6924: 4BD704FD  bl 0x82466e20
	ctx.lr = 0x826F6928;
	sub_82466E20(ctx, base);
	// 826F6928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F692C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6938 size=112
    let mut pc: u32 = 0x826F6938;
    'dispatch: loop {
        match pc {
            0x826F6938 => {
    //   block [0x826F6938..0x826F69A8)
	// 826F6938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F693C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6944: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6948: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F694C: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6954: 390BEAE8  addi r8, r11, -0x1518
	ctx.r[8].s64 = ctx.r[11].s64 + -5400;
	// 826F6958: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F695C: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826F6960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6964: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F696C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6970: 386AA174  addi r3, r10, -0x5e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -24204;
	// 826F6974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F697C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F698C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6994: 4BD7048D  bl 0x82466e20
	ctx.lr = 0x826F6998;
	sub_82466E20(ctx, base);
	// 826F6998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F699C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F69A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F69A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F69A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F69A8 size=108
    let mut pc: u32 = 0x826F69A8;
    'dispatch: loop {
        match pc {
            0x826F69A8 => {
    //   block [0x826F69A8..0x826F6A14)
	// 826F69A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F69AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F69B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F69B4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F69B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F69BC: 38EBEB00  addi r7, r11, -0x1500
	ctx.r[7].s64 = ctx.r[11].s64 + -5376;
	// 826F69C0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826F69C4: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 826F69C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F69CC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F69D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F69D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F69D8: 386AA1A4  addi r3, r10, -0x5e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -24156;
	// 826F69DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F69E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F69E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F69E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F69EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F69F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F69F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F69F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F69FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6A00: 4BD70421  bl 0x82466e20
	ctx.lr = 0x826F6A04;
	sub_82466E20(ctx, base);
	// 826F6A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6A18 size=116
    let mut pc: u32 = 0x826F6A18;
    'dispatch: loop {
        match pc {
            0x826F6A18 => {
    //   block [0x826F6A18..0x826F6A8C)
	// 826F6A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6A24: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F6A28: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826F6A2C: 390AEB90  addi r8, r10, -0x1470
	ctx.r[8].s64 = ctx.r[10].s64 + -5232;
	// 826F6A30: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6A34: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F6A38: 38AAA054  addi r5, r10, -0x5fac
	ctx.r[5].s64 = ctx.r[10].s64 + -24492;
	// 826F6A3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6A40: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6A4C: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826F6A50: 396BA698  addi r11, r11, -0x5968
	ctx.r[11].s64 = ctx.r[11].s64 + -22888;
	// 826F6A54: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6A58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6A5C: 386AA1D4  addi r3, r10, -0x5e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -24108;
	// 826F6A60: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826F6A64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6A68: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826F6A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6A78: 4BD703A9  bl 0x82466e20
	ctx.lr = 0x826F6A7C;
	sub_82466E20(ctx, base);
	// 826F6A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6A90 size=108
    let mut pc: u32 = 0x826F6A90;
    'dispatch: loop {
        match pc {
            0x826F6A90 => {
    //   block [0x826F6A90..0x826F6AFC)
	// 826F6A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6A9C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6AA4: 38EBEC68  addi r7, r11, -0x1398
	ctx.r[7].s64 = ctx.r[11].s64 + -5016;
	// 826F6AA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F6AAC: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826F6AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6AB4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6AB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F6ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6AC0: 386AA204  addi r3, r10, -0x5dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -24060;
	// 826F6AC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F6AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6AE8: 4BD70339  bl 0x82466e20
	ctx.lr = 0x826F6AEC;
	sub_82466E20(ctx, base);
	// 826F6AEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6B00 size=112
    let mut pc: u32 = 0x826F6B00;
    'dispatch: loop {
        match pc {
            0x826F6B00 => {
    //   block [0x826F6B00..0x826F6B70)
	// 826F6B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6B0C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6B10: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6B14: 38AAA054  addi r5, r10, -0x5fac
	ctx.r[5].s64 = ctx.r[10].s64 + -24492;
	// 826F6B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6B1C: 390BECB0  addi r8, r11, -0x1350
	ctx.r[8].s64 = ctx.r[11].s64 + -4944;
	// 826F6B20: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826F6B24: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 826F6B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6B2C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6B38: 386AA234  addi r3, r10, -0x5dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -24012;
	// 826F6B3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6B5C: 4BD702C5  bl 0x82466e20
	ctx.lr = 0x826F6B60;
	sub_82466E20(ctx, base);
	// 826F6B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6B70 size=112
    let mut pc: u32 = 0x826F6B70;
    'dispatch: loop {
        match pc {
            0x826F6B70 => {
    //   block [0x826F6B70..0x826F6BE0)
	// 826F6B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6B7C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6B80: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6B84: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6B8C: 390BED28  addi r8, r11, -0x12d8
	ctx.r[8].s64 = ctx.r[11].s64 + -4824;
	// 826F6B90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F6B94: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826F6B98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6B9C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6BA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6BA8: 386AA264  addi r3, r10, -0x5d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -23964;
	// 826F6BAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6BB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6BC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6BCC: 4BD70255  bl 0x82466e20
	ctx.lr = 0x826F6BD0;
	sub_82466E20(ctx, base);
	// 826F6BD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6BD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6BD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6BE0 size=108
    let mut pc: u32 = 0x826F6BE0;
    'dispatch: loop {
        match pc {
            0x826F6BE0 => {
    //   block [0x826F6BE0..0x826F6C4C)
	// 826F6BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6BEC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6BF4: 38EBED58  addi r7, r11, -0x12a8
	ctx.r[7].s64 = ctx.r[11].s64 + -4776;
	// 826F6BF8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F6BFC: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 826F6C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6C04: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6C08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F6C0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6C10: 386AA294  addi r3, r10, -0x5d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -23916;
	// 826F6C14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F6C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6C1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6C24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6C38: 4BD701E9  bl 0x82466e20
	ctx.lr = 0x826F6C3C;
	sub_82466E20(ctx, base);
	// 826F6C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6C50 size=108
    let mut pc: u32 = 0x826F6C50;
    'dispatch: loop {
        match pc {
            0x826F6C50 => {
    //   block [0x826F6C50..0x826F6CBC)
	// 826F6C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6C5C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6C64: 38EBEDB8  addi r7, r11, -0x1248
	ctx.r[7].s64 = ctx.r[11].s64 + -4680;
	// 826F6C68: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826F6C6C: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826F6C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6C74: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6C78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F6C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6C80: 386AA2C4  addi r3, r10, -0x5d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -23868;
	// 826F6C84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F6C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6CA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F6CA8: 4BD70179  bl 0x82466e20
	ctx.lr = 0x826F6CAC;
	sub_82466E20(ctx, base);
	// 826F6CAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6CC0 size=112
    let mut pc: u32 = 0x826F6CC0;
    'dispatch: loop {
        match pc {
            0x826F6CC0 => {
    //   block [0x826F6CC0..0x826F6D30)
	// 826F6CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6CCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6CD0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6CD4: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F6CD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6CDC: 390BEE30  addi r8, r11, -0x11d0
	ctx.r[8].s64 = ctx.r[11].s64 + -4560;
	// 826F6CE0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F6CE4: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 826F6CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6CEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6CF8: 386AA2F4  addi r3, r10, -0x5d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -23820;
	// 826F6CFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6D1C: 4BD70105  bl 0x82466e20
	ctx.lr = 0x826F6D20;
	sub_82466E20(ctx, base);
	// 826F6D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F6D30 size=24
    let mut pc: u32 = 0x826F6D30;
    'dispatch: loop {
        match pc {
            0x826F6D30 => {
    //   block [0x826F6D30..0x826F6D48)
	// 826F6D30: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6D34: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F6D38: 394A2510  addi r10, r10, 0x2510
	ctx.r[10].s64 = ctx.r[10].s64 + 9488;
	// 826F6D3C: 816BEA9C  lwz r11, -0x1564(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5476 as u32) ) } as u64;
	// 826F6D40: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F6D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6D48 size=116
    let mut pc: u32 = 0x826F6D48;
    'dispatch: loop {
        match pc {
            0x826F6D48 => {
    //   block [0x826F6D48..0x826F6DBC)
	// 826F6D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6D54: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6D58: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F6D5C: 390B2510  addi r8, r11, 0x2510
	ctx.r[8].s64 = ctx.r[11].s64 + 9488;
	// 826F6D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6D64: 392AA6FC  addi r9, r10, -0x5904
	ctx.r[9].s64 = ctx.r[10].s64 + -22788;
	// 826F6D68: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F6D6C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826F6D70: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F6D74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6D7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6D8C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F6D90: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 826F6D94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6D98: 386BA324  addi r3, r11, -0x5cdc
	ctx.r[3].s64 = ctx.r[11].s64 + -23772;
	// 826F6D9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F6DA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6DA8: 4BD70079  bl 0x82466e20
	ctx.lr = 0x826F6DAC;
	sub_82466E20(ctx, base);
	// 826F6DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6DC0 size=112
    let mut pc: u32 = 0x826F6DC0;
    'dispatch: loop {
        match pc {
            0x826F6DC0 => {
    //   block [0x826F6DC0..0x826F6E30)
	// 826F6DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6DCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6DD0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6DD4: 38AAA324  addi r5, r10, -0x5cdc
	ctx.r[5].s64 = ctx.r[10].s64 + -23772;
	// 826F6DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6DDC: 390BEE78  addi r8, r11, -0x1188
	ctx.r[8].s64 = ctx.r[11].s64 + -4488;
	// 826F6DE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F6DE4: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 826F6DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6DEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6DF8: 386AA354  addi r3, r10, -0x5cac
	ctx.r[3].s64 = ctx.r[10].s64 + -23724;
	// 826F6DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6E1C: 4BD70005  bl 0x82466e20
	ctx.lr = 0x826F6E20;
	sub_82466E20(ctx, base);
	// 826F6E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F6E30 size=24
    let mut pc: u32 = 0x826F6E30;
    'dispatch: loop {
        match pc {
            0x826F6E30 => {
    //   block [0x826F6E30..0x826F6E48)
	// 826F6E30: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6E34: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F6E38: 394A2528  addi r10, r10, 0x2528
	ctx.r[10].s64 = ctx.r[10].s64 + 9512;
	// 826F6E3C: 816BEEA8  lwz r11, -0x1158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4440 as u32) ) } as u64;
	// 826F6E40: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826F6E44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6E48 size=116
    let mut pc: u32 = 0x826F6E48;
    'dispatch: loop {
        match pc {
            0x826F6E48 => {
    //   block [0x826F6E48..0x826F6EBC)
	// 826F6E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6E54: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6E58: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F6E5C: 390B2528  addi r8, r11, 0x2528
	ctx.r[8].s64 = ctx.r[11].s64 + 9512;
	// 826F6E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6E64: 392AA738  addi r9, r10, -0x58c8
	ctx.r[9].s64 = ctx.r[10].s64 + -22728;
	// 826F6E68: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6E6C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826F6E70: 38AAA354  addi r5, r10, -0x5cac
	ctx.r[5].s64 = ctx.r[10].s64 + -23724;
	// 826F6E74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6E7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6E8C: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 826F6E90: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 826F6E94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826F6E98: 386BA384  addi r3, r11, -0x5c7c
	ctx.r[3].s64 = ctx.r[11].s64 + -23676;
	// 826F6E9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F6EA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6EA8: 4BD6FF79  bl 0x82466e20
	ctx.lr = 0x826F6EAC;
	sub_82466E20(ctx, base);
	// 826F6EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6EC0 size=112
    let mut pc: u32 = 0x826F6EC0;
    'dispatch: loop {
        match pc {
            0x826F6EC0 => {
    //   block [0x826F6EC0..0x826F6F30)
	// 826F6EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6ECC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6ED0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6ED4: 38AAA354  addi r5, r10, -0x5cac
	ctx.r[5].s64 = ctx.r[10].s64 + -23724;
	// 826F6ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6EDC: 390BEEB0  addi r8, r11, -0x1150
	ctx.r[8].s64 = ctx.r[11].s64 + -4432;
	// 826F6EE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826F6EE4: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826F6EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6EEC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6EF8: 386AA3B4  addi r3, r10, -0x5c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -23628;
	// 826F6EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6F1C: 4BD6FF05  bl 0x82466e20
	ctx.lr = 0x826F6F20;
	sub_82466E20(ctx, base);
	// 826F6F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6F30 size=112
    let mut pc: u32 = 0x826F6F30;
    'dispatch: loop {
        match pc {
            0x826F6F30 => {
    //   block [0x826F6F30..0x826F6FA0)
	// 826F6F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6F3C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6F40: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6F44: 38AAA354  addi r5, r10, -0x5cac
	ctx.r[5].s64 = ctx.r[10].s64 + -23724;
	// 826F6F48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6F4C: 390BEF10  addi r8, r11, -0x10f0
	ctx.r[8].s64 = ctx.r[11].s64 + -4336;
	// 826F6F50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826F6F54: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826F6F58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6F5C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6F60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6F64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6F68: 386AA3E4  addi r3, r10, -0x5c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -23580;
	// 826F6F6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6F70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6F78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6F80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6F88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6F8C: 4BD6FE95  bl 0x82466e20
	ctx.lr = 0x826F6F90;
	sub_82466E20(ctx, base);
	// 826F6F90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F6F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F6F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F6F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F6FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F6FA0 size=112
    let mut pc: u32 = 0x826F6FA0;
    'dispatch: loop {
        match pc {
            0x826F6FA0 => {
    //   block [0x826F6FA0..0x826F7010)
	// 826F6FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F6FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F6FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F6FAC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6FB0: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F6FB4: 38AAA354  addi r5, r10, -0x5cac
	ctx.r[5].s64 = ctx.r[10].s64 + -23724;
	// 826F6FB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F6FBC: 390BEF40  addi r8, r11, -0x10c0
	ctx.r[8].s64 = ctx.r[11].s64 + -4288;
	// 826F6FC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826F6FC4: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826F6FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F6FCC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F6FD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F6FD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F6FD8: 386AA414  addi r3, r10, -0x5bec
	ctx.r[3].s64 = ctx.r[10].s64 + -23532;
	// 826F6FDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F6FE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F6FE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F6FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F6FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F6FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F6FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F6FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F6FFC: 4BD6FE25  bl 0x82466e20
	ctx.lr = 0x826F7000;
	sub_82466E20(ctx, base);
	// 826F7000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F700C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7010 size=108
    let mut pc: u32 = 0x826F7010;
    'dispatch: loop {
        match pc {
            0x826F7010 => {
    //   block [0x826F7010..0x826F707C)
	// 826F7010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F701C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7024: 38EBEF88  addi r7, r11, -0x1078
	ctx.r[7].s64 = ctx.r[11].s64 + -4216;
	// 826F7028: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F702C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 826F7030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7034: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F703C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7040: 386AA444  addi r3, r10, -0x5bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -23484;
	// 826F7044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F704C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F705C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7068: 4BD6FDB9  bl 0x82466e20
	ctx.lr = 0x826F706C;
	sub_82466E20(ctx, base);
	// 826F706C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7080 size=112
    let mut pc: u32 = 0x826F7080;
    'dispatch: loop {
        match pc {
            0x826F7080 => {
    //   block [0x826F7080..0x826F70F0)
	// 826F7080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F708C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7090: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7094: 38AA9C94  addi r5, r10, -0x636c
	ctx.r[5].s64 = ctx.r[10].s64 + -25452;
	// 826F7098: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F709C: 390BEFB8  addi r8, r11, -0x1048
	ctx.r[8].s64 = ctx.r[11].s64 + -4168;
	// 826F70A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826F70A4: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 826F70A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F70AC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F70B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F70B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F70B8: 386AA474  addi r3, r10, -0x5b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -23436;
	// 826F70BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F70C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F70C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F70C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F70CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F70D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F70D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F70D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F70DC: 4BD6FD45  bl 0x82466e20
	ctx.lr = 0x826F70E0;
	sub_82466E20(ctx, base);
	// 826F70E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F70E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F70E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F70EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F70F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F70F0 size=108
    let mut pc: u32 = 0x826F70F0;
    'dispatch: loop {
        match pc {
            0x826F70F0 => {
    //   block [0x826F70F0..0x826F715C)
	// 826F70F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F70F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F70F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F70FC: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7104: 38EBEFD0  addi r7, r11, -0x1030
	ctx.r[7].s64 = ctx.r[11].s64 + -4144;
	// 826F7108: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826F710C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 826F7110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7114: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F711C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7120: 386AA4A4  addi r3, r10, -0x5b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -23388;
	// 826F7124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F712C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F713C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7148: 4BD6FCD9  bl 0x82466e20
	ctx.lr = 0x826F714C;
	sub_82466E20(ctx, base);
	// 826F714C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7160 size=108
    let mut pc: u32 = 0x826F7160;
    'dispatch: loop {
        match pc {
            0x826F7160 => {
    //   block [0x826F7160..0x826F71CC)
	// 826F7160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F716C: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F7170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7174: 38EBF018  addi r7, r11, -0xfe8
	ctx.r[7].s64 = ctx.r[11].s64 + -4072;
	// 826F7178: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826F717C: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 826F7180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7184: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F718C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7190: 386AA4D4  addi r3, r10, -0x5b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -23340;
	// 826F7194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F719C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F71A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F71A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F71A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F71AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F71B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F71B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F71B8: 4BD6FC69  bl 0x82466e20
	ctx.lr = 0x826F71BC;
	sub_82466E20(ctx, base);
	// 826F71BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F71C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F71C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F71C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F71D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F71D0 size=116
    let mut pc: u32 = 0x826F71D0;
    'dispatch: loop {
        match pc {
            0x826F71D0 => {
    //   block [0x826F71D0..0x826F7244)
	// 826F71D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F71D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F71D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F71DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 826F71E0: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F71E4: 392BA76C  addi r9, r11, -0x5894
	ctx.r[9].s64 = ctx.r[11].s64 + -22676;
	// 826F71E8: 38AAA954  addi r5, r10, -0x56ac
	ctx.r[5].s64 = ctx.r[10].s64 + -22188;
	// 826F71EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F71F0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826F71F4: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 826F71F8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F71FC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826F7200: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7204: 396BF078  addi r11, r11, -0xf88
	ctx.r[11].s64 = ctx.r[11].s64 + -3976;
	// 826F7208: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826F720C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7210: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826F7214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7218: 386AA504  addi r3, r10, -0x5afc
	ctx.r[3].s64 = ctx.r[10].s64 + -23292;
	// 826F721C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F7220: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826F7224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7228: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826F722C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F7230: 4BD6FBF1  bl 0x82466e20
	ctx.lr = 0x826F7234;
	sub_82466E20(ctx, base);
	// 826F7234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F723C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7248 size=100
    let mut pc: u32 = 0x826F7248;
    'dispatch: loop {
        match pc {
            0x826F7248 => {
    //   block [0x826F7248..0x826F72AC)
	// 826F7248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F724C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826F7258: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F725C: 38AA7954  addi r5, r10, 0x7954
	ctx.r[5].s64 = ctx.r[10].s64 + 31060;
	// 826F7260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7264: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7268: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826F726C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F727C: 386AA534  addi r3, r10, -0x5acc
	ctx.r[3].s64 = ctx.r[10].s64 + -23244;
	// 826F7280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7284: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7288: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F728C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7290: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F7294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7298: 4BD6FB89  bl 0x82466e20
	ctx.lr = 0x826F729C;
	sub_82466E20(ctx, base);
	// 826F729C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F72A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F72A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F72A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F72B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F72B0 size=100
    let mut pc: u32 = 0x826F72B0;
    'dispatch: loop {
        match pc {
            0x826F72B0 => {
    //   block [0x826F72B0..0x826F7314)
	// 826F72B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F72B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F72B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F72BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F72C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F72C4: 38AAA5C4  addi r5, r10, -0x5a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -23100;
	// 826F72C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F72CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F72D0: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826F72D4: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F72D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F72DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F72E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F72E4: 386AA564  addi r3, r10, -0x5a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -23196;
	// 826F72E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F72EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F72F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F72F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F72F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F72FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7300: 4BD6FB21  bl 0x82466e20
	ctx.lr = 0x826F7304;
	sub_82466E20(ctx, base);
	// 826F7304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F730C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7318 size=100
    let mut pc: u32 = 0x826F7318;
    'dispatch: loop {
        match pc {
            0x826F7318 => {
    //   block [0x826F7318..0x826F737C)
	// 826F7318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7324: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F732C: 38AAA504  addi r5, r10, -0x5afc
	ctx.r[5].s64 = ctx.r[10].s64 + -23292;
	// 826F7330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7338: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826F733C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F7344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F734C: 386AA594  addi r3, r10, -0x5a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -23148;
	// 826F7350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F7354: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7358: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F735C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7360: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F7364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7368: 4BD6FAB9  bl 0x82466e20
	ctx.lr = 0x826F736C;
	sub_82466E20(ctx, base);
	// 826F736C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F7374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7380 size=104
    let mut pc: u32 = 0x826F7380;
    'dispatch: loop {
        match pc {
            0x826F7380 => {
    //   block [0x826F7380..0x826F73E8)
	// 826F7380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F7384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F738C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 826F7390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7394: 392AA7EC  addi r9, r10, -0x5814
	ctx.r[9].s64 = ctx.r[10].s64 + -22548;
	// 826F7398: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F739C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F73A0: 38AAA534  addi r5, r10, -0x5acc
	ctx.r[5].s64 = ctx.r[10].s64 + -23244;
	// 826F73A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F73A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F73AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F73B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F73B4: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826F73B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F73BC: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F73C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826F73C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F73C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826F73CC: 386AA5C4  addi r3, r10, -0x5a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -23100;
	// 826F73D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826F73D4: 4BD6FA4D  bl 0x82466e20
	ctx.lr = 0x826F73D8;
	sub_82466E20(ctx, base);
	// 826F73D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F73DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F73E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F73E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F73E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F73E8 size=108
    let mut pc: u32 = 0x826F73E8;
    'dispatch: loop {
        match pc {
            0x826F73E8 => {
    //   block [0x826F73E8..0x826F7454)
	// 826F73E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F73EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F73F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F73F4: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F73F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F73FC: 38EBF22C  addi r7, r11, -0xdd4
	ctx.r[7].s64 = ctx.r[11].s64 + -3540;
	// 826F7400: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826F7404: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 826F7408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F740C: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7410: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826F7414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F7418: 386AA5F4  addi r3, r10, -0x5a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -23052;
	// 826F741C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826F7420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F7424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F7428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F742C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F7434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F7438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F743C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826F7440: 4BD6F9E1  bl 0x82466e20
	ctx.lr = 0x826F7444;
	sub_82466E20(ctx, base);
	// 826F7444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F7448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F744C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F7450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F7458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826F7458 size=112
    let mut pc: u32 = 0x826F7458;
    'dispatch: loop {
        match pc {
            0x826F7458 => {
    //   block [0x826F7458..0x826F74C8)
	// 826F7458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826F745C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826F7460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826F7464: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7468: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F746C: 38AAA5C4  addi r5, r10, -0x5a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -23100;
	// 826F7470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826F7474: 390BF260  addi r8, r11, -0xda0
	ctx.r[8].s64 = ctx.r[11].s64 + -3488;
	// 826F7478: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826F747C: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826F7480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826F7484: 3D40829A  lis r10, -0x7d66
	ctx.r[10].s64 = -2103836672;
	// 826F7488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826F748C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826F7490: 386AA624  addi r3, r10, -0x59dc
	ctx.r[3].s64 = ctx.r[10].s64 + -23004;
	// 826F7494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826F7498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826F749C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826F74A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826F74A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826F74A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826F74AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826F74B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826F74B4: 4BD6F96D  bl 0x82466e20
	ctx.lr = 0x826F74B8;
	sub_82466E20(ctx, base);
	// 826F74B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826F74BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826F74C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826F74C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826F74C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826F74C8 size=24
    let mut pc: u32 = 0x826F74C8;
    'dispatch: loop {
        match pc {
            0x826F74C8 => {
    //   block [0x826F74C8..0x826F74E0)
	// 826F74C8: 3D608281  lis r11, -0x7d7f
	ctx.r[11].s64 = -2105475072;
	// 826F74CC: 3D408281  lis r10, -0x7d7f
	ctx.r[10].s64 = -2105475072;
	// 826F74D0: 394A25A0  addi r10, r10, 0x25a0
	ctx.r[10].s64 = ctx.r[10].s64 + 9632;
	// 826F74D4: 816BF25C  lwz r11, -0xda4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3492 as u32) ) } as u64;
	// 826F74D8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826F74DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


