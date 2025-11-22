pub fn sub_82615FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82615FE0 size=100
    let mut pc: u32 = 0x82615FE0;
    'dispatch: loop {
        match pc {
            0x82615FE0 => {
    //   block [0x82615FE0..0x82616044)
	// 82615FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82615FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82615FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82615FEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82615FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82615FF4: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 82615FF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82615FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616000: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82616004: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261600C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616014: 386AB72C  addi r3, r10, -0x48d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18644;
	// 82616018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261601C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616020: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82616024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616028: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261602C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616030: 4BE50DF1  bl 0x82466e20
	ctx.lr = 0x82616034;
	sub_82466E20(ctx, base);
	// 82616034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261603C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616048 size=96
    let mut pc: u32 = 0x82616048;
    'dispatch: loop {
        match pc {
            0x82616048 => {
    //   block [0x82616048..0x826160A8)
	// 82616048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261604C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616054: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261605C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82616060: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616068: 386AB75C  addi r3, r10, -0x48a4
	ctx.r[3].s64 = ctx.r[10].s64 + -18596;
	// 8261606C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616074: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261607C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616088: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261608C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616090: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616094: 4BE50D8D  bl 0x82466e20
	ctx.lr = 0x82616098;
	sub_82466E20(ctx, base);
	// 82616098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261609C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826160A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826160A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826160A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826160A8 size=112
    let mut pc: u32 = 0x826160A8;
    'dispatch: loop {
        match pc {
            0x826160A8 => {
    //   block [0x826160A8..0x82616118)
	// 826160A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826160AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826160B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826160B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826160B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826160BC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826160C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826160C4: 390B8D28  addi r8, r11, -0x72d8
	ctx.r[8].s64 = ctx.r[11].s64 + -29400;
	// 826160C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826160CC: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826160D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826160D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826160D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826160DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826160E0: 386AB78C  addi r3, r10, -0x4874
	ctx.r[3].s64 = ctx.r[10].s64 + -18548;
	// 826160E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826160E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826160EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826160F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826160F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826160F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826160FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616104: 4BE50D1D  bl 0x82466e20
	ctx.lr = 0x82616108;
	sub_82466E20(ctx, base);
	// 82616108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261610C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616118 size=96
    let mut pc: u32 = 0x82616118;
    'dispatch: loop {
        match pc {
            0x82616118 => {
    //   block [0x82616118..0x82616178)
	// 82616118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261611C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616124: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261612C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82616130: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616138: 386AB7BC  addi r3, r10, -0x4844
	ctx.r[3].s64 = ctx.r[10].s64 + -18500;
	// 8261613C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616144: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261614C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616158: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261615C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616160: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616164: 4BE50CBD  bl 0x82466e20
	ctx.lr = 0x82616168;
	sub_82466E20(ctx, base);
	// 82616168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261616C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616178 size=112
    let mut pc: u32 = 0x82616178;
    'dispatch: loop {
        match pc {
            0x82616178 => {
    //   block [0x82616178..0x826161E8)
	// 82616178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261617C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616184: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616188: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261618C: 38AAB7BC  addi r5, r10, -0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + -18500;
	// 82616190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616194: 390B8D58  addi r8, r11, -0x72a8
	ctx.r[8].s64 = ctx.r[11].s64 + -29352;
	// 82616198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261619C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826161A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826161A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826161A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826161AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826161B0: 386AB7EC  addi r3, r10, -0x4814
	ctx.r[3].s64 = ctx.r[10].s64 + -18452;
	// 826161B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826161B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826161BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826161C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826161C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826161C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826161CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826161D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826161D4: 4BE50C4D  bl 0x82466e20
	ctx.lr = 0x826161D8;
	sub_82466E20(ctx, base);
	// 826161D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826161DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826161E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826161E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826161E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826161E8 size=108
    let mut pc: u32 = 0x826161E8;
    'dispatch: loop {
        match pc {
            0x826161E8 => {
    //   block [0x826161E8..0x82616254)
	// 826161E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826161EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826161F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826161F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826161F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826161FC: 38EB8D70  addi r7, r11, -0x7290
	ctx.r[7].s64 = ctx.r[11].s64 + -29328;
	// 82616200: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82616204: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 82616208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261620C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82616214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616218: 386AB81C  addi r3, r10, -0x47e4
	ctx.r[3].s64 = ctx.r[10].s64 + -18404;
	// 8261621C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261622C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261623C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616240: 4BE50BE1  bl 0x82466e20
	ctx.lr = 0x82616244;
	sub_82466E20(ctx, base);
	// 82616244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261624C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616258 size=112
    let mut pc: u32 = 0x82616258;
    'dispatch: loop {
        match pc {
            0x82616258 => {
    //   block [0x82616258..0x826162C8)
	// 82616258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261625C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616264: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616268: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261626C: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 82616270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616274: 390B8DD0  addi r8, r11, -0x7230
	ctx.r[8].s64 = ctx.r[11].s64 + -29232;
	// 82616278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261627C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 82616280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616284: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261628C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616290: 386AB84C  addi r3, r10, -0x47b4
	ctx.r[3].s64 = ctx.r[10].s64 + -18356;
	// 82616294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261629C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826162A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826162A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826162A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826162AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826162B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826162B4: 4BE50B6D  bl 0x82466e20
	ctx.lr = 0x826162B8;
	sub_82466E20(ctx, base);
	// 826162B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826162BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826162C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826162C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826162C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826162C8 size=112
    let mut pc: u32 = 0x826162C8;
    'dispatch: loop {
        match pc {
            0x826162C8 => {
    //   block [0x826162C8..0x82616338)
	// 826162C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826162CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826162D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826162D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826162D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826162DC: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 826162E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826162E4: 390B8DE8  addi r8, r11, -0x7218
	ctx.r[8].s64 = ctx.r[11].s64 + -29208;
	// 826162E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826162EC: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826162F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826162F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826162F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826162FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616300: 386AB87C  addi r3, r10, -0x4784
	ctx.r[3].s64 = ctx.r[10].s64 + -18308;
	// 82616304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261630C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261631C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616324: 4BE50AFD  bl 0x82466e20
	ctx.lr = 0x82616328;
	sub_82466E20(ctx, base);
	// 82616328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261632C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616338 size=100
    let mut pc: u32 = 0x82616338;
    'dispatch: loop {
        match pc {
            0x82616338 => {
    //   block [0x82616338..0x8261639C)
	// 82616338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261633C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616344: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261634C: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82616350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616358: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8261635C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261636C: 386AB8AC  addi r3, r10, -0x4754
	ctx.r[3].s64 = ctx.r[10].s64 + -18260;
	// 82616370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616374: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616378: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261637C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616380: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616388: 4BE50A99  bl 0x82466e20
	ctx.lr = 0x8261638C;
	sub_82466E20(ctx, base);
	// 8261638C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826163A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826163A0 size=116
    let mut pc: u32 = 0x826163A0;
    'dispatch: loop {
        match pc {
            0x826163A0 => {
    //   block [0x826163A0..0x82616414)
	// 826163A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826163A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826163A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826163AC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826163B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826163B4: 390B8E1C  addi r8, r11, -0x71e4
	ctx.r[8].s64 = ctx.r[11].s64 + -29156;
	// 826163B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826163BC: 392AF928  addi r9, r10, -0x6d8
	ctx.r[9].s64 = ctx.r[10].s64 + -1752;
	// 826163C0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826163C4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826163C8: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 826163CC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826163D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826163D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826163D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826163DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826163E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826163E4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826163E8: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826163EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826163F0: 386BB8DC  addi r3, r11, -0x4724
	ctx.r[3].s64 = ctx.r[11].s64 + -18212;
	// 826163F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826163F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826163FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616400: 4BE50A21  bl 0x82466e20
	ctx.lr = 0x82616404;
	sub_82466E20(ctx, base);
	// 82616404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261640C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616418 size=112
    let mut pc: u32 = 0x82616418;
    'dispatch: loop {
        match pc {
            0x82616418 => {
    //   block [0x82616418..0x82616488)
	// 82616418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261641C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616428: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261642C: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82616430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616434: 390B8E4C  addi r8, r11, -0x71b4
	ctx.r[8].s64 = ctx.r[11].s64 + -29108;
	// 82616438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261643C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 82616440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616444: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261644C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616450: 386AB90C  addi r3, r10, -0x46f4
	ctx.r[3].s64 = ctx.r[10].s64 + -18164;
	// 82616454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261645C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616464: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261646C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616474: 4BE509AD  bl 0x82466e20
	ctx.lr = 0x82616478;
	sub_82466E20(ctx, base);
	// 82616478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261647C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616488 size=116
    let mut pc: u32 = 0x82616488;
    'dispatch: loop {
        match pc {
            0x82616488 => {
    //   block [0x82616488..0x826164FC)
	// 82616488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261648C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616494: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616498: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261649C: 390B8E68  addi r8, r11, -0x7198
	ctx.r[8].s64 = ctx.r[11].s64 + -29080;
	// 826164A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826164A4: 392AF954  addi r9, r10, -0x6ac
	ctx.r[9].s64 = ctx.r[10].s64 + -1708;
	// 826164A8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826164AC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826164B0: 38AABF9C  addi r5, r10, -0x4064
	ctx.r[5].s64 = ctx.r[10].s64 + -16484;
	// 826164B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826164B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826164BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826164C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826164C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826164C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826164CC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826164D0: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826164D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826164D8: 386BB93C  addi r3, r11, -0x46c4
	ctx.r[3].s64 = ctx.r[11].s64 + -18116;
	// 826164DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826164E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826164E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826164E8: 4BE50939  bl 0x82466e20
	ctx.lr = 0x826164EC;
	sub_82466E20(ctx, base);
	// 826164EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826164F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826164F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826164F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616500 size=112
    let mut pc: u32 = 0x82616500;
    'dispatch: loop {
        match pc {
            0x82616500 => {
    //   block [0x82616500..0x82616570)
	// 82616500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261650C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616510: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616514: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 82616518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261651C: 390B8E80  addi r8, r11, -0x7180
	ctx.r[8].s64 = ctx.r[11].s64 + -29056;
	// 82616520: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82616524: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 82616528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261652C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616538: 386AB96C  addi r3, r10, -0x4694
	ctx.r[3].s64 = ctx.r[10].s64 + -18068;
	// 8261653C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261654C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261655C: 4BE508C5  bl 0x82466e20
	ctx.lr = 0x82616560;
	sub_82466E20(ctx, base);
	// 82616560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261656C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616570 size=112
    let mut pc: u32 = 0x82616570;
    'dispatch: loop {
        match pc {
            0x82616570 => {
    //   block [0x82616570..0x826165E0)
	// 82616570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261657C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616580: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616584: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 82616588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261658C: 390B8EF8  addi r8, r11, -0x7108
	ctx.r[8].s64 = ctx.r[11].s64 + -28936;
	// 82616590: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82616594: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 82616598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261659C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826165A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826165A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826165A8: 386AB99C  addi r3, r10, -0x4664
	ctx.r[3].s64 = ctx.r[10].s64 + -18020;
	// 826165AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826165B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826165B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826165B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826165BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826165C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826165C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826165C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826165CC: 4BE50855  bl 0x82466e20
	ctx.lr = 0x826165D0;
	sub_82466E20(ctx, base);
	// 826165D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826165D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826165D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826165DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826165E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826165E0 size=112
    let mut pc: u32 = 0x826165E0;
    'dispatch: loop {
        match pc {
            0x826165E0 => {
    //   block [0x826165E0..0x82616650)
	// 826165E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826165E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826165E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826165EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826165F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826165F4: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 826165F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826165FC: 390B8F40  addi r8, r11, -0x70c0
	ctx.r[8].s64 = ctx.r[11].s64 + -28864;
	// 82616600: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82616604: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 82616608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261660C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616618: 386AB9CC  addi r3, r10, -0x4634
	ctx.r[3].s64 = ctx.r[10].s64 + -17972;
	// 8261661C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261662C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261663C: 4BE507E5  bl 0x82466e20
	ctx.lr = 0x82616640;
	sub_82466E20(ctx, base);
	// 82616640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261664C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616650 size=112
    let mut pc: u32 = 0x82616650;
    'dispatch: loop {
        match pc {
            0x82616650 => {
    //   block [0x82616650..0x826166C0)
	// 82616650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261665C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616660: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616664: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 82616668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261666C: 390B8F88  addi r8, r11, -0x7078
	ctx.r[8].s64 = ctx.r[11].s64 + -28792;
	// 82616670: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82616674: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 82616678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261667C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616688: 386AB9FC  addi r3, r10, -0x4604
	ctx.r[3].s64 = ctx.r[10].s64 + -17924;
	// 8261668C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261669C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826166A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826166A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826166A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826166AC: 4BE50775  bl 0x82466e20
	ctx.lr = 0x826166B0;
	sub_82466E20(ctx, base);
	// 826166B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826166B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826166B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826166BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826166C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826166C0 size=108
    let mut pc: u32 = 0x826166C0;
    'dispatch: loop {
        match pc {
            0x826166C0 => {
    //   block [0x826166C0..0x8261672C)
	// 826166C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826166C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826166C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826166CC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826166D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826166D4: 38EB8FD0  addi r7, r11, -0x7030
	ctx.r[7].s64 = ctx.r[11].s64 + -28720;
	// 826166D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826166DC: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826166E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826166E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826166E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826166EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826166F0: 386ABA2C  addi r3, r10, -0x45d4
	ctx.r[3].s64 = ctx.r[10].s64 + -17876;
	// 826166F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826166F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826166FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261670C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616718: 4BE50709  bl 0x82466e20
	ctx.lr = 0x8261671C;
	sub_82466E20(ctx, base);
	// 8261671C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616730 size=112
    let mut pc: u32 = 0x82616730;
    'dispatch: loop {
        match pc {
            0x82616730 => {
    //   block [0x82616730..0x826167A0)
	// 82616730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261673C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616740: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616744: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 82616748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261674C: 390B9018  addi r8, r11, -0x6fe8
	ctx.r[8].s64 = ctx.r[11].s64 + -28648;
	// 82616750: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82616754: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 82616758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261675C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616768: 386ABA5C  addi r3, r10, -0x45a4
	ctx.r[3].s64 = ctx.r[10].s64 + -17828;
	// 8261676C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261677C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261678C: 4BE50695  bl 0x82466e20
	ctx.lr = 0x82616790;
	sub_82466E20(ctx, base);
	// 82616790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261679C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826167A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826167A0 size=116
    let mut pc: u32 = 0x826167A0;
    'dispatch: loop {
        match pc {
            0x826167A0 => {
    //   block [0x826167A0..0x82616814)
	// 826167A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826167A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826167A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826167AC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826167B0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826167B4: 392BF990  addi r9, r11, -0x670
	ctx.r[9].s64 = ctx.r[11].s64 + -1648;
	// 826167B8: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 826167BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826167C0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826167C4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826167C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826167CC: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826167D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826167D4: 396B9098  addi r11, r11, -0x6f68
	ctx.r[11].s64 = ctx.r[11].s64 + -28520;
	// 826167D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826167DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826167E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826167E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826167E8: 386ABA8C  addi r3, r10, -0x4574
	ctx.r[3].s64 = ctx.r[10].s64 + -17780;
	// 826167EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826167F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826167F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826167F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826167FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616800: 4BE50621  bl 0x82466e20
	ctx.lr = 0x82616804;
	sub_82466E20(ctx, base);
	// 82616804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261680C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82616818 size=36
    let mut pc: u32 = 0x82616818;
    'dispatch: loop {
        match pc {
            0x82616818 => {
    //   block [0x82616818..0x8261683C)
	// 82616818: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261681C: 814B912C  lwz r10, -0x6ed4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28372 as u32) ) } as u64;
	// 82616820: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616824: 396BDB10  addi r11, r11, -0x24f0
	ctx.r[11].s64 = ctx.r[11].s64 + -9456;
	// 82616828: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8261682C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616830: 814A9094  lwz r10, -0x6f6c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28524 as u32) ) } as u64;
	// 82616834: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82616838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616840 size=108
    let mut pc: u32 = 0x82616840;
    'dispatch: loop {
        match pc {
            0x82616840 => {
    //   block [0x82616840..0x826168AC)
	// 82616840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261684C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616854: 38EBDB10  addi r7, r11, -0x24f0
	ctx.r[7].s64 = ctx.r[11].s64 + -9456;
	// 82616858: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8261685C: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 82616860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616864: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261686C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616870: 386ABABC  addi r3, r10, -0x4544
	ctx.r[3].s64 = ctx.r[10].s64 + -17732;
	// 82616874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261687C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261688C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616898: 4BE50589  bl 0x82466e20
	ctx.lr = 0x8261689C;
	sub_82466E20(ctx, base);
	// 8261689C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826168A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826168A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826168A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826168B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826168B0 size=24
    let mut pc: u32 = 0x826168B0;
    'dispatch: loop {
        match pc {
            0x826168B0 => {
    //   block [0x826168B0..0x826168C8)
	// 826168B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826168B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826168B8: 394ADBB8  addi r10, r10, -0x2448
	ctx.r[10].s64 = ctx.r[10].s64 + -9288;
	// 826168BC: 816B9094  lwz r11, -0x6f6c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28524 as u32) ) } as u64;
	// 826168C0: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826168C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826168C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826168C8 size=116
    let mut pc: u32 = 0x826168C8;
    'dispatch: loop {
        match pc {
            0x826168C8 => {
    //   block [0x826168C8..0x8261693C)
	// 826168C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826168CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826168D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826168D4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826168D8: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826168DC: 390ADBB8  addi r8, r10, -0x2448
	ctx.r[8].s64 = ctx.r[10].s64 + -9288;
	// 826168E0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826168E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826168E8: 38AABABC  addi r5, r10, -0x4544
	ctx.r[5].s64 = ctx.r[10].s64 + -17732;
	// 826168EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826168F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826168F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826168F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826168FC: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 82616900: 396BFA4C  addi r11, r11, -0x5b4
	ctx.r[11].s64 = ctx.r[11].s64 + -1460;
	// 82616904: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616908: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261690C: 386ABAEC  addi r3, r10, -0x4514
	ctx.r[3].s64 = ctx.r[10].s64 + -17684;
	// 82616910: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82616914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616918: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261691C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616928: 4BE504F9  bl 0x82466e20
	ctx.lr = 0x8261692C;
	sub_82466E20(ctx, base);
	// 8261692C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616940 size=112
    let mut pc: u32 = 0x82616940;
    'dispatch: loop {
        match pc {
            0x82616940 => {
    //   block [0x82616940..0x826169B0)
	// 82616940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261694C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616950: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616954: 38AABABC  addi r5, r10, -0x4544
	ctx.r[5].s64 = ctx.r[10].s64 + -17732;
	// 82616958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261695C: 390B9130  addi r8, r11, -0x6ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -28368;
	// 82616960: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82616964: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 82616968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261696C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616978: 386ABB1C  addi r3, r10, -0x44e4
	ctx.r[3].s64 = ctx.r[10].s64 + -17636;
	// 8261697C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261698C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261699C: 4BE50485  bl 0x82466e20
	ctx.lr = 0x826169A0;
	sub_82466E20(ctx, base);
	// 826169A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826169A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826169A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826169AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826169B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826169B0 size=24
    let mut pc: u32 = 0x826169B0;
    'dispatch: loop {
        match pc {
            0x826169B0 => {
    //   block [0x826169B0..0x826169C8)
	// 826169B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826169B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826169B8: 394ADCA8  addi r10, r10, -0x2358
	ctx.r[10].s64 = ctx.r[10].s64 + -9048;
	// 826169BC: 816B98E8  lwz r11, -0x6718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26392 as u32) ) } as u64;
	// 826169C0: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826169C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826169C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826169C8 size=116
    let mut pc: u32 = 0x826169C8;
    'dispatch: loop {
        match pc {
            0x826169C8 => {
    //   block [0x826169C8..0x82616A3C)
	// 826169C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826169CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826169D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826169D4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826169D8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826169DC: 392BFA10  addi r9, r11, -0x5f0
	ctx.r[9].s64 = ctx.r[11].s64 + -1520;
	// 826169E0: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 826169E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826169E8: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826169EC: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 826169F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826169F4: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 826169F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826169FC: 396BDCA8  addi r11, r11, -0x2358
	ctx.r[11].s64 = ctx.r[11].s64 + -9048;
	// 82616A00: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82616A04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616A08: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82616A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616A10: 386ABB4C  addi r3, r10, -0x44b4
	ctx.r[3].s64 = ctx.r[10].s64 + -17588;
	// 82616A14: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82616A18: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82616A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616A20: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82616A24: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616A28: 4BE503F9  bl 0x82466e20
	ctx.lr = 0x82616A2C;
	sub_82466E20(ctx, base);
	// 82616A2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616A40 size=100
    let mut pc: u32 = 0x82616A40;
    'dispatch: loop {
        match pc {
            0x82616A40 => {
    //   block [0x82616A40..0x82616AA4)
	// 82616A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616A54: 38AABC9C  addi r5, r10, -0x4364
	ctx.r[5].s64 = ctx.r[10].s64 + -17252;
	// 82616A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616A60: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 82616A64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616A74: 386ABB7C  addi r3, r10, -0x4484
	ctx.r[3].s64 = ctx.r[10].s64 + -17540;
	// 82616A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616A7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616A80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82616A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616A88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616A90: 4BE50391  bl 0x82466e20
	ctx.lr = 0x82616A94;
	sub_82466E20(ctx, base);
	// 82616A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616AA8 size=100
    let mut pc: u32 = 0x82616AA8;
    'dispatch: loop {
        match pc {
            0x82616AA8 => {
    //   block [0x82616AA8..0x82616B0C)
	// 82616AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616AB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616ABC: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82616AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616AC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616AC8: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 82616ACC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616ADC: 386ABBAC  addi r3, r10, -0x4454
	ctx.r[3].s64 = ctx.r[10].s64 + -17492;
	// 82616AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616AE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616AE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82616AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616AF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616AF8: 4BE50329  bl 0x82466e20
	ctx.lr = 0x82616AFC;
	sub_82466E20(ctx, base);
	// 82616AFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616B00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616B04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616B10 size=108
    let mut pc: u32 = 0x82616B10;
    'dispatch: loop {
        match pc {
            0x82616B10 => {
    //   block [0x82616B10..0x82616B7C)
	// 82616B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616B1C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616B20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616B24: 38EB91A8  addi r7, r11, -0x6e58
	ctx.r[7].s64 = ctx.r[11].s64 + -28248;
	// 82616B28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82616B2C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 82616B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616B34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616B38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82616B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616B40: 386ABBDC  addi r3, r10, -0x4424
	ctx.r[3].s64 = ctx.r[10].s64 + -17444;
	// 82616B44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616B48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616B50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616B58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616B60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616B68: 4BE502B9  bl 0x82466e20
	ctx.lr = 0x82616B6C;
	sub_82466E20(ctx, base);
	// 82616B6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616B70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616B74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616B78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616B80 size=112
    let mut pc: u32 = 0x82616B80;
    'dispatch: loop {
        match pc {
            0x82616B80 => {
    //   block [0x82616B80..0x82616BF0)
	// 82616B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616B8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616B90: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616B94: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 82616B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616B9C: 390B9208  addi r8, r11, -0x6df8
	ctx.r[8].s64 = ctx.r[11].s64 + -28152;
	// 82616BA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82616BA4: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82616BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616BAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616BB8: 386ABC0C  addi r3, r10, -0x43f4
	ctx.r[3].s64 = ctx.r[10].s64 + -17396;
	// 82616BBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616BDC: 4BE50245  bl 0x82466e20
	ctx.lr = 0x82616BE0;
	sub_82466E20(ctx, base);
	// 82616BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616BF0 size=108
    let mut pc: u32 = 0x82616BF0;
    'dispatch: loop {
        match pc {
            0x82616BF0 => {
    //   block [0x82616BF0..0x82616C5C)
	// 82616BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616BFC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616C04: 38EB9268  addi r7, r11, -0x6d98
	ctx.r[7].s64 = ctx.r[11].s64 + -28056;
	// 82616C08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616C0C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 82616C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616C14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616C18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82616C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616C20: 386ABC3C  addi r3, r10, -0x43c4
	ctx.r[3].s64 = ctx.r[10].s64 + -17348;
	// 82616C24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616C48: 4BE501D9  bl 0x82466e20
	ctx.lr = 0x82616C4C;
	sub_82466E20(ctx, base);
	// 82616C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82616C60 size=28
    let mut pc: u32 = 0x82616C60;
    'dispatch: loop {
        match pc {
            0x82616C60 => {
    //   block [0x82616C60..0x82616C7C)
	// 82616C60: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616C64: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616C68: 394ADD98  addi r10, r10, -0x2268
	ctx.r[10].s64 = ctx.r[10].s64 + -8808;
	// 82616C6C: 816B9280  lwz r11, -0x6d80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28032 as u32) ) } as u64;
	// 82616C70: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82616C74: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82616C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616C80 size=112
    let mut pc: u32 = 0x82616C80;
    'dispatch: loop {
        match pc {
            0x82616C80 => {
    //   block [0x82616C80..0x82616CF0)
	// 82616C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616C8C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616C90: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 82616C94: 38EADD98  addi r7, r10, -0x2268
	ctx.r[7].s64 = ctx.r[10].s64 + -8808;
	// 82616C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616C9C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82616CA0: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82616CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616CA8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616CAC: 396BFB10  addi r11, r11, -0x4f0
	ctx.r[11].s64 = ctx.r[11].s64 + -1264;
	// 82616CB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82616CB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616CB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616CBC: 386ABC6C  addi r3, r10, -0x4394
	ctx.r[3].s64 = ctx.r[10].s64 + -17300;
	// 82616CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616CC4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82616CC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616CCC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82616CD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616CD4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616CD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616CDC: 4BE50145  bl 0x82466e20
	ctx.lr = 0x82616CE0;
	sub_82466E20(ctx, base);
	// 82616CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82616CF0 size=24
    let mut pc: u32 = 0x82616CF0;
    'dispatch: loop {
        match pc {
            0x82616CF0 => {
    //   block [0x82616CF0..0x82616D08)
	// 82616CF0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616CF4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616CF8: 394ADF00  addi r10, r10, -0x2100
	ctx.r[10].s64 = ctx.r[10].s64 + -8448;
	// 82616CFC: 816B98E8  lwz r11, -0x6718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26392 as u32) ) } as u64;
	// 82616D00: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82616D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616D08 size=116
    let mut pc: u32 = 0x82616D08;
    'dispatch: loop {
        match pc {
            0x82616D08 => {
    //   block [0x82616D08..0x82616D7C)
	// 82616D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616D14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82616D18: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616D1C: 392BFAE4  addi r9, r11, -0x51c
	ctx.r[9].s64 = ctx.r[11].s64 + -1308;
	// 82616D20: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 82616D24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616D28: 38E9006C  addi r7, r9, 0x6c
	ctx.r[7].s64 = ctx.r[9].s64 + 108;
	// 82616D2C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82616D30: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616D34: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 82616D38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616D3C: 396BDF00  addi r11, r11, -0x2100
	ctx.r[11].s64 = ctx.r[11].s64 + -8448;
	// 82616D40: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82616D44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616D48: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82616D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616D50: 386ABC9C  addi r3, r10, -0x4364
	ctx.r[3].s64 = ctx.r[10].s64 + -17252;
	// 82616D54: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82616D58: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82616D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616D60: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82616D64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82616D68: 4BE500B9  bl 0x82466e20
	ctx.lr = 0x82616D6C;
	sub_82466E20(ctx, base);
	// 82616D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616D80 size=112
    let mut pc: u32 = 0x82616D80;
    'dispatch: loop {
        match pc {
            0x82616D80 => {
    //   block [0x82616D80..0x82616DF0)
	// 82616D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616D8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616D90: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616D94: 38AAB8AC  addi r5, r10, -0x4754
	ctx.r[5].s64 = ctx.r[10].s64 + -18260;
	// 82616D98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616D9C: 390B9288  addi r8, r11, -0x6d78
	ctx.r[8].s64 = ctx.r[11].s64 + -28024;
	// 82616DA0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82616DA4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82616DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616DAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616DB8: 386ABCCC  addi r3, r10, -0x4334
	ctx.r[3].s64 = ctx.r[10].s64 + -17204;
	// 82616DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616DDC: 4BE50045  bl 0x82466e20
	ctx.lr = 0x82616DE0;
	sub_82466E20(ctx, base);
	// 82616DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82616DF0 size=24
    let mut pc: u32 = 0x82616DF0;
    'dispatch: loop {
        match pc {
            0x82616DF0 => {
    //   block [0x82616DF0..0x82616E08)
	// 82616DF0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616DF4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616DF8: 394ADFA8  addi r10, r10, -0x2058
	ctx.r[10].s64 = ctx.r[10].s64 + -8280;
	// 82616DFC: 816B98E8  lwz r11, -0x6718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26392 as u32) ) } as u64;
	// 82616E00: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82616E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616E08 size=116
    let mut pc: u32 = 0x82616E08;
    'dispatch: loop {
        match pc {
            0x82616E08 => {
    //   block [0x82616E08..0x82616E7C)
	// 82616E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616E14: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82616E18: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82616E1C: 390ADFA8  addi r8, r10, -0x2058
	ctx.r[8].s64 = ctx.r[10].s64 + -8280;
	// 82616E20: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616E24: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82616E28: 38AAB8AC  addi r5, r10, -0x4754
	ctx.r[5].s64 = ctx.r[10].s64 + -18260;
	// 82616E2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616E30: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82616E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616E3C: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 82616E40: 396BFB70  addi r11, r11, -0x490
	ctx.r[11].s64 = ctx.r[11].s64 + -1168;
	// 82616E44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616E48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616E4C: 386ABCFC  addi r3, r10, -0x4304
	ctx.r[3].s64 = ctx.r[10].s64 + -17156;
	// 82616E50: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82616E54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616E58: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82616E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616E68: 4BE4FFB9  bl 0x82466e20
	ctx.lr = 0x82616E6C;
	sub_82466E20(ctx, base);
	// 82616E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616E80 size=112
    let mut pc: u32 = 0x82616E80;
    'dispatch: loop {
        match pc {
            0x82616E80 => {
    //   block [0x82616E80..0x82616EF0)
	// 82616E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616E8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616E90: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616E94: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82616E98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616E9C: 390B9318  addi r8, r11, -0x6ce8
	ctx.r[8].s64 = ctx.r[11].s64 + -27880;
	// 82616EA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82616EA4: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 82616EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616EAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616EB8: 386ABD2C  addi r3, r10, -0x42d4
	ctx.r[3].s64 = ctx.r[10].s64 + -17108;
	// 82616EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616ECC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82616ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616EDC: 4BE4FF45  bl 0x82466e20
	ctx.lr = 0x82616EE0;
	sub_82466E20(ctx, base);
	// 82616EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616EF0 size=108
    let mut pc: u32 = 0x82616EF0;
    'dispatch: loop {
        match pc {
            0x82616EF0 => {
    //   block [0x82616EF0..0x82616F5C)
	// 82616EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616EFC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616F04: 38EB9348  addi r7, r11, -0x6cb8
	ctx.r[7].s64 = ctx.r[11].s64 + -27832;
	// 82616F08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82616F0C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 82616F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82616F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616F20: 386ABD5C  addi r3, r10, -0x42a4
	ctx.r[3].s64 = ctx.r[10].s64 + -17060;
	// 82616F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82616F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82616F48: 4BE4FED9  bl 0x82466e20
	ctx.lr = 0x82616F4C;
	sub_82466E20(ctx, base);
	// 82616F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616F60 size=112
    let mut pc: u32 = 0x82616F60;
    'dispatch: loop {
        match pc {
            0x82616F60 => {
    //   block [0x82616F60..0x82616FD0)
	// 82616F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616F6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616F70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616F74: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82616F78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616F7C: 390B9378  addi r8, r11, -0x6c88
	ctx.r[8].s64 = ctx.r[11].s64 + -27784;
	// 82616F80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82616F84: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82616F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616F8C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616F90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82616F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82616F98: 386ABD8C  addi r3, r10, -0x4274
	ctx.r[3].s64 = ctx.r[10].s64 + -17012;
	// 82616F9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82616FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82616FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82616FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82616FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82616FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82616FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82616FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82616FBC: 4BE4FE65  bl 0x82466e20
	ctx.lr = 0x82616FC0;
	sub_82466E20(ctx, base);
	// 82616FC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82616FC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82616FC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82616FCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82616FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82616FD0 size=112
    let mut pc: u32 = 0x82616FD0;
    'dispatch: loop {
        match pc {
            0x82616FD0 => {
    //   block [0x82616FD0..0x82617040)
	// 82616FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82616FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82616FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82616FDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82616FE0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82616FE4: 38AABF9C  addi r5, r10, -0x4064
	ctx.r[5].s64 = ctx.r[10].s64 + -16484;
	// 82616FE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82616FEC: 390B93A8  addi r8, r11, -0x6c58
	ctx.r[8].s64 = ctx.r[11].s64 + -27736;
	// 82616FF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82616FF4: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82616FF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82616FFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617000: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617008: 386ABDBC  addi r3, r10, -0x4244
	ctx.r[3].s64 = ctx.r[10].s64 + -16964;
	// 8261700C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261701C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261702C: 4BE4FDF5  bl 0x82466e20
	ctx.lr = 0x82617030;
	sub_82466E20(ctx, base);
	// 82617030: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261703C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617040 size=108
    let mut pc: u32 = 0x82617040;
    'dispatch: loop {
        match pc {
            0x82617040 => {
    //   block [0x82617040..0x826170AC)
	// 82617040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261704C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617054: 38EB93D8  addi r7, r11, -0x6c28
	ctx.r[7].s64 = ctx.r[11].s64 + -27688;
	// 82617058: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261705C: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 82617060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617064: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617068: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261706C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617070: 386ABDEC  addi r3, r10, -0x4214
	ctx.r[3].s64 = ctx.r[10].s64 + -16916;
	// 82617074: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82617078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261707C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261708C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617098: 4BE4FD89  bl 0x82466e20
	ctx.lr = 0x8261709C;
	sub_82466E20(ctx, base);
	// 8261709C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826170A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826170A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826170A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826170B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826170B0 size=108
    let mut pc: u32 = 0x826170B0;
    'dispatch: loop {
        match pc {
            0x826170B0 => {
    //   block [0x826170B0..0x8261711C)
	// 826170B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826170B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826170B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826170BC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826170C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826170C4: 38EB9420  addi r7, r11, -0x6be0
	ctx.r[7].s64 = ctx.r[11].s64 + -27616;
	// 826170C8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826170CC: 388A0DDC  addi r4, r10, 0xddc
	ctx.r[4].s64 = ctx.r[10].s64 + 3548;
	// 826170D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826170D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826170D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826170DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826170E0: 386ABE1C  addi r3, r10, -0x41e4
	ctx.r[3].s64 = ctx.r[10].s64 + -16868;
	// 826170E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826170E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826170EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826170F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826170F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826170F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826170FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617108: 4BE4FD19  bl 0x82466e20
	ctx.lr = 0x8261710C;
	sub_82466E20(ctx, base);
	// 8261710C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617120 size=112
    let mut pc: u32 = 0x82617120;
    'dispatch: loop {
        match pc {
            0x82617120 => {
    //   block [0x82617120..0x82617190)
	// 82617120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261712C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617130: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617134: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 82617138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261713C: 390B9480  addi r8, r11, -0x6b80
	ctx.r[8].s64 = ctx.r[11].s64 + -27520;
	// 82617140: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82617144: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 82617148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261714C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617158: 386ABE4C  addi r3, r10, -0x41b4
	ctx.r[3].s64 = ctx.r[10].s64 + -16820;
	// 8261715C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261716C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261717C: 4BE4FCA5  bl 0x82466e20
	ctx.lr = 0x82617180;
	sub_82466E20(ctx, base);
	// 82617180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261718C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617190 size=100
    let mut pc: u32 = 0x82617190;
    'dispatch: loop {
        match pc {
            0x82617190 => {
    //   block [0x82617190..0x826171F4)
	// 82617190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261719C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826171A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826171A4: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 826171A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826171AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826171B0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826171B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826171B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826171BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826171C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826171C4: 386ABE7C  addi r3, r10, -0x4184
	ctx.r[3].s64 = ctx.r[10].s64 + -16772;
	// 826171C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826171CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826171D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826171D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826171D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826171DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826171E0: 4BE4FC41  bl 0x82466e20
	ctx.lr = 0x826171E4;
	sub_82466E20(ctx, base);
	// 826171E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826171E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826171EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826171F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826171F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826171F8 size=112
    let mut pc: u32 = 0x826171F8;
    'dispatch: loop {
        match pc {
            0x826171F8 => {
    //   block [0x826171F8..0x82617268)
	// 826171F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826171FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617204: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617208: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261720C: 38AABBAC  addi r5, r10, -0x4454
	ctx.r[5].s64 = ctx.r[10].s64 + -17492;
	// 82617210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617214: 390B94E0  addi r8, r11, -0x6b20
	ctx.r[8].s64 = ctx.r[11].s64 + -27424;
	// 82617218: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261721C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82617220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617224: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261722C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617230: 386ABEAC  addi r3, r10, -0x4154
	ctx.r[3].s64 = ctx.r[10].s64 + -16724;
	// 82617234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261723C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261724C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617254: 4BE4FBCD  bl 0x82466e20
	ctx.lr = 0x82617258;
	sub_82466E20(ctx, base);
	// 82617258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261725C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617268 size=112
    let mut pc: u32 = 0x82617268;
    'dispatch: loop {
        match pc {
            0x82617268 => {
    //   block [0x82617268..0x826172D8)
	// 82617268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261726C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617274: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617278: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261727C: 38AABBAC  addi r5, r10, -0x4454
	ctx.r[5].s64 = ctx.r[10].s64 + -17492;
	// 82617280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617284: 390B9528  addi r8, r11, -0x6ad8
	ctx.r[8].s64 = ctx.r[11].s64 + -27352;
	// 82617288: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261728C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82617290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261729C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826172A0: 386ABEDC  addi r3, r10, -0x4124
	ctx.r[3].s64 = ctx.r[10].s64 + -16676;
	// 826172A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826172A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826172AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826172B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826172B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826172B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826172BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826172C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826172C4: 4BE4FB5D  bl 0x82466e20
	ctx.lr = 0x826172C8;
	sub_82466E20(ctx, base);
	// 826172C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826172CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826172D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826172D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826172D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826172D8 size=108
    let mut pc: u32 = 0x826172D8;
    'dispatch: loop {
        match pc {
            0x826172D8 => {
    //   block [0x826172D8..0x82617344)
	// 826172D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826172DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826172E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826172E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826172E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826172EC: 38EB95D0  addi r7, r11, -0x6a30
	ctx.r[7].s64 = ctx.r[11].s64 + -27184;
	// 826172F0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826172F4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826172F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826172FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82617304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617308: 386ABF0C  addi r3, r10, -0x40f4
	ctx.r[3].s64 = ctx.r[10].s64 + -16628;
	// 8261730C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82617310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261731C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261732C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617330: 4BE4FAF1  bl 0x82466e20
	ctx.lr = 0x82617334;
	sub_82466E20(ctx, base);
	// 82617334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261733C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82617348 size=24
    let mut pc: u32 = 0x82617348;
    'dispatch: loop {
        match pc {
            0x82617348 => {
    //   block [0x82617348..0x82617360)
	// 82617348: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261734C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82617350: 394AE0E0  addi r10, r10, -0x1f20
	ctx.r[10].s64 = ctx.r[10].s64 + -7968;
	// 82617354: 816B98E8  lwz r11, -0x6718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26392 as u32) ) } as u64;
	// 82617358: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8261735C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617360 size=116
    let mut pc: u32 = 0x82617360;
    'dispatch: loop {
        match pc {
            0x82617360 => {
    //   block [0x82617360..0x826173D4)
	// 82617360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261736C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82617370: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82617374: 390AE0E0  addi r8, r10, -0x1f20
	ctx.r[8].s64 = ctx.r[10].s64 + -7968;
	// 82617378: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261737C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82617380: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 82617384: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617388: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261738C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617390: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617394: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82617398: 396BFBA8  addi r11, r11, -0x458
	ctx.r[11].s64 = ctx.r[11].s64 + -1112;
	// 8261739C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826173A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826173A4: 386ABF3C  addi r3, r10, -0x40c4
	ctx.r[3].s64 = ctx.r[10].s64 + -16580;
	// 826173A8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826173AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826173B0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826173B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826173B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826173BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826173C0: 4BE4FA61  bl 0x82466e20
	ctx.lr = 0x826173C4;
	sub_82466E20(ctx, base);
	// 826173C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826173C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826173CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826173D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826173D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826173D8 size=112
    let mut pc: u32 = 0x826173D8;
    'dispatch: loop {
        match pc {
            0x826173D8 => {
    //   block [0x826173D8..0x82617448)
	// 826173D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826173DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826173E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826173E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826173E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826173EC: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 826173F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826173F4: 390B9630  addi r8, r11, -0x69d0
	ctx.r[8].s64 = ctx.r[11].s64 + -27088;
	// 826173F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826173FC: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82617400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617404: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261740C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617410: 386ABF6C  addi r3, r10, -0x4094
	ctx.r[3].s64 = ctx.r[10].s64 + -16532;
	// 82617414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261741C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261742C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617434: 4BE4F9ED  bl 0x82466e20
	ctx.lr = 0x82617438;
	sub_82466E20(ctx, base);
	// 82617438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261743C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617448 size=100
    let mut pc: u32 = 0x82617448;
    'dispatch: loop {
        match pc {
            0x82617448 => {
    //   block [0x82617448..0x826174AC)
	// 82617448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261744C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617454: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261745C: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82617460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617468: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8261746C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261747C: 386ABF9C  addi r3, r10, -0x4064
	ctx.r[3].s64 = ctx.r[10].s64 + -16484;
	// 82617480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617484: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617488: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261748C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617490: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82617494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617498: 4BE4F989  bl 0x82466e20
	ctx.lr = 0x8261749C;
	sub_82466E20(ctx, base);
	// 8261749C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826174A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826174A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826174A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826174B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826174B0 size=112
    let mut pc: u32 = 0x826174B0;
    'dispatch: loop {
        match pc {
            0x826174B0 => {
    //   block [0x826174B0..0x82617520)
	// 826174B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826174B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826174B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826174BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826174C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826174C4: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 826174C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826174CC: 390B9648  addi r8, r11, -0x69b8
	ctx.r[8].s64 = ctx.r[11].s64 + -27064;
	// 826174D0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826174D4: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 826174D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826174DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826174E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826174E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826174E8: 386ABFCC  addi r3, r10, -0x4034
	ctx.r[3].s64 = ctx.r[10].s64 + -16436;
	// 826174EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826174F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826174F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826174F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826174FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261750C: 4BE4F915  bl 0x82466e20
	ctx.lr = 0x82617510;
	sub_82466E20(ctx, base);
	// 82617510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261751C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617520 size=112
    let mut pc: u32 = 0x82617520;
    'dispatch: loop {
        match pc {
            0x82617520 => {
    //   block [0x82617520..0x82617590)
	// 82617520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261752C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617530: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617534: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82617538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261753C: 390B96D8  addi r8, r11, -0x6928
	ctx.r[8].s64 = ctx.r[11].s64 + -26920;
	// 82617540: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82617544: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 82617548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261754C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617558: 386ABFFC  addi r3, r10, -0x4004
	ctx.r[3].s64 = ctx.r[10].s64 + -16388;
	// 8261755C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261756C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261757C: 4BE4F8A5  bl 0x82466e20
	ctx.lr = 0x82617580;
	sub_82466E20(ctx, base);
	// 82617580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261758C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617590 size=112
    let mut pc: u32 = 0x82617590;
    'dispatch: loop {
        match pc {
            0x82617590 => {
    //   block [0x82617590..0x82617600)
	// 82617590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261759C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826175A0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826175A4: 38AABB4C  addi r5, r10, -0x44b4
	ctx.r[5].s64 = ctx.r[10].s64 + -17588;
	// 826175A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826175AC: 390B9738  addi r8, r11, -0x68c8
	ctx.r[8].s64 = ctx.r[11].s64 + -26824;
	// 826175B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826175B4: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 826175B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826175BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826175C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826175C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826175C8: 386AC02C  addi r3, r10, -0x3fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -16340;
	// 826175CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826175D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826175D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826175D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826175DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826175E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826175E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826175E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826175EC: 4BE4F835  bl 0x82466e20
	ctx.lr = 0x826175F0;
	sub_82466E20(ctx, base);
	// 826175F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826175F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826175F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826175FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617600 size=112
    let mut pc: u32 = 0x82617600;
    'dispatch: loop {
        match pc {
            0x82617600 => {
    //   block [0x82617600..0x82617670)
	// 82617600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261760C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617610: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617614: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82617618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261761C: 390B9768  addi r8, r11, -0x6898
	ctx.r[8].s64 = ctx.r[11].s64 + -26776;
	// 82617620: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82617624: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82617628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261762C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617630: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617638: 386AC05C  addi r3, r10, -0x3fa4
	ctx.r[3].s64 = ctx.r[10].s64 + -16292;
	// 8261763C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261764C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261765C: 4BE4F7C5  bl 0x82466e20
	ctx.lr = 0x82617660;
	sub_82466E20(ctx, base);
	// 82617660: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617664: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617668: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261766C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617670 size=112
    let mut pc: u32 = 0x82617670;
    'dispatch: loop {
        match pc {
            0x82617670 => {
    //   block [0x82617670..0x826176E0)
	// 82617670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261767C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617680: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617684: 38AABC9C  addi r5, r10, -0x4364
	ctx.r[5].s64 = ctx.r[10].s64 + -17252;
	// 82617688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261768C: 390B97F8  addi r8, r11, -0x6808
	ctx.r[8].s64 = ctx.r[11].s64 + -26632;
	// 82617690: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82617694: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 82617698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261769C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826176A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826176A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826176A8: 386AC08C  addi r3, r10, -0x3f74
	ctx.r[3].s64 = ctx.r[10].s64 + -16244;
	// 826176AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826176B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826176B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826176B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826176BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826176C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826176C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826176C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826176CC: 4BE4F755  bl 0x82466e20
	ctx.lr = 0x826176D0;
	sub_82466E20(ctx, base);
	// 826176D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826176D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826176D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826176DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826176E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826176E0 size=112
    let mut pc: u32 = 0x826176E0;
    'dispatch: loop {
        match pc {
            0x826176E0 => {
    //   block [0x826176E0..0x82617750)
	// 826176E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826176E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826176E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826176EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826176F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826176F4: 38AABEDC  addi r5, r10, -0x4124
	ctx.r[5].s64 = ctx.r[10].s64 + -16676;
	// 826176F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826176FC: 390B9810  addi r8, r11, -0x67f0
	ctx.r[8].s64 = ctx.r[11].s64 + -26608;
	// 82617700: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617704: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 82617708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261770C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617718: 386AC0BC  addi r3, r10, -0x3f44
	ctx.r[3].s64 = ctx.r[10].s64 + -16196;
	// 8261771C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261772C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261773C: 4BE4F6E5  bl 0x82466e20
	ctx.lr = 0x82617740;
	sub_82466E20(ctx, base);
	// 82617740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261774C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617750 size=112
    let mut pc: u32 = 0x82617750;
    'dispatch: loop {
        match pc {
            0x82617750 => {
    //   block [0x82617750..0x826177C0)
	// 82617750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261775C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617760: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617764: 38AAB78C  addi r5, r10, -0x4874
	ctx.r[5].s64 = ctx.r[10].s64 + -18548;
	// 82617768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261776C: 390B9840  addi r8, r11, -0x67c0
	ctx.r[8].s64 = ctx.r[11].s64 + -26560;
	// 82617770: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82617774: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 82617778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261777C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617788: 386AC0EC  addi r3, r10, -0x3f14
	ctx.r[3].s64 = ctx.r[10].s64 + -16148;
	// 8261778C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261779C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826177A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826177A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826177A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826177AC: 4BE4F675  bl 0x82466e20
	ctx.lr = 0x826177B0;
	sub_82466E20(ctx, base);
	// 826177B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826177B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826177B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826177BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826177C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826177C0 size=24
    let mut pc: u32 = 0x826177C0;
    'dispatch: loop {
        match pc {
            0x826177C0 => {
    //   block [0x826177C0..0x826177D8)
	// 826177C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826177C4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826177C8: 394AE158  addi r10, r10, -0x1ea8
	ctx.r[10].s64 = ctx.r[10].s64 + -7848;
	// 826177CC: 816B98E8  lwz r11, -0x6718(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26392 as u32) ) } as u64;
	// 826177D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826177D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826177D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826177D8 size=116
    let mut pc: u32 = 0x826177D8;
    'dispatch: loop {
        match pc {
            0x826177D8 => {
    //   block [0x826177D8..0x8261784C)
	// 826177D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826177DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826177E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826177E4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826177E8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826177EC: 390AE158  addi r8, r10, -0x1ea8
	ctx.r[8].s64 = ctx.r[10].s64 + -7848;
	// 826177F0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826177F4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826177F8: 38AAB93C  addi r5, r10, -0x46c4
	ctx.r[5].s64 = ctx.r[10].s64 + -18116;
	// 826177FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617800: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82617804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261780C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 82617810: 396BFBC0  addi r11, r11, -0x440
	ctx.r[11].s64 = ctx.r[11].s64 + -1088;
	// 82617814: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617818: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261781C: 386AC11C  addi r3, r10, -0x3ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -16100;
	// 82617820: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82617824: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617828: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261782C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617838: 4BE4F5E9  bl 0x82466e20
	ctx.lr = 0x8261783C;
	sub_82466E20(ctx, base);
	// 8261783C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617850 size=112
    let mut pc: u32 = 0x82617850;
    'dispatch: loop {
        match pc {
            0x82617850 => {
    //   block [0x82617850..0x826178C0)
	// 82617850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261785C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617860: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617864: 38AAB8AC  addi r5, r10, -0x4754
	ctx.r[5].s64 = ctx.r[10].s64 + -18260;
	// 82617868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261786C: 390B9888  addi r8, r11, -0x6778
	ctx.r[8].s64 = ctx.r[11].s64 + -26488;
	// 82617870: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617874: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 82617878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261787C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617888: 386AC14C  addi r3, r10, -0x3eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -16052;
	// 8261788C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261789C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826178A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826178A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826178A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826178AC: 4BE4F575  bl 0x82466e20
	ctx.lr = 0x826178B0;
	sub_82466E20(ctx, base);
	// 826178B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826178B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826178B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826178BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826178C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826178C0 size=112
    let mut pc: u32 = 0x826178C0;
    'dispatch: loop {
        match pc {
            0x826178C0 => {
    //   block [0x826178C0..0x82617930)
	// 826178C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826178C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826178C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826178CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826178D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826178D4: 38AAB90C  addi r5, r10, -0x46f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18164;
	// 826178D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826178DC: 390B98B8  addi r8, r11, -0x6748
	ctx.r[8].s64 = ctx.r[11].s64 + -26440;
	// 826178E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826178E4: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826178E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826178EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826178F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826178F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826178F8: 386AC17C  addi r3, r10, -0x3e84
	ctx.r[3].s64 = ctx.r[10].s64 + -16004;
	// 826178FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617900: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261790C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261791C: 4BE4F505  bl 0x82466e20
	ctx.lr = 0x82617920;
	sub_82466E20(ctx, base);
	// 82617920: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617924: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617928: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261792C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617930 size=100
    let mut pc: u32 = 0x82617930;
    'dispatch: loop {
        match pc {
            0x82617930 => {
    //   block [0x82617930..0x82617994)
	// 82617930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261793C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82617940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617944: 392AFC30  addi r9, r10, -0x3d0
	ctx.r[9].s64 = ctx.r[10].s64 + -976;
	// 82617948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261794C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617950: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 82617954: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261795C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617964: 386AC1AC  addi r3, r10, -0x3e54
	ctx.r[3].s64 = ctx.r[10].s64 + -15956;
	// 82617968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261796C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82617970: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82617974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261797C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617980: 4BE4F4A1  bl 0x82466e20
	ctx.lr = 0x82617984;
	sub_82466E20(ctx, base);
	// 82617984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261798C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617990: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82617998 size=24
    let mut pc: u32 = 0x82617998;
    'dispatch: loop {
        match pc {
            0x82617998 => {
    //   block [0x82617998..0x826179B0)
	// 82617998: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261799C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826179A0: 394AE200  addi r10, r10, -0x1e00
	ctx.r[10].s64 = ctx.r[10].s64 + -7680;
	// 826179A4: 816B98F4  lwz r11, -0x670c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26380 as u32) ) } as u64;
	// 826179A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826179AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826179B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826179B0 size=112
    let mut pc: u32 = 0x826179B0;
    'dispatch: loop {
        match pc {
            0x826179B0 => {
    //   block [0x826179B0..0x82617A20)
	// 826179B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826179B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826179B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826179BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826179C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826179C4: 392AFD70  addi r9, r10, -0x290
	ctx.r[9].s64 = ctx.r[10].s64 + -656;
	// 826179C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826179CC: 390BE200  addi r8, r11, -0x1e00
	ctx.r[8].s64 = ctx.r[11].s64 + -7680;
	// 826179D0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826179D4: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826179D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826179DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826179E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826179E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826179E8: 386AC1DC  addi r3, r10, -0x3e24
	ctx.r[3].s64 = ctx.r[10].s64 + -15908;
	// 826179EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826179F0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826179F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826179F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826179FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617A04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617A0C: 4BE4F415  bl 0x82466e20
	ctx.lr = 0x82617A10;
	sub_82466E20(ctx, base);
	// 82617A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617A20 size=112
    let mut pc: u32 = 0x82617A20;
    'dispatch: loop {
        match pc {
            0x82617A20 => {
    //   block [0x82617A20..0x82617A90)
	// 82617A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617A2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617A30: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617A34: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617A38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617A3C: 390B98FC  addi r8, r11, -0x6704
	ctx.r[8].s64 = ctx.r[11].s64 + -26372;
	// 82617A40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617A44: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 82617A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617A4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617A50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617A58: 386AC20C  addi r3, r10, -0x3df4
	ctx.r[3].s64 = ctx.r[10].s64 + -15860;
	// 82617A5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617A60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617A68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617A70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617A78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617A7C: 4BE4F3A5  bl 0x82466e20
	ctx.lr = 0x82617A80;
	sub_82466E20(ctx, base);
	// 82617A80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617A84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617A88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617A8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617A90 size=108
    let mut pc: u32 = 0x82617A90;
    'dispatch: loop {
        match pc {
            0x82617A90 => {
    //   block [0x82617A90..0x82617AFC)
	// 82617A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617A9C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617AA4: 38EB992C  addi r7, r11, -0x66d4
	ctx.r[7].s64 = ctx.r[11].s64 + -26324;
	// 82617AA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82617AAC: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 82617AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617AB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617AB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82617ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617AC0: 386AC23C  addi r3, r10, -0x3dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -15812;
	// 82617AC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82617AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82617AE8: 4BE4F339  bl 0x82466e20
	ctx.lr = 0x82617AEC;
	sub_82466E20(ctx, base);
	// 82617AEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617B00 size=112
    let mut pc: u32 = 0x82617B00;
    'dispatch: loop {
        match pc {
            0x82617B00 => {
    //   block [0x82617B00..0x82617B70)
	// 82617B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617B0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617B10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617B14: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617B18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82617B1C: 390B9948  addi r8, r11, -0x66b8
	ctx.r[8].s64 = ctx.r[11].s64 + -26296;
	// 82617B20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82617B24: 388A0E04  addi r4, r10, 0xe04
	ctx.r[4].s64 = ctx.r[10].s64 + 3588;
	// 82617B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617B2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617B38: 386AC26C  addi r3, r10, -0x3d94
	ctx.r[3].s64 = ctx.r[10].s64 + -15764;
	// 82617B3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617B5C: 4BE4F2C5  bl 0x82466e20
	ctx.lr = 0x82617B60;
	sub_82466E20(ctx, base);
	// 82617B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617B70 size=100
    let mut pc: u32 = 0x82617B70;
    'dispatch: loop {
        match pc {
            0x82617B70 => {
    //   block [0x82617B70..0x82617BD4)
	// 82617B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617B7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617B84: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617B88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617B90: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 82617B94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617BA4: 386AC29C  addi r3, r10, -0x3d64
	ctx.r[3].s64 = ctx.r[10].s64 + -15716;
	// 82617BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617BAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617BB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82617BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617BB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82617BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617BC0: 4BE4F261  bl 0x82466e20
	ctx.lr = 0x82617BC4;
	sub_82466E20(ctx, base);
	// 82617BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617BD8 size=112
    let mut pc: u32 = 0x82617BD8;
    'dispatch: loop {
        match pc {
            0x82617BD8 => {
    //   block [0x82617BD8..0x82617C48)
	// 82617BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617BE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617BE8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617BEC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617BF4: 390B99A8  addi r8, r11, -0x6658
	ctx.r[8].s64 = ctx.r[11].s64 + -26200;
	// 82617BF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82617BFC: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 82617C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617C04: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617C10: 386AC2CC  addi r3, r10, -0x3d34
	ctx.r[3].s64 = ctx.r[10].s64 + -15668;
	// 82617C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617C34: 4BE4F1ED  bl 0x82466e20
	ctx.lr = 0x82617C38;
	sub_82466E20(ctx, base);
	// 82617C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617C48 size=112
    let mut pc: u32 = 0x82617C48;
    'dispatch: loop {
        match pc {
            0x82617C48 => {
    //   block [0x82617C48..0x82617CB8)
	// 82617C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617C54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617C58: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617C5C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617C64: 390B99C0  addi r8, r11, -0x6640
	ctx.r[8].s64 = ctx.r[11].s64 + -26176;
	// 82617C68: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617C6C: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 82617C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617C74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617C80: 386AC2FC  addi r3, r10, -0x3d04
	ctx.r[3].s64 = ctx.r[10].s64 + -15620;
	// 82617C84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617CA4: 4BE4F17D  bl 0x82466e20
	ctx.lr = 0x82617CA8;
	sub_82466E20(ctx, base);
	// 82617CA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617CB8 size=112
    let mut pc: u32 = 0x82617CB8;
    'dispatch: loop {
        match pc {
            0x82617CB8 => {
    //   block [0x82617CB8..0x82617D28)
	// 82617CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617CC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617CC8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617CCC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617CD4: 390B99F0  addi r8, r11, -0x6610
	ctx.r[8].s64 = ctx.r[11].s64 + -26128;
	// 82617CD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617CDC: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 82617CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617CE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617CF0: 386AC32C  addi r3, r10, -0x3cd4
	ctx.r[3].s64 = ctx.r[10].s64 + -15572;
	// 82617CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617D14: 4BE4F10D  bl 0x82466e20
	ctx.lr = 0x82617D18;
	sub_82466E20(ctx, base);
	// 82617D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617D28 size=112
    let mut pc: u32 = 0x82617D28;
    'dispatch: loop {
        match pc {
            0x82617D28 => {
    //   block [0x82617D28..0x82617D98)
	// 82617D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617D34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617D38: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617D3C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617D44: 390B9A20  addi r8, r11, -0x65e0
	ctx.r[8].s64 = ctx.r[11].s64 + -26080;
	// 82617D48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617D4C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 82617D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617D54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617D60: 386AC35C  addi r3, r10, -0x3ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -15524;
	// 82617D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617D84: 4BE4F09D  bl 0x82466e20
	ctx.lr = 0x82617D88;
	sub_82466E20(ctx, base);
	// 82617D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617D98 size=112
    let mut pc: u32 = 0x82617D98;
    'dispatch: loop {
        match pc {
            0x82617D98 => {
    //   block [0x82617D98..0x82617E08)
	// 82617D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617DA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617DA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617DAC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617DB4: 390B9A50  addi r8, r11, -0x65b0
	ctx.r[8].s64 = ctx.r[11].s64 + -26032;
	// 82617DB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82617DBC: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 82617DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617DC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617DC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617DD0: 386AC38C  addi r3, r10, -0x3c74
	ctx.r[3].s64 = ctx.r[10].s64 + -15476;
	// 82617DD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617DDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617DF4: 4BE4F02D  bl 0x82466e20
	ctx.lr = 0x82617DF8;
	sub_82466E20(ctx, base);
	// 82617DF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617E08 size=112
    let mut pc: u32 = 0x82617E08;
    'dispatch: loop {
        match pc {
            0x82617E08 => {
    //   block [0x82617E08..0x82617E78)
	// 82617E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617E14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617E18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617E1C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617E24: 390B9A68  addi r8, r11, -0x6598
	ctx.r[8].s64 = ctx.r[11].s64 + -26008;
	// 82617E28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82617E2C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 82617E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617E34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617E40: 386AC3BC  addi r3, r10, -0x3c44
	ctx.r[3].s64 = ctx.r[10].s64 + -15428;
	// 82617E44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617E54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617E64: 4BE4EFBD  bl 0x82466e20
	ctx.lr = 0x82617E68;
	sub_82466E20(ctx, base);
	// 82617E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617E78 size=112
    let mut pc: u32 = 0x82617E78;
    'dispatch: loop {
        match pc {
            0x82617E78 => {
    //   block [0x82617E78..0x82617EE8)
	// 82617E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617E84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617E88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617E8C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617E94: 390B9A80  addi r8, r11, -0x6580
	ctx.r[8].s64 = ctx.r[11].s64 + -25984;
	// 82617E98: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82617E9C: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 82617EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617EA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617EB0: 386AC3EC  addi r3, r10, -0x3c14
	ctx.r[3].s64 = ctx.r[10].s64 + -15380;
	// 82617EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617ED4: 4BE4EF4D  bl 0x82466e20
	ctx.lr = 0x82617ED8;
	sub_82466E20(ctx, base);
	// 82617ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617EE8 size=112
    let mut pc: u32 = 0x82617EE8;
    'dispatch: loop {
        match pc {
            0x82617EE8 => {
    //   block [0x82617EE8..0x82617F58)
	// 82617EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617EF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617EF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617EFC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617F04: 390B9AC8  addi r8, r11, -0x6538
	ctx.r[8].s64 = ctx.r[11].s64 + -25912;
	// 82617F08: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82617F0C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 82617F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617F14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617F20: 386AC41C  addi r3, r10, -0x3be4
	ctx.r[3].s64 = ctx.r[10].s64 + -15332;
	// 82617F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617F44: 4BE4EEDD  bl 0x82466e20
	ctx.lr = 0x82617F48;
	sub_82466E20(ctx, base);
	// 82617F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617F58 size=112
    let mut pc: u32 = 0x82617F58;
    'dispatch: loop {
        match pc {
            0x82617F58 => {
    //   block [0x82617F58..0x82617FC8)
	// 82617F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617F64: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617F68: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617F6C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617F74: 390B9B10  addi r8, r11, -0x64f0
	ctx.r[8].s64 = ctx.r[11].s64 + -25840;
	// 82617F78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82617F7C: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 82617F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617F84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617F88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82617F90: 386AC44C  addi r3, r10, -0x3bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -15284;
	// 82617F94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82617F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82617F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82617FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82617FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82617FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82617FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82617FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82617FB4: 4BE4EE6D  bl 0x82466e20
	ctx.lr = 0x82617FB8;
	sub_82466E20(ctx, base);
	// 82617FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82617FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82617FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82617FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82617FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82617FC8 size=112
    let mut pc: u32 = 0x82617FC8;
    'dispatch: loop {
        match pc {
            0x82617FC8 => {
    //   block [0x82617FC8..0x82618038)
	// 82617FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82617FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82617FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82617FD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617FD8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82617FDC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82617FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82617FE4: 390B9B28  addi r8, r11, -0x64d8
	ctx.r[8].s64 = ctx.r[11].s64 + -25816;
	// 82617FE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82617FEC: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 82617FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82617FF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82617FF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82617FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618000: 386AC47C  addi r3, r10, -0x3b84
	ctx.r[3].s64 = ctx.r[10].s64 + -15236;
	// 82618004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261800C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261801C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618024: 4BE4EDFD  bl 0x82466e20
	ctx.lr = 0x82618028;
	sub_82466E20(ctx, base);
	// 82618028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261802C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618038 size=116
    let mut pc: u32 = 0x82618038;
    'dispatch: loop {
        match pc {
            0x82618038 => {
    //   block [0x82618038..0x826180AC)
	// 82618038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261803C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618044: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82618048: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8261804C: 390A9B58  addi r8, r10, -0x64a8
	ctx.r[8].s64 = ctx.r[10].s64 + -25768;
	// 82618050: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618054: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82618058: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 8261805C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618060: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261806C: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 82618070: 396BFD98  addi r11, r11, -0x268
	ctx.r[11].s64 = ctx.r[11].s64 + -616;
	// 82618074: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618078: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261807C: 386AC4AC  addi r3, r10, -0x3b54
	ctx.r[3].s64 = ctx.r[10].s64 + -15188;
	// 82618080: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82618084: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618088: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261808C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618098: 4BE4ED89  bl 0x82466e20
	ctx.lr = 0x8261809C;
	sub_82466E20(ctx, base);
	// 8261809C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826180A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826180A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826180A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826180B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826180B0 size=116
    let mut pc: u32 = 0x826180B0;
    'dispatch: loop {
        match pc {
            0x826180B0 => {
    //   block [0x826180B0..0x82618124)
	// 826180B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826180B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826180B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826180BC: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826180C0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826180C4: 390A9BD0  addi r8, r10, -0x6430
	ctx.r[8].s64 = ctx.r[10].s64 + -25648;
	// 826180C8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826180CC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 826180D0: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 826180D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826180D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826180DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826180E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826180E4: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 826180E8: 396BFDB0  addi r11, r11, -0x250
	ctx.r[11].s64 = ctx.r[11].s64 + -592;
	// 826180EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826180F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826180F4: 386AC4DC  addi r3, r10, -0x3b24
	ctx.r[3].s64 = ctx.r[10].s64 + -15140;
	// 826180F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826180FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618100: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82618104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261810C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618110: 4BE4ED11  bl 0x82466e20
	ctx.lr = 0x82618114;
	sub_82466E20(ctx, base);
	// 82618114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261811C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82618128 size=24
    let mut pc: u32 = 0x82618128;
    'dispatch: loop {
        match pc {
            0x82618128 => {
    //   block [0x82618128..0x82618140)
	// 82618128: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261812C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82618130: 394AE218  addi r10, r10, -0x1de8
	ctx.r[10].s64 = ctx.r[10].s64 + -7656;
	// 82618134: 816B9944  lwz r11, -0x66bc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 82618138: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8261813C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618140 size=116
    let mut pc: u32 = 0x82618140;
    'dispatch: loop {
        match pc {
            0x82618140 => {
    //   block [0x82618140..0x826181B4)
	// 82618140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261814C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82618150: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618154: 392BFDDC  addi r9, r11, -0x224
	ctx.r[9].s64 = ctx.r[11].s64 + -548;
	// 82618158: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 8261815C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618160: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82618164: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82618168: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261816C: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 82618170: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618174: 396BE218  addi r11, r11, -0x1de8
	ctx.r[11].s64 = ctx.r[11].s64 + -7656;
	// 82618178: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8261817C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618180: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82618184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618188: 386AC50C  addi r3, r10, -0x3af4
	ctx.r[3].s64 = ctx.r[10].s64 + -15092;
	// 8261818C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82618190: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82618194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618198: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8261819C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826181A0: 4BE4EC81  bl 0x82466e20
	ctx.lr = 0x826181A4;
	sub_82466E20(ctx, base);
	// 826181A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826181A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826181AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826181B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826181B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826181B8 size=112
    let mut pc: u32 = 0x826181B8;
    'dispatch: loop {
        match pc {
            0x826181B8 => {
    //   block [0x826181B8..0x82618228)
	// 826181B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826181BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826181C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826181C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826181C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826181CC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 826181D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826181D4: 390B9C60  addi r8, r11, -0x63a0
	ctx.r[8].s64 = ctx.r[11].s64 + -25504;
	// 826181D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826181DC: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 826181E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826181E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826181E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826181EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826181F0: 386AC53C  addi r3, r10, -0x3ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -15044;
	// 826181F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826181F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826181FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261820C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618214: 4BE4EC0D  bl 0x82466e20
	ctx.lr = 0x82618218;
	sub_82466E20(ctx, base);
	// 82618218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261821C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618228 size=112
    let mut pc: u32 = 0x82618228;
    'dispatch: loop {
        match pc {
            0x82618228 => {
    //   block [0x82618228..0x82618298)
	// 82618228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261822C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618234: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618238: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261823C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82618240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618244: 390B9CC0  addi r8, r11, -0x6340
	ctx.r[8].s64 = ctx.r[11].s64 + -25408;
	// 82618248: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261824C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 82618250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618254: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618258: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261825C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618260: 386AC56C  addi r3, r10, -0x3a94
	ctx.r[3].s64 = ctx.r[10].s64 + -14996;
	// 82618264: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261826C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261827C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618284: 4BE4EB9D  bl 0x82466e20
	ctx.lr = 0x82618288;
	sub_82466E20(ctx, base);
	// 82618288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261828C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618298 size=112
    let mut pc: u32 = 0x82618298;
    'dispatch: loop {
        match pc {
            0x82618298 => {
    //   block [0x82618298..0x82618308)
	// 82618298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826182A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826182A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826182A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826182AC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 826182B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826182B4: 390B9D68  addi r8, r11, -0x6298
	ctx.r[8].s64 = ctx.r[11].s64 + -25240;
	// 826182B8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826182BC: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 826182C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826182C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826182C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826182CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826182D0: 386AC59C  addi r3, r10, -0x3a64
	ctx.r[3].s64 = ctx.r[10].s64 + -14948;
	// 826182D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826182D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826182DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826182E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826182E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826182E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826182EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826182F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826182F4: 4BE4EB2D  bl 0x82466e20
	ctx.lr = 0x826182F8;
	sub_82466E20(ctx, base);
	// 826182F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826182FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618308 size=112
    let mut pc: u32 = 0x82618308;
    'dispatch: loop {
        match pc {
            0x82618308 => {
    //   block [0x82618308..0x82618378)
	// 82618308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261830C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618314: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618318: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261831C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82618320: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618324: 390B9DE0  addi r8, r11, -0x6220
	ctx.r[8].s64 = ctx.r[11].s64 + -25120;
	// 82618328: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261832C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 82618330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618334: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261833C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618340: 386AC5CC  addi r3, r10, -0x3a34
	ctx.r[3].s64 = ctx.r[10].s64 + -14900;
	// 82618344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261834C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261835C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618364: 4BE4EABD  bl 0x82466e20
	ctx.lr = 0x82618368;
	sub_82466E20(ctx, base);
	// 82618368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261836C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618378 size=112
    let mut pc: u32 = 0x82618378;
    'dispatch: loop {
        match pc {
            0x82618378 => {
    //   block [0x82618378..0x826183E8)
	// 82618378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261837C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618384: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618388: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261838C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82618390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618394: 390B9E28  addi r8, r11, -0x61d8
	ctx.r[8].s64 = ctx.r[11].s64 + -25048;
	// 82618398: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8261839C: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826183A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826183A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826183A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826183AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826183B0: 386AC5FC  addi r3, r10, -0x3a04
	ctx.r[3].s64 = ctx.r[10].s64 + -14852;
	// 826183B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826183B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826183BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826183C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826183C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826183C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826183CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826183D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826183D4: 4BE4EA4D  bl 0x82466e20
	ctx.lr = 0x826183D8;
	sub_82466E20(ctx, base);
	// 826183D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826183DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826183E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826183E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826183E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826183E8 size=112
    let mut pc: u32 = 0x826183E8;
    'dispatch: loop {
        match pc {
            0x826183E8 => {
    //   block [0x826183E8..0x82618458)
	// 826183E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826183EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826183F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826183F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826183F8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826183FC: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82618400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618404: 390B9EB8  addi r8, r11, -0x6148
	ctx.r[8].s64 = ctx.r[11].s64 + -24904;
	// 82618408: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261840C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 82618410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618414: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261841C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618420: 386AC62C  addi r3, r10, -0x39d4
	ctx.r[3].s64 = ctx.r[10].s64 + -14804;
	// 82618424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261842C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261843C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618444: 4BE4E9DD  bl 0x82466e20
	ctx.lr = 0x82618448;
	sub_82466E20(ctx, base);
	// 82618448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261844C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618458 size=112
    let mut pc: u32 = 0x82618458;
    'dispatch: loop {
        match pc {
            0x82618458 => {
    //   block [0x82618458..0x826184C8)
	// 82618458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261845C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618464: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618468: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261846C: 38AAC1DC  addi r5, r10, -0x3e24
	ctx.r[5].s64 = ctx.r[10].s64 + -15908;
	// 82618470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618474: 390B9F18  addi r8, r11, -0x60e8
	ctx.r[8].s64 = ctx.r[11].s64 + -24808;
	// 82618478: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261847C: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 82618480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618484: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261848C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618490: 386AC65C  addi r3, r10, -0x39a4
	ctx.r[3].s64 = ctx.r[10].s64 + -14756;
	// 82618494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261849C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826184A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826184A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826184A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826184AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826184B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826184B4: 4BE4E96D  bl 0x82466e20
	ctx.lr = 0x826184B8;
	sub_82466E20(ctx, base);
	// 826184B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826184BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826184C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826184C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826184C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826184C8 size=112
    let mut pc: u32 = 0x826184C8;
    'dispatch: loop {
        match pc {
            0x826184C8 => {
    //   block [0x826184C8..0x82618538)
	// 826184C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826184CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826184D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826184D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826184D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826184DC: 38AAC65C  addi r5, r10, -0x39a4
	ctx.r[5].s64 = ctx.r[10].s64 + -14756;
	// 826184E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826184E4: 390B9F78  addi r8, r11, -0x6088
	ctx.r[8].s64 = ctx.r[11].s64 + -24712;
	// 826184E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826184EC: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 826184F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826184F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826184F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826184FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618500: 386AC68C  addi r3, r10, -0x3974
	ctx.r[3].s64 = ctx.r[10].s64 + -14708;
	// 82618504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261850C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261851C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618524: 4BE4E8FD  bl 0x82466e20
	ctx.lr = 0x82618528;
	sub_82466E20(ctx, base);
	// 82618528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261852C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618538 size=112
    let mut pc: u32 = 0x82618538;
    'dispatch: loop {
        match pc {
            0x82618538 => {
    //   block [0x82618538..0x826185A8)
	// 82618538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261853C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618544: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618548: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261854C: 38AAC65C  addi r5, r10, -0x39a4
	ctx.r[5].s64 = ctx.r[10].s64 + -14756;
	// 82618550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618554: 390B9FA8  addi r8, r11, -0x6058
	ctx.r[8].s64 = ctx.r[11].s64 + -24664;
	// 82618558: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261855C: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 82618560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261856C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618570: 386AC6BC  addi r3, r10, -0x3944
	ctx.r[3].s64 = ctx.r[10].s64 + -14660;
	// 82618574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261857C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261858C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618594: 4BE4E88D  bl 0x82466e20
	ctx.lr = 0x82618598;
	sub_82466E20(ctx, base);
	// 82618598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261859C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826185A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826185A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826185A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826185A8 size=100
    let mut pc: u32 = 0x826185A8;
    'dispatch: loop {
        match pc {
            0x826185A8 => {
    //   block [0x826185A8..0x8261860C)
	// 826185A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826185AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826185B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826185B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826185B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826185BC: 38AAC65C  addi r5, r10, -0x39a4
	ctx.r[5].s64 = ctx.r[10].s64 + -14756;
	// 826185C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826185C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826185C8: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 826185CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826185D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826185D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826185D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826185DC: 386AC6EC  addi r3, r10, -0x3914
	ctx.r[3].s64 = ctx.r[10].s64 + -14612;
	// 826185E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826185E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826185E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826185EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826185F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826185F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826185F8: 4BE4E829  bl 0x82466e20
	ctx.lr = 0x826185FC;
	sub_82466E20(ctx, base);
	// 826185FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618610 size=112
    let mut pc: u32 = 0x82618610;
    'dispatch: loop {
        match pc {
            0x82618610 => {
    //   block [0x82618610..0x82618680)
	// 82618610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261861C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618620: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618624: 38AAC65C  addi r5, r10, -0x39a4
	ctx.r[5].s64 = ctx.r[10].s64 + -14756;
	// 82618628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261862C: 390B9FF0  addi r8, r11, -0x6010
	ctx.r[8].s64 = ctx.r[11].s64 + -24592;
	// 82618630: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618634: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 82618638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261863C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618648: 386AC71C  addi r3, r10, -0x38e4
	ctx.r[3].s64 = ctx.r[10].s64 + -14564;
	// 8261864C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261865C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261866C: 4BE4E7B5  bl 0x82466e20
	ctx.lr = 0x82618670;
	sub_82466E20(ctx, base);
	// 82618670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618680 size=108
    let mut pc: u32 = 0x82618680;
    'dispatch: loop {
        match pc {
            0x82618680 => {
    //   block [0x82618680..0x826186EC)
	// 82618680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261868C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618694: 38EBA008  addi r7, r11, -0x5ff8
	ctx.r[7].s64 = ctx.r[11].s64 + -24568;
	// 82618698: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261869C: 388A0E24  addi r4, r10, 0xe24
	ctx.r[4].s64 = ctx.r[10].s64 + 3620;
	// 826186A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826186A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826186A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826186AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826186B0: 386AC74C  addi r3, r10, -0x38b4
	ctx.r[3].s64 = ctx.r[10].s64 + -14516;
	// 826186B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826186B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826186BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826186C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826186C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826186C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826186CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826186D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826186D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826186D8: 4BE4E749  bl 0x82466e20
	ctx.lr = 0x826186DC;
	sub_82466E20(ctx, base);
	// 826186DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826186E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826186E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826186E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826186F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826186F0 size=112
    let mut pc: u32 = 0x826186F0;
    'dispatch: loop {
        match pc {
            0x826186F0 => {
    //   block [0x826186F0..0x82618760)
	// 826186F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826186F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826186F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826186FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618700: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618704: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82618708: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261870C: 390BA050  addi r8, r11, -0x5fb0
	ctx.r[8].s64 = ctx.r[11].s64 + -24496;
	// 82618710: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82618714: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 82618718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261871C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618728: 386AC77C  addi r3, r10, -0x3884
	ctx.r[3].s64 = ctx.r[10].s64 + -14468;
	// 8261872C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261873C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261874C: 4BE4E6D5  bl 0x82466e20
	ctx.lr = 0x82618750;
	sub_82466E20(ctx, base);
	// 82618750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261875C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618760 size=112
    let mut pc: u32 = 0x82618760;
    'dispatch: loop {
        match pc {
            0x82618760 => {
    //   block [0x82618760..0x826187D0)
	// 82618760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261876C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618770: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618774: 38AAC77C  addi r5, r10, -0x3884
	ctx.r[5].s64 = ctx.r[10].s64 + -14468;
	// 82618778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261877C: 390BA0B0  addi r8, r11, -0x5f50
	ctx.r[8].s64 = ctx.r[11].s64 + -24400;
	// 82618780: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618784: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82618788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261878C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618798: 386AC7AC  addi r3, r10, -0x3854
	ctx.r[3].s64 = ctx.r[10].s64 + -14420;
	// 8261879C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826187A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826187A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826187A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826187AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826187B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826187B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826187B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826187BC: 4BE4E665  bl 0x82466e20
	ctx.lr = 0x826187C0;
	sub_82466E20(ctx, base);
	// 826187C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826187C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826187C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826187CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826187D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826187D0 size=112
    let mut pc: u32 = 0x826187D0;
    'dispatch: loop {
        match pc {
            0x826187D0 => {
    //   block [0x826187D0..0x82618840)
	// 826187D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826187D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826187D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826187DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826187E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826187E4: 38AAC77C  addi r5, r10, -0x3884
	ctx.r[5].s64 = ctx.r[10].s64 + -14468;
	// 826187E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826187EC: 390BA0C8  addi r8, r11, -0x5f38
	ctx.r[8].s64 = ctx.r[11].s64 + -24376;
	// 826187F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826187F4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826187F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826187FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618808: 386AC7DC  addi r3, r10, -0x3824
	ctx.r[3].s64 = ctx.r[10].s64 + -14372;
	// 8261880C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261881C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261882C: 4BE4E5F5  bl 0x82466e20
	ctx.lr = 0x82618830;
	sub_82466E20(ctx, base);
	// 82618830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261883C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618840 size=112
    let mut pc: u32 = 0x82618840;
    'dispatch: loop {
        match pc {
            0x82618840 => {
    //   block [0x82618840..0x826188B0)
	// 82618840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261884C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618850: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618854: 38AAC77C  addi r5, r10, -0x3884
	ctx.r[5].s64 = ctx.r[10].s64 + -14468;
	// 82618858: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261885C: 390BA0F8  addi r8, r11, -0x5f08
	ctx.r[8].s64 = ctx.r[11].s64 + -24328;
	// 82618860: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618864: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82618868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261886C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618878: 386AC80C  addi r3, r10, -0x37f4
	ctx.r[3].s64 = ctx.r[10].s64 + -14324;
	// 8261887C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261888C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261889C: 4BE4E585  bl 0x82466e20
	ctx.lr = 0x826188A0;
	sub_82466E20(ctx, base);
	// 826188A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826188A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826188A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826188AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826188B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826188B0 size=24
    let mut pc: u32 = 0x826188B0;
    'dispatch: loop {
        match pc {
            0x826188B0 => {
    //   block [0x826188B0..0x826188C8)
	// 826188B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826188B4: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 826188B8: 394AE2C0  addi r10, r10, -0x1d40
	ctx.r[10].s64 = ctx.r[10].s64 + -7488;
	// 826188BC: 816BA110  lwz r11, -0x5ef0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24304 as u32) ) } as u64;
	// 826188C0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826188C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826188C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826188C8 size=112
    let mut pc: u32 = 0x826188C8;
    'dispatch: loop {
        match pc {
            0x826188C8 => {
    //   block [0x826188C8..0x82618938)
	// 826188C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826188CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826188D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826188D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826188D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826188DC: 392AFE38  addi r9, r10, -0x1c8
	ctx.r[9].s64 = ctx.r[10].s64 + -456;
	// 826188E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826188E4: 390BE2C0  addi r8, r11, -0x1d40
	ctx.r[8].s64 = ctx.r[11].s64 + -7488;
	// 826188E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826188EC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826188F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826188F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826188F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826188FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618900: 386AC83C  addi r3, r10, -0x37c4
	ctx.r[3].s64 = ctx.r[10].s64 + -14276;
	// 82618904: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618908: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261890C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261891C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618924: 4BE4E4FD  bl 0x82466e20
	ctx.lr = 0x82618928;
	sub_82466E20(ctx, base);
	// 82618928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261892C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618938 size=108
    let mut pc: u32 = 0x82618938;
    'dispatch: loop {
        match pc {
            0x82618938 => {
    //   block [0x82618938..0x826189A4)
	// 82618938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261893C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618944: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261894C: 38EBA114  addi r7, r11, -0x5eec
	ctx.r[7].s64 = ctx.r[11].s64 + -24300;
	// 82618950: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82618954: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82618958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261895C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82618964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618968: 386AC86C  addi r3, r10, -0x3794
	ctx.r[3].s64 = ctx.r[10].s64 + -14228;
	// 8261896C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82618970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261897C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261898C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618990: 4BE4E491  bl 0x82466e20
	ctx.lr = 0x82618994;
	sub_82466E20(ctx, base);
	// 82618994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261899C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826189A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826189A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826189A8 size=108
    let mut pc: u32 = 0x826189A8;
    'dispatch: loop {
        match pc {
            0x826189A8 => {
    //   block [0x826189A8..0x82618A14)
	// 826189A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826189AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826189B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826189B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826189B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826189BC: 38EBA130  addi r7, r11, -0x5ed0
	ctx.r[7].s64 = ctx.r[11].s64 + -24272;
	// 826189C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826189C4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826189C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826189CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826189D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826189D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826189D8: 386AC89C  addi r3, r10, -0x3764
	ctx.r[3].s64 = ctx.r[10].s64 + -14180;
	// 826189DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826189E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826189E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826189E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826189EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826189F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826189F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826189F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826189FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618A00: 4BE4E421  bl 0x82466e20
	ctx.lr = 0x82618A04;
	sub_82466E20(ctx, base);
	// 82618A04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618A08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618A0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618A10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618A18 size=116
    let mut pc: u32 = 0x82618A18;
    'dispatch: loop {
        match pc {
            0x82618A18 => {
    //   block [0x82618A18..0x82618A8C)
	// 82618A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618A24: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618A28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618A2C: 390BA178  addi r8, r11, -0x5e88
	ctx.r[8].s64 = ctx.r[11].s64 + -24200;
	// 82618A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618A34: 392AFEF0  addi r9, r10, -0x110
	ctx.r[9].s64 = ctx.r[10].s64 + -272;
	// 82618A38: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618A3C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82618A40: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82618A44: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618A4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618A5C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82618A60: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 82618A64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618A68: 386BC8CC  addi r3, r11, -0x3734
	ctx.r[3].s64 = ctx.r[11].s64 + -14132;
	// 82618A6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82618A70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618A74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618A78: 4BE4E3A9  bl 0x82466e20
	ctx.lr = 0x82618A7C;
	sub_82466E20(ctx, base);
	// 82618A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82618A90 size=24
    let mut pc: u32 = 0x82618A90;
    'dispatch: loop {
        match pc {
            0x82618A90 => {
    //   block [0x82618A90..0x82618AA8)
	// 82618A90: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618A94: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82618A98: 394AE308  addi r10, r10, -0x1cf8
	ctx.r[10].s64 = ctx.r[10].s64 + -7416;
	// 82618A9C: 816BA190  lwz r11, -0x5e70(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24176 as u32) ) } as u64;
	// 82618AA0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82618AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618AA8 size=116
    let mut pc: u32 = 0x82618AA8;
    'dispatch: loop {
        match pc {
            0x82618AA8 => {
    //   block [0x82618AA8..0x82618B1C)
	// 82618AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618AB4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618AB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618ABC: 390BE308  addi r8, r11, -0x1cf8
	ctx.r[8].s64 = ctx.r[11].s64 + -7416;
	// 82618AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618AC4: 392AFF60  addi r9, r10, -0xa0
	ctx.r[9].s64 = ctx.r[10].s64 + -160;
	// 82618AC8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618ACC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82618AD0: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82618AD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618AD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618ADC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618AE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618AE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618AE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618AEC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82618AF0: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 82618AF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618AF8: 386BC8FC  addi r3, r11, -0x3704
	ctx.r[3].s64 = ctx.r[11].s64 + -14084;
	// 82618AFC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82618B00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618B08: 4BE4E319  bl 0x82466e20
	ctx.lr = 0x82618B0C;
	sub_82466E20(ctx, base);
	// 82618B0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618B20 size=108
    let mut pc: u32 = 0x82618B20;
    'dispatch: loop {
        match pc {
            0x82618B20 => {
    //   block [0x82618B20..0x82618B8C)
	// 82618B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618B2C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618B34: 38EBA1A0  addi r7, r11, -0x5e60
	ctx.r[7].s64 = ctx.r[11].s64 + -24160;
	// 82618B38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82618B3C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 82618B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618B44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618B48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82618B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618B50: 386AC92C  addi r3, r10, -0x36d4
	ctx.r[3].s64 = ctx.r[10].s64 + -14036;
	// 82618B54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82618B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618B74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618B78: 4BE4E2A9  bl 0x82466e20
	ctx.lr = 0x82618B7C;
	sub_82466E20(ctx, base);
	// 82618B7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618B90 size=112
    let mut pc: u32 = 0x82618B90;
    'dispatch: loop {
        match pc {
            0x82618B90 => {
    //   block [0x82618B90..0x82618C00)
	// 82618B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618B9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618BA0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618BA4: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82618BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618BAC: 390BA1D0  addi r8, r11, -0x5e30
	ctx.r[8].s64 = ctx.r[11].s64 + -24112;
	// 82618BB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618BB4: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82618BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618BBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618BC8: 386AC95C  addi r3, r10, -0x36a4
	ctx.r[3].s64 = ctx.r[10].s64 + -13988;
	// 82618BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618BEC: 4BE4E235  bl 0x82466e20
	ctx.lr = 0x82618BF0;
	sub_82466E20(ctx, base);
	// 82618BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618C00 size=112
    let mut pc: u32 = 0x82618C00;
    'dispatch: loop {
        match pc {
            0x82618C00 => {
    //   block [0x82618C00..0x82618C70)
	// 82618C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618C0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618C10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618C14: 392AFFB8  addi r9, r10, -0x48
	ctx.r[9].s64 = ctx.r[10].s64 + -72;
	// 82618C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618C1C: 390BA1F0  addi r8, r11, -0x5e10
	ctx.r[8].s64 = ctx.r[11].s64 + -24080;
	// 82618C20: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82618C24: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 82618C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618C2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618C30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618C34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618C38: 386AC98C  addi r3, r10, -0x3674
	ctx.r[3].s64 = ctx.r[10].s64 + -13940;
	// 82618C3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618C40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82618C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618C5C: 4BE4E1C5  bl 0x82466e20
	ctx.lr = 0x82618C60;
	sub_82466E20(ctx, base);
	// 82618C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618C70 size=112
    let mut pc: u32 = 0x82618C70;
    'dispatch: loop {
        match pc {
            0x82618C70 => {
    //   block [0x82618C70..0x82618CE0)
	// 82618C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618C7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618C80: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618C84: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82618C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618C8C: 390BA238  addi r8, r11, -0x5dc8
	ctx.r[8].s64 = ctx.r[11].s64 + -24008;
	// 82618C90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618C94: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 82618C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618C9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618CA8: 386AC9BC  addi r3, r10, -0x3644
	ctx.r[3].s64 = ctx.r[10].s64 + -13892;
	// 82618CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618CCC: 4BE4E155  bl 0x82466e20
	ctx.lr = 0x82618CD0;
	sub_82466E20(ctx, base);
	// 82618CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618CE0 size=112
    let mut pc: u32 = 0x82618CE0;
    'dispatch: loop {
        match pc {
            0x82618CE0 => {
    //   block [0x82618CE0..0x82618D50)
	// 82618CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618CEC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618CF0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618CF4: 392AFFE4  addi r9, r10, -0x1c
	ctx.r[9].s64 = ctx.r[10].s64 + -28;
	// 82618CF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618CFC: 390BA250  addi r8, r11, -0x5db0
	ctx.r[8].s64 = ctx.r[11].s64 + -23984;
	// 82618D00: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82618D04: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 82618D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618D0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618D10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618D18: 386AC9EC  addi r3, r10, -0x3614
	ctx.r[3].s64 = ctx.r[10].s64 + -13844;
	// 82618D1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618D20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82618D24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618D34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618D3C: 4BE4E0E5  bl 0x82466e20
	ctx.lr = 0x82618D40;
	sub_82466E20(ctx, base);
	// 82618D40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618D44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618D48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618D50 size=112
    let mut pc: u32 = 0x82618D50;
    'dispatch: loop {
        match pc {
            0x82618D50 => {
    //   block [0x82618D50..0x82618DC0)
	// 82618D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618D5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618D60: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618D64: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82618D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618D6C: 390BA2E0  addi r8, r11, -0x5d20
	ctx.r[8].s64 = ctx.r[11].s64 + -23840;
	// 82618D70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618D74: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82618D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618D7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618D88: 386ACA1C  addi r3, r10, -0x35e4
	ctx.r[3].s64 = ctx.r[10].s64 + -13796;
	// 82618D8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618DAC: 4BE4E075  bl 0x82466e20
	ctx.lr = 0x82618DB0;
	sub_82466E20(ctx, base);
	// 82618DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618DC0 size=112
    let mut pc: u32 = 0x82618DC0;
    'dispatch: loop {
        match pc {
            0x82618DC0 => {
    //   block [0x82618DC0..0x82618E30)
	// 82618DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618DCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618DD0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618DD4: 38AACA7C  addi r5, r10, -0x3584
	ctx.r[5].s64 = ctx.r[10].s64 + -13700;
	// 82618DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618DDC: 390BA2F8  addi r8, r11, -0x5d08
	ctx.r[8].s64 = ctx.r[11].s64 + -23816;
	// 82618DE0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82618DE4: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 82618DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618DF8: 386ACA4C  addi r3, r10, -0x35b4
	ctx.r[3].s64 = ctx.r[10].s64 + -13748;
	// 82618DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618E1C: 4BE4E005  bl 0x82466e20
	ctx.lr = 0x82618E20;
	sub_82466E20(ctx, base);
	// 82618E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618E30 size=100
    let mut pc: u32 = 0x82618E30;
    'dispatch: loop {
        match pc {
            0x82618E30 => {
    //   block [0x82618E30..0x82618E94)
	// 82618E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618E3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618E44: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82618E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618E50: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 82618E54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618E64: 386ACA7C  addi r3, r10, -0x3584
	ctx.r[3].s64 = ctx.r[10].s64 + -13700;
	// 82618E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618E6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618E70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82618E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618E78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82618E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618E80: 4BE4DFA1  bl 0x82466e20
	ctx.lr = 0x82618E84;
	sub_82466E20(ctx, base);
	// 82618E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82618E98 size=24
    let mut pc: u32 = 0x82618E98;
    'dispatch: loop {
        match pc {
            0x82618E98 => {
    //   block [0x82618E98..0x82618EB0)
	// 82618E98: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618E9C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82618EA0: 394AE3E0  addi r10, r10, -0x1c20
	ctx.r[10].s64 = ctx.r[10].s64 + -7200;
	// 82618EA4: 816BA370  lwz r11, -0x5c90(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23696 as u32) ) } as u64;
	// 82618EA8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82618EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618EB0 size=116
    let mut pc: u32 = 0x82618EB0;
    'dispatch: loop {
        match pc {
            0x82618EB0 => {
    //   block [0x82618EB0..0x82618F24)
	// 82618EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618EB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618EB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618EBC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618EC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82618EC4: 390BE3E0  addi r8, r11, -0x1c20
	ctx.r[8].s64 = ctx.r[11].s64 + -7200;
	// 82618EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618ECC: 392A0020  addi r9, r10, 0x20
	ctx.r[9].s64 = ctx.r[10].s64 + 32;
	// 82618ED0: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618ED4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82618ED8: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82618EDC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618EE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618EF4: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82618EF8: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 82618EFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82618F00: 386BCAAC  addi r3, r11, -0x3554
	ctx.r[3].s64 = ctx.r[11].s64 + -13652;
	// 82618F04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82618F08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618F10: 4BE4DF11  bl 0x82466e20
	ctx.lr = 0x82618F14;
	sub_82466E20(ctx, base);
	// 82618F14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618F28 size=108
    let mut pc: u32 = 0x82618F28;
    'dispatch: loop {
        match pc {
            0x82618F28 => {
    //   block [0x82618F28..0x82618F94)
	// 82618F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618F34: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618F3C: 38EBA374  addi r7, r11, -0x5c8c
	ctx.r[7].s64 = ctx.r[11].s64 + -23692;
	// 82618F40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82618F44: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 82618F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618F4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618F50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82618F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618F58: 386ACADC  addi r3, r10, -0x3524
	ctx.r[3].s64 = ctx.r[10].s64 + -13604;
	// 82618F5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82618F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82618F80: 4BE4DEA1  bl 0x82466e20
	ctx.lr = 0x82618F84;
	sub_82466E20(ctx, base);
	// 82618F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82618F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82618F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82618F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82618F98 size=112
    let mut pc: u32 = 0x82618F98;
    'dispatch: loop {
        match pc {
            0x82618F98 => {
    //   block [0x82618F98..0x82619008)
	// 82618F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82618F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82618FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82618FA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618FA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82618FAC: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82618FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82618FB4: 390BA3A4  addi r8, r11, -0x5c5c
	ctx.r[8].s64 = ctx.r[11].s64 + -23644;
	// 82618FB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82618FBC: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 82618FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82618FC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82618FC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82618FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82618FD0: 386ACB0C  addi r3, r10, -0x34f4
	ctx.r[3].s64 = ctx.r[10].s64 + -13556;
	// 82618FD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82618FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82618FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82618FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82618FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82618FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82618FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82618FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82618FF4: 4BE4DE2D  bl 0x82466e20
	ctx.lr = 0x82618FF8;
	sub_82466E20(ctx, base);
	// 82618FF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82618FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619008 size=112
    let mut pc: u32 = 0x82619008;
    'dispatch: loop {
        match pc {
            0x82619008 => {
    //   block [0x82619008..0x82619078)
	// 82619008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261900C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619014: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82619018: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261901C: 392A0044  addi r9, r10, 0x44
	ctx.r[9].s64 = ctx.r[10].s64 + 68;
	// 82619020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619024: 390BA3C0  addi r8, r11, -0x5c40
	ctx.r[8].s64 = ctx.r[11].s64 + -23616;
	// 82619028: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8261902C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 82619030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619034: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261903C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619040: 386ACB3C  addi r3, r10, -0x34c4
	ctx.r[3].s64 = ctx.r[10].s64 + -13508;
	// 82619044: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619048: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261904C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261905C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619064: 4BE4DDBD  bl 0x82466e20
	ctx.lr = 0x82619068;
	sub_82466E20(ctx, base);
	// 82619068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261906C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619078 size=112
    let mut pc: u32 = 0x82619078;
    'dispatch: loop {
        match pc {
            0x82619078 => {
    //   block [0x82619078..0x826190E8)
	// 82619078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261907C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619084: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619088: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261908C: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619094: 390BA468  addi r8, r11, -0x5b98
	ctx.r[8].s64 = ctx.r[11].s64 + -23448;
	// 82619098: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261909C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826190A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826190A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826190A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826190AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826190B0: 386ACB6C  addi r3, r10, -0x3494
	ctx.r[3].s64 = ctx.r[10].s64 + -13460;
	// 826190B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826190B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826190BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826190C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826190C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826190C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826190CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826190D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826190D4: 4BE4DD4D  bl 0x82466e20
	ctx.lr = 0x826190D8;
	sub_82466E20(ctx, base);
	// 826190D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826190DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826190E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826190E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826190E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826190E8 size=112
    let mut pc: u32 = 0x826190E8;
    'dispatch: loop {
        match pc {
            0x826190E8 => {
    //   block [0x826190E8..0x82619158)
	// 826190E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826190EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826190F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826190F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826190F8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826190FC: 392A009C  addi r9, r10, 0x9c
	ctx.r[9].s64 = ctx.r[10].s64 + 156;
	// 82619100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619104: 390BA488  addi r8, r11, -0x5b78
	ctx.r[8].s64 = ctx.r[11].s64 + -23416;
	// 82619108: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8261910C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82619110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261911C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619120: 386ACB9C  addi r3, r10, -0x3464
	ctx.r[3].s64 = ctx.r[10].s64 + -13412;
	// 82619124: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619128: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261912C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261913C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619144: 4BE4DCDD  bl 0x82466e20
	ctx.lr = 0x82619148;
	sub_82466E20(ctx, base);
	// 82619148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261914C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619158 size=116
    let mut pc: u32 = 0x82619158;
    'dispatch: loop {
        match pc {
            0x82619158 => {
    //   block [0x82619158..0x826191CC)
	// 82619158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261915C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619164: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619168: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261916C: 390BA530  addi r8, r11, -0x5ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -23248;
	// 82619170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619174: 392A0070  addi r9, r10, 0x70
	ctx.r[9].s64 = ctx.r[10].s64 + 112;
	// 82619178: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261917C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82619180: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619184: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261918C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261919C: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 826191A0: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826191A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826191A8: 386BCBCC  addi r3, r11, -0x3434
	ctx.r[3].s64 = ctx.r[11].s64 + -13364;
	// 826191AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826191B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826191B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826191B8: 4BE4DC69  bl 0x82466e20
	ctx.lr = 0x826191BC;
	sub_82466E20(ctx, base);
	// 826191BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826191C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826191C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826191C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826191D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826191D0 size=108
    let mut pc: u32 = 0x826191D0;
    'dispatch: loop {
        match pc {
            0x826191D0 => {
    //   block [0x826191D0..0x8261923C)
	// 826191D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826191D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826191D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826191DC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826191E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826191E4: 38EBA548  addi r7, r11, -0x5ab8
	ctx.r[7].s64 = ctx.r[11].s64 + -23224;
	// 826191E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826191EC: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826191F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826191F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826191F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826191FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619200: 386ACBFC  addi r3, r10, -0x3404
	ctx.r[3].s64 = ctx.r[10].s64 + -13316;
	// 82619204: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261920C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261921C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619228: 4BE4DBF9  bl 0x82466e20
	ctx.lr = 0x8261922C;
	sub_82466E20(ctx, base);
	// 8261922C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619240 size=112
    let mut pc: u32 = 0x82619240;
    'dispatch: loop {
        match pc {
            0x82619240 => {
    //   block [0x82619240..0x826192B0)
	// 82619240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261924C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619250: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619254: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261925C: 390BA578  addi r8, r11, -0x5a88
	ctx.r[8].s64 = ctx.r[11].s64 + -23176;
	// 82619260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82619264: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82619268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261926C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619278: 386ACC2C  addi r3, r10, -0x33d4
	ctx.r[3].s64 = ctx.r[10].s64 + -13268;
	// 8261927C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261928C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261929C: 4BE4DB85  bl 0x82466e20
	ctx.lr = 0x826192A0;
	sub_82466E20(ctx, base);
	// 826192A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826192A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826192A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826192AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826192B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826192B0 size=112
    let mut pc: u32 = 0x826192B0;
    'dispatch: loop {
        match pc {
            0x826192B0 => {
    //   block [0x826192B0..0x82619320)
	// 826192B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826192B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826192B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826192BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826192C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826192C4: 392A00D0  addi r9, r10, 0xd0
	ctx.r[9].s64 = ctx.r[10].s64 + 208;
	// 826192C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826192CC: 390BA598  addi r8, r11, -0x5a68
	ctx.r[8].s64 = ctx.r[11].s64 + -23144;
	// 826192D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826192D4: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826192D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826192DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826192E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826192E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826192E8: 386ACC5C  addi r3, r10, -0x33a4
	ctx.r[3].s64 = ctx.r[10].s64 + -13220;
	// 826192EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826192F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826192F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826192F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826192FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261930C: 4BE4DB15  bl 0x82466e20
	ctx.lr = 0x82619310;
	sub_82466E20(ctx, base);
	// 82619310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261931C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619320 size=112
    let mut pc: u32 = 0x82619320;
    'dispatch: loop {
        match pc {
            0x82619320 => {
    //   block [0x82619320..0x82619390)
	// 82619320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261932C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619330: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619334: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261933C: 390BA640  addi r8, r11, -0x59c0
	ctx.r[8].s64 = ctx.r[11].s64 + -22976;
	// 82619340: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82619344: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82619348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261934C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619358: 386ACC8C  addi r3, r10, -0x3374
	ctx.r[3].s64 = ctx.r[10].s64 + -13172;
	// 8261935C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261936C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261937C: 4BE4DAA5  bl 0x82466e20
	ctx.lr = 0x82619380;
	sub_82466E20(ctx, base);
	// 82619380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261938C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619390 size=112
    let mut pc: u32 = 0x82619390;
    'dispatch: loop {
        match pc {
            0x82619390 => {
    //   block [0x82619390..0x82619400)
	// 82619390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261939C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826193A0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826193A4: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 826193A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826193AC: 390BA688  addi r8, r11, -0x5978
	ctx.r[8].s64 = ctx.r[11].s64 + -22904;
	// 826193B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826193B4: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826193B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826193BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826193C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826193C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826193C8: 386ACCBC  addi r3, r10, -0x3344
	ctx.r[3].s64 = ctx.r[10].s64 + -13124;
	// 826193CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826193D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826193D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826193D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826193DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826193E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826193E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826193E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826193EC: 4BE4DA35  bl 0x82466e20
	ctx.lr = 0x826193F0;
	sub_82466E20(ctx, base);
	// 826193F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826193F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826193F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826193FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619400 size=100
    let mut pc: u32 = 0x82619400;
    'dispatch: loop {
        match pc {
            0x82619400 => {
    //   block [0x82619400..0x82619464)
	// 82619400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261940C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619414: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261941C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619420: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 82619424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261942C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619434: 386ACCEC  addi r3, r10, -0x3314
	ctx.r[3].s64 = ctx.r[10].s64 + -13076;
	// 82619438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261943C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619440: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82619444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619448: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261944C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619450: 4BE4D9D1  bl 0x82466e20
	ctx.lr = 0x82619454;
	sub_82466E20(ctx, base);
	// 82619454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261945C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619468 size=112
    let mut pc: u32 = 0x82619468;
    'dispatch: loop {
        match pc {
            0x82619468 => {
    //   block [0x82619468..0x826194D8)
	// 82619468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261946C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619474: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619478: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261947C: 38AAC8FC  addi r5, r10, -0x3704
	ctx.r[5].s64 = ctx.r[10].s64 + -14084;
	// 82619480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619484: 390BA748  addi r8, r11, -0x58b8
	ctx.r[8].s64 = ctx.r[11].s64 + -22712;
	// 82619488: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261948C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 82619490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619494: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261949C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826194A0: 386ACD1C  addi r3, r10, -0x32e4
	ctx.r[3].s64 = ctx.r[10].s64 + -13028;
	// 826194A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826194A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826194AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826194B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826194B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826194B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826194BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826194C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826194C4: 4BE4D95D  bl 0x82466e20
	ctx.lr = 0x826194C8;
	sub_82466E20(ctx, base);
	// 826194C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826194CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826194D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826194D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826194D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826194D8 size=112
    let mut pc: u32 = 0x826194D8;
    'dispatch: loop {
        match pc {
            0x826194D8 => {
    //   block [0x826194D8..0x82619548)
	// 826194D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826194DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826194E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826194E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826194E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826194EC: 38AAC77C  addi r5, r10, -0x3884
	ctx.r[5].s64 = ctx.r[10].s64 + -14468;
	// 826194F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826194F4: 390BA778  addi r8, r11, -0x5888
	ctx.r[8].s64 = ctx.r[11].s64 + -22664;
	// 826194F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826194FC: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 82619500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619504: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261950C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619510: 386ACD4C  addi r3, r10, -0x32b4
	ctx.r[3].s64 = ctx.r[10].s64 + -12980;
	// 82619514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261951C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261952C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619534: 4BE4D8ED  bl 0x82466e20
	ctx.lr = 0x82619538;
	sub_82466E20(ctx, base);
	// 82619538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261953C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619548 size=108
    let mut pc: u32 = 0x82619548;
    'dispatch: loop {
        match pc {
            0x82619548 => {
    //   block [0x82619548..0x826195B4)
	// 82619548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261954C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619554: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261955C: 38EBA790  addi r7, r11, -0x5870
	ctx.r[7].s64 = ctx.r[11].s64 + -22640;
	// 82619560: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82619564: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82619568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261956C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619578: 386ACD7C  addi r3, r10, -0x3284
	ctx.r[3].s64 = ctx.r[10].s64 + -12932;
	// 8261957C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261958C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261959C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826195A0: 4BE4D881  bl 0x82466e20
	ctx.lr = 0x826195A4;
	sub_82466E20(ctx, base);
	// 826195A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826195A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826195AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826195B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826195B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826195B8 size=112
    let mut pc: u32 = 0x826195B8;
    'dispatch: loop {
        match pc {
            0x826195B8 => {
    //   block [0x826195B8..0x82619628)
	// 826195B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826195BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826195C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826195C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826195C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826195CC: 38AACCEC  addi r5, r10, -0x3314
	ctx.r[5].s64 = ctx.r[10].s64 + -13076;
	// 826195D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826195D4: 390BA7C0  addi r8, r11, -0x5840
	ctx.r[8].s64 = ctx.r[11].s64 + -22592;
	// 826195D8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826195DC: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826195E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826195E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826195E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826195EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826195F0: 386ACDAC  addi r3, r10, -0x3254
	ctx.r[3].s64 = ctx.r[10].s64 + -12884;
	// 826195F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826195F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826195FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261960C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619614: 4BE4D80D  bl 0x82466e20
	ctx.lr = 0x82619618;
	sub_82466E20(ctx, base);
	// 82619618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261961C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619628 size=112
    let mut pc: u32 = 0x82619628;
    'dispatch: loop {
        match pc {
            0x82619628 => {
    //   block [0x82619628..0x82619698)
	// 82619628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261962C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619634: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82619638: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261963C: 392A00FC  addi r9, r10, 0xfc
	ctx.r[9].s64 = ctx.r[10].s64 + 252;
	// 82619640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619644: 390BA850  addi r8, r11, -0x57b0
	ctx.r[8].s64 = ctx.r[11].s64 + -22448;
	// 82619648: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8261964C: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 82619650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619654: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261965C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619660: 386ACDDC  addi r3, r10, -0x3224
	ctx.r[3].s64 = ctx.r[10].s64 + -12836;
	// 82619664: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619668: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261966C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261967C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619684: 4BE4D79D  bl 0x82466e20
	ctx.lr = 0x82619688;
	sub_82466E20(ctx, base);
	// 82619688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261968C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619698 size=112
    let mut pc: u32 = 0x82619698;
    'dispatch: loop {
        match pc {
            0x82619698 => {
    //   block [0x82619698..0x82619708)
	// 82619698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261969C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826196A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826196A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826196A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826196AC: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 826196B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826196B4: 390BA898  addi r8, r11, -0x5768
	ctx.r[8].s64 = ctx.r[11].s64 + -22376;
	// 826196B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826196BC: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826196C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826196C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826196C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826196CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826196D0: 386ACE0C  addi r3, r10, -0x31f4
	ctx.r[3].s64 = ctx.r[10].s64 + -12788;
	// 826196D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826196D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826196DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826196E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826196E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826196E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826196EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826196F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826196F4: 4BE4D72D  bl 0x82466e20
	ctx.lr = 0x826196F8;
	sub_82466E20(ctx, base);
	// 826196F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826196FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619708 size=108
    let mut pc: u32 = 0x82619708;
    'dispatch: loop {
        match pc {
            0x82619708 => {
    //   block [0x82619708..0x82619774)
	// 82619708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261970C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619714: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261971C: 38EBA8B0  addi r7, r11, -0x5750
	ctx.r[7].s64 = ctx.r[11].s64 + -22352;
	// 82619720: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82619724: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82619728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261972C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619738: 386ACE3C  addi r3, r10, -0x31c4
	ctx.r[3].s64 = ctx.r[10].s64 + -12740;
	// 8261973C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261974C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261975C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619760: 4BE4D6C1  bl 0x82466e20
	ctx.lr = 0x82619764;
	sub_82466E20(ctx, base);
	// 82619764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261976C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619778 size=116
    let mut pc: u32 = 0x82619778;
    'dispatch: loop {
        match pc {
            0x82619778 => {
    //   block [0x82619778..0x826197EC)
	// 82619778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261977C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619784: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82619788: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8261978C: 390AA940  addi r8, r10, -0x56c0
	ctx.r[8].s64 = ctx.r[10].s64 + -22208;
	// 82619790: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619794: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82619798: 38AACCEC  addi r5, r10, -0x3314
	ctx.r[5].s64 = ctx.r[10].s64 + -13076;
	// 8261979C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826197A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826197A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826197A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826197AC: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826197B0: 396B0110  addi r11, r11, 0x110
	ctx.r[11].s64 = ctx.r[11].s64 + 272;
	// 826197B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826197B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826197BC: 386ACE6C  addi r3, r10, -0x3194
	ctx.r[3].s64 = ctx.r[10].s64 + -12692;
	// 826197C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826197C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826197C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826197CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826197D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826197D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826197D8: 4BE4D649  bl 0x82466e20
	ctx.lr = 0x826197DC;
	sub_82466E20(ctx, base);
	// 826197DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826197E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826197E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826197E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826197F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826197F0 size=112
    let mut pc: u32 = 0x826197F0;
    'dispatch: loop {
        match pc {
            0x826197F0 => {
    //   block [0x826197F0..0x82619860)
	// 826197F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826197F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826197F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826197FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82619800: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619804: 392A015C  addi r9, r10, 0x15c
	ctx.r[9].s64 = ctx.r[10].s64 + 348;
	// 82619808: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261980C: 390BAA20  addi r8, r11, -0x55e0
	ctx.r[8].s64 = ctx.r[11].s64 + -21984;
	// 82619810: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82619814: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82619818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261981C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619828: 386ACE9C  addi r3, r10, -0x3164
	ctx.r[3].s64 = ctx.r[10].s64 + -12644;
	// 8261982C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619830: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82619834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261983C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261984C: 4BE4D5D5  bl 0x82466e20
	ctx.lr = 0x82619850;
	sub_82466E20(ctx, base);
	// 82619850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261985C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619860 size=112
    let mut pc: u32 = 0x82619860;
    'dispatch: loop {
        match pc {
            0x82619860 => {
    //   block [0x82619860..0x826198D0)
	// 82619860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261986C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619870: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619874: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619878: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261987C: 390BAA80  addi r8, r11, -0x5580
	ctx.r[8].s64 = ctx.r[11].s64 + -21888;
	// 82619880: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82619884: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 82619888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261988C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619898: 386ACECC  addi r3, r10, -0x3134
	ctx.r[3].s64 = ctx.r[10].s64 + -12596;
	// 8261989C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826198A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826198A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826198A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826198AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826198B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826198B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826198B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826198BC: 4BE4D565  bl 0x82466e20
	ctx.lr = 0x826198C0;
	sub_82466E20(ctx, base);
	// 826198C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826198C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826198C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826198CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826198D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826198D0 size=108
    let mut pc: u32 = 0x826198D0;
    'dispatch: loop {
        match pc {
            0x826198D0 => {
    //   block [0x826198D0..0x8261993C)
	// 826198D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826198D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826198D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826198DC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826198E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826198E4: 38EBAA98  addi r7, r11, -0x5568
	ctx.r[7].s64 = ctx.r[11].s64 + -21864;
	// 826198E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826198EC: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 826198F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826198F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826198F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826198FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619900: 386ACEFC  addi r3, r10, -0x3104
	ctx.r[3].s64 = ctx.r[10].s64 + -12548;
	// 82619904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261990C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261991C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619928: 4BE4D4F9  bl 0x82466e20
	ctx.lr = 0x8261992C;
	sub_82466E20(ctx, base);
	// 8261992C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619940 size=112
    let mut pc: u32 = 0x82619940;
    'dispatch: loop {
        match pc {
            0x82619940 => {
    //   block [0x82619940..0x826199B0)
	// 82619940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261994C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619950: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619954: 38AACCEC  addi r5, r10, -0x3314
	ctx.r[5].s64 = ctx.r[10].s64 + -13076;
	// 82619958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261995C: 390BAAE0  addi r8, r11, -0x5520
	ctx.r[8].s64 = ctx.r[11].s64 + -21792;
	// 82619960: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82619964: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 82619968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261996C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619978: 386ACF2C  addi r3, r10, -0x30d4
	ctx.r[3].s64 = ctx.r[10].s64 + -12500;
	// 8261997C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261998C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261999C: 4BE4D485  bl 0x82466e20
	ctx.lr = 0x826199A0;
	sub_82466E20(ctx, base);
	// 826199A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826199A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826199A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826199AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826199B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826199B0 size=112
    let mut pc: u32 = 0x826199B0;
    'dispatch: loop {
        match pc {
            0x826199B0 => {
    //   block [0x826199B0..0x82619A20)
	// 826199B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826199B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826199B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826199BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826199C0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 826199C4: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 826199C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826199CC: 390BAB58  addi r8, r11, -0x54a8
	ctx.r[8].s64 = ctx.r[11].s64 + -21672;
	// 826199D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826199D4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826199D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826199DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 826199E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826199E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826199E8: 386ACF5C  addi r3, r10, -0x30a4
	ctx.r[3].s64 = ctx.r[10].s64 + -12452;
	// 826199EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826199F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826199F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826199F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826199FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619A0C: 4BE4D415  bl 0x82466e20
	ctx.lr = 0x82619A10;
	sub_82466E20(ctx, base);
	// 82619A10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619A1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619A20 size=108
    let mut pc: u32 = 0x82619A20;
    'dispatch: loop {
        match pc {
            0x82619A20 => {
    //   block [0x82619A20..0x82619A8C)
	// 82619A20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619A24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619A28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619A2C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619A30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619A34: 38EBAB88  addi r7, r11, -0x5478
	ctx.r[7].s64 = ctx.r[11].s64 + -21624;
	// 82619A38: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82619A3C: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 82619A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619A44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619A48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619A4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619A50: 386ACF8C  addi r3, r10, -0x3074
	ctx.r[3].s64 = ctx.r[10].s64 + -12404;
	// 82619A54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619A64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619A74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619A78: 4BE4D3A9  bl 0x82466e20
	ctx.lr = 0x82619A7C;
	sub_82466E20(ctx, base);
	// 82619A7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619A80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619A84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619A90 size=108
    let mut pc: u32 = 0x82619A90;
    'dispatch: loop {
        match pc {
            0x82619A90 => {
    //   block [0x82619A90..0x82619AFC)
	// 82619A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619A98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619A9C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619AA4: 38EBABE8  addi r7, r11, -0x5418
	ctx.r[7].s64 = ctx.r[11].s64 + -21528;
	// 82619AA8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82619AAC: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82619AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619AB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619AB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619AC0: 386ACFBC  addi r3, r10, -0x3044
	ctx.r[3].s64 = ctx.r[10].s64 + -12356;
	// 82619AC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619AE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619AE8: 4BE4D339  bl 0x82466e20
	ctx.lr = 0x82619AEC;
	sub_82466E20(ctx, base);
	// 82619AEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619B00 size=112
    let mut pc: u32 = 0x82619B00;
    'dispatch: loop {
        match pc {
            0x82619B00 => {
    //   block [0x82619B00..0x82619B70)
	// 82619B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619B08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619B0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619B10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619B14: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619B18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619B1C: 390BAC60  addi r8, r11, -0x53a0
	ctx.r[8].s64 = ctx.r[11].s64 + -21408;
	// 82619B20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82619B24: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82619B28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619B2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619B30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619B38: 386ACFEC  addi r3, r10, -0x3014
	ctx.r[3].s64 = ctx.r[10].s64 + -12308;
	// 82619B3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619B40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619B54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619B5C: 4BE4D2C5  bl 0x82466e20
	ctx.lr = 0x82619B60;
	sub_82466E20(ctx, base);
	// 82619B60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619B64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619B68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82619B70 size=24
    let mut pc: u32 = 0x82619B70;
    'dispatch: loop {
        match pc {
            0x82619B70 => {
    //   block [0x82619B70..0x82619B88)
	// 82619B70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619B74: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82619B78: 394AE458  addi r10, r10, -0x1ba8
	ctx.r[10].s64 = ctx.r[10].s64 + -7080;
	// 82619B7C: 816BAA1C  lwz r11, -0x55e4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21988 as u32) ) } as u64;
	// 82619B80: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82619B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619B88 size=116
    let mut pc: u32 = 0x82619B88;
    'dispatch: loop {
        match pc {
            0x82619B88 => {
    //   block [0x82619B88..0x82619BFC)
	// 82619B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619B94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619B98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82619B9C: 390BE458  addi r8, r11, -0x1ba8
	ctx.r[8].s64 = ctx.r[11].s64 + -7080;
	// 82619BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619BA4: 392A01A0  addi r9, r10, 0x1a0
	ctx.r[9].s64 = ctx.r[10].s64 + 416;
	// 82619BA8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619BAC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82619BB0: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 82619BB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619BB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619BBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619BC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619BC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619BCC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82619BD0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82619BD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619BD8: 386BD01C  addi r3, r11, -0x2fe4
	ctx.r[3].s64 = ctx.r[11].s64 + -12260;
	// 82619BDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82619BE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619BE8: 4BE4D239  bl 0x82466e20
	ctx.lr = 0x82619BEC;
	sub_82466E20(ctx, base);
	// 82619BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619C00 size=112
    let mut pc: u32 = 0x82619C00;
    'dispatch: loop {
        match pc {
            0x82619C00 => {
    //   block [0x82619C00..0x82619C70)
	// 82619C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619C0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619C10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619C14: 38AAD01C  addi r5, r10, -0x2fe4
	ctx.r[5].s64 = ctx.r[10].s64 + -12260;
	// 82619C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619C1C: 390BACA8  addi r8, r11, -0x5358
	ctx.r[8].s64 = ctx.r[11].s64 + -21336;
	// 82619C20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82619C24: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82619C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619C2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619C30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619C38: 386AD04C  addi r3, r10, -0x2fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -12212;
	// 82619C3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619C5C: 4BE4D1C5  bl 0x82466e20
	ctx.lr = 0x82619C60;
	sub_82466E20(ctx, base);
	// 82619C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82619C70 size=24
    let mut pc: u32 = 0x82619C70;
    'dispatch: loop {
        match pc {
            0x82619C70 => {
    //   block [0x82619C70..0x82619C88)
	// 82619C70: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619C74: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 82619C78: 394AE470  addi r10, r10, -0x1b90
	ctx.r[10].s64 = ctx.r[10].s64 + -7056;
	// 82619C7C: 816BACD8  lwz r11, -0x5328(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21288 as u32) ) } as u64;
	// 82619C80: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82619C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619C88 size=116
    let mut pc: u32 = 0x82619C88;
    'dispatch: loop {
        match pc {
            0x82619C88 => {
    //   block [0x82619C88..0x82619CFC)
	// 82619C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619C94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619C98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82619C9C: 390BE470  addi r8, r11, -0x1b90
	ctx.r[8].s64 = ctx.r[11].s64 + -7056;
	// 82619CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619CA4: 392A01DC  addi r9, r10, 0x1dc
	ctx.r[9].s64 = ctx.r[10].s64 + 476;
	// 82619CA8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619CAC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82619CB0: 38AAD04C  addi r5, r10, -0x2fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -12212;
	// 82619CB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619CBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619CCC: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 82619CD0: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 82619CD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82619CD8: 386BD07C  addi r3, r11, -0x2f84
	ctx.r[3].s64 = ctx.r[11].s64 + -12164;
	// 82619CDC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82619CE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619CE8: 4BE4D139  bl 0x82466e20
	ctx.lr = 0x82619CEC;
	sub_82466E20(ctx, base);
	// 82619CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619CF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619CF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619D00 size=112
    let mut pc: u32 = 0x82619D00;
    'dispatch: loop {
        match pc {
            0x82619D00 => {
    //   block [0x82619D00..0x82619D70)
	// 82619D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619D0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619D10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619D14: 38AAD04C  addi r5, r10, -0x2fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -12212;
	// 82619D18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619D1C: 390BACE0  addi r8, r11, -0x5320
	ctx.r[8].s64 = ctx.r[11].s64 + -21280;
	// 82619D20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82619D24: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82619D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619D2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619D30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619D38: 386AD0AC  addi r3, r10, -0x2f54
	ctx.r[3].s64 = ctx.r[10].s64 + -12116;
	// 82619D3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619D4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619D5C: 4BE4D0C5  bl 0x82466e20
	ctx.lr = 0x82619D60;
	sub_82466E20(ctx, base);
	// 82619D60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619D6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619D70 size=112
    let mut pc: u32 = 0x82619D70;
    'dispatch: loop {
        match pc {
            0x82619D70 => {
    //   block [0x82619D70..0x82619DE0)
	// 82619D70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619D74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619D78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619D7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619D80: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619D84: 38AAD04C  addi r5, r10, -0x2fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -12212;
	// 82619D88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619D8C: 390BAD40  addi r8, r11, -0x52c0
	ctx.r[8].s64 = ctx.r[11].s64 + -21184;
	// 82619D90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82619D94: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82619D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619D9C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619DA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619DA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619DA8: 386AD0DC  addi r3, r10, -0x2f24
	ctx.r[3].s64 = ctx.r[10].s64 + -12068;
	// 82619DAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619DCC: 4BE4D055  bl 0x82466e20
	ctx.lr = 0x82619DD0;
	sub_82466E20(ctx, base);
	// 82619DD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619DD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619DD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619DE0 size=112
    let mut pc: u32 = 0x82619DE0;
    'dispatch: loop {
        match pc {
            0x82619DE0 => {
    //   block [0x82619DE0..0x82619E50)
	// 82619DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619DEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619DF0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619DF4: 38AAD04C  addi r5, r10, -0x2fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -12212;
	// 82619DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619DFC: 390BAD70  addi r8, r11, -0x5290
	ctx.r[8].s64 = ctx.r[11].s64 + -21136;
	// 82619E00: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82619E04: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82619E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619E0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619E14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619E18: 386AD10C  addi r3, r10, -0x2ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -12020;
	// 82619E1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619E2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619E34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619E3C: 4BE4CFE5  bl 0x82466e20
	ctx.lr = 0x82619E40;
	sub_82466E20(ctx, base);
	// 82619E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619E50 size=108
    let mut pc: u32 = 0x82619E50;
    'dispatch: loop {
        match pc {
            0x82619E50 => {
    //   block [0x82619E50..0x82619EBC)
	// 82619E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619E5C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619E64: 38EBADB8  addi r7, r11, -0x5248
	ctx.r[7].s64 = ctx.r[11].s64 + -21064;
	// 82619E68: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82619E6C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82619E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619E74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619E78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619E80: 386AD13C  addi r3, r10, -0x2ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -11972;
	// 82619E84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619EA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619EA8: 4BE4CF79  bl 0x82466e20
	ctx.lr = 0x82619EAC;
	sub_82466E20(ctx, base);
	// 82619EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619EC0 size=112
    let mut pc: u32 = 0x82619EC0;
    'dispatch: loop {
        match pc {
            0x82619EC0 => {
    //   block [0x82619EC0..0x82619F30)
	// 82619EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619ECC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619ED0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619ED4: 38AAC8CC  addi r5, r10, -0x3734
	ctx.r[5].s64 = ctx.r[10].s64 + -14132;
	// 82619ED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619EDC: 390BADE8  addi r8, r11, -0x5218
	ctx.r[8].s64 = ctx.r[11].s64 + -21016;
	// 82619EE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82619EE4: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82619EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619EEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619EF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82619EF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619EF8: 386AD16C  addi r3, r10, -0x2e94
	ctx.r[3].s64 = ctx.r[10].s64 + -11924;
	// 82619EFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82619F00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619F0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619F1C: 4BE4CF05  bl 0x82466e20
	ctx.lr = 0x82619F20;
	sub_82466E20(ctx, base);
	// 82619F20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619F24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619F28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619F2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619F30 size=108
    let mut pc: u32 = 0x82619F30;
    'dispatch: loop {
        match pc {
            0x82619F30 => {
    //   block [0x82619F30..0x82619F9C)
	// 82619F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619F3C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619F40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619F44: 38EBAE00  addi r7, r11, -0x5200
	ctx.r[7].s64 = ctx.r[11].s64 + -20992;
	// 82619F48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82619F4C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 82619F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619F54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619F58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619F60: 386AD19C  addi r3, r10, -0x2e64
	ctx.r[3].s64 = ctx.r[10].s64 + -11876;
	// 82619F64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619F68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619F6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619F88: 4BE4CE99  bl 0x82466e20
	ctx.lr = 0x82619F8C;
	sub_82466E20(ctx, base);
	// 82619F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82619F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82619F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82619F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82619FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82619FA0 size=108
    let mut pc: u32 = 0x82619FA0;
    'dispatch: loop {
        match pc {
            0x82619FA0 => {
    //   block [0x82619FA0..0x8261A00C)
	// 82619FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82619FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82619FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82619FAC: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 82619FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82619FB4: 38EBAE48  addi r7, r11, -0x51b8
	ctx.r[7].s64 = ctx.r[11].s64 + -20920;
	// 82619FB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82619FBC: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 82619FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82619FC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82619FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82619FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82619FD0: 386AD1CC  addi r3, r10, -0x2e34
	ctx.r[3].s64 = ctx.r[10].s64 + -11828;
	// 82619FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82619FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82619FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82619FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82619FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82619FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82619FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82619FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82619FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82619FF8: 4BE4CE29  bl 0x82466e20
	ctx.lr = 0x82619FFC;
	sub_82466E20(ctx, base);
	// 82619FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A010 size=116
    let mut pc: u32 = 0x8261A010;
    'dispatch: loop {
        match pc {
            0x8261A010 => {
    //   block [0x8261A010..0x8261A084)
	// 8261A010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A01C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261A020: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A024: 392B0210  addi r9, r11, 0x210
	ctx.r[9].s64 = ctx.r[11].s64 + 528;
	// 8261A028: 38AAD64C  addi r5, r10, -0x29b4
	ctx.r[5].s64 = ctx.r[10].s64 + -10676;
	// 8261A02C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A030: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 8261A034: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 8261A038: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A03C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 8261A040: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A044: 396BAEA8  addi r11, r11, -0x5158
	ctx.r[11].s64 = ctx.r[11].s64 + -20824;
	// 8261A048: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8261A04C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A050: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8261A054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A058: 386AD1FC  addi r3, r10, -0x2e04
	ctx.r[3].s64 = ctx.r[10].s64 + -11780;
	// 8261A05C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261A060: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8261A064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A068: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8261A06C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A070: 4BE4CDB1  bl 0x82466e20
	ctx.lr = 0x8261A074;
	sub_82466E20(ctx, base);
	// 8261A074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A088 size=100
    let mut pc: u32 = 0x8261A088;
    'dispatch: loop {
        match pc {
            0x8261A088 => {
    //   block [0x8261A088..0x8261A0EC)
	// 8261A088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A094: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A09C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261A0A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A0A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A0A8: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 8261A0AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A0B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A0B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A0B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A0BC: 386AD22C  addi r3, r10, -0x2dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -11732;
	// 8261A0C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A0C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A0C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A0D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A0D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A0D8: 4BE4CD49  bl 0x82466e20
	ctx.lr = 0x8261A0DC;
	sub_82466E20(ctx, base);
	// 8261A0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A0F0 size=100
    let mut pc: u32 = 0x8261A0F0;
    'dispatch: loop {
        match pc {
            0x8261A0F0 => {
    //   block [0x8261A0F0..0x8261A154)
	// 8261A0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A0FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A104: 38AAD2BC  addi r5, r10, -0x2d44
	ctx.r[5].s64 = ctx.r[10].s64 + -11588;
	// 8261A108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A110: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8261A114: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A124: 386AD25C  addi r3, r10, -0x2da4
	ctx.r[3].s64 = ctx.r[10].s64 + -11684;
	// 8261A128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A12C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A130: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A138: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A13C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A140: 4BE4CCE1  bl 0x82466e20
	ctx.lr = 0x8261A144;
	sub_82466E20(ctx, base);
	// 8261A144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A158 size=100
    let mut pc: u32 = 0x8261A158;
    'dispatch: loop {
        match pc {
            0x8261A158 => {
    //   block [0x8261A158..0x8261A1BC)
	// 8261A158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A16C: 38AAD1FC  addi r5, r10, -0x2e04
	ctx.r[5].s64 = ctx.r[10].s64 + -11780;
	// 8261A170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A178: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 8261A17C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A18C: 386AD28C  addi r3, r10, -0x2d74
	ctx.r[3].s64 = ctx.r[10].s64 + -11636;
	// 8261A190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A194: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A198: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A1A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A1A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A1A8: 4BE4CC79  bl 0x82466e20
	ctx.lr = 0x8261A1AC;
	sub_82466E20(ctx, base);
	// 8261A1AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A1C0 size=104
    let mut pc: u32 = 0x8261A1C0;
    'dispatch: loop {
        match pc {
            0x8261A1C0 => {
    //   block [0x8261A1C0..0x8261A228)
	// 8261A1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A1CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261A1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A1D4: 392A0294  addi r9, r10, 0x294
	ctx.r[9].s64 = ctx.r[10].s64 + 660;
	// 8261A1D8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A1DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A1E0: 38AAD22C  addi r5, r10, -0x2dd4
	ctx.r[5].s64 = ctx.r[10].s64 + -11732;
	// 8261A1E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A1E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A1F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A1F4: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 8261A1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A1FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A200: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A208: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A20C: 386AD2BC  addi r3, r10, -0x2d44
	ctx.r[3].s64 = ctx.r[10].s64 + -11588;
	// 8261A210: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261A214: 4BE4CC0D  bl 0x82466e20
	ctx.lr = 0x8261A218;
	sub_82466E20(ctx, base);
	// 8261A218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A228 size=108
    let mut pc: u32 = 0x8261A228;
    'dispatch: loop {
        match pc {
            0x8261A228 => {
    //   block [0x8261A228..0x8261A294)
	// 8261A228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A22C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A234: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A23C: 38EBB05C  addi r7, r11, -0x4fa4
	ctx.r[7].s64 = ctx.r[11].s64 + -20388;
	// 8261A240: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261A244: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 8261A248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A24C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A250: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261A254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A258: 386AD2EC  addi r3, r10, -0x2d14
	ctx.r[3].s64 = ctx.r[10].s64 + -11540;
	// 8261A25C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261A260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A26C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A27C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261A280: 4BE4CBA1  bl 0x82466e20
	ctx.lr = 0x8261A284;
	sub_82466E20(ctx, base);
	// 8261A284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A298 size=112
    let mut pc: u32 = 0x8261A298;
    'dispatch: loop {
        match pc {
            0x8261A298 => {
    //   block [0x8261A298..0x8261A308)
	// 8261A298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A2A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A2A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A2AC: 38AAD2BC  addi r5, r10, -0x2d44
	ctx.r[5].s64 = ctx.r[10].s64 + -11588;
	// 8261A2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A2B4: 390BB090  addi r8, r11, -0x4f70
	ctx.r[8].s64 = ctx.r[11].s64 + -20336;
	// 8261A2B8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261A2BC: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 8261A2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A2C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A2D0: 386AD31C  addi r3, r10, -0x2ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -11492;
	// 8261A2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A2F4: 4BE4CB2D  bl 0x82466e20
	ctx.lr = 0x8261A2F8;
	sub_82466E20(ctx, base);
	// 8261A2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261A308 size=24
    let mut pc: u32 = 0x8261A308;
    'dispatch: loop {
        match pc {
            0x8261A308 => {
    //   block [0x8261A308..0x8261A320)
	// 8261A308: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A30C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261A310: 394AE4E8  addi r10, r10, -0x1b18
	ctx.r[10].s64 = ctx.r[10].s64 + -6936;
	// 8261A314: 816BB08C  lwz r11, -0x4f74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20340 as u32) ) } as u64;
	// 8261A318: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8261A31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A320 size=116
    let mut pc: u32 = 0x8261A320;
    'dispatch: loop {
        match pc {
            0x8261A320 => {
    //   block [0x8261A320..0x8261A394)
	// 8261A320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A32C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A330: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261A334: 390BE4E8  addi r8, r11, -0x1b18
	ctx.r[8].s64 = ctx.r[11].s64 + -6936;
	// 8261A338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A33C: 392A02F8  addi r9, r10, 0x2f8
	ctx.r[9].s64 = ctx.r[10].s64 + 760;
	// 8261A340: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A344: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8261A348: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261A34C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A354: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A35C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A364: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261A368: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8261A36C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261A370: 386BD34C  addi r3, r11, -0x2cb4
	ctx.r[3].s64 = ctx.r[11].s64 + -11444;
	// 8261A374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261A378: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A380: 4BE4CAA1  bl 0x82466e20
	ctx.lr = 0x8261A384;
	sub_82466E20(ctx, base);
	// 8261A384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A38C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A398 size=100
    let mut pc: u32 = 0x8261A398;
    'dispatch: loop {
        match pc {
            0x8261A398 => {
    //   block [0x8261A398..0x8261A3FC)
	// 8261A398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A3A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A3AC: 38AAD34C  addi r5, r10, -0x2cb4
	ctx.r[5].s64 = ctx.r[10].s64 + -11444;
	// 8261A3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A3B8: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 8261A3BC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A3CC: 386AD37C  addi r3, r10, -0x2c84
	ctx.r[3].s64 = ctx.r[10].s64 + -11396;
	// 8261A3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A3D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A3D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A3E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A3E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A3E8: 4BE4CA39  bl 0x82466e20
	ctx.lr = 0x8261A3EC;
	sub_82466E20(ctx, base);
	// 8261A3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A400 size=100
    let mut pc: u32 = 0x8261A400;
    'dispatch: loop {
        match pc {
            0x8261A400 => {
    //   block [0x8261A400..0x8261A464)
	// 8261A400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A40C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A414: 38AAD3DC  addi r5, r10, -0x2c24
	ctx.r[5].s64 = ctx.r[10].s64 + -11300;
	// 8261A418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A420: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8261A424: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A434: 386AD3AC  addi r3, r10, -0x2c54
	ctx.r[3].s64 = ctx.r[10].s64 + -11348;
	// 8261A438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A43C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A440: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A448: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A450: 4BE4C9D1  bl 0x82466e20
	ctx.lr = 0x8261A454;
	sub_82466E20(ctx, base);
	// 8261A454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A468 size=112
    let mut pc: u32 = 0x8261A468;
    'dispatch: loop {
        match pc {
            0x8261A468 => {
    //   block [0x8261A468..0x8261A4D8)
	// 8261A468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A474: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A478: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A47C: 38AAD34C  addi r5, r10, -0x2cb4
	ctx.r[5].s64 = ctx.r[10].s64 + -11444;
	// 8261A480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A484: 390BB138  addi r8, r11, -0x4ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -20168;
	// 8261A488: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261A48C: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 8261A490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A494: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A4A0: 386AD3DC  addi r3, r10, -0x2c24
	ctx.r[3].s64 = ctx.r[10].s64 + -11300;
	// 8261A4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A4C4: 4BE4C95D  bl 0x82466e20
	ctx.lr = 0x8261A4C8;
	sub_82466E20(ctx, base);
	// 8261A4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A4D8 size=100
    let mut pc: u32 = 0x8261A4D8;
    'dispatch: loop {
        match pc {
            0x8261A4D8 => {
    //   block [0x8261A4D8..0x8261A53C)
	// 8261A4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A4E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A4E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A4EC: 38AAD3DC  addi r5, r10, -0x2c24
	ctx.r[5].s64 = ctx.r[10].s64 + -11300;
	// 8261A4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A4F8: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 8261A4FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A50C: 386AD40C  addi r3, r10, -0x2bf4
	ctx.r[3].s64 = ctx.r[10].s64 + -11252;
	// 8261A510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A514: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A518: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A520: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A528: 4BE4C8F9  bl 0x82466e20
	ctx.lr = 0x8261A52C;
	sub_82466E20(ctx, base);
	// 8261A52C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A540 size=100
    let mut pc: u32 = 0x8261A540;
    'dispatch: loop {
        match pc {
            0x8261A540 => {
    //   block [0x8261A540..0x8261A5A4)
	// 8261A540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A54C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A554: 38AAD34C  addi r5, r10, -0x2cb4
	ctx.r[5].s64 = ctx.r[10].s64 + -11444;
	// 8261A558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A55C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A560: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 8261A564: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A574: 386AD43C  addi r3, r10, -0x2bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -11204;
	// 8261A578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A57C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A580: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A588: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A590: 4BE4C891  bl 0x82466e20
	ctx.lr = 0x8261A594;
	sub_82466E20(ctx, base);
	// 8261A594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A5A8 size=100
    let mut pc: u32 = 0x8261A5A8;
    'dispatch: loop {
        match pc {
            0x8261A5A8 => {
    //   block [0x8261A5A8..0x8261A60C)
	// 8261A5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A5B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A5BC: 38AAD37C  addi r5, r10, -0x2c84
	ctx.r[5].s64 = ctx.r[10].s64 + -11396;
	// 8261A5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A5C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A5C8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 8261A5CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A5DC: 386AD46C  addi r3, r10, -0x2b94
	ctx.r[3].s64 = ctx.r[10].s64 + -11156;
	// 8261A5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A5E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A5E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A5F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A5F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A5F8: 4BE4C829  bl 0x82466e20
	ctx.lr = 0x8261A5FC;
	sub_82466E20(ctx, base);
	// 8261A5FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A610 size=100
    let mut pc: u32 = 0x8261A610;
    'dispatch: loop {
        match pc {
            0x8261A610 => {
    //   block [0x8261A610..0x8261A674)
	// 8261A610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A61C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A624: 38AAD43C  addi r5, r10, -0x2bc4
	ctx.r[5].s64 = ctx.r[10].s64 + -11204;
	// 8261A628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A62C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A630: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 8261A634: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A63C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A644: 386AD49C  addi r3, r10, -0x2b64
	ctx.r[3].s64 = ctx.r[10].s64 + -11108;
	// 8261A648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A64C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A650: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A658: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A660: 4BE4C7C1  bl 0x82466e20
	ctx.lr = 0x8261A664;
	sub_82466E20(ctx, base);
	// 8261A664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A66C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A678 size=100
    let mut pc: u32 = 0x8261A678;
    'dispatch: loop {
        match pc {
            0x8261A678 => {
    //   block [0x8261A678..0x8261A6DC)
	// 8261A678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A684: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A68C: 38AAD37C  addi r5, r10, -0x2c84
	ctx.r[5].s64 = ctx.r[10].s64 + -11396;
	// 8261A690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A698: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8261A69C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A6AC: 386AD4CC  addi r3, r10, -0x2b34
	ctx.r[3].s64 = ctx.r[10].s64 + -11060;
	// 8261A6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A6B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A6B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261A6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A6C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261A6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A6C8: 4BE4C759  bl 0x82466e20
	ctx.lr = 0x8261A6CC;
	sub_82466E20(ctx, base);
	// 8261A6CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A6D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A6D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A6E0 size=112
    let mut pc: u32 = 0x8261A6E0;
    'dispatch: loop {
        match pc {
            0x8261A6E0 => {
    //   block [0x8261A6E0..0x8261A750)
	// 8261A6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A6EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A6F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A6F4: 38AAD55C  addi r5, r10, -0x2aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -10916;
	// 8261A6F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A6FC: 390BB168  addi r8, r11, -0x4e98
	ctx.r[8].s64 = ctx.r[11].s64 + -20120;
	// 8261A700: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261A704: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 8261A708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A70C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A718: 386AD4FC  addi r3, r10, -0x2b04
	ctx.r[3].s64 = ctx.r[10].s64 + -11012;
	// 8261A71C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A73C: 4BE4C6E5  bl 0x82466e20
	ctx.lr = 0x8261A740;
	sub_82466E20(ctx, base);
	// 8261A740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A750 size=112
    let mut pc: u32 = 0x8261A750;
    'dispatch: loop {
        match pc {
            0x8261A750 => {
    //   block [0x8261A750..0x8261A7C0)
	// 8261A750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A75C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A760: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A764: 38AAD58C  addi r5, r10, -0x2a74
	ctx.r[5].s64 = ctx.r[10].s64 + -10868;
	// 8261A768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A76C: 390BB198  addi r8, r11, -0x4e68
	ctx.r[8].s64 = ctx.r[11].s64 + -20072;
	// 8261A770: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261A774: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 8261A778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A77C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A788: 386AD52C  addi r3, r10, -0x2ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -10964;
	// 8261A78C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A7AC: 4BE4C675  bl 0x82466e20
	ctx.lr = 0x8261A7B0;
	sub_82466E20(ctx, base);
	// 8261A7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A7C0 size=112
    let mut pc: u32 = 0x8261A7C0;
    'dispatch: loop {
        match pc {
            0x8261A7C0 => {
    //   block [0x8261A7C0..0x8261A830)
	// 8261A7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A7CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A7D0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A7D4: 38AAD64C  addi r5, r10, -0x29b4
	ctx.r[5].s64 = ctx.r[10].s64 + -10676;
	// 8261A7D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A7DC: 390BB1B0  addi r8, r11, -0x4e50
	ctx.r[8].s64 = ctx.r[11].s64 + -20048;
	// 8261A7E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261A7E4: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 8261A7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A7EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A7F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A7F8: 386AD55C  addi r3, r10, -0x2aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -10916;
	// 8261A7FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A81C: 4BE4C605  bl 0x82466e20
	ctx.lr = 0x8261A820;
	sub_82466E20(ctx, base);
	// 8261A820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A830 size=112
    let mut pc: u32 = 0x8261A830;
    'dispatch: loop {
        match pc {
            0x8261A830 => {
    //   block [0x8261A830..0x8261A8A0)
	// 8261A830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A83C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A840: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A844: 38AAD55C  addi r5, r10, -0x2aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -10916;
	// 8261A848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A84C: 390BB1E0  addi r8, r11, -0x4e20
	ctx.r[8].s64 = ctx.r[11].s64 + -20000;
	// 8261A850: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261A854: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 8261A858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A85C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A868: 386AD58C  addi r3, r10, -0x2a74
	ctx.r[3].s64 = ctx.r[10].s64 + -10868;
	// 8261A86C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A88C: 4BE4C595  bl 0x82466e20
	ctx.lr = 0x8261A890;
	sub_82466E20(ctx, base);
	// 8261A890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A8A0 size=112
    let mut pc: u32 = 0x8261A8A0;
    'dispatch: loop {
        match pc {
            0x8261A8A0 => {
    //   block [0x8261A8A0..0x8261A910)
	// 8261A8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A8AC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A8B0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A8B4: 38AAD58C  addi r5, r10, -0x2a74
	ctx.r[5].s64 = ctx.r[10].s64 + -10868;
	// 8261A8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A8BC: 390BB1F8  addi r8, r11, -0x4e08
	ctx.r[8].s64 = ctx.r[11].s64 + -19976;
	// 8261A8C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261A8C4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 8261A8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261A8CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A8D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A8D8: 386AD5BC  addi r3, r10, -0x2a44
	ctx.r[3].s64 = ctx.r[10].s64 + -10820;
	// 8261A8DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261A8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261A8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261A8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A8FC: 4BE4C525  bl 0x82466e20
	ctx.lr = 0x8261A900;
	sub_82466E20(ctx, base);
	// 8261A900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A910 size=116
    let mut pc: u32 = 0x8261A910;
    'dispatch: loop {
        match pc {
            0x8261A910 => {
    //   block [0x8261A910..0x8261A984)
	// 8261A910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A91C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261A920: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8261A924: 390AB210  addi r8, r10, -0x4df0
	ctx.r[8].s64 = ctx.r[10].s64 + -19952;
	// 8261A928: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A92C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261A930: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261A934: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A938: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261A93C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261A940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261A944: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 8261A948: 396B030C  addi r11, r11, 0x30c
	ctx.r[11].s64 = ctx.r[11].s64 + 780;
	// 8261A94C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A950: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A954: 386AD5EC  addi r3, r10, -0x2a14
	ctx.r[3].s64 = ctx.r[10].s64 + -10772;
	// 8261A958: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261A95C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261A960: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261A964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261A968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261A96C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261A970: 4BE4C4B1  bl 0x82466e20
	ctx.lr = 0x8261A974;
	sub_82466E20(ctx, base);
	// 8261A974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261A978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261A97C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261A980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261A988 size=48
    let mut pc: u32 = 0x8261A988;
    'dispatch: loop {
        match pc {
            0x8261A988 => {
    //   block [0x8261A988..0x8261A9B8)
	// 8261A988: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A98C: 814BB2C4  lwz r10, -0x4d3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19772 as u32) ) } as u64;
	// 8261A990: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A994: 396BE5A8  addi r11, r11, -0x1a58
	ctx.r[11].s64 = ctx.r[11].s64 + -6744;
	// 8261A998: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8261A99C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261A9A0: 814AB2C0  lwz r10, -0x4d40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19776 as u32) ) } as u64;
	// 8261A9A4: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 8261A9A8: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261A9AC: 814AB2BC  lwz r10, -0x4d44(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-19780 as u32) ) } as u64;
	// 8261A9B0: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 8261A9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261A9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261A9B8 size=116
    let mut pc: u32 = 0x8261A9B8;
    'dispatch: loop {
        match pc {
            0x8261A9B8 => {
    //   block [0x8261A9B8..0x8261AA2C)
	// 8261A9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261A9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261A9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261A9C4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261A9C8: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A9CC: 392B03E0  addi r9, r11, 0x3e0
	ctx.r[9].s64 = ctx.r[11].s64 + 992;
	// 8261A9D0: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261A9D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261A9D8: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8261A9DC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8261A9E0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261A9E4: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 8261A9E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261A9EC: 396BE5A8  addi r11, r11, -0x1a58
	ctx.r[11].s64 = ctx.r[11].s64 + -6744;
	// 8261A9F0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8261A9F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261A9F8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8261A9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AA00: 386AD61C  addi r3, r10, -0x29e4
	ctx.r[3].s64 = ctx.r[10].s64 + -10724;
	// 8261AA04: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8261AA08: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8261AA0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AA10: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8261AA14: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261AA18: 4BE4C409  bl 0x82466e20
	ctx.lr = 0x8261AA1C;
	sub_82466E20(ctx, base);
	// 8261AA1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AA28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AA30 size=116
    let mut pc: u32 = 0x8261AA30;
    'dispatch: loop {
        match pc {
            0x8261AA30 => {
    //   block [0x8261AA30..0x8261AAA4)
	// 8261AA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AA3C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AA40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261AA44: 390BB2D0  addi r8, r11, -0x4d30
	ctx.r[8].s64 = ctx.r[11].s64 + -19760;
	// 8261AA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AA4C: 392A0580  addi r9, r10, 0x580
	ctx.r[9].s64 = ctx.r[10].s64 + 1408;
	// 8261AA50: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AA54: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8261AA58: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261AA5C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AA64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AA74: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261AA78: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8261AA7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261AA80: 386BD64C  addi r3, r11, -0x29b4
	ctx.r[3].s64 = ctx.r[11].s64 + -10676;
	// 8261AA84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261AA88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AA90: 4BE4C391  bl 0x82466e20
	ctx.lr = 0x8261AA94;
	sub_82466E20(ctx, base);
	// 8261AA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AAA8 size=112
    let mut pc: u32 = 0x8261AAA8;
    'dispatch: loop {
        match pc {
            0x8261AAA8 => {
    //   block [0x8261AAA8..0x8261AB18)
	// 8261AAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AAB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AAB8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AABC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261AAC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AAC4: 390BB360  addi r8, r11, -0x4ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -19616;
	// 8261AAC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261AACC: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 8261AAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AAD4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AAE0: 386AD67C  addi r3, r10, -0x2984
	ctx.r[3].s64 = ctx.r[10].s64 + -10628;
	// 8261AAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261AAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AB04: 4BE4C31D  bl 0x82466e20
	ctx.lr = 0x8261AB08;
	sub_82466E20(ctx, base);
	// 8261AB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AB18 size=108
    let mut pc: u32 = 0x8261AB18;
    'dispatch: loop {
        match pc {
            0x8261AB18 => {
    //   block [0x8261AB18..0x8261AB84)
	// 8261AB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AB24: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AB28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261AB2C: 38EBB378  addi r7, r11, -0x4c88
	ctx.r[7].s64 = ctx.r[11].s64 + -19592;
	// 8261AB30: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8261AB34: 388A0E48  addi r4, r10, 0xe48
	ctx.r[4].s64 = ctx.r[10].s64 + 3656;
	// 8261AB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AB3C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AB40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261AB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AB48: 386AD6AC  addi r3, r10, -0x2954
	ctx.r[3].s64 = ctx.r[10].s64 + -10580;
	// 8261AB4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261AB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AB54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AB5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AB6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261AB70: 4BE4C2B1  bl 0x82466e20
	ctx.lr = 0x8261AB74;
	sub_82466E20(ctx, base);
	// 8261AB74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AB78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AB7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AB80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AB88 size=112
    let mut pc: u32 = 0x8261AB88;
    'dispatch: loop {
        match pc {
            0x8261AB88 => {
    //   block [0x8261AB88..0x8261ABF8)
	// 8261AB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AB94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AB98: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AB9C: 38AAB54C  addi r5, r10, -0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + -19124;
	// 8261ABA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261ABA4: 390BB3F0  addi r8, r11, -0x4c10
	ctx.r[8].s64 = ctx.r[11].s64 + -19472;
	// 8261ABA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261ABAC: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 8261ABB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261ABB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261ABB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261ABBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261ABC0: 386AD6DC  addi r3, r10, -0x2924
	ctx.r[3].s64 = ctx.r[10].s64 + -10532;
	// 8261ABC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261ABC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261ABCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261ABD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261ABD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261ABD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261ABDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261ABE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261ABE4: 4BE4C23D  bl 0x82466e20
	ctx.lr = 0x8261ABE8;
	sub_82466E20(ctx, base);
	// 8261ABE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261ABEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261ABF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261ABF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261ABF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261ABF8 size=108
    let mut pc: u32 = 0x8261ABF8;
    'dispatch: loop {
        match pc {
            0x8261ABF8 => {
    //   block [0x8261ABF8..0x8261AC64)
	// 8261ABF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261ABFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AC04: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AC08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AC0C: 38EBB408  addi r7, r11, -0x4bf8
	ctx.r[7].s64 = ctx.r[11].s64 + -19448;
	// 8261AC10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261AC14: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 8261AC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AC1C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AC20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261AC24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AC28: 386AD70C  addi r3, r10, -0x28f4
	ctx.r[3].s64 = ctx.r[10].s64 + -10484;
	// 8261AC2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261AC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AC34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AC4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261AC50: 4BE4C1D1  bl 0x82466e20
	ctx.lr = 0x8261AC54;
	sub_82466E20(ctx, base);
	// 8261AC54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AC68 size=112
    let mut pc: u32 = 0x8261AC68;
    'dispatch: loop {
        match pc {
            0x8261AC68 => {
    //   block [0x8261AC68..0x8261ACD8)
	// 8261AC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AC74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AC78: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AC7C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261AC80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AC84: 390BB420  addi r8, r11, -0x4be0
	ctx.r[8].s64 = ctx.r[11].s64 + -19424;
	// 8261AC88: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261AC8C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 8261AC90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AC94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AC98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261ACA0: 386AD73C  addi r3, r10, -0x28c4
	ctx.r[3].s64 = ctx.r[10].s64 + -10436;
	// 8261ACA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261ACA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261ACAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261ACB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261ACB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261ACB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261ACBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261ACC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261ACC4: 4BE4C15D  bl 0x82466e20
	ctx.lr = 0x8261ACC8;
	sub_82466E20(ctx, base);
	// 8261ACC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261ACCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261ACD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261ACD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261ACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261ACD8 size=108
    let mut pc: u32 = 0x8261ACD8;
    'dispatch: loop {
        match pc {
            0x8261ACD8 => {
    //   block [0x8261ACD8..0x8261AD44)
	// 8261ACD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261ACDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261ACE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261ACE4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261ACE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261ACEC: 38EBB468  addi r7, r11, -0x4b98
	ctx.r[7].s64 = ctx.r[11].s64 + -19352;
	// 8261ACF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261ACF4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 8261ACF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261ACFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AD00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261AD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AD08: 386AD76C  addi r3, r10, -0x2894
	ctx.r[3].s64 = ctx.r[10].s64 + -10388;
	// 8261AD0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261AD10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AD14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AD24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AD2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261AD30: 4BE4C0F1  bl 0x82466e20
	ctx.lr = 0x8261AD34;
	sub_82466E20(ctx, base);
	// 8261AD34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AD38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AD3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AD40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AD48 size=108
    let mut pc: u32 = 0x8261AD48;
    'dispatch: loop {
        match pc {
            0x8261AD48 => {
    //   block [0x8261AD48..0x8261ADB4)
	// 8261AD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AD54: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AD58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AD5C: 38EBB498  addi r7, r11, -0x4b68
	ctx.r[7].s64 = ctx.r[11].s64 + -19304;
	// 8261AD60: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261AD64: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 8261AD68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AD6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AD70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261AD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AD78: 386AD79C  addi r3, r10, -0x2864
	ctx.r[3].s64 = ctx.r[10].s64 + -10340;
	// 8261AD7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261AD80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AD84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AD88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AD90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AD94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AD9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261ADA0: 4BE4C081  bl 0x82466e20
	ctx.lr = 0x8261ADA4;
	sub_82466E20(ctx, base);
	// 8261ADA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261ADA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261ADAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261ADB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261ADB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261ADB8 size=112
    let mut pc: u32 = 0x8261ADB8;
    'dispatch: loop {
        match pc {
            0x8261ADB8 => {
    //   block [0x8261ADB8..0x8261AE28)
	// 8261ADB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261ADBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261ADC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261ADC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261ADC8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261ADCC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261ADD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261ADD4: 390BB4B0  addi r8, r11, -0x4b50
	ctx.r[8].s64 = ctx.r[11].s64 + -19280;
	// 8261ADD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261ADDC: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 8261ADE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261ADE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261ADE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261ADEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261ADF0: 386AD7CC  addi r3, r10, -0x2834
	ctx.r[3].s64 = ctx.r[10].s64 + -10292;
	// 8261ADF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261ADF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261ADFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AE14: 4BE4C00D  bl 0x82466e20
	ctx.lr = 0x8261AE18;
	sub_82466E20(ctx, base);
	// 8261AE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AE28 size=112
    let mut pc: u32 = 0x8261AE28;
    'dispatch: loop {
        match pc {
            0x8261AE28 => {
    //   block [0x8261AE28..0x8261AE98)
	// 8261AE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AE34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AE38: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AE3C: 38AAC7DC  addi r5, r10, -0x3824
	ctx.r[5].s64 = ctx.r[10].s64 + -14372;
	// 8261AE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AE44: 390BB4E0  addi r8, r11, -0x4b20
	ctx.r[8].s64 = ctx.r[11].s64 + -19232;
	// 8261AE48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261AE4C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 8261AE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AE54: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AE60: 386AD7FC  addi r3, r10, -0x2804
	ctx.r[3].s64 = ctx.r[10].s64 + -10244;
	// 8261AE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261AE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AE84: 4BE4BF9D  bl 0x82466e20
	ctx.lr = 0x8261AE88;
	sub_82466E20(ctx, base);
	// 8261AE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AE98 size=112
    let mut pc: u32 = 0x8261AE98;
    'dispatch: loop {
        match pc {
            0x8261AE98 => {
    //   block [0x8261AE98..0x8261AF08)
	// 8261AE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AEA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AEA8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AEAC: 38AAC7DC  addi r5, r10, -0x3824
	ctx.r[5].s64 = ctx.r[10].s64 + -14372;
	// 8261AEB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AEB4: 390BB528  addi r8, r11, -0x4ad8
	ctx.r[8].s64 = ctx.r[11].s64 + -19160;
	// 8261AEB8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261AEBC: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 8261AEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AEC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AEC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AED0: 386AD82C  addi r3, r10, -0x27d4
	ctx.r[3].s64 = ctx.r[10].s64 + -10196;
	// 8261AED4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261AED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AEF4: 4BE4BF2D  bl 0x82466e20
	ctx.lr = 0x8261AEF8;
	sub_82466E20(ctx, base);
	// 8261AEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AF08 size=112
    let mut pc: u32 = 0x8261AF08;
    'dispatch: loop {
        match pc {
            0x8261AF08 => {
    //   block [0x8261AF08..0x8261AF78)
	// 8261AF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AF14: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AF18: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AF1C: 38AAC80C  addi r5, r10, -0x37f4
	ctx.r[5].s64 = ctx.r[10].s64 + -14324;
	// 8261AF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AF24: 390BB588  addi r8, r11, -0x4a78
	ctx.r[8].s64 = ctx.r[11].s64 + -19064;
	// 8261AF28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261AF2C: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 8261AF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AF34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AF38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AF40: 386AD85C  addi r3, r10, -0x27a4
	ctx.r[3].s64 = ctx.r[10].s64 + -10148;
	// 8261AF44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261AF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AF64: 4BE4BEBD  bl 0x82466e20
	ctx.lr = 0x8261AF68;
	sub_82466E20(ctx, base);
	// 8261AF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AF78 size=112
    let mut pc: u32 = 0x8261AF78;
    'dispatch: loop {
        match pc {
            0x8261AF78 => {
    //   block [0x8261AF78..0x8261AFE8)
	// 8261AF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AF84: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AF88: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AF8C: 38AAC80C  addi r5, r10, -0x37f4
	ctx.r[5].s64 = ctx.r[10].s64 + -14324;
	// 8261AF90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261AF94: 390BB5E8  addi r8, r11, -0x4a18
	ctx.r[8].s64 = ctx.r[11].s64 + -18968;
	// 8261AF98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261AF9C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 8261AFA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261AFA4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AFA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261AFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261AFB0: 386AD88C  addi r3, r10, -0x2774
	ctx.r[3].s64 = ctx.r[10].s64 + -10100;
	// 8261AFB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261AFB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261AFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261AFC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261AFC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261AFC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261AFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261AFD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261AFD4: 4BE4BE4D  bl 0x82466e20
	ctx.lr = 0x8261AFD8;
	sub_82466E20(ctx, base);
	// 8261AFD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261AFDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261AFE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261AFE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261AFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261AFE8 size=112
    let mut pc: u32 = 0x8261AFE8;
    'dispatch: loop {
        match pc {
            0x8261AFE8 => {
    //   block [0x8261AFE8..0x8261B058)
	// 8261AFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261AFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261AFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261AFF4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261AFF8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261AFFC: 38AAC7DC  addi r5, r10, -0x3824
	ctx.r[5].s64 = ctx.r[10].s64 + -14372;
	// 8261B000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B004: 390BB648  addi r8, r11, -0x49b8
	ctx.r[8].s64 = ctx.r[11].s64 + -18872;
	// 8261B008: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8261B00C: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 8261B010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B014: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B020: 386AD8BC  addi r3, r10, -0x2744
	ctx.r[3].s64 = ctx.r[10].s64 + -10052;
	// 8261B024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B044: 4BE4BDDD  bl 0x82466e20
	ctx.lr = 0x8261B048;
	sub_82466E20(ctx, base);
	// 8261B048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B04C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B058 size=112
    let mut pc: u32 = 0x8261B058;
    'dispatch: loop {
        match pc {
            0x8261B058 => {
    //   block [0x8261B058..0x8261B0C8)
	// 8261B058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B064: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261B068: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8261B06C: 38EAB708  addi r7, r10, -0x48f8
	ctx.r[7].s64 = ctx.r[10].s64 + -18680;
	// 8261B070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B074: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261B078: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8261B07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B080: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B084: 396B0598  addi r11, r11, 0x598
	ctx.r[11].s64 = ctx.r[11].s64 + 1432;
	// 8261B088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B08C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B090: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B094: 386AD8EC  addi r3, r10, -0x2714
	ctx.r[3].s64 = ctx.r[10].s64 + -10004;
	// 8261B098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B09C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261B0A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B0A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261B0A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B0AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B0B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B0B4: 4BE4BD6D  bl 0x82466e20
	ctx.lr = 0x8261B0B8;
	sub_82466E20(ctx, base);
	// 8261B0B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B0C8 size=112
    let mut pc: u32 = 0x8261B0C8;
    'dispatch: loop {
        match pc {
            0x8261B0C8 => {
    //   block [0x8261B0C8..0x8261B138)
	// 8261B0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B0D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B0D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B0DC: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 8261B0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B0E4: 390BB8D0  addi r8, r11, -0x4730
	ctx.r[8].s64 = ctx.r[11].s64 + -18224;
	// 8261B0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B0EC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 8261B0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B0F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B100: 386AD91C  addi r3, r10, -0x26e4
	ctx.r[3].s64 = ctx.r[10].s64 + -9956;
	// 8261B104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B114: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261B118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B124: 4BE4BCFD  bl 0x82466e20
	ctx.lr = 0x8261B128;
	sub_82466E20(ctx, base);
	// 8261B128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B138 size=112
    let mut pc: u32 = 0x8261B138;
    'dispatch: loop {
        match pc {
            0x8261B138 => {
    //   block [0x8261B138..0x8261B1A8)
	// 8261B138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B144: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B148: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B14C: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 8261B150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B154: 390BB8E8  addi r8, r11, -0x4718
	ctx.r[8].s64 = ctx.r[11].s64 + -18200;
	// 8261B158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B15C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8261B160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B164: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B170: 386AD94C  addi r3, r10, -0x26b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9908;
	// 8261B174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B184: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261B188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B194: 4BE4BC8D  bl 0x82466e20
	ctx.lr = 0x8261B198;
	sub_82466E20(ctx, base);
	// 8261B198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B1A8 size=112
    let mut pc: u32 = 0x8261B1A8;
    'dispatch: loop {
        match pc {
            0x8261B1A8 => {
    //   block [0x8261B1A8..0x8261B218)
	// 8261B1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B1B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B1B8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B1BC: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 8261B1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B1C4: 390BB900  addi r8, r11, -0x4700
	ctx.r[8].s64 = ctx.r[11].s64 + -18176;
	// 8261B1C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261B1CC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8261B1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B1D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B1E0: 386AD97C  addi r3, r10, -0x2684
	ctx.r[3].s64 = ctx.r[10].s64 + -9860;
	// 8261B1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B204: 4BE4BC1D  bl 0x82466e20
	ctx.lr = 0x8261B208;
	sub_82466E20(ctx, base);
	// 8261B208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B218 size=108
    let mut pc: u32 = 0x8261B218;
    'dispatch: loop {
        match pc {
            0x8261B218 => {
    //   block [0x8261B218..0x8261B284)
	// 8261B218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B224: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B22C: 38EBB930  addi r7, r11, -0x46d0
	ctx.r[7].s64 = ctx.r[11].s64 + -18128;
	// 8261B230: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B234: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8261B238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B23C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B248: 386AD9AC  addi r3, r10, -0x2654
	ctx.r[3].s64 = ctx.r[10].s64 + -9812;
	// 8261B24C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B270: 4BE4BBB1  bl 0x82466e20
	ctx.lr = 0x8261B274;
	sub_82466E20(ctx, base);
	// 8261B274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B288 size=112
    let mut pc: u32 = 0x8261B288;
    'dispatch: loop {
        match pc {
            0x8261B288 => {
    //   block [0x8261B288..0x8261B2F8)
	// 8261B288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B294: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B298: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B29C: 38AAB60C  addi r5, r10, -0x49f4
	ctx.r[5].s64 = ctx.r[10].s64 + -18932;
	// 8261B2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B2A4: 390BB960  addi r8, r11, -0x46a0
	ctx.r[8].s64 = ctx.r[11].s64 + -18080;
	// 8261B2A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B2AC: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8261B2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B2B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B2C0: 386AD9DC  addi r3, r10, -0x2624
	ctx.r[3].s64 = ctx.r[10].s64 + -9764;
	// 8261B2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B2D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261B2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B2E4: 4BE4BB3D  bl 0x82466e20
	ctx.lr = 0x8261B2E8;
	sub_82466E20(ctx, base);
	// 8261B2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B2F8 size=108
    let mut pc: u32 = 0x8261B2F8;
    'dispatch: loop {
        match pc {
            0x8261B2F8 => {
    //   block [0x8261B2F8..0x8261B364)
	// 8261B2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B304: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B30C: 38EBB978  addi r7, r11, -0x4688
	ctx.r[7].s64 = ctx.r[11].s64 + -18056;
	// 8261B310: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B314: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8261B318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B31C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B328: 386ADA0C  addi r3, r10, -0x25f4
	ctx.r[3].s64 = ctx.r[10].s64 + -9716;
	// 8261B32C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B350: 4BE4BAD1  bl 0x82466e20
	ctx.lr = 0x8261B354;
	sub_82466E20(ctx, base);
	// 8261B354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B368 size=108
    let mut pc: u32 = 0x8261B368;
    'dispatch: loop {
        match pc {
            0x8261B368 => {
    //   block [0x8261B368..0x8261B3D4)
	// 8261B368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B374: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B378: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B37C: 38EBB9A8  addi r7, r11, -0x4658
	ctx.r[7].s64 = ctx.r[11].s64 + -18008;
	// 8261B380: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261B384: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8261B388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B38C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B398: 386ADA3C  addi r3, r10, -0x25c4
	ctx.r[3].s64 = ctx.r[10].s64 + -9668;
	// 8261B39C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B3BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B3C0: 4BE4BA61  bl 0x82466e20
	ctx.lr = 0x8261B3C4;
	sub_82466E20(ctx, base);
	// 8261B3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B3D8 size=112
    let mut pc: u32 = 0x8261B3D8;
    'dispatch: loop {
        match pc {
            0x8261B3D8 => {
    //   block [0x8261B3D8..0x8261B448)
	// 8261B3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B3E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B3E8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B3EC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261B3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B3F4: 390BB9F0  addi r8, r11, -0x4610
	ctx.r[8].s64 = ctx.r[11].s64 + -17936;
	// 8261B3F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261B3FC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8261B400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B404: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B410: 386ADA6C  addi r3, r10, -0x2594
	ctx.r[3].s64 = ctx.r[10].s64 + -9620;
	// 8261B414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B434: 4BE4B9ED  bl 0x82466e20
	ctx.lr = 0x8261B438;
	sub_82466E20(ctx, base);
	// 8261B438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B448 size=112
    let mut pc: u32 = 0x8261B448;
    'dispatch: loop {
        match pc {
            0x8261B448 => {
    //   block [0x8261B448..0x8261B4B8)
	// 8261B448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B454: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B458: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B45C: 38AAD5EC  addi r5, r10, -0x2a14
	ctx.r[5].s64 = ctx.r[10].s64 + -10772;
	// 8261B460: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B464: 390BBA38  addi r8, r11, -0x45c8
	ctx.r[8].s64 = ctx.r[11].s64 + -17864;
	// 8261B468: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B46C: 388A0E58  addi r4, r10, 0xe58
	ctx.r[4].s64 = ctx.r[10].s64 + 3672;
	// 8261B470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B474: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B480: 386ADA9C  addi r3, r10, -0x2564
	ctx.r[3].s64 = ctx.r[10].s64 + -9572;
	// 8261B484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B4A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B4A4: 4BE4B97D  bl 0x82466e20
	ctx.lr = 0x8261B4A8;
	sub_82466E20(ctx, base);
	// 8261B4A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B4B8 size=108
    let mut pc: u32 = 0x8261B4B8;
    'dispatch: loop {
        match pc {
            0x8261B4B8 => {
    //   block [0x8261B4B8..0x8261B524)
	// 8261B4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B4C4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B4C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B4CC: 38EBBA50  addi r7, r11, -0x45b0
	ctx.r[7].s64 = ctx.r[11].s64 + -17840;
	// 8261B4D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261B4D4: 388A0E74  addi r4, r10, 0xe74
	ctx.r[4].s64 = ctx.r[10].s64 + 3700;
	// 8261B4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B4DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B4E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B4E8: 386ADACC  addi r3, r10, -0x2534
	ctx.r[3].s64 = ctx.r[10].s64 + -9524;
	// 8261B4EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B50C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B510: 4BE4B911  bl 0x82466e20
	ctx.lr = 0x8261B514;
	sub_82466E20(ctx, base);
	// 8261B514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B528 size=108
    let mut pc: u32 = 0x8261B528;
    'dispatch: loop {
        match pc {
            0x8261B528 => {
    //   block [0x8261B528..0x8261B594)
	// 8261B528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B534: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B53C: 38EBBA98  addi r7, r11, -0x4568
	ctx.r[7].s64 = ctx.r[11].s64 + -17768;
	// 8261B540: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B544: 388A0E9C  addi r4, r10, 0xe9c
	ctx.r[4].s64 = ctx.r[10].s64 + 3740;
	// 8261B548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B54C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B558: 386ADAFC  addi r3, r10, -0x2504
	ctx.r[3].s64 = ctx.r[10].s64 + -9476;
	// 8261B55C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B580: 4BE4B8A1  bl 0x82466e20
	ctx.lr = 0x8261B584;
	sub_82466E20(ctx, base);
	// 8261B584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B598 size=112
    let mut pc: u32 = 0x8261B598;
    'dispatch: loop {
        match pc {
            0x8261B598 => {
    //   block [0x8261B598..0x8261B608)
	// 8261B598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B5A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B5A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B5AC: 38AADAFC  addi r5, r10, -0x2504
	ctx.r[5].s64 = ctx.r[10].s64 + -9476;
	// 8261B5B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B5B4: 390BBAC8  addi r8, r11, -0x4538
	ctx.r[8].s64 = ctx.r[11].s64 + -17720;
	// 8261B5B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261B5BC: 388A0EB4  addi r4, r10, 0xeb4
	ctx.r[4].s64 = ctx.r[10].s64 + 3764;
	// 8261B5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B5C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B5D0: 386ADB2C  addi r3, r10, -0x24d4
	ctx.r[3].s64 = ctx.r[10].s64 + -9428;
	// 8261B5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B5F4: 4BE4B82D  bl 0x82466e20
	ctx.lr = 0x8261B5F8;
	sub_82466E20(ctx, base);
	// 8261B5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8261B608 size=24
    let mut pc: u32 = 0x8261B608;
    'dispatch: loop {
        match pc {
            0x8261B608 => {
    //   block [0x8261B608..0x8261B620)
	// 8261B608: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B60C: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261B610: 394AE968  addi r10, r10, -0x1698
	ctx.r[10].s64 = ctx.r[10].s64 + -5784;
	// 8261B614: 816BBAF8  lwz r11, -0x4508(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17672 as u32) ) } as u64;
	// 8261B618: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8261B61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B620 size=116
    let mut pc: u32 = 0x8261B620;
    'dispatch: loop {
        match pc {
            0x8261B620 => {
    //   block [0x8261B620..0x8261B694)
	// 8261B620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B62C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B634: 390BE968  addi r8, r11, -0x1698
	ctx.r[8].s64 = ctx.r[11].s64 + -5784;
	// 8261B638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B63C: 392A0630  addi r9, r10, 0x630
	ctx.r[9].s64 = ctx.r[10].s64 + 1584;
	// 8261B640: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B644: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8261B648: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261B64C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8261B658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B664: 3D608294  lis r11, -0x7d6c
	ctx.r[11].s64 = -2104229888;
	// 8261B668: 388A0EDC  addi r4, r10, 0xedc
	ctx.r[4].s64 = ctx.r[10].s64 + 3804;
	// 8261B66C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261B670: 386BDB5C  addi r3, r11, -0x24a4
	ctx.r[3].s64 = ctx.r[11].s64 + -9380;
	// 8261B674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8261B678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B680: 4BE4B7A1  bl 0x82466e20
	ctx.lr = 0x8261B684;
	sub_82466E20(ctx, base);
	// 8261B684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B698 size=112
    let mut pc: u32 = 0x8261B698;
    'dispatch: loop {
        match pc {
            0x8261B698 => {
    //   block [0x8261B698..0x8261B708)
	// 8261B698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B6A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B6A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B6AC: 38AAC80C  addi r5, r10, -0x37f4
	ctx.r[5].s64 = ctx.r[10].s64 + -14324;
	// 8261B6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B6B4: 390BBB00  addi r8, r11, -0x4500
	ctx.r[8].s64 = ctx.r[11].s64 + -17664;
	// 8261B6B8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8261B6BC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8261B6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B6C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B6D0: 386ADB8C  addi r3, r10, -0x2474
	ctx.r[3].s64 = ctx.r[10].s64 + -9332;
	// 8261B6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B6F4: 4BE4B72D  bl 0x82466e20
	ctx.lr = 0x8261B6F8;
	sub_82466E20(ctx, base);
	// 8261B6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B708 size=108
    let mut pc: u32 = 0x8261B708;
    'dispatch: loop {
        match pc {
            0x8261B708 => {
    //   block [0x8261B708..0x8261B774)
	// 8261B708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B714: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B71C: 38EBBB90  addi r7, r11, -0x4470
	ctx.r[7].s64 = ctx.r[11].s64 + -17520;
	// 8261B720: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261B724: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8261B728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B72C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B738: 386ADBBC  addi r3, r10, -0x2444
	ctx.r[3].s64 = ctx.r[10].s64 + -9284;
	// 8261B73C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B75C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B760: 4BE4B6C1  bl 0x82466e20
	ctx.lr = 0x8261B764;
	sub_82466E20(ctx, base);
	// 8261B764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B76C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B778 size=108
    let mut pc: u32 = 0x8261B778;
    'dispatch: loop {
        match pc {
            0x8261B778 => {
    //   block [0x8261B778..0x8261B7E4)
	// 8261B778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B784: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B78C: 38EBBBD8  addi r7, r11, -0x4428
	ctx.r[7].s64 = ctx.r[11].s64 + -17448;
	// 8261B790: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B794: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8261B798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B79C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B7A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B7A8: 386ADBEC  addi r3, r10, -0x2414
	ctx.r[3].s64 = ctx.r[10].s64 + -9236;
	// 8261B7AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B7B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B7B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B7C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B7C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B7CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B7D0: 4BE4B651  bl 0x82466e20
	ctx.lr = 0x8261B7D4;
	sub_82466E20(ctx, base);
	// 8261B7D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B7D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B7DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B7E8 size=108
    let mut pc: u32 = 0x8261B7E8;
    'dispatch: loop {
        match pc {
            0x8261B7E8 => {
    //   block [0x8261B7E8..0x8261B854)
	// 8261B7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B7F4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B7FC: 38EBBC08  addi r7, r11, -0x43f8
	ctx.r[7].s64 = ctx.r[11].s64 + -17400;
	// 8261B800: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B804: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8261B808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B80C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B818: 386ADC1C  addi r3, r10, -0x23e4
	ctx.r[3].s64 = ctx.r[10].s64 + -9188;
	// 8261B81C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261B840: 4BE4B5E1  bl 0x82466e20
	ctx.lr = 0x8261B844;
	sub_82466E20(ctx, base);
	// 8261B844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B858 size=112
    let mut pc: u32 = 0x8261B858;
    'dispatch: loop {
        match pc {
            0x8261B858 => {
    //   block [0x8261B858..0x8261B8C8)
	// 8261B858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B864: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B868: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B86C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261B870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B874: 390BBC38  addi r8, r11, -0x43c8
	ctx.r[8].s64 = ctx.r[11].s64 + -17352;
	// 8261B878: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261B87C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8261B880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B884: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B890: 386ADC4C  addi r3, r10, -0x23b4
	ctx.r[3].s64 = ctx.r[10].s64 + -9140;
	// 8261B894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B8A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B8AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B8B4: 4BE4B56D  bl 0x82466e20
	ctx.lr = 0x8261B8B8;
	sub_82466E20(ctx, base);
	// 8261B8B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B8C8 size=112
    let mut pc: u32 = 0x8261B8C8;
    'dispatch: loop {
        match pc {
            0x8261B8C8 => {
    //   block [0x8261B8C8..0x8261B938)
	// 8261B8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B8D4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B8D8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B8DC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261B8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B8E4: 390BBC68  addi r8, r11, -0x4398
	ctx.r[8].s64 = ctx.r[11].s64 + -17304;
	// 8261B8E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B8EC: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8261B8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B8F4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B8F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B900: 386ADC7C  addi r3, r10, -0x2384
	ctx.r[3].s64 = ctx.r[10].s64 + -9092;
	// 8261B904: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B924: 4BE4B4FD  bl 0x82466e20
	ctx.lr = 0x8261B928;
	sub_82466E20(ctx, base);
	// 8261B928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B938 size=112
    let mut pc: u32 = 0x8261B938;
    'dispatch: loop {
        match pc {
            0x8261B938 => {
    //   block [0x8261B938..0x8261B9A8)
	// 8261B938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B944: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B948: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B94C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261B950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B954: 390BBC80  addi r8, r11, -0x4380
	ctx.r[8].s64 = ctx.r[11].s64 + -17280;
	// 8261B958: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261B95C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8261B960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B964: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261B96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B970: 386ADCAC  addi r3, r10, -0x2354
	ctx.r[3].s64 = ctx.r[10].s64 + -9044;
	// 8261B974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261B978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B994: 4BE4B48D  bl 0x82466e20
	ctx.lr = 0x8261B998;
	sub_82466E20(ctx, base);
	// 8261B998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261B99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261B9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261B9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261B9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261B9A8 size=108
    let mut pc: u32 = 0x8261B9A8;
    'dispatch: loop {
        match pc {
            0x8261B9A8 => {
    //   block [0x8261B9A8..0x8261BA14)
	// 8261B9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261B9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261B9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261B9B4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261B9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261B9BC: 38EBBC98  addi r7, r11, -0x4368
	ctx.r[7].s64 = ctx.r[11].s64 + -17256;
	// 8261B9C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261B9C4: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8261B9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261B9CC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261B9D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261B9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261B9D8: 386ADCDC  addi r3, r10, -0x2324
	ctx.r[3].s64 = ctx.r[10].s64 + -8996;
	// 8261B9DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261B9E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261B9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261B9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261B9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261B9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261B9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261B9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261B9FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261BA00: 4BE4B421  bl 0x82466e20
	ctx.lr = 0x8261BA04;
	sub_82466E20(ctx, base);
	// 8261BA04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BA08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BA0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BA18 size=112
    let mut pc: u32 = 0x8261BA18;
    'dispatch: loop {
        match pc {
            0x8261BA18 => {
    //   block [0x8261BA18..0x8261BA88)
	// 8261BA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BA24: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BA28: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BA2C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BA34: 390BBCC8  addi r8, r11, -0x4338
	ctx.r[8].s64 = ctx.r[11].s64 + -17208;
	// 8261BA38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261BA3C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8261BA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BA44: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BA48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BA4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BA50: 386ADD0C  addi r3, r10, -0x22f4
	ctx.r[3].s64 = ctx.r[10].s64 + -8948;
	// 8261BA54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BA74: 4BE4B3AD  bl 0x82466e20
	ctx.lr = 0x8261BA78;
	sub_82466E20(ctx, base);
	// 8261BA78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BA88 size=108
    let mut pc: u32 = 0x8261BA88;
    'dispatch: loop {
        match pc {
            0x8261BA88 => {
    //   block [0x8261BA88..0x8261BAF4)
	// 8261BA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BA94: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BA98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BA9C: 38EBBCE0  addi r7, r11, -0x4320
	ctx.r[7].s64 = ctx.r[11].s64 + -17184;
	// 8261BAA0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8261BAA4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8261BAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BAAC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BAB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261BAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BAB8: 386ADD3C  addi r3, r10, -0x22c4
	ctx.r[3].s64 = ctx.r[10].s64 + -8900;
	// 8261BABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261BAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261BAE0: 4BE4B341  bl 0x82466e20
	ctx.lr = 0x8261BAE4;
	sub_82466E20(ctx, base);
	// 8261BAE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BAE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BAEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BAF8 size=116
    let mut pc: u32 = 0x8261BAF8;
    'dispatch: loop {
        match pc {
            0x8261BAF8 => {
    //   block [0x8261BAF8..0x8261BB6C)
	// 8261BAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BB04: 3D408276  lis r10, -0x7d8a
	ctx.r[10].s64 = -2106195968;
	// 8261BB08: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 8261BB0C: 390ABDD0  addi r8, r10, -0x4230
	ctx.r[8].s64 = ctx.r[10].s64 + -16944;
	// 8261BB10: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BB14: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8261BB18: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BB1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BB20: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8261BB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BB28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BB2C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8261BB30: 396B0648  addi r11, r11, 0x648
	ctx.r[11].s64 = ctx.r[11].s64 + 1608;
	// 8261BB34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BB38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BB3C: 386ADD6C  addi r3, r10, -0x2294
	ctx.r[3].s64 = ctx.r[10].s64 + -8852;
	// 8261BB40: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8261BB44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BB48: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8261BB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BB58: 4BE4B2C9  bl 0x82466e20
	ctx.lr = 0x8261BB5C;
	sub_82466E20(ctx, base);
	// 8261BB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BB70 size=108
    let mut pc: u32 = 0x8261BB70;
    'dispatch: loop {
        match pc {
            0x8261BB70 => {
    //   block [0x8261BB70..0x8261BBDC)
	// 8261BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BB7C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BB84: 38EBBF98  addi r7, r11, -0x4068
	ctx.r[7].s64 = ctx.r[11].s64 + -16488;
	// 8261BB88: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8261BB8C: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8261BB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BB94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BB98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261BB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BBA0: 386ADD9C  addi r3, r10, -0x2264
	ctx.r[3].s64 = ctx.r[10].s64 + -8804;
	// 8261BBA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261BBA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BBB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BBC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261BBC8: 4BE4B259  bl 0x82466e20
	ctx.lr = 0x8261BBCC;
	sub_82466E20(ctx, base);
	// 8261BBCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BBD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BBD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BBE0 size=112
    let mut pc: u32 = 0x8261BBE0;
    'dispatch: loop {
        match pc {
            0x8261BBE0 => {
    //   block [0x8261BBE0..0x8261BC50)
	// 8261BBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BBEC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BBF0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BBF4: 38AAC80C  addi r5, r10, -0x37f4
	ctx.r[5].s64 = ctx.r[10].s64 + -14324;
	// 8261BBF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BBFC: 390BC130  addi r8, r11, -0x3ed0
	ctx.r[8].s64 = ctx.r[11].s64 + -16080;
	// 8261BC00: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8261BC04: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8261BC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BC0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BC10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BC18: 386ADDCC  addi r3, r10, -0x2234
	ctx.r[3].s64 = ctx.r[10].s64 + -8756;
	// 8261BC1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BC24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BC28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BC2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BC30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BC38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BC3C: 4BE4B1E5  bl 0x82466e20
	ctx.lr = 0x8261BC40;
	sub_82466E20(ctx, base);
	// 8261BC40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BC50 size=100
    let mut pc: u32 = 0x8261BC50;
    'dispatch: loop {
        match pc {
            0x8261BC50 => {
    //   block [0x8261BC50..0x8261BCB4)
	// 8261BC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BC5C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BC64: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BC70: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8261BC74: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BC84: 386ADDFC  addi r3, r10, -0x2204
	ctx.r[3].s64 = ctx.r[10].s64 + -8708;
	// 8261BC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BC8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BC90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261BC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BC98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261BC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BCA0: 4BE4B181  bl 0x82466e20
	ctx.lr = 0x8261BCA4;
	sub_82466E20(ctx, base);
	// 8261BCA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BCA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BCAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BCB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BCB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BCB8 size=112
    let mut pc: u32 = 0x8261BCB8;
    'dispatch: loop {
        match pc {
            0x8261BCB8 => {
    //   block [0x8261BCB8..0x8261BD28)
	// 8261BCB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BCBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BCC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BCC4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BCC8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BCCC: 38AADDFC  addi r5, r10, -0x2204
	ctx.r[5].s64 = ctx.r[10].s64 + -8708;
	// 8261BCD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BCD4: 390BC388  addi r8, r11, -0x3c78
	ctx.r[8].s64 = ctx.r[11].s64 + -15480;
	// 8261BCD8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8261BCDC: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8261BCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BCE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BCE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BCF0: 386ADE2C  addi r3, r10, -0x21d4
	ctx.r[3].s64 = ctx.r[10].s64 + -8660;
	// 8261BCF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BCF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BCFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BD14: 4BE4B10D  bl 0x82466e20
	ctx.lr = 0x8261BD18;
	sub_82466E20(ctx, base);
	// 8261BD18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BD28 size=100
    let mut pc: u32 = 0x8261BD28;
    'dispatch: loop {
        match pc {
            0x8261BD28 => {
    //   block [0x8261BD28..0x8261BD8C)
	// 8261BD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BD34: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BD3C: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BD40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BD48: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8261BD4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BD5C: 386ADE5C  addi r3, r10, -0x21a4
	ctx.r[3].s64 = ctx.r[10].s64 + -8612;
	// 8261BD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BD64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BD68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261BD6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BD70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261BD74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BD78: 4BE4B0A9  bl 0x82466e20
	ctx.lr = 0x8261BD7C;
	sub_82466E20(ctx, base);
	// 8261BD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BD90 size=108
    let mut pc: u32 = 0x8261BD90;
    'dispatch: loop {
        match pc {
            0x8261BD90 => {
    //   block [0x8261BD90..0x8261BDFC)
	// 8261BD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BD9C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BDA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BDA4: 38EBC400  addi r7, r11, -0x3c00
	ctx.r[7].s64 = ctx.r[11].s64 + -15360;
	// 8261BDA8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261BDAC: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8261BDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BDB4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BDB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261BDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BDC0: 386ADE8C  addi r3, r10, -0x2174
	ctx.r[3].s64 = ctx.r[10].s64 + -8564;
	// 8261BDC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261BDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BDCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BDD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BDE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261BDE8: 4BE4B039  bl 0x82466e20
	ctx.lr = 0x8261BDEC;
	sub_82466E20(ctx, base);
	// 8261BDEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BDF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BE00 size=112
    let mut pc: u32 = 0x8261BE00;
    'dispatch: loop {
        match pc {
            0x8261BE00 => {
    //   block [0x8261BE00..0x8261BE70)
	// 8261BE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BE08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BE0C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BE10: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BE14: 38AADE5C  addi r5, r10, -0x21a4
	ctx.r[5].s64 = ctx.r[10].s64 + -8612;
	// 8261BE18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BE1C: 390BC448  addi r8, r11, -0x3bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -15288;
	// 8261BE20: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8261BE24: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8261BE28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BE2C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BE30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BE34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BE38: 386ADEBC  addi r3, r10, -0x2144
	ctx.r[3].s64 = ctx.r[10].s64 + -8516;
	// 8261BE3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BE40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BE54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BE5C: 4BE4AFC5  bl 0x82466e20
	ctx.lr = 0x8261BE60;
	sub_82466E20(ctx, base);
	// 8261BE60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BE64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BE68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BE6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BE70 size=100
    let mut pc: u32 = 0x8261BE70;
    'dispatch: loop {
        match pc {
            0x8261BE70 => {
    //   block [0x8261BE70..0x8261BED4)
	// 8261BE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BE7C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BE84: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BE88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BE90: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8261BE94: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BE98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BE9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BEA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BEA4: 386ADEEC  addi r3, r10, -0x2114
	ctx.r[3].s64 = ctx.r[10].s64 + -8468;
	// 8261BEA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BEAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BEB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261BEB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BEB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261BEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BEC0: 4BE4AF61  bl 0x82466e20
	ctx.lr = 0x8261BEC4;
	sub_82466E20(ctx, base);
	// 8261BEC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BED8 size=100
    let mut pc: u32 = 0x8261BED8;
    'dispatch: loop {
        match pc {
            0x8261BED8 => {
    //   block [0x8261BED8..0x8261BF3C)
	// 8261BED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BEE4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BEEC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261BEF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BEF8: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8261BEFC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BF04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BF0C: 386ADF1C  addi r3, r10, -0x20e4
	ctx.r[3].s64 = ctx.r[10].s64 + -8420;
	// 8261BF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BF14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BF18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261BF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BF20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261BF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BF28: 4BE4AEF9  bl 0x82466e20
	ctx.lr = 0x8261BF2C;
	sub_82466E20(ctx, base);
	// 8261BF2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BF30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BF34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BF40 size=112
    let mut pc: u32 = 0x8261BF40;
    'dispatch: loop {
        match pc {
            0x8261BF40 => {
    //   block [0x8261BF40..0x8261BFB0)
	// 8261BF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BF4C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BF50: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BF54: 38AADEEC  addi r5, r10, -0x2114
	ctx.r[5].s64 = ctx.r[10].s64 + -8468;
	// 8261BF58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BF5C: 390BC478  addi r8, r11, -0x3b88
	ctx.r[8].s64 = ctx.r[11].s64 + -15240;
	// 8261BF60: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261BF64: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8261BF68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BF6C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BF70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BF74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BF78: 386ADF4C  addi r3, r10, -0x20b4
	ctx.r[3].s64 = ctx.r[10].s64 + -8372;
	// 8261BF7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BF80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BF88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BF8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261BF90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261BF94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261BF98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261BF9C: 4BE4AE85  bl 0x82466e20
	ctx.lr = 0x8261BFA0;
	sub_82466E20(ctx, base);
	// 8261BFA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261BFA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261BFA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261BFAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261BFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261BFB0 size=112
    let mut pc: u32 = 0x8261BFB0;
    'dispatch: loop {
        match pc {
            0x8261BFB0 => {
    //   block [0x8261BFB0..0x8261C020)
	// 8261BFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261BFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261BFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261BFBC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BFC0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261BFC4: 38AADF1C  addi r5, r10, -0x20e4
	ctx.r[5].s64 = ctx.r[10].s64 + -8420;
	// 8261BFC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261BFCC: 390BC4D8  addi r8, r11, -0x3b28
	ctx.r[8].s64 = ctx.r[11].s64 + -15144;
	// 8261BFD0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8261BFD4: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8261BFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261BFDC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261BFE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261BFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261BFE8: 386ADF7C  addi r3, r10, -0x2084
	ctx.r[3].s64 = ctx.r[10].s64 + -8324;
	// 8261BFEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261BFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261BFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261BFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261BFFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C00C: 4BE4AE15  bl 0x82466e20
	ctx.lr = 0x8261C010;
	sub_82466E20(ctx, base);
	// 8261C010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C020 size=100
    let mut pc: u32 = 0x8261C020;
    'dispatch: loop {
        match pc {
            0x8261C020 => {
    //   block [0x8261C020..0x8261C084)
	// 8261C020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C02C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C034: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C040: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8261C044: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C054: 386ADFAC  addi r3, r10, -0x2054
	ctx.r[3].s64 = ctx.r[10].s64 + -8276;
	// 8261C058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C05C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C060: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261C064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C068: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261C06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C070: 4BE4ADB1  bl 0x82466e20
	ctx.lr = 0x8261C074;
	sub_82466E20(ctx, base);
	// 8261C074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C088 size=112
    let mut pc: u32 = 0x8261C088;
    'dispatch: loop {
        match pc {
            0x8261C088 => {
    //   block [0x8261C088..0x8261C0F8)
	// 8261C088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C094: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C098: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C09C: 38AADFAC  addi r5, r10, -0x2054
	ctx.r[5].s64 = ctx.r[10].s64 + -8276;
	// 8261C0A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C0A4: 390BC538  addi r8, r11, -0x3ac8
	ctx.r[8].s64 = ctx.r[11].s64 + -15048;
	// 8261C0A8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8261C0AC: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8261C0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C0B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C0B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C0C0: 386ADFDC  addi r3, r10, -0x2024
	ctx.r[3].s64 = ctx.r[10].s64 + -8228;
	// 8261C0C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C0C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C0D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C0E4: 4BE4AD3D  bl 0x82466e20
	ctx.lr = 0x8261C0E8;
	sub_82466E20(ctx, base);
	// 8261C0E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C0F8 size=108
    let mut pc: u32 = 0x8261C0F8;
    'dispatch: loop {
        match pc {
            0x8261C0F8 => {
    //   block [0x8261C0F8..0x8261C164)
	// 8261C0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C104: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C10C: 38EBC628  addi r7, r11, -0x39d8
	ctx.r[7].s64 = ctx.r[11].s64 + -14808;
	// 8261C110: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8261C114: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8261C118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C11C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C120: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C128: 386AE00C  addi r3, r10, -0x1ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -8180;
	// 8261C12C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C13C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C14C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C150: 4BE4ACD1  bl 0x82466e20
	ctx.lr = 0x8261C154;
	sub_82466E20(ctx, base);
	// 8261C154: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C160: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C168 size=108
    let mut pc: u32 = 0x8261C168;
    'dispatch: loop {
        match pc {
            0x8261C168 => {
    //   block [0x8261C168..0x8261C1D4)
	// 8261C168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C174: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C17C: 38EBC718  addi r7, r11, -0x38e8
	ctx.r[7].s64 = ctx.r[11].s64 + -14568;
	// 8261C180: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261C184: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8261C188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C18C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C190: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C198: 386AE03C  addi r3, r10, -0x1fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -8132;
	// 8261C19C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C1A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C1A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C1A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C1B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C1B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C1B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C1BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C1C0: 4BE4AC61  bl 0x82466e20
	ctx.lr = 0x8261C1C4;
	sub_82466E20(ctx, base);
	// 8261C1C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C1C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C1CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C1D8 size=108
    let mut pc: u32 = 0x8261C1D8;
    'dispatch: loop {
        match pc {
            0x8261C1D8 => {
    //   block [0x8261C1D8..0x8261C244)
	// 8261C1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C1E4: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C1E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C1EC: 38EBC760  addi r7, r11, -0x38a0
	ctx.r[7].s64 = ctx.r[11].s64 + -14496;
	// 8261C1F0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8261C1F4: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8261C1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C1FC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C208: 386AE06C  addi r3, r10, -0x1f94
	ctx.r[3].s64 = ctx.r[10].s64 + -8084;
	// 8261C20C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C230: 4BE4ABF1  bl 0x82466e20
	ctx.lr = 0x8261C234;
	sub_82466E20(ctx, base);
	// 8261C234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C248 size=108
    let mut pc: u32 = 0x8261C248;
    'dispatch: loop {
        match pc {
            0x8261C248 => {
    //   block [0x8261C248..0x8261C2B4)
	// 8261C248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C254: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C25C: 38EBC838  addi r7, r11, -0x37c8
	ctx.r[7].s64 = ctx.r[11].s64 + -14280;
	// 8261C260: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8261C264: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8261C268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C26C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C278: 386AE09C  addi r3, r10, -0x1f64
	ctx.r[3].s64 = ctx.r[10].s64 + -8036;
	// 8261C27C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C29C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C2A0: 4BE4AB81  bl 0x82466e20
	ctx.lr = 0x8261C2A4;
	sub_82466E20(ctx, base);
	// 8261C2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C2B8 size=100
    let mut pc: u32 = 0x8261C2B8;
    'dispatch: loop {
        match pc {
            0x8261C2B8 => {
    //   block [0x8261C2B8..0x8261C31C)
	// 8261C2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C2C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C2CC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C2D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C2D8: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8261C2DC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C2EC: 386AE0CC  addi r3, r10, -0x1f34
	ctx.r[3].s64 = ctx.r[10].s64 + -7988;
	// 8261C2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C2F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C2F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261C2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C300: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261C304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C308: 4BE4AB19  bl 0x82466e20
	ctx.lr = 0x8261C30C;
	sub_82466E20(ctx, base);
	// 8261C30C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C310: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C314: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C320 size=112
    let mut pc: u32 = 0x8261C320;
    'dispatch: loop {
        match pc {
            0x8261C320 => {
    //   block [0x8261C320..0x8261C390)
	// 8261C320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C32C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C330: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C334: 38AAE0CC  addi r5, r10, -0x1f34
	ctx.r[5].s64 = ctx.r[10].s64 + -7988;
	// 8261C338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C33C: 390BC850  addi r8, r11, -0x37b0
	ctx.r[8].s64 = ctx.r[11].s64 + -14256;
	// 8261C340: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261C344: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8261C348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C34C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C358: 386AE0FC  addi r3, r10, -0x1f04
	ctx.r[3].s64 = ctx.r[10].s64 + -7940;
	// 8261C35C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C37C: 4BE4AAA5  bl 0x82466e20
	ctx.lr = 0x8261C380;
	sub_82466E20(ctx, base);
	// 8261C380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C390 size=108
    let mut pc: u32 = 0x8261C390;
    'dispatch: loop {
        match pc {
            0x8261C390 => {
    //   block [0x8261C390..0x8261C3FC)
	// 8261C390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C39C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C3A4: 38EBC898  addi r7, r11, -0x3768
	ctx.r[7].s64 = ctx.r[11].s64 + -14184;
	// 8261C3A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261C3AC: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8261C3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C3B4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C3B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C3C0: 386AE12C  addi r3, r10, -0x1ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -7892;
	// 8261C3C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C3E8: 4BE4AA39  bl 0x82466e20
	ctx.lr = 0x8261C3EC;
	sub_82466E20(ctx, base);
	// 8261C3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C400 size=112
    let mut pc: u32 = 0x8261C400;
    'dispatch: loop {
        match pc {
            0x8261C400 => {
    //   block [0x8261C400..0x8261C470)
	// 8261C400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C40C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C410: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C414: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C41C: 390BC8E0  addi r8, r11, -0x3720
	ctx.r[8].s64 = ctx.r[11].s64 + -14112;
	// 8261C420: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261C424: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8261C428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C42C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C438: 386AE15C  addi r3, r10, -0x1ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -7844;
	// 8261C43C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C45C: 4BE4A9C5  bl 0x82466e20
	ctx.lr = 0x8261C460;
	sub_82466E20(ctx, base);
	// 8261C460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C470 size=108
    let mut pc: u32 = 0x8261C470;
    'dispatch: loop {
        match pc {
            0x8261C470 => {
    //   block [0x8261C470..0x8261C4DC)
	// 8261C470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C47C: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C484: 38EBC8F8  addi r7, r11, -0x3708
	ctx.r[7].s64 = ctx.r[11].s64 + -14088;
	// 8261C488: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8261C48C: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8261C490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C494: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C4A0: 386AE18C  addi r3, r10, -0x1e74
	ctx.r[3].s64 = ctx.r[10].s64 + -7796;
	// 8261C4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C4C8: 4BE4A959  bl 0x82466e20
	ctx.lr = 0x8261C4CC;
	sub_82466E20(ctx, base);
	// 8261C4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C4E0 size=112
    let mut pc: u32 = 0x8261C4E0;
    'dispatch: loop {
        match pc {
            0x8261C4E0 => {
    //   block [0x8261C4E0..0x8261C550)
	// 8261C4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C4EC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C4F0: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C4F4: 38AAE15C  addi r5, r10, -0x1ea4
	ctx.r[5].s64 = ctx.r[10].s64 + -7844;
	// 8261C4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C4FC: 390BC940  addi r8, r11, -0x36c0
	ctx.r[8].s64 = ctx.r[11].s64 + -14016;
	// 8261C500: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8261C504: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8261C508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C50C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C518: 386AE1BC  addi r3, r10, -0x1e44
	ctx.r[3].s64 = ctx.r[10].s64 + -7748;
	// 8261C51C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C53C: 4BE4A8E5  bl 0x82466e20
	ctx.lr = 0x8261C540;
	sub_82466E20(ctx, base);
	// 8261C540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C550 size=100
    let mut pc: u32 = 0x8261C550;
    'dispatch: loop {
        match pc {
            0x8261C550 => {
    //   block [0x8261C550..0x8261C5B4)
	// 8261C550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C55C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C564: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C568: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C570: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8261C574: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C584: 386AE1EC  addi r3, r10, -0x1e14
	ctx.r[3].s64 = ctx.r[10].s64 + -7700;
	// 8261C588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C58C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C590: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8261C594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8261C59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C5A0: 4BE4A881  bl 0x82466e20
	ctx.lr = 0x8261C5A4;
	sub_82466E20(ctx, base);
	// 8261C5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C5B8 size=112
    let mut pc: u32 = 0x8261C5B8;
    'dispatch: loop {
        match pc {
            0x8261C5B8 => {
    //   block [0x8261C5B8..0x8261C628)
	// 8261C5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C5C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C5C8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C5CC: 38AAE1EC  addi r5, r10, -0x1e14
	ctx.r[5].s64 = ctx.r[10].s64 + -7700;
	// 8261C5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C5D4: 390BC958  addi r8, r11, -0x36a8
	ctx.r[8].s64 = ctx.r[11].s64 + -13992;
	// 8261C5D8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8261C5DC: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 8261C5E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C5E4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C5E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C5EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C5F0: 386AE21C  addi r3, r10, -0x1de4
	ctx.r[3].s64 = ctx.r[10].s64 + -7652;
	// 8261C5F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C5F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C5FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C60C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C614: 4BE4A80D  bl 0x82466e20
	ctx.lr = 0x8261C618;
	sub_82466E20(ctx, base);
	// 8261C618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C61C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C628 size=108
    let mut pc: u32 = 0x8261C628;
    'dispatch: loop {
        match pc {
            0x8261C628 => {
    //   block [0x8261C628..0x8261C694)
	// 8261C628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C634: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C63C: 38EBCA00  addi r7, r11, -0x3600
	ctx.r[7].s64 = ctx.r[11].s64 + -13824;
	// 8261C640: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8261C644: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 8261C648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C64C: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8261C654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C658: 386AE24C  addi r3, r10, -0x1db4
	ctx.r[3].s64 = ctx.r[10].s64 + -7604;
	// 8261C65C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8261C660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C66C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C67C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8261C680: 4BE4A7A1  bl 0x82466e20
	ctx.lr = 0x8261C684;
	sub_82466E20(ctx, base);
	// 8261C684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8261C698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8261C698 size=112
    let mut pc: u32 = 0x8261C698;
    'dispatch: loop {
        match pc {
            0x8261C698 => {
    //   block [0x8261C698..0x8261C708)
	// 8261C698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8261C69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8261C6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8261C6A4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C6A8: 3D608276  lis r11, -0x7d8a
	ctx.r[11].s64 = -2106195968;
	// 8261C6AC: 38AAA58C  addi r5, r10, -0x5a74
	ctx.r[5].s64 = ctx.r[10].s64 + -23156;
	// 8261C6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8261C6B4: 390BCA30  addi r8, r11, -0x35d0
	ctx.r[8].s64 = ctx.r[11].s64 + -13776;
	// 8261C6B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8261C6BC: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 8261C6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8261C6C4: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8261C6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8261C6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8261C6D0: 386AE27C  addi r3, r10, -0x1d84
	ctx.r[3].s64 = ctx.r[10].s64 + -7556;
	// 8261C6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8261C6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8261C6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8261C6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8261C6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8261C6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8261C6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8261C6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8261C6F4: 4BE4A72D  bl 0x82466e20
	ctx.lr = 0x8261C6F8;
	sub_82466E20(ctx, base);
	// 8261C6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8261C6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8261C700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8261C704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


