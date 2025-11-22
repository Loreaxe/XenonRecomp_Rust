pub fn sub_826D77B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D77B8 size=108
    let mut pc: u32 = 0x826D77B8;
    'dispatch: loop {
        match pc {
            0x826D77B8 => {
    //   block [0x826D77B8..0x826D7824)
	// 826D77B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D77BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D77C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D77C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D77C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D77CC: 38EB3938  addi r7, r11, 0x3938
	ctx.r[7].s64 = ctx.r[11].s64 + 14648;
	// 826D77D0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D77D4: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 826D77D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D77DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D77E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D77E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D77E8: 386AD154  addi r3, r10, -0x2eac
	ctx.r[3].s64 = ctx.r[10].s64 + -11948;
	// 826D77EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D77F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D77F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D77F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D77FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D780C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7810: 4BD8F611  bl 0x82466e20
	ctx.lr = 0x826D7814;
	sub_82466E20(ctx, base);
	// 826D7814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D781C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7828 size=108
    let mut pc: u32 = 0x826D7828;
    'dispatch: loop {
        match pc {
            0x826D7828 => {
    //   block [0x826D7828..0x826D7894)
	// 826D7828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D782C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7834: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7838: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D783C: 38EB39C8  addi r7, r11, 0x39c8
	ctx.r[7].s64 = ctx.r[11].s64 + 14792;
	// 826D7840: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D7844: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826D7848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D784C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7858: 386AD184  addi r3, r10, -0x2e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -11900;
	// 826D785C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D786C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D787C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7880: 4BD8F5A1  bl 0x82466e20
	ctx.lr = 0x826D7884;
	sub_82466E20(ctx, base);
	// 826D7884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D788C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7898 size=108
    let mut pc: u32 = 0x826D7898;
    'dispatch: loop {
        match pc {
            0x826D7898 => {
    //   block [0x826D7898..0x826D7904)
	// 826D7898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D789C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D78A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D78A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D78A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D78AC: 38EB39E0  addi r7, r11, 0x39e0
	ctx.r[7].s64 = ctx.r[11].s64 + 14816;
	// 826D78B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D78B4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826D78B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D78BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D78C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D78C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D78C8: 386AD1B4  addi r3, r10, -0x2e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11852;
	// 826D78CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D78D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D78D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D78D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D78DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D78E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D78E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D78E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D78EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D78F0: 4BD8F531  bl 0x82466e20
	ctx.lr = 0x826D78F4;
	sub_82466E20(ctx, base);
	// 826D78F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D78F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D78FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7908 size=112
    let mut pc: u32 = 0x826D7908;
    'dispatch: loop {
        match pc {
            0x826D7908 => {
    //   block [0x826D7908..0x826D7978)
	// 826D7908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7914: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7918: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D791C: 392A4F98  addi r9, r10, 0x4f98
	ctx.r[9].s64 = ctx.r[10].s64 + 20376;
	// 826D7920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7924: 390B3A18  addi r8, r11, 0x3a18
	ctx.r[8].s64 = ctx.r[11].s64 + 14872;
	// 826D7928: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D792C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826D7930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7934: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D793C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7940: 386AD1E4  addi r3, r10, -0x2e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -11804;
	// 826D7944: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D7948: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D794C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D795C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7964: 4BD8F4BD  bl 0x82466e20
	ctx.lr = 0x826D7968;
	sub_82466E20(ctx, base);
	// 826D7968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D796C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7978 size=108
    let mut pc: u32 = 0x826D7978;
    'dispatch: loop {
        match pc {
            0x826D7978 => {
    //   block [0x826D7978..0x826D79E4)
	// 826D7978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D797C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7984: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7988: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826D798C: 38EB3A90  addi r7, r11, 0x3a90
	ctx.r[7].s64 = ctx.r[11].s64 + 14992;
	// 826D7990: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D7994: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826D7998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D799C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D79A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D79A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D79A8: 386AD214  addi r3, r10, -0x2dec
	ctx.r[3].s64 = ctx.r[10].s64 + -11756;
	// 826D79AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D79B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D79B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D79B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D79BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D79C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D79C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D79C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D79CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D79D0: 4BD8F451  bl 0x82466e20
	ctx.lr = 0x826D79D4;
	sub_82466E20(ctx, base);
	// 826D79D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D79D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D79DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D79E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D79E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D79E8 size=24
    let mut pc: u32 = 0x826D79E8;
    'dispatch: loop {
        match pc {
            0x826D79E8 => {
    //   block [0x826D79E8..0x826D7A00)
	// 826D79E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D79EC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D79F0: 394AAE80  addi r10, r10, -0x5180
	ctx.r[10].s64 = ctx.r[10].s64 + -20864;
	// 826D79F4: 816B3A14  lwz r11, 0x3a14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14868 as u32) ) } as u64;
	// 826D79F8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826D79FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7A00 size=108
    let mut pc: u32 = 0x826D7A00;
    'dispatch: loop {
        match pc {
            0x826D7A00 => {
    //   block [0x826D7A00..0x826D7A6C)
	// 826D7A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7A0C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D7A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7A14: 38EBAE80  addi r7, r11, -0x5180
	ctx.r[7].s64 = ctx.r[11].s64 + -20864;
	// 826D7A18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7A1C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826D7A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7A24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7A30: 386AD244  addi r3, r10, -0x2dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -11708;
	// 826D7A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7A58: 4BD8F3C9  bl 0x82466e20
	ctx.lr = 0x826D7A5C;
	sub_82466E20(ctx, base);
	// 826D7A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D7A70 size=24
    let mut pc: u32 = 0x826D7A70;
    'dispatch: loop {
        match pc {
            0x826D7A70 => {
    //   block [0x826D7A70..0x826D7A88)
	// 826D7A70: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7A74: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D7A78: 394AAEB0  addi r10, r10, -0x5150
	ctx.r[10].s64 = ctx.r[10].s64 + -20816;
	// 826D7A7C: 816B3A14  lwz r11, 0x3a14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14868 as u32) ) } as u64;
	// 826D7A80: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826D7A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7A88 size=108
    let mut pc: u32 = 0x826D7A88;
    'dispatch: loop {
        match pc {
            0x826D7A88 => {
    //   block [0x826D7A88..0x826D7AF4)
	// 826D7A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7A94: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D7A98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7A9C: 38EBAEB0  addi r7, r11, -0x5150
	ctx.r[7].s64 = ctx.r[11].s64 + -20816;
	// 826D7AA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7AA4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826D7AA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7AAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7AB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7AB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7AB8: 386AD274  addi r3, r10, -0x2d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -11660;
	// 826D7ABC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7AC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7ACC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7AD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7AD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7AD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7AE0: 4BD8F341  bl 0x82466e20
	ctx.lr = 0x826D7AE4;
	sub_82466E20(ctx, base);
	// 826D7AE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7AF8 size=108
    let mut pc: u32 = 0x826D7AF8;
    'dispatch: loop {
        match pc {
            0x826D7AF8 => {
    //   block [0x826D7AF8..0x826D7B64)
	// 826D7AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7B04: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7B0C: 38EB3B68  addi r7, r11, 0x3b68
	ctx.r[7].s64 = ctx.r[11].s64 + 15208;
	// 826D7B10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D7B14: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826D7B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7B1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7B28: 386AD2A4  addi r3, r10, -0x2d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -11612;
	// 826D7B2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7B50: 4BD8F2D1  bl 0x82466e20
	ctx.lr = 0x826D7B54;
	sub_82466E20(ctx, base);
	// 826D7B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D7B68 size=24
    let mut pc: u32 = 0x826D7B68;
    'dispatch: loop {
        match pc {
            0x826D7B68 => {
    //   block [0x826D7B68..0x826D7B80)
	// 826D7B68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7B6C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D7B70: 394AAEE0  addi r10, r10, -0x5120
	ctx.r[10].s64 = ctx.r[10].s64 + -20768;
	// 826D7B74: 816B3A14  lwz r11, 0x3a14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14868 as u32) ) } as u64;
	// 826D7B78: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826D7B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7B80 size=108
    let mut pc: u32 = 0x826D7B80;
    'dispatch: loop {
        match pc {
            0x826D7B80 => {
    //   block [0x826D7B80..0x826D7BEC)
	// 826D7B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7B8C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D7B90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7B94: 38EBAEE0  addi r7, r11, -0x5120
	ctx.r[7].s64 = ctx.r[11].s64 + -20768;
	// 826D7B98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7B9C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826D7BA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7BA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7BA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7BB0: 386AD2D4  addi r3, r10, -0x2d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -11564;
	// 826D7BB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7BB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7BBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7BC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7BC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7BD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7BD8: 4BD8F249  bl 0x82466e20
	ctx.lr = 0x826D7BDC;
	sub_82466E20(ctx, base);
	// 826D7BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7BE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7BE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7BF0 size=112
    let mut pc: u32 = 0x826D7BF0;
    'dispatch: loop {
        match pc {
            0x826D7BF0 => {
    //   block [0x826D7BF0..0x826D7C60)
	// 826D7BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7BFC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D7C00: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7C04: 392A4FDC  addi r9, r10, 0x4fdc
	ctx.r[9].s64 = ctx.r[10].s64 + 20444;
	// 826D7C08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7C0C: 390B3B80  addi r8, r11, 0x3b80
	ctx.r[8].s64 = ctx.r[11].s64 + 15232;
	// 826D7C10: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D7C14: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826D7C18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7C1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7C20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D7C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7C28: 386AD304  addi r3, r10, -0x2cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -11516;
	// 826D7C2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D7C30: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D7C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7C4C: 4BD8F1D5  bl 0x82466e20
	ctx.lr = 0x826D7C50;
	sub_82466E20(ctx, base);
	// 826D7C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7C60 size=108
    let mut pc: u32 = 0x826D7C60;
    'dispatch: loop {
        match pc {
            0x826D7C60 => {
    //   block [0x826D7C60..0x826D7CCC)
	// 826D7C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7C6C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7C70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7C74: 38EB3BB0  addi r7, r11, 0x3bb0
	ctx.r[7].s64 = ctx.r[11].s64 + 15280;
	// 826D7C78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7C7C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826D7C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7C84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7C88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7C90: 386AD334  addi r3, r10, -0x2ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -11468;
	// 826D7C94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7CB8: 4BD8F169  bl 0x82466e20
	ctx.lr = 0x826D7CBC;
	sub_82466E20(ctx, base);
	// 826D7CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7CD0 size=108
    let mut pc: u32 = 0x826D7CD0;
    'dispatch: loop {
        match pc {
            0x826D7CD0 => {
    //   block [0x826D7CD0..0x826D7D3C)
	// 826D7CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7CDC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7CE4: 38EB3BE0  addi r7, r11, 0x3be0
	ctx.r[7].s64 = ctx.r[11].s64 + 15328;
	// 826D7CE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D7CEC: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826D7CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7CF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7CF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7D00: 386AD364  addi r3, r10, -0x2c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -11420;
	// 826D7D04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7D28: 4BD8F0F9  bl 0x82466e20
	ctx.lr = 0x826D7D2C;
	sub_82466E20(ctx, base);
	// 826D7D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7D40 size=108
    let mut pc: u32 = 0x826D7D40;
    'dispatch: loop {
        match pc {
            0x826D7D40 => {
    //   block [0x826D7D40..0x826D7DAC)
	// 826D7D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7D4C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7D54: 38EB3BF8  addi r7, r11, 0x3bf8
	ctx.r[7].s64 = ctx.r[11].s64 + 15352;
	// 826D7D58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7D5C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826D7D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7D64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7D68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7D70: 386AD394  addi r3, r10, -0x2c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -11372;
	// 826D7D74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7D94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7D98: 4BD8F089  bl 0x82466e20
	ctx.lr = 0x826D7D9C;
	sub_82466E20(ctx, base);
	// 826D7D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7DB0 size=112
    let mut pc: u32 = 0x826D7DB0;
    'dispatch: loop {
        match pc {
            0x826D7DB0 => {
    //   block [0x826D7DB0..0x826D7E20)
	// 826D7DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7DBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7DC0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7DC4: 38AAD3F4  addi r5, r10, -0x2c0c
	ctx.r[5].s64 = ctx.r[10].s64 + -11276;
	// 826D7DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7DCC: 390B3C28  addi r8, r11, 0x3c28
	ctx.r[8].s64 = ctx.r[11].s64 + 15400;
	// 826D7DD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D7DD4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826D7DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7DDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7DE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D7DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7DE8: 386AD3C4  addi r3, r10, -0x2c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -11324;
	// 826D7DEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D7DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7E0C: 4BD8F015  bl 0x82466e20
	ctx.lr = 0x826D7E10;
	sub_82466E20(ctx, base);
	// 826D7E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7E20 size=108
    let mut pc: u32 = 0x826D7E20;
    'dispatch: loop {
        match pc {
            0x826D7E20 => {
    //   block [0x826D7E20..0x826D7E8C)
	// 826D7E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7E2C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7E30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7E34: 38EB3C40  addi r7, r11, 0x3c40
	ctx.r[7].s64 = ctx.r[11].s64 + 15424;
	// 826D7E38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7E3C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826D7E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7E44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7E48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7E4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7E50: 386AD3F4  addi r3, r10, -0x2c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -11276;
	// 826D7E54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7E74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7E78: 4BD8EFA9  bl 0x82466e20
	ctx.lr = 0x826D7E7C;
	sub_82466E20(ctx, base);
	// 826D7E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7E90 size=108
    let mut pc: u32 = 0x826D7E90;
    'dispatch: loop {
        match pc {
            0x826D7E90 => {
    //   block [0x826D7E90..0x826D7EFC)
	// 826D7E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7E9C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7EA4: 38EB3C70  addi r7, r11, 0x3c70
	ctx.r[7].s64 = ctx.r[11].s64 + 15472;
	// 826D7EA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D7EAC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826D7EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7EB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7EB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7EC0: 386AD424  addi r3, r10, -0x2bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -11228;
	// 826D7EC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7EE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7EE8: 4BD8EF39  bl 0x82466e20
	ctx.lr = 0x826D7EEC;
	sub_82466E20(ctx, base);
	// 826D7EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7F00 size=108
    let mut pc: u32 = 0x826D7F00;
    'dispatch: loop {
        match pc {
            0x826D7F00 => {
    //   block [0x826D7F00..0x826D7F6C)
	// 826D7F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7F0C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7F14: 38EB3C88  addi r7, r11, 0x3c88
	ctx.r[7].s64 = ctx.r[11].s64 + 15496;
	// 826D7F18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7F1C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826D7F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7F28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7F30: 386AD454  addi r3, r10, -0x2bac
	ctx.r[3].s64 = ctx.r[10].s64 + -11180;
	// 826D7F34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7F58: 4BD8EEC9  bl 0x82466e20
	ctx.lr = 0x826D7F5C;
	sub_82466E20(ctx, base);
	// 826D7F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7F70 size=108
    let mut pc: u32 = 0x826D7F70;
    'dispatch: loop {
        match pc {
            0x826D7F70 => {
    //   block [0x826D7F70..0x826D7FDC)
	// 826D7F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7F7C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7F80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7F84: 38EB3CB8  addi r7, r11, 0x3cb8
	ctx.r[7].s64 = ctx.r[11].s64 + 15544;
	// 826D7F88: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D7F8C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826D7F90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D7F94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D7F98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D7F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D7FA0: 386AD484  addi r3, r10, -0x2b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -11132;
	// 826D7FA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D7FA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D7FAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D7FB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D7FB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D7FB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D7FBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D7FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D7FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D7FC8: 4BD8EE59  bl 0x82466e20
	ctx.lr = 0x826D7FCC;
	sub_82466E20(ctx, base);
	// 826D7FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D7FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D7FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D7FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D7FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D7FE0 size=108
    let mut pc: u32 = 0x826D7FE0;
    'dispatch: loop {
        match pc {
            0x826D7FE0 => {
    //   block [0x826D7FE0..0x826D804C)
	// 826D7FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D7FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D7FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D7FEC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D7FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D7FF4: 38EB3D60  addi r7, r11, 0x3d60
	ctx.r[7].s64 = ctx.r[11].s64 + 15712;
	// 826D7FF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D7FFC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826D8000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8004: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8008: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D800C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8010: 386AD4B4  addi r3, r10, -0x2b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -11084;
	// 826D8014: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D801C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D802C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8034: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8038: 4BD8EDE9  bl 0x82466e20
	ctx.lr = 0x826D803C;
	sub_82466E20(ctx, base);
	// 826D803C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8050 size=108
    let mut pc: u32 = 0x826D8050;
    'dispatch: loop {
        match pc {
            0x826D8050 => {
    //   block [0x826D8050..0x826D80BC)
	// 826D8050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D805C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8060: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8064: 38EB3D90  addi r7, r11, 0x3d90
	ctx.r[7].s64 = ctx.r[11].s64 + 15760;
	// 826D8068: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D806C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826D8070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8078: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D807C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8080: 386AD4E4  addi r3, r10, -0x2b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -11036;
	// 826D8084: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8088: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D808C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D809C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D80A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D80A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D80A8: 4BD8ED79  bl 0x82466e20
	ctx.lr = 0x826D80AC;
	sub_82466E20(ctx, base);
	// 826D80AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D80B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D80B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D80B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D80C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D80C0 size=108
    let mut pc: u32 = 0x826D80C0;
    'dispatch: loop {
        match pc {
            0x826D80C0 => {
    //   block [0x826D80C0..0x826D812C)
	// 826D80C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D80C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D80C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D80CC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D80D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D80D4: 38EB3DA8  addi r7, r11, 0x3da8
	ctx.r[7].s64 = ctx.r[11].s64 + 15784;
	// 826D80D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D80DC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826D80E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D80E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D80E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D80EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D80F0: 386AD514  addi r3, r10, -0x2aec
	ctx.r[3].s64 = ctx.r[10].s64 + -10988;
	// 826D80F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D80F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D80FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D810C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8114: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8118: 4BD8ED09  bl 0x82466e20
	ctx.lr = 0x826D811C;
	sub_82466E20(ctx, base);
	// 826D811C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8130 size=112
    let mut pc: u32 = 0x826D8130;
    'dispatch: loop {
        match pc {
            0x826D8130 => {
    //   block [0x826D8130..0x826D81A0)
	// 826D8130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D813C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8140: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8144: 38AAD364  addi r5, r10, -0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11420;
	// 826D8148: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D814C: 390B3DD8  addi r8, r11, 0x3dd8
	ctx.r[8].s64 = ctx.r[11].s64 + 15832;
	// 826D8150: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D8154: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826D8158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D815C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D8164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8168: 386AD544  addi r3, r10, -0x2abc
	ctx.r[3].s64 = ctx.r[10].s64 + -10940;
	// 826D816C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D8170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D817C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D818C: 4BD8EC95  bl 0x82466e20
	ctx.lr = 0x826D8190;
	sub_82466E20(ctx, base);
	// 826D8190: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8194: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8198: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D819C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D81A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D81A0 size=24
    let mut pc: u32 = 0x826D81A0;
    'dispatch: loop {
        match pc {
            0x826D81A0 => {
    //   block [0x826D81A0..0x826D81B8)
	// 826D81A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D81A4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D81A8: 394AAF10  addi r10, r10, -0x50f0
	ctx.r[10].s64 = ctx.r[10].s64 + -20720;
	// 826D81AC: 816B3E80  lwz r11, 0x3e80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16000 as u32) ) } as u64;
	// 826D81B0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D81B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D81B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D81B8 size=112
    let mut pc: u32 = 0x826D81B8;
    'dispatch: loop {
        match pc {
            0x826D81B8 => {
    //   block [0x826D81B8..0x826D8228)
	// 826D81B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D81BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D81C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D81C4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D81C8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D81CC: 392A5008  addi r9, r10, 0x5008
	ctx.r[9].s64 = ctx.r[10].s64 + 20488;
	// 826D81D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D81D4: 390BAF10  addi r8, r11, -0x50f0
	ctx.r[8].s64 = ctx.r[11].s64 + -20720;
	// 826D81D8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D81DC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826D81E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D81E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D81E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D81EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D81F0: 386AD574  addi r3, r10, -0x2a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -10892;
	// 826D81F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D81F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D81FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D820C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8214: 4BD8EC0D  bl 0x82466e20
	ctx.lr = 0x826D8218;
	sub_82466E20(ctx, base);
	// 826D8218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D821C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8228 size=108
    let mut pc: u32 = 0x826D8228;
    'dispatch: loop {
        match pc {
            0x826D8228 => {
    //   block [0x826D8228..0x826D8294)
	// 826D8228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D822C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8234: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D823C: 38EB3E88  addi r7, r11, 0x3e88
	ctx.r[7].s64 = ctx.r[11].s64 + 16008;
	// 826D8240: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D8244: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826D8248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D824C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8250: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8258: 386AD5A4  addi r3, r10, -0x2a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -10844;
	// 826D825C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D826C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D827C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8280: 4BD8EBA1  bl 0x82466e20
	ctx.lr = 0x826D8284;
	sub_82466E20(ctx, base);
	// 826D8284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D828C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8298 size=116
    let mut pc: u32 = 0x826D8298;
    'dispatch: loop {
        match pc {
            0x826D8298 => {
    //   block [0x826D8298..0x826D830C)
	// 826D8298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D829C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D82A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D82A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D82A8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D82AC: 390B3EB8  addi r8, r11, 0x3eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 16056;
	// 826D82B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D82B4: 392A504C  addi r9, r10, 0x504c
	ctx.r[9].s64 = ctx.r[10].s64 + 20556;
	// 826D82B8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D82BC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826D82C0: 38AAD364  addi r5, r10, -0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11420;
	// 826D82C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D82C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D82CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D82D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D82D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D82D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D82DC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D82E0: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826D82E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D82E8: 386BD5D4  addi r3, r11, -0x2a2c
	ctx.r[3].s64 = ctx.r[11].s64 + -10796;
	// 826D82EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D82F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D82F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D82F8: 4BD8EB29  bl 0x82466e20
	ctx.lr = 0x826D82FC;
	sub_82466E20(ctx, base);
	// 826D82FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D8310 size=24
    let mut pc: u32 = 0x826D8310;
    'dispatch: loop {
        match pc {
            0x826D8310 => {
    //   block [0x826D8310..0x826D8328)
	// 826D8310: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8314: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D8318: 394AAF88  addi r10, r10, -0x5078
	ctx.r[10].s64 = ctx.r[10].s64 + -20600;
	// 826D831C: 816B3F78  lwz r11, 0x3f78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16248 as u32) ) } as u64;
	// 826D8320: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826D8324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8328 size=112
    let mut pc: u32 = 0x826D8328;
    'dispatch: loop {
        match pc {
            0x826D8328 => {
    //   block [0x826D8328..0x826D8398)
	// 826D8328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D832C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8334: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D8338: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D833C: 392A5088  addi r9, r10, 0x5088
	ctx.r[9].s64 = ctx.r[10].s64 + 20616;
	// 826D8340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8344: 390BAF88  addi r8, r11, -0x5078
	ctx.r[8].s64 = ctx.r[11].s64 + -20600;
	// 826D8348: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826D834C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826D8350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D835C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8360: 386AD604  addi r3, r10, -0x29fc
	ctx.r[3].s64 = ctx.r[10].s64 + -10748;
	// 826D8364: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D8368: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D836C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D837C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8384: 4BD8EA9D  bl 0x82466e20
	ctx.lr = 0x826D8388;
	sub_82466E20(ctx, base);
	// 826D8388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D838C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8398 size=108
    let mut pc: u32 = 0x826D8398;
    'dispatch: loop {
        match pc {
            0x826D8398 => {
    //   block [0x826D8398..0x826D8404)
	// 826D8398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D839C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D83A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D83A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D83A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D83AC: 38EB3F7C  addi r7, r11, 0x3f7c
	ctx.r[7].s64 = ctx.r[11].s64 + 16252;
	// 826D83B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D83B4: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826D83B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D83BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D83C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D83C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D83C8: 386AD634  addi r3, r10, -0x29cc
	ctx.r[3].s64 = ctx.r[10].s64 + -10700;
	// 826D83CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D83D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D83D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D83D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D83DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D83E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D83E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D83E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D83EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D83F0: 4BD8EA31  bl 0x82466e20
	ctx.lr = 0x826D83F4;
	sub_82466E20(ctx, base);
	// 826D83F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D83F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D83FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8408 size=108
    let mut pc: u32 = 0x826D8408;
    'dispatch: loop {
        match pc {
            0x826D8408 => {
    //   block [0x826D8408..0x826D8474)
	// 826D8408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D840C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8414: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D841C: 38EB3F94  addi r7, r11, 0x3f94
	ctx.r[7].s64 = ctx.r[11].s64 + 16276;
	// 826D8420: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D8424: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826D8428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D842C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8430: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8438: 386AD664  addi r3, r10, -0x299c
	ctx.r[3].s64 = ctx.r[10].s64 + -10652;
	// 826D843C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D844C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D845C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8460: 4BD8E9C1  bl 0x82466e20
	ctx.lr = 0x826D8464;
	sub_82466E20(ctx, base);
	// 826D8464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D846C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D8478 size=24
    let mut pc: u32 = 0x826D8478;
    'dispatch: loop {
        match pc {
            0x826D8478 => {
    //   block [0x826D8478..0x826D8490)
	// 826D8478: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D847C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D8480: 394AAFD0  addi r10, r10, -0x5030
	ctx.r[10].s64 = ctx.r[10].s64 + -20528;
	// 826D8484: 816B3FC4  lwz r11, 0x3fc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16324 as u32) ) } as u64;
	// 826D8488: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D848C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8490 size=112
    let mut pc: u32 = 0x826D8490;
    'dispatch: loop {
        match pc {
            0x826D8490 => {
    //   block [0x826D8490..0x826D8500)
	// 826D8490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D849C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D84A0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D84A4: 392A50C4  addi r9, r10, 0x50c4
	ctx.r[9].s64 = ctx.r[10].s64 + 20676;
	// 826D84A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D84AC: 390BAFD0  addi r8, r11, -0x5030
	ctx.r[8].s64 = ctx.r[11].s64 + -20528;
	// 826D84B0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D84B4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826D84B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D84BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D84C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D84C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D84C8: 386AD694  addi r3, r10, -0x296c
	ctx.r[3].s64 = ctx.r[10].s64 + -10604;
	// 826D84CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D84D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D84D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D84D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D84DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D84E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D84E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D84E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D84EC: 4BD8E935  bl 0x82466e20
	ctx.lr = 0x826D84F0;
	sub_82466E20(ctx, base);
	// 826D84F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D84F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D84F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D84FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8500 size=112
    let mut pc: u32 = 0x826D8500;
    'dispatch: loop {
        match pc {
            0x826D8500 => {
    //   block [0x826D8500..0x826D8570)
	// 826D8500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D850C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8510: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8514: 38AAD364  addi r5, r10, -0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11420;
	// 826D8518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D851C: 390B3FC8  addi r8, r11, 0x3fc8
	ctx.r[8].s64 = ctx.r[11].s64 + 16328;
	// 826D8520: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D8524: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 826D8528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D852C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D8534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8538: 386AD6C4  addi r3, r10, -0x293c
	ctx.r[3].s64 = ctx.r[10].s64 + -10556;
	// 826D853C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D8540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D854C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D855C: 4BD8E8C5  bl 0x82466e20
	ctx.lr = 0x826D8560;
	sub_82466E20(ctx, base);
	// 826D8560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D856C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8570 size=108
    let mut pc: u32 = 0x826D8570;
    'dispatch: loop {
        match pc {
            0x826D8570 => {
    //   block [0x826D8570..0x826D85DC)
	// 826D8570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D857C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8584: 38EB3FF8  addi r7, r11, 0x3ff8
	ctx.r[7].s64 = ctx.r[11].s64 + 16376;
	// 826D8588: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D858C: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826D8590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8594: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D859C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D85A0: 386AD6F4  addi r3, r10, -0x290c
	ctx.r[3].s64 = ctx.r[10].s64 + -10508;
	// 826D85A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D85A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D85AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D85B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D85B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D85B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D85BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D85C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D85C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D85C8: 4BD8E859  bl 0x82466e20
	ctx.lr = 0x826D85CC;
	sub_82466E20(ctx, base);
	// 826D85CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D85D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D85D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D85D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D85E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D85E0 size=108
    let mut pc: u32 = 0x826D85E0;
    'dispatch: loop {
        match pc {
            0x826D85E0 => {
    //   block [0x826D85E0..0x826D864C)
	// 826D85E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D85E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D85E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D85EC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D85F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D85F4: 38EB4028  addi r7, r11, 0x4028
	ctx.r[7].s64 = ctx.r[11].s64 + 16424;
	// 826D85F8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D85FC: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826D8600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8604: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D860C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8610: 386AD724  addi r3, r10, -0x28dc
	ctx.r[3].s64 = ctx.r[10].s64 + -10460;
	// 826D8614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D861C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D862C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8638: 4BD8E7E9  bl 0x82466e20
	ctx.lr = 0x826D863C;
	sub_82466E20(ctx, base);
	// 826D863C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8650 size=108
    let mut pc: u32 = 0x826D8650;
    'dispatch: loop {
        match pc {
            0x826D8650 => {
    //   block [0x826D8650..0x826D86BC)
	// 826D8650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D865C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8664: 38EB4088  addi r7, r11, 0x4088
	ctx.r[7].s64 = ctx.r[11].s64 + 16520;
	// 826D8668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D866C: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826D8670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8674: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D867C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8680: 386AD754  addi r3, r10, -0x28ac
	ctx.r[3].s64 = ctx.r[10].s64 + -10412;
	// 826D8684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D868C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D869C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D86A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D86A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D86A8: 4BD8E779  bl 0x82466e20
	ctx.lr = 0x826D86AC;
	sub_82466E20(ctx, base);
	// 826D86AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D86B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D86B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D86B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D86C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D86C0 size=108
    let mut pc: u32 = 0x826D86C0;
    'dispatch: loop {
        match pc {
            0x826D86C0 => {
    //   block [0x826D86C0..0x826D872C)
	// 826D86C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D86C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D86C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D86CC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D86D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D86D4: 38EB40B8  addi r7, r11, 0x40b8
	ctx.r[7].s64 = ctx.r[11].s64 + 16568;
	// 826D86D8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826D86DC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826D86E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D86E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D86E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D86EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D86F0: 386AD784  addi r3, r10, -0x287c
	ctx.r[3].s64 = ctx.r[10].s64 + -10364;
	// 826D86F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D86F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D86FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D870C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8718: 4BD8E709  bl 0x82466e20
	ctx.lr = 0x826D871C;
	sub_82466E20(ctx, base);
	// 826D871C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8730 size=108
    let mut pc: u32 = 0x826D8730;
    'dispatch: loop {
        match pc {
            0x826D8730 => {
    //   block [0x826D8730..0x826D879C)
	// 826D8730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D873C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8744: 38EB41D8  addi r7, r11, 0x41d8
	ctx.r[7].s64 = ctx.r[11].s64 + 16856;
	// 826D8748: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D874C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 826D8750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8754: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D875C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8760: 386AD7B4  addi r3, r10, -0x284c
	ctx.r[3].s64 = ctx.r[10].s64 + -10316;
	// 826D8764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D876C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D877C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8788: 4BD8E699  bl 0x82466e20
	ctx.lr = 0x826D878C;
	sub_82466E20(ctx, base);
	// 826D878C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D87A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D87A0 size=108
    let mut pc: u32 = 0x826D87A0;
    'dispatch: loop {
        match pc {
            0x826D87A0 => {
    //   block [0x826D87A0..0x826D880C)
	// 826D87A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D87A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D87A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D87AC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D87B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D87B4: 38EB41F0  addi r7, r11, 0x41f0
	ctx.r[7].s64 = ctx.r[11].s64 + 16880;
	// 826D87B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D87BC: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 826D87C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D87C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D87C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D87CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D87D0: 386AD7E4  addi r3, r10, -0x281c
	ctx.r[3].s64 = ctx.r[10].s64 + -10268;
	// 826D87D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D87D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D87DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D87E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D87E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D87E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D87EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D87F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D87F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D87F8: 4BD8E629  bl 0x82466e20
	ctx.lr = 0x826D87FC;
	sub_82466E20(ctx, base);
	// 826D87FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8810 size=108
    let mut pc: u32 = 0x826D8810;
    'dispatch: loop {
        match pc {
            0x826D8810 => {
    //   block [0x826D8810..0x826D887C)
	// 826D8810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D881C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8824: 38EB4208  addi r7, r11, 0x4208
	ctx.r[7].s64 = ctx.r[11].s64 + 16904;
	// 826D8828: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D882C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 826D8830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8834: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8838: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D883C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8840: 386AD814  addi r3, r10, -0x27ec
	ctx.r[3].s64 = ctx.r[10].s64 + -10220;
	// 826D8844: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D884C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D885C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8868: 4BD8E5B9  bl 0x82466e20
	ctx.lr = 0x826D886C;
	sub_82466E20(ctx, base);
	// 826D886C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8880 size=108
    let mut pc: u32 = 0x826D8880;
    'dispatch: loop {
        match pc {
            0x826D8880 => {
    //   block [0x826D8880..0x826D88EC)
	// 826D8880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D888C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8894: 38EB4220  addi r7, r11, 0x4220
	ctx.r[7].s64 = ctx.r[11].s64 + 16928;
	// 826D8898: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D889C: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 826D88A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D88A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D88A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D88AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D88B0: 386AD844  addi r3, r10, -0x27bc
	ctx.r[3].s64 = ctx.r[10].s64 + -10172;
	// 826D88B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D88B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D88BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D88C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D88C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D88C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D88CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D88D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D88D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D88D8: 4BD8E549  bl 0x82466e20
	ctx.lr = 0x826D88DC;
	sub_82466E20(ctx, base);
	// 826D88DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D88E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D88E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D88E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D88F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D88F0 size=108
    let mut pc: u32 = 0x826D88F0;
    'dispatch: loop {
        match pc {
            0x826D88F0 => {
    //   block [0x826D88F0..0x826D895C)
	// 826D88F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D88F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D88F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D88FC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8904: 38EB4238  addi r7, r11, 0x4238
	ctx.r[7].s64 = ctx.r[11].s64 + 16952;
	// 826D8908: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D890C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826D8910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8914: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8918: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D891C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8920: 386AD874  addi r3, r10, -0x278c
	ctx.r[3].s64 = ctx.r[10].s64 + -10124;
	// 826D8924: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8928: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D892C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8930: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8938: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D893C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8940: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8944: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8948: 4BD8E4D9  bl 0x82466e20
	ctx.lr = 0x826D894C;
	sub_82466E20(ctx, base);
	// 826D894C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8960 size=108
    let mut pc: u32 = 0x826D8960;
    'dispatch: loop {
        match pc {
            0x826D8960 => {
    //   block [0x826D8960..0x826D89CC)
	// 826D8960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D896C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8974: 38EB4250  addi r7, r11, 0x4250
	ctx.r[7].s64 = ctx.r[11].s64 + 16976;
	// 826D8978: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D897C: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 826D8980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8984: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D898C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8990: 386AD8A4  addi r3, r10, -0x275c
	ctx.r[3].s64 = ctx.r[10].s64 + -10076;
	// 826D8994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D899C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D89A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D89A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D89A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D89AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D89B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D89B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D89B8: 4BD8E469  bl 0x82466e20
	ctx.lr = 0x826D89BC;
	sub_82466E20(ctx, base);
	// 826D89BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D89C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D89C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D89C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D89D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D89D0 size=108
    let mut pc: u32 = 0x826D89D0;
    'dispatch: loop {
        match pc {
            0x826D89D0 => {
    //   block [0x826D89D0..0x826D8A3C)
	// 826D89D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D89D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D89D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D89DC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D89E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D89E4: 38EB4268  addi r7, r11, 0x4268
	ctx.r[7].s64 = ctx.r[11].s64 + 17000;
	// 826D89E8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D89EC: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826D89F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D89F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D89F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D89FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8A00: 386AD8D4  addi r3, r10, -0x272c
	ctx.r[3].s64 = ctx.r[10].s64 + -10028;
	// 826D8A04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8A08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8A24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8A28: 4BD8E3F9  bl 0x82466e20
	ctx.lr = 0x826D8A2C;
	sub_82466E20(ctx, base);
	// 826D8A2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8A40 size=108
    let mut pc: u32 = 0x826D8A40;
    'dispatch: loop {
        match pc {
            0x826D8A40 => {
    //   block [0x826D8A40..0x826D8AAC)
	// 826D8A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8A4C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8A50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8A54: 38EB42F8  addi r7, r11, 0x42f8
	ctx.r[7].s64 = ctx.r[11].s64 + 17144;
	// 826D8A58: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826D8A5C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826D8A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8A64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8A68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8A70: 386AD904  addi r3, r10, -0x26fc
	ctx.r[3].s64 = ctx.r[10].s64 + -9980;
	// 826D8A74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8A94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8A98: 4BD8E389  bl 0x82466e20
	ctx.lr = 0x826D8A9C;
	sub_82466E20(ctx, base);
	// 826D8A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8AB0 size=108
    let mut pc: u32 = 0x826D8AB0;
    'dispatch: loop {
        match pc {
            0x826D8AB0 => {
    //   block [0x826D8AB0..0x826D8B1C)
	// 826D8AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8ABC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8AC4: 38EB43B8  addi r7, r11, 0x43b8
	ctx.r[7].s64 = ctx.r[11].s64 + 17336;
	// 826D8AC8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D8ACC: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826D8AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8AD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8AD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8ADC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8AE0: 386AD934  addi r3, r10, -0x26cc
	ctx.r[3].s64 = ctx.r[10].s64 + -9932;
	// 826D8AE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8B04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8B08: 4BD8E319  bl 0x82466e20
	ctx.lr = 0x826D8B0C;
	sub_82466E20(ctx, base);
	// 826D8B0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8B20 size=108
    let mut pc: u32 = 0x826D8B20;
    'dispatch: loop {
        match pc {
            0x826D8B20 => {
    //   block [0x826D8B20..0x826D8B8C)
	// 826D8B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8B2C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8B34: 38EB4490  addi r7, r11, 0x4490
	ctx.r[7].s64 = ctx.r[11].s64 + 17552;
	// 826D8B38: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826D8B3C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826D8B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8B44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8B48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8B50: 386AD964  addi r3, r10, -0x269c
	ctx.r[3].s64 = ctx.r[10].s64 + -9884;
	// 826D8B54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8B74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8B78: 4BD8E2A9  bl 0x82466e20
	ctx.lr = 0x826D8B7C;
	sub_82466E20(ctx, base);
	// 826D8B7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8B90 size=108
    let mut pc: u32 = 0x826D8B90;
    'dispatch: loop {
        match pc {
            0x826D8B90 => {
    //   block [0x826D8B90..0x826D8BFC)
	// 826D8B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8B9C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8BA4: 38EB4550  addi r7, r11, 0x4550
	ctx.r[7].s64 = ctx.r[11].s64 + 17744;
	// 826D8BA8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D8BAC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826D8BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8BB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8BB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8BBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8BC0: 386AD994  addi r3, r10, -0x266c
	ctx.r[3].s64 = ctx.r[10].s64 + -9836;
	// 826D8BC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8BE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8BE8: 4BD8E239  bl 0x82466e20
	ctx.lr = 0x826D8BEC;
	sub_82466E20(ctx, base);
	// 826D8BEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8BF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8C00 size=112
    let mut pc: u32 = 0x826D8C00;
    'dispatch: loop {
        match pc {
            0x826D8C00 => {
    //   block [0x826D8C00..0x826D8C70)
	// 826D8C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8C0C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D8C10: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826D8C14: 38EA45F8  addi r7, r10, 0x45f8
	ctx.r[7].s64 = ctx.r[10].s64 + 17912;
	// 826D8C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8C1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D8C20: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826D8C24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8C28: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8C2C: 396B50D8  addi r11, r11, 0x50d8
	ctx.r[11].s64 = ctx.r[11].s64 + 20696;
	// 826D8C30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8C34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8C38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8C3C: 386AD9C4  addi r3, r10, -0x263c
	ctx.r[3].s64 = ctx.r[10].s64 + -9788;
	// 826D8C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8C44: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D8C48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8C4C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D8C50: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8C54: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8C58: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8C5C: 4BD8E1C5  bl 0x82466e20
	ctx.lr = 0x826D8C60;
	sub_82466E20(ctx, base);
	// 826D8C60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8C64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8C68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8C70 size=112
    let mut pc: u32 = 0x826D8C70;
    'dispatch: loop {
        match pc {
            0x826D8C70 => {
    //   block [0x826D8C70..0x826D8CE0)
	// 826D8C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8C7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8C80: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8C84: 38AAD364  addi r5, r10, -0x2c9c
	ctx.r[5].s64 = ctx.r[10].s64 + -11420;
	// 826D8C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8C8C: 390B4730  addi r8, r11, 0x4730
	ctx.r[8].s64 = ctx.r[11].s64 + 18224;
	// 826D8C90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D8C94: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 826D8C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8C9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D8CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8CA8: 386AD9F4  addi r3, r10, -0x260c
	ctx.r[3].s64 = ctx.r[10].s64 + -9740;
	// 826D8CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D8CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8CCC: 4BD8E155  bl 0x82466e20
	ctx.lr = 0x826D8CD0;
	sub_82466E20(ctx, base);
	// 826D8CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8CE0 size=108
    let mut pc: u32 = 0x826D8CE0;
    'dispatch: loop {
        match pc {
            0x826D8CE0 => {
    //   block [0x826D8CE0..0x826D8D4C)
	// 826D8CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8CE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8CEC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8CF4: 38EB4760  addi r7, r11, 0x4760
	ctx.r[7].s64 = ctx.r[11].s64 + 18272;
	// 826D8CF8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D8CFC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826D8D00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8D04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8D08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8D10: 386ADA24  addi r3, r10, -0x25dc
	ctx.r[3].s64 = ctx.r[10].s64 + -9692;
	// 826D8D14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8D20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8D24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8D28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8D2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8D30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8D34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8D38: 4BD8E0E9  bl 0x82466e20
	ctx.lr = 0x826D8D3C;
	sub_82466E20(ctx, base);
	// 826D8D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8D50 size=108
    let mut pc: u32 = 0x826D8D50;
    'dispatch: loop {
        match pc {
            0x826D8D50 => {
    //   block [0x826D8D50..0x826D8DBC)
	// 826D8D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8D5C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8D60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8D64: 38EB47C0  addi r7, r11, 0x47c0
	ctx.r[7].s64 = ctx.r[11].s64 + 18368;
	// 826D8D68: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826D8D6C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826D8D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8D74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8D78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8D80: 386ADA54  addi r3, r10, -0x25ac
	ctx.r[3].s64 = ctx.r[10].s64 + -9644;
	// 826D8D84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8D88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8D9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8DA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8DA8: 4BD8E079  bl 0x82466e20
	ctx.lr = 0x826D8DAC;
	sub_82466E20(ctx, base);
	// 826D8DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8DC0 size=108
    let mut pc: u32 = 0x826D8DC0;
    'dispatch: loop {
        match pc {
            0x826D8DC0 => {
    //   block [0x826D8DC0..0x826D8E2C)
	// 826D8DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8DCC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8DD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8DD4: 38EB48C8  addi r7, r11, 0x48c8
	ctx.r[7].s64 = ctx.r[11].s64 + 18632;
	// 826D8DD8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D8DDC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826D8DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8DE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8DE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8DF0: 386ADA84  addi r3, r10, -0x257c
	ctx.r[3].s64 = ctx.r[10].s64 + -9596;
	// 826D8DF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8DF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8E00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8E04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8E08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8E0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8E10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8E14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8E18: 4BD8E009  bl 0x82466e20
	ctx.lr = 0x826D8E1C;
	sub_82466E20(ctx, base);
	// 826D8E1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8E20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8E24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8E28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8E30 size=108
    let mut pc: u32 = 0x826D8E30;
    'dispatch: loop {
        match pc {
            0x826D8E30 => {
    //   block [0x826D8E30..0x826D8E9C)
	// 826D8E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8E3C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8E40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8E44: 38EB49A0  addi r7, r11, 0x49a0
	ctx.r[7].s64 = ctx.r[11].s64 + 18848;
	// 826D8E48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D8E4C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826D8E50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8E54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8E58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8E60: 386ADAB4  addi r3, r10, -0x254c
	ctx.r[3].s64 = ctx.r[10].s64 + -9548;
	// 826D8E64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8E68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8E70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8E74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8E78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8E7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8E80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8E84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8E88: 4BD8DF99  bl 0x82466e20
	ctx.lr = 0x826D8E8C;
	sub_82466E20(ctx, base);
	// 826D8E8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8E90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8E94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8EA0 size=108
    let mut pc: u32 = 0x826D8EA0;
    'dispatch: loop {
        match pc {
            0x826D8EA0 => {
    //   block [0x826D8EA0..0x826D8F0C)
	// 826D8EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8EAC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8EB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D8EB4: 38EB49E8  addi r7, r11, 0x49e8
	ctx.r[7].s64 = ctx.r[11].s64 + 18920;
	// 826D8EB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D8EBC: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826D8EC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8EC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8EC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8ED0: 386ADAE4  addi r3, r10, -0x251c
	ctx.r[3].s64 = ctx.r[10].s64 + -9500;
	// 826D8ED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8ED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8EE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8EE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8EE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8EEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8EF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8EF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8EF8: 4BD8DF29  bl 0x82466e20
	ctx.lr = 0x826D8EFC;
	sub_82466E20(ctx, base);
	// 826D8EFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8F10 size=108
    let mut pc: u32 = 0x826D8F10;
    'dispatch: loop {
        match pc {
            0x826D8F10 => {
    //   block [0x826D8F10..0x826D8F7C)
	// 826D8F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8F1C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8F20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D8F24: 38EB4A00  addi r7, r11, 0x4a00
	ctx.r[7].s64 = ctx.r[11].s64 + 18944;
	// 826D8F28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D8F2C: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 826D8F30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8F34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8F38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8F40: 386ADB14  addi r3, r10, -0x24ec
	ctx.r[3].s64 = ctx.r[10].s64 + -9452;
	// 826D8F44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8F48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8F4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8F50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8F58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8F60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8F64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8F68: 4BD8DEB9  bl 0x82466e20
	ctx.lr = 0x826D8F6C;
	sub_82466E20(ctx, base);
	// 826D8F6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8F70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8F74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8F78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8F80 size=108
    let mut pc: u32 = 0x826D8F80;
    'dispatch: loop {
        match pc {
            0x826D8F80 => {
    //   block [0x826D8F80..0x826D8FEC)
	// 826D8F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8F8C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D8F90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D8F94: 38EB4A68  addi r7, r11, 0x4a68
	ctx.r[7].s64 = ctx.r[11].s64 + 19048;
	// 826D8F98: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826D8F9C: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 826D8FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D8FA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D8FA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D8FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D8FB0: 386ADB44  addi r3, r10, -0x24bc
	ctx.r[3].s64 = ctx.r[10].s64 + -9404;
	// 826D8FB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D8FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D8FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D8FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D8FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D8FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D8FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D8FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D8FD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D8FD8: 4BD8DE49  bl 0x82466e20
	ctx.lr = 0x826D8FDC;
	sub_82466E20(ctx, base);
	// 826D8FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D8FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D8FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D8FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D8FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D8FF0 size=116
    let mut pc: u32 = 0x826D8FF0;
    'dispatch: loop {
        match pc {
            0x826D8FF0 => {
    //   block [0x826D8FF0..0x826D9064)
	// 826D8FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D8FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D8FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D8FFC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9000: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D9004: 390B4B28  addi r8, r11, 0x4b28
	ctx.r[8].s64 = ctx.r[11].s64 + 19240;
	// 826D9008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D900C: 392A5154  addi r9, r10, 0x5154
	ctx.r[9].s64 = ctx.r[10].s64 + 20820;
	// 826D9010: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9014: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826D9018: 38AADB14  addi r5, r10, -0x24ec
	ctx.r[5].s64 = ctx.r[10].s64 + -9452;
	// 826D901C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9024: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D902C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9034: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D9038: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 826D903C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D9040: 386BDB74  addi r3, r11, -0x248c
	ctx.r[3].s64 = ctx.r[11].s64 + -9356;
	// 826D9044: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D9048: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D904C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9050: 4BD8DDD1  bl 0x82466e20
	ctx.lr = 0x826D9054;
	sub_82466E20(ctx, base);
	// 826D9054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D905C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9068 size=112
    let mut pc: u32 = 0x826D9068;
    'dispatch: loop {
        match pc {
            0x826D9068 => {
    //   block [0x826D9068..0x826D90D8)
	// 826D9068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9078: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D907C: 38AAFEB4  addi r5, r10, -0x14c
	ctx.r[5].s64 = ctx.r[10].s64 + -332;
	// 826D9080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9084: 390B4BB8  addi r8, r11, 0x4bb8
	ctx.r[8].s64 = ctx.r[11].s64 + 19384;
	// 826D9088: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D908C: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 826D9090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9094: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9098: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D909C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D90A0: 386ADBA4  addi r3, r10, -0x245c
	ctx.r[3].s64 = ctx.r[10].s64 + -9308;
	// 826D90A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D90A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D90AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D90B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D90B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D90B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D90BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D90C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D90C4: 4BD8DD5D  bl 0x82466e20
	ctx.lr = 0x826D90C8;
	sub_82466E20(ctx, base);
	// 826D90C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D90CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D90D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D90D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D90D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D90D8 size=96
    let mut pc: u32 = 0x826D90D8;
    'dispatch: loop {
        match pc {
            0x826D90D8 => {
    //   block [0x826D90D8..0x826D9138)
	// 826D90D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D90DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D90E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D90E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D90E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D90EC: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 826D90F0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D90F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D90F8: 386ADBD4  addi r3, r10, -0x242c
	ctx.r[3].s64 = ctx.r[10].s64 + -9260;
	// 826D90FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9104: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D9108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D910C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9118: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D911C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9124: 4BD8DCFD  bl 0x82466e20
	ctx.lr = 0x826D9128;
	sub_82466E20(ctx, base);
	// 826D9128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D912C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9138 size=24
    let mut pc: u32 = 0x826D9138;
    'dispatch: loop {
        match pc {
            0x826D9138 => {
    //   block [0x826D9138..0x826D9150)
	// 826D9138: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D913C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9140: 394AB048  addi r10, r10, -0x4fb8
	ctx.r[10].s64 = ctx.r[10].s64 + -20408;
	// 826D9144: 816B4A64  lwz r11, 0x4a64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19044 as u32) ) } as u64;
	// 826D9148: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D914C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9150 size=116
    let mut pc: u32 = 0x826D9150;
    'dispatch: loop {
        match pc {
            0x826D9150 => {
    //   block [0x826D9150..0x826D91C4)
	// 826D9150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D915C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D9160: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D9164: 390BB048  addi r8, r11, -0x4fb8
	ctx.r[8].s64 = ctx.r[11].s64 + -20408;
	// 826D9168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D916C: 392A51A0  addi r9, r10, 0x51a0
	ctx.r[9].s64 = ctx.r[10].s64 + 20896;
	// 826D9170: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9174: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826D9178: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D917C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9184: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D918C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D9190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9194: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D9198: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 826D919C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D91A0: 386BDC04  addi r3, r11, -0x23fc
	ctx.r[3].s64 = ctx.r[11].s64 + -9212;
	// 826D91A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D91A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D91AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D91B0: 4BD8DC71  bl 0x82466e20
	ctx.lr = 0x826D91B4;
	sub_82466E20(ctx, base);
	// 826D91B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D91B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D91BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D91C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D91C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D91C8 size=104
    let mut pc: u32 = 0x826D91C8;
    'dispatch: loop {
        match pc {
            0x826D91C8 => {
    //   block [0x826D91C8..0x826D9230)
	// 826D91C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D91CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D91D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D91D4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D91D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D91DC: 392A51CC  addi r9, r10, 0x51cc
	ctx.r[9].s64 = ctx.r[10].s64 + 20940;
	// 826D91E0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D91E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D91E8: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D91EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D91F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D91F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D91F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D91FC: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 826D9200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9204: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9208: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D920C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9210: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9214: 386ADC34  addi r3, r10, -0x23cc
	ctx.r[3].s64 = ctx.r[10].s64 + -9164;
	// 826D9218: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D921C: 4BD8DC05  bl 0x82466e20
	ctx.lr = 0x826D9220;
	sub_82466E20(ctx, base);
	// 826D9220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D922C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9230 size=96
    let mut pc: u32 = 0x826D9230;
    'dispatch: loop {
        match pc {
            0x826D9230 => {
    //   block [0x826D9230..0x826D9290)
	// 826D9230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D923C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9244: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 826D9248: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D924C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9250: 386ADC64  addi r3, r10, -0x239c
	ctx.r[3].s64 = ctx.r[10].s64 + -9116;
	// 826D9254: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D925C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D9260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D926C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9270: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D9274: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9278: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D927C: 4BD8DBA5  bl 0x82466e20
	ctx.lr = 0x826D9280;
	sub_82466E20(ctx, base);
	// 826D9280: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9284: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9288: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D928C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9290 size=96
    let mut pc: u32 = 0x826D9290;
    'dispatch: loop {
        match pc {
            0x826D9290 => {
    //   block [0x826D9290..0x826D92F0)
	// 826D9290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D929C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D92A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D92A4: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 826D92A8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D92AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D92B0: 386ADC94  addi r3, r10, -0x236c
	ctx.r[3].s64 = ctx.r[10].s64 + -9068;
	// 826D92B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D92B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D92BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D92C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D92C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D92C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D92CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D92D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D92D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D92D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D92DC: 4BD8DB45  bl 0x82466e20
	ctx.lr = 0x826D92E0;
	sub_82466E20(ctx, base);
	// 826D92E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D92E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D92E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D92EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D92F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D92F0 size=100
    let mut pc: u32 = 0x826D92F0;
    'dispatch: loop {
        match pc {
            0x826D92F0 => {
    //   block [0x826D92F0..0x826D9354)
	// 826D92F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D92F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D92F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D92FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9304: 38AADC34  addi r5, r10, -0x23cc
	ctx.r[5].s64 = ctx.r[10].s64 + -9164;
	// 826D9308: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D930C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9310: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 826D9314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D931C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9324: 386ADCC4  addi r3, r10, -0x233c
	ctx.r[3].s64 = ctx.r[10].s64 + -9020;
	// 826D9328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D932C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9330: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D9334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9338: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D933C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9340: 4BD8DAE1  bl 0x82466e20
	ctx.lr = 0x826D9344;
	sub_82466E20(ctx, base);
	// 826D9344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D934C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9358 size=112
    let mut pc: u32 = 0x826D9358;
    'dispatch: loop {
        match pc {
            0x826D9358 => {
    //   block [0x826D9358..0x826D93C8)
	// 826D9358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D935C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9364: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9368: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D936C: 38AADC04  addi r5, r10, -0x23fc
	ctx.r[5].s64 = ctx.r[10].s64 + -9212;
	// 826D9370: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9374: 390B4C20  addi r8, r11, 0x4c20
	ctx.r[8].s64 = ctx.r[11].s64 + 19488;
	// 826D9378: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D937C: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 826D9380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9384: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D938C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9390: 386ADCF4  addi r3, r10, -0x230c
	ctx.r[3].s64 = ctx.r[10].s64 + -8972;
	// 826D9394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D939C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D93A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D93A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D93A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D93AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D93B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D93B4: 4BD8DA6D  bl 0x82466e20
	ctx.lr = 0x826D93B8;
	sub_82466E20(ctx, base);
	// 826D93B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D93BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D93C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D93C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D93C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D93C8 size=112
    let mut pc: u32 = 0x826D93C8;
    'dispatch: loop {
        match pc {
            0x826D93C8 => {
    //   block [0x826D93C8..0x826D9438)
	// 826D93C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D93CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D93D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D93D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D93D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D93DC: 38AADC04  addi r5, r10, -0x23fc
	ctx.r[5].s64 = ctx.r[10].s64 + -9212;
	// 826D93E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D93E4: 390B4C68  addi r8, r11, 0x4c68
	ctx.r[8].s64 = ctx.r[11].s64 + 19560;
	// 826D93E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D93EC: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 826D93F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D93F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D93F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D93FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9400: 386ADD24  addi r3, r10, -0x22dc
	ctx.r[3].s64 = ctx.r[10].s64 + -8924;
	// 826D9404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D940C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D941C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9424: 4BD8D9FD  bl 0x82466e20
	ctx.lr = 0x826D9428;
	sub_82466E20(ctx, base);
	// 826D9428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D942C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9438 size=100
    let mut pc: u32 = 0x826D9438;
    'dispatch: loop {
        match pc {
            0x826D9438 => {
    //   block [0x826D9438..0x826D949C)
	// 826D9438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D943C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9444: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D944C: 38AADC04  addi r5, r10, -0x23fc
	ctx.r[5].s64 = ctx.r[10].s64 + -9212;
	// 826D9450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9458: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 826D945C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D946C: 386ADD54  addi r3, r10, -0x22ac
	ctx.r[3].s64 = ctx.r[10].s64 + -8876;
	// 826D9470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9474: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9478: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D947C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9480: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9488: 4BD8D999  bl 0x82466e20
	ctx.lr = 0x826D948C;
	sub_82466E20(ctx, base);
	// 826D948C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D94A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D94A0 size=112
    let mut pc: u32 = 0x826D94A0;
    'dispatch: loop {
        match pc {
            0x826D94A0 => {
    //   block [0x826D94A0..0x826D9510)
	// 826D94A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D94A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D94A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D94AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D94B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D94B4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D94B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D94BC: 390B4C80  addi r8, r11, 0x4c80
	ctx.r[8].s64 = ctx.r[11].s64 + 19584;
	// 826D94C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D94C4: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 826D94C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D94CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D94D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D94D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D94D8: 386ADD84  addi r3, r10, -0x227c
	ctx.r[3].s64 = ctx.r[10].s64 + -8828;
	// 826D94DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D94E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D94E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D94E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D94EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D94F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D94F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D94F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D94FC: 4BD8D925  bl 0x82466e20
	ctx.lr = 0x826D9500;
	sub_82466E20(ctx, base);
	// 826D9500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D950C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9510 size=96
    let mut pc: u32 = 0x826D9510;
    'dispatch: loop {
        match pc {
            0x826D9510 => {
    //   block [0x826D9510..0x826D9570)
	// 826D9510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D951C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9524: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 826D9528: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D952C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9530: 386ADDB4  addi r3, r10, -0x224c
	ctx.r[3].s64 = ctx.r[10].s64 + -8780;
	// 826D9534: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D953C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D9540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D954C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9550: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D9554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D955C: 4BD8D8C5  bl 0x82466e20
	ctx.lr = 0x826D9560;
	sub_82466E20(ctx, base);
	// 826D9560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D956C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9570 size=112
    let mut pc: u32 = 0x826D9570;
    'dispatch: loop {
        match pc {
            0x826D9570 => {
    //   block [0x826D9570..0x826D95E0)
	// 826D9570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D957C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9580: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9584: 38AADDB4  addi r5, r10, -0x224c
	ctx.r[5].s64 = ctx.r[10].s64 + -8780;
	// 826D9588: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D958C: 390B4CB0  addi r8, r11, 0x4cb0
	ctx.r[8].s64 = ctx.r[11].s64 + 19632;
	// 826D9590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D9594: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 826D9598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D959C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D95A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D95A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D95A8: 386ADDE4  addi r3, r10, -0x221c
	ctx.r[3].s64 = ctx.r[10].s64 + -8732;
	// 826D95AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D95B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D95B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D95B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D95BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D95C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D95C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D95C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D95CC: 4BD8D855  bl 0x82466e20
	ctx.lr = 0x826D95D0;
	sub_82466E20(ctx, base);
	// 826D95D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D95D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D95D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D95DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D95E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D95E0 size=112
    let mut pc: u32 = 0x826D95E0;
    'dispatch: loop {
        match pc {
            0x826D95E0 => {
    //   block [0x826D95E0..0x826D9650)
	// 826D95E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D95E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D95E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D95EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D95F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D95F4: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826D95F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D95FC: 390B4CC8  addi r8, r11, 0x4cc8
	ctx.r[8].s64 = ctx.r[11].s64 + 19656;
	// 826D9600: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D9604: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 826D9608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D960C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9618: 386ADE14  addi r3, r10, -0x21ec
	ctx.r[3].s64 = ctx.r[10].s64 + -8684;
	// 826D961C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D962C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D9630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D963C: 4BD8D7E5  bl 0x82466e20
	ctx.lr = 0x826D9640;
	sub_82466E20(ctx, base);
	// 826D9640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D964C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9650 size=112
    let mut pc: u32 = 0x826D9650;
    'dispatch: loop {
        match pc {
            0x826D9650 => {
    //   block [0x826D9650..0x826D96C0)
	// 826D9650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D965C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9660: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9664: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826D9668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D966C: 390B4CE0  addi r8, r11, 0x4ce0
	ctx.r[8].s64 = ctx.r[11].s64 + 19680;
	// 826D9670: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D9674: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 826D9678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D967C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9688: 386ADE44  addi r3, r10, -0x21bc
	ctx.r[3].s64 = ctx.r[10].s64 + -8636;
	// 826D968C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D969C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D96A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D96A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D96A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D96AC: 4BD8D775  bl 0x82466e20
	ctx.lr = 0x826D96B0;
	sub_82466E20(ctx, base);
	// 826D96B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D96B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D96B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D96BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D96C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D96C0 size=36
    let mut pc: u32 = 0x826D96C0;
    'dispatch: loop {
        match pc {
            0x826D96C0 => {
    //   block [0x826D96C0..0x826D96E4)
	// 826D96C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D96C4: 814B4D2C  lwz r10, 0x4d2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19756 as u32) ) } as u64;
	// 826D96C8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D96CC: 396BB078  addi r11, r11, -0x4f88
	ctx.r[11].s64 = ctx.r[11].s64 + -20360;
	// 826D96D0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826D96D4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D96D8: 814A4C1C  lwz r10, 0x4c1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(19484 as u32) ) } as u64;
	// 826D96DC: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826D96E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D96E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D96E8 size=108
    let mut pc: u32 = 0x826D96E8;
    'dispatch: loop {
        match pc {
            0x826D96E8 => {
    //   block [0x826D96E8..0x826D9754)
	// 826D96E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D96EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D96F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D96F4: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D96F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D96FC: 38EBB078  addi r7, r11, -0x4f88
	ctx.r[7].s64 = ctx.r[11].s64 + -20360;
	// 826D9700: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D9704: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 826D9708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D970C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D9714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9718: 386ADE74  addi r3, r10, -0x218c
	ctx.r[3].s64 = ctx.r[10].s64 + -8588;
	// 826D971C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D9720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D972C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D973C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9740: 4BD8D6E1  bl 0x82466e20
	ctx.lr = 0x826D9744;
	sub_82466E20(ctx, base);
	// 826D9744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D974C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9758 size=24
    let mut pc: u32 = 0x826D9758;
    'dispatch: loop {
        match pc {
            0x826D9758 => {
    //   block [0x826D9758..0x826D9770)
	// 826D9758: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D975C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9760: 394AB120  addi r10, r10, -0x4ee0
	ctx.r[10].s64 = ctx.r[10].s64 + -20192;
	// 826D9764: 816B4C1C  lwz r11, 0x4c1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19484 as u32) ) } as u64;
	// 826D9768: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826D976C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9770 size=116
    let mut pc: u32 = 0x826D9770;
    'dispatch: loop {
        match pc {
            0x826D9770 => {
    //   block [0x826D9770..0x826D97E4)
	// 826D9770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D977C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9780: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826D9784: 390AB120  addi r8, r10, -0x4ee0
	ctx.r[8].s64 = ctx.r[10].s64 + -20192;
	// 826D9788: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D978C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D9790: 38AADE74  addi r5, r10, -0x218c
	ctx.r[5].s64 = ctx.r[10].s64 + -8588;
	// 826D9794: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9798: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D979C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D97A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D97A4: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 826D97A8: 396B526C  addi r11, r11, 0x526c
	ctx.r[11].s64 = ctx.r[11].s64 + 21100;
	// 826D97AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D97B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D97B4: 386ADEA4  addi r3, r10, -0x215c
	ctx.r[3].s64 = ctx.r[10].s64 + -8540;
	// 826D97B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D97BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D97C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D97C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D97C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D97CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D97D0: 4BD8D651  bl 0x82466e20
	ctx.lr = 0x826D97D4;
	sub_82466E20(ctx, base);
	// 826D97D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D97D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D97DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D97E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D97E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D97E8 size=112
    let mut pc: u32 = 0x826D97E8;
    'dispatch: loop {
        match pc {
            0x826D97E8 => {
    //   block [0x826D97E8..0x826D9858)
	// 826D97E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D97EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D97F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D97F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D97F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D97FC: 38AADE74  addi r5, r10, -0x218c
	ctx.r[5].s64 = ctx.r[10].s64 + -8588;
	// 826D9800: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9804: 390B4D30  addi r8, r11, 0x4d30
	ctx.r[8].s64 = ctx.r[11].s64 + 19760;
	// 826D9808: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D980C: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 826D9810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D981C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9820: 386ADED4  addi r3, r10, -0x212c
	ctx.r[3].s64 = ctx.r[10].s64 + -8492;
	// 826D9824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D982C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D983C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9844: 4BD8D5DD  bl 0x82466e20
	ctx.lr = 0x826D9848;
	sub_82466E20(ctx, base);
	// 826D9848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D984C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9858 size=24
    let mut pc: u32 = 0x826D9858;
    'dispatch: loop {
        match pc {
            0x826D9858 => {
    //   block [0x826D9858..0x826D9870)
	// 826D9858: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D985C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9860: 394AB210  addi r10, r10, -0x4df0
	ctx.r[10].s64 = ctx.r[10].s64 + -19952;
	// 826D9864: 816B532C  lwz r11, 0x532c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21292 as u32) ) } as u64;
	// 826D9868: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826D986C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9870 size=116
    let mut pc: u32 = 0x826D9870;
    'dispatch: loop {
        match pc {
            0x826D9870 => {
    //   block [0x826D9870..0x826D98E4)
	// 826D9870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D987C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D9880: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9884: 392B5230  addi r9, r11, 0x5230
	ctx.r[9].s64 = ctx.r[11].s64 + 21040;
	// 826D9888: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826D988C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9890: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826D9894: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 826D9898: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D989C: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 826D98A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D98A4: 396BB210  addi r11, r11, -0x4df0
	ctx.r[11].s64 = ctx.r[11].s64 + -19952;
	// 826D98A8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D98AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D98B0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D98B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D98B8: 386ADF04  addi r3, r10, -0x20fc
	ctx.r[3].s64 = ctx.r[10].s64 + -8444;
	// 826D98BC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826D98C0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D98C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D98C8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D98CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D98D0: 4BD8D551  bl 0x82466e20
	ctx.lr = 0x826D98D4;
	sub_82466E20(ctx, base);
	// 826D98D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D98D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D98DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D98E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D98E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D98E8 size=100
    let mut pc: u32 = 0x826D98E8;
    'dispatch: loop {
        match pc {
            0x826D98E8 => {
    //   block [0x826D98E8..0x826D994C)
	// 826D98E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D98EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D98F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D98F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D98F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D98FC: 38AAE024  addi r5, r10, -0x1fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -8156;
	// 826D9900: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9908: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 826D990C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D991C: 386ADF34  addi r3, r10, -0x20cc
	ctx.r[3].s64 = ctx.r[10].s64 + -8396;
	// 826D9920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9924: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9928: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D992C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9930: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9938: 4BD8D4E9  bl 0x82466e20
	ctx.lr = 0x826D993C;
	sub_82466E20(ctx, base);
	// 826D993C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9950 size=108
    let mut pc: u32 = 0x826D9950;
    'dispatch: loop {
        match pc {
            0x826D9950 => {
    //   block [0x826D9950..0x826D99BC)
	// 826D9950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D995C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9960: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9964: 38EB4DA8  addi r7, r11, 0x4da8
	ctx.r[7].s64 = ctx.r[11].s64 + 19880;
	// 826D9968: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D996C: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 826D9970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9974: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9978: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D997C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9980: 386ADF64  addi r3, r10, -0x209c
	ctx.r[3].s64 = ctx.r[10].s64 + -8348;
	// 826D9984: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D9988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D998C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D999C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D99A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D99A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D99A8: 4BD8D479  bl 0x82466e20
	ctx.lr = 0x826D99AC;
	sub_82466E20(ctx, base);
	// 826D99AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D99B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D99B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D99B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D99C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D99C0 size=112
    let mut pc: u32 = 0x826D99C0;
    'dispatch: loop {
        match pc {
            0x826D99C0 => {
    //   block [0x826D99C0..0x826D9A30)
	// 826D99C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D99C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D99C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D99CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D99D0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D99D4: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826D99D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D99DC: 390B4E08  addi r8, r11, 0x4e08
	ctx.r[8].s64 = ctx.r[11].s64 + 19976;
	// 826D99E0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D99E4: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 826D99E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D99EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D99F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D99F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D99F8: 386ADF94  addi r3, r10, -0x206c
	ctx.r[3].s64 = ctx.r[10].s64 + -8300;
	// 826D99FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9A1C: 4BD8D405  bl 0x82466e20
	ctx.lr = 0x826D9A20;
	sub_82466E20(ctx, base);
	// 826D9A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9A30 size=108
    let mut pc: u32 = 0x826D9A30;
    'dispatch: loop {
        match pc {
            0x826D9A30 => {
    //   block [0x826D9A30..0x826D9A9C)
	// 826D9A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9A38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9A3C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9A40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9A44: 38EB4E68  addi r7, r11, 0x4e68
	ctx.r[7].s64 = ctx.r[11].s64 + 20072;
	// 826D9A48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D9A4C: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 826D9A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9A54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9A58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D9A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9A60: 386ADFC4  addi r3, r10, -0x203c
	ctx.r[3].s64 = ctx.r[10].s64 + -8252;
	// 826D9A64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D9A68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9A6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9A7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9A84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9A88: 4BD8D399  bl 0x82466e20
	ctx.lr = 0x826D9A8C;
	sub_82466E20(ctx, base);
	// 826D9A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9AA0 size=28
    let mut pc: u32 = 0x826D9AA0;
    'dispatch: loop {
        match pc {
            0x826D9AA0 => {
    //   block [0x826D9AA0..0x826D9ABC)
	// 826D9AA0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9AA4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9AA8: 394AB300  addi r10, r10, -0x4d00
	ctx.r[10].s64 = ctx.r[10].s64 + -19712;
	// 826D9AAC: 816B4E80  lwz r11, 0x4e80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20096 as u32) ) } as u64;
	// 826D9AB0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826D9AB4: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826D9AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9AC0 size=112
    let mut pc: u32 = 0x826D9AC0;
    'dispatch: loop {
        match pc {
            0x826D9AC0 => {
    //   block [0x826D9AC0..0x826D9B30)
	// 826D9AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9ACC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9AD0: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 826D9AD4: 38EAB300  addi r7, r10, -0x4d00
	ctx.r[7].s64 = ctx.r[10].s64 + -19712;
	// 826D9AD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9ADC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D9AE0: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 826D9AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9AE8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D9AEC: 396B5330  addi r11, r11, 0x5330
	ctx.r[11].s64 = ctx.r[11].s64 + 21296;
	// 826D9AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D9AF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9AF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9AFC: 386ADFF4  addi r3, r10, -0x200c
	ctx.r[3].s64 = ctx.r[10].s64 + -8204;
	// 826D9B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9B04: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D9B08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9B0C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D9B10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9B14: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9B18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9B1C: 4BD8D305  bl 0x82466e20
	ctx.lr = 0x826D9B20;
	sub_82466E20(ctx, base);
	// 826D9B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9B30 size=24
    let mut pc: u32 = 0x826D9B30;
    'dispatch: loop {
        match pc {
            0x826D9B30 => {
    //   block [0x826D9B30..0x826D9B48)
	// 826D9B30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9B34: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9B38: 394AB468  addi r10, r10, -0x4b98
	ctx.r[10].s64 = ctx.r[10].s64 + -19352;
	// 826D9B3C: 816B532C  lwz r11, 0x532c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21292 as u32) ) } as u64;
	// 826D9B40: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826D9B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9B48 size=116
    let mut pc: u32 = 0x826D9B48;
    'dispatch: loop {
        match pc {
            0x826D9B48 => {
    //   block [0x826D9B48..0x826D9BBC)
	// 826D9B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9B54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D9B58: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9B5C: 392B5304  addi r9, r11, 0x5304
	ctx.r[9].s64 = ctx.r[11].s64 + 21252;
	// 826D9B60: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826D9B64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9B68: 38E9006C  addi r7, r9, 0x6c
	ctx.r[7].s64 = ctx.r[9].s64 + 108;
	// 826D9B6C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826D9B70: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D9B74: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 826D9B78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9B7C: 396BB468  addi r11, r11, -0x4b98
	ctx.r[11].s64 = ctx.r[11].s64 + -19352;
	// 826D9B80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D9B84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9B88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D9B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9B90: 386AE024  addi r3, r10, -0x1fdc
	ctx.r[3].s64 = ctx.r[10].s64 + -8156;
	// 826D9B94: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826D9B98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D9B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9BA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D9BA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9BA8: 4BD8D279  bl 0x82466e20
	ctx.lr = 0x826D9BAC;
	sub_82466E20(ctx, base);
	// 826D9BAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9BC0 size=108
    let mut pc: u32 = 0x826D9BC0;
    'dispatch: loop {
        match pc {
            0x826D9BC0 => {
    //   block [0x826D9BC0..0x826D9C2C)
	// 826D9BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9BCC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9BD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9BD4: 38EB4E88  addi r7, r11, 0x4e88
	ctx.r[7].s64 = ctx.r[11].s64 + 20104;
	// 826D9BD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D9BDC: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 826D9BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9BE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D9BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9BF0: 386AE054  addi r3, r10, -0x1fac
	ctx.r[3].s64 = ctx.r[10].s64 + -8108;
	// 826D9BF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D9BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9C14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D9C18: 4BD8D209  bl 0x82466e20
	ctx.lr = 0x826D9C1C;
	sub_82466E20(ctx, base);
	// 826D9C1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D9C30 size=24
    let mut pc: u32 = 0x826D9C30;
    'dispatch: loop {
        match pc {
            0x826D9C30 => {
    //   block [0x826D9C30..0x826D9C48)
	// 826D9C30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9C34: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9C38: 394AB510  addi r10, r10, -0x4af0
	ctx.r[10].s64 = ctx.r[10].s64 + -19184;
	// 826D9C3C: 816B532C  lwz r11, 0x532c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21292 as u32) ) } as u64;
	// 826D9C40: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826D9C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9C48 size=116
    let mut pc: u32 = 0x826D9C48;
    'dispatch: loop {
        match pc {
            0x826D9C48 => {
    //   block [0x826D9C48..0x826D9CBC)
	// 826D9C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9C54: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D9C58: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D9C5C: 390AB510  addi r8, r10, -0x4af0
	ctx.r[8].s64 = ctx.r[10].s64 + -19184;
	// 826D9C60: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9C64: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D9C68: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826D9C6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9C70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D9C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9C7C: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 826D9C80: 396B5390  addi r11, r11, 0x5390
	ctx.r[11].s64 = ctx.r[11].s64 + 21392;
	// 826D9C84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9C88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9C8C: 386AE084  addi r3, r10, -0x1f7c
	ctx.r[3].s64 = ctx.r[10].s64 + -8060;
	// 826D9C90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D9C94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9C98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D9C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9CA8: 4BD8D179  bl 0x82466e20
	ctx.lr = 0x826D9CAC;
	sub_82466E20(ctx, base);
	// 826D9CAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9CB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9CC0 size=112
    let mut pc: u32 = 0x826D9CC0;
    'dispatch: loop {
        match pc {
            0x826D9CC0 => {
    //   block [0x826D9CC0..0x826D9D30)
	// 826D9CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9CCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9CD0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9CD4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D9CD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9CDC: 390B4EE8  addi r8, r11, 0x4ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 20200;
	// 826D9CE0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D9CE4: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 826D9CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9CEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9CF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9CF8: 386AE0B4  addi r3, r10, -0x1f4c
	ctx.r[3].s64 = ctx.r[10].s64 + -8012;
	// 826D9CFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9D00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9D08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9D10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9D14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9D18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9D1C: 4BD8D105  bl 0x82466e20
	ctx.lr = 0x826D9D20;
	sub_82466E20(ctx, base);
	// 826D9D20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9D30 size=112
    let mut pc: u32 = 0x826D9D30;
    'dispatch: loop {
        match pc {
            0x826D9D30 => {
    //   block [0x826D9D30..0x826D9DA0)
	// 826D9D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9D3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9D40: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9D44: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D9D48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9D4C: 390B4F78  addi r8, r11, 0x4f78
	ctx.r[8].s64 = ctx.r[11].s64 + 20344;
	// 826D9D50: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D9D54: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 826D9D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9D5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9D60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9D64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9D68: 386AE0E4  addi r3, r10, -0x1f1c
	ctx.r[3].s64 = ctx.r[10].s64 + -7964;
	// 826D9D6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9D74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9D8C: 4BD8D095  bl 0x82466e20
	ctx.lr = 0x826D9D90;
	sub_82466E20(ctx, base);
	// 826D9D90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9D94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9D98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9DA0 size=112
    let mut pc: u32 = 0x826D9DA0;
    'dispatch: loop {
        match pc {
            0x826D9DA0 => {
    //   block [0x826D9DA0..0x826D9E10)
	// 826D9DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9DAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9DB0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9DB4: 38AADF04  addi r5, r10, -0x20fc
	ctx.r[5].s64 = ctx.r[10].s64 + -8444;
	// 826D9DB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9DBC: 390B4FD8  addi r8, r11, 0x4fd8
	ctx.r[8].s64 = ctx.r[11].s64 + 20440;
	// 826D9DC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D9DC4: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 826D9DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9DCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9DD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9DD8: 386AE114  addi r3, r10, -0x1eec
	ctx.r[3].s64 = ctx.r[10].s64 + -7916;
	// 826D9DDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9DEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9DFC: 4BD8D025  bl 0x82466e20
	ctx.lr = 0x826D9E00;
	sub_82466E20(ctx, base);
	// 826D9E00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9E04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9E08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9E10 size=112
    let mut pc: u32 = 0x826D9E10;
    'dispatch: loop {
        match pc {
            0x826D9E10 => {
    //   block [0x826D9E10..0x826D9E80)
	// 826D9E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9E1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9E20: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9E24: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D9E28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9E2C: 390B5008  addi r8, r11, 0x5008
	ctx.r[8].s64 = ctx.r[11].s64 + 20488;
	// 826D9E30: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D9E34: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 826D9E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9E3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9E40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9E48: 386AE144  addi r3, r10, -0x1ebc
	ctx.r[3].s64 = ctx.r[10].s64 + -7868;
	// 826D9E4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9E50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9E58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9E5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9E60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9E68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9E6C: 4BD8CFB5  bl 0x82466e20
	ctx.lr = 0x826D9E70;
	sub_82466E20(ctx, base);
	// 826D9E70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9E74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9E78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9E80 size=112
    let mut pc: u32 = 0x826D9E80;
    'dispatch: loop {
        match pc {
            0x826D9E80 => {
    //   block [0x826D9E80..0x826D9EF0)
	// 826D9E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9E8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9E90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9E94: 38AAE024  addi r5, r10, -0x1fdc
	ctx.r[5].s64 = ctx.r[10].s64 + -8156;
	// 826D9E98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9E9C: 390B5098  addi r8, r11, 0x5098
	ctx.r[8].s64 = ctx.r[11].s64 + 20632;
	// 826D9EA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D9EA4: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 826D9EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9EAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9EB8: 386AE174  addi r3, r10, -0x1e8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7820;
	// 826D9EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9EDC: 4BD8CF45  bl 0x82466e20
	ctx.lr = 0x826D9EE0;
	sub_82466E20(ctx, base);
	// 826D9EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9EF0 size=100
    let mut pc: u32 = 0x826D9EF0;
    'dispatch: loop {
        match pc {
            0x826D9EF0 => {
    //   block [0x826D9EF0..0x826D9F54)
	// 826D9EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9EFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9F04: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826D9F08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9F10: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 826D9F14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9F18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9F20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9F24: 386AE1A4  addi r3, r10, -0x1e5c
	ctx.r[3].s64 = ctx.r[10].s64 + -7772;
	// 826D9F28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9F2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9F30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D9F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9F38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D9F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9F40: 4BD8CEE1  bl 0x82466e20
	ctx.lr = 0x826D9F44;
	sub_82466E20(ctx, base);
	// 826D9F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9F58 size=112
    let mut pc: u32 = 0x826D9F58;
    'dispatch: loop {
        match pc {
            0x826D9F58 => {
    //   block [0x826D9F58..0x826D9FC8)
	// 826D9F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9F64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9F68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9F6C: 38AAE1A4  addi r5, r10, -0x1e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -7772;
	// 826D9F70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9F74: 390B50B0  addi r8, r11, 0x50b0
	ctx.r[8].s64 = ctx.r[11].s64 + 20656;
	// 826D9F78: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D9F7C: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 826D9F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9F84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9F88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D9F90: 386AE1D4  addi r3, r10, -0x1e2c
	ctx.r[3].s64 = ctx.r[10].s64 + -7724;
	// 826D9F94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D9F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D9F9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D9FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D9FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D9FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D9FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D9FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D9FB4: 4BD8CE6D  bl 0x82466e20
	ctx.lr = 0x826D9FB8;
	sub_82466E20(ctx, base);
	// 826D9FB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D9FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D9FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D9FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D9FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D9FC8 size=112
    let mut pc: u32 = 0x826D9FC8;
    'dispatch: loop {
        match pc {
            0x826D9FC8 => {
    //   block [0x826D9FC8..0x826DA038)
	// 826D9FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D9FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D9FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D9FD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9FD8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D9FDC: 38AAE1D4  addi r5, r10, -0x1e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -7724;
	// 826D9FE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D9FE4: 390B5110  addi r8, r11, 0x5110
	ctx.r[8].s64 = ctx.r[11].s64 + 20752;
	// 826D9FE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D9FEC: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 826D9FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D9FF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D9FF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D9FFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA000: 386AE204  addi r3, r10, -0x1dfc
	ctx.r[3].s64 = ctx.r[10].s64 + -7676;
	// 826DA004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA024: 4BD8CDFD  bl 0x82466e20
	ctx.lr = 0x826DA028;
	sub_82466E20(ctx, base);
	// 826DA028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DA038 size=24
    let mut pc: u32 = 0x826DA038;
    'dispatch: loop {
        match pc {
            0x826DA038 => {
    //   block [0x826DA038..0x826DA050)
	// 826DA038: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA03C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DA040: 394AB588  addi r10, r10, -0x4a78
	ctx.r[10].s64 = ctx.r[10].s64 + -19064;
	// 826DA044: 816B532C  lwz r11, 0x532c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21292 as u32) ) } as u64;
	// 826DA048: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826DA04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA050 size=116
    let mut pc: u32 = 0x826DA050;
    'dispatch: loop {
        match pc {
            0x826DA050 => {
    //   block [0x826DA050..0x826DA0C4)
	// 826DA050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA05C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DA060: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826DA064: 390AB588  addi r8, r10, -0x4a78
	ctx.r[8].s64 = ctx.r[10].s64 + -19064;
	// 826DA068: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA06C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DA070: 38AAE1D4  addi r5, r10, -0x1e2c
	ctx.r[5].s64 = ctx.r[10].s64 + -7724;
	// 826DA074: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA078: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DA07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA084: 388AB78C  addi r4, r10, -0x4874
	ctx.r[4].s64 = ctx.r[10].s64 + -18548;
	// 826DA088: 396B53A8  addi r11, r11, 0x53a8
	ctx.r[11].s64 = ctx.r[11].s64 + 21416;
	// 826DA08C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA090: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DA094: 386AE234  addi r3, r10, -0x1dcc
	ctx.r[3].s64 = ctx.r[10].s64 + -7628;
	// 826DA098: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DA09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA0A0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DA0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA0B0: 4BD8CD71  bl 0x82466e20
	ctx.lr = 0x826DA0B4;
	sub_82466E20(ctx, base);
	// 826DA0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA0C8 size=112
    let mut pc: u32 = 0x826DA0C8;
    'dispatch: loop {
        match pc {
            0x826DA0C8 => {
    //   block [0x826DA0C8..0x826DA138)
	// 826DA0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA0D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA0D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA0DC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DA0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA0E4: 390B5140  addi r8, r11, 0x5140
	ctx.r[8].s64 = ctx.r[11].s64 + 20800;
	// 826DA0E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DA0EC: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 826DA0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA0F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA100: 386AE264  addi r3, r10, -0x1d9c
	ctx.r[3].s64 = ctx.r[10].s64 + -7580;
	// 826DA104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA114: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DA118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA124: 4BD8CCFD  bl 0x82466e20
	ctx.lr = 0x826DA128;
	sub_82466E20(ctx, base);
	// 826DA128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA138 size=116
    let mut pc: u32 = 0x826DA138;
    'dispatch: loop {
        match pc {
            0x826DA138 => {
    //   block [0x826DA138..0x826DA1AC)
	// 826DA138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA144: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA148: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DA14C: 390B5174  addi r8, r11, 0x5174
	ctx.r[8].s64 = ctx.r[11].s64 + 20852;
	// 826DA150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA154: 392A53E8  addi r9, r10, 0x53e8
	ctx.r[9].s64 = ctx.r[10].s64 + 21480;
	// 826DA158: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA15C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826DA160: 38AAE534  addi r5, r10, -0x1acc
	ctx.r[5].s64 = ctx.r[10].s64 + -6860;
	// 826DA164: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA16C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA17C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DA180: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 826DA184: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DA188: 386BE294  addi r3, r11, -0x1d6c
	ctx.r[3].s64 = ctx.r[11].s64 + -7532;
	// 826DA18C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DA190: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA198: 4BD8CC89  bl 0x82466e20
	ctx.lr = 0x826DA19C;
	sub_82466E20(ctx, base);
	// 826DA19C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA1B0 size=112
    let mut pc: u32 = 0x826DA1B0;
    'dispatch: loop {
        match pc {
            0x826DA1B0 => {
    //   block [0x826DA1B0..0x826DA220)
	// 826DA1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA1B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA1BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA1C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA1C4: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA1C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA1CC: 390B518C  addi r8, r11, 0x518c
	ctx.r[8].s64 = ctx.r[11].s64 + 20876;
	// 826DA1D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DA1D4: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 826DA1D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA1DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA1E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA1E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA1E8: 386AE2C4  addi r3, r10, -0x1d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -7484;
	// 826DA1EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA1F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA1F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA20C: 4BD8CC15  bl 0x82466e20
	ctx.lr = 0x826DA210;
	sub_82466E20(ctx, base);
	// 826DA210: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA21C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA220 size=116
    let mut pc: u32 = 0x826DA220;
    'dispatch: loop {
        match pc {
            0x826DA220 => {
    //   block [0x826DA220..0x826DA294)
	// 826DA220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA22C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA230: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DA234: 390B51A8  addi r8, r11, 0x51a8
	ctx.r[8].s64 = ctx.r[11].s64 + 20904;
	// 826DA238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA23C: 392A5414  addi r9, r10, 0x5414
	ctx.r[9].s64 = ctx.r[10].s64 + 21524;
	// 826DA240: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA244: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826DA248: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA24C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA254: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA264: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DA268: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 826DA26C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DA270: 386BE2F4  addi r3, r11, -0x1d0c
	ctx.r[3].s64 = ctx.r[11].s64 + -7436;
	// 826DA274: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DA278: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA280: 4BD8CBA1  bl 0x82466e20
	ctx.lr = 0x826DA284;
	sub_82466E20(ctx, base);
	// 826DA284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA298 size=112
    let mut pc: u32 = 0x826DA298;
    'dispatch: loop {
        match pc {
            0x826DA298 => {
    //   block [0x826DA298..0x826DA308)
	// 826DA298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA2A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA2A8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA2AC: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA2B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA2B4: 390B51D8  addi r8, r11, 0x51d8
	ctx.r[8].s64 = ctx.r[11].s64 + 20952;
	// 826DA2B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DA2BC: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 826DA2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA2C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA2D0: 386AE324  addi r3, r10, -0x1cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -7388;
	// 826DA2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA2F4: 4BD8CB2D  bl 0x82466e20
	ctx.lr = 0x826DA2F8;
	sub_82466E20(ctx, base);
	// 826DA2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA308 size=112
    let mut pc: u32 = 0x826DA308;
    'dispatch: loop {
        match pc {
            0x826DA308 => {
    //   block [0x826DA308..0x826DA378)
	// 826DA308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA318: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA31C: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA320: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA324: 390B5220  addi r8, r11, 0x5220
	ctx.r[8].s64 = ctx.r[11].s64 + 21024;
	// 826DA328: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DA32C: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 826DA330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA338: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA340: 386AE354  addi r3, r10, -0x1cac
	ctx.r[3].s64 = ctx.r[10].s64 + -7340;
	// 826DA344: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA34C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA35C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA364: 4BD8CABD  bl 0x82466e20
	ctx.lr = 0x826DA368;
	sub_82466E20(ctx, base);
	// 826DA368: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA36C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA370: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA378 size=108
    let mut pc: u32 = 0x826DA378;
    'dispatch: loop {
        match pc {
            0x826DA378 => {
    //   block [0x826DA378..0x826DA3E4)
	// 826DA378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA384: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA388: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA38C: 38EB5268  addi r7, r11, 0x5268
	ctx.r[7].s64 = ctx.r[11].s64 + 21096;
	// 826DA390: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DA394: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 826DA398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA39C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA3A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DA3A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA3A8: 386AE384  addi r3, r10, -0x1c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -7292;
	// 826DA3AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DA3B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA3B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA3C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA3C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA3CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DA3D0: 4BD8CA51  bl 0x82466e20
	ctx.lr = 0x826DA3D4;
	sub_82466E20(ctx, base);
	// 826DA3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA3E8 size=112
    let mut pc: u32 = 0x826DA3E8;
    'dispatch: loop {
        match pc {
            0x826DA3E8 => {
    //   block [0x826DA3E8..0x826DA458)
	// 826DA3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA3F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA3F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA3FC: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA404: 390B52B0  addi r8, r11, 0x52b0
	ctx.r[8].s64 = ctx.r[11].s64 + 21168;
	// 826DA408: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DA40C: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 826DA410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA41C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA420: 386AE3B4  addi r3, r10, -0x1c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -7244;
	// 826DA424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA444: 4BD8C9DD  bl 0x82466e20
	ctx.lr = 0x826DA448;
	sub_82466E20(ctx, base);
	// 826DA448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA458 size=116
    let mut pc: u32 = 0x826DA458;
    'dispatch: loop {
        match pc {
            0x826DA458 => {
    //   block [0x826DA458..0x826DA4CC)
	// 826DA458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA464: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DA468: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA46C: 392B5450  addi r9, r11, 0x5450
	ctx.r[9].s64 = ctx.r[11].s64 + 21584;
	// 826DA470: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA474: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA478: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826DA47C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826DA480: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA484: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 826DA488: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA48C: 396B5330  addi r11, r11, 0x5330
	ctx.r[11].s64 = ctx.r[11].s64 + 21296;
	// 826DA490: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826DA494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA498: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826DA49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA4A0: 386AE3E4  addi r3, r10, -0x1c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -7196;
	// 826DA4A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DA4A8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826DA4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA4B0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826DA4B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DA4B8: 4BD8C969  bl 0x82466e20
	ctx.lr = 0x826DA4BC;
	sub_82466E20(ctx, base);
	// 826DA4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA4D0 size=108
    let mut pc: u32 = 0x826DA4D0;
    'dispatch: loop {
        match pc {
            0x826DA4D0 => {
    //   block [0x826DA4D0..0x826DA53C)
	// 826DA4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA4DC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA4E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA4E4: 38EB53C0  addi r7, r11, 0x53c0
	ctx.r[7].s64 = ctx.r[11].s64 + 21440;
	// 826DA4E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DA4EC: 388AB870  addi r4, r10, -0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + -18320;
	// 826DA4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA4F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA4F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DA4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA500: 386AE414  addi r3, r10, -0x1bec
	ctx.r[3].s64 = ctx.r[10].s64 + -7148;
	// 826DA504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DA508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DA528: 4BD8C8F9  bl 0x82466e20
	ctx.lr = 0x826DA52C;
	sub_82466E20(ctx, base);
	// 826DA52C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA540 size=108
    let mut pc: u32 = 0x826DA540;
    'dispatch: loop {
        match pc {
            0x826DA540 => {
    //   block [0x826DA540..0x826DA5AC)
	// 826DA540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA54C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA550: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA554: 38EB5408  addi r7, r11, 0x5408
	ctx.r[7].s64 = ctx.r[11].s64 + 21512;
	// 826DA558: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826DA55C: 388AB898  addi r4, r10, -0x4768
	ctx.r[4].s64 = ctx.r[10].s64 + -18280;
	// 826DA560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DA56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA570: 386AE444  addi r3, r10, -0x1bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -7100;
	// 826DA574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DA578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DA598: 4BD8C889  bl 0x82466e20
	ctx.lr = 0x826DA59C;
	sub_82466E20(ctx, base);
	// 826DA59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA5B0 size=112
    let mut pc: u32 = 0x826DA5B0;
    'dispatch: loop {
        match pc {
            0x826DA5B0 => {
    //   block [0x826DA5B0..0x826DA620)
	// 826DA5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA5BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA5C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA5C4: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA5C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA5CC: 390B5468  addi r8, r11, 0x5468
	ctx.r[8].s64 = ctx.r[11].s64 + 21608;
	// 826DA5D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DA5D4: 388AB8C0  addi r4, r10, -0x4740
	ctx.r[4].s64 = ctx.r[10].s64 + -18240;
	// 826DA5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA5DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA5E8: 386AE474  addi r3, r10, -0x1b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -7052;
	// 826DA5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA60C: 4BD8C815  bl 0x82466e20
	ctx.lr = 0x826DA610;
	sub_82466E20(ctx, base);
	// 826DA610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA620 size=112
    let mut pc: u32 = 0x826DA620;
    'dispatch: loop {
        match pc {
            0x826DA620 => {
    //   block [0x826DA620..0x826DA690)
	// 826DA620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA62C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA630: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA634: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA638: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA63C: 390B54C8  addi r8, r11, 0x54c8
	ctx.r[8].s64 = ctx.r[11].s64 + 21704;
	// 826DA640: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DA644: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 826DA648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA64C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA658: 386AE4A4  addi r3, r10, -0x1b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -7004;
	// 826DA65C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA67C: 4BD8C7A5  bl 0x82466e20
	ctx.lr = 0x826DA680;
	sub_82466E20(ctx, base);
	// 826DA680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA68C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DA690 size=24
    let mut pc: u32 = 0x826DA690;
    'dispatch: loop {
        match pc {
            0x826DA690 => {
    //   block [0x826DA690..0x826DA6A8)
	// 826DA690: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA694: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DA698: 394AB660  addi r10, r10, -0x49a0
	ctx.r[10].s64 = ctx.r[10].s64 + -18848;
	// 826DA69C: 816B532C  lwz r11, 0x532c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21292 as u32) ) } as u64;
	// 826DA6A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826DA6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA6A8 size=116
    let mut pc: u32 = 0x826DA6A8;
    'dispatch: loop {
        match pc {
            0x826DA6A8 => {
    //   block [0x826DA6A8..0x826DA71C)
	// 826DA6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA6B4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DA6B8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826DA6BC: 390AB660  addi r8, r10, -0x49a0
	ctx.r[8].s64 = ctx.r[10].s64 + -18848;
	// 826DA6C0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA6C4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DA6C8: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DA6CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA6D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DA6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA6D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA6DC: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 826DA6E0: 396B5480  addi r11, r11, 0x5480
	ctx.r[11].s64 = ctx.r[11].s64 + 21632;
	// 826DA6E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA6E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA6EC: 386AE4D4  addi r3, r10, -0x1b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -6956;
	// 826DA6F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DA6F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA6F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DA6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA708: 4BD8C719  bl 0x82466e20
	ctx.lr = 0x826DA70C;
	sub_82466E20(ctx, base);
	// 826DA70C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA720 size=100
    let mut pc: u32 = 0x826DA720;
    'dispatch: loop {
        match pc {
            0x826DA720 => {
    //   block [0x826DA720..0x826DA784)
	// 826DA720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA72C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA734: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DA738: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA740: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 826DA744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA754: 386AE504  addi r3, r10, -0x1afc
	ctx.r[3].s64 = ctx.r[10].s64 + -6908;
	// 826DA758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA75C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA760: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DA764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA768: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DA76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA770: 4BD8C6B1  bl 0x82466e20
	ctx.lr = 0x826DA774;
	sub_82466E20(ctx, base);
	// 826DA774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA778: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA77C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA788 size=100
    let mut pc: u32 = 0x826DA788;
    'dispatch: loop {
        match pc {
            0x826DA788 => {
    //   block [0x826DA788..0x826DA7EC)
	// 826DA788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA79C: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DA7A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA7A8: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 826DA7AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA7BC: 386AE534  addi r3, r10, -0x1acc
	ctx.r[3].s64 = ctx.r[10].s64 + -6860;
	// 826DA7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA7C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA7C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DA7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA7D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DA7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA7D8: 4BD8C649  bl 0x82466e20
	ctx.lr = 0x826DA7DC;
	sub_82466E20(ctx, base);
	// 826DA7DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA7F0 size=112
    let mut pc: u32 = 0x826DA7F0;
    'dispatch: loop {
        match pc {
            0x826DA7F0 => {
    //   block [0x826DA7F0..0x826DA860)
	// 826DA7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA7FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA800: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA804: 38AAE504  addi r5, r10, -0x1afc
	ctx.r[5].s64 = ctx.r[10].s64 + -6908;
	// 826DA808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA80C: 390B54E0  addi r8, r11, 0x54e0
	ctx.r[8].s64 = ctx.r[11].s64 + 21728;
	// 826DA810: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DA814: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 826DA818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA81C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA828: 386AE564  addi r3, r10, -0x1a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -6812;
	// 826DA82C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA83C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA84C: 4BD8C5D5  bl 0x82466e20
	ctx.lr = 0x826DA850;
	sub_82466E20(ctx, base);
	// 826DA850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA860 size=112
    let mut pc: u32 = 0x826DA860;
    'dispatch: loop {
        match pc {
            0x826DA860 => {
    //   block [0x826DA860..0x826DA8D0)
	// 826DA860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA86C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA870: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA874: 38AAE504  addi r5, r10, -0x1afc
	ctx.r[5].s64 = ctx.r[10].s64 + -6908;
	// 826DA878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA87C: 390B5528  addi r8, r11, 0x5528
	ctx.r[8].s64 = ctx.r[11].s64 + 21800;
	// 826DA880: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826DA884: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 826DA888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA88C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA898: 386AE594  addi r3, r10, -0x1a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -6764;
	// 826DA89C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA8A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA8B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA8BC: 4BD8C565  bl 0x82466e20
	ctx.lr = 0x826DA8C0;
	sub_82466E20(ctx, base);
	// 826DA8C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA8C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA8C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA8D0 size=112
    let mut pc: u32 = 0x826DA8D0;
    'dispatch: loop {
        match pc {
            0x826DA8D0 => {
    //   block [0x826DA8D0..0x826DA940)
	// 826DA8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA8DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA8E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA8E4: 38AAE594  addi r5, r10, -0x1a6c
	ctx.r[5].s64 = ctx.r[10].s64 + -6764;
	// 826DA8E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA8EC: 390B55D0  addi r8, r11, 0x55d0
	ctx.r[8].s64 = ctx.r[11].s64 + 21968;
	// 826DA8F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DA8F4: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 826DA8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA8FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA900: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA908: 386AE5C4  addi r3, r10, -0x1a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -6716;
	// 826DA90C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA92C: 4BD8C4F5  bl 0x82466e20
	ctx.lr = 0x826DA930;
	sub_82466E20(ctx, base);
	// 826DA930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA940 size=112
    let mut pc: u32 = 0x826DA940;
    'dispatch: loop {
        match pc {
            0x826DA940 => {
    //   block [0x826DA940..0x826DA9B0)
	// 826DA940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA94C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA950: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA954: 38AAE1A4  addi r5, r10, -0x1e5c
	ctx.r[5].s64 = ctx.r[10].s64 + -7772;
	// 826DA958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA95C: 390B5600  addi r8, r11, 0x5600
	ctx.r[8].s64 = ctx.r[11].s64 + 22016;
	// 826DA960: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DA964: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 826DA968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA96C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA978: 386AE5F4  addi r3, r10, -0x1a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -6668;
	// 826DA97C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA98C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DA990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DA994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DA998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DA99C: 4BD8C485  bl 0x82466e20
	ctx.lr = 0x826DA9A0;
	sub_82466E20(ctx, base);
	// 826DA9A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DA9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DA9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DA9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DA9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DA9B0 size=112
    let mut pc: u32 = 0x826DA9B0;
    'dispatch: loop {
        match pc {
            0x826DA9B0 => {
    //   block [0x826DA9B0..0x826DAA20)
	// 826DA9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DA9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DA9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DA9BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA9C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DA9C4: 38AADE14  addi r5, r10, -0x21ec
	ctx.r[5].s64 = ctx.r[10].s64 + -8684;
	// 826DA9C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DA9CC: 390B5630  addi r8, r11, 0x5630
	ctx.r[8].s64 = ctx.r[11].s64 + 22064;
	// 826DA9D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DA9D4: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 826DA9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DA9DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DA9E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DA9E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DA9E8: 386AE624  addi r3, r10, -0x19dc
	ctx.r[3].s64 = ctx.r[10].s64 + -6620;
	// 826DA9EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DA9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DA9F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DA9F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DA9FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAA00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAA0C: 4BD8C415  bl 0x82466e20
	ctx.lr = 0x826DAA10;
	sub_82466E20(ctx, base);
	// 826DAA10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAA14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAA18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAA1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAA20 size=112
    let mut pc: u32 = 0x826DAA20;
    'dispatch: loop {
        match pc {
            0x826DAA20 => {
    //   block [0x826DAA20..0x826DAA90)
	// 826DAA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAA2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAA30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAA34: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DAA38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAA3C: 390B5660  addi r8, r11, 0x5660
	ctx.r[8].s64 = ctx.r[11].s64 + 22112;
	// 826DAA40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DAA44: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 826DAA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAA4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAA50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAA58: 386AE654  addi r3, r10, -0x19ac
	ctx.r[3].s64 = ctx.r[10].s64 + -6572;
	// 826DAA5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DAA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAA7C: 4BD8C3A5  bl 0x82466e20
	ctx.lr = 0x826DAA80;
	sub_82466E20(ctx, base);
	// 826DAA80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAA90 size=112
    let mut pc: u32 = 0x826DAA90;
    'dispatch: loop {
        match pc {
            0x826DAA90 => {
    //   block [0x826DAA90..0x826DAB00)
	// 826DAA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAA9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAAA0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAAA4: 38AAE294  addi r5, r10, -0x1d6c
	ctx.r[5].s64 = ctx.r[10].s64 + -7532;
	// 826DAAA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAAAC: 390B5690  addi r8, r11, 0x5690
	ctx.r[8].s64 = ctx.r[11].s64 + 22160;
	// 826DAAB0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DAAB4: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 826DAAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAABC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAAC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAAC8: 386AE684  addi r3, r10, -0x197c
	ctx.r[3].s64 = ctx.r[10].s64 + -6524;
	// 826DAACC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DAAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAAD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAADC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DAAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAAEC: 4BD8C335  bl 0x82466e20
	ctx.lr = 0x826DAAF0;
	sub_82466E20(ctx, base);
	// 826DAAF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAB00 size=108
    let mut pc: u32 = 0x826DAB00;
    'dispatch: loop {
        match pc {
            0x826DAB00 => {
    //   block [0x826DAB00..0x826DAB6C)
	// 826DAB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAB0C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAB10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAB14: 38EB5708  addi r7, r11, 0x5708
	ctx.r[7].s64 = ctx.r[11].s64 + 22280;
	// 826DAB18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DAB1C: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 826DAB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAB24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAB28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DAB2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAB30: 386AE6B4  addi r3, r10, -0x194c
	ctx.r[3].s64 = ctx.r[10].s64 + -6476;
	// 826DAB34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DAB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAB54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DAB58: 4BD8C2C9  bl 0x82466e20
	ctx.lr = 0x826DAB5C;
	sub_82466E20(ctx, base);
	// 826DAB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAB70 size=112
    let mut pc: u32 = 0x826DAB70;
    'dispatch: loop {
        match pc {
            0x826DAB70 => {
    //   block [0x826DAB70..0x826DABE0)
	// 826DAB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAB7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAB80: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAB84: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DAB88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAB8C: 390B5738  addi r8, r11, 0x5738
	ctx.r[8].s64 = ctx.r[11].s64 + 22328;
	// 826DAB90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DAB94: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 826DAB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAB9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DABA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DABA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DABA8: 386AE6E4  addi r3, r10, -0x191c
	ctx.r[3].s64 = ctx.r[10].s64 + -6428;
	// 826DABAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DABB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DABB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DABB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DABBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DABC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DABC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DABC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DABCC: 4BD8C255  bl 0x82466e20
	ctx.lr = 0x826DABD0;
	sub_82466E20(ctx, base);
	// 826DABD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DABD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DABD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DABDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DABE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DABE0 size=112
    let mut pc: u32 = 0x826DABE0;
    'dispatch: loop {
        match pc {
            0x826DABE0 => {
    //   block [0x826DABE0..0x826DAC50)
	// 826DABE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DABE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DABE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DABEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DABF0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DABF4: 38AAE534  addi r5, r10, -0x1acc
	ctx.r[5].s64 = ctx.r[10].s64 + -6860;
	// 826DABF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DABFC: 390B5768  addi r8, r11, 0x5768
	ctx.r[8].s64 = ctx.r[11].s64 + 22376;
	// 826DAC00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DAC04: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 826DAC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAC0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAC10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAC18: 386AE714  addi r3, r10, -0x18ec
	ctx.r[3].s64 = ctx.r[10].s64 + -6380;
	// 826DAC1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DAC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAC24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAC28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAC2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAC30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAC38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAC3C: 4BD8C1E5  bl 0x82466e20
	ctx.lr = 0x826DAC40;
	sub_82466E20(ctx, base);
	// 826DAC40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAC44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAC48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAC4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAC50 size=100
    let mut pc: u32 = 0x826DAC50;
    'dispatch: loop {
        match pc {
            0x826DAC50 => {
    //   block [0x826DAC50..0x826DACB4)
	// 826DAC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAC5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAC64: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DAC68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAC70: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 826DAC74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAC84: 386AE744  addi r3, r10, -0x18bc
	ctx.r[3].s64 = ctx.r[10].s64 + -6332;
	// 826DAC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAC8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAC90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DAC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAC98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DAC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DACA0: 4BD8C181  bl 0x82466e20
	ctx.lr = 0x826DACA4;
	sub_82466E20(ctx, base);
	// 826DACA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DACA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DACAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DACB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DACB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DACB8 size=112
    let mut pc: u32 = 0x826DACB8;
    'dispatch: loop {
        match pc {
            0x826DACB8 => {
    //   block [0x826DACB8..0x826DAD28)
	// 826DACB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DACBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DACC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DACC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DACC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DACCC: 38AADD84  addi r5, r10, -0x227c
	ctx.r[5].s64 = ctx.r[10].s64 + -8828;
	// 826DACD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DACD4: 390B5798  addi r8, r11, 0x5798
	ctx.r[8].s64 = ctx.r[11].s64 + 22424;
	// 826DACD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DACDC: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 826DACE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DACE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DACE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DACEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DACF0: 386AE774  addi r3, r10, -0x188c
	ctx.r[3].s64 = ctx.r[10].s64 + -6284;
	// 826DACF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DACF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DACFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAD14: 4BD8C10D  bl 0x82466e20
	ctx.lr = 0x826DAD18;
	sub_82466E20(ctx, base);
	// 826DAD18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAD1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAD20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAD24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAD28 size=96
    let mut pc: u32 = 0x826DAD28;
    'dispatch: loop {
        match pc {
            0x826DAD28 => {
    //   block [0x826DAD28..0x826DAD88)
	// 826DAD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAD30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAD34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAD3C: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 826DAD40: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAD48: 386AE7A4  addi r3, r10, -0x185c
	ctx.r[3].s64 = ctx.r[10].s64 + -6236;
	// 826DAD4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAD54: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DAD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAD5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAD68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DAD6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DAD70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DAD74: 4BD8C0AD  bl 0x82466e20
	ctx.lr = 0x826DAD78;
	sub_82466E20(ctx, base);
	// 826DAD78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAD88 size=108
    let mut pc: u32 = 0x826DAD88;
    'dispatch: loop {
        match pc {
            0x826DAD88 => {
    //   block [0x826DAD88..0x826DADF4)
	// 826DAD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAD94: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAD98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAD9C: 38EB57E0  addi r7, r11, 0x57e0
	ctx.r[7].s64 = ctx.r[11].s64 + 22496;
	// 826DADA0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826DADA4: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 826DADA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DADAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DADB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DADB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DADB8: 386AE7D4  addi r3, r10, -0x182c
	ctx.r[3].s64 = ctx.r[10].s64 + -6188;
	// 826DADBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DADC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DADC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DADC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DADCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DADD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DADD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DADD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DADDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DADE0: 4BD8C041  bl 0x82466e20
	ctx.lr = 0x826DADE4;
	sub_82466E20(ctx, base);
	// 826DADE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DADE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DADEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DADF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DADF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DADF8 size=100
    let mut pc: u32 = 0x826DADF8;
    'dispatch: loop {
        match pc {
            0x826DADF8 => {
    //   block [0x826DADF8..0x826DAE5C)
	// 826DADF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DADFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAE04: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DAE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAE0C: 392A54F0  addi r9, r10, 0x54f0
	ctx.r[9].s64 = ctx.r[10].s64 + 21744;
	// 826DAE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAE14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAE18: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 826DAE1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAE24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAE2C: 386AE804  addi r3, r10, -0x17fc
	ctx.r[3].s64 = ctx.r[10].s64 + -6140;
	// 826DAE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAE34: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826DAE38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DAE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAE40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DAE44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DAE48: 4BD8BFD9  bl 0x82466e20
	ctx.lr = 0x826DAE4C;
	sub_82466E20(ctx, base);
	// 826DAE4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DAE60 size=24
    let mut pc: u32 = 0x826DAE60;
    'dispatch: loop {
        match pc {
            0x826DAE60 => {
    //   block [0x826DAE60..0x826DAE78)
	// 826DAE60: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAE64: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DAE68: 394AB708  addi r10, r10, -0x48f8
	ctx.r[10].s64 = ctx.r[10].s64 + -18680;
	// 826DAE6C: 816B5848  lwz r11, 0x5848(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22600 as u32) ) } as u64;
	// 826DAE70: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826DAE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAE78 size=112
    let mut pc: u32 = 0x826DAE78;
    'dispatch: loop {
        match pc {
            0x826DAE78 => {
    //   block [0x826DAE78..0x826DAEE8)
	// 826DAE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAE84: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DAE88: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DAE8C: 392A5630  addi r9, r10, 0x5630
	ctx.r[9].s64 = ctx.r[10].s64 + 22064;
	// 826DAE90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAE94: 390BB708  addi r8, r11, -0x48f8
	ctx.r[8].s64 = ctx.r[11].s64 + -18680;
	// 826DAE98: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826DAE9C: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 826DAEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAEA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAEA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAEB0: 386AE834  addi r3, r10, -0x17cc
	ctx.r[3].s64 = ctx.r[10].s64 + -6092;
	// 826DAEB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DAEB8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826DAEBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAEC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DAED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAED4: 4BD8BF4D  bl 0x82466e20
	ctx.lr = 0x826DAED8;
	sub_82466E20(ctx, base);
	// 826DAED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAEE8 size=112
    let mut pc: u32 = 0x826DAEE8;
    'dispatch: loop {
        match pc {
            0x826DAEE8 => {
    //   block [0x826DAEE8..0x826DAF58)
	// 826DAEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAEF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAEF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAEFC: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DAF00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAF04: 390B5850  addi r8, r11, 0x5850
	ctx.r[8].s64 = ctx.r[11].s64 + 22608;
	// 826DAF08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DAF0C: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 826DAF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAF14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAF18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAF20: 386AE864  addi r3, r10, -0x179c
	ctx.r[3].s64 = ctx.r[10].s64 + -6044;
	// 826DAF24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DAF28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAF2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAF3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAF44: 4BD8BEDD  bl 0x82466e20
	ctx.lr = 0x826DAF48;
	sub_82466E20(ctx, base);
	// 826DAF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAF58 size=108
    let mut pc: u32 = 0x826DAF58;
    'dispatch: loop {
        match pc {
            0x826DAF58 => {
    //   block [0x826DAF58..0x826DAFC4)
	// 826DAF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAF64: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAF68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAF6C: 38EB5880  addi r7, r11, 0x5880
	ctx.r[7].s64 = ctx.r[11].s64 + 22656;
	// 826DAF70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DAF74: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 826DAF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAF7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAF80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DAF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DAF88: 386AE894  addi r3, r10, -0x176c
	ctx.r[3].s64 = ctx.r[10].s64 + -5996;
	// 826DAF8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DAF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DAF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DAF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DAF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DAFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DAFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DAFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DAFAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DAFB0: 4BD8BE71  bl 0x82466e20
	ctx.lr = 0x826DAFB4;
	sub_82466E20(ctx, base);
	// 826DAFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DAFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DAFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DAFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DAFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DAFC8 size=112
    let mut pc: u32 = 0x826DAFC8;
    'dispatch: loop {
        match pc {
            0x826DAFC8 => {
    //   block [0x826DAFC8..0x826DB038)
	// 826DAFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DAFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DAFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DAFD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAFD8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DAFDC: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DAFE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DAFE4: 390B5898  addi r8, r11, 0x5898
	ctx.r[8].s64 = ctx.r[11].s64 + 22680;
	// 826DAFE8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826DAFEC: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 826DAFF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DAFF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DAFF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DAFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB000: 386AE8C4  addi r3, r10, -0x173c
	ctx.r[3].s64 = ctx.r[10].s64 + -5948;
	// 826DB004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB024: 4BD8BDFD  bl 0x82466e20
	ctx.lr = 0x826DB028;
	sub_82466E20(ctx, base);
	// 826DB028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB038 size=100
    let mut pc: u32 = 0x826DB038;
    'dispatch: loop {
        match pc {
            0x826DB038 => {
    //   block [0x826DB038..0x826DB09C)
	// 826DB038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB044: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB04C: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB050: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB058: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 826DB05C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB06C: 386AE8F4  addi r3, r10, -0x170c
	ctx.r[3].s64 = ctx.r[10].s64 + -5900;
	// 826DB070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB074: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB078: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DB07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB080: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DB084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB088: 4BD8BD99  bl 0x82466e20
	ctx.lr = 0x826DB08C;
	sub_82466E20(ctx, base);
	// 826DB08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB0A0 size=112
    let mut pc: u32 = 0x826DB0A0;
    'dispatch: loop {
        match pc {
            0x826DB0A0 => {
    //   block [0x826DB0A0..0x826DB110)
	// 826DB0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB0AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB0B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB0B4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB0B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB0BC: 390B5940  addi r8, r11, 0x5940
	ctx.r[8].s64 = ctx.r[11].s64 + 22848;
	// 826DB0C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DB0C4: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 826DB0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB0CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB0D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB0D8: 386AE924  addi r3, r10, -0x16dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5852;
	// 826DB0DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB0FC: 4BD8BD25  bl 0x82466e20
	ctx.lr = 0x826DB100;
	sub_82466E20(ctx, base);
	// 826DB100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB110 size=112
    let mut pc: u32 = 0x826DB110;
    'dispatch: loop {
        match pc {
            0x826DB110 => {
    //   block [0x826DB110..0x826DB180)
	// 826DB110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB11C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB120: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB124: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB128: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB12C: 390B5958  addi r8, r11, 0x5958
	ctx.r[8].s64 = ctx.r[11].s64 + 22872;
	// 826DB130: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DB134: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 826DB138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB13C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB148: 386AE954  addi r3, r10, -0x16ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5804;
	// 826DB14C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB16C: 4BD8BCB5  bl 0x82466e20
	ctx.lr = 0x826DB170;
	sub_82466E20(ctx, base);
	// 826DB170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB180 size=112
    let mut pc: u32 = 0x826DB180;
    'dispatch: loop {
        match pc {
            0x826DB180 => {
    //   block [0x826DB180..0x826DB1F0)
	// 826DB180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB18C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB190: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB194: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB198: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB19C: 390B5988  addi r8, r11, 0x5988
	ctx.r[8].s64 = ctx.r[11].s64 + 22920;
	// 826DB1A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DB1A4: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 826DB1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB1AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB1B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB1B8: 386AE984  addi r3, r10, -0x167c
	ctx.r[3].s64 = ctx.r[10].s64 + -5756;
	// 826DB1BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB1C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB1C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB1D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB1D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB1D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB1DC: 4BD8BC45  bl 0x82466e20
	ctx.lr = 0x826DB1E0;
	sub_82466E20(ctx, base);
	// 826DB1E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB1F0 size=112
    let mut pc: u32 = 0x826DB1F0;
    'dispatch: loop {
        match pc {
            0x826DB1F0 => {
    //   block [0x826DB1F0..0x826DB260)
	// 826DB1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB1FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB200: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB204: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB208: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB20C: 390B59B8  addi r8, r11, 0x59b8
	ctx.r[8].s64 = ctx.r[11].s64 + 22968;
	// 826DB210: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DB214: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 826DB218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB21C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB228: 386AE9B4  addi r3, r10, -0x164c
	ctx.r[3].s64 = ctx.r[10].s64 + -5708;
	// 826DB22C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB24C: 4BD8BBD5  bl 0x82466e20
	ctx.lr = 0x826DB250;
	sub_82466E20(ctx, base);
	// 826DB250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB260 size=112
    let mut pc: u32 = 0x826DB260;
    'dispatch: loop {
        match pc {
            0x826DB260 => {
    //   block [0x826DB260..0x826DB2D0)
	// 826DB260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB26C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB270: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB274: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB278: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB27C: 390B59E8  addi r8, r11, 0x59e8
	ctx.r[8].s64 = ctx.r[11].s64 + 23016;
	// 826DB280: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DB284: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 826DB288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB28C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB298: 386AE9E4  addi r3, r10, -0x161c
	ctx.r[3].s64 = ctx.r[10].s64 + -5660;
	// 826DB29C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB2A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB2A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB2BC: 4BD8BB65  bl 0x82466e20
	ctx.lr = 0x826DB2C0;
	sub_82466E20(ctx, base);
	// 826DB2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB2D0 size=112
    let mut pc: u32 = 0x826DB2D0;
    'dispatch: loop {
        match pc {
            0x826DB2D0 => {
    //   block [0x826DB2D0..0x826DB340)
	// 826DB2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB2DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB2E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB2E4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB2E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB2EC: 390B5A00  addi r8, r11, 0x5a00
	ctx.r[8].s64 = ctx.r[11].s64 + 23040;
	// 826DB2F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DB2F4: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 826DB2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB2FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB308: 386AEA14  addi r3, r10, -0x15ec
	ctx.r[3].s64 = ctx.r[10].s64 + -5612;
	// 826DB30C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB32C: 4BD8BAF5  bl 0x82466e20
	ctx.lr = 0x826DB330;
	sub_82466E20(ctx, base);
	// 826DB330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB340 size=112
    let mut pc: u32 = 0x826DB340;
    'dispatch: loop {
        match pc {
            0x826DB340 => {
    //   block [0x826DB340..0x826DB3B0)
	// 826DB340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB34C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB350: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB354: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB358: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB35C: 390B5A18  addi r8, r11, 0x5a18
	ctx.r[8].s64 = ctx.r[11].s64 + 23064;
	// 826DB360: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DB364: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 826DB368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB36C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB378: 386AEA44  addi r3, r10, -0x15bc
	ctx.r[3].s64 = ctx.r[10].s64 + -5564;
	// 826DB37C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB38C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB39C: 4BD8BA85  bl 0x82466e20
	ctx.lr = 0x826DB3A0;
	sub_82466E20(ctx, base);
	// 826DB3A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB3B0 size=112
    let mut pc: u32 = 0x826DB3B0;
    'dispatch: loop {
        match pc {
            0x826DB3B0 => {
    //   block [0x826DB3B0..0x826DB420)
	// 826DB3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB3BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB3C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB3C4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB3C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB3CC: 390B5A60  addi r8, r11, 0x5a60
	ctx.r[8].s64 = ctx.r[11].s64 + 23136;
	// 826DB3D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DB3D4: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 826DB3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB3DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB3E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB3E8: 386AEA74  addi r3, r10, -0x158c
	ctx.r[3].s64 = ctx.r[10].s64 + -5516;
	// 826DB3EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB3F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB40C: 4BD8BA15  bl 0x82466e20
	ctx.lr = 0x826DB410;
	sub_82466E20(ctx, base);
	// 826DB410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB420 size=112
    let mut pc: u32 = 0x826DB420;
    'dispatch: loop {
        match pc {
            0x826DB420 => {
    //   block [0x826DB420..0x826DB490)
	// 826DB420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB42C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB430: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB434: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB438: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB43C: 390B5AA8  addi r8, r11, 0x5aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 23208;
	// 826DB440: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DB444: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 826DB448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB44C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB458: 386AEAA4  addi r3, r10, -0x155c
	ctx.r[3].s64 = ctx.r[10].s64 + -5468;
	// 826DB45C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB47C: 4BD8B9A5  bl 0x82466e20
	ctx.lr = 0x826DB480;
	sub_82466E20(ctx, base);
	// 826DB480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB490 size=112
    let mut pc: u32 = 0x826DB490;
    'dispatch: loop {
        match pc {
            0x826DB490 => {
    //   block [0x826DB490..0x826DB500)
	// 826DB490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB49C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB4A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB4A4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB4A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB4AC: 390B5AC0  addi r8, r11, 0x5ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 23232;
	// 826DB4B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DB4B4: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 826DB4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB4BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB4C8: 386AEAD4  addi r3, r10, -0x152c
	ctx.r[3].s64 = ctx.r[10].s64 + -5420;
	// 826DB4CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB4D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB4E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB4EC: 4BD8B935  bl 0x82466e20
	ctx.lr = 0x826DB4F0;
	sub_82466E20(ctx, base);
	// 826DB4F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB500 size=116
    let mut pc: u32 = 0x826DB500;
    'dispatch: loop {
        match pc {
            0x826DB500 => {
    //   block [0x826DB500..0x826DB574)
	// 826DB500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB50C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DB510: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826DB514: 390A5AF0  addi r8, r10, 0x5af0
	ctx.r[8].s64 = ctx.r[10].s64 + 23280;
	// 826DB518: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB51C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DB520: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB524: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB528: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DB52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB534: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 826DB538: 396B5658  addi r11, r11, 0x5658
	ctx.r[11].s64 = ctx.r[11].s64 + 22104;
	// 826DB53C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB540: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB544: 386AEB04  addi r3, r10, -0x14fc
	ctx.r[3].s64 = ctx.r[10].s64 + -5372;
	// 826DB548: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DB54C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB550: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DB554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB560: 4BD8B8C1  bl 0x82466e20
	ctx.lr = 0x826DB564;
	sub_82466E20(ctx, base);
	// 826DB564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB578 size=116
    let mut pc: u32 = 0x826DB578;
    'dispatch: loop {
        match pc {
            0x826DB578 => {
    //   block [0x826DB578..0x826DB5EC)
	// 826DB578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB584: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DB588: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826DB58C: 390A5B68  addi r8, r10, 0x5b68
	ctx.r[8].s64 = ctx.r[10].s64 + 23400;
	// 826DB590: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB594: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DB598: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB59C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB5A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DB5A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB5AC: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 826DB5B0: 396B5670  addi r11, r11, 0x5670
	ctx.r[11].s64 = ctx.r[11].s64 + 22128;
	// 826DB5B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB5B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB5BC: 386AEB34  addi r3, r10, -0x14cc
	ctx.r[3].s64 = ctx.r[10].s64 + -5324;
	// 826DB5C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DB5C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB5C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DB5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB5D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB5D8: 4BD8B849  bl 0x82466e20
	ctx.lr = 0x826DB5DC;
	sub_82466E20(ctx, base);
	// 826DB5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DB5F0 size=24
    let mut pc: u32 = 0x826DB5F0;
    'dispatch: loop {
        match pc {
            0x826DB5F0 => {
    //   block [0x826DB5F0..0x826DB608)
	// 826DB5F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB5F4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DB5F8: 394AB720  addi r10, r10, -0x48e0
	ctx.r[10].s64 = ctx.r[10].s64 + -18656;
	// 826DB5FC: 816B5BF8  lwz r11, 0x5bf8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23544 as u32) ) } as u64;
	// 826DB600: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826DB604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB608 size=116
    let mut pc: u32 = 0x826DB608;
    'dispatch: loop {
        match pc {
            0x826DB608 => {
    //   block [0x826DB608..0x826DB67C)
	// 826DB608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB614: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DB618: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB61C: 392B569C  addi r9, r11, 0x569c
	ctx.r[9].s64 = ctx.r[11].s64 + 22172;
	// 826DB620: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB624: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB628: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826DB62C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826DB630: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DB634: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 826DB638: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB63C: 396BB720  addi r11, r11, -0x48e0
	ctx.r[11].s64 = ctx.r[11].s64 + -18656;
	// 826DB640: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826DB644: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB648: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826DB64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB650: 386AEB64  addi r3, r10, -0x149c
	ctx.r[3].s64 = ctx.r[10].s64 + -5276;
	// 826DB654: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DB658: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826DB65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB660: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826DB664: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DB668: 4BD8B7B9  bl 0x82466e20
	ctx.lr = 0x826DB66C;
	sub_82466E20(ctx, base);
	// 826DB66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB680 size=112
    let mut pc: u32 = 0x826DB680;
    'dispatch: loop {
        match pc {
            0x826DB680 => {
    //   block [0x826DB680..0x826DB6F0)
	// 826DB680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB68C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB690: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB694: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB69C: 390B5C00  addi r8, r11, 0x5c00
	ctx.r[8].s64 = ctx.r[11].s64 + 23552;
	// 826DB6A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DB6A4: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 826DB6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB6AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB6B8: 386AEB94  addi r3, r10, -0x146c
	ctx.r[3].s64 = ctx.r[10].s64 + -5228;
	// 826DB6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB6DC: 4BD8B745  bl 0x82466e20
	ctx.lr = 0x826DB6E0;
	sub_82466E20(ctx, base);
	// 826DB6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB6F0 size=112
    let mut pc: u32 = 0x826DB6F0;
    'dispatch: loop {
        match pc {
            0x826DB6F0 => {
    //   block [0x826DB6F0..0x826DB760)
	// 826DB6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB6FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB700: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB704: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB70C: 390B5C60  addi r8, r11, 0x5c60
	ctx.r[8].s64 = ctx.r[11].s64 + 23648;
	// 826DB710: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826DB714: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 826DB718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB71C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB728: 386AEBC4  addi r3, r10, -0x143c
	ctx.r[3].s64 = ctx.r[10].s64 + -5180;
	// 826DB72C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB74C: 4BD8B6D5  bl 0x82466e20
	ctx.lr = 0x826DB750;
	sub_82466E20(ctx, base);
	// 826DB750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB760 size=112
    let mut pc: u32 = 0x826DB760;
    'dispatch: loop {
        match pc {
            0x826DB760 => {
    //   block [0x826DB760..0x826DB7D0)
	// 826DB760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB76C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB770: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB774: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB77C: 390B5D08  addi r8, r11, 0x5d08
	ctx.r[8].s64 = ctx.r[11].s64 + 23816;
	// 826DB780: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DB784: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 826DB788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB78C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB798: 386AEBF4  addi r3, r10, -0x140c
	ctx.r[3].s64 = ctx.r[10].s64 + -5132;
	// 826DB79C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB7BC: 4BD8B665  bl 0x82466e20
	ctx.lr = 0x826DB7C0;
	sub_82466E20(ctx, base);
	// 826DB7C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB7D0 size=112
    let mut pc: u32 = 0x826DB7D0;
    'dispatch: loop {
        match pc {
            0x826DB7D0 => {
    //   block [0x826DB7D0..0x826DB840)
	// 826DB7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB7DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB7E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB7E4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB7E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB7EC: 390B5D80  addi r8, r11, 0x5d80
	ctx.r[8].s64 = ctx.r[11].s64 + 23936;
	// 826DB7F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DB7F4: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 826DB7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB7FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB808: 386AEC24  addi r3, r10, -0x13dc
	ctx.r[3].s64 = ctx.r[10].s64 + -5084;
	// 826DB80C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB82C: 4BD8B5F5  bl 0x82466e20
	ctx.lr = 0x826DB830;
	sub_82466E20(ctx, base);
	// 826DB830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB840 size=112
    let mut pc: u32 = 0x826DB840;
    'dispatch: loop {
        match pc {
            0x826DB840 => {
    //   block [0x826DB840..0x826DB8B0)
	// 826DB840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB84C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB850: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB854: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB85C: 390B5DC8  addi r8, r11, 0x5dc8
	ctx.r[8].s64 = ctx.r[11].s64 + 24008;
	// 826DB860: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826DB864: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 826DB868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB86C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB878: 386AEC54  addi r3, r10, -0x13ac
	ctx.r[3].s64 = ctx.r[10].s64 + -5036;
	// 826DB87C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB89C: 4BD8B585  bl 0x82466e20
	ctx.lr = 0x826DB8A0;
	sub_82466E20(ctx, base);
	// 826DB8A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB8B0 size=112
    let mut pc: u32 = 0x826DB8B0;
    'dispatch: loop {
        match pc {
            0x826DB8B0 => {
    //   block [0x826DB8B0..0x826DB920)
	// 826DB8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB8BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB8C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB8C4: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB8C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB8CC: 390B5E58  addi r8, r11, 0x5e58
	ctx.r[8].s64 = ctx.r[11].s64 + 24152;
	// 826DB8D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DB8D4: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 826DB8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB8DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB8E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB8E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB8E8: 386AEC84  addi r3, r10, -0x137c
	ctx.r[3].s64 = ctx.r[10].s64 + -4988;
	// 826DB8EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB90C: 4BD8B515  bl 0x82466e20
	ctx.lr = 0x826DB910;
	sub_82466E20(ctx, base);
	// 826DB910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB920 size=112
    let mut pc: u32 = 0x826DB920;
    'dispatch: loop {
        match pc {
            0x826DB920 => {
    //   block [0x826DB920..0x826DB990)
	// 826DB920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB92C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB930: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB934: 38AAE834  addi r5, r10, -0x17cc
	ctx.r[5].s64 = ctx.r[10].s64 + -6092;
	// 826DB938: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB93C: 390B5EB8  addi r8, r11, 0x5eb8
	ctx.r[8].s64 = ctx.r[11].s64 + 24248;
	// 826DB940: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DB944: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 826DB948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB94C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB958: 386AECB4  addi r3, r10, -0x134c
	ctx.r[3].s64 = ctx.r[10].s64 + -4940;
	// 826DB95C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB97C: 4BD8B4A5  bl 0x82466e20
	ctx.lr = 0x826DB980;
	sub_82466E20(ctx, base);
	// 826DB980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DB990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DB990 size=112
    let mut pc: u32 = 0x826DB990;
    'dispatch: loop {
        match pc {
            0x826DB990 => {
    //   block [0x826DB990..0x826DBA00)
	// 826DB990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DB994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DB998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DB99C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB9A0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DB9A4: 38AAECB4  addi r5, r10, -0x134c
	ctx.r[5].s64 = ctx.r[10].s64 + -4940;
	// 826DB9A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DB9AC: 390B5F18  addi r8, r11, 0x5f18
	ctx.r[8].s64 = ctx.r[11].s64 + 24344;
	// 826DB9B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DB9B4: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 826DB9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DB9BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DB9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DB9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DB9C8: 386AECE4  addi r3, r10, -0x131c
	ctx.r[3].s64 = ctx.r[10].s64 + -4892;
	// 826DB9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DB9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DB9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DB9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DB9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DB9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DB9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DB9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DB9EC: 4BD8B435  bl 0x82466e20
	ctx.lr = 0x826DB9F0;
	sub_82466E20(ctx, base);
	// 826DB9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DB9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DB9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DB9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBA00 size=112
    let mut pc: u32 = 0x826DBA00;
    'dispatch: loop {
        match pc {
            0x826DBA00 => {
    //   block [0x826DBA00..0x826DBA70)
	// 826DBA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBA0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBA10: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBA14: 38AAECB4  addi r5, r10, -0x134c
	ctx.r[5].s64 = ctx.r[10].s64 + -4940;
	// 826DBA18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBA1C: 390B5F48  addi r8, r11, 0x5f48
	ctx.r[8].s64 = ctx.r[11].s64 + 24392;
	// 826DBA20: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DBA24: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 826DBA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBA2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBA30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBA38: 386AED14  addi r3, r10, -0x12ec
	ctx.r[3].s64 = ctx.r[10].s64 + -4844;
	// 826DBA3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBA40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBA44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBA48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBA50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBA54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBA58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBA5C: 4BD8B3C5  bl 0x82466e20
	ctx.lr = 0x826DBA60;
	sub_82466E20(ctx, base);
	// 826DBA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBA70 size=100
    let mut pc: u32 = 0x826DBA70;
    'dispatch: loop {
        match pc {
            0x826DBA70 => {
    //   block [0x826DBA70..0x826DBAD4)
	// 826DBA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBA7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBA80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBA84: 38AAECB4  addi r5, r10, -0x134c
	ctx.r[5].s64 = ctx.r[10].s64 + -4940;
	// 826DBA88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBA8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBA90: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 826DBA94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBA98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBA9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBAA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBAA4: 386AED44  addi r3, r10, -0x12bc
	ctx.r[3].s64 = ctx.r[10].s64 + -4796;
	// 826DBAA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBAAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBAB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DBAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBAB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DBABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBAC0: 4BD8B361  bl 0x82466e20
	ctx.lr = 0x826DBAC4;
	sub_82466E20(ctx, base);
	// 826DBAC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBAC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBAD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBAD8 size=112
    let mut pc: u32 = 0x826DBAD8;
    'dispatch: loop {
        match pc {
            0x826DBAD8 => {
    //   block [0x826DBAD8..0x826DBB48)
	// 826DBAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBAE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBAE8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBAEC: 38AAECB4  addi r5, r10, -0x134c
	ctx.r[5].s64 = ctx.r[10].s64 + -4940;
	// 826DBAF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBAF4: 390B5F90  addi r8, r11, 0x5f90
	ctx.r[8].s64 = ctx.r[11].s64 + 24464;
	// 826DBAF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DBAFC: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 826DBB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBB04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBB08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBB10: 386AED74  addi r3, r10, -0x128c
	ctx.r[3].s64 = ctx.r[10].s64 + -4748;
	// 826DBB14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBB34: 4BD8B2ED  bl 0x82466e20
	ctx.lr = 0x826DBB38;
	sub_82466E20(ctx, base);
	// 826DBB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBB48 size=108
    let mut pc: u32 = 0x826DBB48;
    'dispatch: loop {
        match pc {
            0x826DBB48 => {
    //   block [0x826DBB48..0x826DBBB4)
	// 826DBB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBB54: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBB58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBB5C: 38EB5FA8  addi r7, r11, 0x5fa8
	ctx.r[7].s64 = ctx.r[11].s64 + 24488;
	// 826DBB60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DBB64: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 826DBB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBB6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBB70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DBB74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBB78: 386AEDA4  addi r3, r10, -0x125c
	ctx.r[3].s64 = ctx.r[10].s64 + -4700;
	// 826DBB7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DBB80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBB9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DBBA0: 4BD8B281  bl 0x82466e20
	ctx.lr = 0x826DBBA4;
	sub_82466E20(ctx, base);
	// 826DBBA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBBB8 size=112
    let mut pc: u32 = 0x826DBBB8;
    'dispatch: loop {
        match pc {
            0x826DBBB8 => {
    //   block [0x826DBBB8..0x826DBC28)
	// 826DBBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBBC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBBC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBBCC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DBBD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBBD4: 390B5FF0  addi r8, r11, 0x5ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 24560;
	// 826DBBD8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DBBDC: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 826DBBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBBE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBBE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBBEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBBF0: 386AEDD4  addi r3, r10, -0x122c
	ctx.r[3].s64 = ctx.r[10].s64 + -4652;
	// 826DBBF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBBF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBBFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBC04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBC14: 4BD8B20D  bl 0x82466e20
	ctx.lr = 0x826DBC18;
	sub_82466E20(ctx, base);
	// 826DBC18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBC24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBC28 size=112
    let mut pc: u32 = 0x826DBC28;
    'dispatch: loop {
        match pc {
            0x826DBC28 => {
    //   block [0x826DBC28..0x826DBC98)
	// 826DBC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBC34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBC38: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBC3C: 38AAEDD4  addi r5, r10, -0x122c
	ctx.r[5].s64 = ctx.r[10].s64 + -4652;
	// 826DBC40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBC44: 390B6050  addi r8, r11, 0x6050
	ctx.r[8].s64 = ctx.r[11].s64 + 24656;
	// 826DBC48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DBC4C: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 826DBC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBC54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBC58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBC5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBC60: 386AEE04  addi r3, r10, -0x11fc
	ctx.r[3].s64 = ctx.r[10].s64 + -4604;
	// 826DBC64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBC68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBC70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBC74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBC78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBC80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBC84: 4BD8B19D  bl 0x82466e20
	ctx.lr = 0x826DBC88;
	sub_82466E20(ctx, base);
	// 826DBC88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBC98 size=112
    let mut pc: u32 = 0x826DBC98;
    'dispatch: loop {
        match pc {
            0x826DBC98 => {
    //   block [0x826DBC98..0x826DBD08)
	// 826DBC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBCA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBCA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBCA8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBCAC: 38AAEDD4  addi r5, r10, -0x122c
	ctx.r[5].s64 = ctx.r[10].s64 + -4652;
	// 826DBCB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBCB4: 390B6068  addi r8, r11, 0x6068
	ctx.r[8].s64 = ctx.r[11].s64 + 24680;
	// 826DBCB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DBCBC: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 826DBCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBCC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBCC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBCCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBCD0: 386AEE34  addi r3, r10, -0x11cc
	ctx.r[3].s64 = ctx.r[10].s64 + -4556;
	// 826DBCD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBCD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBCDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBCE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBCE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBCE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBCEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBCF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBCF4: 4BD8B12D  bl 0x82466e20
	ctx.lr = 0x826DBCF8;
	sub_82466E20(ctx, base);
	// 826DBCF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBD08 size=112
    let mut pc: u32 = 0x826DBD08;
    'dispatch: loop {
        match pc {
            0x826DBD08 => {
    //   block [0x826DBD08..0x826DBD78)
	// 826DBD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBD10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBD14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBD18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBD1C: 38AAEDD4  addi r5, r10, -0x122c
	ctx.r[5].s64 = ctx.r[10].s64 + -4652;
	// 826DBD20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBD24: 390B6098  addi r8, r11, 0x6098
	ctx.r[8].s64 = ctx.r[11].s64 + 24728;
	// 826DBD28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DBD2C: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 826DBD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBD34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBD38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBD40: 386AEE64  addi r3, r10, -0x119c
	ctx.r[3].s64 = ctx.r[10].s64 + -4508;
	// 826DBD44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DBD48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBD64: 4BD8B0BD  bl 0x82466e20
	ctx.lr = 0x826DBD68;
	sub_82466E20(ctx, base);
	// 826DBD68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DBD78 size=24
    let mut pc: u32 = 0x826DBD78;
    'dispatch: loop {
        match pc {
            0x826DBD78 => {
    //   block [0x826DBD78..0x826DBD90)
	// 826DBD78: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBD7C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DBD80: 394AB7C8  addi r10, r10, -0x4838
	ctx.r[10].s64 = ctx.r[10].s64 + -18488;
	// 826DBD84: 816B5BFC  lwz r11, 0x5bfc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23548 as u32) ) } as u64;
	// 826DBD88: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826DBD8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBD90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBD90 size=112
    let mut pc: u32 = 0x826DBD90;
    'dispatch: loop {
        match pc {
            0x826DBD90 => {
    //   block [0x826DBD90..0x826DBE00)
	// 826DBD90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBD94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBD98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBD9C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DBDA0: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DBDA4: 392A56F8  addi r9, r10, 0x56f8
	ctx.r[9].s64 = ctx.r[10].s64 + 22264;
	// 826DBDA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBDAC: 390BB7C8  addi r8, r11, -0x4838
	ctx.r[8].s64 = ctx.r[11].s64 + -18488;
	// 826DBDB0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826DBDB4: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 826DBDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBDBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBDC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBDC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBDC8: 386AEE94  addi r3, r10, -0x116c
	ctx.r[3].s64 = ctx.r[10].s64 + -4460;
	// 826DBDCC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DBDD0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DBDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBDD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBDDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBDE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBDE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DBDE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBDEC: 4BD8B035  bl 0x82466e20
	ctx.lr = 0x826DBDF0;
	sub_82466E20(ctx, base);
	// 826DBDF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBDF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBDF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBE00 size=108
    let mut pc: u32 = 0x826DBE00;
    'dispatch: loop {
        match pc {
            0x826DBE00 => {
    //   block [0x826DBE00..0x826DBE6C)
	// 826DBE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBE08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBE0C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBE10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBE14: 38EB60B0  addi r7, r11, 0x60b0
	ctx.r[7].s64 = ctx.r[11].s64 + 24752;
	// 826DBE18: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826DBE1C: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 826DBE20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBE24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBE28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DBE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBE30: 386AEEC4  addi r3, r10, -0x113c
	ctx.r[3].s64 = ctx.r[10].s64 + -4412;
	// 826DBE34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DBE38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBE40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBE44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBE48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBE50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBE54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DBE58: 4BD8AFC9  bl 0x82466e20
	ctx.lr = 0x826DBE5C;
	sub_82466E20(ctx, base);
	// 826DBE5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBE60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBE64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBE68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBE70 size=108
    let mut pc: u32 = 0x826DBE70;
    'dispatch: loop {
        match pc {
            0x826DBE70 => {
    //   block [0x826DBE70..0x826DBEDC)
	// 826DBE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBE74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBE7C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBE80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBE84: 38EB60C8  addi r7, r11, 0x60c8
	ctx.r[7].s64 = ctx.r[11].s64 + 24776;
	// 826DBE88: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DBE8C: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 826DBE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBE94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBE98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DBE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DBEA0: 386AEEF4  addi r3, r10, -0x110c
	ctx.r[3].s64 = ctx.r[10].s64 + -4364;
	// 826DBEA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DBEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DBEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBEB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBEC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DBEC8: 4BD8AF59  bl 0x82466e20
	ctx.lr = 0x826DBECC;
	sub_82466E20(ctx, base);
	// 826DBECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBEE0 size=116
    let mut pc: u32 = 0x826DBEE0;
    'dispatch: loop {
        match pc {
            0x826DBEE0 => {
    //   block [0x826DBEE0..0x826DBF54)
	// 826DBEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBEE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBEEC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBEF0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DBEF4: 390B6114  addi r8, r11, 0x6114
	ctx.r[8].s64 = ctx.r[11].s64 + 24852;
	// 826DBEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBEFC: 392A57B8  addi r9, r10, 0x57b8
	ctx.r[9].s64 = ctx.r[10].s64 + 22456;
	// 826DBF00: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBF04: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826DBF08: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DBF0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBF14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBF24: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DBF28: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 826DBF2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DBF30: 386BEF24  addi r3, r11, -0x10dc
	ctx.r[3].s64 = ctx.r[11].s64 + -4316;
	// 826DBF34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DBF38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBF3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBF40: 4BD8AEE1  bl 0x82466e20
	ctx.lr = 0x826DBF44;
	sub_82466E20(ctx, base);
	// 826DBF44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBF48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBF4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DBF58 size=24
    let mut pc: u32 = 0x826DBF58;
    'dispatch: loop {
        match pc {
            0x826DBF58 => {
    //   block [0x826DBF58..0x826DBF70)
	// 826DBF58: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBF5C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DBF60: 394AB810  addi r10, r10, -0x47f0
	ctx.r[10].s64 = ctx.r[10].s64 + -18416;
	// 826DBF64: 816B612C  lwz r11, 0x612c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24876 as u32) ) } as u64;
	// 826DBF68: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826DBF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBF70 size=116
    let mut pc: u32 = 0x826DBF70;
    'dispatch: loop {
        match pc {
            0x826DBF70 => {
    //   block [0x826DBF70..0x826DBFE4)
	// 826DBF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBF7C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DBF80: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DBF84: 390BB810  addi r8, r11, -0x47f0
	ctx.r[8].s64 = ctx.r[11].s64 + -18416;
	// 826DBF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DBF8C: 392A5828  addi r9, r10, 0x5828
	ctx.r[9].s64 = ctx.r[10].s64 + 22568;
	// 826DBF90: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DBF94: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826DBF98: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DBF9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DBFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DBFA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DBFAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DBFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DBFB4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DBFB8: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 826DBFBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DBFC0: 386BEF54  addi r3, r11, -0x10ac
	ctx.r[3].s64 = ctx.r[11].s64 + -4268;
	// 826DBFC4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826DBFC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DBFCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DBFD0: 4BD8AE51  bl 0x82466e20
	ctx.lr = 0x826DBFD4;
	sub_82466E20(ctx, base);
	// 826DBFD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DBFD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DBFDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DBFE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DBFE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DBFE8 size=108
    let mut pc: u32 = 0x826DBFE8;
    'dispatch: loop {
        match pc {
            0x826DBFE8 => {
    //   block [0x826DBFE8..0x826DC054)
	// 826DBFE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DBFEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DBFF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DBFF4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DBFF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DBFFC: 38EB613C  addi r7, r11, 0x613c
	ctx.r[7].s64 = ctx.r[11].s64 + 24892;
	// 826DC000: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DC004: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 826DC008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC00C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DC014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC018: 386AEF84  addi r3, r10, -0x107c
	ctx.r[3].s64 = ctx.r[10].s64 + -4220;
	// 826DC01C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DC020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC02C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC03C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC040: 4BD8ADE1  bl 0x82466e20
	ctx.lr = 0x826DC044;
	sub_82466E20(ctx, base);
	// 826DC044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC058 size=112
    let mut pc: u32 = 0x826DC058;
    'dispatch: loop {
        match pc {
            0x826DC058 => {
    //   block [0x826DC058..0x826DC0C8)
	// 826DC058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC064: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC068: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC06C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC070: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC074: 390B616C  addi r8, r11, 0x616c
	ctx.r[8].s64 = ctx.r[11].s64 + 24940;
	// 826DC078: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC07C: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 826DC080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC090: 386AEFB4  addi r3, r10, -0x104c
	ctx.r[3].s64 = ctx.r[10].s64 + -4172;
	// 826DC094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC0B4: 4BD8AD6D  bl 0x82466e20
	ctx.lr = 0x826DC0B8;
	sub_82466E20(ctx, base);
	// 826DC0B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC0C8 size=112
    let mut pc: u32 = 0x826DC0C8;
    'dispatch: loop {
        match pc {
            0x826DC0C8 => {
    //   block [0x826DC0C8..0x826DC138)
	// 826DC0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC0D4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC0D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC0DC: 392A5880  addi r9, r10, 0x5880
	ctx.r[9].s64 = ctx.r[10].s64 + 22656;
	// 826DC0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC0E4: 390B6188  addi r8, r11, 0x6188
	ctx.r[8].s64 = ctx.r[11].s64 + 24968;
	// 826DC0E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826DC0EC: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 826DC0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC0F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC100: 386AEFE4  addi r3, r10, -0x101c
	ctx.r[3].s64 = ctx.r[10].s64 + -4124;
	// 826DC104: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC108: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC11C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC124: 4BD8ACFD  bl 0x82466e20
	ctx.lr = 0x826DC128;
	sub_82466E20(ctx, base);
	// 826DC128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC138 size=112
    let mut pc: u32 = 0x826DC138;
    'dispatch: loop {
        match pc {
            0x826DC138 => {
    //   block [0x826DC138..0x826DC1A8)
	// 826DC138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC144: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC148: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC14C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC150: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC154: 390B61D0  addi r8, r11, 0x61d0
	ctx.r[8].s64 = ctx.r[11].s64 + 25040;
	// 826DC158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC15C: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 826DC160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC170: 386AF014  addi r3, r10, -0xfec
	ctx.r[3].s64 = ctx.r[10].s64 + -4076;
	// 826DC174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC194: 4BD8AC8D  bl 0x82466e20
	ctx.lr = 0x826DC198;
	sub_82466E20(ctx, base);
	// 826DC198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC1A8 size=112
    let mut pc: u32 = 0x826DC1A8;
    'dispatch: loop {
        match pc {
            0x826DC1A8 => {
    //   block [0x826DC1A8..0x826DC218)
	// 826DC1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC1B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC1B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC1BC: 392A58AC  addi r9, r10, 0x58ac
	ctx.r[9].s64 = ctx.r[10].s64 + 22700;
	// 826DC1C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC1C4: 390B61F0  addi r8, r11, 0x61f0
	ctx.r[8].s64 = ctx.r[11].s64 + 25072;
	// 826DC1C8: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826DC1CC: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 826DC1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC1D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC1E0: 386AF044  addi r3, r10, -0xfbc
	ctx.r[3].s64 = ctx.r[10].s64 + -4028;
	// 826DC1E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC1E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC1EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC1FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC204: 4BD8AC1D  bl 0x82466e20
	ctx.lr = 0x826DC208;
	sub_82466E20(ctx, base);
	// 826DC208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC218 size=112
    let mut pc: u32 = 0x826DC218;
    'dispatch: loop {
        match pc {
            0x826DC218 => {
    //   block [0x826DC218..0x826DC288)
	// 826DC218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC224: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC228: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC22C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC234: 390B6280  addi r8, r11, 0x6280
	ctx.r[8].s64 = ctx.r[11].s64 + 25216;
	// 826DC238: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC23C: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 826DC240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC244: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC250: 386AF074  addi r3, r10, -0xf8c
	ctx.r[3].s64 = ctx.r[10].s64 + -3980;
	// 826DC254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC274: 4BD8ABAD  bl 0x82466e20
	ctx.lr = 0x826DC278;
	sub_82466E20(ctx, base);
	// 826DC278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC288 size=112
    let mut pc: u32 = 0x826DC288;
    'dispatch: loop {
        match pc {
            0x826DC288 => {
    //   block [0x826DC288..0x826DC2F8)
	// 826DC288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC298: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC29C: 38AAF0D4  addi r5, r10, -0xf2c
	ctx.r[5].s64 = ctx.r[10].s64 + -3884;
	// 826DC2A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC2A4: 390B6298  addi r8, r11, 0x6298
	ctx.r[8].s64 = ctx.r[11].s64 + 25240;
	// 826DC2A8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DC2AC: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 826DC2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC2B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC2C0: 386AF0A4  addi r3, r10, -0xf5c
	ctx.r[3].s64 = ctx.r[10].s64 + -3932;
	// 826DC2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC2E4: 4BD8AB3D  bl 0x82466e20
	ctx.lr = 0x826DC2E8;
	sub_82466E20(ctx, base);
	// 826DC2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC2F8 size=100
    let mut pc: u32 = 0x826DC2F8;
    'dispatch: loop {
        match pc {
            0x826DC2F8 => {
    //   block [0x826DC2F8..0x826DC35C)
	// 826DC2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC304: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC30C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DC310: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC318: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 826DC31C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC32C: 386AF0D4  addi r3, r10, -0xf2c
	ctx.r[3].s64 = ctx.r[10].s64 + -3884;
	// 826DC330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC334: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC338: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DC33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC340: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DC344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC348: 4BD8AAD9  bl 0x82466e20
	ctx.lr = 0x826DC34C;
	sub_82466E20(ctx, base);
	// 826DC34C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DC360 size=24
    let mut pc: u32 = 0x826DC360;
    'dispatch: loop {
        match pc {
            0x826DC360 => {
    //   block [0x826DC360..0x826DC378)
	// 826DC360: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC364: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DC368: 394AB8E8  addi r10, r10, -0x4718
	ctx.r[10].s64 = ctx.r[10].s64 + -18200;
	// 826DC36C: 816B61EC  lwz r11, 0x61ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25068 as u32) ) } as u64;
	// 826DC370: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826DC374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC378 size=116
    let mut pc: u32 = 0x826DC378;
    'dispatch: loop {
        match pc {
            0x826DC378 => {
    //   block [0x826DC378..0x826DC3EC)
	// 826DC378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC37C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC384: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DC388: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC38C: 390BB8E8  addi r8, r11, -0x4718
	ctx.r[8].s64 = ctx.r[11].s64 + -18200;
	// 826DC390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC394: 392A58E8  addi r9, r10, 0x58e8
	ctx.r[9].s64 = ctx.r[10].s64 + 22760;
	// 826DC398: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC39C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826DC3A0: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC3A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC3AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC3BC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DC3C0: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 826DC3C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC3C8: 386BF104  addi r3, r11, -0xefc
	ctx.r[3].s64 = ctx.r[11].s64 + -3836;
	// 826DC3CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC3D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC3D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC3D8: 4BD8AA49  bl 0x82466e20
	ctx.lr = 0x826DC3DC;
	sub_82466E20(ctx, base);
	// 826DC3DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC3E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC3E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC3F0 size=108
    let mut pc: u32 = 0x826DC3F0;
    'dispatch: loop {
        match pc {
            0x826DC3F0 => {
    //   block [0x826DC3F0..0x826DC45C)
	// 826DC3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC3FC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC404: 38EB6310  addi r7, r11, 0x6310
	ctx.r[7].s64 = ctx.r[11].s64 + 25360;
	// 826DC408: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DC40C: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 826DC410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DC41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC420: 386AF134  addi r3, r10, -0xecc
	ctx.r[3].s64 = ctx.r[10].s64 + -3788;
	// 826DC424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DC428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC448: 4BD8A9D9  bl 0x82466e20
	ctx.lr = 0x826DC44C;
	sub_82466E20(ctx, base);
	// 826DC44C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC460 size=112
    let mut pc: u32 = 0x826DC460;
    'dispatch: loop {
        match pc {
            0x826DC460 => {
    //   block [0x826DC460..0x826DC4D0)
	// 826DC460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC46C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC470: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC474: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC478: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC47C: 390B6340  addi r8, r11, 0x6340
	ctx.r[8].s64 = ctx.r[11].s64 + 25408;
	// 826DC480: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC484: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 826DC488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC48C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC498: 386AF164  addi r3, r10, -0xe9c
	ctx.r[3].s64 = ctx.r[10].s64 + -3740;
	// 826DC49C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC4A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC4A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC4B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC4B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC4B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC4BC: 4BD8A965  bl 0x82466e20
	ctx.lr = 0x826DC4C0;
	sub_82466E20(ctx, base);
	// 826DC4C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC4C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC4C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC4D0 size=112
    let mut pc: u32 = 0x826DC4D0;
    'dispatch: loop {
        match pc {
            0x826DC4D0 => {
    //   block [0x826DC4D0..0x826DC540)
	// 826DC4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC4DC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC4E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC4E4: 392A590C  addi r9, r10, 0x590c
	ctx.r[9].s64 = ctx.r[10].s64 + 22796;
	// 826DC4E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC4EC: 390B6360  addi r8, r11, 0x6360
	ctx.r[8].s64 = ctx.r[11].s64 + 25440;
	// 826DC4F0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826DC4F4: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 826DC4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC4FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC508: 386AF194  addi r3, r10, -0xe6c
	ctx.r[3].s64 = ctx.r[10].s64 + -3692;
	// 826DC50C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC510: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC52C: 4BD8A8F5  bl 0x82466e20
	ctx.lr = 0x826DC530;
	sub_82466E20(ctx, base);
	// 826DC530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC540 size=112
    let mut pc: u32 = 0x826DC540;
    'dispatch: loop {
        match pc {
            0x826DC540 => {
    //   block [0x826DC540..0x826DC5B0)
	// 826DC540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC54C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC550: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC554: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC558: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC55C: 390B6408  addi r8, r11, 0x6408
	ctx.r[8].s64 = ctx.r[11].s64 + 25608;
	// 826DC560: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC564: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 826DC568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC56C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC578: 386AF1C4  addi r3, r10, -0xe3c
	ctx.r[3].s64 = ctx.r[10].s64 + -3644;
	// 826DC57C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC59C: 4BD8A885  bl 0x82466e20
	ctx.lr = 0x826DC5A0;
	sub_82466E20(ctx, base);
	// 826DC5A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC5B0 size=112
    let mut pc: u32 = 0x826DC5B0;
    'dispatch: loop {
        match pc {
            0x826DC5B0 => {
    //   block [0x826DC5B0..0x826DC620)
	// 826DC5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC5BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC5C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC5C4: 392A5964  addi r9, r10, 0x5964
	ctx.r[9].s64 = ctx.r[10].s64 + 22884;
	// 826DC5C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC5CC: 390B6428  addi r8, r11, 0x6428
	ctx.r[8].s64 = ctx.r[11].s64 + 25640;
	// 826DC5D0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826DC5D4: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 826DC5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC5DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC5E8: 386AF1F4  addi r3, r10, -0xe0c
	ctx.r[3].s64 = ctx.r[10].s64 + -3596;
	// 826DC5EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC5F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC60C: 4BD8A815  bl 0x82466e20
	ctx.lr = 0x826DC610;
	sub_82466E20(ctx, base);
	// 826DC610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC620 size=116
    let mut pc: u32 = 0x826DC620;
    'dispatch: loop {
        match pc {
            0x826DC620 => {
    //   block [0x826DC620..0x826DC694)
	// 826DC620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC62C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC630: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC634: 390B64D0  addi r8, r11, 0x64d0
	ctx.r[8].s64 = ctx.r[11].s64 + 25808;
	// 826DC638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC63C: 392A5938  addi r9, r10, 0x5938
	ctx.r[9].s64 = ctx.r[10].s64 + 22840;
	// 826DC640: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC644: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826DC648: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC64C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC664: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DC668: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 826DC66C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC670: 386BF224  addi r3, r11, -0xddc
	ctx.r[3].s64 = ctx.r[11].s64 + -3548;
	// 826DC674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC680: 4BD8A7A1  bl 0x82466e20
	ctx.lr = 0x826DC684;
	sub_82466E20(ctx, base);
	// 826DC684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC68C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC698 size=108
    let mut pc: u32 = 0x826DC698;
    'dispatch: loop {
        match pc {
            0x826DC698 => {
    //   block [0x826DC698..0x826DC704)
	// 826DC698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC6A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC6A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC6AC: 38EB64E8  addi r7, r11, 0x64e8
	ctx.r[7].s64 = ctx.r[11].s64 + 25832;
	// 826DC6B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DC6B4: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 826DC6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC6BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC6C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DC6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC6C8: 386AF254  addi r3, r10, -0xdac
	ctx.r[3].s64 = ctx.r[10].s64 + -3500;
	// 826DC6CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DC6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC6D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC6E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC6E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC6EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC6F0: 4BD8A731  bl 0x82466e20
	ctx.lr = 0x826DC6F4;
	sub_82466E20(ctx, base);
	// 826DC6F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC708 size=112
    let mut pc: u32 = 0x826DC708;
    'dispatch: loop {
        match pc {
            0x826DC708 => {
    //   block [0x826DC708..0x826DC778)
	// 826DC708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC718: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC71C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC720: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC724: 390B6518  addi r8, r11, 0x6518
	ctx.r[8].s64 = ctx.r[11].s64 + 25880;
	// 826DC728: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC72C: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 826DC730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC734: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC740: 386AF284  addi r3, r10, -0xd7c
	ctx.r[3].s64 = ctx.r[10].s64 + -3452;
	// 826DC744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC764: 4BD8A6BD  bl 0x82466e20
	ctx.lr = 0x826DC768;
	sub_82466E20(ctx, base);
	// 826DC768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC778 size=112
    let mut pc: u32 = 0x826DC778;
    'dispatch: loop {
        match pc {
            0x826DC778 => {
    //   block [0x826DC778..0x826DC7E8)
	// 826DC778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC784: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DC788: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC78C: 392A5998  addi r9, r10, 0x5998
	ctx.r[9].s64 = ctx.r[10].s64 + 22936;
	// 826DC790: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC794: 390B6530  addi r8, r11, 0x6530
	ctx.r[8].s64 = ctx.r[11].s64 + 25904;
	// 826DC798: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826DC79C: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 826DC7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC7A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC7B0: 386AF2B4  addi r3, r10, -0xd4c
	ctx.r[3].s64 = ctx.r[10].s64 + -3404;
	// 826DC7B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DC7B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DC7BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC7CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC7D4: 4BD8A64D  bl 0x82466e20
	ctx.lr = 0x826DC7D8;
	sub_82466E20(ctx, base);
	// 826DC7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC7E8 size=112
    let mut pc: u32 = 0x826DC7E8;
    'dispatch: loop {
        match pc {
            0x826DC7E8 => {
    //   block [0x826DC7E8..0x826DC858)
	// 826DC7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC7F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC7F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC7FC: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC800: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC804: 390B65D8  addi r8, r11, 0x65d8
	ctx.r[8].s64 = ctx.r[11].s64 + 26072;
	// 826DC808: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DC80C: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 826DC810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC820: 386AF2E4  addi r3, r10, -0xd1c
	ctx.r[3].s64 = ctx.r[10].s64 + -3356;
	// 826DC824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC82C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC844: 4BD8A5DD  bl 0x82466e20
	ctx.lr = 0x826DC848;
	sub_82466E20(ctx, base);
	// 826DC848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC858 size=108
    let mut pc: u32 = 0x826DC858;
    'dispatch: loop {
        match pc {
            0x826DC858 => {
    //   block [0x826DC858..0x826DC8C4)
	// 826DC858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC864: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC868: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC86C: 38EB6620  addi r7, r11, 0x6620
	ctx.r[7].s64 = ctx.r[11].s64 + 26144;
	// 826DC870: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DC874: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 826DC878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC87C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DC884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC888: 386AF314  addi r3, r10, -0xcec
	ctx.r[3].s64 = ctx.r[10].s64 + -3308;
	// 826DC88C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DC890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DC8B0: 4BD8A571  bl 0x82466e20
	ctx.lr = 0x826DC8B4;
	sub_82466E20(ctx, base);
	// 826DC8B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC8B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC8BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC8C8 size=112
    let mut pc: u32 = 0x826DC8C8;
    'dispatch: loop {
        match pc {
            0x826DC8C8 => {
    //   block [0x826DC8C8..0x826DC938)
	// 826DC8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC8D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC8D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC8DC: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC8E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC8E4: 390B6650  addi r8, r11, 0x6650
	ctx.r[8].s64 = ctx.r[11].s64 + 26192;
	// 826DC8E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DC8EC: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 826DC8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC8F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC8F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC8FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC900: 386AF344  addi r3, r10, -0xcbc
	ctx.r[3].s64 = ctx.r[10].s64 + -3260;
	// 826DC904: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC90C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC924: 4BD8A4FD  bl 0x82466e20
	ctx.lr = 0x826DC928;
	sub_82466E20(ctx, base);
	// 826DC928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC92C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC938 size=112
    let mut pc: u32 = 0x826DC938;
    'dispatch: loop {
        match pc {
            0x826DC938 => {
    //   block [0x826DC938..0x826DC9A8)
	// 826DC938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC944: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC948: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DC94C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC950: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC954: 390B6668  addi r8, r11, 0x6668
	ctx.r[8].s64 = ctx.r[11].s64 + 26216;
	// 826DC958: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826DC95C: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 826DC960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC964: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DC96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC970: 386AF374  addi r3, r10, -0xc8c
	ctx.r[3].s64 = ctx.r[10].s64 + -3212;
	// 826DC974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DC978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC994: 4BD8A48D  bl 0x82466e20
	ctx.lr = 0x826DC998;
	sub_82466E20(ctx, base);
	// 826DC998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DC99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DC9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DC9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DC9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DC9A8 size=100
    let mut pc: u32 = 0x826DC9A8;
    'dispatch: loop {
        match pc {
            0x826DC9A8 => {
    //   block [0x826DC9A8..0x826DCA0C)
	// 826DC9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DC9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DC9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DC9B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DC9BC: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DC9C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DC9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DC9C8: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 826DC9CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DC9D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DC9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DC9D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DC9DC: 386AF3A4  addi r3, r10, -0xc5c
	ctx.r[3].s64 = ctx.r[10].s64 + -3164;
	// 826DC9E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DC9E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DC9E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DC9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DC9F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DC9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DC9F8: 4BD8A429  bl 0x82466e20
	ctx.lr = 0x826DC9FC;
	sub_82466E20(ctx, base);
	// 826DC9FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCA10 size=112
    let mut pc: u32 = 0x826DCA10;
    'dispatch: loop {
        match pc {
            0x826DCA10 => {
    //   block [0x826DCA10..0x826DCA80)
	// 826DCA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCA1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCA20: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCA24: 38AAEF54  addi r5, r10, -0x10ac
	ctx.r[5].s64 = ctx.r[10].s64 + -4268;
	// 826DCA28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCA2C: 390B6728  addi r8, r11, 0x6728
	ctx.r[8].s64 = ctx.r[11].s64 + 26408;
	// 826DCA30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DCA34: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 826DCA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCA3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCA48: 386AF3D4  addi r3, r10, -0xc2c
	ctx.r[3].s64 = ctx.r[10].s64 + -3116;
	// 826DCA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCA6C: 4BD8A3B5  bl 0x82466e20
	ctx.lr = 0x826DCA70;
	sub_82466E20(ctx, base);
	// 826DCA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCA80 size=112
    let mut pc: u32 = 0x826DCA80;
    'dispatch: loop {
        match pc {
            0x826DCA80 => {
    //   block [0x826DCA80..0x826DCAF0)
	// 826DCA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCA8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCA90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCA94: 38AAEDD4  addi r5, r10, -0x122c
	ctx.r[5].s64 = ctx.r[10].s64 + -4652;
	// 826DCA98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCA9C: 390B6758  addi r8, r11, 0x6758
	ctx.r[8].s64 = ctx.r[11].s64 + 26456;
	// 826DCAA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DCAA4: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 826DCAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCAAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCAB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCAB8: 386AF404  addi r3, r10, -0xbfc
	ctx.r[3].s64 = ctx.r[10].s64 + -3068;
	// 826DCABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCADC: 4BD8A345  bl 0x82466e20
	ctx.lr = 0x826DCAE0;
	sub_82466E20(ctx, base);
	// 826DCAE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCAF0 size=108
    let mut pc: u32 = 0x826DCAF0;
    'dispatch: loop {
        match pc {
            0x826DCAF0 => {
    //   block [0x826DCAF0..0x826DCB5C)
	// 826DCAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCAFC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCB00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCB04: 38EB6770  addi r7, r11, 0x6770
	ctx.r[7].s64 = ctx.r[11].s64 + 26480;
	// 826DCB08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DCB0C: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 826DCB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCB14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCB18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DCB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCB20: 386AF434  addi r3, r10, -0xbcc
	ctx.r[3].s64 = ctx.r[10].s64 + -3020;
	// 826DCB24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DCB28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCB30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCB38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCB40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCB44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DCB48: 4BD8A2D9  bl 0x82466e20
	ctx.lr = 0x826DCB4C;
	sub_82466E20(ctx, base);
	// 826DCB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCB60 size=112
    let mut pc: u32 = 0x826DCB60;
    'dispatch: loop {
        match pc {
            0x826DCB60 => {
    //   block [0x826DCB60..0x826DCBD0)
	// 826DCB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCB6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCB70: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCB74: 38AAF3A4  addi r5, r10, -0xc5c
	ctx.r[5].s64 = ctx.r[10].s64 + -3164;
	// 826DCB78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCB7C: 390B67A0  addi r8, r11, 0x67a0
	ctx.r[8].s64 = ctx.r[11].s64 + 26528;
	// 826DCB80: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826DCB84: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 826DCB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCB8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCB98: 386AF464  addi r3, r10, -0xb9c
	ctx.r[3].s64 = ctx.r[10].s64 + -2972;
	// 826DCB9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCBBC: 4BD8A265  bl 0x82466e20
	ctx.lr = 0x826DCBC0;
	sub_82466E20(ctx, base);
	// 826DCBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCBD0 size=112
    let mut pc: u32 = 0x826DCBD0;
    'dispatch: loop {
        match pc {
            0x826DCBD0 => {
    //   block [0x826DCBD0..0x826DCC40)
	// 826DCBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCBDC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DCBE0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCBE4: 392A59C4  addi r9, r10, 0x59c4
	ctx.r[9].s64 = ctx.r[10].s64 + 22980;
	// 826DCBE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCBEC: 390B6838  addi r8, r11, 0x6838
	ctx.r[8].s64 = ctx.r[11].s64 + 26680;
	// 826DCBF0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826DCBF4: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 826DCBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCBFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCC00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCC08: 386AF494  addi r3, r10, -0xb6c
	ctx.r[3].s64 = ctx.r[10].s64 + -2924;
	// 826DCC0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DCC10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DCC14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCC24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DCC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCC2C: 4BD8A1F5  bl 0x82466e20
	ctx.lr = 0x826DCC30;
	sub_82466E20(ctx, base);
	// 826DCC30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCC40 size=112
    let mut pc: u32 = 0x826DCC40;
    'dispatch: loop {
        match pc {
            0x826DCC40 => {
    //   block [0x826DCC40..0x826DCCB0)
	// 826DCC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCC4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCC50: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCC54: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DCC58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCC5C: 390B6880  addi r8, r11, 0x6880
	ctx.r[8].s64 = ctx.r[11].s64 + 26752;
	// 826DCC60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DCC64: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 826DCC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCC6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCC70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCC78: 386AF4C4  addi r3, r10, -0xb3c
	ctx.r[3].s64 = ctx.r[10].s64 + -2876;
	// 826DCC7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCC9C: 4BD8A185  bl 0x82466e20
	ctx.lr = 0x826DCCA0;
	sub_82466E20(ctx, base);
	// 826DCCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCCB0 size=108
    let mut pc: u32 = 0x826DCCB0;
    'dispatch: loop {
        match pc {
            0x826DCCB0 => {
    //   block [0x826DCCB0..0x826DCD1C)
	// 826DCCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCCBC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCCC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCCC4: 38EB6898  addi r7, r11, 0x6898
	ctx.r[7].s64 = ctx.r[11].s64 + 26776;
	// 826DCCC8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826DCCCC: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 826DCCD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCCD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCCD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DCCDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCCE0: 386AF4F4  addi r3, r10, -0xb0c
	ctx.r[3].s64 = ctx.r[10].s64 + -2828;
	// 826DCCE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DCCE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCCEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCCF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCCF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCCF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCCFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCD00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCD04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DCD08: 4BD8A119  bl 0x82466e20
	ctx.lr = 0x826DCD0C;
	sub_82466E20(ctx, base);
	// 826DCD0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCD20 size=116
    let mut pc: u32 = 0x826DCD20;
    'dispatch: loop {
        match pc {
            0x826DCD20 => {
    //   block [0x826DCD20..0x826DCD94)
	// 826DCD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCD28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCD2C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826DCD30: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826DCD34: 390A6928  addi r8, r10, 0x6928
	ctx.r[8].s64 = ctx.r[10].s64 + 26920;
	// 826DCD38: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCD3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DCD40: 38AAF3A4  addi r5, r10, -0xc5c
	ctx.r[5].s64 = ctx.r[10].s64 + -3164;
	// 826DCD44: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCD48: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DCD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCD50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCD54: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 826DCD58: 396B59D8  addi r11, r11, 0x59d8
	ctx.r[11].s64 = ctx.r[11].s64 + 23000;
	// 826DCD5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCD60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCD64: 386AF524  addi r3, r10, -0xadc
	ctx.r[3].s64 = ctx.r[10].s64 + -2780;
	// 826DCD68: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826DCD6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCD70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826DCD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCD78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCD80: 4BD8A0A1  bl 0x82466e20
	ctx.lr = 0x826DCD84;
	sub_82466E20(ctx, base);
	// 826DCD84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCD88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCD8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCD90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCD98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCD98 size=112
    let mut pc: u32 = 0x826DCD98;
    'dispatch: loop {
        match pc {
            0x826DCD98 => {
    //   block [0x826DCD98..0x826DCE08)
	// 826DCD98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCD9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCDA4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DCDA8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCDAC: 392A5A24  addi r9, r10, 0x5a24
	ctx.r[9].s64 = ctx.r[10].s64 + 23076;
	// 826DCDB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCDB4: 390B6A00  addi r8, r11, 0x6a00
	ctx.r[8].s64 = ctx.r[11].s64 + 27136;
	// 826DCDB8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826DCDBC: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 826DCDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCDC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCDC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCDD0: 386AF554  addi r3, r10, -0xaac
	ctx.r[3].s64 = ctx.r[10].s64 + -2732;
	// 826DCDD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DCDD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DCDDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCDEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DCDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCDF4: 4BD8A02D  bl 0x82466e20
	ctx.lr = 0x826DCDF8;
	sub_82466E20(ctx, base);
	// 826DCDF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCE08 size=112
    let mut pc: u32 = 0x826DCE08;
    'dispatch: loop {
        match pc {
            0x826DCE08 => {
    //   block [0x826DCE08..0x826DCE78)
	// 826DCE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCE14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCE18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCE1C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DCE20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCE24: 390B6A60  addi r8, r11, 0x6a60
	ctx.r[8].s64 = ctx.r[11].s64 + 27232;
	// 826DCE28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DCE2C: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 826DCE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCE34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCE38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCE3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCE40: 386AF584  addi r3, r10, -0xa7c
	ctx.r[3].s64 = ctx.r[10].s64 + -2684;
	// 826DCE44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCE48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCE4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCE64: 4BD89FBD  bl 0x82466e20
	ctx.lr = 0x826DCE68;
	sub_82466E20(ctx, base);
	// 826DCE68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCE6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCE70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCE78 size=108
    let mut pc: u32 = 0x826DCE78;
    'dispatch: loop {
        match pc {
            0x826DCE78 => {
    //   block [0x826DCE78..0x826DCEE4)
	// 826DCE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCE84: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCE88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCE8C: 38EB6A78  addi r7, r11, 0x6a78
	ctx.r[7].s64 = ctx.r[11].s64 + 27256;
	// 826DCE90: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DCE94: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 826DCE98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCE9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCEA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DCEA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCEA8: 386AF5B4  addi r3, r10, -0xa4c
	ctx.r[3].s64 = ctx.r[10].s64 + -2636;
	// 826DCEAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DCEB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCEB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCEC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCEC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCEC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DCED0: 4BD89F51  bl 0x82466e20
	ctx.lr = 0x826DCED4;
	sub_82466E20(ctx, base);
	// 826DCED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCEDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCEE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCEE8 size=112
    let mut pc: u32 = 0x826DCEE8;
    'dispatch: loop {
        match pc {
            0x826DCEE8 => {
    //   block [0x826DCEE8..0x826DCF58)
	// 826DCEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCEF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCEF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCEFC: 38AAF3A4  addi r5, r10, -0xc5c
	ctx.r[5].s64 = ctx.r[10].s64 + -3164;
	// 826DCF00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCF04: 390B6AC0  addi r8, r11, 0x6ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 27328;
	// 826DCF08: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826DCF0C: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 826DCF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCF14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCF18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCF20: 386AF5E4  addi r3, r10, -0xa1c
	ctx.r[3].s64 = ctx.r[10].s64 + -2588;
	// 826DCF24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCF28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCF2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCF3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCF44: 4BD89EDD  bl 0x82466e20
	ctx.lr = 0x826DCF48;
	sub_82466E20(ctx, base);
	// 826DCF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCF58 size=112
    let mut pc: u32 = 0x826DCF58;
    'dispatch: loop {
        match pc {
            0x826DCF58 => {
    //   block [0x826DCF58..0x826DCFC8)
	// 826DCF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCF64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCF68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCF6C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DCF70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCF74: 390B6B38  addi r8, r11, 0x6b38
	ctx.r[8].s64 = ctx.r[11].s64 + 27448;
	// 826DCF78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DCF7C: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 826DCF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCF84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCF88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DCF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DCF90: 386AF614  addi r3, r10, -0x9ec
	ctx.r[3].s64 = ctx.r[10].s64 + -2540;
	// 826DCF94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DCF98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DCF9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DCFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DCFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DCFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DCFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DCFB4: 4BD89E6D  bl 0x82466e20
	ctx.lr = 0x826DCFB8;
	sub_82466E20(ctx, base);
	// 826DCFB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DCFBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DCFC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DCFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DCFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DCFC8 size=108
    let mut pc: u32 = 0x826DCFC8;
    'dispatch: loop {
        match pc {
            0x826DCFC8 => {
    //   block [0x826DCFC8..0x826DD034)
	// 826DCFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DCFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DCFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DCFD4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DCFD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DCFDC: 38EB6B68  addi r7, r11, 0x6b68
	ctx.r[7].s64 = ctx.r[11].s64 + 27496;
	// 826DCFE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826DCFE4: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 826DCFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DCFEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DCFF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DCFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DCFF8: 386AF644  addi r3, r10, -0x9bc
	ctx.r[3].s64 = ctx.r[10].s64 + -2492;
	// 826DCFFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD01C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD020: 4BD89E01  bl 0x82466e20
	ctx.lr = 0x826DD024;
	sub_82466E20(ctx, base);
	// 826DD024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD038 size=108
    let mut pc: u32 = 0x826DD038;
    'dispatch: loop {
        match pc {
            0x826DD038 => {
    //   block [0x826DD038..0x826DD0A4)
	// 826DD038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD044: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD048: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD04C: 38EB6BC8  addi r7, r11, 0x6bc8
	ctx.r[7].s64 = ctx.r[11].s64 + 27592;
	// 826DD050: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826DD054: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 826DD058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD05C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD060: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD068: 386AF674  addi r3, r10, -0x98c
	ctx.r[3].s64 = ctx.r[10].s64 + -2444;
	// 826DD06C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD090: 4BD89D91  bl 0x82466e20
	ctx.lr = 0x826DD094;
	sub_82466E20(ctx, base);
	// 826DD094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD09C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD0A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD0A8 size=112
    let mut pc: u32 = 0x826DD0A8;
    'dispatch: loop {
        match pc {
            0x826DD0A8 => {
    //   block [0x826DD0A8..0x826DD118)
	// 826DD0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD0B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD0B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD0BC: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DD0C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD0C4: 390B6C40  addi r8, r11, 0x6c40
	ctx.r[8].s64 = ctx.r[11].s64 + 27712;
	// 826DD0C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DD0CC: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 826DD0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD0D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD0D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD0E0: 386AF6A4  addi r3, r10, -0x95c
	ctx.r[3].s64 = ctx.r[10].s64 + -2396;
	// 826DD0E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD0EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD104: 4BD89D1D  bl 0x82466e20
	ctx.lr = 0x826DD108;
	sub_82466E20(ctx, base);
	// 826DD108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DD118 size=24
    let mut pc: u32 = 0x826DD118;
    'dispatch: loop {
        match pc {
            0x826DD118 => {
    //   block [0x826DD118..0x826DD130)
	// 826DD118: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD11C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DD120: 394AB960  addi r10, r10, -0x46a0
	ctx.r[10].s64 = ctx.r[10].s64 + -18080;
	// 826DD124: 816B6C88  lwz r11, 0x6c88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27784 as u32) ) } as u64;
	// 826DD128: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826DD12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD130 size=116
    let mut pc: u32 = 0x826DD130;
    'dispatch: loop {
        match pc {
            0x826DD130 => {
    //   block [0x826DD130..0x826DD1A4)
	// 826DD130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD13C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DD140: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DD144: 390BB960  addi r8, r11, -0x46a0
	ctx.r[8].s64 = ctx.r[11].s64 + -18080;
	// 826DD148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD14C: 392A5A68  addi r9, r10, 0x5a68
	ctx.r[9].s64 = ctx.r[10].s64 + 23144;
	// 826DD150: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD154: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826DD158: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DD15C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD164: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD174: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DD178: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 826DD17C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DD180: 386BF6D4  addi r3, r11, -0x92c
	ctx.r[3].s64 = ctx.r[11].s64 + -2348;
	// 826DD184: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DD188: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD190: 4BD89C91  bl 0x82466e20
	ctx.lr = 0x826DD194;
	sub_82466E20(ctx, base);
	// 826DD194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD1A8 size=112
    let mut pc: u32 = 0x826DD1A8;
    'dispatch: loop {
        match pc {
            0x826DD1A8 => {
    //   block [0x826DD1A8..0x826DD218)
	// 826DD1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD1B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD1B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD1BC: 38AAF6D4  addi r5, r10, -0x92c
	ctx.r[5].s64 = ctx.r[10].s64 + -2348;
	// 826DD1C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD1C4: 390B6C8C  addi r8, r11, 0x6c8c
	ctx.r[8].s64 = ctx.r[11].s64 + 27788;
	// 826DD1C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DD1CC: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 826DD1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD1D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD1E0: 386AF704  addi r3, r10, -0x8fc
	ctx.r[3].s64 = ctx.r[10].s64 + -2300;
	// 826DD1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD204: 4BD89C1D  bl 0x82466e20
	ctx.lr = 0x826DD208;
	sub_82466E20(ctx, base);
	// 826DD208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DD218 size=24
    let mut pc: u32 = 0x826DD218;
    'dispatch: loop {
        match pc {
            0x826DD218 => {
    //   block [0x826DD218..0x826DD230)
	// 826DD218: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD21C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DD220: 394AB978  addi r10, r10, -0x4688
	ctx.r[10].s64 = ctx.r[10].s64 + -18056;
	// 826DD224: 816B6CBC  lwz r11, 0x6cbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27836 as u32) ) } as u64;
	// 826DD228: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826DD22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD230 size=116
    let mut pc: u32 = 0x826DD230;
    'dispatch: loop {
        match pc {
            0x826DD230 => {
    //   block [0x826DD230..0x826DD2A4)
	// 826DD230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD23C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DD240: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DD244: 390BB978  addi r8, r11, -0x4688
	ctx.r[8].s64 = ctx.r[11].s64 + -18056;
	// 826DD248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD24C: 392A5AA4  addi r9, r10, 0x5aa4
	ctx.r[9].s64 = ctx.r[10].s64 + 23204;
	// 826DD250: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD254: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826DD258: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 826DD25C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD264: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD274: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DD278: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 826DD27C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DD280: 386BF734  addi r3, r11, -0x8cc
	ctx.r[3].s64 = ctx.r[11].s64 + -2252;
	// 826DD284: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DD288: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD28C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD290: 4BD89B91  bl 0x82466e20
	ctx.lr = 0x826DD294;
	sub_82466E20(ctx, base);
	// 826DD294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD29C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD2A8 size=112
    let mut pc: u32 = 0x826DD2A8;
    'dispatch: loop {
        match pc {
            0x826DD2A8 => {
    //   block [0x826DD2A8..0x826DD318)
	// 826DD2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD2B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD2B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD2BC: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 826DD2C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD2C4: 390B6CC0  addi r8, r11, 0x6cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 27840;
	// 826DD2C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826DD2CC: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 826DD2D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD2D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD2D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD2E0: 386AF764  addi r3, r10, -0x89c
	ctx.r[3].s64 = ctx.r[10].s64 + -2204;
	// 826DD2E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD2E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD2F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD2F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD2FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD304: 4BD89B1D  bl 0x82466e20
	ctx.lr = 0x826DD308;
	sub_82466E20(ctx, base);
	// 826DD308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD318 size=112
    let mut pc: u32 = 0x826DD318;
    'dispatch: loop {
        match pc {
            0x826DD318 => {
    //   block [0x826DD318..0x826DD388)
	// 826DD318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD324: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD328: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD32C: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 826DD330: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD334: 390B6D20  addi r8, r11, 0x6d20
	ctx.r[8].s64 = ctx.r[11].s64 + 27936;
	// 826DD338: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826DD33C: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 826DD340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD344: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD350: 386AF794  addi r3, r10, -0x86c
	ctx.r[3].s64 = ctx.r[10].s64 + -2156;
	// 826DD354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD35C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD374: 4BD89AAD  bl 0x82466e20
	ctx.lr = 0x826DD378;
	sub_82466E20(ctx, base);
	// 826DD378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD37C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD388 size=112
    let mut pc: u32 = 0x826DD388;
    'dispatch: loop {
        match pc {
            0x826DD388 => {
    //   block [0x826DD388..0x826DD3F8)
	// 826DD388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD394: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD398: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD39C: 38AAF704  addi r5, r10, -0x8fc
	ctx.r[5].s64 = ctx.r[10].s64 + -2300;
	// 826DD3A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD3A4: 390B6D50  addi r8, r11, 0x6d50
	ctx.r[8].s64 = ctx.r[11].s64 + 27984;
	// 826DD3A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826DD3AC: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 826DD3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD3B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD3B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD3C0: 386AF7C4  addi r3, r10, -0x83c
	ctx.r[3].s64 = ctx.r[10].s64 + -2108;
	// 826DD3C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD3D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD3E4: 4BD89A3D  bl 0x82466e20
	ctx.lr = 0x826DD3E8;
	sub_82466E20(ctx, base);
	// 826DD3E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD3F8 size=108
    let mut pc: u32 = 0x826DD3F8;
    'dispatch: loop {
        match pc {
            0x826DD3F8 => {
    //   block [0x826DD3F8..0x826DD464)
	// 826DD3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD404: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD40C: 38EB6D98  addi r7, r11, 0x6d98
	ctx.r[7].s64 = ctx.r[11].s64 + 28056;
	// 826DD410: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DD414: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 826DD418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD41C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD428: 386AF7F4  addi r3, r10, -0x80c
	ctx.r[3].s64 = ctx.r[10].s64 + -2060;
	// 826DD42C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD44C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD450: 4BD899D1  bl 0x82466e20
	ctx.lr = 0x826DD454;
	sub_82466E20(ctx, base);
	// 826DD454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD45C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD468 size=112
    let mut pc: u32 = 0x826DD468;
    'dispatch: loop {
        match pc {
            0x826DD468 => {
    //   block [0x826DD468..0x826DD4D8)
	// 826DD468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD474: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD478: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD47C: 38AAEF24  addi r5, r10, -0x10dc
	ctx.r[5].s64 = ctx.r[10].s64 + -4316;
	// 826DD480: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD484: 390B6DC8  addi r8, r11, 0x6dc8
	ctx.r[8].s64 = ctx.r[11].s64 + 28104;
	// 826DD488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826DD48C: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 826DD490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD494: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD4A0: 386AF824  addi r3, r10, -0x7dc
	ctx.r[3].s64 = ctx.r[10].s64 + -2012;
	// 826DD4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD4C4: 4BD8995D  bl 0x82466e20
	ctx.lr = 0x826DD4C8;
	sub_82466E20(ctx, base);
	// 826DD4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD4D8 size=108
    let mut pc: u32 = 0x826DD4D8;
    'dispatch: loop {
        match pc {
            0x826DD4D8 => {
    //   block [0x826DD4D8..0x826DD544)
	// 826DD4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD4E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD4E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD4EC: 38EB6DE8  addi r7, r11, 0x6de8
	ctx.r[7].s64 = ctx.r[11].s64 + 28136;
	// 826DD4F0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826DD4F4: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 826DD4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD4FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD508: 386AF854  addi r3, r10, -0x7ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1964;
	// 826DD50C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD52C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD530: 4BD898F1  bl 0x82466e20
	ctx.lr = 0x826DD534;
	sub_82466E20(ctx, base);
	// 826DD534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD548 size=108
    let mut pc: u32 = 0x826DD548;
    'dispatch: loop {
        match pc {
            0x826DD548 => {
    //   block [0x826DD548..0x826DD5B4)
	// 826DD548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD554: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD558: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD55C: 38EB6E30  addi r7, r11, 0x6e30
	ctx.r[7].s64 = ctx.r[11].s64 + 28208;
	// 826DD560: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826DD564: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 826DD568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD56C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD578: 386AF884  addi r3, r10, -0x77c
	ctx.r[3].s64 = ctx.r[10].s64 + -1916;
	// 826DD57C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD59C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD5A0: 4BD89881  bl 0x82466e20
	ctx.lr = 0x826DD5A4;
	sub_82466E20(ctx, base);
	// 826DD5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD5B8 size=108
    let mut pc: u32 = 0x826DD5B8;
    'dispatch: loop {
        match pc {
            0x826DD5B8 => {
    //   block [0x826DD5B8..0x826DD624)
	// 826DD5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD5C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD5C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD5CC: 38EB6E90  addi r7, r11, 0x6e90
	ctx.r[7].s64 = ctx.r[11].s64 + 28304;
	// 826DD5D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DD5D4: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 826DD5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD5DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD5E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD5E8: 386AF8B4  addi r3, r10, -0x74c
	ctx.r[3].s64 = ctx.r[10].s64 + -1868;
	// 826DD5EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD60C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD610: 4BD89811  bl 0x82466e20
	ctx.lr = 0x826DD614;
	sub_82466E20(ctx, base);
	// 826DD614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD61C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD628 size=116
    let mut pc: u32 = 0x826DD628;
    'dispatch: loop {
        match pc {
            0x826DD628 => {
    //   block [0x826DD628..0x826DD69C)
	// 826DD628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD634: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826DD638: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD63C: 392B5AE0  addi r9, r11, 0x5ae0
	ctx.r[9].s64 = ctx.r[11].s64 + 23264;
	// 826DD640: 38AAFDC4  addi r5, r10, -0x23c
	ctx.r[5].s64 = ctx.r[10].s64 + -572;
	// 826DD644: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD648: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826DD64C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826DD650: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD654: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 826DD658: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD65C: 396B6EC0  addi r11, r11, 0x6ec0
	ctx.r[11].s64 = ctx.r[11].s64 + 28352;
	// 826DD660: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826DD664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD668: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826DD66C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD670: 386AF8E4  addi r3, r10, -0x71c
	ctx.r[3].s64 = ctx.r[10].s64 + -1820;
	// 826DD674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DD678: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826DD67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD680: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826DD684: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD688: 4BD89799  bl 0x82466e20
	ctx.lr = 0x826DD68C;
	sub_82466E20(ctx, base);
	// 826DD68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD6A0 size=100
    let mut pc: u32 = 0x826DD6A0;
    'dispatch: loop {
        match pc {
            0x826DD6A0 => {
    //   block [0x826DD6A0..0x826DD704)
	// 826DD6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD6AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD6B4: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DD6B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD6C0: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 826DD6C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD6D4: 386AF914  addi r3, r10, -0x6ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1772;
	// 826DD6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD6DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD6E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DD6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD6E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD6F0: 4BD89731  bl 0x82466e20
	ctx.lr = 0x826DD6F4;
	sub_82466E20(ctx, base);
	// 826DD6F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD708 size=100
    let mut pc: u32 = 0x826DD708;
    'dispatch: loop {
        match pc {
            0x826DD708 => {
    //   block [0x826DD708..0x826DD76C)
	// 826DD708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD71C: 38AAF9A4  addi r5, r10, -0x65c
	ctx.r[5].s64 = ctx.r[10].s64 + -1628;
	// 826DD720: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD728: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 826DD72C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD73C: 386AF944  addi r3, r10, -0x6bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1724;
	// 826DD740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD744: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD748: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DD74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD750: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD758: 4BD896C9  bl 0x82466e20
	ctx.lr = 0x826DD75C;
	sub_82466E20(ctx, base);
	// 826DD75C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD770 size=100
    let mut pc: u32 = 0x826DD770;
    'dispatch: loop {
        match pc {
            0x826DD770 => {
    //   block [0x826DD770..0x826DD7D4)
	// 826DD770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD77C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD784: 38AAF8E4  addi r5, r10, -0x71c
	ctx.r[5].s64 = ctx.r[10].s64 + -1820;
	// 826DD788: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD790: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 826DD794: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD7A4: 386AF974  addi r3, r10, -0x68c
	ctx.r[3].s64 = ctx.r[10].s64 + -1676;
	// 826DD7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD7AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD7B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DD7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD7B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD7C0: 4BD89661  bl 0x82466e20
	ctx.lr = 0x826DD7C4;
	sub_82466E20(ctx, base);
	// 826DD7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD7D8 size=104
    let mut pc: u32 = 0x826DD7D8;
    'dispatch: loop {
        match pc {
            0x826DD7D8 => {
    //   block [0x826DD7D8..0x826DD840)
	// 826DD7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD7E4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DD7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD7EC: 392A5B60  addi r9, r10, 0x5b60
	ctx.r[9].s64 = ctx.r[10].s64 + 23392;
	// 826DD7F0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD7F8: 38AAF914  addi r5, r10, -0x6ec
	ctx.r[5].s64 = ctx.r[10].s64 + -1772;
	// 826DD7FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD80C: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 826DD810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD818: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DD81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD820: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD824: 386AF9A4  addi r3, r10, -0x65c
	ctx.r[3].s64 = ctx.r[10].s64 + -1628;
	// 826DD828: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DD82C: 4BD895F5  bl 0x82466e20
	ctx.lr = 0x826DD830;
	sub_82466E20(ctx, base);
	// 826DD830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD840 size=108
    let mut pc: u32 = 0x826DD840;
    'dispatch: loop {
        match pc {
            0x826DD840 => {
    //   block [0x826DD840..0x826DD8AC)
	// 826DD840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD84C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD854: 38EB7058  addi r7, r11, 0x7058
	ctx.r[7].s64 = ctx.r[11].s64 + 28760;
	// 826DD858: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826DD85C: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 826DD860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD864: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD868: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826DD86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD870: 386AF9D4  addi r3, r10, -0x62c
	ctx.r[3].s64 = ctx.r[10].s64 + -1580;
	// 826DD874: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826DD878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826DD898: 4BD89589  bl 0x82466e20
	ctx.lr = 0x826DD89C;
	sub_82466E20(ctx, base);
	// 826DD89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD8B0 size=112
    let mut pc: u32 = 0x826DD8B0;
    'dispatch: loop {
        match pc {
            0x826DD8B0 => {
    //   block [0x826DD8B0..0x826DD920)
	// 826DD8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD8BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD8C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD8C4: 38AAF9A4  addi r5, r10, -0x65c
	ctx.r[5].s64 = ctx.r[10].s64 + -1628;
	// 826DD8C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD8CC: 390B7088  addi r8, r11, 0x7088
	ctx.r[8].s64 = ctx.r[11].s64 + 28808;
	// 826DD8D0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826DD8D4: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 826DD8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD8DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD8E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD8E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD8E8: 386AFA04  addi r3, r10, -0x5fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1532;
	// 826DD8EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826DD8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD90C: 4BD89515  bl 0x82466e20
	ctx.lr = 0x826DD910;
	sub_82466E20(ctx, base);
	// 826DD910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826DD920 size=24
    let mut pc: u32 = 0x826DD920;
    'dispatch: loop {
        match pc {
            0x826DD920 => {
    //   block [0x826DD920..0x826DD938)
	// 826DD920: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826DD924: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826DD928: 394AB9F0  addi r10, r10, -0x4610
	ctx.r[10].s64 = ctx.r[10].s64 + -17936;
	// 826DD92C: 816B7130  lwz r11, 0x7130(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28976 as u32) ) } as u64;
	// 826DD930: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826DD934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD938 size=116
    let mut pc: u32 = 0x826DD938;
    'dispatch: loop {
        match pc {
            0x826DD938 => {
    //   block [0x826DD938..0x826DD9AC)
	// 826DD938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD944: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826DD948: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826DD94C: 390BB9F0  addi r8, r11, -0x4610
	ctx.r[8].s64 = ctx.r[11].s64 + -17936;
	// 826DD950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD954: 392A5BD0  addi r9, r10, 0x5bd0
	ctx.r[9].s64 = ctx.r[10].s64 + 23504;
	// 826DD958: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD95C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826DD960: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826DD964: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826DD968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD96C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD97C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826DD980: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 826DD984: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826DD988: 386BFA34  addi r3, r11, -0x5cc
	ctx.r[3].s64 = ctx.r[11].s64 + -1484;
	// 826DD98C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826DD990: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DD998: 4BD89489  bl 0x82466e20
	ctx.lr = 0x826DD99C;
	sub_82466E20(ctx, base);
	// 826DD99C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DD9A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DD9A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DD9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DD9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DD9B0 size=100
    let mut pc: u32 = 0x826DD9B0;
    'dispatch: loop {
        match pc {
            0x826DD9B0 => {
    //   block [0x826DD9B0..0x826DDA14)
	// 826DD9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DD9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DD9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DD9BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DD9C4: 38AAFA34  addi r5, r10, -0x5cc
	ctx.r[5].s64 = ctx.r[10].s64 + -1484;
	// 826DD9C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DD9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DD9D0: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 826DD9D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DD9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DD9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DD9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DD9E4: 386AFA64  addi r3, r10, -0x59c
	ctx.r[3].s64 = ctx.r[10].s64 + -1436;
	// 826DD9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DD9EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DD9F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DD9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DD9F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DD9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDA00: 4BD89421  bl 0x82466e20
	ctx.lr = 0x826DDA04;
	sub_82466E20(ctx, base);
	// 826DDA04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDA08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDA0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDA10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDA18 size=100
    let mut pc: u32 = 0x826DDA18;
    'dispatch: loop {
        match pc {
            0x826DDA18 => {
    //   block [0x826DDA18..0x826DDA7C)
	// 826DDA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDA24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDA2C: 38AAFA34  addi r5, r10, -0x5cc
	ctx.r[5].s64 = ctx.r[10].s64 + -1484;
	// 826DDA30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDA34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDA38: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 826DDA3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDA4C: 386AFA94  addi r3, r10, -0x56c
	ctx.r[3].s64 = ctx.r[10].s64 + -1388;
	// 826DDA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDA54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDA58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDA60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDA68: 4BD893B9  bl 0x82466e20
	ctx.lr = 0x826DDA6C;
	sub_82466E20(ctx, base);
	// 826DDA6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDA70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDA74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDA78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDA80 size=100
    let mut pc: u32 = 0x826DDA80;
    'dispatch: loop {
        match pc {
            0x826DDA80 => {
    //   block [0x826DDA80..0x826DDAE4)
	// 826DDA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDA8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDA94: 38AAFAF4  addi r5, r10, -0x50c
	ctx.r[5].s64 = ctx.r[10].s64 + -1292;
	// 826DDA98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDAA0: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 826DDAA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDAA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDAAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDAB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDAB4: 386AFAC4  addi r3, r10, -0x53c
	ctx.r[3].s64 = ctx.r[10].s64 + -1340;
	// 826DDAB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDABC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDAC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDAC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDAD0: 4BD89351  bl 0x82466e20
	ctx.lr = 0x826DDAD4;
	sub_82466E20(ctx, base);
	// 826DDAD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDAD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDADC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDAE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDAE8 size=100
    let mut pc: u32 = 0x826DDAE8;
    'dispatch: loop {
        match pc {
            0x826DDAE8 => {
    //   block [0x826DDAE8..0x826DDB4C)
	// 826DDAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDAF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDAFC: 38AAFA34  addi r5, r10, -0x5cc
	ctx.r[5].s64 = ctx.r[10].s64 + -1484;
	// 826DDB00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDB08: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 826DDB0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDB10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDB18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDB1C: 386AFAF4  addi r3, r10, -0x50c
	ctx.r[3].s64 = ctx.r[10].s64 + -1292;
	// 826DDB20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDB24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDB28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDB30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDB38: 4BD892E9  bl 0x82466e20
	ctx.lr = 0x826DDB3C;
	sub_82466E20(ctx, base);
	// 826DDB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDB50 size=100
    let mut pc: u32 = 0x826DDB50;
    'dispatch: loop {
        match pc {
            0x826DDB50 => {
    //   block [0x826DDB50..0x826DDBB4)
	// 826DDB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDB5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDB64: 38AAFAF4  addi r5, r10, -0x50c
	ctx.r[5].s64 = ctx.r[10].s64 + -1292;
	// 826DDB68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDB6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDB70: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 826DDB74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDB78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDB7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDB80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDB84: 386AFB24  addi r3, r10, -0x4dc
	ctx.r[3].s64 = ctx.r[10].s64 + -1244;
	// 826DDB88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDB8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDB90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDB94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDB98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDBA0: 4BD89281  bl 0x82466e20
	ctx.lr = 0x826DDBA4;
	sub_82466E20(ctx, base);
	// 826DDBA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDBA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDBAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDBB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDBB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDBB8 size=100
    let mut pc: u32 = 0x826DDBB8;
    'dispatch: loop {
        match pc {
            0x826DDBB8 => {
    //   block [0x826DDBB8..0x826DDC1C)
	// 826DDBB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDBBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDBC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDBC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDBC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDBCC: 38AAFA34  addi r5, r10, -0x5cc
	ctx.r[5].s64 = ctx.r[10].s64 + -1484;
	// 826DDBD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDBD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDBD8: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 826DDBDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDBE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDBEC: 386AFB54  addi r3, r10, -0x4ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1196;
	// 826DDBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDBF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDBF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDBFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDC00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDC08: 4BD89219  bl 0x82466e20
	ctx.lr = 0x826DDC0C;
	sub_82466E20(ctx, base);
	// 826DDC0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDC10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDC14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDC20 size=100
    let mut pc: u32 = 0x826DDC20;
    'dispatch: loop {
        match pc {
            0x826DDC20 => {
    //   block [0x826DDC20..0x826DDC84)
	// 826DDC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDC2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDC34: 38AAFA64  addi r5, r10, -0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + -1436;
	// 826DDC38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDC40: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 826DDC44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDC54: 386AFB84  addi r3, r10, -0x47c
	ctx.r[3].s64 = ctx.r[10].s64 + -1148;
	// 826DDC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDC5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDC60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDC68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDC6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDC70: 4BD891B1  bl 0x82466e20
	ctx.lr = 0x826DDC74;
	sub_82466E20(ctx, base);
	// 826DDC74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDC78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDC7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826DDC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826DDC88 size=100
    let mut pc: u32 = 0x826DDC88;
    'dispatch: loop {
        match pc {
            0x826DDC88 => {
    //   block [0x826DDC88..0x826DDCEC)
	// 826DDC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826DDC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826DDC90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826DDC94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826DDC9C: 38AAFB54  addi r5, r10, -0x4ac
	ctx.r[5].s64 = ctx.r[10].s64 + -1196;
	// 826DDCA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826DDCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826DDCA8: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 826DDCAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826DDCB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826DDCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826DDCB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826DDCBC: 386AFBB4  addi r3, r10, -0x44c
	ctx.r[3].s64 = ctx.r[10].s64 + -1100;
	// 826DDCC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826DDCC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826DDCC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826DDCCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826DDCD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826DDCD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826DDCD8: 4BD89149  bl 0x82466e20
	ctx.lr = 0x826DDCDC;
	sub_82466E20(ctx, base);
	// 826DDCDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826DDCE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826DDCE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826DDCE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


