pub fn sub_82638FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82638FB8 size=100
    let mut pc: u32 = 0x82638FB8;
    'dispatch: loop {
        match pc {
            0x82638FB8 => {
    //   block [0x82638FB8..0x8263901C)
	// 82638FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82638FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82638FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82638FC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82638FCC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82638FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82638FD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82638FD8: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 82638FDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82638FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82638FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82638FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82638FEC: 386AA42C  addi r3, r10, -0x5bd4
	ctx.r[3].s64 = ctx.r[10].s64 + -23508;
	// 82638FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82638FF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82638FF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82638FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639000: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82639004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639008: 4BE2DE19  bl 0x82466e20
	ctx.lr = 0x8263900C;
	sub_82466E20(ctx, base);
	// 8263900C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639020 size=108
    let mut pc: u32 = 0x82639020;
    'dispatch: loop {
        match pc {
            0x82639020 => {
    //   block [0x82639020..0x8263908C)
	// 82639020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263902C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639034: 38EB4778  addi r7, r11, 0x4778
	ctx.r[7].s64 = ctx.r[11].s64 + 18296;
	// 82639038: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263903C: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82639040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639044: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263904C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639050: 386AA45C  addi r3, r10, -0x5ba4
	ctx.r[3].s64 = ctx.r[10].s64 + -23460;
	// 82639054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263905C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263906C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639078: 4BE2DDA9  bl 0x82466e20
	ctx.lr = 0x8263907C;
	sub_82466E20(ctx, base);
	// 8263907C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639090 size=112
    let mut pc: u32 = 0x82639090;
    'dispatch: loop {
        match pc {
            0x82639090 => {
    //   block [0x82639090..0x82639100)
	// 82639090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263909C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826390A0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826390A4: 38AAA42C  addi r5, r10, -0x5bd4
	ctx.r[5].s64 = ctx.r[10].s64 + -23508;
	// 826390A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826390AC: 390B47A8  addi r8, r11, 0x47a8
	ctx.r[8].s64 = ctx.r[11].s64 + 18344;
	// 826390B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826390B4: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826390B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826390BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826390C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826390C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826390C8: 386AA48C  addi r3, r10, -0x5b74
	ctx.r[3].s64 = ctx.r[10].s64 + -23412;
	// 826390CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826390D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826390D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826390D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826390DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826390E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826390E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826390E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826390EC: 4BE2DD35  bl 0x82466e20
	ctx.lr = 0x826390F0;
	sub_82466E20(ctx, base);
	// 826390F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826390F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826390F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826390FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639100 size=108
    let mut pc: u32 = 0x82639100;
    'dispatch: loop {
        match pc {
            0x82639100 => {
    //   block [0x82639100..0x8263916C)
	// 82639100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263910C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639114: 38EB47D8  addi r7, r11, 0x47d8
	ctx.r[7].s64 = ctx.r[11].s64 + 18392;
	// 82639118: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263911C: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 82639120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639124: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639128: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263912C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639130: 386AA4BC  addi r3, r10, -0x5b44
	ctx.r[3].s64 = ctx.r[10].s64 + -23364;
	// 82639134: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263913C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263914C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639158: 4BE2DCC9  bl 0x82466e20
	ctx.lr = 0x8263915C;
	sub_82466E20(ctx, base);
	// 8263915C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639170 size=112
    let mut pc: u32 = 0x82639170;
    'dispatch: loop {
        match pc {
            0x82639170 => {
    //   block [0x82639170..0x826391E0)
	// 82639170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263917C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639180: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639184: 38AAA42C  addi r5, r10, -0x5bd4
	ctx.r[5].s64 = ctx.r[10].s64 + -23508;
	// 82639188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263918C: 390B4808  addi r8, r11, 0x4808
	ctx.r[8].s64 = ctx.r[11].s64 + 18440;
	// 82639190: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82639194: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 82639198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263919C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826391A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826391A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826391A8: 386AA4EC  addi r3, r10, -0x5b14
	ctx.r[3].s64 = ctx.r[10].s64 + -23316;
	// 826391AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826391B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826391B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826391B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826391BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826391C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826391C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826391C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826391CC: 4BE2DC55  bl 0x82466e20
	ctx.lr = 0x826391D0;
	sub_82466E20(ctx, base);
	// 826391D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826391D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826391D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826391DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826391E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826391E0 size=108
    let mut pc: u32 = 0x826391E0;
    'dispatch: loop {
        match pc {
            0x826391E0 => {
    //   block [0x826391E0..0x8263924C)
	// 826391E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826391E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826391E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826391EC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826391F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826391F4: 38EB4850  addi r7, r11, 0x4850
	ctx.r[7].s64 = ctx.r[11].s64 + 18512;
	// 826391F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826391FC: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 82639200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639204: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263920C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639210: 386AA51C  addi r3, r10, -0x5ae4
	ctx.r[3].s64 = ctx.r[10].s64 + -23268;
	// 82639214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263921C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263922C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639238: 4BE2DBE9  bl 0x82466e20
	ctx.lr = 0x8263923C;
	sub_82466E20(ctx, base);
	// 8263923C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639250 size=112
    let mut pc: u32 = 0x82639250;
    'dispatch: loop {
        match pc {
            0x82639250 => {
    //   block [0x82639250..0x826392C0)
	// 82639250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263925C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639260: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639264: 38AAA42C  addi r5, r10, -0x5bd4
	ctx.r[5].s64 = ctx.r[10].s64 + -23508;
	// 82639268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263926C: 390B4880  addi r8, r11, 0x4880
	ctx.r[8].s64 = ctx.r[11].s64 + 18560;
	// 82639270: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82639274: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 82639278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263927C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639280: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639288: 386AA54C  addi r3, r10, -0x5ab4
	ctx.r[3].s64 = ctx.r[10].s64 + -23220;
	// 8263928C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263929C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826392A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826392A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826392A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826392AC: 4BE2DB75  bl 0x82466e20
	ctx.lr = 0x826392B0;
	sub_82466E20(ctx, base);
	// 826392B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826392B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826392B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826392BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826392C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826392C0 size=108
    let mut pc: u32 = 0x826392C0;
    'dispatch: loop {
        match pc {
            0x826392C0 => {
    //   block [0x826392C0..0x8263932C)
	// 826392C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826392C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826392C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826392CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826392D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826392D4: 38EB48C8  addi r7, r11, 0x48c8
	ctx.r[7].s64 = ctx.r[11].s64 + 18632;
	// 826392D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826392DC: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 826392E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826392E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826392E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826392EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826392F0: 386AA57C  addi r3, r10, -0x5a84
	ctx.r[3].s64 = ctx.r[10].s64 + -23172;
	// 826392F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826392F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826392FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263930C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639318: 4BE2DB09  bl 0x82466e20
	ctx.lr = 0x8263931C;
	sub_82466E20(ctx, base);
	// 8263931C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639330 size=112
    let mut pc: u32 = 0x82639330;
    'dispatch: loop {
        match pc {
            0x82639330 => {
    //   block [0x82639330..0x826393A0)
	// 82639330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263933C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639340: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639344: 38AAA42C  addi r5, r10, -0x5bd4
	ctx.r[5].s64 = ctx.r[10].s64 + -23508;
	// 82639348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263934C: 390B48F8  addi r8, r11, 0x48f8
	ctx.r[8].s64 = ctx.r[11].s64 + 18680;
	// 82639350: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82639354: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82639358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263935C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639368: 386AA5AC  addi r3, r10, -0x5a54
	ctx.r[3].s64 = ctx.r[10].s64 + -23124;
	// 8263936C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263937C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263938C: 4BE2DA95  bl 0x82466e20
	ctx.lr = 0x82639390;
	sub_82466E20(ctx, base);
	// 82639390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263939C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826393A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826393A0 size=108
    let mut pc: u32 = 0x826393A0;
    'dispatch: loop {
        match pc {
            0x826393A0 => {
    //   block [0x826393A0..0x8263940C)
	// 826393A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826393A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826393A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826393AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826393B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826393B4: 38EB4948  addi r7, r11, 0x4948
	ctx.r[7].s64 = ctx.r[11].s64 + 18760;
	// 826393B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826393BC: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826393C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826393C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826393C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826393CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826393D0: 386AA5DC  addi r3, r10, -0x5a24
	ctx.r[3].s64 = ctx.r[10].s64 + -23076;
	// 826393D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826393D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826393DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826393E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826393E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826393E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826393EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826393F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826393F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826393F8: 4BE2DA29  bl 0x82466e20
	ctx.lr = 0x826393FC;
	sub_82466E20(ctx, base);
	// 826393FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639410 size=112
    let mut pc: u32 = 0x82639410;
    'dispatch: loop {
        match pc {
            0x82639410 => {
    //   block [0x82639410..0x82639480)
	// 82639410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263941C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639420: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639424: 392A61B8  addi r9, r10, 0x61b8
	ctx.r[9].s64 = ctx.r[10].s64 + 25016;
	// 82639428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263942C: 390B49A8  addi r8, r11, 0x49a8
	ctx.r[8].s64 = ctx.r[11].s64 + 18856;
	// 82639430: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82639434: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82639438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263943C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639448: 386AA60C  addi r3, r10, -0x59f4
	ctx.r[3].s64 = ctx.r[10].s64 + -23028;
	// 8263944C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82639450: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82639454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263945C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263946C: 4BE2D9B5  bl 0x82466e20
	ctx.lr = 0x82639470;
	sub_82466E20(ctx, base);
	// 82639470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263947C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639480 size=108
    let mut pc: u32 = 0x82639480;
    'dispatch: loop {
        match pc {
            0x82639480 => {
    //   block [0x82639480..0x826394EC)
	// 82639480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263948C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639494: 38EB4A68  addi r7, r11, 0x4a68
	ctx.r[7].s64 = ctx.r[11].s64 + 19048;
	// 82639498: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263949C: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 826394A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826394A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826394A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826394AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826394B0: 386AA63C  addi r3, r10, -0x59c4
	ctx.r[3].s64 = ctx.r[10].s64 + -22980;
	// 826394B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826394B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826394BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826394C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826394C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826394C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826394CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826394D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826394D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826394D8: 4BE2D949  bl 0x82466e20
	ctx.lr = 0x826394DC;
	sub_82466E20(ctx, base);
	// 826394DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826394E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826394E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826394E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826394F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826394F0 size=108
    let mut pc: u32 = 0x826394F0;
    'dispatch: loop {
        match pc {
            0x826394F0 => {
    //   block [0x826394F0..0x8263955C)
	// 826394F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826394F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826394F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826394FC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639504: 38EB4AB0  addi r7, r11, 0x4ab0
	ctx.r[7].s64 = ctx.r[11].s64 + 19120;
	// 82639508: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8263950C: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 82639510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639514: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263951C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639520: 386AA66C  addi r3, r10, -0x5994
	ctx.r[3].s64 = ctx.r[10].s64 + -22932;
	// 82639524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263952C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263953C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639548: 4BE2D8D9  bl 0x82466e20
	ctx.lr = 0x8263954C;
	sub_82466E20(ctx, base);
	// 8263954C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82639560 size=24
    let mut pc: u32 = 0x82639560;
    'dispatch: loop {
        match pc {
            0x82639560 => {
    //   block [0x82639560..0x82639578)
	// 82639560: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639564: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82639568: 394AAEE8  addi r10, r10, -0x5118
	ctx.r[10].s64 = ctx.r[10].s64 + -20760;
	// 8263956C: 816B4940  lwz r11, 0x4940(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18752 as u32) ) } as u64;
	// 82639570: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82639574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639578 size=116
    let mut pc: u32 = 0x82639578;
    'dispatch: loop {
        match pc {
            0x82639578 => {
    //   block [0x82639578..0x826395EC)
	// 82639578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639584: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82639588: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263958C: 392B6144  addi r9, r11, 0x6144
	ctx.r[9].s64 = ctx.r[11].s64 + 24900;
	// 82639590: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82639594: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639598: 38E9008C  addi r7, r9, 0x8c
	ctx.r[7].s64 = ctx.r[9].s64 + 140;
	// 8263959C: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 826395A0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 826395A4: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826395A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826395AC: 396BAEE8  addi r11, r11, -0x5118
	ctx.r[11].s64 = ctx.r[11].s64 + -20760;
	// 826395B0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826395B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826395B8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826395BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826395C0: 386AA69C  addi r3, r10, -0x5964
	ctx.r[3].s64 = ctx.r[10].s64 + -22884;
	// 826395C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826395C8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826395CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826395D0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826395D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826395D8: 4BE2D849  bl 0x82466e20
	ctx.lr = 0x826395DC;
	sub_82466E20(ctx, base);
	// 826395DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826395E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826395E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826395E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826395F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826395F0 size=100
    let mut pc: u32 = 0x826395F0;
    'dispatch: loop {
        match pc {
            0x826395F0 => {
    //   block [0x826395F0..0x82639654)
	// 826395F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826395F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826395F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826395FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639604: 38AA9B2C  addi r5, r10, -0x64d4
	ctx.r[5].s64 = ctx.r[10].s64 + -25812;
	// 82639608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263960C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639610: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 82639614: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263961C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639624: 386AA6CC  addi r3, r10, -0x5934
	ctx.r[3].s64 = ctx.r[10].s64 + -22836;
	// 82639628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263962C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639630: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82639634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639638: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263963C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639640: 4BE2D7E1  bl 0x82466e20
	ctx.lr = 0x82639644;
	sub_82466E20(ctx, base);
	// 82639644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263964C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82639658 size=24
    let mut pc: u32 = 0x82639658;
    'dispatch: loop {
        match pc {
            0x82639658 => {
    //   block [0x82639658..0x82639670)
	// 82639658: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263965C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82639660: 394AB098  addi r10, r10, -0x4f68
	ctx.r[10].s64 = ctx.r[10].s64 + -20328;
	// 82639664: 816B4B44  lwz r11, 0x4b44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19268 as u32) ) } as u64;
	// 82639668: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8263966C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639670 size=116
    let mut pc: u32 = 0x82639670;
    'dispatch: loop {
        match pc {
            0x82639670 => {
    //   block [0x82639670..0x826396E4)
	// 82639670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263967C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82639680: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639684: 390BB098  addi r8, r11, -0x4f68
	ctx.r[8].s64 = ctx.r[11].s64 + -20328;
	// 82639688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263968C: 392A6250  addi r9, r10, 0x6250
	ctx.r[9].s64 = ctx.r[10].s64 + 25168;
	// 82639690: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639694: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82639698: 38AAA6CC  addi r5, r10, -0x5934
	ctx.r[5].s64 = ctx.r[10].s64 + -22836;
	// 8263969C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826396A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826396A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826396A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826396AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826396B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826396B4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 826396B8: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826396BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826396C0: 386BA6FC  addi r3, r11, -0x5904
	ctx.r[3].s64 = ctx.r[11].s64 + -22788;
	// 826396C4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826396C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826396CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826396D0: 4BE2D751  bl 0x82466e20
	ctx.lr = 0x826396D4;
	sub_82466E20(ctx, base);
	// 826396D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826396D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826396DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826396E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826396E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826396E8 size=112
    let mut pc: u32 = 0x826396E8;
    'dispatch: loop {
        match pc {
            0x826396E8 => {
    //   block [0x826396E8..0x82639758)
	// 826396E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826396EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826396F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826396F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826396F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826396FC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82639700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639704: 390B4B48  addi r8, r11, 0x4b48
	ctx.r[8].s64 = ctx.r[11].s64 + 19272;
	// 82639708: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8263970C: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 82639710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639714: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263971C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639720: 386AA72C  addi r3, r10, -0x58d4
	ctx.r[3].s64 = ctx.r[10].s64 + -22740;
	// 82639724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263972C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263973C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639744: 4BE2D6DD  bl 0x82466e20
	ctx.lr = 0x82639748;
	sub_82466E20(ctx, base);
	// 82639748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263974C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639758 size=108
    let mut pc: u32 = 0x82639758;
    'dispatch: loop {
        match pc {
            0x82639758 => {
    //   block [0x82639758..0x826397C4)
	// 82639758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263975C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639764: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263976C: 38EB4C20  addi r7, r11, 0x4c20
	ctx.r[7].s64 = ctx.r[11].s64 + 19488;
	// 82639770: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82639774: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 82639778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263977C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639780: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639788: 386AA75C  addi r3, r10, -0x58a4
	ctx.r[3].s64 = ctx.r[10].s64 + -22692;
	// 8263978C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263979C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826397A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826397A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826397A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826397AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826397B0: 4BE2D671  bl 0x82466e20
	ctx.lr = 0x826397B4;
	sub_82466E20(ctx, base);
	// 826397B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826397B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826397BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826397C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826397C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826397C8 size=108
    let mut pc: u32 = 0x826397C8;
    'dispatch: loop {
        match pc {
            0x826397C8 => {
    //   block [0x826397C8..0x82639834)
	// 826397C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826397CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826397D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826397D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826397D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826397DC: 38EB4C98  addi r7, r11, 0x4c98
	ctx.r[7].s64 = ctx.r[11].s64 + 19608;
	// 826397E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826397E4: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 826397E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826397EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826397F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826397F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826397F8: 386AA78C  addi r3, r10, -0x5874
	ctx.r[3].s64 = ctx.r[10].s64 + -22644;
	// 826397FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263980C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263981C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639820: 4BE2D601  bl 0x82466e20
	ctx.lr = 0x82639824;
	sub_82466E20(ctx, base);
	// 82639824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263982C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639838 size=112
    let mut pc: u32 = 0x82639838;
    'dispatch: loop {
        match pc {
            0x82639838 => {
    //   block [0x82639838..0x826398A8)
	// 82639838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263983C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639844: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639848: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263984C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82639850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639854: 390B4CE0  addi r8, r11, 0x4ce0
	ctx.r[8].s64 = ctx.r[11].s64 + 19680;
	// 82639858: 39200013  li r9, 0x13
	ctx.r[9].s64 = 19;
	// 8263985C: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 82639860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263986C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639870: 386AA7BC  addi r3, r10, -0x5844
	ctx.r[3].s64 = ctx.r[10].s64 + -22596;
	// 82639874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263987C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263988C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639894: 4BE2D58D  bl 0x82466e20
	ctx.lr = 0x82639898;
	sub_82466E20(ctx, base);
	// 82639898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263989C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826398A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826398A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826398A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826398A8 size=108
    let mut pc: u32 = 0x826398A8;
    'dispatch: loop {
        match pc {
            0x826398A8 => {
    //   block [0x826398A8..0x82639914)
	// 826398A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826398AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826398B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826398B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826398B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826398BC: 38EB4EA8  addi r7, r11, 0x4ea8
	ctx.r[7].s64 = ctx.r[11].s64 + 20136;
	// 826398C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826398C4: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826398C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826398CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826398D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826398D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826398D8: 386AA7EC  addi r3, r10, -0x5814
	ctx.r[3].s64 = ctx.r[10].s64 + -22548;
	// 826398DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826398E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826398E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826398E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826398EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826398F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826398F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826398F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826398FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639900: 4BE2D521  bl 0x82466e20
	ctx.lr = 0x82639904;
	sub_82466E20(ctx, base);
	// 82639904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263990C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82639918 size=24
    let mut pc: u32 = 0x82639918;
    'dispatch: loop {
        match pc {
            0x82639918 => {
    //   block [0x82639918..0x82639930)
	// 82639918: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263991C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82639920: 394AB1B8  addi r10, r10, -0x4e48
	ctx.r[10].s64 = ctx.r[10].s64 + -20040;
	// 82639924: 816B4EC0  lwz r11, 0x4ec0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20160 as u32) ) } as u64;
	// 82639928: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263992C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639930 size=112
    let mut pc: u32 = 0x82639930;
    'dispatch: loop {
        match pc {
            0x82639930 => {
    //   block [0x82639930..0x826399A0)
	// 82639930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263993C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639940: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82639944: 392A62A8  addi r9, r10, 0x62a8
	ctx.r[9].s64 = ctx.r[10].s64 + 25256;
	// 82639948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263994C: 390BB1B8  addi r8, r11, -0x4e48
	ctx.r[8].s64 = ctx.r[11].s64 + -20040;
	// 82639950: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82639954: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82639958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263995C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639968: 386AA81C  addi r3, r10, -0x57e4
	ctx.r[3].s64 = ctx.r[10].s64 + -22500;
	// 8263996C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82639970: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82639974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263997C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263998C: 4BE2D495  bl 0x82466e20
	ctx.lr = 0x82639990;
	sub_82466E20(ctx, base);
	// 82639990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263999C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826399A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826399A0 size=108
    let mut pc: u32 = 0x826399A0;
    'dispatch: loop {
        match pc {
            0x826399A0 => {
    //   block [0x826399A0..0x82639A0C)
	// 826399A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826399A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826399A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826399AC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 826399B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826399B4: 38EB4EC8  addi r7, r11, 0x4ec8
	ctx.r[7].s64 = ctx.r[11].s64 + 20168;
	// 826399B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826399BC: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826399C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826399C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826399C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826399CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826399D0: 386AA84C  addi r3, r10, -0x57b4
	ctx.r[3].s64 = ctx.r[10].s64 + -22452;
	// 826399D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826399D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826399DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826399E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826399E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826399E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826399EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826399F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826399F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826399F8: 4BE2D429  bl 0x82466e20
	ctx.lr = 0x826399FC;
	sub_82466E20(ctx, base);
	// 826399FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639A10 size=112
    let mut pc: u32 = 0x82639A10;
    'dispatch: loop {
        match pc {
            0x82639A10 => {
    //   block [0x82639A10..0x82639A80)
	// 82639A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639A1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639A20: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639A24: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82639A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639A2C: 390B4F28  addi r8, r11, 0x4f28
	ctx.r[8].s64 = ctx.r[11].s64 + 20264;
	// 82639A30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82639A34: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 82639A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639A3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639A40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639A48: 386AA87C  addi r3, r10, -0x5784
	ctx.r[3].s64 = ctx.r[10].s64 + -22404;
	// 82639A4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639A54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639A5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639A6C: 4BE2D3B5  bl 0x82466e20
	ctx.lr = 0x82639A70;
	sub_82466E20(ctx, base);
	// 82639A70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639A74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639A78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639A7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639A80 size=108
    let mut pc: u32 = 0x82639A80;
    'dispatch: loop {
        match pc {
            0x82639A80 => {
    //   block [0x82639A80..0x82639AEC)
	// 82639A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639A8C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639A94: 38EB4F40  addi r7, r11, 0x4f40
	ctx.r[7].s64 = ctx.r[11].s64 + 20288;
	// 82639A98: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82639A9C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82639AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639AA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639AB0: 386AA8AC  addi r3, r10, -0x5754
	ctx.r[3].s64 = ctx.r[10].s64 + -22356;
	// 82639AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639AD8: 4BE2D349  bl 0x82466e20
	ctx.lr = 0x82639ADC;
	sub_82466E20(ctx, base);
	// 82639ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639AF0 size=108
    let mut pc: u32 = 0x82639AF0;
    'dispatch: loop {
        match pc {
            0x82639AF0 => {
    //   block [0x82639AF0..0x82639B5C)
	// 82639AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639AFC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639B04: 38EB4FA0  addi r7, r11, 0x4fa0
	ctx.r[7].s64 = ctx.r[11].s64 + 20384;
	// 82639B08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82639B0C: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 82639B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639B14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639B20: 386AA8DC  addi r3, r10, -0x5724
	ctx.r[3].s64 = ctx.r[10].s64 + -22308;
	// 82639B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639B48: 4BE2D2D9  bl 0x82466e20
	ctx.lr = 0x82639B4C;
	sub_82466E20(ctx, base);
	// 82639B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639B60 size=116
    let mut pc: u32 = 0x82639B60;
    'dispatch: loop {
        match pc {
            0x82639B60 => {
    //   block [0x82639B60..0x82639BD4)
	// 82639B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639B6C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639B70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639B74: 390B4FD0  addi r8, r11, 0x4fd0
	ctx.r[8].s64 = ctx.r[11].s64 + 20432;
	// 82639B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639B7C: 392A62D4  addi r9, r10, 0x62d4
	ctx.r[9].s64 = ctx.r[10].s64 + 25300;
	// 82639B80: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639B84: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82639B88: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 82639B8C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639B94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82639B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639BA4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 82639BA8: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82639BAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82639BB0: 386BA90C  addi r3, r11, -0x56f4
	ctx.r[3].s64 = ctx.r[11].s64 + -22260;
	// 82639BB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82639BB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639BC0: 4BE2D261  bl 0x82466e20
	ctx.lr = 0x82639BC4;
	sub_82466E20(ctx, base);
	// 82639BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639BD8 size=96
    let mut pc: u32 = 0x82639BD8;
    'dispatch: loop {
        match pc {
            0x82639BD8 => {
    //   block [0x82639BD8..0x82639C38)
	// 82639BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639BE4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639BEC: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 82639BF0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639BF8: 386AA93C  addi r3, r10, -0x56c4
	ctx.r[3].s64 = ctx.r[10].s64 + -22212;
	// 82639BFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639C04: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82639C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639C18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82639C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639C20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82639C24: 4BE2D1FD  bl 0x82466e20
	ctx.lr = 0x82639C28;
	sub_82466E20(ctx, base);
	// 82639C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639C38 size=112
    let mut pc: u32 = 0x82639C38;
    'dispatch: loop {
        match pc {
            0x82639C38 => {
    //   block [0x82639C38..0x82639CA8)
	// 82639C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639C44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639C48: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639C4C: 38AAA93C  addi r5, r10, -0x56c4
	ctx.r[5].s64 = ctx.r[10].s64 + -22212;
	// 82639C50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639C54: 390B4FE8  addi r8, r11, 0x4fe8
	ctx.r[8].s64 = ctx.r[11].s64 + 20456;
	// 82639C58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82639C5C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82639C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639C64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639C70: 386AA96C  addi r3, r10, -0x5694
	ctx.r[3].s64 = ctx.r[10].s64 + -22164;
	// 82639C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82639C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639C94: 4BE2D18D  bl 0x82466e20
	ctx.lr = 0x82639C98;
	sub_82466E20(ctx, base);
	// 82639C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639CA8 size=112
    let mut pc: u32 = 0x82639CA8;
    'dispatch: loop {
        match pc {
            0x82639CA8 => {
    //   block [0x82639CA8..0x82639D18)
	// 82639CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639CB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639CB8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639CBC: 392A62F0  addi r9, r10, 0x62f0
	ctx.r[9].s64 = ctx.r[10].s64 + 25328;
	// 82639CC0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639CC4: 390B5020  addi r8, r11, 0x5020
	ctx.r[8].s64 = ctx.r[11].s64 + 20512;
	// 82639CC8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82639CCC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82639CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639CD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639CDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639CE0: 386AA99C  addi r3, r10, -0x5664
	ctx.r[3].s64 = ctx.r[10].s64 + -22116;
	// 82639CE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82639CE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82639CEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639D04: 4BE2D11D  bl 0x82466e20
	ctx.lr = 0x82639D08;
	sub_82466E20(ctx, base);
	// 82639D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639D18 size=108
    let mut pc: u32 = 0x82639D18;
    'dispatch: loop {
        match pc {
            0x82639D18 => {
    //   block [0x82639D18..0x82639D84)
	// 82639D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639D24: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639D28: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639D2C: 38EB50C8  addi r7, r11, 0x50c8
	ctx.r[7].s64 = ctx.r[11].s64 + 20680;
	// 82639D30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82639D34: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82639D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639D3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639D48: 386AA9CC  addi r3, r10, -0x5634
	ctx.r[3].s64 = ctx.r[10].s64 + -22068;
	// 82639D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639D70: 4BE2D0B1  bl 0x82466e20
	ctx.lr = 0x82639D74;
	sub_82466E20(ctx, base);
	// 82639D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639D88 size=108
    let mut pc: u32 = 0x82639D88;
    'dispatch: loop {
        match pc {
            0x82639D88 => {
    //   block [0x82639D88..0x82639DF4)
	// 82639D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639D94: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639D98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639D9C: 38EB50F8  addi r7, r11, 0x50f8
	ctx.r[7].s64 = ctx.r[11].s64 + 20728;
	// 82639DA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82639DA4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82639DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639DAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639DB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639DB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639DB8: 386AA9FC  addi r3, r10, -0x5604
	ctx.r[3].s64 = ctx.r[10].s64 + -22020;
	// 82639DBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639DCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639DDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639DE0: 4BE2D041  bl 0x82466e20
	ctx.lr = 0x82639DE4;
	sub_82466E20(ctx, base);
	// 82639DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82639DF8 size=28
    let mut pc: u32 = 0x82639DF8;
    'dispatch: loop {
        match pc {
            0x82639DF8 => {
    //   block [0x82639DF8..0x82639E14)
	// 82639DF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639DFC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82639E00: 394AB1E8  addi r10, r10, -0x4e18
	ctx.r[10].s64 = ctx.r[10].s64 + -19992;
	// 82639E04: 816B501C  lwz r11, 0x501c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20508 as u32) ) } as u64;
	// 82639E08: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82639E0C: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82639E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639E18 size=112
    let mut pc: u32 = 0x82639E18;
    'dispatch: loop {
        match pc {
            0x82639E18 => {
    //   block [0x82639E18..0x82639E88)
	// 82639E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639E24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82639E28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 82639E2C: 392A6480  addi r9, r10, 0x6480
	ctx.r[9].s64 = ctx.r[10].s64 + 25728;
	// 82639E30: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639E34: 390BB1E8  addi r8, r11, -0x4e18
	ctx.r[8].s64 = ctx.r[11].s64 + -19992;
	// 82639E38: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82639E3C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82639E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639E44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82639E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639E50: 386AAA2C  addi r3, r10, -0x55d4
	ctx.r[3].s64 = ctx.r[10].s64 + -21972;
	// 82639E54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82639E58: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82639E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639E6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639E74: 4BE2CFAD  bl 0x82466e20
	ctx.lr = 0x82639E78;
	sub_82466E20(ctx, base);
	// 82639E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639E88 size=108
    let mut pc: u32 = 0x82639E88;
    'dispatch: loop {
        match pc {
            0x82639E88 => {
    //   block [0x82639E88..0x82639EF4)
	// 82639E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639E94: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639E98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639E9C: 38EB5130  addi r7, r11, 0x5130
	ctx.r[7].s64 = ctx.r[11].s64 + 20784;
	// 82639EA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82639EA4: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82639EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639EAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639EB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639EB8: 386AAA5C  addi r3, r10, -0x55a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21924;
	// 82639EBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639EDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639EE0: 4BE2CF41  bl 0x82466e20
	ctx.lr = 0x82639EE4;
	sub_82466E20(ctx, base);
	// 82639EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639EF8 size=108
    let mut pc: u32 = 0x82639EF8;
    'dispatch: loop {
        match pc {
            0x82639EF8 => {
    //   block [0x82639EF8..0x82639F64)
	// 82639EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639F04: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639F08: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639F0C: 38EB5160  addi r7, r11, 0x5160
	ctx.r[7].s64 = ctx.r[11].s64 + 20832;
	// 82639F10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82639F14: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 82639F18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639F1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639F20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639F24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639F28: 386AAA8C  addi r3, r10, -0x5574
	ctx.r[3].s64 = ctx.r[10].s64 + -21876;
	// 82639F2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639F30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639F3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639F44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639F48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639F4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639F50: 4BE2CED1  bl 0x82466e20
	ctx.lr = 0x82639F54;
	sub_82466E20(ctx, base);
	// 82639F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639F68 size=108
    let mut pc: u32 = 0x82639F68;
    'dispatch: loop {
        match pc {
            0x82639F68 => {
    //   block [0x82639F68..0x82639FD4)
	// 82639F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639F74: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639F78: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82639F7C: 38EB5190  addi r7, r11, 0x5190
	ctx.r[7].s64 = ctx.r[11].s64 + 20880;
	// 82639F80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82639F84: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 82639F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82639F8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82639F90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82639F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82639F98: 386AAABC  addi r3, r10, -0x5544
	ctx.r[3].s64 = ctx.r[10].s64 + -21828;
	// 82639F9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82639FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82639FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82639FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82639FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82639FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82639FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82639FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82639FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82639FC0: 4BE2CE61  bl 0x82466e20
	ctx.lr = 0x82639FC4;
	sub_82466E20(ctx, base);
	// 82639FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82639FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82639FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82639FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82639FD8 size=24
    let mut pc: u32 = 0x82639FD8;
    'dispatch: loop {
        match pc {
            0x82639FD8 => {
    //   block [0x82639FD8..0x82639FF0)
	// 82639FD8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 82639FDC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 82639FE0: 394AB2A8  addi r10, r10, -0x4d58
	ctx.r[10].s64 = ctx.r[10].s64 + -19800;
	// 82639FE4: 816B51A8  lwz r11, 0x51a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20904 as u32) ) } as u64;
	// 82639FE8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82639FEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82639FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82639FF0 size=112
    let mut pc: u32 = 0x82639FF0;
    'dispatch: loop {
        match pc {
            0x82639FF0 => {
    //   block [0x82639FF0..0x8263A060)
	// 82639FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82639FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82639FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82639FFC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263A000: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263A004: 392A64D4  addi r9, r10, 0x64d4
	ctx.r[9].s64 = ctx.r[10].s64 + 25812;
	// 8263A008: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8263A00C: 390BB2A8  addi r8, r11, -0x4d58
	ctx.r[8].s64 = ctx.r[11].s64 + -19800;
	// 8263A010: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8263A014: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 8263A018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A01C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263A024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A028: 386AAAEC  addi r3, r10, -0x5514
	ctx.r[3].s64 = ctx.r[10].s64 + -21780;
	// 8263A02C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263A030: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263A034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A04C: 4BE2CDD5  bl 0x82466e20
	ctx.lr = 0x8263A050;
	sub_82466E20(ctx, base);
	// 8263A050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A060 size=112
    let mut pc: u32 = 0x8263A060;
    'dispatch: loop {
        match pc {
            0x8263A060 => {
    //   block [0x8263A060..0x8263A0D0)
	// 8263A060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A06C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263A070: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A074: 392A6510  addi r9, r10, 0x6510
	ctx.r[9].s64 = ctx.r[10].s64 + 25872;
	// 8263A078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A07C: 390B51B8  addi r8, r11, 0x51b8
	ctx.r[8].s64 = ctx.r[11].s64 + 20920;
	// 8263A080: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263A084: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 8263A088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A08C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A090: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263A094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A098: 386AAB1C  addi r3, r10, -0x54e4
	ctx.r[3].s64 = ctx.r[10].s64 + -21732;
	// 8263A09C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263A0A0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8263A0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A0AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A0B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A0BC: 4BE2CD65  bl 0x82466e20
	ctx.lr = 0x8263A0C0;
	sub_82466E20(ctx, base);
	// 8263A0C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A0C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A0C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A0CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A0D0 size=108
    let mut pc: u32 = 0x8263A0D0;
    'dispatch: loop {
        match pc {
            0x8263A0D0 => {
    //   block [0x8263A0D0..0x8263A13C)
	// 8263A0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A0DC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A0E0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8263A0E4: 38EB5200  addi r7, r11, 0x5200
	ctx.r[7].s64 = ctx.r[11].s64 + 20992;
	// 8263A0E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A0EC: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 8263A0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A0F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A0F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A100: 386AAB4C  addi r3, r10, -0x54b4
	ctx.r[3].s64 = ctx.r[10].s64 + -21684;
	// 8263A104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A128: 4BE2CCF9  bl 0x82466e20
	ctx.lr = 0x8263A12C;
	sub_82466E20(ctx, base);
	// 8263A12C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A140 size=108
    let mut pc: u32 = 0x8263A140;
    'dispatch: loop {
        match pc {
            0x8263A140 => {
    //   block [0x8263A140..0x8263A1AC)
	// 8263A140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A14C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A150: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8263A154: 38EB5230  addi r7, r11, 0x5230
	ctx.r[7].s64 = ctx.r[11].s64 + 21040;
	// 8263A158: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A15C: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 8263A160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A168: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A16C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A170: 386AAB7C  addi r3, r10, -0x5484
	ctx.r[3].s64 = ctx.r[10].s64 + -21636;
	// 8263A174: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A17C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A198: 4BE2CC89  bl 0x82466e20
	ctx.lr = 0x8263A19C;
	sub_82466E20(ctx, base);
	// 8263A19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A1B0 size=112
    let mut pc: u32 = 0x8263A1B0;
    'dispatch: loop {
        match pc {
            0x8263A1B0 => {
    //   block [0x8263A1B0..0x8263A220)
	// 8263A1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A1BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263A1C0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A1C4: 392A6548  addi r9, r10, 0x6548
	ctx.r[9].s64 = ctx.r[10].s64 + 25928;
	// 8263A1C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A1CC: 390B5260  addi r8, r11, 0x5260
	ctx.r[8].s64 = ctx.r[11].s64 + 21088;
	// 8263A1D0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8263A1D4: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 8263A1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A1DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263A1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A1E8: 386AABAC  addi r3, r10, -0x5454
	ctx.r[3].s64 = ctx.r[10].s64 + -21588;
	// 8263A1EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263A1F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263A1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A1FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A204: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A20C: 4BE2CC15  bl 0x82466e20
	ctx.lr = 0x8263A210;
	sub_82466E20(ctx, base);
	// 8263A210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A220 size=108
    let mut pc: u32 = 0x8263A220;
    'dispatch: loop {
        match pc {
            0x8263A220 => {
    //   block [0x8263A220..0x8263A28C)
	// 8263A220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A22C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A230: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8263A234: 38EB52C0  addi r7, r11, 0x52c0
	ctx.r[7].s64 = ctx.r[11].s64 + 21184;
	// 8263A238: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8263A23C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 8263A240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A244: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A248: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A24C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A250: 386AABDC  addi r3, r10, -0x5424
	ctx.r[3].s64 = ctx.r[10].s64 + -21540;
	// 8263A254: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A278: 4BE2CBA9  bl 0x82466e20
	ctx.lr = 0x8263A27C;
	sub_82466E20(ctx, base);
	// 8263A27C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A290 size=108
    let mut pc: u32 = 0x8263A290;
    'dispatch: loop {
        match pc {
            0x8263A290 => {
    //   block [0x8263A290..0x8263A2FC)
	// 8263A290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A29C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A2A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8263A2A4: 38EB53C8  addi r7, r11, 0x53c8
	ctx.r[7].s64 = ctx.r[11].s64 + 21448;
	// 8263A2A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263A2AC: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 8263A2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A2B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A2B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A2C0: 386AAC0C  addi r3, r10, -0x53f4
	ctx.r[3].s64 = ctx.r[10].s64 + -21492;
	// 8263A2C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A2CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A2E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A2E8: 4BE2CB39  bl 0x82466e20
	ctx.lr = 0x8263A2EC;
	sub_82466E20(ctx, base);
	// 8263A2EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A2F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A2F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A2F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A300 size=108
    let mut pc: u32 = 0x8263A300;
    'dispatch: loop {
        match pc {
            0x8263A300 => {
    //   block [0x8263A300..0x8263A36C)
	// 8263A300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A30C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A314: 38EB53E0  addi r7, r11, 0x53e0
	ctx.r[7].s64 = ctx.r[11].s64 + 21472;
	// 8263A318: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8263A31C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 8263A320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A324: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A328: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A330: 386AAC3C  addi r3, r10, -0x53c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21444;
	// 8263A334: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A33C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A354: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A358: 4BE2CAC9  bl 0x82466e20
	ctx.lr = 0x8263A35C;
	sub_82466E20(ctx, base);
	// 8263A35C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A368: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263A370 size=24
    let mut pc: u32 = 0x8263A370;
    'dispatch: loop {
        match pc {
            0x8263A370 => {
    //   block [0x8263A370..0x8263A388)
	// 8263A370: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A374: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263A378: 394AB380  addi r10, r10, -0x4c80
	ctx.r[10].s64 = ctx.r[10].s64 + -19584;
	// 8263A37C: 816B5470  lwz r11, 0x5470(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21616 as u32) ) } as u64;
	// 8263A380: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263A384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A388 size=108
    let mut pc: u32 = 0x8263A388;
    'dispatch: loop {
        match pc {
            0x8263A388 => {
    //   block [0x8263A388..0x8263A3F4)
	// 8263A388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A394: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263A398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A39C: 38EBB380  addi r7, r11, -0x4c80
	ctx.r[7].s64 = ctx.r[11].s64 + -19584;
	// 8263A3A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A3A4: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 8263A3A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A3AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A3B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A3B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A3B8: 386AAC6C  addi r3, r10, -0x5394
	ctx.r[3].s64 = ctx.r[10].s64 + -21396;
	// 8263A3BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A3C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A3C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A3D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A3D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A3DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A3E0: 4BE2CA41  bl 0x82466e20
	ctx.lr = 0x8263A3E4;
	sub_82466E20(ctx, base);
	// 8263A3E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A3E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A3EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263A3F8 size=24
    let mut pc: u32 = 0x8263A3F8;
    'dispatch: loop {
        match pc {
            0x8263A3F8 => {
    //   block [0x8263A3F8..0x8263A410)
	// 8263A3F8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A3FC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263A400: 394AB3B0  addi r10, r10, -0x4c50
	ctx.r[10].s64 = ctx.r[10].s64 + -19536;
	// 8263A404: 816B5470  lwz r11, 0x5470(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21616 as u32) ) } as u64;
	// 8263A408: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263A40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A410 size=108
    let mut pc: u32 = 0x8263A410;
    'dispatch: loop {
        match pc {
            0x8263A410 => {
    //   block [0x8263A410..0x8263A47C)
	// 8263A410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A41C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263A420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A424: 38EBB3B0  addi r7, r11, -0x4c50
	ctx.r[7].s64 = ctx.r[11].s64 + -19536;
	// 8263A428: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A42C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 8263A430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A434: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A438: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A43C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A440: 386AAC9C  addi r3, r10, -0x5364
	ctx.r[3].s64 = ctx.r[10].s64 + -21348;
	// 8263A444: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A468: 4BE2C9B9  bl 0x82466e20
	ctx.lr = 0x8263A46C;
	sub_82466E20(ctx, base);
	// 8263A46C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A480 size=108
    let mut pc: u32 = 0x8263A480;
    'dispatch: loop {
        match pc {
            0x8263A480 => {
    //   block [0x8263A480..0x8263A4EC)
	// 8263A480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A48C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A494: 38EB5458  addi r7, r11, 0x5458
	ctx.r[7].s64 = ctx.r[11].s64 + 21592;
	// 8263A498: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263A49C: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 8263A4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A4A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A4A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A4B0: 386AACCC  addi r3, r10, -0x5334
	ctx.r[3].s64 = ctx.r[10].s64 + -21300;
	// 8263A4B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A4D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A4D8: 4BE2C949  bl 0x82466e20
	ctx.lr = 0x8263A4DC;
	sub_82466E20(ctx, base);
	// 8263A4DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263A4F0 size=24
    let mut pc: u32 = 0x8263A4F0;
    'dispatch: loop {
        match pc {
            0x8263A4F0 => {
    //   block [0x8263A4F0..0x8263A508)
	// 8263A4F0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A4F4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263A4F8: 394AB3E0  addi r10, r10, -0x4c20
	ctx.r[10].s64 = ctx.r[10].s64 + -19488;
	// 8263A4FC: 816B5470  lwz r11, 0x5470(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21616 as u32) ) } as u64;
	// 8263A500: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263A504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A508 size=108
    let mut pc: u32 = 0x8263A508;
    'dispatch: loop {
        match pc {
            0x8263A508 => {
    //   block [0x8263A508..0x8263A574)
	// 8263A508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A514: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263A518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A51C: 38EBB3E0  addi r7, r11, -0x4c20
	ctx.r[7].s64 = ctx.r[11].s64 + -19488;
	// 8263A520: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A524: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 8263A528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A52C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A538: 386AACFC  addi r3, r10, -0x5304
	ctx.r[3].s64 = ctx.r[10].s64 + -21252;
	// 8263A53C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A55C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A560: 4BE2C8C1  bl 0x82466e20
	ctx.lr = 0x8263A564;
	sub_82466E20(ctx, base);
	// 8263A564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A578 size=112
    let mut pc: u32 = 0x8263A578;
    'dispatch: loop {
        match pc {
            0x8263A578 => {
    //   block [0x8263A578..0x8263A5E8)
	// 8263A578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A584: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263A588: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A58C: 392A658C  addi r9, r10, 0x658c
	ctx.r[9].s64 = ctx.r[10].s64 + 25996;
	// 8263A590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A594: 390B5474  addi r8, r11, 0x5474
	ctx.r[8].s64 = ctx.r[11].s64 + 21620;
	// 8263A598: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263A59C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 8263A5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A5A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263A5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A5B0: 386AAD2C  addi r3, r10, -0x52d4
	ctx.r[3].s64 = ctx.r[10].s64 + -21204;
	// 8263A5B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263A5B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A5CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A5D4: 4BE2C84D  bl 0x82466e20
	ctx.lr = 0x8263A5D8;
	sub_82466E20(ctx, base);
	// 8263A5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A5E8 size=108
    let mut pc: u32 = 0x8263A5E8;
    'dispatch: loop {
        match pc {
            0x8263A5E8 => {
    //   block [0x8263A5E8..0x8263A654)
	// 8263A5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A5F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A5FC: 38EB54A4  addi r7, r11, 0x54a4
	ctx.r[7].s64 = ctx.r[11].s64 + 21668;
	// 8263A600: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A604: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 8263A608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A60C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A618: 386AAD5C  addi r3, r10, -0x52a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21156;
	// 8263A61C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A63C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A640: 4BE2C7E1  bl 0x82466e20
	ctx.lr = 0x8263A644;
	sub_82466E20(ctx, base);
	// 8263A644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A658 size=108
    let mut pc: u32 = 0x8263A658;
    'dispatch: loop {
        match pc {
            0x8263A658 => {
    //   block [0x8263A658..0x8263A6C4)
	// 8263A658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A664: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A66C: 38EB54D4  addi r7, r11, 0x54d4
	ctx.r[7].s64 = ctx.r[11].s64 + 21716;
	// 8263A670: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263A674: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 8263A678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A67C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A688: 386AAD8C  addi r3, r10, -0x5274
	ctx.r[3].s64 = ctx.r[10].s64 + -21108;
	// 8263A68C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A69C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A6AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A6B0: 4BE2C771  bl 0x82466e20
	ctx.lr = 0x8263A6B4;
	sub_82466E20(ctx, base);
	// 8263A6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A6C8 size=108
    let mut pc: u32 = 0x8263A6C8;
    'dispatch: loop {
        match pc {
            0x8263A6C8 => {
    //   block [0x8263A6C8..0x8263A734)
	// 8263A6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A6D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A6DC: 38EB54EC  addi r7, r11, 0x54ec
	ctx.r[7].s64 = ctx.r[11].s64 + 21740;
	// 8263A6E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A6E4: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 8263A6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A6EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A6F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A6F8: 386AADBC  addi r3, r10, -0x5244
	ctx.r[3].s64 = ctx.r[10].s64 + -21060;
	// 8263A6FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A71C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A720: 4BE2C701  bl 0x82466e20
	ctx.lr = 0x8263A724;
	sub_82466E20(ctx, base);
	// 8263A724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A738 size=112
    let mut pc: u32 = 0x8263A738;
    'dispatch: loop {
        match pc {
            0x8263A738 => {
    //   block [0x8263A738..0x8263A7A8)
	// 8263A738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A744: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A748: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A74C: 38AAAE1C  addi r5, r10, -0x51e4
	ctx.r[5].s64 = ctx.r[10].s64 + -20964;
	// 8263A750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A754: 390B551C  addi r8, r11, 0x551c
	ctx.r[8].s64 = ctx.r[11].s64 + 21788;
	// 8263A758: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263A75C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 8263A760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A764: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263A76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A770: 386AADEC  addi r3, r10, -0x5214
	ctx.r[3].s64 = ctx.r[10].s64 + -21012;
	// 8263A774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263A778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A77C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A78C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A794: 4BE2C68D  bl 0x82466e20
	ctx.lr = 0x8263A798;
	sub_82466E20(ctx, base);
	// 8263A798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A79C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A7A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A7A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A7A8 size=108
    let mut pc: u32 = 0x8263A7A8;
    'dispatch: loop {
        match pc {
            0x8263A7A8 => {
    //   block [0x8263A7A8..0x8263A814)
	// 8263A7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A7B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A7B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A7BC: 38EB5534  addi r7, r11, 0x5534
	ctx.r[7].s64 = ctx.r[11].s64 + 21812;
	// 8263A7C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A7C4: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 8263A7C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A7CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A7D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A7D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A7D8: 386AAE1C  addi r3, r10, -0x51e4
	ctx.r[3].s64 = ctx.r[10].s64 + -20964;
	// 8263A7DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A7E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A7E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A7F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A7F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A7FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A800: 4BE2C621  bl 0x82466e20
	ctx.lr = 0x8263A804;
	sub_82466E20(ctx, base);
	// 8263A804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A818 size=108
    let mut pc: u32 = 0x8263A818;
    'dispatch: loop {
        match pc {
            0x8263A818 => {
    //   block [0x8263A818..0x8263A884)
	// 8263A818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A824: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A82C: 38EB5564  addi r7, r11, 0x5564
	ctx.r[7].s64 = ctx.r[11].s64 + 21860;
	// 8263A830: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263A834: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 8263A838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A83C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A848: 386AAE4C  addi r3, r10, -0x51b4
	ctx.r[3].s64 = ctx.r[10].s64 + -20916;
	// 8263A84C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A86C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A870: 4BE2C5B1  bl 0x82466e20
	ctx.lr = 0x8263A874;
	sub_82466E20(ctx, base);
	// 8263A874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A888 size=108
    let mut pc: u32 = 0x8263A888;
    'dispatch: loop {
        match pc {
            0x8263A888 => {
    //   block [0x8263A888..0x8263A8F4)
	// 8263A888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A894: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A89C: 38EB557C  addi r7, r11, 0x557c
	ctx.r[7].s64 = ctx.r[11].s64 + 21884;
	// 8263A8A0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A8A4: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 8263A8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A8AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A8B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A8B8: 386AAE7C  addi r3, r10, -0x5184
	ctx.r[3].s64 = ctx.r[10].s64 + -20868;
	// 8263A8BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A8DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A8E0: 4BE2C541  bl 0x82466e20
	ctx.lr = 0x8263A8E4;
	sub_82466E20(ctx, base);
	// 8263A8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A8F8 size=108
    let mut pc: u32 = 0x8263A8F8;
    'dispatch: loop {
        match pc {
            0x8263A8F8 => {
    //   block [0x8263A8F8..0x8263A964)
	// 8263A8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A904: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A90C: 38EB55B0  addi r7, r11, 0x55b0
	ctx.r[7].s64 = ctx.r[11].s64 + 21936;
	// 8263A910: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8263A914: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 8263A918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A91C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A928: 386AAEAC  addi r3, r10, -0x5154
	ctx.r[3].s64 = ctx.r[10].s64 + -20820;
	// 8263A92C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A94C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A950: 4BE2C4D1  bl 0x82466e20
	ctx.lr = 0x8263A954;
	sub_82466E20(ctx, base);
	// 8263A954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A968 size=108
    let mut pc: u32 = 0x8263A968;
    'dispatch: loop {
        match pc {
            0x8263A968 => {
    //   block [0x8263A968..0x8263A9D4)
	// 8263A968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A974: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A97C: 38EB5658  addi r7, r11, 0x5658
	ctx.r[7].s64 = ctx.r[11].s64 + 22104;
	// 8263A980: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263A984: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 8263A988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A98C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263A990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263A994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263A998: 386AAEDC  addi r3, r10, -0x5124
	ctx.r[3].s64 = ctx.r[10].s64 + -20772;
	// 8263A99C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263A9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263A9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263A9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263A9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263A9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263A9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263A9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263A9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263A9C0: 4BE2C461  bl 0x82466e20
	ctx.lr = 0x8263A9C4;
	sub_82466E20(ctx, base);
	// 8263A9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263A9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263A9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263A9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263A9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263A9D8 size=108
    let mut pc: u32 = 0x8263A9D8;
    'dispatch: loop {
        match pc {
            0x8263A9D8 => {
    //   block [0x8263A9D8..0x8263AA44)
	// 8263A9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263A9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263A9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263A9E4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263A9E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263A9EC: 38EB5688  addi r7, r11, 0x5688
	ctx.r[7].s64 = ctx.r[11].s64 + 22152;
	// 8263A9F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263A9F4: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 8263A9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263A9FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AA00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263AA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AA08: 386AAF0C  addi r3, r10, -0x50f4
	ctx.r[3].s64 = ctx.r[10].s64 + -20724;
	// 8263AA0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263AA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AA14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AA1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AA2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AA30: 4BE2C3F1  bl 0x82466e20
	ctx.lr = 0x8263AA34;
	sub_82466E20(ctx, base);
	// 8263AA34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AA38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AA3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AA48 size=108
    let mut pc: u32 = 0x8263AA48;
    'dispatch: loop {
        match pc {
            0x8263AA48 => {
    //   block [0x8263AA48..0x8263AAB4)
	// 8263AA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AA54: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AA58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AA5C: 38EB56A0  addi r7, r11, 0x56a0
	ctx.r[7].s64 = ctx.r[11].s64 + 22176;
	// 8263AA60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263AA64: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 8263AA68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AA6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AA70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263AA74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AA78: 386AAF3C  addi r3, r10, -0x50c4
	ctx.r[3].s64 = ctx.r[10].s64 + -20676;
	// 8263AA7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263AA80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AA84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AA9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AAA0: 4BE2C381  bl 0x82466e20
	ctx.lr = 0x8263AAA4;
	sub_82466E20(ctx, base);
	// 8263AAA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AAA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AAAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AAB8 size=112
    let mut pc: u32 = 0x8263AAB8;
    'dispatch: loop {
        match pc {
            0x8263AAB8 => {
    //   block [0x8263AAB8..0x8263AB28)
	// 8263AAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AAC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AAC8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AACC: 38AAAD8C  addi r5, r10, -0x5274
	ctx.r[5].s64 = ctx.r[10].s64 + -21108;
	// 8263AAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AAD4: 390B56D0  addi r8, r11, 0x56d0
	ctx.r[8].s64 = ctx.r[11].s64 + 22224;
	// 8263AAD8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8263AADC: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 8263AAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AAE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AAE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263AAEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AAF0: 386AAF6C  addi r3, r10, -0x5094
	ctx.r[3].s64 = ctx.r[10].s64 + -20628;
	// 8263AAF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263AAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AAFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AB04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AB14: 4BE2C30D  bl 0x82466e20
	ctx.lr = 0x8263AB18;
	sub_82466E20(ctx, base);
	// 8263AB18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263AB28 size=24
    let mut pc: u32 = 0x8263AB28;
    'dispatch: loop {
        match pc {
            0x8263AB28 => {
    //   block [0x8263AB28..0x8263AB40)
	// 8263AB28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AB2C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263AB30: 394AB410  addi r10, r10, -0x4bf0
	ctx.r[10].s64 = ctx.r[10].s64 + -19440;
	// 8263AB34: 816B55AC  lwz r11, 0x55ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21932 as u32) ) } as u64;
	// 8263AB38: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263AB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AB40 size=112
    let mut pc: u32 = 0x8263AB40;
    'dispatch: loop {
        match pc {
            0x8263AB40 => {
    //   block [0x8263AB40..0x8263ABB0)
	// 8263AB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AB48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AB4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263AB50: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263AB54: 392A65B8  addi r9, r10, 0x65b8
	ctx.r[9].s64 = ctx.r[10].s64 + 26040;
	// 8263AB58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AB5C: 390BB410  addi r8, r11, -0x4bf0
	ctx.r[8].s64 = ctx.r[11].s64 + -19440;
	// 8263AB60: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263AB64: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 8263AB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AB6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AB70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263AB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AB78: 386AAF9C  addi r3, r10, -0x5064
	ctx.r[3].s64 = ctx.r[10].s64 + -20580;
	// 8263AB7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263AB80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263AB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AB94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AB9C: 4BE2C285  bl 0x82466e20
	ctx.lr = 0x8263ABA0;
	sub_82466E20(ctx, base);
	// 8263ABA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263ABA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263ABA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263ABAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263ABB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263ABB0 size=108
    let mut pc: u32 = 0x8263ABB0;
    'dispatch: loop {
        match pc {
            0x8263ABB0 => {
    //   block [0x8263ABB0..0x8263AC1C)
	// 8263ABB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263ABB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263ABB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263ABBC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263ABC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263ABC4: 38EB577C  addi r7, r11, 0x577c
	ctx.r[7].s64 = ctx.r[11].s64 + 22396;
	// 8263ABC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263ABCC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 8263ABD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263ABD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ABD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263ABDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263ABE0: 386AAFCC  addi r3, r10, -0x5034
	ctx.r[3].s64 = ctx.r[10].s64 + -20532;
	// 8263ABE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263ABE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263ABEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ABF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263ABF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263ABF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263ABFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AC00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AC04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AC08: 4BE2C219  bl 0x82466e20
	ctx.lr = 0x8263AC0C;
	sub_82466E20(ctx, base);
	// 8263AC0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AC10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AC14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AC20 size=116
    let mut pc: u32 = 0x8263AC20;
    'dispatch: loop {
        match pc {
            0x8263AC20 => {
    //   block [0x8263AC20..0x8263AC94)
	// 8263AC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AC2C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AC30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263AC34: 390B57B0  addi r8, r11, 0x57b0
	ctx.r[8].s64 = ctx.r[11].s64 + 22448;
	// 8263AC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AC3C: 392A65FC  addi r9, r10, 0x65fc
	ctx.r[9].s64 = ctx.r[10].s64 + 26108;
	// 8263AC40: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AC44: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8263AC48: 38AAAD8C  addi r5, r10, -0x5274
	ctx.r[5].s64 = ctx.r[10].s64 + -21108;
	// 8263AC4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263AC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AC54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AC5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AC64: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263AC68: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 8263AC6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263AC70: 386BAFFC  addi r3, r11, -0x5004
	ctx.r[3].s64 = ctx.r[11].s64 + -20484;
	// 8263AC74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263AC78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AC80: 4BE2C1A1  bl 0x82466e20
	ctx.lr = 0x8263AC84;
	sub_82466E20(ctx, base);
	// 8263AC84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AC90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263AC98 size=24
    let mut pc: u32 = 0x8263AC98;
    'dispatch: loop {
        match pc {
            0x8263AC98 => {
    //   block [0x8263AC98..0x8263ACB0)
	// 8263AC98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AC9C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263ACA0: 394AB488  addi r10, r10, -0x4b78
	ctx.r[10].s64 = ctx.r[10].s64 + -19320;
	// 8263ACA4: 816B57AC  lwz r11, 0x57ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22444 as u32) ) } as u64;
	// 8263ACA8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263ACAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263ACB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263ACB0 size=112
    let mut pc: u32 = 0x8263ACB0;
    'dispatch: loop {
        match pc {
            0x8263ACB0 => {
    //   block [0x8263ACB0..0x8263AD20)
	// 8263ACB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263ACB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263ACB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263ACBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263ACC0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263ACC4: 392A6638  addi r9, r10, 0x6638
	ctx.r[9].s64 = ctx.r[10].s64 + 26168;
	// 8263ACC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263ACCC: 390BB488  addi r8, r11, -0x4b78
	ctx.r[8].s64 = ctx.r[11].s64 + -19320;
	// 8263ACD0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263ACD4: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 8263ACD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263ACDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ACE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263ACE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263ACE8: 386AB02C  addi r3, r10, -0x4fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -20436;
	// 8263ACEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263ACF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263ACF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ACF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263ACFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AD04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AD0C: 4BE2C115  bl 0x82466e20
	ctx.lr = 0x8263AD10;
	sub_82466E20(ctx, base);
	// 8263AD10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AD20 size=108
    let mut pc: u32 = 0x8263AD20;
    'dispatch: loop {
        match pc {
            0x8263AD20 => {
    //   block [0x8263AD20..0x8263AD8C)
	// 8263AD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AD2C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AD30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AD34: 38EB5870  addi r7, r11, 0x5870
	ctx.r[7].s64 = ctx.r[11].s64 + 22640;
	// 8263AD38: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263AD3C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 8263AD40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AD44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AD48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263AD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AD50: 386AB05C  addi r3, r10, -0x4fa4
	ctx.r[3].s64 = ctx.r[10].s64 + -20388;
	// 8263AD54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263AD58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AD5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AD60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AD64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AD68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AD6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AD70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AD74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AD78: 4BE2C0A9  bl 0x82466e20
	ctx.lr = 0x8263AD7C;
	sub_82466E20(ctx, base);
	// 8263AD7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AD80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AD84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AD88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AD90 size=108
    let mut pc: u32 = 0x8263AD90;
    'dispatch: loop {
        match pc {
            0x8263AD90 => {
    //   block [0x8263AD90..0x8263ADFC)
	// 8263AD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AD9C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263ADA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263ADA4: 38EB5888  addi r7, r11, 0x5888
	ctx.r[7].s64 = ctx.r[11].s64 + 22664;
	// 8263ADA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263ADAC: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 8263ADB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263ADB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ADB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263ADBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263ADC0: 386AB08C  addi r3, r10, -0x4f74
	ctx.r[3].s64 = ctx.r[10].s64 + -20340;
	// 8263ADC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263ADC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263ADCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ADD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263ADD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263ADD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263ADDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263ADE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263ADE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263ADE8: 4BE2C039  bl 0x82466e20
	ctx.lr = 0x8263ADEC;
	sub_82466E20(ctx, base);
	// 8263ADEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263ADF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263ADF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263ADF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263AE00 size=24
    let mut pc: u32 = 0x8263AE00;
    'dispatch: loop {
        match pc {
            0x8263AE00 => {
    //   block [0x8263AE00..0x8263AE18)
	// 8263AE00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AE04: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263AE08: 394AB4D0  addi r10, r10, -0x4b30
	ctx.r[10].s64 = ctx.r[10].s64 + -19248;
	// 8263AE0C: 816B58B8  lwz r11, 0x58b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22712 as u32) ) } as u64;
	// 8263AE10: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263AE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AE18 size=112
    let mut pc: u32 = 0x8263AE18;
    'dispatch: loop {
        match pc {
            0x8263AE18 => {
    //   block [0x8263AE18..0x8263AE88)
	// 8263AE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AE24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263AE28: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263AE2C: 392A6674  addi r9, r10, 0x6674
	ctx.r[9].s64 = ctx.r[10].s64 + 26228;
	// 8263AE30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AE34: 390BB4D0  addi r8, r11, -0x4b30
	ctx.r[8].s64 = ctx.r[11].s64 + -19248;
	// 8263AE38: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263AE3C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 8263AE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AE44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AE48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263AE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AE50: 386AB0BC  addi r3, r10, -0x4f44
	ctx.r[3].s64 = ctx.r[10].s64 + -20292;
	// 8263AE54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263AE58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263AE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AE60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AE68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AE70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AE74: 4BE2BFAD  bl 0x82466e20
	ctx.lr = 0x8263AE78;
	sub_82466E20(ctx, base);
	// 8263AE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AE88 size=112
    let mut pc: u32 = 0x8263AE88;
    'dispatch: loop {
        match pc {
            0x8263AE88 => {
    //   block [0x8263AE88..0x8263AEF8)
	// 8263AE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AE94: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AE98: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AE9C: 38AAAD8C  addi r5, r10, -0x5274
	ctx.r[5].s64 = ctx.r[10].s64 + -21108;
	// 8263AEA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AEA4: 390B58BC  addi r8, r11, 0x58bc
	ctx.r[8].s64 = ctx.r[11].s64 + 22716;
	// 8263AEA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263AEAC: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 8263AEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AEB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AEB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263AEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AEC0: 386AB0EC  addi r3, r10, -0x4f14
	ctx.r[3].s64 = ctx.r[10].s64 + -20244;
	// 8263AEC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263AEC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AEDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AEE4: 4BE2BF3D  bl 0x82466e20
	ctx.lr = 0x8263AEE8;
	sub_82466E20(ctx, base);
	// 8263AEE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AEF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AEF8 size=108
    let mut pc: u32 = 0x8263AEF8;
    'dispatch: loop {
        match pc {
            0x8263AEF8 => {
    //   block [0x8263AEF8..0x8263AF64)
	// 8263AEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AF04: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AF0C: 38EB58EC  addi r7, r11, 0x58ec
	ctx.r[7].s64 = ctx.r[11].s64 + 22764;
	// 8263AF10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263AF14: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 8263AF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AF1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AF20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263AF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AF28: 386AB11C  addi r3, r10, -0x4ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -20196;
	// 8263AF2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263AF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AF50: 4BE2BED1  bl 0x82466e20
	ctx.lr = 0x8263AF54;
	sub_82466E20(ctx, base);
	// 8263AF54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AF68 size=108
    let mut pc: u32 = 0x8263AF68;
    'dispatch: loop {
        match pc {
            0x8263AF68 => {
    //   block [0x8263AF68..0x8263AFD4)
	// 8263AF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AF74: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AF78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AF7C: 38EB5920  addi r7, r11, 0x5920
	ctx.r[7].s64 = ctx.r[11].s64 + 22816;
	// 8263AF80: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263AF84: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 8263AF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AF8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263AF90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263AF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263AF98: 386AB14C  addi r3, r10, -0x4eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -20148;
	// 8263AF9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263AFA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263AFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263AFA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263AFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263AFB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263AFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263AFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263AFBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263AFC0: 4BE2BE61  bl 0x82466e20
	ctx.lr = 0x8263AFC4;
	sub_82466E20(ctx, base);
	// 8263AFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263AFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263AFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263AFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263AFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263AFD8 size=108
    let mut pc: u32 = 0x8263AFD8;
    'dispatch: loop {
        match pc {
            0x8263AFD8 => {
    //   block [0x8263AFD8..0x8263B044)
	// 8263AFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263AFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263AFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263AFE4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263AFE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263AFEC: 38EB5980  addi r7, r11, 0x5980
	ctx.r[7].s64 = ctx.r[11].s64 + 22912;
	// 8263AFF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263AFF4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 8263AFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263AFFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B008: 386AB17C  addi r3, r10, -0x4e84
	ctx.r[3].s64 = ctx.r[10].s64 + -20100;
	// 8263B00C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B02C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B030: 4BE2BDF1  bl 0x82466e20
	ctx.lr = 0x8263B034;
	sub_82466E20(ctx, base);
	// 8263B034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B048 size=108
    let mut pc: u32 = 0x8263B048;
    'dispatch: loop {
        match pc {
            0x8263B048 => {
    //   block [0x8263B048..0x8263B0B4)
	// 8263B048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B054: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B05C: 38EB59B0  addi r7, r11, 0x59b0
	ctx.r[7].s64 = ctx.r[11].s64 + 22960;
	// 8263B060: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8263B064: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 8263B068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B06C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B078: 386AB1AC  addi r3, r10, -0x4e54
	ctx.r[3].s64 = ctx.r[10].s64 + -20052;
	// 8263B07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B0A0: 4BE2BD81  bl 0x82466e20
	ctx.lr = 0x8263B0A4;
	sub_82466E20(ctx, base);
	// 8263B0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B0B8 size=108
    let mut pc: u32 = 0x8263B0B8;
    'dispatch: loop {
        match pc {
            0x8263B0B8 => {
    //   block [0x8263B0B8..0x8263B124)
	// 8263B0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B0C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B0C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B0CC: 38EB5AD0  addi r7, r11, 0x5ad0
	ctx.r[7].s64 = ctx.r[11].s64 + 23248;
	// 8263B0D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B0D4: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 8263B0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B0DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B0E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B0E8: 386AB1DC  addi r3, r10, -0x4e24
	ctx.r[3].s64 = ctx.r[10].s64 + -20004;
	// 8263B0EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B0F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B104: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B10C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B110: 4BE2BD11  bl 0x82466e20
	ctx.lr = 0x8263B114;
	sub_82466E20(ctx, base);
	// 8263B114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B128 size=108
    let mut pc: u32 = 0x8263B128;
    'dispatch: loop {
        match pc {
            0x8263B128 => {
    //   block [0x8263B128..0x8263B194)
	// 8263B128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B134: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B13C: 38EB5AE8  addi r7, r11, 0x5ae8
	ctx.r[7].s64 = ctx.r[11].s64 + 23272;
	// 8263B140: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B144: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 8263B148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B14C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B158: 386AB20C  addi r3, r10, -0x4df4
	ctx.r[3].s64 = ctx.r[10].s64 + -19956;
	// 8263B15C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B180: 4BE2BCA1  bl 0x82466e20
	ctx.lr = 0x8263B184;
	sub_82466E20(ctx, base);
	// 8263B184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B198 size=108
    let mut pc: u32 = 0x8263B198;
    'dispatch: loop {
        match pc {
            0x8263B198 => {
    //   block [0x8263B198..0x8263B204)
	// 8263B198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B1A4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B1A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B1AC: 38EB5B00  addi r7, r11, 0x5b00
	ctx.r[7].s64 = ctx.r[11].s64 + 23296;
	// 8263B1B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B1B4: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 8263B1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B1BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B1C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B1C8: 386AB23C  addi r3, r10, -0x4dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -19908;
	// 8263B1CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B1D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B1D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B1D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B1E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B1E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B1EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B1F0: 4BE2BC31  bl 0x82466e20
	ctx.lr = 0x8263B1F4;
	sub_82466E20(ctx, base);
	// 8263B1F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B208 size=108
    let mut pc: u32 = 0x8263B208;
    'dispatch: loop {
        match pc {
            0x8263B208 => {
    //   block [0x8263B208..0x8263B274)
	// 8263B208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B214: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B21C: 38EB5B18  addi r7, r11, 0x5b18
	ctx.r[7].s64 = ctx.r[11].s64 + 23320;
	// 8263B220: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B224: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 8263B228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B22C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B230: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B238: 386AB26C  addi r3, r10, -0x4d94
	ctx.r[3].s64 = ctx.r[10].s64 + -19860;
	// 8263B23C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B25C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B260: 4BE2BBC1  bl 0x82466e20
	ctx.lr = 0x8263B264;
	sub_82466E20(ctx, base);
	// 8263B264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B278 size=108
    let mut pc: u32 = 0x8263B278;
    'dispatch: loop {
        match pc {
            0x8263B278 => {
    //   block [0x8263B278..0x8263B2E4)
	// 8263B278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B284: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B28C: 38EB5B30  addi r7, r11, 0x5b30
	ctx.r[7].s64 = ctx.r[11].s64 + 23344;
	// 8263B290: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B294: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 8263B298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B29C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B2A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B2A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B2A8: 386AB29C  addi r3, r10, -0x4d64
	ctx.r[3].s64 = ctx.r[10].s64 + -19812;
	// 8263B2AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B2B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B2B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B2B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B2C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B2C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B2C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B2CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B2D0: 4BE2BB51  bl 0x82466e20
	ctx.lr = 0x8263B2D4;
	sub_82466E20(ctx, base);
	// 8263B2D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B2E8 size=108
    let mut pc: u32 = 0x8263B2E8;
    'dispatch: loop {
        match pc {
            0x8263B2E8 => {
    //   block [0x8263B2E8..0x8263B354)
	// 8263B2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B2F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B2F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B2FC: 38EB5B48  addi r7, r11, 0x5b48
	ctx.r[7].s64 = ctx.r[11].s64 + 23368;
	// 8263B300: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B304: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 8263B308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B30C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B310: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B318: 386AB2CC  addi r3, r10, -0x4d34
	ctx.r[3].s64 = ctx.r[10].s64 + -19764;
	// 8263B31C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B33C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B340: 4BE2BAE1  bl 0x82466e20
	ctx.lr = 0x8263B344;
	sub_82466E20(ctx, base);
	// 8263B344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B358 size=108
    let mut pc: u32 = 0x8263B358;
    'dispatch: loop {
        match pc {
            0x8263B358 => {
    //   block [0x8263B358..0x8263B3C4)
	// 8263B358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B364: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B36C: 38EB5B60  addi r7, r11, 0x5b60
	ctx.r[7].s64 = ctx.r[11].s64 + 23392;
	// 8263B370: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8263B374: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8263B378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B37C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B388: 386AB2FC  addi r3, r10, -0x4d04
	ctx.r[3].s64 = ctx.r[10].s64 + -19716;
	// 8263B38C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B3A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B3A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B3A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B3AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B3B0: 4BE2BA71  bl 0x82466e20
	ctx.lr = 0x8263B3B4;
	sub_82466E20(ctx, base);
	// 8263B3B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B3B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B3BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B3C8 size=108
    let mut pc: u32 = 0x8263B3C8;
    'dispatch: loop {
        match pc {
            0x8263B3C8 => {
    //   block [0x8263B3C8..0x8263B434)
	// 8263B3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B3D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B3DC: 38EB5BF0  addi r7, r11, 0x5bf0
	ctx.r[7].s64 = ctx.r[11].s64 + 23536;
	// 8263B3E0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8263B3E4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8263B3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B3EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B3F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B3F8: 386AB32C  addi r3, r10, -0x4cd4
	ctx.r[3].s64 = ctx.r[10].s64 + -19668;
	// 8263B3FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B41C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B420: 4BE2BA01  bl 0x82466e20
	ctx.lr = 0x8263B424;
	sub_82466E20(ctx, base);
	// 8263B424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B438 size=108
    let mut pc: u32 = 0x8263B438;
    'dispatch: loop {
        match pc {
            0x8263B438 => {
    //   block [0x8263B438..0x8263B4A4)
	// 8263B438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B444: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B448: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B44C: 38EB5CB0  addi r7, r11, 0x5cb0
	ctx.r[7].s64 = ctx.r[11].s64 + 23728;
	// 8263B450: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8263B454: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8263B458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B45C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B460: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B468: 386AB35C  addi r3, r10, -0x4ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -19620;
	// 8263B46C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B470: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B478: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B480: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B488: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B48C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B490: 4BE2B991  bl 0x82466e20
	ctx.lr = 0x8263B494;
	sub_82466E20(ctx, base);
	// 8263B494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B49C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B4A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B4A8 size=108
    let mut pc: u32 = 0x8263B4A8;
    'dispatch: loop {
        match pc {
            0x8263B4A8 => {
    //   block [0x8263B4A8..0x8263B514)
	// 8263B4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B4B4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B4B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B4BC: 38EB5D88  addi r7, r11, 0x5d88
	ctx.r[7].s64 = ctx.r[11].s64 + 23944;
	// 8263B4C0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8263B4C4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 8263B4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B4CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B4D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B4D8: 386AB38C  addi r3, r10, -0x4c74
	ctx.r[3].s64 = ctx.r[10].s64 + -19572;
	// 8263B4DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B4FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B500: 4BE2B921  bl 0x82466e20
	ctx.lr = 0x8263B504;
	sub_82466E20(ctx, base);
	// 8263B504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B518 size=108
    let mut pc: u32 = 0x8263B518;
    'dispatch: loop {
        match pc {
            0x8263B518 => {
    //   block [0x8263B518..0x8263B584)
	// 8263B518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B524: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B52C: 38EB5E48  addi r7, r11, 0x5e48
	ctx.r[7].s64 = ctx.r[11].s64 + 24136;
	// 8263B530: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8263B534: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 8263B538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B53C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B540: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B548: 386AB3BC  addi r3, r10, -0x4c44
	ctx.r[3].s64 = ctx.r[10].s64 + -19524;
	// 8263B54C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B55C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B56C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B570: 4BE2B8B1  bl 0x82466e20
	ctx.lr = 0x8263B574;
	sub_82466E20(ctx, base);
	// 8263B574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B588 size=112
    let mut pc: u32 = 0x8263B588;
    'dispatch: loop {
        match pc {
            0x8263B588 => {
    //   block [0x8263B588..0x8263B5F8)
	// 8263B588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B594: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8263B598: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8263B59C: 38EA5EF0  addi r7, r10, 0x5ef0
	ctx.r[7].s64 = ctx.r[10].s64 + 24304;
	// 8263B5A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B5A4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263B5A8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8263B5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B5B0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B5B4: 396B6688  addi r11, r11, 0x6688
	ctx.r[11].s64 = ctx.r[11].s64 + 26248;
	// 8263B5B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B5BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B5C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B5C4: 386AB3EC  addi r3, r10, -0x4c14
	ctx.r[3].s64 = ctx.r[10].s64 + -19476;
	// 8263B5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B5CC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263B5D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B5D4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263B5D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B5DC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B5E0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B5E4: 4BE2B83D  bl 0x82466e20
	ctx.lr = 0x8263B5E8;
	sub_82466E20(ctx, base);
	// 8263B5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B5F8 size=108
    let mut pc: u32 = 0x8263B5F8;
    'dispatch: loop {
        match pc {
            0x8263B5F8 => {
    //   block [0x8263B5F8..0x8263B664)
	// 8263B5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B604: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B608: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B60C: 38EB6010  addi r7, r11, 0x6010
	ctx.r[7].s64 = ctx.r[11].s64 + 24592;
	// 8263B610: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263B614: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 8263B618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B61C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B620: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B628: 386AB41C  addi r3, r10, -0x4be4
	ctx.r[3].s64 = ctx.r[10].s64 + -19428;
	// 8263B62C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B630: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B64C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B650: 4BE2B7D1  bl 0x82466e20
	ctx.lr = 0x8263B654;
	sub_82466E20(ctx, base);
	// 8263B654: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B658: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B65C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B668 size=108
    let mut pc: u32 = 0x8263B668;
    'dispatch: loop {
        match pc {
            0x8263B668 => {
    //   block [0x8263B668..0x8263B6D4)
	// 8263B668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B674: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B67C: 38EB6070  addi r7, r11, 0x6070
	ctx.r[7].s64 = ctx.r[11].s64 + 24688;
	// 8263B680: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8263B684: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 8263B688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B68C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B690: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B698: 386AB44C  addi r3, r10, -0x4bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -19380;
	// 8263B69C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B6A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B6A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B6B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B6B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B6B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B6BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B6C0: 4BE2B761  bl 0x82466e20
	ctx.lr = 0x8263B6C4;
	sub_82466E20(ctx, base);
	// 8263B6C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B6D8 size=108
    let mut pc: u32 = 0x8263B6D8;
    'dispatch: loop {
        match pc {
            0x8263B6D8 => {
    //   block [0x8263B6D8..0x8263B744)
	// 8263B6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B6E4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B6E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B6EC: 38EB6178  addi r7, r11, 0x6178
	ctx.r[7].s64 = ctx.r[11].s64 + 24952;
	// 8263B6F0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8263B6F4: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 8263B6F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B6FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B700: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B708: 386AB47C  addi r3, r10, -0x4b84
	ctx.r[3].s64 = ctx.r[10].s64 + -19332;
	// 8263B70C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B71C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B72C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B730: 4BE2B6F1  bl 0x82466e20
	ctx.lr = 0x8263B734;
	sub_82466E20(ctx, base);
	// 8263B734: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B73C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B740: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B748 size=108
    let mut pc: u32 = 0x8263B748;
    'dispatch: loop {
        match pc {
            0x8263B748 => {
    //   block [0x8263B748..0x8263B7B4)
	// 8263B748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B754: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B75C: 38EB6250  addi r7, r11, 0x6250
	ctx.r[7].s64 = ctx.r[11].s64 + 25168;
	// 8263B760: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263B764: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 8263B768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B76C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B778: 386AB4AC  addi r3, r10, -0x4b54
	ctx.r[3].s64 = ctx.r[10].s64 + -19284;
	// 8263B77C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B79C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B7A0: 4BE2B681  bl 0x82466e20
	ctx.lr = 0x8263B7A4;
	sub_82466E20(ctx, base);
	// 8263B7A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B7B8 size=108
    let mut pc: u32 = 0x8263B7B8;
    'dispatch: loop {
        match pc {
            0x8263B7B8 => {
    //   block [0x8263B7B8..0x8263B824)
	// 8263B7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B7C4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B7C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B7CC: 38EB6298  addi r7, r11, 0x6298
	ctx.r[7].s64 = ctx.r[11].s64 + 25240;
	// 8263B7D0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B7D4: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 8263B7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B7DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B7E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B7E8: 386AB4DC  addi r3, r10, -0x4b24
	ctx.r[3].s64 = ctx.r[10].s64 + -19236;
	// 8263B7EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B7F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B7F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B810: 4BE2B611  bl 0x82466e20
	ctx.lr = 0x8263B814;
	sub_82466E20(ctx, base);
	// 8263B814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B828 size=112
    let mut pc: u32 = 0x8263B828;
    'dispatch: loop {
        match pc {
            0x8263B828 => {
    //   block [0x8263B828..0x8263B898)
	// 8263B828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B834: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B838: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B83C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263B840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B844: 390B62B0  addi r8, r11, 0x62b0
	ctx.r[8].s64 = ctx.r[11].s64 + 25264;
	// 8263B848: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263B84C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 8263B850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B854: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263B85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B860: 386AB50C  addi r3, r10, -0x4af4
	ctx.r[3].s64 = ctx.r[10].s64 + -19188;
	// 8263B864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263B868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B87C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B884: 4BE2B59D  bl 0x82466e20
	ctx.lr = 0x8263B888;
	sub_82466E20(ctx, base);
	// 8263B888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B898 size=112
    let mut pc: u32 = 0x8263B898;
    'dispatch: loop {
        match pc {
            0x8263B898 => {
    //   block [0x8263B898..0x8263B908)
	// 8263B898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B8A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B8A8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B8AC: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263B8B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B8B4: 390B62F8  addi r8, r11, 0x62f8
	ctx.r[8].s64 = ctx.r[11].s64 + 25336;
	// 8263B8B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263B8BC: 388A3824  addi r4, r10, 0x3824
	ctx.r[4].s64 = ctx.r[10].s64 + 14372;
	// 8263B8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B8C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263B8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B8D0: 386AB53C  addi r3, r10, -0x4ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -19140;
	// 8263B8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263B8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B8F4: 4BE2B52D  bl 0x82466e20
	ctx.lr = 0x8263B8F8;
	sub_82466E20(ctx, base);
	// 8263B8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B908 size=108
    let mut pc: u32 = 0x8263B908;
    'dispatch: loop {
        match pc {
            0x8263B908 => {
    //   block [0x8263B908..0x8263B974)
	// 8263B908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B914: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B91C: 38EB6340  addi r7, r11, 0x6340
	ctx.r[7].s64 = ctx.r[11].s64 + 25408;
	// 8263B920: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263B924: 388A3838  addi r4, r10, 0x3838
	ctx.r[4].s64 = ctx.r[10].s64 + 14392;
	// 8263B928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B92C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263B934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B938: 386AB56C  addi r3, r10, -0x4a94
	ctx.r[3].s64 = ctx.r[10].s64 + -19092;
	// 8263B93C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263B940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B95C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263B960: 4BE2B4C1  bl 0x82466e20
	ctx.lr = 0x8263B964;
	sub_82466E20(ctx, base);
	// 8263B964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263B978 size=24
    let mut pc: u32 = 0x8263B978;
    'dispatch: loop {
        match pc {
            0x8263B978 => {
    //   block [0x8263B978..0x8263B990)
	// 8263B978: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263B97C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263B980: 394AB548  addi r10, r10, -0x4ab8
	ctx.r[10].s64 = ctx.r[10].s64 + -19128;
	// 8263B984: 816B591C  lwz r11, 0x591c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22812 as u32) ) } as u64;
	// 8263B988: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8263B98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263B990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263B990 size=112
    let mut pc: u32 = 0x8263B990;
    'dispatch: loop {
        match pc {
            0x8263B990 => {
    //   block [0x8263B990..0x8263BA00)
	// 8263B990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263B994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263B998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263B99C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B9A0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263B9A4: 38AAB77C  addi r5, r10, -0x4884
	ctx.r[5].s64 = ctx.r[10].s64 + -18564;
	// 8263B9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263B9AC: 390BB548  addi r8, r11, -0x4ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -19128;
	// 8263B9B0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8263B9B4: 388A3888  addi r4, r10, 0x3888
	ctx.r[4].s64 = ctx.r[10].s64 + 14472;
	// 8263B9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263B9BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263B9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263B9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263B9C8: 386AB59C  addi r3, r10, -0x4a64
	ctx.r[3].s64 = ctx.r[10].s64 + -19044;
	// 8263B9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263B9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263B9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263B9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263B9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263B9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263B9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263B9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263B9EC: 4BE2B435  bl 0x82466e20
	ctx.lr = 0x8263B9F0;
	sub_82466E20(ctx, base);
	// 8263B9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263B9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263B9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263B9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BA00 size=108
    let mut pc: u32 = 0x8263BA00;
    'dispatch: loop {
        match pc {
            0x8263BA00 => {
    //   block [0x8263BA00..0x8263BA6C)
	// 8263BA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BA0C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BA10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BA14: 38EB6358  addi r7, r11, 0x6358
	ctx.r[7].s64 = ctx.r[11].s64 + 25432;
	// 8263BA18: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263BA1C: 388A38A0  addi r4, r10, 0x38a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14496;
	// 8263BA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BA24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BA28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BA30: 386AB5CC  addi r3, r10, -0x4a34
	ctx.r[3].s64 = ctx.r[10].s64 + -18996;
	// 8263BA34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BA3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BA4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BA54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BA58: 4BE2B3C9  bl 0x82466e20
	ctx.lr = 0x8263BA5C;
	sub_82466E20(ctx, base);
	// 8263BA5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BA70 size=112
    let mut pc: u32 = 0x8263BA70;
    'dispatch: loop {
        match pc {
            0x8263BA70 => {
    //   block [0x8263BA70..0x8263BAE0)
	// 8263BA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BA7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BA80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BA84: 38AAB77C  addi r5, r10, -0x4884
	ctx.r[5].s64 = ctx.r[10].s64 + -18564;
	// 8263BA88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BA8C: 390B63B8  addi r8, r11, 0x63b8
	ctx.r[8].s64 = ctx.r[11].s64 + 25528;
	// 8263BA90: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8263BA94: 388A38BC  addi r4, r10, 0x38bc
	ctx.r[4].s64 = ctx.r[10].s64 + 14524;
	// 8263BA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BA9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BAA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263BAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BAA8: 386AB5FC  addi r3, r10, -0x4a04
	ctx.r[3].s64 = ctx.r[10].s64 + -18948;
	// 8263BAAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263BAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BACC: 4BE2B355  bl 0x82466e20
	ctx.lr = 0x8263BAD0;
	sub_82466E20(ctx, base);
	// 8263BAD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BAE0 size=108
    let mut pc: u32 = 0x8263BAE0;
    'dispatch: loop {
        match pc {
            0x8263BAE0 => {
    //   block [0x8263BAE0..0x8263BB4C)
	// 8263BAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BAEC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BAF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BAF4: 38EB6478  addi r7, r11, 0x6478
	ctx.r[7].s64 = ctx.r[11].s64 + 25720;
	// 8263BAF8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263BAFC: 388A38D0  addi r4, r10, 0x38d0
	ctx.r[4].s64 = ctx.r[10].s64 + 14544;
	// 8263BB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BB04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BB08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BB10: 386AB62C  addi r3, r10, -0x49d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18900;
	// 8263BB14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BB34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BB38: 4BE2B2E9  bl 0x82466e20
	ctx.lr = 0x8263BB3C;
	sub_82466E20(ctx, base);
	// 8263BB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BB50 size=108
    let mut pc: u32 = 0x8263BB50;
    'dispatch: loop {
        match pc {
            0x8263BB50 => {
    //   block [0x8263BB50..0x8263BBBC)
	// 8263BB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BB5C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BB60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BB64: 38EB64D8  addi r7, r11, 0x64d8
	ctx.r[7].s64 = ctx.r[11].s64 + 25816;
	// 8263BB68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263BB6C: 388A38E0  addi r4, r10, 0x38e0
	ctx.r[4].s64 = ctx.r[10].s64 + 14560;
	// 8263BB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BB74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BB78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BB7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BB80: 386AB65C  addi r3, r10, -0x49a4
	ctx.r[3].s64 = ctx.r[10].s64 + -18852;
	// 8263BB84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BB88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BB8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BB90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BB98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BBA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BBA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BBA8: 4BE2B279  bl 0x82466e20
	ctx.lr = 0x8263BBAC;
	sub_82466E20(ctx, base);
	// 8263BBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BBC0 size=108
    let mut pc: u32 = 0x8263BBC0;
    'dispatch: loop {
        match pc {
            0x8263BBC0 => {
    //   block [0x8263BBC0..0x8263BC2C)
	// 8263BBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BBCC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BBD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BBD4: 38EB6520  addi r7, r11, 0x6520
	ctx.r[7].s64 = ctx.r[11].s64 + 25888;
	// 8263BBD8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263BBDC: 388A3904  addi r4, r10, 0x3904
	ctx.r[4].s64 = ctx.r[10].s64 + 14596;
	// 8263BBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BBE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BBE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BBEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BBF0: 386AB68C  addi r3, r10, -0x4974
	ctx.r[3].s64 = ctx.r[10].s64 + -18804;
	// 8263BBF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BBF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BBFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BC14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BC18: 4BE2B209  bl 0x82466e20
	ctx.lr = 0x8263BC1C;
	sub_82466E20(ctx, base);
	// 8263BC1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BC20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BC24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BC30 size=112
    let mut pc: u32 = 0x8263BC30;
    'dispatch: loop {
        match pc {
            0x8263BC30 => {
    //   block [0x8263BC30..0x8263BCA0)
	// 8263BC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BC3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BC40: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BC44: 38AAB68C  addi r5, r10, -0x4974
	ctx.r[5].s64 = ctx.r[10].s64 + -18804;
	// 8263BC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BC4C: 390B6568  addi r8, r11, 0x6568
	ctx.r[8].s64 = ctx.r[11].s64 + 25960;
	// 8263BC50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263BC54: 388A3928  addi r4, r10, 0x3928
	ctx.r[4].s64 = ctx.r[10].s64 + 14632;
	// 8263BC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BC5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BC60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263BC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BC68: 386AB6BC  addi r3, r10, -0x4944
	ctx.r[3].s64 = ctx.r[10].s64 + -18756;
	// 8263BC6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263BC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BC8C: 4BE2B195  bl 0x82466e20
	ctx.lr = 0x8263BC90;
	sub_82466E20(ctx, base);
	// 8263BC90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BCA0 size=108
    let mut pc: u32 = 0x8263BCA0;
    'dispatch: loop {
        match pc {
            0x8263BCA0 => {
    //   block [0x8263BCA0..0x8263BD0C)
	// 8263BCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BCAC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BCB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BCB4: 38EB6598  addi r7, r11, 0x6598
	ctx.r[7].s64 = ctx.r[11].s64 + 26008;
	// 8263BCB8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263BCBC: 388A394C  addi r4, r10, 0x394c
	ctx.r[4].s64 = ctx.r[10].s64 + 14668;
	// 8263BCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BCC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BCC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BCCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BCD0: 386AB6EC  addi r3, r10, -0x4914
	ctx.r[3].s64 = ctx.r[10].s64 + -18708;
	// 8263BCD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BCD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BCE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BCE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BCE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BCEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BCF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BCF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BCF8: 4BE2B129  bl 0x82466e20
	ctx.lr = 0x8263BCFC;
	sub_82466E20(ctx, base);
	// 8263BCFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BD00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BD04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BD08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BD10 size=108
    let mut pc: u32 = 0x8263BD10;
    'dispatch: loop {
        match pc {
            0x8263BD10 => {
    //   block [0x8263BD10..0x8263BD7C)
	// 8263BD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BD1C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BD20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BD24: 38EB65E0  addi r7, r11, 0x65e0
	ctx.r[7].s64 = ctx.r[11].s64 + 26080;
	// 8263BD28: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8263BD2C: 388A3968  addi r4, r10, 0x3968
	ctx.r[4].s64 = ctx.r[10].s64 + 14696;
	// 8263BD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BD34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BD38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BD3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BD40: 386AB71C  addi r3, r10, -0x48e4
	ctx.r[3].s64 = ctx.r[10].s64 + -18660;
	// 8263BD44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BD48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BD64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BD68: 4BE2B0B9  bl 0x82466e20
	ctx.lr = 0x8263BD6C;
	sub_82466E20(ctx, base);
	// 8263BD6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BD70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BD74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BD80 size=108
    let mut pc: u32 = 0x8263BD80;
    'dispatch: loop {
        match pc {
            0x8263BD80 => {
    //   block [0x8263BD80..0x8263BDEC)
	// 8263BD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BD8C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BD94: 38EB66A0  addi r7, r11, 0x66a0
	ctx.r[7].s64 = ctx.r[11].s64 + 26272;
	// 8263BD98: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 8263BD9C: 388A3980  addi r4, r10, 0x3980
	ctx.r[4].s64 = ctx.r[10].s64 + 14720;
	// 8263BDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BDA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BDA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BDB0: 386AB74C  addi r3, r10, -0x48b4
	ctx.r[3].s64 = ctx.r[10].s64 + -18612;
	// 8263BDB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BDC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BDD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BDD8: 4BE2B049  bl 0x82466e20
	ctx.lr = 0x8263BDDC;
	sub_82466E20(ctx, base);
	// 8263BDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BDF0 size=112
    let mut pc: u32 = 0x8263BDF0;
    'dispatch: loop {
        match pc {
            0x8263BDF0 => {
    //   block [0x8263BDF0..0x8263BE60)
	// 8263BDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BDFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BE00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BE04: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263BE08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BE0C: 390B6820  addi r8, r11, 0x6820
	ctx.r[8].s64 = ctx.r[11].s64 + 26656;
	// 8263BE10: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263BE14: 388A3990  addi r4, r10, 0x3990
	ctx.r[4].s64 = ctx.r[10].s64 + 14736;
	// 8263BE18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BE1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BE20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263BE24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BE28: 386AB77C  addi r3, r10, -0x4884
	ctx.r[3].s64 = ctx.r[10].s64 + -18564;
	// 8263BE2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263BE30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BE44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BE4C: 4BE2AFD5  bl 0x82466e20
	ctx.lr = 0x8263BE50;
	sub_82466E20(ctx, base);
	// 8263BE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BE60 size=108
    let mut pc: u32 = 0x8263BE60;
    'dispatch: loop {
        match pc {
            0x8263BE60 => {
    //   block [0x8263BE60..0x8263BECC)
	// 8263BE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BE6C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BE74: 38EB6880  addi r7, r11, 0x6880
	ctx.r[7].s64 = ctx.r[11].s64 + 26752;
	// 8263BE78: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8263BE7C: 388A39A0  addi r4, r10, 0x39a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14752;
	// 8263BE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BE84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BE88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BE90: 386AB7AC  addi r3, r10, -0x4854
	ctx.r[3].s64 = ctx.r[10].s64 + -18516;
	// 8263BE94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BE9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BEA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BEB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BEB8: 4BE2AF69  bl 0x82466e20
	ctx.lr = 0x8263BEBC;
	sub_82466E20(ctx, base);
	// 8263BEBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BEC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BEC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BEC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BED0 size=112
    let mut pc: u32 = 0x8263BED0;
    'dispatch: loop {
        match pc {
            0x8263BED0 => {
    //   block [0x8263BED0..0x8263BF40)
	// 8263BED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BEDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BEE0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BEE4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263BEE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BEEC: 390B68F8  addi r8, r11, 0x68f8
	ctx.r[8].s64 = ctx.r[11].s64 + 26872;
	// 8263BEF0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263BEF4: 388A39B4  addi r4, r10, 0x39b4
	ctx.r[4].s64 = ctx.r[10].s64 + 14772;
	// 8263BEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BEFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BF00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263BF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BF08: 386AB7DC  addi r3, r10, -0x4824
	ctx.r[3].s64 = ctx.r[10].s64 + -18468;
	// 8263BF0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263BF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BF18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BF20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BF28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BF2C: 4BE2AEF5  bl 0x82466e20
	ctx.lr = 0x8263BF30;
	sub_82466E20(ctx, base);
	// 8263BF30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BF40 size=108
    let mut pc: u32 = 0x8263BF40;
    'dispatch: loop {
        match pc {
            0x8263BF40 => {
    //   block [0x8263BF40..0x8263BFAC)
	// 8263BF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BF4C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BF54: 38EB6940  addi r7, r11, 0x6940
	ctx.r[7].s64 = ctx.r[11].s64 + 26944;
	// 8263BF58: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263BF5C: 388A39C0  addi r4, r10, 0x39c0
	ctx.r[4].s64 = ctx.r[10].s64 + 14784;
	// 8263BF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BF64: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BF68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BF70: 386AB80C  addi r3, r10, -0x47f4
	ctx.r[3].s64 = ctx.r[10].s64 + -18420;
	// 8263BF74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263BF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263BF94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263BF98: 4BE2AE89  bl 0x82466e20
	ctx.lr = 0x8263BF9C;
	sub_82466E20(ctx, base);
	// 8263BF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263BFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263BFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263BFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263BFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263BFB0 size=108
    let mut pc: u32 = 0x8263BFB0;
    'dispatch: loop {
        match pc {
            0x8263BFB0 => {
    //   block [0x8263BFB0..0x8263C01C)
	// 8263BFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263BFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263BFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263BFBC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263BFC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263BFC4: 38EB69A0  addi r7, r11, 0x69a0
	ctx.r[7].s64 = ctx.r[11].s64 + 27040;
	// 8263BFC8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8263BFCC: 388A39D8  addi r4, r10, 0x39d8
	ctx.r[4].s64 = ctx.r[10].s64 + 14808;
	// 8263BFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263BFD4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263BFD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263BFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263BFE0: 386AB83C  addi r3, r10, -0x47c4
	ctx.r[3].s64 = ctx.r[10].s64 + -18372;
	// 8263BFE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263BFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263BFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263BFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263BFF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263BFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263BFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C008: 4BE2AE19  bl 0x82466e20
	ctx.lr = 0x8263C00C;
	sub_82466E20(ctx, base);
	// 8263C00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C020 size=108
    let mut pc: u32 = 0x8263C020;
    'dispatch: loop {
        match pc {
            0x8263C020 => {
    //   block [0x8263C020..0x8263C08C)
	// 8263C020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C02C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C034: 38EB6A60  addi r7, r11, 0x6a60
	ctx.r[7].s64 = ctx.r[11].s64 + 27232;
	// 8263C038: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8263C03C: 388A39F4  addi r4, r10, 0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14836;
	// 8263C040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C044: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263C04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C050: 386AB86C  addi r3, r10, -0x4794
	ctx.r[3].s64 = ctx.r[10].s64 + -18324;
	// 8263C054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263C058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C078: 4BE2ADA9  bl 0x82466e20
	ctx.lr = 0x8263C07C;
	sub_82466E20(ctx, base);
	// 8263C07C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C090 size=108
    let mut pc: u32 = 0x8263C090;
    'dispatch: loop {
        match pc {
            0x8263C090 => {
    //   block [0x8263C090..0x8263C0FC)
	// 8263C090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C09C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C0A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C0A4: 38EB6AF0  addi r7, r11, 0x6af0
	ctx.r[7].s64 = ctx.r[11].s64 + 27376;
	// 8263C0A8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8263C0AC: 388A3A14  addi r4, r10, 0x3a14
	ctx.r[4].s64 = ctx.r[10].s64 + 14868;
	// 8263C0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C0B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C0B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263C0BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C0C0: 386AB89C  addi r3, r10, -0x4764
	ctx.r[3].s64 = ctx.r[10].s64 + -18276;
	// 8263C0C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263C0C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C0CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C0E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C0E8: 4BE2AD39  bl 0x82466e20
	ctx.lr = 0x8263C0EC;
	sub_82466E20(ctx, base);
	// 8263C0EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C0F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C0F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C100 size=108
    let mut pc: u32 = 0x8263C100;
    'dispatch: loop {
        match pc {
            0x8263C100 => {
    //   block [0x8263C100..0x8263C16C)
	// 8263C100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C10C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C114: 38EB6C28  addi r7, r11, 0x6c28
	ctx.r[7].s64 = ctx.r[11].s64 + 27688;
	// 8263C118: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263C11C: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 8263C120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C124: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C128: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263C12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C130: 386AB8CC  addi r3, r10, -0x4734
	ctx.r[3].s64 = ctx.r[10].s64 + -18228;
	// 8263C134: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263C138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C154: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C158: 4BE2ACC9  bl 0x82466e20
	ctx.lr = 0x8263C15C;
	sub_82466E20(ctx, base);
	// 8263C15C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C170 size=116
    let mut pc: u32 = 0x8263C170;
    'dispatch: loop {
        match pc {
            0x8263C170 => {
    //   block [0x8263C170..0x8263C1E4)
	// 8263C170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C17C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C180: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263C184: 390B6C90  addi r8, r11, 0x6c90
	ctx.r[8].s64 = ctx.r[11].s64 + 27792;
	// 8263C188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C18C: 392A673C  addi r9, r10, 0x673c
	ctx.r[9].s64 = ctx.r[10].s64 + 26428;
	// 8263C190: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C194: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263C198: 38AAB8CC  addi r5, r10, -0x4734
	ctx.r[5].s64 = ctx.r[10].s64 + -18228;
	// 8263C19C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C1A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C1B4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263C1B8: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 8263C1BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263C1C0: 386BB8FC  addi r3, r11, -0x4704
	ctx.r[3].s64 = ctx.r[11].s64 + -18180;
	// 8263C1C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263C1C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C1D0: 4BE2AC51  bl 0x82466e20
	ctx.lr = 0x8263C1D4;
	sub_82466E20(ctx, base);
	// 8263C1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C1E8 size=96
    let mut pc: u32 = 0x8263C1E8;
    'dispatch: loop {
        match pc {
            0x8263C1E8 => {
    //   block [0x8263C1E8..0x8263C248)
	// 8263C1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C1F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C1FC: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 8263C200: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C208: 386AB92C  addi r3, r10, -0x46d4
	ctx.r[3].s64 = ctx.r[10].s64 + -18132;
	// 8263C20C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C214: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263C218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C228: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C230: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C234: 4BE2ABED  bl 0x82466e20
	ctx.lr = 0x8263C238;
	sub_82466E20(ctx, base);
	// 8263C238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C248 size=112
    let mut pc: u32 = 0x8263C248;
    'dispatch: loop {
        match pc {
            0x8263C248 => {
    //   block [0x8263C248..0x8263C2B8)
	// 8263C248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C254: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C258: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C25C: 38AAD93C  addi r5, r10, -0x26c4
	ctx.r[5].s64 = ctx.r[10].s64 + -9924;
	// 8263C260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C264: 390B6D08  addi r8, r11, 0x6d08
	ctx.r[8].s64 = ctx.r[11].s64 + 27912;
	// 8263C268: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263C26C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 8263C270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C274: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C280: 386AB95C  addi r3, r10, -0x46a4
	ctx.r[3].s64 = ctx.r[10].s64 + -18084;
	// 8263C284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C2A4: 4BE2AB7D  bl 0x82466e20
	ctx.lr = 0x8263C2A8;
	sub_82466E20(ctx, base);
	// 8263C2A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C2B8 size=96
    let mut pc: u32 = 0x8263C2B8;
    'dispatch: loop {
        match pc {
            0x8263C2B8 => {
    //   block [0x8263C2B8..0x8263C318)
	// 8263C2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C2C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C2C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C2CC: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 8263C2D0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C2D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C2D8: 386AB98C  addi r3, r10, -0x4674
	ctx.r[3].s64 = ctx.r[10].s64 + -18036;
	// 8263C2DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C2E4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263C2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C2F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C2F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C300: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C304: 4BE2AB1D  bl 0x82466e20
	ctx.lr = 0x8263C308;
	sub_82466E20(ctx, base);
	// 8263C308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263C318 size=24
    let mut pc: u32 = 0x8263C318;
    'dispatch: loop {
        match pc {
            0x8263C318 => {
    //   block [0x8263C318..0x8263C330)
	// 8263C318: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C31C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263C320: 394AB608  addi r10, r10, -0x49f8
	ctx.r[10].s64 = ctx.r[10].s64 + -18936;
	// 8263C324: 816B6C8C  lwz r11, 0x6c8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27788 as u32) ) } as u64;
	// 8263C328: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263C32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C330 size=116
    let mut pc: u32 = 0x8263C330;
    'dispatch: loop {
        match pc {
            0x8263C330 => {
    //   block [0x8263C330..0x8263C3A4)
	// 8263C330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C33C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263C340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263C344: 390BB608  addi r8, r11, -0x49f8
	ctx.r[8].s64 = ctx.r[11].s64 + -18936;
	// 8263C348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C34C: 392A6778  addi r9, r10, 0x6778
	ctx.r[9].s64 = ctx.r[10].s64 + 26488;
	// 8263C350: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C354: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263C358: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263C35C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C364: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C36C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263C370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C374: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263C378: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 8263C37C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263C380: 386BB9BC  addi r3, r11, -0x4644
	ctx.r[3].s64 = ctx.r[11].s64 + -17988;
	// 8263C384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263C388: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C390: 4BE2AA91  bl 0x82466e20
	ctx.lr = 0x8263C394;
	sub_82466E20(ctx, base);
	// 8263C394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C3A8 size=104
    let mut pc: u32 = 0x8263C3A8;
    'dispatch: loop {
        match pc {
            0x8263C3A8 => {
    //   block [0x8263C3A8..0x8263C410)
	// 8263C3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C3B4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263C3B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C3BC: 392A67A4  addi r9, r10, 0x67a4
	ctx.r[9].s64 = ctx.r[10].s64 + 26532;
	// 8263C3C0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C3C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C3C8: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263C3CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C3DC: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 8263C3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C3E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C3E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C3F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C3F4: 386AB9EC  addi r3, r10, -0x4614
	ctx.r[3].s64 = ctx.r[10].s64 + -17940;
	// 8263C3F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263C3FC: 4BE2AA25  bl 0x82466e20
	ctx.lr = 0x8263C400;
	sub_82466E20(ctx, base);
	// 8263C400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C410 size=96
    let mut pc: u32 = 0x8263C410;
    'dispatch: loop {
        match pc {
            0x8263C410 => {
    //   block [0x8263C410..0x8263C470)
	// 8263C410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C41C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C424: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 8263C428: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C430: 386ABA1C  addi r3, r10, -0x45e4
	ctx.r[3].s64 = ctx.r[10].s64 + -17892;
	// 8263C434: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C43C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263C440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C450: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C458: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C45C: 4BE2A9C5  bl 0x82466e20
	ctx.lr = 0x8263C460;
	sub_82466E20(ctx, base);
	// 8263C460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C470 size=100
    let mut pc: u32 = 0x8263C470;
    'dispatch: loop {
        match pc {
            0x8263C470 => {
    //   block [0x8263C470..0x8263C4D4)
	// 8263C470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C47C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C484: 38AAB9EC  addi r5, r10, -0x4614
	ctx.r[5].s64 = ctx.r[10].s64 + -17940;
	// 8263C488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C490: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 8263C494: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C4A4: 386ABA4C  addi r3, r10, -0x45b4
	ctx.r[3].s64 = ctx.r[10].s64 + -17844;
	// 8263C4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C4AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C4B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C4B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C4C0: 4BE2A961  bl 0x82466e20
	ctx.lr = 0x8263C4C4;
	sub_82466E20(ctx, base);
	// 8263C4C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C4C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C4CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C4D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C4D8 size=112
    let mut pc: u32 = 0x8263C4D8;
    'dispatch: loop {
        match pc {
            0x8263C4D8 => {
    //   block [0x8263C4D8..0x8263C548)
	// 8263C4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C4E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C4E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C4EC: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 8263C4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C4F4: 390B6D70  addi r8, r11, 0x6d70
	ctx.r[8].s64 = ctx.r[11].s64 + 28016;
	// 8263C4F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263C4FC: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 8263C500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C504: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C510: 386ABA7C  addi r3, r10, -0x4584
	ctx.r[3].s64 = ctx.r[10].s64 + -17796;
	// 8263C514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C534: 4BE2A8ED  bl 0x82466e20
	ctx.lr = 0x8263C538;
	sub_82466E20(ctx, base);
	// 8263C538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C548 size=112
    let mut pc: u32 = 0x8263C548;
    'dispatch: loop {
        match pc {
            0x8263C548 => {
    //   block [0x8263C548..0x8263C5B8)
	// 8263C548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C554: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C558: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C55C: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 8263C560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C564: 390B6DB8  addi r8, r11, 0x6db8
	ctx.r[8].s64 = ctx.r[11].s64 + 28088;
	// 8263C568: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263C56C: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 8263C570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C574: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C578: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C580: 386ABAAC  addi r3, r10, -0x4554
	ctx.r[3].s64 = ctx.r[10].s64 + -17748;
	// 8263C584: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C5A4: 4BE2A87D  bl 0x82466e20
	ctx.lr = 0x8263C5A8;
	sub_82466E20(ctx, base);
	// 8263C5A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C5AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C5B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C5B8 size=100
    let mut pc: u32 = 0x8263C5B8;
    'dispatch: loop {
        match pc {
            0x8263C5B8 => {
    //   block [0x8263C5B8..0x8263C61C)
	// 8263C5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C5C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C5CC: 38AAB9BC  addi r5, r10, -0x4644
	ctx.r[5].s64 = ctx.r[10].s64 + -17988;
	// 8263C5D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C5D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C5D8: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8263C5DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C5EC: 386ABADC  addi r3, r10, -0x4524
	ctx.r[3].s64 = ctx.r[10].s64 + -17700;
	// 8263C5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C5F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C5F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C600: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C608: 4BE2A819  bl 0x82466e20
	ctx.lr = 0x8263C60C;
	sub_82466E20(ctx, base);
	// 8263C60C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C620 size=96
    let mut pc: u32 = 0x8263C620;
    'dispatch: loop {
        match pc {
            0x8263C620 => {
    //   block [0x8263C620..0x8263C680)
	// 8263C620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C62C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C634: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 8263C638: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C63C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C640: 386ABB0C  addi r3, r10, -0x44f4
	ctx.r[3].s64 = ctx.r[10].s64 + -17652;
	// 8263C644: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C64C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263C650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C660: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C668: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C66C: 4BE2A7B5  bl 0x82466e20
	ctx.lr = 0x8263C670;
	sub_82466E20(ctx, base);
	// 8263C670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C680 size=112
    let mut pc: u32 = 0x8263C680;
    'dispatch: loop {
        match pc {
            0x8263C680 => {
    //   block [0x8263C680..0x8263C6F0)
	// 8263C680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C68C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C690: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C694: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263C698: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C69C: 390B6DD0  addi r8, r11, 0x6dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 28112;
	// 8263C6A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263C6A4: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 8263C6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C6AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C6B8: 386ABB3C  addi r3, r10, -0x44c4
	ctx.r[3].s64 = ctx.r[10].s64 + -17604;
	// 8263C6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C6DC: 4BE2A745  bl 0x82466e20
	ctx.lr = 0x8263C6E0;
	sub_82466E20(ctx, base);
	// 8263C6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C6F0 size=96
    let mut pc: u32 = 0x8263C6F0;
    'dispatch: loop {
        match pc {
            0x8263C6F0 => {
    //   block [0x8263C6F0..0x8263C750)
	// 8263C6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C6FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C704: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 8263C708: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C70C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C710: 386ABB6C  addi r3, r10, -0x4494
	ctx.r[3].s64 = ctx.r[10].s64 + -17556;
	// 8263C714: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C71C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263C720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C72C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C730: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C738: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C73C: 4BE2A6E5  bl 0x82466e20
	ctx.lr = 0x8263C740;
	sub_82466E20(ctx, base);
	// 8263C740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C750 size=112
    let mut pc: u32 = 0x8263C750;
    'dispatch: loop {
        match pc {
            0x8263C750 => {
    //   block [0x8263C750..0x8263C7C0)
	// 8263C750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C75C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C760: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C764: 38AABB6C  addi r5, r10, -0x4494
	ctx.r[5].s64 = ctx.r[10].s64 + -17556;
	// 8263C768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C76C: 390B6E00  addi r8, r11, 0x6e00
	ctx.r[8].s64 = ctx.r[11].s64 + 28160;
	// 8263C770: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263C774: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 8263C778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C77C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C788: 386ABB9C  addi r3, r10, -0x4464
	ctx.r[3].s64 = ctx.r[10].s64 + -17508;
	// 8263C78C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C7AC: 4BE2A675  bl 0x82466e20
	ctx.lr = 0x8263C7B0;
	sub_82466E20(ctx, base);
	// 8263C7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C7C0 size=108
    let mut pc: u32 = 0x8263C7C0;
    'dispatch: loop {
        match pc {
            0x8263C7C0 => {
    //   block [0x8263C7C0..0x8263C82C)
	// 8263C7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C7CC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C7D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C7D4: 38EB6E18  addi r7, r11, 0x6e18
	ctx.r[7].s64 = ctx.r[11].s64 + 28184;
	// 8263C7D8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263C7DC: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8263C7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C7E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C7E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263C7EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C7F0: 386ABBCC  addi r3, r10, -0x4434
	ctx.r[3].s64 = ctx.r[10].s64 + -17460;
	// 8263C7F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263C7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C7FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C814: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263C818: 4BE2A609  bl 0x82466e20
	ctx.lr = 0x8263C81C;
	sub_82466E20(ctx, base);
	// 8263C81C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C830 size=112
    let mut pc: u32 = 0x8263C830;
    'dispatch: loop {
        match pc {
            0x8263C830 => {
    //   block [0x8263C830..0x8263C8A0)
	// 8263C830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C83C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C840: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C844: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263C848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C84C: 390B6E78  addi r8, r11, 0x6e78
	ctx.r[8].s64 = ctx.r[11].s64 + 28280;
	// 8263C850: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263C854: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8263C858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C85C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C868: 386ABBFC  addi r3, r10, -0x4404
	ctx.r[3].s64 = ctx.r[10].s64 + -17412;
	// 8263C86C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C88C: 4BE2A595  bl 0x82466e20
	ctx.lr = 0x8263C890;
	sub_82466E20(ctx, base);
	// 8263C890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C8A0 size=112
    let mut pc: u32 = 0x8263C8A0;
    'dispatch: loop {
        match pc {
            0x8263C8A0 => {
    //   block [0x8263C8A0..0x8263C910)
	// 8263C8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C8AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C8B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C8B4: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263C8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C8BC: 390B6E90  addi r8, r11, 0x6e90
	ctx.r[8].s64 = ctx.r[11].s64 + 28304;
	// 8263C8C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263C8C4: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8263C8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C8CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C8D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C8D8: 386ABC2C  addi r3, r10, -0x43d4
	ctx.r[3].s64 = ctx.r[10].s64 + -17364;
	// 8263C8DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263C8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C8FC: 4BE2A525  bl 0x82466e20
	ctx.lr = 0x8263C900;
	sub_82466E20(ctx, base);
	// 8263C900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C910 size=100
    let mut pc: u32 = 0x8263C910;
    'dispatch: loop {
        match pc {
            0x8263C910 => {
    //   block [0x8263C910..0x8263C974)
	// 8263C910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C91C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C924: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263C928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263C930: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8263C934: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C944: 386ABC5C  addi r3, r10, -0x43a4
	ctx.r[3].s64 = ctx.r[10].s64 + -17316;
	// 8263C948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C94C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263C950: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263C954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C958: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263C95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C960: 4BE2A4C1  bl 0x82466e20
	ctx.lr = 0x8263C964;
	sub_82466E20(ctx, base);
	// 8263C964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C978 size=116
    let mut pc: u32 = 0x8263C978;
    'dispatch: loop {
        match pc {
            0x8263C978 => {
    //   block [0x8263C978..0x8263C9EC)
	// 8263C978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C984: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263C988: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263C98C: 390B6EC0  addi r8, r11, 0x6ec0
	ctx.r[8].s64 = ctx.r[11].s64 + 28352;
	// 8263C990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263C994: 392A67D0  addi r9, r10, 0x67d0
	ctx.r[9].s64 = ctx.r[10].s64 + 26576;
	// 8263C998: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263C99C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8263C9A0: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263C9A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263C9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263C9AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263C9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263C9B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263C9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263C9BC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263C9C0: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8263C9C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263C9C8: 386BBC8C  addi r3, r11, -0x4374
	ctx.r[3].s64 = ctx.r[11].s64 + -17268;
	// 8263C9CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263C9D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263C9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263C9D8: 4BE2A449  bl 0x82466e20
	ctx.lr = 0x8263C9DC;
	sub_82466E20(ctx, base);
	// 8263C9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263C9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263C9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263C9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263C9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263C9F0 size=112
    let mut pc: u32 = 0x8263C9F0;
    'dispatch: loop {
        match pc {
            0x8263C9F0 => {
    //   block [0x8263C9F0..0x8263CA60)
	// 8263C9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263C9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263C9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263C9FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CA00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CA04: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263CA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CA0C: 390B6EF0  addi r8, r11, 0x6ef0
	ctx.r[8].s64 = ctx.r[11].s64 + 28400;
	// 8263CA10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263CA14: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8263CA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CA1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CA20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CA28: 386ABCBC  addi r3, r10, -0x4344
	ctx.r[3].s64 = ctx.r[10].s64 + -17220;
	// 8263CA2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CA34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CA3C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263CA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CA4C: 4BE2A3D5  bl 0x82466e20
	ctx.lr = 0x8263CA50;
	sub_82466E20(ctx, base);
	// 8263CA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CA60 size=116
    let mut pc: u32 = 0x8263CA60;
    'dispatch: loop {
        match pc {
            0x8263CA60 => {
    //   block [0x8263CA60..0x8263CAD4)
	// 8263CA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CA6C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CA70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263CA74: 390B6F0C  addi r8, r11, 0x6f0c
	ctx.r[8].s64 = ctx.r[11].s64 + 28428;
	// 8263CA78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CA7C: 392A67FC  addi r9, r10, 0x67fc
	ctx.r[9].s64 = ctx.r[10].s64 + 26620;
	// 8263CA80: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CA84: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8263CA88: 38AAC31C  addi r5, r10, -0x3ce4
	ctx.r[5].s64 = ctx.r[10].s64 + -15588;
	// 8263CA8C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CA94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CA9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CAA4: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263CAA8: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8263CAAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263CAB0: 386BBCEC  addi r3, r11, -0x4314
	ctx.r[3].s64 = ctx.r[11].s64 + -17172;
	// 8263CAB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263CAB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CAC0: 4BE2A361  bl 0x82466e20
	ctx.lr = 0x8263CAC4;
	sub_82466E20(ctx, base);
	// 8263CAC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CAC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CAD8 size=112
    let mut pc: u32 = 0x8263CAD8;
    'dispatch: loop {
        match pc {
            0x8263CAD8 => {
    //   block [0x8263CAD8..0x8263CB48)
	// 8263CAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CAE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CAE8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CAEC: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263CAF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CAF4: 390B6F28  addi r8, r11, 0x6f28
	ctx.r[8].s64 = ctx.r[11].s64 + 28456;
	// 8263CAF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263CAFC: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8263CB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CB04: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CB08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CB10: 386ABD1C  addi r3, r10, -0x42e4
	ctx.r[3].s64 = ctx.r[10].s64 + -17124;
	// 8263CB14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CB24: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263CB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CB34: 4BE2A2ED  bl 0x82466e20
	ctx.lr = 0x8263CB38;
	sub_82466E20(ctx, base);
	// 8263CB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CB48 size=112
    let mut pc: u32 = 0x8263CB48;
    'dispatch: loop {
        match pc {
            0x8263CB48 => {
    //   block [0x8263CB48..0x8263CBB8)
	// 8263CB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CB54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CB58: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CB5C: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263CB60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CB64: 390B6FA0  addi r8, r11, 0x6fa0
	ctx.r[8].s64 = ctx.r[11].s64 + 28576;
	// 8263CB68: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263CB6C: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8263CB70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CB74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CB78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CB7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CB80: 386ABD4C  addi r3, r10, -0x42b4
	ctx.r[3].s64 = ctx.r[10].s64 + -17076;
	// 8263CB84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CB88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CB8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CB90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CB94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CB98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CBA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CBA4: 4BE2A27D  bl 0x82466e20
	ctx.lr = 0x8263CBA8;
	sub_82466E20(ctx, base);
	// 8263CBA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CBB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CBB8 size=112
    let mut pc: u32 = 0x8263CBB8;
    'dispatch: loop {
        match pc {
            0x8263CBB8 => {
    //   block [0x8263CBB8..0x8263CC28)
	// 8263CBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CBC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CBC8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CBCC: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263CBD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CBD4: 390B6FE8  addi r8, r11, 0x6fe8
	ctx.r[8].s64 = ctx.r[11].s64 + 28648;
	// 8263CBD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263CBDC: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8263CBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CBE4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CBE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CBEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CBF0: 386ABD7C  addi r3, r10, -0x4284
	ctx.r[3].s64 = ctx.r[10].s64 + -17028;
	// 8263CBF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CBF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CBFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CC14: 4BE2A20D  bl 0x82466e20
	ctx.lr = 0x8263CC18;
	sub_82466E20(ctx, base);
	// 8263CC18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CC28 size=112
    let mut pc: u32 = 0x8263CC28;
    'dispatch: loop {
        match pc {
            0x8263CC28 => {
    //   block [0x8263CC28..0x8263CC98)
	// 8263CC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CC34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CC38: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CC3C: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263CC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CC44: 390B7030  addi r8, r11, 0x7030
	ctx.r[8].s64 = ctx.r[11].s64 + 28720;
	// 8263CC48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263CC4C: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8263CC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CC54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CC58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CC5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CC60: 386ABDAC  addi r3, r10, -0x4254
	ctx.r[3].s64 = ctx.r[10].s64 + -16980;
	// 8263CC64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CC68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CC70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CC74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CC78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CC80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CC84: 4BE2A19D  bl 0x82466e20
	ctx.lr = 0x8263CC88;
	sub_82466E20(ctx, base);
	// 8263CC88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CC98 size=108
    let mut pc: u32 = 0x8263CC98;
    'dispatch: loop {
        match pc {
            0x8263CC98 => {
    //   block [0x8263CC98..0x8263CD04)
	// 8263CC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CCA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CCA4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CCA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CCAC: 38EB7078  addi r7, r11, 0x7078
	ctx.r[7].s64 = ctx.r[11].s64 + 28792;
	// 8263CCB0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263CCB4: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8263CCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CCBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CCC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263CCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CCC8: 386ABDDC  addi r3, r10, -0x4224
	ctx.r[3].s64 = ctx.r[10].s64 + -16932;
	// 8263CCCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263CCD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CCD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CCDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CCE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CCEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263CCF0: 4BE2A131  bl 0x82466e20
	ctx.lr = 0x8263CCF4;
	sub_82466E20(ctx, base);
	// 8263CCF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CCF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CCFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CD00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CD08 size=112
    let mut pc: u32 = 0x8263CD08;
    'dispatch: loop {
        match pc {
            0x8263CD08 => {
    //   block [0x8263CD08..0x8263CD78)
	// 8263CD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CD10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CD14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CD18: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CD1C: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263CD20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CD24: 390B70C0  addi r8, r11, 0x70c0
	ctx.r[8].s64 = ctx.r[11].s64 + 28864;
	// 8263CD28: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263CD2C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8263CD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CD34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CD38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CD40: 386ABE0C  addi r3, r10, -0x41f4
	ctx.r[3].s64 = ctx.r[10].s64 + -16884;
	// 8263CD44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CD48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CD64: 4BE2A0BD  bl 0x82466e20
	ctx.lr = 0x8263CD68;
	sub_82466E20(ctx, base);
	// 8263CD68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CD78 size=116
    let mut pc: u32 = 0x8263CD78;
    'dispatch: loop {
        match pc {
            0x8263CD78 => {
    //   block [0x8263CD78..0x8263CDEC)
	// 8263CD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CD84: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263CD88: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CD8C: 392B6838  addi r9, r11, 0x6838
	ctx.r[9].s64 = ctx.r[11].s64 + 26680;
	// 8263CD90: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263CD94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CD98: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8263CD9C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8263CDA0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CDA4: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8263CDA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CDAC: 396B7138  addi r11, r11, 0x7138
	ctx.r[11].s64 = ctx.r[11].s64 + 28984;
	// 8263CDB0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8263CDB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CDB8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8263CDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CDC0: 386ABE3C  addi r3, r10, -0x41c4
	ctx.r[3].s64 = ctx.r[10].s64 + -16836;
	// 8263CDC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263CDC8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8263CDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CDD0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8263CDD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263CDD8: 4BE2A049  bl 0x82466e20
	ctx.lr = 0x8263CDDC;
	sub_82466E20(ctx, base);
	// 8263CDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263CDF0 size=36
    let mut pc: u32 = 0x8263CDF0;
    'dispatch: loop {
        match pc {
            0x8263CDF0 => {
    //   block [0x8263CDF0..0x8263CE14)
	// 8263CDF0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CDF4: 814B71D0  lwz r10, 0x71d0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29136 as u32) ) } as u64;
	// 8263CDF8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263CDFC: 396BB638  addi r11, r11, -0x49c8
	ctx.r[11].s64 = ctx.r[11].s64 + -18888;
	// 8263CE00: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8263CE04: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8263CE08: 814A71C8  lwz r10, 0x71c8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29128 as u32) ) } as u64;
	// 8263CE0C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8263CE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CE18 size=108
    let mut pc: u32 = 0x8263CE18;
    'dispatch: loop {
        match pc {
            0x8263CE18 => {
    //   block [0x8263CE18..0x8263CE84)
	// 8263CE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CE24: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263CE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CE2C: 38EBB638  addi r7, r11, -0x49c8
	ctx.r[7].s64 = ctx.r[11].s64 + -18888;
	// 8263CE30: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8263CE34: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 8263CE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CE3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CE40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263CE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CE48: 386ABE6C  addi r3, r10, -0x4194
	ctx.r[3].s64 = ctx.r[10].s64 + -16788;
	// 8263CE4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263CE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263CE70: 4BE29FB1  bl 0x82466e20
	ctx.lr = 0x8263CE74;
	sub_82466E20(ctx, base);
	// 8263CE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263CE88 size=24
    let mut pc: u32 = 0x8263CE88;
    'dispatch: loop {
        match pc {
            0x8263CE88 => {
    //   block [0x8263CE88..0x8263CEA0)
	// 8263CE88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CE8C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263CE90: 394AB6E0  addi r10, r10, -0x4920
	ctx.r[10].s64 = ctx.r[10].s64 + -18720;
	// 8263CE94: 816B71C8  lwz r11, 0x71c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29128 as u32) ) } as u64;
	// 8263CE98: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8263CE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CEA0 size=116
    let mut pc: u32 = 0x8263CEA0;
    'dispatch: loop {
        match pc {
            0x8263CEA0 => {
    //   block [0x8263CEA0..0x8263CF14)
	// 8263CEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CEAC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263CEB0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8263CEB4: 390AB6E0  addi r8, r10, -0x4920
	ctx.r[8].s64 = ctx.r[10].s64 + -18720;
	// 8263CEB8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CEBC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263CEC0: 38AABE6C  addi r5, r10, -0x4194
	ctx.r[5].s64 = ctx.r[10].s64 + -16788;
	// 8263CEC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CEC8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263CECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CED4: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 8263CED8: 396B68F4  addi r11, r11, 0x68f4
	ctx.r[11].s64 = ctx.r[11].s64 + 26868;
	// 8263CEDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CEE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CEE4: 386ABE9C  addi r3, r10, -0x4164
	ctx.r[3].s64 = ctx.r[10].s64 + -16740;
	// 8263CEE8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263CEEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CEF0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263CEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CEF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CF00: 4BE29F21  bl 0x82466e20
	ctx.lr = 0x8263CF04;
	sub_82466E20(ctx, base);
	// 8263CF04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CF08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CF0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CF10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CF18 size=112
    let mut pc: u32 = 0x8263CF18;
    'dispatch: loop {
        match pc {
            0x8263CF18 => {
    //   block [0x8263CF18..0x8263CF88)
	// 8263CF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CF24: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CF28: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CF2C: 38AABE6C  addi r5, r10, -0x4194
	ctx.r[5].s64 = ctx.r[10].s64 + -16788;
	// 8263CF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CF34: 390B71D8  addi r8, r11, 0x71d8
	ctx.r[8].s64 = ctx.r[11].s64 + 29144;
	// 8263CF38: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263CF3C: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 8263CF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263CF44: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CF48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263CF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CF50: 386ABECC  addi r3, r10, -0x4134
	ctx.r[3].s64 = ctx.r[10].s64 + -16692;
	// 8263CF54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263CF58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263CF5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263CF60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263CF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CF68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263CF6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CF70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263CF74: 4BE29EAD  bl 0x82466e20
	ctx.lr = 0x8263CF78;
	sub_82466E20(ctx, base);
	// 8263CF78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263CF7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263CF80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263CF84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263CF88 size=24
    let mut pc: u32 = 0x8263CF88;
    'dispatch: loop {
        match pc {
            0x8263CF88 => {
    //   block [0x8263CF88..0x8263CFA0)
	// 8263CF88: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263CF8C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263CF90: 394AB7A0  addi r10, r10, -0x4860
	ctx.r[10].s64 = ctx.r[10].s64 + -18528;
	// 8263CF94: 816B73EC  lwz r11, 0x73ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29676 as u32) ) } as u64;
	// 8263CF98: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8263CF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263CFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263CFA0 size=116
    let mut pc: u32 = 0x8263CFA0;
    'dispatch: loop {
        match pc {
            0x8263CFA0 => {
    //   block [0x8263CFA0..0x8263D014)
	// 8263CFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263CFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263CFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263CFAC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263CFB0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CFB4: 392B68B8  addi r9, r11, 0x68b8
	ctx.r[9].s64 = ctx.r[11].s64 + 26808;
	// 8263CFB8: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263CFBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263CFC0: 38E90060  addi r7, r9, 0x60
	ctx.r[7].s64 = ctx.r[9].s64 + 96;
	// 8263CFC4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8263CFC8: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263CFCC: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 8263CFD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263CFD4: 396BB7A0  addi r11, r11, -0x4860
	ctx.r[11].s64 = ctx.r[11].s64 + -18528;
	// 8263CFD8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8263CFDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263CFE0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8263CFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263CFE8: 386ABEFC  addi r3, r10, -0x4104
	ctx.r[3].s64 = ctx.r[10].s64 + -16644;
	// 8263CFEC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8263CFF0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8263CFF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263CFF8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8263CFFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D000: 4BE29E21  bl 0x82466e20
	ctx.lr = 0x8263D004;
	sub_82466E20(ctx, base);
	// 8263D004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D00C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D018 size=100
    let mut pc: u32 = 0x8263D018;
    'dispatch: loop {
        match pc {
            0x8263D018 => {
    //   block [0x8263D018..0x8263D07C)
	// 8263D018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D024: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D02C: 38AAC04C  addi r5, r10, -0x3fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -16308;
	// 8263D030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D038: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8263D03C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D04C: 386ABF2C  addi r3, r10, -0x40d4
	ctx.r[3].s64 = ctx.r[10].s64 + -16596;
	// 8263D050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D054: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D058: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263D05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D060: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D068: 4BE29DB9  bl 0x82466e20
	ctx.lr = 0x8263D06C;
	sub_82466E20(ctx, base);
	// 8263D06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D080 size=100
    let mut pc: u32 = 0x8263D080;
    'dispatch: loop {
        match pc {
            0x8263D080 => {
    //   block [0x8263D080..0x8263D0E4)
	// 8263D080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D08C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D094: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263D098: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D0A0: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8263D0A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D0A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D0AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D0B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D0B4: 386ABF5C  addi r3, r10, -0x40a4
	ctx.r[3].s64 = ctx.r[10].s64 + -16548;
	// 8263D0B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D0BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D0C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263D0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D0C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D0CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D0D0: 4BE29D51  bl 0x82466e20
	ctx.lr = 0x8263D0D4;
	sub_82466E20(ctx, base);
	// 8263D0D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D0D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D0DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D0E8 size=108
    let mut pc: u32 = 0x8263D0E8;
    'dispatch: loop {
        match pc {
            0x8263D0E8 => {
    //   block [0x8263D0E8..0x8263D154)
	// 8263D0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D0F4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D0F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D0FC: 38EB7238  addi r7, r11, 0x7238
	ctx.r[7].s64 = ctx.r[11].s64 + 29240;
	// 8263D100: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263D104: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8263D108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D10C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D110: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D118: 386ABF8C  addi r3, r10, -0x4074
	ctx.r[3].s64 = ctx.r[10].s64 + -16500;
	// 8263D11C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D12C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D13C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D140: 4BE29CE1  bl 0x82466e20
	ctx.lr = 0x8263D144;
	sub_82466E20(ctx, base);
	// 8263D144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D158 size=112
    let mut pc: u32 = 0x8263D158;
    'dispatch: loop {
        match pc {
            0x8263D158 => {
    //   block [0x8263D158..0x8263D1C8)
	// 8263D158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D15C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D168: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D16C: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263D170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D174: 390B7298  addi r8, r11, 0x7298
	ctx.r[8].s64 = ctx.r[11].s64 + 29336;
	// 8263D178: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263D17C: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8263D180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D184: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D18C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D190: 386ABFBC  addi r3, r10, -0x4044
	ctx.r[3].s64 = ctx.r[10].s64 + -16452;
	// 8263D194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D19C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D1A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D1AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D1B4: 4BE29C6D  bl 0x82466e20
	ctx.lr = 0x8263D1B8;
	sub_82466E20(ctx, base);
	// 8263D1B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D1BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D1C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D1C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D1C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D1C8 size=108
    let mut pc: u32 = 0x8263D1C8;
    'dispatch: loop {
        match pc {
            0x8263D1C8 => {
    //   block [0x8263D1C8..0x8263D234)
	// 8263D1C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D1CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D1D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D1D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D1D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D1DC: 38EB72E0  addi r7, r11, 0x72e0
	ctx.r[7].s64 = ctx.r[11].s64 + 29408;
	// 8263D1E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263D1E4: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8263D1E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D1EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D1F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D1F8: 386ABFEC  addi r3, r10, -0x4014
	ctx.r[3].s64 = ctx.r[10].s64 + -16404;
	// 8263D1FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D21C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D220: 4BE29C01  bl 0x82466e20
	ctx.lr = 0x8263D224;
	sub_82466E20(ctx, base);
	// 8263D224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D22C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263D238 size=28
    let mut pc: u32 = 0x8263D238;
    'dispatch: loop {
        match pc {
            0x8263D238 => {
    //   block [0x8263D238..0x8263D254)
	// 8263D238: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D23C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D240: 394AB848  addi r10, r10, -0x47b8
	ctx.r[10].s64 = ctx.r[10].s64 + -18360;
	// 8263D244: 816B71D4  lwz r11, 0x71d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29140 as u32) ) } as u64;
	// 8263D248: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8263D24C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8263D250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D258 size=112
    let mut pc: u32 = 0x8263D258;
    'dispatch: loop {
        match pc {
            0x8263D258 => {
    //   block [0x8263D258..0x8263D2C8)
	// 8263D258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D264: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D268: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 8263D26C: 38EAB848  addi r7, r10, -0x47b8
	ctx.r[7].s64 = ctx.r[10].s64 + -18360;
	// 8263D270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D274: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263D278: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8263D27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D280: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D284: 396B69A0  addi r11, r11, 0x69a0
	ctx.r[11].s64 = ctx.r[11].s64 + 27040;
	// 8263D288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D28C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D290: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D294: 386AC01C  addi r3, r10, -0x3fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -16356;
	// 8263D298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D29C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263D2A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D2A4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263D2A8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D2AC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D2B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D2B4: 4BE29B6D  bl 0x82466e20
	ctx.lr = 0x8263D2B8;
	sub_82466E20(ctx, base);
	// 8263D2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263D2C8 size=24
    let mut pc: u32 = 0x8263D2C8;
    'dispatch: loop {
        match pc {
            0x8263D2C8 => {
    //   block [0x8263D2C8..0x8263D2E0)
	// 8263D2C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D2CC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D2D0: 394AB998  addi r10, r10, -0x4668
	ctx.r[10].s64 = ctx.r[10].s64 + -18024;
	// 8263D2D4: 816B73EC  lwz r11, 0x73ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29676 as u32) ) } as u64;
	// 8263D2D8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8263D2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D2E0 size=116
    let mut pc: u32 = 0x8263D2E0;
    'dispatch: loop {
        match pc {
            0x8263D2E0 => {
    //   block [0x8263D2E0..0x8263D354)
	// 8263D2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D2EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263D2F0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D2F4: 392B6978  addi r9, r11, 0x6978
	ctx.r[9].s64 = ctx.r[11].s64 + 27000;
	// 8263D2F8: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263D2FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D300: 38E90064  addi r7, r9, 0x64
	ctx.r[7].s64 = ctx.r[9].s64 + 100;
	// 8263D304: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8263D308: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263D30C: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8263D310: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D314: 396BB998  addi r11, r11, -0x4668
	ctx.r[11].s64 = ctx.r[11].s64 + -18024;
	// 8263D318: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8263D31C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D320: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8263D324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D328: 386AC04C  addi r3, r10, -0x3fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -16308;
	// 8263D32C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8263D330: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8263D334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D338: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8263D33C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D340: 4BE29AE1  bl 0x82466e20
	ctx.lr = 0x8263D344;
	sub_82466E20(ctx, base);
	// 8263D344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D358 size=112
    let mut pc: u32 = 0x8263D358;
    'dispatch: loop {
        match pc {
            0x8263D358 => {
    //   block [0x8263D358..0x8263D3C8)
	// 8263D358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D364: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D368: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D36C: 38AABC5C  addi r5, r10, -0x43a4
	ctx.r[5].s64 = ctx.r[10].s64 + -17316;
	// 8263D370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D374: 390B72FC  addi r8, r11, 0x72fc
	ctx.r[8].s64 = ctx.r[11].s64 + 29436;
	// 8263D378: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263D37C: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8263D380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D384: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D390: 386AC07C  addi r3, r10, -0x3f84
	ctx.r[3].s64 = ctx.r[10].s64 + -16260;
	// 8263D394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D3B4: 4BE29A6D  bl 0x82466e20
	ctx.lr = 0x8263D3B8;
	sub_82466E20(ctx, base);
	// 8263D3B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263D3C8 size=24
    let mut pc: u32 = 0x8263D3C8;
    'dispatch: loop {
        match pc {
            0x8263D3C8 => {
    //   block [0x8263D3C8..0x8263D3E0)
	// 8263D3C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D3CC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D3D0: 394ABA40  addi r10, r10, -0x45c0
	ctx.r[10].s64 = ctx.r[10].s64 + -17856;
	// 8263D3D4: 816B73EC  lwz r11, 0x73ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29676 as u32) ) } as u64;
	// 8263D3D8: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 8263D3DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D3E0 size=116
    let mut pc: u32 = 0x8263D3E0;
    'dispatch: loop {
        match pc {
            0x8263D3E0 => {
    //   block [0x8263D3E0..0x8263D454)
	// 8263D3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D3EC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D3F0: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8263D3F4: 390ABA40  addi r8, r10, -0x45c0
	ctx.r[8].s64 = ctx.r[10].s64 + -17856;
	// 8263D3F8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D3FC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263D400: 38AABC5C  addi r5, r10, -0x43a4
	ctx.r[5].s64 = ctx.r[10].s64 + -17316;
	// 8263D404: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D408: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263D40C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D414: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 8263D418: 396B69FC  addi r11, r11, 0x69fc
	ctx.r[11].s64 = ctx.r[11].s64 + 27132;
	// 8263D41C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D420: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263D424: 386AC0AC  addi r3, r10, -0x3f54
	ctx.r[3].s64 = ctx.r[10].s64 + -16212;
	// 8263D428: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263D42C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D430: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263D434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D440: 4BE299E1  bl 0x82466e20
	ctx.lr = 0x8263D444;
	sub_82466E20(ctx, base);
	// 8263D444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D44C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D458 size=112
    let mut pc: u32 = 0x8263D458;
    'dispatch: loop {
        match pc {
            0x8263D458 => {
    //   block [0x8263D458..0x8263D4C8)
	// 8263D458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D464: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D468: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D46C: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263D470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D474: 390B732C  addi r8, r11, 0x732c
	ctx.r[8].s64 = ctx.r[11].s64 + 29484;
	// 8263D478: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263D47C: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 8263D480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D484: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D490: 386AC0DC  addi r3, r10, -0x3f24
	ctx.r[3].s64 = ctx.r[10].s64 + -16164;
	// 8263D494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D4A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263D4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D4B4: 4BE2996D  bl 0x82466e20
	ctx.lr = 0x8263D4B8;
	sub_82466E20(ctx, base);
	// 8263D4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D4C8 size=108
    let mut pc: u32 = 0x8263D4C8;
    'dispatch: loop {
        match pc {
            0x8263D4C8 => {
    //   block [0x8263D4C8..0x8263D534)
	// 8263D4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D4D4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D4D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D4DC: 38EB735C  addi r7, r11, 0x735c
	ctx.r[7].s64 = ctx.r[11].s64 + 29532;
	// 8263D4E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263D4E4: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8263D4E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D4EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D4F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D4F8: 386AC10C  addi r3, r10, -0x3ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -16116;
	// 8263D4FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D51C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D520: 4BE29901  bl 0x82466e20
	ctx.lr = 0x8263D524;
	sub_82466E20(ctx, base);
	// 8263D524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D52C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D538 size=112
    let mut pc: u32 = 0x8263D538;
    'dispatch: loop {
        match pc {
            0x8263D538 => {
    //   block [0x8263D538..0x8263D5A8)
	// 8263D538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D544: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D548: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D54C: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263D550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D554: 390B738C  addi r8, r11, 0x738c
	ctx.r[8].s64 = ctx.r[11].s64 + 29580;
	// 8263D558: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263D55C: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8263D560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D564: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D570: 386AC13C  addi r3, r10, -0x3ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -16068;
	// 8263D574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D594: 4BE2988D  bl 0x82466e20
	ctx.lr = 0x8263D598;
	sub_82466E20(ctx, base);
	// 8263D598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D59C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D5A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D5A8 size=112
    let mut pc: u32 = 0x8263D5A8;
    'dispatch: loop {
        match pc {
            0x8263D5A8 => {
    //   block [0x8263D5A8..0x8263D618)
	// 8263D5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D5B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D5B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D5BC: 38AAC31C  addi r5, r10, -0x3ce4
	ctx.r[5].s64 = ctx.r[10].s64 + -15588;
	// 8263D5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D5C4: 390B73BC  addi r8, r11, 0x73bc
	ctx.r[8].s64 = ctx.r[11].s64 + 29628;
	// 8263D5C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263D5CC: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8263D5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D5D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D5E0: 386AC16C  addi r3, r10, -0x3e94
	ctx.r[3].s64 = ctx.r[10].s64 + -16020;
	// 8263D5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D604: 4BE2981D  bl 0x82466e20
	ctx.lr = 0x8263D608;
	sub_82466E20(ctx, base);
	// 8263D608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D618 size=108
    let mut pc: u32 = 0x8263D618;
    'dispatch: loop {
        match pc {
            0x8263D618 => {
    //   block [0x8263D618..0x8263D684)
	// 8263D618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D624: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D628: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D62C: 38EB73F0  addi r7, r11, 0x73f0
	ctx.r[7].s64 = ctx.r[11].s64 + 29680;
	// 8263D630: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263D634: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 8263D638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D63C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D648: 386AC19C  addi r3, r10, -0x3e64
	ctx.r[3].s64 = ctx.r[10].s64 + -15972;
	// 8263D64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D670: 4BE297B1  bl 0x82466e20
	ctx.lr = 0x8263D674;
	sub_82466E20(ctx, base);
	// 8263D674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D688 size=112
    let mut pc: u32 = 0x8263D688;
    'dispatch: loop {
        match pc {
            0x8263D688 => {
    //   block [0x8263D688..0x8263D6F8)
	// 8263D688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D694: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D698: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D69C: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263D6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D6A4: 390B7438  addi r8, r11, 0x7438
	ctx.r[8].s64 = ctx.r[11].s64 + 29752;
	// 8263D6A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263D6AC: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 8263D6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D6B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D6C0: 386AC1CC  addi r3, r10, -0x3e34
	ctx.r[3].s64 = ctx.r[10].s64 + -15924;
	// 8263D6C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D6E4: 4BE2973D  bl 0x82466e20
	ctx.lr = 0x8263D6E8;
	sub_82466E20(ctx, base);
	// 8263D6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D6F8 size=100
    let mut pc: u32 = 0x8263D6F8;
    'dispatch: loop {
        match pc {
            0x8263D6F8 => {
    //   block [0x8263D6F8..0x8263D75C)
	// 8263D6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D704: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D70C: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263D710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D718: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8263D71C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D72C: 386AC1FC  addi r3, r10, -0x3e04
	ctx.r[3].s64 = ctx.r[10].s64 + -15876;
	// 8263D730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D734: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263D73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D748: 4BE296D9  bl 0x82466e20
	ctx.lr = 0x8263D74C;
	sub_82466E20(ctx, base);
	// 8263D74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D760 size=112
    let mut pc: u32 = 0x8263D760;
    'dispatch: loop {
        match pc {
            0x8263D760 => {
    //   block [0x8263D760..0x8263D7D0)
	// 8263D760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D76C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D770: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D774: 38AABF5C  addi r5, r10, -0x40a4
	ctx.r[5].s64 = ctx.r[10].s64 + -16548;
	// 8263D778: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D77C: 390B7498  addi r8, r11, 0x7498
	ctx.r[8].s64 = ctx.r[11].s64 + 29848;
	// 8263D780: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263D784: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8263D788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D78C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D798: 386AC22C  addi r3, r10, -0x3dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -15828;
	// 8263D79C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D7BC: 4BE29665  bl 0x82466e20
	ctx.lr = 0x8263D7C0;
	sub_82466E20(ctx, base);
	// 8263D7C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D7D0 size=112
    let mut pc: u32 = 0x8263D7D0;
    'dispatch: loop {
        match pc {
            0x8263D7D0 => {
    //   block [0x8263D7D0..0x8263D840)
	// 8263D7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D7DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D7E0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D7E4: 38AABF5C  addi r5, r10, -0x40a4
	ctx.r[5].s64 = ctx.r[10].s64 + -16548;
	// 8263D7E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D7EC: 390B74E0  addi r8, r11, 0x74e0
	ctx.r[8].s64 = ctx.r[11].s64 + 29920;
	// 8263D7F0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8263D7F4: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8263D7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D7FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D808: 386AC25C  addi r3, r10, -0x3da4
	ctx.r[3].s64 = ctx.r[10].s64 + -15780;
	// 8263D80C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263D810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D82C: 4BE295F5  bl 0x82466e20
	ctx.lr = 0x8263D830;
	sub_82466E20(ctx, base);
	// 8263D830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D840 size=108
    let mut pc: u32 = 0x8263D840;
    'dispatch: loop {
        match pc {
            0x8263D840 => {
    //   block [0x8263D840..0x8263D8AC)
	// 8263D840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D84C: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D854: 38EB7588  addi r7, r11, 0x7588
	ctx.r[7].s64 = ctx.r[11].s64 + 30088;
	// 8263D858: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8263D85C: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8263D860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263D86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D870: 386AC28C  addi r3, r10, -0x3d74
	ctx.r[3].s64 = ctx.r[10].s64 + -15732;
	// 8263D874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263D878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263D898: 4BE29589  bl 0x82466e20
	ctx.lr = 0x8263D89C;
	sub_82466E20(ctx, base);
	// 8263D89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263D8B0 size=24
    let mut pc: u32 = 0x8263D8B0;
    'dispatch: loop {
        match pc {
            0x8263D8B0 => {
    //   block [0x8263D8B0..0x8263D8C8)
	// 8263D8B0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263D8B4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D8B8: 394ABB78  addi r10, r10, -0x4488
	ctx.r[10].s64 = ctx.r[10].s64 + -17544;
	// 8263D8BC: 816B73EC  lwz r11, 0x73ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29676 as u32) ) } as u64;
	// 8263D8C0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8263D8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D8C8 size=116
    let mut pc: u32 = 0x8263D8C8;
    'dispatch: loop {
        match pc {
            0x8263D8C8 => {
    //   block [0x8263D8C8..0x8263D93C)
	// 8263D8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D8D4: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263D8D8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263D8DC: 390ABB78  addi r8, r10, -0x4488
	ctx.r[8].s64 = ctx.r[10].s64 + -17544;
	// 8263D8E0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D8E4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263D8E8: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263D8EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D8F0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263D8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D8F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263D8FC: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8263D900: 396B6A34  addi r11, r11, 0x6a34
	ctx.r[11].s64 = ctx.r[11].s64 + 27188;
	// 8263D904: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D908: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D90C: 386AC2BC  addi r3, r10, -0x3d44
	ctx.r[3].s64 = ctx.r[10].s64 + -15684;
	// 8263D910: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263D914: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D918: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263D91C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D928: 4BE294F9  bl 0x82466e20
	ctx.lr = 0x8263D92C;
	sub_82466E20(ctx, base);
	// 8263D92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D940 size=100
    let mut pc: u32 = 0x8263D940;
    'dispatch: loop {
        match pc {
            0x8263D940 => {
    //   block [0x8263D940..0x8263D9A4)
	// 8263D940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D94C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D954: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263D958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D960: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8263D964: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D974: 386AC2EC  addi r3, r10, -0x3d14
	ctx.r[3].s64 = ctx.r[10].s64 + -15636;
	// 8263D978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D97C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D980: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263D984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D988: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D990: 4BE29491  bl 0x82466e20
	ctx.lr = 0x8263D994;
	sub_82466E20(ctx, base);
	// 8263D994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263D998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263D99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263D9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263D9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263D9A8 size=100
    let mut pc: u32 = 0x8263D9A8;
    'dispatch: loop {
        match pc {
            0x8263D9A8 => {
    //   block [0x8263D9A8..0x8263DA0C)
	// 8263D9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263D9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263D9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263D9B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263D9BC: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263D9C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263D9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263D9C8: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8263D9CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263D9D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263D9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263D9D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263D9DC: 386AC31C  addi r3, r10, -0x3ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -15588;
	// 8263D9E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263D9E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263D9E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263D9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263D9F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263D9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263D9F8: 4BE29429  bl 0x82466e20
	ctx.lr = 0x8263D9FC;
	sub_82466E20(ctx, base);
	// 8263D9FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DA10 size=112
    let mut pc: u32 = 0x8263DA10;
    'dispatch: loop {
        match pc {
            0x8263DA10 => {
    //   block [0x8263DA10..0x8263DA80)
	// 8263DA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DA1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DA20: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DA24: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263DA28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DA2C: 390B75E8  addi r8, r11, 0x75e8
	ctx.r[8].s64 = ctx.r[11].s64 + 30184;
	// 8263DA30: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8263DA34: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 8263DA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DA3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DA48: 386AC34C  addi r3, r10, -0x3cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -15540;
	// 8263DA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DA6C: 4BE293B5  bl 0x82466e20
	ctx.lr = 0x8263DA70;
	sub_82466E20(ctx, base);
	// 8263DA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DA80 size=112
    let mut pc: u32 = 0x8263DA80;
    'dispatch: loop {
        match pc {
            0x8263DA80 => {
    //   block [0x8263DA80..0x8263DAF0)
	// 8263DA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DA8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DA90: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DA94: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263DA98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DA9C: 390B7678  addi r8, r11, 0x7678
	ctx.r[8].s64 = ctx.r[11].s64 + 30328;
	// 8263DAA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263DAA4: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 8263DAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DAAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DAB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DAB8: 386AC37C  addi r3, r10, -0x3c84
	ctx.r[3].s64 = ctx.r[10].s64 + -15492;
	// 8263DABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DADC: 4BE29345  bl 0x82466e20
	ctx.lr = 0x8263DAE0;
	sub_82466E20(ctx, base);
	// 8263DAE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DAF0 size=112
    let mut pc: u32 = 0x8263DAF0;
    'dispatch: loop {
        match pc {
            0x8263DAF0 => {
    //   block [0x8263DAF0..0x8263DB60)
	// 8263DAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DAFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DB00: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DB04: 38AABEFC  addi r5, r10, -0x4104
	ctx.r[5].s64 = ctx.r[10].s64 + -16644;
	// 8263DB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DB0C: 390B76D8  addi r8, r11, 0x76d8
	ctx.r[8].s64 = ctx.r[11].s64 + 30424;
	// 8263DB10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263DB14: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 8263DB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DB1C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DB28: 386AC3AC  addi r3, r10, -0x3c54
	ctx.r[3].s64 = ctx.r[10].s64 + -15444;
	// 8263DB2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DB4C: 4BE292D5  bl 0x82466e20
	ctx.lr = 0x8263DB50;
	sub_82466E20(ctx, base);
	// 8263DB50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DB60 size=112
    let mut pc: u32 = 0x8263DB60;
    'dispatch: loop {
        match pc {
            0x8263DB60 => {
    //   block [0x8263DB60..0x8263DBD0)
	// 8263DB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DB6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DB70: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DB74: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263DB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DB7C: 390B7708  addi r8, r11, 0x7708
	ctx.r[8].s64 = ctx.r[11].s64 + 30472;
	// 8263DB80: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8263DB84: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8263DB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DB8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DB98: 386AC3DC  addi r3, r10, -0x3c24
	ctx.r[3].s64 = ctx.r[10].s64 + -15396;
	// 8263DB9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DBBC: 4BE29265  bl 0x82466e20
	ctx.lr = 0x8263DBC0;
	sub_82466E20(ctx, base);
	// 8263DBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DBD0 size=112
    let mut pc: u32 = 0x8263DBD0;
    'dispatch: loop {
        match pc {
            0x8263DBD0 => {
    //   block [0x8263DBD0..0x8263DC40)
	// 8263DBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DBDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DBE0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DBE4: 38AAC04C  addi r5, r10, -0x3fb4
	ctx.r[5].s64 = ctx.r[10].s64 + -16308;
	// 8263DBE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DBEC: 390B7798  addi r8, r11, 0x7798
	ctx.r[8].s64 = ctx.r[11].s64 + 30616;
	// 8263DBF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263DBF4: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8263DBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DBFC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DC00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DC08: 386AC40C  addi r3, r10, -0x3bf4
	ctx.r[3].s64 = ctx.r[10].s64 + -15348;
	// 8263DC0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DC14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DC2C: 4BE291F5  bl 0x82466e20
	ctx.lr = 0x8263DC30;
	sub_82466E20(ctx, base);
	// 8263DC30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DC40 size=112
    let mut pc: u32 = 0x8263DC40;
    'dispatch: loop {
        match pc {
            0x8263DC40 => {
    //   block [0x8263DC40..0x8263DCB0)
	// 8263DC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DC4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DC50: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DC54: 38AAC25C  addi r5, r10, -0x3da4
	ctx.r[5].s64 = ctx.r[10].s64 + -15780;
	// 8263DC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DC5C: 390B77B0  addi r8, r11, 0x77b0
	ctx.r[8].s64 = ctx.r[11].s64 + 30640;
	// 8263DC60: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263DC64: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8263DC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DC6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DC70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DC78: 386AC43C  addi r3, r10, -0x3bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -15300;
	// 8263DC7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DC9C: 4BE29185  bl 0x82466e20
	ctx.lr = 0x8263DCA0;
	sub_82466E20(ctx, base);
	// 8263DCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DCB0 size=112
    let mut pc: u32 = 0x8263DCB0;
    'dispatch: loop {
        match pc {
            0x8263DCB0 => {
    //   block [0x8263DCB0..0x8263DD20)
	// 8263DCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DCBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DCC0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DCC4: 38AABB3C  addi r5, r10, -0x44c4
	ctx.r[5].s64 = ctx.r[10].s64 + -17604;
	// 8263DCC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DCCC: 390B77E0  addi r8, r11, 0x77e0
	ctx.r[8].s64 = ctx.r[11].s64 + 30688;
	// 8263DCD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263DCD4: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8263DCD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DCDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DCE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DCE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DCE8: 386AC46C  addi r3, r10, -0x3b94
	ctx.r[3].s64 = ctx.r[10].s64 + -15252;
	// 8263DCEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DCF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DCF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DD00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DD04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DD08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DD0C: 4BE29115  bl 0x82466e20
	ctx.lr = 0x8263DD10;
	sub_82466E20(ctx, base);
	// 8263DD10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DD14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DD18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263DD20 size=24
    let mut pc: u32 = 0x8263DD20;
    'dispatch: loop {
        match pc {
            0x8263DD20 => {
    //   block [0x8263DD20..0x8263DD38)
	// 8263DD20: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DD24: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263DD28: 394ABBF0  addi r10, r10, -0x4410
	ctx.r[10].s64 = ctx.r[10].s64 + -17424;
	// 8263DD2C: 816B73EC  lwz r11, 0x73ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29676 as u32) ) } as u64;
	// 8263DD30: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8263DD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DD38 size=116
    let mut pc: u32 = 0x8263DD38;
    'dispatch: loop {
        match pc {
            0x8263DD38 => {
    //   block [0x8263DD38..0x8263DDAC)
	// 8263DD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DD44: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263DD48: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8263DD4C: 390ABBF0  addi r8, r10, -0x4410
	ctx.r[8].s64 = ctx.r[10].s64 + -17424;
	// 8263DD50: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DD54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263DD58: 38AABCEC  addi r5, r10, -0x4314
	ctx.r[5].s64 = ctx.r[10].s64 + -17172;
	// 8263DD5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DD60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263DD64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DD68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DD6C: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8263DD70: 396B6A4C  addi r11, r11, 0x6a4c
	ctx.r[11].s64 = ctx.r[11].s64 + 27212;
	// 8263DD74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DD78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DD7C: 386AC49C  addi r3, r10, -0x3b64
	ctx.r[3].s64 = ctx.r[10].s64 + -15204;
	// 8263DD80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263DD84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DD88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263DD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DD90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DD94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DD98: 4BE29089  bl 0x82466e20
	ctx.lr = 0x8263DD9C;
	sub_82466E20(ctx, base);
	// 8263DD9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DDA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DDA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DDA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DDB0 size=112
    let mut pc: u32 = 0x8263DDB0;
    'dispatch: loop {
        match pc {
            0x8263DDB0 => {
    //   block [0x8263DDB0..0x8263DE20)
	// 8263DDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DDBC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DDC0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DDC4: 38AABC5C  addi r5, r10, -0x43a4
	ctx.r[5].s64 = ctx.r[10].s64 + -17316;
	// 8263DDC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DDCC: 390B7828  addi r8, r11, 0x7828
	ctx.r[8].s64 = ctx.r[11].s64 + 30760;
	// 8263DDD0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263DDD4: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8263DDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DDDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DDE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DDE8: 386AC4CC  addi r3, r10, -0x3b34
	ctx.r[3].s64 = ctx.r[10].s64 + -15156;
	// 8263DDEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DE0C: 4BE29015  bl 0x82466e20
	ctx.lr = 0x8263DE10;
	sub_82466E20(ctx, base);
	// 8263DE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DE20 size=112
    let mut pc: u32 = 0x8263DE20;
    'dispatch: loop {
        match pc {
            0x8263DE20 => {
    //   block [0x8263DE20..0x8263DE90)
	// 8263DE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DE2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DE30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DE34: 38AABCBC  addi r5, r10, -0x4344
	ctx.r[5].s64 = ctx.r[10].s64 + -17220;
	// 8263DE38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DE3C: 390B7858  addi r8, r11, 0x7858
	ctx.r[8].s64 = ctx.r[11].s64 + 30808;
	// 8263DE40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263DE44: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8263DE48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DE4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DE50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DE58: 386AC4FC  addi r3, r10, -0x3b04
	ctx.r[3].s64 = ctx.r[10].s64 + -15108;
	// 8263DE5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DE60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DE68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DE70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DE78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DE7C: 4BE28FA5  bl 0x82466e20
	ctx.lr = 0x8263DE80;
	sub_82466E20(ctx, base);
	// 8263DE80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DE84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DE88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DE90 size=100
    let mut pc: u32 = 0x8263DE90;
    'dispatch: loop {
        match pc {
            0x8263DE90 => {
    //   block [0x8263DE90..0x8263DEF4)
	// 8263DE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DE9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263DEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DEA4: 392A6ABC  addi r9, r10, 0x6abc
	ctx.r[9].s64 = ctx.r[10].s64 + 27324;
	// 8263DEA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DEB0: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 8263DEB4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DEB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DEC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DEC4: 386AC52C  addi r3, r10, -0x3ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -15060;
	// 8263DEC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DECC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8263DED0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263DED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DED8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263DEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263DEE0: 4BE28F41  bl 0x82466e20
	ctx.lr = 0x8263DEE4;
	sub_82466E20(ctx, base);
	// 8263DEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263DEF8 size=24
    let mut pc: u32 = 0x8263DEF8;
    'dispatch: loop {
        match pc {
            0x8263DEF8 => {
    //   block [0x8263DEF8..0x8263DF10)
	// 8263DEF8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DEFC: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263DF00: 394ABC98  addi r10, r10, -0x4368
	ctx.r[10].s64 = ctx.r[10].s64 + -17256;
	// 8263DF04: 816B7890  lwz r11, 0x7890(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30864 as u32) ) } as u64;
	// 8263DF08: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263DF0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DF10 size=112
    let mut pc: u32 = 0x8263DF10;
    'dispatch: loop {
        match pc {
            0x8263DF10 => {
    //   block [0x8263DF10..0x8263DF80)
	// 8263DF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DF1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263DF20: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263DF24: 392A6BF8  addi r9, r10, 0x6bf8
	ctx.r[9].s64 = ctx.r[10].s64 + 27640;
	// 8263DF28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DF2C: 390BBC98  addi r8, r11, -0x4368
	ctx.r[8].s64 = ctx.r[11].s64 + -17256;
	// 8263DF30: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8263DF34: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 8263DF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DF3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DF40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DF48: 386AC55C  addi r3, r10, -0x3aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -15012;
	// 8263DF4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263DF50: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8263DF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DF64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263DF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DF6C: 4BE28EB5  bl 0x82466e20
	ctx.lr = 0x8263DF70;
	sub_82466E20(ctx, base);
	// 8263DF70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DF80 size=112
    let mut pc: u32 = 0x8263DF80;
    'dispatch: loop {
        match pc {
            0x8263DF80 => {
    //   block [0x8263DF80..0x8263DFF0)
	// 8263DF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DF8C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DF90: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263DF94: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263DF98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263DF9C: 390B7898  addi r8, r11, 0x7898
	ctx.r[8].s64 = ctx.r[11].s64 + 30872;
	// 8263DFA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263DFA4: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 8263DFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263DFAC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263DFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263DFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263DFB8: 386AC58C  addi r3, r10, -0x3a74
	ctx.r[3].s64 = ctx.r[10].s64 + -14964;
	// 8263DFBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263DFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263DFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263DFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263DFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263DFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263DFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263DFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263DFDC: 4BE28E45  bl 0x82466e20
	ctx.lr = 0x8263DFE0;
	sub_82466E20(ctx, base);
	// 8263DFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263DFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263DFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263DFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263DFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263DFF0 size=108
    let mut pc: u32 = 0x8263DFF0;
    'dispatch: loop {
        match pc {
            0x8263DFF0 => {
    //   block [0x8263DFF0..0x8263E05C)
	// 8263DFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263DFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263DFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263DFFC: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E004: 38EB78C8  addi r7, r11, 0x78c8
	ctx.r[7].s64 = ctx.r[11].s64 + 30920;
	// 8263E008: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263E00C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 8263E010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E014: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E018: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263E01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E020: 386AC5BC  addi r3, r10, -0x3a44
	ctx.r[3].s64 = ctx.r[10].s64 + -14916;
	// 8263E024: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263E028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263E048: 4BE28DD9  bl 0x82466e20
	ctx.lr = 0x8263E04C;
	sub_82466E20(ctx, base);
	// 8263E04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E060 size=100
    let mut pc: u32 = 0x8263E060;
    'dispatch: loop {
        match pc {
            0x8263E060 => {
    //   block [0x8263E060..0x8263E0C4)
	// 8263E060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E06C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E074: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E080: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 8263E084: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E08C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E094: 386AC5EC  addi r3, r10, -0x3a14
	ctx.r[3].s64 = ctx.r[10].s64 + -14868;
	// 8263E098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E0A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263E0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E0A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263E0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E0B0: 4BE28D71  bl 0x82466e20
	ctx.lr = 0x8263E0B4;
	sub_82466E20(ctx, base);
	// 8263E0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E0C8 size=112
    let mut pc: u32 = 0x8263E0C8;
    'dispatch: loop {
        match pc {
            0x8263E0C8 => {
    //   block [0x8263E0C8..0x8263E138)
	// 8263E0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E0D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E0D8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E0DC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E0E4: 390B78E0  addi r8, r11, 0x78e0
	ctx.r[8].s64 = ctx.r[11].s64 + 30944;
	// 8263E0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263E0EC: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 8263E0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E0F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E100: 386AC61C  addi r3, r10, -0x39e4
	ctx.r[3].s64 = ctx.r[10].s64 + -14820;
	// 8263E104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E124: 4BE28CFD  bl 0x82466e20
	ctx.lr = 0x8263E128;
	sub_82466E20(ctx, base);
	// 8263E128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E138 size=112
    let mut pc: u32 = 0x8263E138;
    'dispatch: loop {
        match pc {
            0x8263E138 => {
    //   block [0x8263E138..0x8263E1A8)
	// 8263E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E144: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E148: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E14C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E154: 390B78F8  addi r8, r11, 0x78f8
	ctx.r[8].s64 = ctx.r[11].s64 + 30968;
	// 8263E158: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263E15C: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 8263E160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E164: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E170: 386AC64C  addi r3, r10, -0x39b4
	ctx.r[3].s64 = ctx.r[10].s64 + -14772;
	// 8263E174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E194: 4BE28C8D  bl 0x82466e20
	ctx.lr = 0x8263E198;
	sub_82466E20(ctx, base);
	// 8263E198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E1A8 size=112
    let mut pc: u32 = 0x8263E1A8;
    'dispatch: loop {
        match pc {
            0x8263E1A8 => {
    //   block [0x8263E1A8..0x8263E218)
	// 8263E1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E1B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E1B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E1BC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E1C4: 390B7928  addi r8, r11, 0x7928
	ctx.r[8].s64 = ctx.r[11].s64 + 31016;
	// 8263E1C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263E1CC: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 8263E1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E1D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E1E0: 386AC67C  addi r3, r10, -0x3984
	ctx.r[3].s64 = ctx.r[10].s64 + -14724;
	// 8263E1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E204: 4BE28C1D  bl 0x82466e20
	ctx.lr = 0x8263E208;
	sub_82466E20(ctx, base);
	// 8263E208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E218 size=112
    let mut pc: u32 = 0x8263E218;
    'dispatch: loop {
        match pc {
            0x8263E218 => {
    //   block [0x8263E218..0x8263E288)
	// 8263E218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E224: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E228: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E22C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E234: 390B7958  addi r8, r11, 0x7958
	ctx.r[8].s64 = ctx.r[11].s64 + 31064;
	// 8263E238: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263E23C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 8263E240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E244: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E250: 386AC6AC  addi r3, r10, -0x3954
	ctx.r[3].s64 = ctx.r[10].s64 + -14676;
	// 8263E254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E274: 4BE28BAD  bl 0x82466e20
	ctx.lr = 0x8263E278;
	sub_82466E20(ctx, base);
	// 8263E278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E288 size=112
    let mut pc: u32 = 0x8263E288;
    'dispatch: loop {
        match pc {
            0x8263E288 => {
    //   block [0x8263E288..0x8263E2F8)
	// 8263E288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E294: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E298: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E29C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E2A4: 390B7988  addi r8, r11, 0x7988
	ctx.r[8].s64 = ctx.r[11].s64 + 31112;
	// 8263E2A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263E2AC: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 8263E2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E2B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E2C0: 386AC6DC  addi r3, r10, -0x3924
	ctx.r[3].s64 = ctx.r[10].s64 + -14628;
	// 8263E2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E2E4: 4BE28B3D  bl 0x82466e20
	ctx.lr = 0x8263E2E8;
	sub_82466E20(ctx, base);
	// 8263E2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E2F8 size=112
    let mut pc: u32 = 0x8263E2F8;
    'dispatch: loop {
        match pc {
            0x8263E2F8 => {
    //   block [0x8263E2F8..0x8263E368)
	// 8263E2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E304: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E308: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E30C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E314: 390B79A0  addi r8, r11, 0x79a0
	ctx.r[8].s64 = ctx.r[11].s64 + 31136;
	// 8263E318: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263E31C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 8263E320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E324: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E330: 386AC70C  addi r3, r10, -0x38f4
	ctx.r[3].s64 = ctx.r[10].s64 + -14580;
	// 8263E334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E354: 4BE28ACD  bl 0x82466e20
	ctx.lr = 0x8263E358;
	sub_82466E20(ctx, base);
	// 8263E358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E368 size=112
    let mut pc: u32 = 0x8263E368;
    'dispatch: loop {
        match pc {
            0x8263E368 => {
    //   block [0x8263E368..0x8263E3D8)
	// 8263E368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E374: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E378: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E37C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E384: 390B79B8  addi r8, r11, 0x79b8
	ctx.r[8].s64 = ctx.r[11].s64 + 31160;
	// 8263E388: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263E38C: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 8263E390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E394: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E3A0: 386AC73C  addi r3, r10, -0x38c4
	ctx.r[3].s64 = ctx.r[10].s64 + -14532;
	// 8263E3A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E3C4: 4BE28A5D  bl 0x82466e20
	ctx.lr = 0x8263E3C8;
	sub_82466E20(ctx, base);
	// 8263E3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E3D8 size=112
    let mut pc: u32 = 0x8263E3D8;
    'dispatch: loop {
        match pc {
            0x8263E3D8 => {
    //   block [0x8263E3D8..0x8263E448)
	// 8263E3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E3E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E3E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E3EC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E3F4: 390B7A00  addi r8, r11, 0x7a00
	ctx.r[8].s64 = ctx.r[11].s64 + 31232;
	// 8263E3F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263E3FC: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 8263E400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E404: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E410: 386AC76C  addi r3, r10, -0x3894
	ctx.r[3].s64 = ctx.r[10].s64 + -14484;
	// 8263E414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E434: 4BE289ED  bl 0x82466e20
	ctx.lr = 0x8263E438;
	sub_82466E20(ctx, base);
	// 8263E438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E448 size=112
    let mut pc: u32 = 0x8263E448;
    'dispatch: loop {
        match pc {
            0x8263E448 => {
    //   block [0x8263E448..0x8263E4B8)
	// 8263E448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E458: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E45C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E464: 390B7A48  addi r8, r11, 0x7a48
	ctx.r[8].s64 = ctx.r[11].s64 + 31304;
	// 8263E468: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263E46C: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 8263E470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E474: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E478: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E480: 386AC79C  addi r3, r10, -0x3864
	ctx.r[3].s64 = ctx.r[10].s64 + -14436;
	// 8263E484: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E488: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E4A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E4A4: 4BE2897D  bl 0x82466e20
	ctx.lr = 0x8263E4A8;
	sub_82466E20(ctx, base);
	// 8263E4A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E4AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E4B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E4B8 size=112
    let mut pc: u32 = 0x8263E4B8;
    'dispatch: loop {
        match pc {
            0x8263E4B8 => {
    //   block [0x8263E4B8..0x8263E528)
	// 8263E4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E4C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E4C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E4CC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E4D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E4D4: 390B7A60  addi r8, r11, 0x7a60
	ctx.r[8].s64 = ctx.r[11].s64 + 31328;
	// 8263E4D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263E4DC: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 8263E4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E4E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E4F0: 386AC7CC  addi r3, r10, -0x3834
	ctx.r[3].s64 = ctx.r[10].s64 + -14388;
	// 8263E4F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E514: 4BE2890D  bl 0x82466e20
	ctx.lr = 0x8263E518;
	sub_82466E20(ctx, base);
	// 8263E518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E51C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E520: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E528 size=116
    let mut pc: u32 = 0x8263E528;
    'dispatch: loop {
        match pc {
            0x8263E528 => {
    //   block [0x8263E528..0x8263E59C)
	// 8263E528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E534: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8263E538: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263E53C: 390A7A90  addi r8, r10, 0x7a90
	ctx.r[8].s64 = ctx.r[10].s64 + 31376;
	// 8263E540: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E544: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263E548: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E54C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E550: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263E554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E55C: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 8263E560: 396B6C20  addi r11, r11, 0x6c20
	ctx.r[11].s64 = ctx.r[11].s64 + 27680;
	// 8263E564: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E568: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E56C: 386AC7FC  addi r3, r10, -0x3804
	ctx.r[3].s64 = ctx.r[10].s64 + -14340;
	// 8263E570: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263E574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E578: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263E57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E588: 4BE28899  bl 0x82466e20
	ctx.lr = 0x8263E58C;
	sub_82466E20(ctx, base);
	// 8263E58C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E5A0 size=116
    let mut pc: u32 = 0x8263E5A0;
    'dispatch: loop {
        match pc {
            0x8263E5A0 => {
    //   block [0x8263E5A0..0x8263E614)
	// 8263E5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E5AC: 3D408277  lis r10, -0x7d89
	ctx.r[10].s64 = -2106130432;
	// 8263E5B0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8263E5B4: 390A7B08  addi r8, r10, 0x7b08
	ctx.r[8].s64 = ctx.r[10].s64 + 31496;
	// 8263E5B8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E5BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263E5C0: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E5C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E5C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263E5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E5D4: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 8263E5D8: 396B6C38  addi r11, r11, 0x6c38
	ctx.r[11].s64 = ctx.r[11].s64 + 27704;
	// 8263E5DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E5E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E5E4: 386AC82C  addi r3, r10, -0x37d4
	ctx.r[3].s64 = ctx.r[10].s64 + -14292;
	// 8263E5E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8263E5EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E5F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8263E5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E600: 4BE28821  bl 0x82466e20
	ctx.lr = 0x8263E604;
	sub_82466E20(ctx, base);
	// 8263E604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263E618 size=24
    let mut pc: u32 = 0x8263E618;
    'dispatch: loop {
        match pc {
            0x8263E618 => {
    //   block [0x8263E618..0x8263E630)
	// 8263E618: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E61C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263E620: 394ABCB0  addi r10, r10, -0x4350
	ctx.r[10].s64 = ctx.r[10].s64 + -17232;
	// 8263E624: 816B7B98  lwz r11, 0x7b98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31640 as u32) ) } as u64;
	// 8263E628: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8263E62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E630 size=116
    let mut pc: u32 = 0x8263E630;
    'dispatch: loop {
        match pc {
            0x8263E630 => {
    //   block [0x8263E630..0x8263E6A4)
	// 8263E630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E63C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8263E640: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E644: 392B6C64  addi r9, r11, 0x6c64
	ctx.r[9].s64 = ctx.r[11].s64 + 27748;
	// 8263E648: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E64C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E650: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8263E654: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8263E658: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263E65C: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 8263E660: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E664: 396BBCB0  addi r11, r11, -0x4350
	ctx.r[11].s64 = ctx.r[11].s64 + -17232;
	// 8263E668: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8263E66C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E670: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8263E674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E678: 386AC85C  addi r3, r10, -0x37a4
	ctx.r[3].s64 = ctx.r[10].s64 + -14244;
	// 8263E67C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263E680: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8263E684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E688: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8263E68C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263E690: 4BE28791  bl 0x82466e20
	ctx.lr = 0x8263E694;
	sub_82466E20(ctx, base);
	// 8263E694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E69C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E6A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E6A8 size=112
    let mut pc: u32 = 0x8263E6A8;
    'dispatch: loop {
        match pc {
            0x8263E6A8 => {
    //   block [0x8263E6A8..0x8263E718)
	// 8263E6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E6B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E6B8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E6BC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E6C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E6C4: 390B7BA0  addi r8, r11, 0x7ba0
	ctx.r[8].s64 = ctx.r[11].s64 + 31648;
	// 8263E6C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263E6CC: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 8263E6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E6D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E6D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E6E0: 386AC88C  addi r3, r10, -0x3774
	ctx.r[3].s64 = ctx.r[10].s64 + -14196;
	// 8263E6E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E6EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E704: 4BE2871D  bl 0x82466e20
	ctx.lr = 0x8263E708;
	sub_82466E20(ctx, base);
	// 8263E708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E718 size=112
    let mut pc: u32 = 0x8263E718;
    'dispatch: loop {
        match pc {
            0x8263E718 => {
    //   block [0x8263E718..0x8263E788)
	// 8263E718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E724: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E728: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E72C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E734: 390B7C00  addi r8, r11, 0x7c00
	ctx.r[8].s64 = ctx.r[11].s64 + 31744;
	// 8263E738: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8263E73C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 8263E740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E744: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E750: 386AC8BC  addi r3, r10, -0x3744
	ctx.r[3].s64 = ctx.r[10].s64 + -14148;
	// 8263E754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E774: 4BE286AD  bl 0x82466e20
	ctx.lr = 0x8263E778;
	sub_82466E20(ctx, base);
	// 8263E778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E788 size=112
    let mut pc: u32 = 0x8263E788;
    'dispatch: loop {
        match pc {
            0x8263E788 => {
    //   block [0x8263E788..0x8263E7F8)
	// 8263E788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E794: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E798: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E79C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E7A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E7A4: 390B7CA8  addi r8, r11, 0x7ca8
	ctx.r[8].s64 = ctx.r[11].s64 + 31912;
	// 8263E7A8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263E7AC: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 8263E7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E7B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E7B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E7C0: 386AC8EC  addi r3, r10, -0x3714
	ctx.r[3].s64 = ctx.r[10].s64 + -14100;
	// 8263E7C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E7C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E7CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E7D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E7D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E7D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E7DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E7E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E7E4: 4BE2863D  bl 0x82466e20
	ctx.lr = 0x8263E7E8;
	sub_82466E20(ctx, base);
	// 8263E7E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E7F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E7F8 size=112
    let mut pc: u32 = 0x8263E7F8;
    'dispatch: loop {
        match pc {
            0x8263E7F8 => {
    //   block [0x8263E7F8..0x8263E868)
	// 8263E7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E804: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E808: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E80C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E814: 390B7D20  addi r8, r11, 0x7d20
	ctx.r[8].s64 = ctx.r[11].s64 + 32032;
	// 8263E818: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8263E81C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 8263E820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E824: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E828: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E830: 386AC91C  addi r3, r10, -0x36e4
	ctx.r[3].s64 = ctx.r[10].s64 + -14052;
	// 8263E834: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E83C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E854: 4BE285CD  bl 0x82466e20
	ctx.lr = 0x8263E858;
	sub_82466E20(ctx, base);
	// 8263E858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E868 size=112
    let mut pc: u32 = 0x8263E868;
    'dispatch: loop {
        match pc {
            0x8263E868 => {
    //   block [0x8263E868..0x8263E8D8)
	// 8263E868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E874: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E878: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E87C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E880: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E884: 390B7D68  addi r8, r11, 0x7d68
	ctx.r[8].s64 = ctx.r[11].s64 + 32104;
	// 8263E888: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8263E88C: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 8263E890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E894: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E8A0: 386AC94C  addi r3, r10, -0x36b4
	ctx.r[3].s64 = ctx.r[10].s64 + -14004;
	// 8263E8A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E8A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E8AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E8B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E8B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E8C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E8C4: 4BE2855D  bl 0x82466e20
	ctx.lr = 0x8263E8C8;
	sub_82466E20(ctx, base);
	// 8263E8C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E8D8 size=112
    let mut pc: u32 = 0x8263E8D8;
    'dispatch: loop {
        match pc {
            0x8263E8D8 => {
    //   block [0x8263E8D8..0x8263E948)
	// 8263E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E8E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E8E8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E8EC: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E8F4: 390B7DF8  addi r8, r11, 0x7df8
	ctx.r[8].s64 = ctx.r[11].s64 + 32248;
	// 8263E8F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263E8FC: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 8263E900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E904: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E908: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E910: 386AC97C  addi r3, r10, -0x3684
	ctx.r[3].s64 = ctx.r[10].s64 + -13956;
	// 8263E914: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E91C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E934: 4BE284ED  bl 0x82466e20
	ctx.lr = 0x8263E938;
	sub_82466E20(ctx, base);
	// 8263E938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E948 size=112
    let mut pc: u32 = 0x8263E948;
    'dispatch: loop {
        match pc {
            0x8263E948 => {
    //   block [0x8263E948..0x8263E9B8)
	// 8263E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E954: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E958: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E95C: 38AAC55C  addi r5, r10, -0x3aa4
	ctx.r[5].s64 = ctx.r[10].s64 + -15012;
	// 8263E960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E964: 390B7E58  addi r8, r11, 0x7e58
	ctx.r[8].s64 = ctx.r[11].s64 + 32344;
	// 8263E968: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263E96C: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 8263E970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E974: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E978: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E97C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E980: 386AC9AC  addi r3, r10, -0x3654
	ctx.r[3].s64 = ctx.r[10].s64 + -13908;
	// 8263E984: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263E990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263E994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263E998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263E99C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263E9A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263E9A4: 4BE2847D  bl 0x82466e20
	ctx.lr = 0x8263E9A8;
	sub_82466E20(ctx, base);
	// 8263E9A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263E9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263E9B8 size=112
    let mut pc: u32 = 0x8263E9B8;
    'dispatch: loop {
        match pc {
            0x8263E9B8 => {
    //   block [0x8263E9B8..0x8263EA28)
	// 8263E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263E9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263E9C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E9C8: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263E9CC: 38AAC9AC  addi r5, r10, -0x3654
	ctx.r[5].s64 = ctx.r[10].s64 + -13908;
	// 8263E9D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263E9D4: 390B7EB8  addi r8, r11, 0x7eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 32440;
	// 8263E9D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263E9DC: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 8263E9E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263E9E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263E9E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263E9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263E9F0: 386AC9DC  addi r3, r10, -0x3624
	ctx.r[3].s64 = ctx.r[10].s64 + -13860;
	// 8263E9F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263E9F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263E9FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EA00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EA08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EA0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EA10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EA14: 4BE2840D  bl 0x82466e20
	ctx.lr = 0x8263EA18;
	sub_82466E20(ctx, base);
	// 8263EA18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EA28 size=112
    let mut pc: u32 = 0x8263EA28;
    'dispatch: loop {
        match pc {
            0x8263EA28 => {
    //   block [0x8263EA28..0x8263EA98)
	// 8263EA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EA34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EA38: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EA3C: 38AAC9AC  addi r5, r10, -0x3654
	ctx.r[5].s64 = ctx.r[10].s64 + -13908;
	// 8263EA40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EA44: 390B7EE8  addi r8, r11, 0x7ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 32488;
	// 8263EA48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263EA4C: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 8263EA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EA54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EA58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EA60: 386ACA0C  addi r3, r10, -0x35f4
	ctx.r[3].s64 = ctx.r[10].s64 + -13812;
	// 8263EA64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263EA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EA84: 4BE2839D  bl 0x82466e20
	ctx.lr = 0x8263EA88;
	sub_82466E20(ctx, base);
	// 8263EA88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EA98 size=100
    let mut pc: u32 = 0x8263EA98;
    'dispatch: loop {
        match pc {
            0x8263EA98 => {
    //   block [0x8263EA98..0x8263EAFC)
	// 8263EA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EAA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EAAC: 38AAC9AC  addi r5, r10, -0x3654
	ctx.r[5].s64 = ctx.r[10].s64 + -13908;
	// 8263EAB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EAB8: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 8263EABC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EACC: 386ACA3C  addi r3, r10, -0x35c4
	ctx.r[3].s64 = ctx.r[10].s64 + -13764;
	// 8263EAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EAD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EAD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263EADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EAE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263EAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EAE8: 4BE28339  bl 0x82466e20
	ctx.lr = 0x8263EAEC;
	sub_82466E20(ctx, base);
	// 8263EAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EB00 size=112
    let mut pc: u32 = 0x8263EB00;
    'dispatch: loop {
        match pc {
            0x8263EB00 => {
    //   block [0x8263EB00..0x8263EB70)
	// 8263EB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EB0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EB10: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EB14: 38AAC9AC  addi r5, r10, -0x3654
	ctx.r[5].s64 = ctx.r[10].s64 + -13908;
	// 8263EB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EB1C: 390B7F18  addi r8, r11, 0x7f18
	ctx.r[8].s64 = ctx.r[11].s64 + 32536;
	// 8263EB20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263EB24: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 8263EB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EB2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EB30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EB38: 386ACA6C  addi r3, r10, -0x3594
	ctx.r[3].s64 = ctx.r[10].s64 + -13716;
	// 8263EB3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263EB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EB48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EB4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EB50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EB58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EB5C: 4BE282C5  bl 0x82466e20
	ctx.lr = 0x8263EB60;
	sub_82466E20(ctx, base);
	// 8263EB60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EB64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EB68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EB6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EB70 size=112
    let mut pc: u32 = 0x8263EB70;
    'dispatch: loop {
        match pc {
            0x8263EB70 => {
    //   block [0x8263EB70..0x8263EBE0)
	// 8263EB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EB7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EB80: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EB84: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263EB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EB8C: 390B7F30  addi r8, r11, 0x7f30
	ctx.r[8].s64 = ctx.r[11].s64 + 32560;
	// 8263EB90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8263EB94: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8263EB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EB9C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EBA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EBA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EBA8: 386ACA9C  addi r3, r10, -0x3564
	ctx.r[3].s64 = ctx.r[10].s64 + -13668;
	// 8263EBAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263EBB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EBB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EBB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EBBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EBC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EBC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EBC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EBCC: 4BE28255  bl 0x82466e20
	ctx.lr = 0x8263EBD0;
	sub_82466E20(ctx, base);
	// 8263EBD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EBD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EBD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EBDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EBE0 size=112
    let mut pc: u32 = 0x8263EBE0;
    'dispatch: loop {
        match pc {
            0x8263EBE0 => {
    //   block [0x8263EBE0..0x8263EC50)
	// 8263EBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EBEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EBF0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EBF4: 38AACA9C  addi r5, r10, -0x3564
	ctx.r[5].s64 = ctx.r[10].s64 + -13668;
	// 8263EBF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EBFC: 390B7F90  addi r8, r11, 0x7f90
	ctx.r[8].s64 = ctx.r[11].s64 + 32656;
	// 8263EC00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263EC04: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8263EC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EC0C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EC10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EC18: 386ACACC  addi r3, r10, -0x3534
	ctx.r[3].s64 = ctx.r[10].s64 + -13620;
	// 8263EC1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263EC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EC24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EC28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EC2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EC30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EC38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EC3C: 4BE281E5  bl 0x82466e20
	ctx.lr = 0x8263EC40;
	sub_82466E20(ctx, base);
	// 8263EC40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EC50 size=112
    let mut pc: u32 = 0x8263EC50;
    'dispatch: loop {
        match pc {
            0x8263EC50 => {
    //   block [0x8263EC50..0x8263ECC0)
	// 8263EC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EC5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EC60: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EC64: 38AACA9C  addi r5, r10, -0x3564
	ctx.r[5].s64 = ctx.r[10].s64 + -13668;
	// 8263EC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EC6C: 390B7FA8  addi r8, r11, 0x7fa8
	ctx.r[8].s64 = ctx.r[11].s64 + 32680;
	// 8263EC70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8263EC74: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8263EC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EC7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EC80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EC84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EC88: 386ACAFC  addi r3, r10, -0x3504
	ctx.r[3].s64 = ctx.r[10].s64 + -13572;
	// 8263EC8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263EC90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EC94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EC9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ECA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263ECA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263ECA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263ECAC: 4BE28175  bl 0x82466e20
	ctx.lr = 0x8263ECB0;
	sub_82466E20(ctx, base);
	// 8263ECB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263ECB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263ECB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263ECBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263ECC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263ECC0 size=112
    let mut pc: u32 = 0x8263ECC0;
    'dispatch: loop {
        match pc {
            0x8263ECC0 => {
    //   block [0x8263ECC0..0x8263ED30)
	// 8263ECC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263ECC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263ECC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263ECCC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ECD0: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263ECD4: 38AACA9C  addi r5, r10, -0x3564
	ctx.r[5].s64 = ctx.r[10].s64 + -13668;
	// 8263ECD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263ECDC: 390B7FD8  addi r8, r11, 0x7fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 32728;
	// 8263ECE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263ECE4: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8263ECE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263ECEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ECF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263ECF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263ECF8: 386ACB2C  addi r3, r10, -0x34d4
	ctx.r[3].s64 = ctx.r[10].s64 + -13524;
	// 8263ECFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263ED00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263ED04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263ED08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263ED0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ED10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263ED14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263ED18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263ED1C: 4BE28105  bl 0x82466e20
	ctx.lr = 0x8263ED20;
	sub_82466E20(ctx, base);
	// 8263ED20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263ED24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263ED28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263ED2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263ED30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263ED30 size=24
    let mut pc: u32 = 0x8263ED30;
    'dispatch: loop {
        match pc {
            0x8263ED30 => {
    //   block [0x8263ED30..0x8263ED48)
	// 8263ED30: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263ED34: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263ED38: 394ABD58  addi r10, r10, -0x42a8
	ctx.r[10].s64 = ctx.r[10].s64 + -17064;
	// 8263ED3C: 816B7B9C  lwz r11, 0x7b9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31644 as u32) ) } as u64;
	// 8263ED40: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8263ED44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263ED48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263ED48 size=112
    let mut pc: u32 = 0x8263ED48;
    'dispatch: loop {
        match pc {
            0x8263ED48 => {
    //   block [0x8263ED48..0x8263EDB8)
	// 8263ED48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263ED4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263ED50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263ED54: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263ED58: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263ED5C: 392A6CC0  addi r9, r10, 0x6cc0
	ctx.r[9].s64 = ctx.r[10].s64 + 27840;
	// 8263ED60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263ED64: 390BBD58  addi r8, r11, -0x42a8
	ctx.r[8].s64 = ctx.r[11].s64 + -17064;
	// 8263ED68: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263ED6C: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8263ED70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263ED74: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263ED78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263ED7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263ED80: 386ACB5C  addi r3, r10, -0x34a4
	ctx.r[3].s64 = ctx.r[10].s64 + -13476;
	// 8263ED84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263ED88: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263ED8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263ED90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263ED94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263ED98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263ED9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263EDA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EDA4: 4BE2807D  bl 0x82466e20
	ctx.lr = 0x8263EDA8;
	sub_82466E20(ctx, base);
	// 8263EDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EDB8 size=108
    let mut pc: u32 = 0x8263EDB8;
    'dispatch: loop {
        match pc {
            0x8263EDB8 => {
    //   block [0x8263EDB8..0x8263EE24)
	// 8263EDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EDC4: 3D608277  lis r11, -0x7d89
	ctx.r[11].s64 = -2106130432;
	// 8263EDC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EDCC: 38EB7FF0  addi r7, r11, 0x7ff0
	ctx.r[7].s64 = ctx.r[11].s64 + 32752;
	// 8263EDD0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8263EDD4: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8263EDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EDDC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EDE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263EDE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EDE8: 386ACB8C  addi r3, r10, -0x3474
	ctx.r[3].s64 = ctx.r[10].s64 + -13428;
	// 8263EDEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263EDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263EE10: 4BE28011  bl 0x82466e20
	ctx.lr = 0x8263EE14;
	sub_82466E20(ctx, base);
	// 8263EE14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EE28 size=108
    let mut pc: u32 = 0x8263EE28;
    'dispatch: loop {
        match pc {
            0x8263EE28 => {
    //   block [0x8263EE28..0x8263EE94)
	// 8263EE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EE34: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263EE38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EE3C: 38EB8008  addi r7, r11, -0x7ff8
	ctx.r[7].s64 = ctx.r[11].s64 + -32760;
	// 8263EE40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8263EE44: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8263EE48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EE4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EE50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263EE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EE58: 386ACBBC  addi r3, r10, -0x3444
	ctx.r[3].s64 = ctx.r[10].s64 + -13380;
	// 8263EE5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263EE60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EE64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EE68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EE70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EE78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EE7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263EE80: 4BE27FA1  bl 0x82466e20
	ctx.lr = 0x8263EE84;
	sub_82466E20(ctx, base);
	// 8263EE84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EE88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EE8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EE90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EE98 size=116
    let mut pc: u32 = 0x8263EE98;
    'dispatch: loop {
        match pc {
            0x8263EE98 => {
    //   block [0x8263EE98..0x8263EF0C)
	// 8263EE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EEA4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263EEA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263EEAC: 390B8054  addi r8, r11, -0x7fac
	ctx.r[8].s64 = ctx.r[11].s64 + -32684;
	// 8263EEB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EEB4: 392A6D78  addi r9, r10, 0x6d78
	ctx.r[9].s64 = ctx.r[10].s64 + 28024;
	// 8263EEB8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EEBC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8263EEC0: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263EEC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EECC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EEDC: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263EEE0: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8263EEE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263EEE8: 386BCBEC  addi r3, r11, -0x3414
	ctx.r[3].s64 = ctx.r[11].s64 + -13332;
	// 8263EEEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263EEF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EEF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EEF8: 4BE27F29  bl 0x82466e20
	ctx.lr = 0x8263EEFC;
	sub_82466E20(ctx, base);
	// 8263EEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263EF10 size=24
    let mut pc: u32 = 0x8263EF10;
    'dispatch: loop {
        match pc {
            0x8263EF10 => {
    //   block [0x8263EF10..0x8263EF28)
	// 8263EF10: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263EF14: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263EF18: 394ABDA0  addi r10, r10, -0x4260
	ctx.r[10].s64 = ctx.r[10].s64 + -16992;
	// 8263EF1C: 816B806C  lwz r11, -0x7f94(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32660 as u32) ) } as u64;
	// 8263EF20: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8263EF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EF28 size=116
    let mut pc: u32 = 0x8263EF28;
    'dispatch: loop {
        match pc {
            0x8263EF28 => {
    //   block [0x8263EF28..0x8263EF9C)
	// 8263EF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EF34: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263EF38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263EF3C: 390BBDA0  addi r8, r11, -0x4260
	ctx.r[8].s64 = ctx.r[11].s64 + -16992;
	// 8263EF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EF44: 392A6DD4  addi r9, r10, 0x6dd4
	ctx.r[9].s64 = ctx.r[10].s64 + 28116;
	// 8263EF48: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EF4C: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8263EF50: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263EF54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263EF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EF5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EF6C: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263EF70: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8263EF74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263EF78: 386BCC1C  addi r3, r11, -0x33e4
	ctx.r[3].s64 = ctx.r[11].s64 + -13284;
	// 8263EF7C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8263EF80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EF88: 4BE27E99  bl 0x82466e20
	ctx.lr = 0x8263EF8C;
	sub_82466E20(ctx, base);
	// 8263EF8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263EF90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263EF94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263EF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263EFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263EFA0 size=108
    let mut pc: u32 = 0x8263EFA0;
    'dispatch: loop {
        match pc {
            0x8263EFA0 => {
    //   block [0x8263EFA0..0x8263F00C)
	// 8263EFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263EFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263EFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263EFAC: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263EFB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263EFB4: 38EB8078  addi r7, r11, -0x7f88
	ctx.r[7].s64 = ctx.r[11].s64 + -32648;
	// 8263EFB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263EFBC: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 8263EFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263EFC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263EFC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263EFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263EFD0: 386ACC4C  addi r3, r10, -0x33b4
	ctx.r[3].s64 = ctx.r[10].s64 + -13236;
	// 8263EFD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263EFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263EFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263EFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263EFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263EFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263EFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263EFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263EFF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263EFF8: 4BE27E29  bl 0x82466e20
	ctx.lr = 0x8263EFFC;
	sub_82466E20(ctx, base);
	// 8263EFFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F010 size=112
    let mut pc: u32 = 0x8263F010;
    'dispatch: loop {
        match pc {
            0x8263F010 => {
    //   block [0x8263F010..0x8263F080)
	// 8263F010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F01C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F020: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F024: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F02C: 390B80A8  addi r8, r11, -0x7f58
	ctx.r[8].s64 = ctx.r[11].s64 + -32600;
	// 8263F030: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F034: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 8263F038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F03C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F040: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F048: 386ACC7C  addi r3, r10, -0x3384
	ctx.r[3].s64 = ctx.r[10].s64 + -13188;
	// 8263F04C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F06C: 4BE27DB5  bl 0x82466e20
	ctx.lr = 0x8263F070;
	sub_82466E20(ctx, base);
	// 8263F070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F080 size=112
    let mut pc: u32 = 0x8263F080;
    'dispatch: loop {
        match pc {
            0x8263F080 => {
    //   block [0x8263F080..0x8263F0F0)
	// 8263F080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F08C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F090: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F094: 392A6E18  addi r9, r10, 0x6e18
	ctx.r[9].s64 = ctx.r[10].s64 + 28184;
	// 8263F098: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F09C: 390B80C8  addi r8, r11, -0x7f38
	ctx.r[8].s64 = ctx.r[11].s64 + -32568;
	// 8263F0A0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8263F0A4: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 8263F0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F0AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F0B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F0B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F0B8: 386ACCAC  addi r3, r10, -0x3354
	ctx.r[3].s64 = ctx.r[10].s64 + -13140;
	// 8263F0BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263F0C0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263F0C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F0C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F0D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F0D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F0D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F0DC: 4BE27D45  bl 0x82466e20
	ctx.lr = 0x8263F0E0;
	sub_82466E20(ctx, base);
	// 8263F0E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F0E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F0E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F0EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F0F0 size=112
    let mut pc: u32 = 0x8263F0F0;
    'dispatch: loop {
        match pc {
            0x8263F0F0 => {
    //   block [0x8263F0F0..0x8263F160)
	// 8263F0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F0FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F100: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F104: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F10C: 390B8110  addi r8, r11, -0x7ef0
	ctx.r[8].s64 = ctx.r[11].s64 + -32496;
	// 8263F110: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F114: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 8263F118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F11C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F128: 386ACCDC  addi r3, r10, -0x3324
	ctx.r[3].s64 = ctx.r[10].s64 + -13092;
	// 8263F12C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F14C: 4BE27CD5  bl 0x82466e20
	ctx.lr = 0x8263F150;
	sub_82466E20(ctx, base);
	// 8263F150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F160 size=112
    let mut pc: u32 = 0x8263F160;
    'dispatch: loop {
        match pc {
            0x8263F160 => {
    //   block [0x8263F160..0x8263F1D0)
	// 8263F160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F16C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F170: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F174: 392A6E44  addi r9, r10, 0x6e44
	ctx.r[9].s64 = ctx.r[10].s64 + 28228;
	// 8263F178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F17C: 390B8128  addi r8, r11, -0x7ed8
	ctx.r[8].s64 = ctx.r[11].s64 + -32472;
	// 8263F180: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8263F184: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 8263F188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F18C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F190: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F198: 386ACD0C  addi r3, r10, -0x32f4
	ctx.r[3].s64 = ctx.r[10].s64 + -13044;
	// 8263F19C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263F1A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263F1A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F1A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F1B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F1B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F1B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F1BC: 4BE27C65  bl 0x82466e20
	ctx.lr = 0x8263F1C0;
	sub_82466E20(ctx, base);
	// 8263F1C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F1CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F1D0 size=112
    let mut pc: u32 = 0x8263F1D0;
    'dispatch: loop {
        match pc {
            0x8263F1D0 => {
    //   block [0x8263F1D0..0x8263F240)
	// 8263F1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F1D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F1DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F1E0: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F1E4: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F1E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F1EC: 390B81B8  addi r8, r11, -0x7e48
	ctx.r[8].s64 = ctx.r[11].s64 + -32328;
	// 8263F1F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8263F1F4: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 8263F1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F1FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F208: 386ACD3C  addi r3, r10, -0x32c4
	ctx.r[3].s64 = ctx.r[10].s64 + -12996;
	// 8263F20C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F22C: 4BE27BF5  bl 0x82466e20
	ctx.lr = 0x8263F230;
	sub_82466E20(ctx, base);
	// 8263F230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F240 size=112
    let mut pc: u32 = 0x8263F240;
    'dispatch: loop {
        match pc {
            0x8263F240 => {
    //   block [0x8263F240..0x8263F2B0)
	// 8263F240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F24C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F250: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F254: 38AACD9C  addi r5, r10, -0x3264
	ctx.r[5].s64 = ctx.r[10].s64 + -12900;
	// 8263F258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F25C: 390B81D0  addi r8, r11, -0x7e30
	ctx.r[8].s64 = ctx.r[11].s64 + -32304;
	// 8263F260: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8263F264: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 8263F268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F26C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F278: 386ACD6C  addi r3, r10, -0x3294
	ctx.r[3].s64 = ctx.r[10].s64 + -12948;
	// 8263F27C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8263F280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F29C: 4BE27B85  bl 0x82466e20
	ctx.lr = 0x8263F2A0;
	sub_82466E20(ctx, base);
	// 8263F2A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F2A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F2A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F2B0 size=100
    let mut pc: u32 = 0x8263F2B0;
    'dispatch: loop {
        match pc {
            0x8263F2B0 => {
    //   block [0x8263F2B0..0x8263F314)
	// 8263F2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F2B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F2BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F2C4: 38AAA96C  addi r5, r10, -0x5694
	ctx.r[5].s64 = ctx.r[10].s64 + -22164;
	// 8263F2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F2D0: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8263F2D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F2E4: 386ACD9C  addi r3, r10, -0x3264
	ctx.r[3].s64 = ctx.r[10].s64 + -12900;
	// 8263F2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F2EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F2F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8263F2F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F2F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8263F2FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F300: 4BE27B21  bl 0x82466e20
	ctx.lr = 0x8263F304;
	sub_82466E20(ctx, base);
	// 8263F304: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F308: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F30C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F310: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8263F318 size=24
    let mut pc: u32 = 0x8263F318;
    'dispatch: loop {
        match pc {
            0x8263F318 => {
    //   block [0x8263F318..0x8263F330)
	// 8263F318: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F31C: 3D408278  lis r10, -0x7d88
	ctx.r[10].s64 = -2106064896;
	// 8263F320: 394ABE78  addi r10, r10, -0x4188
	ctx.r[10].s64 = ctx.r[10].s64 + -16776;
	// 8263F324: 816B8248  lwz r11, -0x7db8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32184 as u32) ) } as u64;
	// 8263F328: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8263F32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F330 size=116
    let mut pc: u32 = 0x8263F330;
    'dispatch: loop {
        match pc {
            0x8263F330 => {
    //   block [0x8263F330..0x8263F3A4)
	// 8263F330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F33C: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F340: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8263F344: 390BBE78  addi r8, r11, -0x4188
	ctx.r[8].s64 = ctx.r[11].s64 + -16776;
	// 8263F348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F34C: 392A6E80  addi r9, r10, 0x6e80
	ctx.r[9].s64 = ctx.r[10].s64 + 28288;
	// 8263F350: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F354: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8263F358: 38AACBEC  addi r5, r10, -0x3414
	ctx.r[5].s64 = ctx.r[10].s64 + -13332;
	// 8263F35C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8263F360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F364: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F36C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F374: 3D608295  lis r11, -0x7d6b
	ctx.r[11].s64 = -2104164352;
	// 8263F378: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8263F37C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8263F380: 386BCDCC  addi r3, r11, -0x3234
	ctx.r[3].s64 = ctx.r[11].s64 + -12852;
	// 8263F384: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8263F388: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F390: 4BE27A91  bl 0x82466e20
	ctx.lr = 0x8263F394;
	sub_82466E20(ctx, base);
	// 8263F394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F3A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8263F3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8263F3A8 size=108
    let mut pc: u32 = 0x8263F3A8;
    'dispatch: loop {
        match pc {
            0x8263F3A8 => {
    //   block [0x8263F3A8..0x8263F414)
	// 8263F3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8263F3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8263F3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8263F3B4: 3D608278  lis r11, -0x7d88
	ctx.r[11].s64 = -2106064896;
	// 8263F3B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8263F3BC: 38EB824C  addi r7, r11, -0x7db4
	ctx.r[7].s64 = ctx.r[11].s64 + -32180;
	// 8263F3C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8263F3C4: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 8263F3C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8263F3CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8263F3D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8263F3D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8263F3D8: 386ACDFC  addi r3, r10, -0x3204
	ctx.r[3].s64 = ctx.r[10].s64 + -12804;
	// 8263F3DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8263F3E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8263F3E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8263F3E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8263F3EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8263F3F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8263F3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8263F3F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8263F3FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8263F400: 4BE27A21  bl 0x82466e20
	ctx.lr = 0x8263F404;
	sub_82466E20(ctx, base);
	// 8263F404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8263F408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8263F40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8263F410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


