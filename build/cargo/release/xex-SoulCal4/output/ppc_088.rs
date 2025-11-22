pub fn sub_825E98C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E98C0 size=12
    let mut pc: u32 = 0x825E98C0;
    'dispatch: loop {
        match pc {
            0x825E98C0 => {
    //   block [0x825E98C0..0x825E98CC)
	// 825E98C0: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E98C4: 386BCC50  addi r3, r11, -0x33b0
	ctx.r[3].s64 = ctx.r[11].s64 + -13232;
	// 825E98C8: 4BF49270  b 0x82532b38
	sub_82532B38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E98D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825E98D0 size=176
    let mut pc: u32 = 0x825E98D0;
    'dispatch: loop {
        match pc {
            0x825E98D0 => {
    //   block [0x825E98D0..0x825E9980)
	// 825E98D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E98D4: 4BF4B7E5  bl 0x825350b8
	ctx.lr = 0x825E98D8;
	sub_82535080(ctx, base);
	// 825E98D8: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 825E98DC: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 825E98E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E98E4: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 825E98E8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 825E98EC: 396B5870  addi r11, r11, 0x5870
	ctx.r[11].s64 = ctx.r[11].s64 + 22640;
	// 825E98F0: 3B800007  li r28, 7
	ctx.r[28].s64 = 7;
	// 825E98F4: 3BEB00F4  addi r31, r11, 0xf4
	ctx.r[31].s64 = ctx.r[11].s64 + 244;
	// 825E98F8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825E98FC: C3EA1FF8  lfs f31, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825E9900: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 825E9904: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 825E9908: C3CB2268  lfs f30, 0x2268(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8808 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 825E990C: 387F0108  addi r3, r31, 0x108
	ctx.r[3].s64 = ctx.r[31].s64 + 264;
	// 825E9910: 93DFFF98  stw r30, -0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-104 as u32), ctx.r[30].u32 ) };
	// 825E9914: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 825E9918: 9BDF0004  stb r30, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u8 ) };
	// 825E991C: 9BDF0005  stb r30, 5(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(5 as u32), ctx.r[30].u8 ) };
	// 825E9920: 9BDF0006  stb r30, 6(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[30].u8 ) };
	// 825E9924: 9BDF0007  stb r30, 7(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(7 as u32), ctx.r[30].u8 ) };
	// 825E9928: 93BF0040  stw r29, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[29].u32 ) };
	// 825E992C: 4BE1DE6D  bl 0x82407798
	ctx.lr = 0x825E9930;
	sub_82407798(ctx, base);
	// 825E9930: D3DF0118  stfs f30, 0x118(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(280 as u32), tmp.u32 ) };
	// 825E9934: 9BDF011C  stb r30, 0x11c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(284 as u32), ctx.r[30].u8 ) };
	// 825E9938: D3FF1128  stfs f31, 0x1128(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4392 as u32), tmp.u32 ) };
	// 825E993C: 9BDF011D  stb r30, 0x11d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(285 as u32), ctx.r[30].u8 ) };
	// 825E9940: D3FF112C  stfs f31, 0x112c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4396 as u32), tmp.u32 ) };
	// 825E9944: 93BF0120  stw r29, 0x120(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(288 as u32), ctx.r[29].u32 ) };
	// 825E9948: 93DF0124  stw r30, 0x124(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(292 as u32), ctx.r[30].u32 ) };
	// 825E994C: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 825E9950: 93BF1130  stw r29, 0x1130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4400 as u32), ctx.r[29].u32 ) };
	// 825E9954: 93DF1134  stw r30, 0x1134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4404 as u32), ctx.r[30].u32 ) };
	// 825E9958: 93DF1138  stw r30, 0x1138(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4408 as u32), ctx.r[30].u32 ) };
	// 825E995C: 3BFF132C  addi r31, r31, 0x132c
	ctx.r[31].s64 = ctx.r[31].s64 + 4908;
	// 825E9960: 4080FFAC  bge 0x825e990c
	if !ctx.cr[0].lt {
	pc = 0x825E990C; continue 'dispatch;
	}
	// 825E9964: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825E9968: 386BCC58  addi r3, r11, -0x33a8
	ctx.r[3].s64 = ctx.r[11].s64 + -13224;
	// 825E996C: 4BF491CD  bl 0x82532b38
	ctx.lr = 0x825E9970;
	sub_82532B38(ctx, base);
	// 825E9970: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825E9974: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 825E9978: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 825E997C: 4BF4B78C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9980 size=108
    let mut pc: u32 = 0x825E9980;
    'dispatch: loop {
        match pc {
            0x825E9980 => {
    //   block [0x825E9980..0x825E99EC)
	// 825E9980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E998C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9990: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9994: 38EB6E00  addi r7, r11, 0x6e00
	ctx.r[7].s64 = ctx.r[11].s64 + 28160;
	// 825E9998: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825E999C: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 825E99A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E99A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E99A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E99AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E99B0: 386A8D64  addi r3, r10, -0x729c
	ctx.r[3].s64 = ctx.r[10].s64 + -29340;
	// 825E99B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E99B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E99BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E99C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E99C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E99C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E99CC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825E99D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E99D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E99D8: 4BE7D449  bl 0x82466e20
	ctx.lr = 0x825E99DC;
	sub_82466E20(ctx, base);
	// 825E99DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E99E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E99E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E99E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E99F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E99F0 size=108
    let mut pc: u32 = 0x825E99F0;
    'dispatch: loop {
        match pc {
            0x825E99F0 => {
    //   block [0x825E99F0..0x825E9A5C)
	// 825E99F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E99F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E99F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E99FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9A00: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9A04: 38EB6E30  addi r7, r11, 0x6e30
	ctx.r[7].s64 = ctx.r[11].s64 + 28208;
	// 825E9A08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E9A0C: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 825E9A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9A14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9A18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9A20: 386A8D94  addi r3, r10, -0x726c
	ctx.r[3].s64 = ctx.r[10].s64 + -29292;
	// 825E9A24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9A28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9A30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9A38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9A3C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825E9A40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9A44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9A48: 4BE7D3D9  bl 0x82466e20
	ctx.lr = 0x825E9A4C;
	sub_82466E20(ctx, base);
	// 825E9A4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9A60 size=40
    let mut pc: u32 = 0x825E9A60;
    'dispatch: loop {
        match pc {
            0x825E9A60 => {
    //   block [0x825E9A60..0x825E9A88)
	// 825E9A60: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9A64: 814B4558  lwz r10, 0x4558(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(17752 as u32) ) } as u64;
	// 825E9A68: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9A6C: 396B4578  addi r11, r11, 0x4578
	ctx.r[11].s64 = ctx.r[11].s64 + 17784;
	// 825E9A70: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825E9A74: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 825E9A78: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825E9A7C: 814A455C  lwz r10, 0x455c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(17756 as u32) ) } as u64;
	// 825E9A80: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 825E9A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9A88 size=112
    let mut pc: u32 = 0x825E9A88;
    'dispatch: loop {
        match pc {
            0x825E9A88 => {
    //   block [0x825E9A88..0x825E9AF8)
	// 825E9A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9A94: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9A98: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9A9C: 392A72B8  addi r9, r10, 0x72b8
	ctx.r[9].s64 = ctx.r[10].s64 + 29368;
	// 825E9AA0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9AA4: 390B4578  addi r8, r11, 0x4578
	ctx.r[8].s64 = ctx.r[11].s64 + 17784;
	// 825E9AA8: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825E9AAC: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 825E9AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9AB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825E9ABC: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 825E9AC0: 386A8DC4  addi r3, r10, -0x723c
	ctx.r[3].s64 = ctx.r[10].s64 + -29244;
	// 825E9AC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825E9AC8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825E9ACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9ADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9AE4: 4BE7D33D  bl 0x82466e20
	ctx.lr = 0x825E9AE8;
	sub_82466E20(ctx, base);
	// 825E9AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9AF8 size=112
    let mut pc: u32 = 0x825E9AF8;
    'dispatch: loop {
        match pc {
            0x825E9AF8 => {
    //   block [0x825E9AF8..0x825E9B68)
	// 825E9AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9B04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9B08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9B0C: 38AA9130  addi r5, r10, -0x6ed0
	ctx.r[5].s64 = ctx.r[10].s64 + -28368;
	// 825E9B10: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9B14: 390B7340  addi r8, r11, 0x7340
	ctx.r[8].s64 = ctx.r[11].s64 + 29504;
	// 825E9B18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825E9B1C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 825E9B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9B24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825E9B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9B30: 386A8DF4  addi r3, r10, -0x720c
	ctx.r[3].s64 = ctx.r[10].s64 + -29196;
	// 825E9B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825E9B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9B4C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825E9B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9B54: 4BE7D2CD  bl 0x82466e20
	ctx.lr = 0x825E9B58;
	sub_82466E20(ctx, base);
	// 825E9B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9B68 size=24
    let mut pc: u32 = 0x825E9B68;
    'dispatch: loop {
        match pc {
            0x825E9B68 => {
    //   block [0x825E9B68..0x825E9B80)
	// 825E9B68: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9B6C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825E9B70: 394A46A0  addi r10, r10, 0x46a0
	ctx.r[10].s64 = ctx.r[10].s64 + 18080;
	// 825E9B74: 816B468C  lwz r11, 0x468c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18060 as u32) ) } as u64;
	// 825E9B78: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 825E9B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9B80 size=112
    let mut pc: u32 = 0x825E9B80;
    'dispatch: loop {
        match pc {
            0x825E9B80 => {
    //   block [0x825E9B80..0x825E9BF0)
	// 825E9B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9B8C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9B90: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9B94: 392A7440  addi r9, r10, 0x7440
	ctx.r[9].s64 = ctx.r[10].s64 + 29760;
	// 825E9B98: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9B9C: 390B46A0  addi r8, r11, 0x46a0
	ctx.r[8].s64 = ctx.r[11].s64 + 18080;
	// 825E9BA0: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 825E9BA4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 825E9BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9BAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825E9BB4: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825E9BB8: 386A8E24  addi r3, r10, -0x71dc
	ctx.r[3].s64 = ctx.r[10].s64 + -29148;
	// 825E9BBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825E9BC0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825E9BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9BD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9BDC: 4BE7D245  bl 0x82466e20
	ctx.lr = 0x825E9BE0;
	sub_82466E20(ctx, base);
	// 825E9BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9BF0 size=108
    let mut pc: u32 = 0x825E9BF0;
    'dispatch: loop {
        match pc {
            0x825E9BF0 => {
    //   block [0x825E9BF0..0x825E9C5C)
	// 825E9BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9BFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9C00: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9C04: 38EB7714  addi r7, r11, 0x7714
	ctx.r[7].s64 = ctx.r[11].s64 + 30484;
	// 825E9C08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825E9C0C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 825E9C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9C14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9C18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9C20: 386A8E54  addi r3, r10, -0x71ac
	ctx.r[3].s64 = ctx.r[10].s64 + -29100;
	// 825E9C24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9C34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9C3C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825E9C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9C48: 4BE7D1D9  bl 0x82466e20
	ctx.lr = 0x825E9C4C;
	sub_82466E20(ctx, base);
	// 825E9C4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9C50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9C54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9C60 size=108
    let mut pc: u32 = 0x825E9C60;
    'dispatch: loop {
        match pc {
            0x825E9C60 => {
    //   block [0x825E9C60..0x825E9CCC)
	// 825E9C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9C6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9C70: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9C74: 38EB7744  addi r7, r11, 0x7744
	ctx.r[7].s64 = ctx.r[11].s64 + 30532;
	// 825E9C78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E9C7C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 825E9C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9C84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9C88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9C90: 386A8E84  addi r3, r10, -0x717c
	ctx.r[3].s64 = ctx.r[10].s64 + -29052;
	// 825E9C94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9C98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9CAC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825E9CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9CB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9CB8: 4BE7D169  bl 0x82466e20
	ctx.lr = 0x825E9CBC;
	sub_82466E20(ctx, base);
	// 825E9CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9CD0 size=24
    let mut pc: u32 = 0x825E9CD0;
    'dispatch: loop {
        match pc {
            0x825E9CD0 => {
    //   block [0x825E9CD0..0x825E9CE8)
	// 825E9CD0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9CD4: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825E9CD8: 394A4800  addi r10, r10, 0x4800
	ctx.r[10].s64 = ctx.r[10].s64 + 18432;
	// 825E9CDC: 816B47C8  lwz r11, 0x47c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18376 as u32) ) } as u64;
	// 825E9CE0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825E9CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9CE8 size=112
    let mut pc: u32 = 0x825E9CE8;
    'dispatch: loop {
        match pc {
            0x825E9CE8 => {
    //   block [0x825E9CE8..0x825E9D58)
	// 825E9CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9CF4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9CF8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9CFC: 392A7774  addi r9, r10, 0x7774
	ctx.r[9].s64 = ctx.r[10].s64 + 30580;
	// 825E9D00: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9D04: 390B4800  addi r8, r11, 0x4800
	ctx.r[8].s64 = ctx.r[11].s64 + 18432;
	// 825E9D08: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825E9D0C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 825E9D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9D14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825E9D1C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 825E9D20: 386A8EB4  addi r3, r10, -0x714c
	ctx.r[3].s64 = ctx.r[10].s64 + -29004;
	// 825E9D24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825E9D28: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E9D2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9D34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9D3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9D44: 4BE7D0DD  bl 0x82466e20
	ctx.lr = 0x825E9D48;
	sub_82466E20(ctx, base);
	// 825E9D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9D58 size=108
    let mut pc: u32 = 0x825E9D58;
    'dispatch: loop {
        match pc {
            0x825E9D58 => {
    //   block [0x825E9D58..0x825E9DC4)
	// 825E9D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9D64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9D68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9D6C: 38EB7858  addi r7, r11, 0x7858
	ctx.r[7].s64 = ctx.r[11].s64 + 30808;
	// 825E9D70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825E9D74: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 825E9D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9D7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9D80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9D88: 386A8EE4  addi r3, r10, -0x711c
	ctx.r[3].s64 = ctx.r[10].s64 + -28956;
	// 825E9D8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9D94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9D9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9DA4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825E9DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9DAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9DB0: 4BE7D071  bl 0x82466e20
	ctx.lr = 0x825E9DB4;
	sub_82466E20(ctx, base);
	// 825E9DB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9DB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9DBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825E9DC8 size=24
    let mut pc: u32 = 0x825E9DC8;
    'dispatch: loop {
        match pc {
            0x825E9DC8 => {
    //   block [0x825E9DC8..0x825E9DE0)
	// 825E9DC8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9DCC: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825E9DD0: 394A4900  addi r10, r10, 0x4900
	ctx.r[10].s64 = ctx.r[10].s64 + 18688;
	// 825E9DD4: 816B48D8  lwz r11, 0x48d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(18648 as u32) ) } as u64;
	// 825E9DD8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825E9DDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9DE0 size=112
    let mut pc: u32 = 0x825E9DE0;
    'dispatch: loop {
        match pc {
            0x825E9DE0 => {
    //   block [0x825E9DE0..0x825E9E50)
	// 825E9DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9DEC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9DF0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825E9DF4: 392A7844  addi r9, r10, 0x7844
	ctx.r[9].s64 = ctx.r[10].s64 + 30788;
	// 825E9DF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9DFC: 390B4900  addi r8, r11, 0x4900
	ctx.r[8].s64 = ctx.r[11].s64 + 18688;
	// 825E9E00: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825E9E04: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 825E9E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9E0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9E10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825E9E14: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 825E9E18: 386A8F14  addi r3, r10, -0x70ec
	ctx.r[3].s64 = ctx.r[10].s64 + -28908;
	// 825E9E1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825E9E20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825E9E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9E34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9E3C: 4BE7CFE5  bl 0x82466e20
	ctx.lr = 0x825E9E40;
	sub_82466E20(ctx, base);
	// 825E9E40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9E4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9E50 size=108
    let mut pc: u32 = 0x825E9E50;
    'dispatch: loop {
        match pc {
            0x825E9E50 => {
    //   block [0x825E9E50..0x825E9EBC)
	// 825E9E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9E58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9E5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9E60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9E64: 38EB78A8  addi r7, r11, 0x78a8
	ctx.r[7].s64 = ctx.r[11].s64 + 30888;
	// 825E9E68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825E9E6C: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 825E9E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9E74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9E78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9E80: 386A8F44  addi r3, r10, -0x70bc
	ctx.r[3].s64 = ctx.r[10].s64 + -28860;
	// 825E9E84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9E94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9E9C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825E9EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9EA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9EA8: 4BE7CF79  bl 0x82466e20
	ctx.lr = 0x825E9EAC;
	sub_82466E20(ctx, base);
	// 825E9EAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9EB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9EB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9EC0 size=108
    let mut pc: u32 = 0x825E9EC0;
    'dispatch: loop {
        match pc {
            0x825E9EC0 => {
    //   block [0x825E9EC0..0x825E9F2C)
	// 825E9EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9EC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9ECC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9ED0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9ED4: 38EB78E4  addi r7, r11, 0x78e4
	ctx.r[7].s64 = ctx.r[11].s64 + 30948;
	// 825E9ED8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825E9EDC: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 825E9EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9EE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9EE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9EEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9EF0: 386A8F74  addi r3, r10, -0x708c
	ctx.r[3].s64 = ctx.r[10].s64 + -28812;
	// 825E9EF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9EFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9F04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9F0C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825E9F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9F14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9F18: 4BE7CF09  bl 0x82466e20
	ctx.lr = 0x825E9F1C;
	sub_82466E20(ctx, base);
	// 825E9F1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9F30 size=108
    let mut pc: u32 = 0x825E9F30;
    'dispatch: loop {
        match pc {
            0x825E9F30 => {
    //   block [0x825E9F30..0x825E9F9C)
	// 825E9F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9F38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9F3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9F40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9F44: 38EB7940  addi r7, r11, 0x7940
	ctx.r[7].s64 = ctx.r[11].s64 + 31040;
	// 825E9F48: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825E9F4C: 388A7A00  addi r4, r10, 0x7a00
	ctx.r[4].s64 = ctx.r[10].s64 + 31232;
	// 825E9F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9F54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9F58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9F60: 386A8FA4  addi r3, r10, -0x705c
	ctx.r[3].s64 = ctx.r[10].s64 + -28764;
	// 825E9F64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9F68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9F6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9F74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9F7C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825E9F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9F84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9F88: 4BE7CE99  bl 0x82466e20
	ctx.lr = 0x825E9F8C;
	sub_82466E20(ctx, base);
	// 825E9F8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825E9F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825E9F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825E9F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825E9FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825E9FA0 size=108
    let mut pc: u32 = 0x825E9FA0;
    'dispatch: loop {
        match pc {
            0x825E9FA0 => {
    //   block [0x825E9FA0..0x825EA00C)
	// 825E9FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825E9FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825E9FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825E9FAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825E9FB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825E9FB4: 38EB79A0  addi r7, r11, 0x79a0
	ctx.r[7].s64 = ctx.r[11].s64 + 31136;
	// 825E9FB8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825E9FBC: 388A7A18  addi r4, r10, 0x7a18
	ctx.r[4].s64 = ctx.r[10].s64 + 31256;
	// 825E9FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825E9FC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825E9FC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825E9FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825E9FD0: 386A8FD4  addi r3, r10, -0x702c
	ctx.r[3].s64 = ctx.r[10].s64 + -28716;
	// 825E9FD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825E9FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825E9FDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825E9FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825E9FE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825E9FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825E9FEC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825E9FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825E9FF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825E9FF8: 4BE7CE29  bl 0x82466e20
	ctx.lr = 0x825E9FFC;
	sub_82466E20(ctx, base);
	// 825E9FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA010 size=108
    let mut pc: u32 = 0x825EA010;
    'dispatch: loop {
        match pc {
            0x825EA010 => {
    //   block [0x825EA010..0x825EA07C)
	// 825EA010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA01C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825EA020: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EA024: 38EB7AF0  addi r7, r11, 0x7af0
	ctx.r[7].s64 = ctx.r[11].s64 + 31472;
	// 825EA028: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA02C: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 825EA030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA034: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA03C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA040: 386A9008  addi r3, r10, -0x6ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -28664;
	// 825EA044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA05C: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 825EA060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA068: 4BE7CDB9  bl 0x82466e20
	ctx.lr = 0x825EA06C;
	sub_82466E20(ctx, base);
	// 825EA06C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA080 size=108
    let mut pc: u32 = 0x825EA080;
    'dispatch: loop {
        match pc {
            0x825EA080 => {
    //   block [0x825EA080..0x825EA0EC)
	// 825EA080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA08C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825EA090: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EA094: 38EB7B20  addi r7, r11, 0x7b20
	ctx.r[7].s64 = ctx.r[11].s64 + 31520;
	// 825EA098: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EA09C: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 825EA0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA0A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA0A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA0B0: 386A9038  addi r3, r10, -0x6fc8
	ctx.r[3].s64 = ctx.r[10].s64 + -28616;
	// 825EA0B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA0B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA0BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA0C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA0C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA0CC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 825EA0D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA0D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA0D8: 4BE7CD49  bl 0x82466e20
	ctx.lr = 0x825EA0DC;
	sub_82466E20(ctx, base);
	// 825EA0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA0F0 size=28
    let mut pc: u32 = 0x825EA0F0;
    'dispatch: loop {
        match pc {
            0x825EA0F0 => {
    //   block [0x825EA0F0..0x825EA10C)
	// 825EA0F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA0F4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA0F8: 396B4AF0  addi r11, r11, 0x4af0
	ctx.r[11].s64 = ctx.r[11].s64 + 19184;
	// 825EA0FC: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 825EA100: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 825EA104: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825EA108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA110 size=108
    let mut pc: u32 = 0x825EA110;
    'dispatch: loop {
        match pc {
            0x825EA110 => {
    //   block [0x825EA110..0x825EA17C)
	// 825EA110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA11C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825EA120: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EA124: 38EB7E50  addi r7, r11, 0x7e50
	ctx.r[7].s64 = ctx.r[11].s64 + 32336;
	// 825EA128: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825EA12C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 825EA130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA134: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA140: 386A9070  addi r3, r10, -0x6f90
	ctx.r[3].s64 = ctx.r[10].s64 + -28560;
	// 825EA144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA15C: 38C000B0  li r6, 0xb0
	ctx.r[6].s64 = 176;
	// 825EA160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA168: 4BE7CCB9  bl 0x82466e20
	ctx.lr = 0x825EA16C;
	sub_82466E20(ctx, base);
	// 825EA16C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA180 size=108
    let mut pc: u32 = 0x825EA180;
    'dispatch: loop {
        match pc {
            0x825EA180 => {
    //   block [0x825EA180..0x825EA1EC)
	// 825EA180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA18C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825EA190: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EA194: 38EB7F58  addi r7, r11, 0x7f58
	ctx.r[7].s64 = ctx.r[11].s64 + 32600;
	// 825EA198: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA19C: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 825EA1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA1A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA1B0: 386A90A0  addi r3, r10, -0x6f60
	ctx.r[3].s64 = ctx.r[10].s64 + -28512;
	// 825EA1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA1CC: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 825EA1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA1D8: 4BE7CC49  bl 0x82466e20
	ctx.lr = 0x825EA1DC;
	sub_82466E20(ctx, base);
	// 825EA1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA1F0 size=108
    let mut pc: u32 = 0x825EA1F0;
    'dispatch: loop {
        match pc {
            0x825EA1F0 => {
    //   block [0x825EA1F0..0x825EA25C)
	// 825EA1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA1FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825EA200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA204: 38EB7FE8  addi r7, r11, 0x7fe8
	ctx.r[7].s64 = ctx.r[11].s64 + 32744;
	// 825EA208: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EA20C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 825EA210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA214: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA220: 386A90D0  addi r3, r10, -0x6f30
	ctx.r[3].s64 = ctx.r[10].s64 + -28464;
	// 825EA224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA23C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 825EA240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA248: 4BE7CBD9  bl 0x82466e20
	ctx.lr = 0x825EA24C;
	sub_82466E20(ctx, base);
	// 825EA24C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA260 size=112
    let mut pc: u32 = 0x825EA260;
    'dispatch: loop {
        match pc {
            0x825EA260 => {
    //   block [0x825EA260..0x825EA2D0)
	// 825EA260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA26C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA270: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA274: 392A8128  addi r9, r10, -0x7ed8
	ctx.r[9].s64 = ctx.r[10].s64 + -32472;
	// 825EA278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA27C: 390B8150  addi r8, r11, -0x7eb0
	ctx.r[8].s64 = ctx.r[11].s64 + -32432;
	// 825EA280: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825EA284: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 825EA288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA28C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EA294: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EA298: 386A9100  addi r3, r10, -0x6f00
	ctx.r[3].s64 = ctx.r[10].s64 + -28416;
	// 825EA29C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EA2A0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825EA2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA2B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA2BC: 4BE7CB65  bl 0x82466e20
	ctx.lr = 0x825EA2C0;
	sub_82466E20(ctx, base);
	// 825EA2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA2D0 size=96
    let mut pc: u32 = 0x825EA2D0;
    'dispatch: loop {
        match pc {
            0x825EA2D0 => {
    //   block [0x825EA2D0..0x825EA330)
	// 825EA2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA2DC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EA2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA2E4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 825EA2E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA2F0: 386A9130  addi r3, r10, -0x6ed0
	ctx.r[3].s64 = ctx.r[10].s64 + -28368;
	// 825EA2F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA2FC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EA300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA30C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 825EA310: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EA314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA318: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EA31C: 4BE7CB05  bl 0x82466e20
	ctx.lr = 0x825EA320;
	sub_82466E20(ctx, base);
	// 825EA320: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA330 size=112
    let mut pc: u32 = 0x825EA330;
    'dispatch: loop {
        match pc {
            0x825EA330 => {
    //   block [0x825EA330..0x825EA3A0)
	// 825EA330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA33C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA340: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA344: 392A8258  addi r9, r10, -0x7da8
	ctx.r[9].s64 = ctx.r[10].s64 + -32168;
	// 825EA348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA34C: 390B8270  addi r8, r11, -0x7d90
	ctx.r[8].s64 = ctx.r[11].s64 + -32144;
	// 825EA350: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EA354: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 825EA358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA35C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EA364: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EA368: 386A9160  addi r3, r10, -0x6ea0
	ctx.r[3].s64 = ctx.r[10].s64 + -28320;
	// 825EA36C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EA370: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EA374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA384: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA38C: 4BE7CA95  bl 0x82466e20
	ctx.lr = 0x825EA390;
	sub_82466E20(ctx, base);
	// 825EA390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA3A0 size=76
    let mut pc: u32 = 0x825EA3A0;
    'dispatch: loop {
        match pc {
            0x825EA3A0 => {
    //   block [0x825EA3A0..0x825EA3EC)
	// 825EA3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA3A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825EA3AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA3B0: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EA3B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 825EA3B8: 3BEB9198  addi r31, r11, -0x6e68
	ctx.r[31].s64 = ctx.r[11].s64 + -28264;
	// 825EA3BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825EA3C0: 4812395D  bl 0x8270dd1c
	ctx.lr = 0x825EA3C4;
	// extern call 0x8270DD1C → crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount
	crate::xboxkrnl::RtlInitializeCriticalSectionAndSpinCount(ctx, base);
	// 825EA3C4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 825EA3C8: 3D408271  lis r10, -0x7d8f
	ctx.r[10].s64 = -2106523648;
	// 825EA3CC: 386ACCB0  addi r3, r10, -0x3350
	ctx.r[3].s64 = ctx.r[10].s64 + -13136;
	// 825EA3D0: F97F0020  std r11, 0x20(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u64 ) };
	// 825EA3D4: 4BF48765  bl 0x82532b38
	ctx.lr = 0x825EA3D8;
	sub_82532B38(ctx, base);
	// 825EA3D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825EA3DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA3E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA3E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825EA3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA3F0 size=28
    let mut pc: u32 = 0x825EA3F0;
    'dispatch: loop {
        match pc {
            0x825EA3F0 => {
    //   block [0x825EA3F0..0x825EA40C)
	// 825EA3F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA3F4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA3F8: 396B4F74  addi r11, r11, 0x4f74
	ctx.r[11].s64 = ctx.r[11].s64 + 20340;
	// 825EA3FC: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 825EA400: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 825EA404: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825EA408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA410 size=28
    let mut pc: u32 = 0x825EA410;
    'dispatch: loop {
        match pc {
            0x825EA410 => {
    //   block [0x825EA410..0x825EA42C)
	// 825EA410: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA414: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA418: 396B4F88  addi r11, r11, 0x4f88
	ctx.r[11].s64 = ctx.r[11].s64 + 20360;
	// 825EA41C: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 825EA420: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 825EA424: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825EA428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA430 size=108
    let mut pc: u32 = 0x825EA430;
    'dispatch: loop {
        match pc {
            0x825EA430 => {
    //   block [0x825EA430..0x825EA49C)
	// 825EA430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA43C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA444: 38EB8C90  addi r7, r11, -0x7370
	ctx.r[7].s64 = ctx.r[11].s64 + -29552;
	// 825EA448: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825EA44C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 825EA450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA454: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA460: 386A9214  addi r3, r10, -0x6dec
	ctx.r[3].s64 = ctx.r[10].s64 + -28140;
	// 825EA464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA47C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EA480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA488: 4BE7C999  bl 0x82466e20
	ctx.lr = 0x825EA48C;
	sub_82466E20(ctx, base);
	// 825EA48C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA4A0 size=108
    let mut pc: u32 = 0x825EA4A0;
    'dispatch: loop {
        match pc {
            0x825EA4A0 => {
    //   block [0x825EA4A0..0x825EA50C)
	// 825EA4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA4AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA4B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA4B4: 38EB8CD8  addi r7, r11, -0x7328
	ctx.r[7].s64 = ctx.r[11].s64 + -29480;
	// 825EA4B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EA4BC: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 825EA4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA4C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA4D0: 386A9244  addi r3, r10, -0x6dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -28092;
	// 825EA4D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA4EC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EA4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA4F8: 4BE7C929  bl 0x82466e20
	ctx.lr = 0x825EA4FC;
	sub_82466E20(ctx, base);
	// 825EA4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA510 size=108
    let mut pc: u32 = 0x825EA510;
    'dispatch: loop {
        match pc {
            0x825EA510 => {
    //   block [0x825EA510..0x825EA57C)
	// 825EA510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA51C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA524: 38EB8DC0  addi r7, r11, -0x7240
	ctx.r[7].s64 = ctx.r[11].s64 + -29248;
	// 825EA528: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825EA52C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 825EA530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA534: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA540: 386A9274  addi r3, r10, -0x6d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -28044;
	// 825EA544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA55C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825EA560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA568: 4BE7C8B9  bl 0x82466e20
	ctx.lr = 0x825EA56C;
	sub_82466E20(ctx, base);
	// 825EA56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA580 size=108
    let mut pc: u32 = 0x825EA580;
    'dispatch: loop {
        match pc {
            0x825EA580 => {
    //   block [0x825EA580..0x825EA5EC)
	// 825EA580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA58C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA594: 38EB8F68  addi r7, r11, -0x7098
	ctx.r[7].s64 = ctx.r[11].s64 + -28824;
	// 825EA598: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 825EA59C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 825EA5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA5A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA5B0: 386A92A4  addi r3, r10, -0x6d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -27996;
	// 825EA5B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA5CC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 825EA5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA5D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA5D8: 4BE7C849  bl 0x82466e20
	ctx.lr = 0x825EA5DC;
	sub_82466E20(ctx, base);
	// 825EA5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA5F0 size=72
    let mut pc: u32 = 0x825EA5F0;
    'dispatch: loop {
        match pc {
            0x825EA5F0 => {
    //   block [0x825EA5F0..0x825EA638)
	// 825EA5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA5F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA5FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA600: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 825EA604: 38CB9084  addi r6, r11, -0x6f7c
	ctx.r[6].s64 = ctx.r[11].s64 + -28540;
	// 825EA608: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 825EA60C: 388BB518  addi r4, r11, -0x4ae8
	ctx.r[4].s64 = ctx.r[11].s64 + -19176;
	// 825EA610: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EA614: 386B92D8  addi r3, r11, -0x6d28
	ctx.r[3].s64 = ctx.r[11].s64 + -27944;
	// 825EA618: 4BE914A1  bl 0x8247bab8
	ctx.lr = 0x825EA61C;
	sub_8247BAB8(ctx, base);
	// 825EA61C: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 825EA620: 386BCCD8  addi r3, r11, -0x3328
	ctx.r[3].s64 = ctx.r[11].s64 + -13096;
	// 825EA624: 4BF48515  bl 0x82532b38
	ctx.lr = 0x825EA628;
	sub_82532B38(ctx, base);
	// 825EA628: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825EA62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA638 size=28
    let mut pc: u32 = 0x825EA638;
    'dispatch: loop {
        match pc {
            0x825EA638 => {
    //   block [0x825EA638..0x825EA654)
	// 825EA638: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA63C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA640: 396B5308  addi r11, r11, 0x5308
	ctx.r[11].s64 = ctx.r[11].s64 + 21256;
	// 825EA644: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 825EA648: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 825EA64C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825EA650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA658 size=28
    let mut pc: u32 = 0x825EA658;
    'dispatch: loop {
        match pc {
            0x825EA658 => {
    //   block [0x825EA658..0x825EA674)
	// 825EA658: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA65C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA660: 396B5410  addi r11, r11, 0x5410
	ctx.r[11].s64 = ctx.r[11].s64 + 21520;
	// 825EA664: 812A91D0  lwz r9, -0x6e30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-28208 as u32) ) } as u64;
	// 825EA668: 916A91D0  stw r11, -0x6e30(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-28208 as u32), ctx.r[11].u32 ) };
	// 825EA66C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825EA670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA678 size=36
    let mut pc: u32 = 0x825EA678;
    'dispatch: loop {
        match pc {
            0x825EA678 => {
    //   block [0x825EA678..0x825EA69C)
	// 825EA678: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA67C: 816B9A2C  lwz r11, -0x65d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26068 as u32) ) } as u64;
	// 825EA680: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 825EA684: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 825EA688: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA68C: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 825EA690: 396B5470  addi r11, r11, 0x5470
	ctx.r[11].s64 = ctx.r[11].s64 + 21616;
	// 825EA694: 994B0001  stb r10, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[10].u8 ) };
	// 825EA698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA6A0 size=24
    let mut pc: u32 = 0x825EA6A0;
    'dispatch: loop {
        match pc {
            0x825EA6A0 => {
    //   block [0x825EA6A0..0x825EA6B8)
	// 825EA6A0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA6A4: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EA6A8: 394A54FC  addi r10, r10, 0x54fc
	ctx.r[10].s64 = ctx.r[10].s64 + 21756;
	// 825EA6AC: 816B56AC  lwz r11, 0x56ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22188 as u32) ) } as u64;
	// 825EA6B0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EA6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA6B8 size=108
    let mut pc: u32 = 0x825EA6B8;
    'dispatch: loop {
        match pc {
            0x825EA6B8 => {
    //   block [0x825EA6B8..0x825EA724)
	// 825EA6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA6C4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA6C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA6CC: 38EB54FC  addi r7, r11, 0x54fc
	ctx.r[7].s64 = ctx.r[11].s64 + 21756;
	// 825EA6D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA6D4: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 825EA6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA6DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA6E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA6E8: 386A92F4  addi r3, r10, -0x6d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -27916;
	// 825EA6EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA704: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EA708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA710: 4BE7C711  bl 0x82466e20
	ctx.lr = 0x825EA714;
	sub_82466E20(ctx, base);
	// 825EA714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA728 size=108
    let mut pc: u32 = 0x825EA728;
    'dispatch: loop {
        match pc {
            0x825EA728 => {
    //   block [0x825EA728..0x825EA794)
	// 825EA728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA734: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA73C: 38EB9C0C  addi r7, r11, -0x63f4
	ctx.r[7].s64 = ctx.r[11].s64 + -25588;
	// 825EA740: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EA744: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 825EA748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA74C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA758: 386A9324  addi r3, r10, -0x6cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -27868;
	// 825EA75C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA774: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EA778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA77C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA780: 4BE7C6A1  bl 0x82466e20
	ctx.lr = 0x825EA784;
	sub_82466E20(ctx, base);
	// 825EA784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EA798 size=24
    let mut pc: u32 = 0x825EA798;
    'dispatch: loop {
        match pc {
            0x825EA798 => {
    //   block [0x825EA798..0x825EA7B0)
	// 825EA798: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA79C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EA7A0: 394A554C  addi r10, r10, 0x554c
	ctx.r[10].s64 = ctx.r[10].s64 + 21836;
	// 825EA7A4: 816B56AC  lwz r11, 0x56ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22188 as u32) ) } as u64;
	// 825EA7A8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EA7AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA7B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA7B0 size=108
    let mut pc: u32 = 0x825EA7B0;
    'dispatch: loop {
        match pc {
            0x825EA7B0 => {
    //   block [0x825EA7B0..0x825EA81C)
	// 825EA7B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA7B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA7B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA7BC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EA7C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA7C4: 38EB554C  addi r7, r11, 0x554c
	ctx.r[7].s64 = ctx.r[11].s64 + 21836;
	// 825EA7C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA7CC: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 825EA7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA7D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA7D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA7E0: 386A9354  addi r3, r10, -0x6cac
	ctx.r[3].s64 = ctx.r[10].s64 + -27820;
	// 825EA7E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA7E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA7FC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EA800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA808: 4BE7C619  bl 0x82466e20
	ctx.lr = 0x825EA80C;
	sub_82466E20(ctx, base);
	// 825EA80C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA820 size=112
    let mut pc: u32 = 0x825EA820;
    'dispatch: loop {
        match pc {
            0x825EA820 => {
    //   block [0x825EA820..0x825EA890)
	// 825EA820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA82C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA830: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA834: 38AA9A74  addi r5, r10, -0x658c
	ctx.r[5].s64 = ctx.r[10].s64 + -25996;
	// 825EA838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA83C: 390B9C7C  addi r8, r11, -0x6384
	ctx.r[8].s64 = ctx.r[11].s64 + -25476;
	// 825EA840: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EA844: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 825EA848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA84C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EA854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA858: 386A9384  addi r3, r10, -0x6c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -27772;
	// 825EA85C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EA860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA86C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA874: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EA878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA87C: 4BE7C5A5  bl 0x82466e20
	ctx.lr = 0x825EA880;
	sub_82466E20(ctx, base);
	// 825EA880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA890 size=108
    let mut pc: u32 = 0x825EA890;
    'dispatch: loop {
        match pc {
            0x825EA890 => {
    //   block [0x825EA890..0x825EA8FC)
	// 825EA890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA89C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA8A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA8A4: 38EB9CAC  addi r7, r11, -0x6354
	ctx.r[7].s64 = ctx.r[11].s64 + -25428;
	// 825EA8A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA8AC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 825EA8B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA8B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA8B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA8BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA8C0: 386A93B4  addi r3, r10, -0x6c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -27724;
	// 825EA8C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA8C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA8D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA8D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA8DC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EA8E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA8E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA8E8: 4BE7C539  bl 0x82466e20
	ctx.lr = 0x825EA8EC;
	sub_82466E20(ctx, base);
	// 825EA8EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA8F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA8F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA8F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA900 size=108
    let mut pc: u32 = 0x825EA900;
    'dispatch: loop {
        match pc {
            0x825EA900 => {
    //   block [0x825EA900..0x825EA96C)
	// 825EA900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA90C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA914: 38EB9D08  addi r7, r11, -0x62f8
	ctx.r[7].s64 = ctx.r[11].s64 + -25336;
	// 825EA918: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EA91C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 825EA920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA924: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA930: 386A93E4  addi r3, r10, -0x6c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -27676;
	// 825EA934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA94C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EA950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA958: 4BE7C4C9  bl 0x82466e20
	ctx.lr = 0x825EA95C;
	sub_82466E20(ctx, base);
	// 825EA95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA970 size=108
    let mut pc: u32 = 0x825EA970;
    'dispatch: loop {
        match pc {
            0x825EA970 => {
    //   block [0x825EA970..0x825EA9DC)
	// 825EA970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA97C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA984: 38EB9E40  addi r7, r11, -0x61c0
	ctx.r[7].s64 = ctx.r[11].s64 + -25024;
	// 825EA988: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825EA98C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 825EA990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EA994: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EA998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EA99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EA9A0: 386A9414  addi r3, r10, -0x6bec
	ctx.r[3].s64 = ctx.r[10].s64 + -27628;
	// 825EA9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EA9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EA9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EA9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EA9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EA9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EA9BC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 825EA9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EA9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EA9C8: 4BE7C459  bl 0x82466e20
	ctx.lr = 0x825EA9CC;
	sub_82466E20(ctx, base);
	// 825EA9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EA9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EA9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EA9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EA9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EA9E0 size=108
    let mut pc: u32 = 0x825EA9E0;
    'dispatch: loop {
        match pc {
            0x825EA9E0 => {
    //   block [0x825EA9E0..0x825EAA4C)
	// 825EA9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EA9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EA9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EA9EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EA9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EA9F4: 38EB9F28  addi r7, r11, -0x60d8
	ctx.r[7].s64 = ctx.r[11].s64 + -24792;
	// 825EA9F8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825EA9FC: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 825EAA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAA04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAA08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAA10: 386A9444  addi r3, r10, -0x6bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -27580;
	// 825EAA14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAA2C: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 825EAA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAA34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAA38: 4BE7C3E9  bl 0x82466e20
	ctx.lr = 0x825EAA3C;
	sub_82466E20(ctx, base);
	// 825EAA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAA50 size=108
    let mut pc: u32 = 0x825EAA50;
    'dispatch: loop {
        match pc {
            0x825EAA50 => {
    //   block [0x825EAA50..0x825EAABC)
	// 825EAA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAA5C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAA64: 38EBA02C  addi r7, r11, -0x5fd4
	ctx.r[7].s64 = ctx.r[11].s64 + -24532;
	// 825EAA68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAA6C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 825EAA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAA74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAA80: 386A9474  addi r3, r10, -0x6b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -27532;
	// 825EAA84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAA9C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EAAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAAA8: 4BE7C379  bl 0x82466e20
	ctx.lr = 0x825EAAAC;
	sub_82466E20(ctx, base);
	// 825EAAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAAC0 size=108
    let mut pc: u32 = 0x825EAAC0;
    'dispatch: loop {
        match pc {
            0x825EAAC0 => {
    //   block [0x825EAAC0..0x825EAB2C)
	// 825EAAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAACC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAAD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAAD4: 38EBA068  addi r7, r11, -0x5f98
	ctx.r[7].s64 = ctx.r[11].s64 + -24472;
	// 825EAAD8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAADC: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 825EAAE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAAE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAAE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAAF0: 386A94A4  addi r3, r10, -0x6b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -27484;
	// 825EAAF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAAF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAB00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAB08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAB0C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EAB10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAB14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAB18: 4BE7C309  bl 0x82466e20
	ctx.lr = 0x825EAB1C;
	sub_82466E20(ctx, base);
	// 825EAB1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAB20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAB24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAB28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAB30 size=108
    let mut pc: u32 = 0x825EAB30;
    'dispatch: loop {
        match pc {
            0x825EAB30 => {
    //   block [0x825EAB30..0x825EAB9C)
	// 825EAB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAB3C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAB40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAB44: 38EBA0A8  addi r7, r11, -0x5f58
	ctx.r[7].s64 = ctx.r[11].s64 + -24408;
	// 825EAB48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAB4C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 825EAB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAB54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAB58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAB60: 386A94D4  addi r3, r10, -0x6b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -27436;
	// 825EAB64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAB68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAB74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAB7C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EAB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAB84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAB88: 4BE7C299  bl 0x82466e20
	ctx.lr = 0x825EAB8C;
	sub_82466E20(ctx, base);
	// 825EAB8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAB90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAB94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAB98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EABA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EABA0 size=108
    let mut pc: u32 = 0x825EABA0;
    'dispatch: loop {
        match pc {
            0x825EABA0 => {
    //   block [0x825EABA0..0x825EAC0C)
	// 825EABA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EABA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EABA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EABAC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EABB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EABB4: 38EBA0E8  addi r7, r11, -0x5f18
	ctx.r[7].s64 = ctx.r[11].s64 + -24344;
	// 825EABB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EABBC: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 825EABC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EABC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EABC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EABCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EABD0: 386A9504  addi r3, r10, -0x6afc
	ctx.r[3].s64 = ctx.r[10].s64 + -27388;
	// 825EABD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EABD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EABDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EABE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EABE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EABE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EABEC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EABF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EABF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EABF8: 4BE7C229  bl 0x82466e20
	ctx.lr = 0x825EABFC;
	sub_82466E20(ctx, base);
	// 825EABFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAC10 size=108
    let mut pc: u32 = 0x825EAC10;
    'dispatch: loop {
        match pc {
            0x825EAC10 => {
    //   block [0x825EAC10..0x825EAC7C)
	// 825EAC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAC1C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAC24: 38EBA118  addi r7, r11, -0x5ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -24296;
	// 825EAC28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAC2C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 825EAC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAC38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAC40: 386A9534  addi r3, r10, -0x6acc
	ctx.r[3].s64 = ctx.r[10].s64 + -27340;
	// 825EAC44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAC5C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EAC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAC68: 4BE7C1B9  bl 0x82466e20
	ctx.lr = 0x825EAC6C;
	sub_82466E20(ctx, base);
	// 825EAC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAC80 size=108
    let mut pc: u32 = 0x825EAC80;
    'dispatch: loop {
        match pc {
            0x825EAC80 => {
    //   block [0x825EAC80..0x825EACEC)
	// 825EAC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAC8C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAC90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAC94: 38EBA168  addi r7, r11, -0x5e98
	ctx.r[7].s64 = ctx.r[11].s64 + -24216;
	// 825EAC98: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 825EAC9C: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 825EACA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EACA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EACA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EACAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EACB0: 386A9564  addi r3, r10, -0x6a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -27292;
	// 825EACB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EACB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EACBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EACC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EACC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EACC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EACCC: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 825EACD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EACD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EACD8: 4BE7C149  bl 0x82466e20
	ctx.lr = 0x825EACDC;
	sub_82466E20(ctx, base);
	// 825EACDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EACE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EACE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EACE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EACF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EACF0 size=108
    let mut pc: u32 = 0x825EACF0;
    'dispatch: loop {
        match pc {
            0x825EACF0 => {
    //   block [0x825EACF0..0x825EAD5C)
	// 825EACF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EACF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EACF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EACFC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAD04: 38EBA254  addi r7, r11, -0x5dac
	ctx.r[7].s64 = ctx.r[11].s64 + -23980;
	// 825EAD08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAD0C: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 825EAD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAD14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAD18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAD1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAD20: 386A9594  addi r3, r10, -0x6a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -27244;
	// 825EAD24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAD28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAD3C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EAD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAD44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAD48: 4BE7C0D9  bl 0x82466e20
	ctx.lr = 0x825EAD4C;
	sub_82466E20(ctx, base);
	// 825EAD4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAD50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAD54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAD58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAD60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAD60 size=108
    let mut pc: u32 = 0x825EAD60;
    'dispatch: loop {
        match pc {
            0x825EAD60 => {
    //   block [0x825EAD60..0x825EADCC)
	// 825EAD60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAD64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAD68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAD6C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAD74: 38EBA298  addi r7, r11, -0x5d68
	ctx.r[7].s64 = ctx.r[11].s64 + -23912;
	// 825EAD78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EAD7C: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 825EAD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAD84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAD88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAD90: 386A95C4  addi r3, r10, -0x6a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -27196;
	// 825EAD94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EADA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EADA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EADA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EADAC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 825EADB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EADB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EADB8: 4BE7C069  bl 0x82466e20
	ctx.lr = 0x825EADBC;
	sub_82466E20(ctx, base);
	// 825EADBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EADC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EADC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EADC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EADD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EADD0 size=108
    let mut pc: u32 = 0x825EADD0;
    'dispatch: loop {
        match pc {
            0x825EADD0 => {
    //   block [0x825EADD0..0x825EAE3C)
	// 825EADD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EADD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EADD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EADDC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EADE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EADE4: 38EBA2B0  addi r7, r11, -0x5d50
	ctx.r[7].s64 = ctx.r[11].s64 + -23888;
	// 825EADE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EADEC: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 825EADF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EADF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EADF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EADFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAE00: 386A95F4  addi r3, r10, -0x6a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -27148;
	// 825EAE04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAE08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAE10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAE18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAE1C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EAE20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAE24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAE28: 4BE7BFF9  bl 0x82466e20
	ctx.lr = 0x825EAE2C;
	sub_82466E20(ctx, base);
	// 825EAE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAE40 size=108
    let mut pc: u32 = 0x825EAE40;
    'dispatch: loop {
        match pc {
            0x825EAE40 => {
    //   block [0x825EAE40..0x825EAEAC)
	// 825EAE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAE4C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAE50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAE54: 38EBA320  addi r7, r11, -0x5ce0
	ctx.r[7].s64 = ctx.r[11].s64 + -23776;
	// 825EAE58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EAE5C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 825EAE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAE64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAE68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAE70: 386A9624  addi r3, r10, -0x69dc
	ctx.r[3].s64 = ctx.r[10].s64 + -27100;
	// 825EAE74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAE84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAE8C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EAE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAE94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAE98: 4BE7BF89  bl 0x82466e20
	ctx.lr = 0x825EAE9C;
	sub_82466E20(ctx, base);
	// 825EAE9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAEA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAEA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAEA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAEB0 size=112
    let mut pc: u32 = 0x825EAEB0;
    'dispatch: loop {
        match pc {
            0x825EAEB0 => {
    //   block [0x825EAEB0..0x825EAF20)
	// 825EAEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAEBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAEC0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAEC4: 38AA9A74  addi r5, r10, -0x658c
	ctx.r[5].s64 = ctx.r[10].s64 + -25996;
	// 825EAEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAECC: 390BA374  addi r8, r11, -0x5c8c
	ctx.r[8].s64 = ctx.r[11].s64 + -23692;
	// 825EAED0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EAED4: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 825EAED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAEDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EAEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAEE8: 386A9654  addi r3, r10, -0x69ac
	ctx.r[3].s64 = ctx.r[10].s64 + -27052;
	// 825EAEEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EAEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAF04: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 825EAF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAF0C: 4BE7BF15  bl 0x82466e20
	ctx.lr = 0x825EAF10;
	sub_82466E20(ctx, base);
	// 825EAF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAF20 size=108
    let mut pc: u32 = 0x825EAF20;
    'dispatch: loop {
        match pc {
            0x825EAF20 => {
    //   block [0x825EAF20..0x825EAF8C)
	// 825EAF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAF2C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAF30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAF34: 396BA460  addi r11, r11, -0x5ba0
	ctx.r[11].s64 = ctx.r[11].s64 + -23456;
	// 825EAF38: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 825EAF3C: 390B0138  addi r8, r11, 0x138
	ctx.r[8].s64 = ctx.r[11].s64 + 312;
	// 825EAF40: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 825EAF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAF48: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAF4C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EAF50: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EAF54: 38C00080  li r6, 0x80
	ctx.r[6].s64 = 128;
	// 825EAF58: 386A9684  addi r3, r10, -0x697c
	ctx.r[3].s64 = ctx.r[10].s64 + -27004;
	// 825EAF5C: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 825EAF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAF64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAF68: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825EAF6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAF70: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825EAF74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAF78: 4BE7BEA9  bl 0x82466e20
	ctx.lr = 0x825EAF7C;
	sub_82466E20(ctx, base);
	// 825EAF7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAF80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAF84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EAF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EAF90 size=108
    let mut pc: u32 = 0x825EAF90;
    'dispatch: loop {
        match pc {
            0x825EAF90 => {
    //   block [0x825EAF90..0x825EAFFC)
	// 825EAF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EAF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EAF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EAF9C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EAFA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EAFA4: 38EBA610  addi r7, r11, -0x59f0
	ctx.r[7].s64 = ctx.r[11].s64 + -23024;
	// 825EAFA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EAFAC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 825EAFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EAFB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EAFB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EAFBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EAFC0: 386A96B4  addi r3, r10, -0x694c
	ctx.r[3].s64 = ctx.r[10].s64 + -26956;
	// 825EAFC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EAFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EAFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EAFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EAFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EAFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EAFDC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EAFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EAFE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EAFE8: 4BE7BE39  bl 0x82466e20
	ctx.lr = 0x825EAFEC;
	sub_82466E20(ctx, base);
	// 825EAFEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EAFF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EAFF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EAFF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB000 size=108
    let mut pc: u32 = 0x825EB000;
    'dispatch: loop {
        match pc {
            0x825EB000 => {
    //   block [0x825EB000..0x825EB06C)
	// 825EB000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB00C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB014: 38EBA66C  addi r7, r11, -0x5994
	ctx.r[7].s64 = ctx.r[11].s64 + -22932;
	// 825EB018: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EB01C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 825EB020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB024: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB02C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB030: 386A96E4  addi r3, r10, -0x691c
	ctx.r[3].s64 = ctx.r[10].s64 + -26908;
	// 825EB034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB03C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB04C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EB050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB058: 4BE7BDC9  bl 0x82466e20
	ctx.lr = 0x825EB05C;
	sub_82466E20(ctx, base);
	// 825EB05C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB070 size=108
    let mut pc: u32 = 0x825EB070;
    'dispatch: loop {
        match pc {
            0x825EB070 => {
    //   block [0x825EB070..0x825EB0DC)
	// 825EB070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB07C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB084: 392BA734  addi r9, r11, -0x58cc
	ctx.r[9].s64 = ctx.r[11].s64 + -22732;
	// 825EB088: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825EB08C: 39090014  addi r8, r9, 0x14
	ctx.r[8].s64 = ctx.r[9].s64 + 20;
	// 825EB090: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 825EB094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB098: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB09C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EB0A0: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EB0A4: 386A9714  addi r3, r10, -0x68ec
	ctx.r[3].s64 = ctx.r[10].s64 + -26860;
	// 825EB0A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB0AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EB0B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB0B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB0B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB0BC: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB0C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB0C4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB0C8: 4BE7BD59  bl 0x82466e20
	ctx.lr = 0x825EB0CC;
	sub_82466E20(ctx, base);
	// 825EB0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB0E0 size=108
    let mut pc: u32 = 0x825EB0E0;
    'dispatch: loop {
        match pc {
            0x825EB0E0 => {
    //   block [0x825EB0E0..0x825EB14C)
	// 825EB0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB0EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB0F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB0F4: 38EBA858  addi r7, r11, -0x57a8
	ctx.r[7].s64 = ctx.r[11].s64 + -22440;
	// 825EB0F8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 825EB0FC: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 825EB100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB104: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB108: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB110: 386A9744  addi r3, r10, -0x68bc
	ctx.r[3].s64 = ctx.r[10].s64 + -26812;
	// 825EB114: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB11C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB12C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EB130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB134: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB138: 4BE7BCE9  bl 0x82466e20
	ctx.lr = 0x825EB13C;
	sub_82466E20(ctx, base);
	// 825EB13C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB150 size=112
    let mut pc: u32 = 0x825EB150;
    'dispatch: loop {
        match pc {
            0x825EB150 => {
    //   block [0x825EB150..0x825EB1C0)
	// 825EB150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB15C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB160: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB164: 38AA96B4  addi r5, r10, -0x694c
	ctx.r[5].s64 = ctx.r[10].s64 + -26956;
	// 825EB168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB16C: 390BA988  addi r8, r11, -0x5678
	ctx.r[8].s64 = ctx.r[11].s64 + -22136;
	// 825EB170: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EB174: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 825EB178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB17C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB188: 386A9774  addi r3, r10, -0x688c
	ctx.r[3].s64 = ctx.r[10].s64 + -26764;
	// 825EB18C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EB190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB19C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB1A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB1A4: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 825EB1A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB1AC: 4BE7BC75  bl 0x82466e20
	ctx.lr = 0x825EB1B0;
	sub_82466E20(ctx, base);
	// 825EB1B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB1B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB1B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB1C0 size=108
    let mut pc: u32 = 0x825EB1C0;
    'dispatch: loop {
        match pc {
            0x825EB1C0 => {
    //   block [0x825EB1C0..0x825EB22C)
	// 825EB1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB1CC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB1D4: 38EBA9CC  addi r7, r11, -0x5634
	ctx.r[7].s64 = ctx.r[11].s64 + -22068;
	// 825EB1D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EB1DC: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 825EB1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB1E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB1E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB1F0: 386A97A4  addi r3, r10, -0x685c
	ctx.r[3].s64 = ctx.r[10].s64 + -26716;
	// 825EB1F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB1F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB1FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB20C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EB210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB218: 4BE7BC09  bl 0x82466e20
	ctx.lr = 0x825EB21C;
	sub_82466E20(ctx, base);
	// 825EB21C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB230 size=108
    let mut pc: u32 = 0x825EB230;
    'dispatch: loop {
        match pc {
            0x825EB230 => {
    //   block [0x825EB230..0x825EB29C)
	// 825EB230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB23C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB240: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB244: 38EBAA14  addi r7, r11, -0x55ec
	ctx.r[7].s64 = ctx.r[11].s64 + -21996;
	// 825EB248: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EB24C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 825EB250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB254: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB260: 386A97D4  addi r3, r10, -0x682c
	ctx.r[3].s64 = ctx.r[10].s64 + -26668;
	// 825EB264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB26C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB27C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EB280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB288: 4BE7BB99  bl 0x82466e20
	ctx.lr = 0x825EB28C;
	sub_82466E20(ctx, base);
	// 825EB28C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB2A0 size=108
    let mut pc: u32 = 0x825EB2A0;
    'dispatch: loop {
        match pc {
            0x825EB2A0 => {
    //   block [0x825EB2A0..0x825EB30C)
	// 825EB2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB2A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB2A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB2AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB2B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB2B4: 38EBAB7C  addi r7, r11, -0x5484
	ctx.r[7].s64 = ctx.r[11].s64 + -21636;
	// 825EB2B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EB2BC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 825EB2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB2C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB2C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB2D0: 386A9804  addi r3, r10, -0x67fc
	ctx.r[3].s64 = ctx.r[10].s64 + -26620;
	// 825EB2D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB2DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB2E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB2EC: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EB2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB2F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB2F8: 4BE7BB29  bl 0x82466e20
	ctx.lr = 0x825EB2FC;
	sub_82466E20(ctx, base);
	// 825EB2FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB310 size=112
    let mut pc: u32 = 0x825EB310;
    'dispatch: loop {
        match pc {
            0x825EB310 => {
    //   block [0x825EB310..0x825EB380)
	// 825EB310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB31C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB320: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB324: 392BAB68  addi r9, r11, -0x5498
	ctx.r[9].s64 = ctx.r[11].s64 + -21656;
	// 825EB328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB32C: 39090048  addi r8, r9, 0x48
	ctx.r[8].s64 = ctx.r[9].s64 + 72;
	// 825EB330: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825EB334: 38AA9A74  addi r5, r10, -0x658c
	ctx.r[5].s64 = ctx.r[10].s64 + -25996;
	// 825EB338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB33C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB340: 38C00070  li r6, 0x70
	ctx.r[6].s64 = 112;
	// 825EB344: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB348: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 825EB34C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB350: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EB354: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EB358: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EB35C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB360: 386B9834  addi r3, r11, -0x67cc
	ctx.r[3].s64 = ctx.r[11].s64 + -26572;
	// 825EB364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB368: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB36C: 4BE7BAB5  bl 0x82466e20
	ctx.lr = 0x825EB370;
	sub_82466E20(ctx, base);
	// 825EB370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB380 size=108
    let mut pc: u32 = 0x825EB380;
    'dispatch: loop {
        match pc {
            0x825EB380 => {
    //   block [0x825EB380..0x825EB3EC)
	// 825EB380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB38C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB394: 38EBACC8  addi r7, r11, -0x5338
	ctx.r[7].s64 = ctx.r[11].s64 + -21304;
	// 825EB398: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EB39C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 825EB3A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB3A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB3A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB3B0: 386A9864  addi r3, r10, -0x679c
	ctx.r[3].s64 = ctx.r[10].s64 + -26524;
	// 825EB3B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB3B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB3C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB3CC: 38C00060  li r6, 0x60
	ctx.r[6].s64 = 96;
	// 825EB3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB3D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB3D8: 4BE7BA49  bl 0x82466e20
	ctx.lr = 0x825EB3DC;
	sub_82466E20(ctx, base);
	// 825EB3DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB3E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB3E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB3F0 size=108
    let mut pc: u32 = 0x825EB3F0;
    'dispatch: loop {
        match pc {
            0x825EB3F0 => {
    //   block [0x825EB3F0..0x825EB45C)
	// 825EB3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB3F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB3F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB3FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB404: 38EBAD38  addi r7, r11, -0x52c8
	ctx.r[7].s64 = ctx.r[11].s64 + -21192;
	// 825EB408: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825EB40C: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 825EB410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB414: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB420: 386A9894  addi r3, r10, -0x676c
	ctx.r[3].s64 = ctx.r[10].s64 + -26476;
	// 825EB424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB42C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB43C: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825EB440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB448: 4BE7B9D9  bl 0x82466e20
	ctx.lr = 0x825EB44C;
	sub_82466E20(ctx, base);
	// 825EB44C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB460 size=108
    let mut pc: u32 = 0x825EB460;
    'dispatch: loop {
        match pc {
            0x825EB460 => {
    //   block [0x825EB460..0x825EB4CC)
	// 825EB460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB46C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB470: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB474: 38EBAE28  addi r7, r11, -0x51d8
	ctx.r[7].s64 = ctx.r[11].s64 + -20952;
	// 825EB478: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825EB47C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 825EB480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB484: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB488: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB48C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB490: 386A98C4  addi r3, r10, -0x673c
	ctx.r[3].s64 = ctx.r[10].s64 + -26428;
	// 825EB494: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB4AC: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 825EB4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB4B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB4B8: 4BE7B969  bl 0x82466e20
	ctx.lr = 0x825EB4BC;
	sub_82466E20(ctx, base);
	// 825EB4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB4D0 size=108
    let mut pc: u32 = 0x825EB4D0;
    'dispatch: loop {
        match pc {
            0x825EB4D0 => {
    //   block [0x825EB4D0..0x825EB53C)
	// 825EB4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB4DC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB4E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB4E4: 38EBAEE8  addi r7, r11, -0x5118
	ctx.r[7].s64 = ctx.r[11].s64 + -20760;
	// 825EB4E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EB4EC: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 825EB4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB4F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB4F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB500: 386A98F4  addi r3, r10, -0x670c
	ctx.r[3].s64 = ctx.r[10].s64 + -26380;
	// 825EB504: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB51C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 825EB520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB524: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB528: 4BE7B8F9  bl 0x82466e20
	ctx.lr = 0x825EB52C;
	sub_82466E20(ctx, base);
	// 825EB52C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB540 size=108
    let mut pc: u32 = 0x825EB540;
    'dispatch: loop {
        match pc {
            0x825EB540 => {
    //   block [0x825EB540..0x825EB5AC)
	// 825EB540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB54C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB554: 38EBAF24  addi r7, r11, -0x50dc
	ctx.r[7].s64 = ctx.r[11].s64 + -20700;
	// 825EB558: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EB55C: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 825EB560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB564: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB568: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB570: 386A9924  addi r3, r10, -0x66dc
	ctx.r[3].s64 = ctx.r[10].s64 + -26332;
	// 825EB574: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB58C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EB590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB598: 4BE7B889  bl 0x82466e20
	ctx.lr = 0x825EB59C;
	sub_82466E20(ctx, base);
	// 825EB59C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB5A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB5A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB5A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EB5B0 size=24
    let mut pc: u32 = 0x825EB5B0;
    'dispatch: loop {
        match pc {
            0x825EB5B0 => {
    //   block [0x825EB5B0..0x825EB5C8)
	// 825EB5B0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB5B4: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EB5B8: 394A5790  addi r10, r10, 0x5790
	ctx.r[10].s64 = ctx.r[10].s64 + 22416;
	// 825EB5BC: 816B5778  lwz r11, 0x5778(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22392 as u32) ) } as u64;
	// 825EB5C0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EB5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB5C8 size=112
    let mut pc: u32 = 0x825EB5C8;
    'dispatch: loop {
        match pc {
            0x825EB5C8 => {
    //   block [0x825EB5C8..0x825EB638)
	// 825EB5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB5D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB5D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB5D8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB5DC: 392AAFF0  addi r9, r10, -0x5010
	ctx.r[9].s64 = ctx.r[10].s64 + -20496;
	// 825EB5E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB5E4: 390B5790  addi r8, r11, 0x5790
	ctx.r[8].s64 = ctx.r[11].s64 + 22416;
	// 825EB5E8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825EB5EC: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 825EB5F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB5F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB5F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB5FC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 825EB600: 386A9954  addi r3, r10, -0x66ac
	ctx.r[3].s64 = ctx.r[10].s64 + -26284;
	// 825EB604: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EB608: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EB60C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB61C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB624: 4BE7B7FD  bl 0x82466e20
	ctx.lr = 0x825EB628;
	sub_82466E20(ctx, base);
	// 825EB628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EB638 size=24
    let mut pc: u32 = 0x825EB638;
    'dispatch: loop {
        match pc {
            0x825EB638 => {
    //   block [0x825EB638..0x825EB650)
	// 825EB638: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB63C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EB640: 394A57E8  addi r10, r10, 0x57e8
	ctx.r[10].s64 = ctx.r[10].s64 + 22504;
	// 825EB644: 816B56AC  lwz r11, 0x56ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22188 as u32) ) } as u64;
	// 825EB648: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EB64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB650 size=108
    let mut pc: u32 = 0x825EB650;
    'dispatch: loop {
        match pc {
            0x825EB650 => {
    //   block [0x825EB650..0x825EB6BC)
	// 825EB650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB65C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB664: 38EB57E8  addi r7, r11, 0x57e8
	ctx.r[7].s64 = ctx.r[11].s64 + 22504;
	// 825EB668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EB66C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 825EB670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB674: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB680: 386A9984  addi r3, r10, -0x667c
	ctx.r[3].s64 = ctx.r[10].s64 + -26236;
	// 825EB684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB69C: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 825EB6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB6A8: 4BE7B779  bl 0x82466e20
	ctx.lr = 0x825EB6AC;
	sub_82466E20(ctx, base);
	// 825EB6AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EB6C0 size=24
    let mut pc: u32 = 0x825EB6C0;
    'dispatch: loop {
        match pc {
            0x825EB6C0 => {
    //   block [0x825EB6C0..0x825EB6D8)
	// 825EB6C0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB6C4: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EB6C8: 394A5830  addi r10, r10, 0x5830
	ctx.r[10].s64 = ctx.r[10].s64 + 22576;
	// 825EB6CC: 816B5818  lwz r11, 0x5818(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22552 as u32) ) } as u64;
	// 825EB6D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825EB6D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB6D8 size=112
    let mut pc: u32 = 0x825EB6D8;
    'dispatch: loop {
        match pc {
            0x825EB6D8 => {
    //   block [0x825EB6D8..0x825EB748)
	// 825EB6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB6DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB6E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB6E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB6E8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB6EC: 392AB084  addi r9, r10, -0x4f7c
	ctx.r[9].s64 = ctx.r[10].s64 + -20348;
	// 825EB6F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB6F4: 390B5830  addi r8, r11, 0x5830
	ctx.r[8].s64 = ctx.r[11].s64 + 22576;
	// 825EB6F8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EB6FC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 825EB700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB704: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB70C: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 825EB710: 386A99B4  addi r3, r10, -0x664c
	ctx.r[3].s64 = ctx.r[10].s64 + -26188;
	// 825EB714: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EB718: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EB71C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB72C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB734: 4BE7B6ED  bl 0x82466e20
	ctx.lr = 0x825EB738;
	sub_82466E20(ctx, base);
	// 825EB738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB73C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB748 size=108
    let mut pc: u32 = 0x825EB748;
    'dispatch: loop {
        match pc {
            0x825EB748 => {
    //   block [0x825EB748..0x825EB7B4)
	// 825EB748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB74C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB754: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB75C: 38EBB0E8  addi r7, r11, -0x4f18
	ctx.r[7].s64 = ctx.r[11].s64 + -20248;
	// 825EB760: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EB764: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 825EB768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB76C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB770: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB778: 386A99E4  addi r3, r10, -0x661c
	ctx.r[3].s64 = ctx.r[10].s64 + -26140;
	// 825EB77C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB780: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB788: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB790: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB794: 38C00018  li r6, 0x18
	ctx.r[6].s64 = 24;
	// 825EB798: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB79C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB7A0: 4BE7B681  bl 0x82466e20
	ctx.lr = 0x825EB7A4;
	sub_82466E20(ctx, base);
	// 825EB7A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB7B8 size=108
    let mut pc: u32 = 0x825EB7B8;
    'dispatch: loop {
        match pc {
            0x825EB7B8 => {
    //   block [0x825EB7B8..0x825EB824)
	// 825EB7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB7C4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB7C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB7CC: 38EBB1B8  addi r7, r11, -0x4e48
	ctx.r[7].s64 = ctx.r[11].s64 + -20040;
	// 825EB7D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EB7D4: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 825EB7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB7DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB7E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB7E8: 386A9A14  addi r3, r10, -0x65ec
	ctx.r[3].s64 = ctx.r[10].s64 + -26092;
	// 825EB7EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB7F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB7F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB804: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EB808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB810: 4BE7B611  bl 0x82466e20
	ctx.lr = 0x825EB814;
	sub_82466E20(ctx, base);
	// 825EB814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB81C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB828 size=112
    let mut pc: u32 = 0x825EB828;
    'dispatch: loop {
        match pc {
            0x825EB828 => {
    //   block [0x825EB828..0x825EB898)
	// 825EB828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB834: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB838: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB83C: 38AA9A74  addi r5, r10, -0x658c
	ctx.r[5].s64 = ctx.r[10].s64 + -25996;
	// 825EB840: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB844: 390BB1E8  addi r8, r11, -0x4e18
	ctx.r[8].s64 = ctx.r[11].s64 + -19992;
	// 825EB848: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825EB84C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 825EB850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB854: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB860: 386A9A44  addi r3, r10, -0x65bc
	ctx.r[3].s64 = ctx.r[10].s64 + -26044;
	// 825EB864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EB868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB86C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB87C: 38C00034  li r6, 0x34
	ctx.r[6].s64 = 52;
	// 825EB880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB884: 4BE7B59D  bl 0x82466e20
	ctx.lr = 0x825EB888;
	sub_82466E20(ctx, base);
	// 825EB888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB88C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB898 size=108
    let mut pc: u32 = 0x825EB898;
    'dispatch: loop {
        match pc {
            0x825EB898 => {
    //   block [0x825EB898..0x825EB904)
	// 825EB898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB8A4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB8A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB8AC: 38EBB2C0  addi r7, r11, -0x4d40
	ctx.r[7].s64 = ctx.r[11].s64 + -19776;
	// 825EB8B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EB8B4: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 825EB8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB8BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB8C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB8C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB8C8: 386A9A74  addi r3, r10, -0x658c
	ctx.r[3].s64 = ctx.r[10].s64 + -25996;
	// 825EB8CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB8D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB8D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB8E4: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 825EB8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB8EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB8F0: 4BE7B531  bl 0x82466e20
	ctx.lr = 0x825EB8F4;
	sub_82466E20(ctx, base);
	// 825EB8F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB8F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB8FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB908 size=108
    let mut pc: u32 = 0x825EB908;
    'dispatch: loop {
        match pc {
            0x825EB908 => {
    //   block [0x825EB908..0x825EB974)
	// 825EB908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB914: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EB918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB91C: 38EBB2F0  addi r7, r11, -0x4d10
	ctx.r[7].s64 = ctx.r[11].s64 + -19728;
	// 825EB920: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EB924: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 825EB928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB92C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EB934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EB938: 386A9AA4  addi r3, r10, -0x655c
	ctx.r[3].s64 = ctx.r[10].s64 + -25948;
	// 825EB93C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EB940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EB944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB954: 38C00030  li r6, 0x30
	ctx.r[6].s64 = 48;
	// 825EB958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB95C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB960: 4BE7B4C1  bl 0x82466e20
	ctx.lr = 0x825EB964;
	sub_82466E20(ctx, base);
	// 825EB964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EB978 size=24
    let mut pc: u32 = 0x825EB978;
    'dispatch: loop {
        match pc {
            0x825EB978 => {
    //   block [0x825EB978..0x825EB990)
	// 825EB978: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB97C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EB980: 394A5910  addi r10, r10, 0x5910
	ctx.r[10].s64 = ctx.r[10].s64 + 22800;
	// 825EB984: 816B58F8  lwz r11, 0x58f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(22776 as u32) ) } as u64;
	// 825EB988: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825EB98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EB990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EB990 size=112
    let mut pc: u32 = 0x825EB990;
    'dispatch: loop {
        match pc {
            0x825EB990 => {
    //   block [0x825EB990..0x825EBA00)
	// 825EB990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EB994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EB998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EB99C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB9A0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EB9A4: 392AB43C  addi r9, r10, -0x4bc4
	ctx.r[9].s64 = ctx.r[10].s64 + -19396;
	// 825EB9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EB9AC: 390B5910  addi r8, r11, 0x5910
	ctx.r[8].s64 = ctx.r[11].s64 + 22800;
	// 825EB9B0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EB9B4: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 825EB9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EB9BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EB9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EB9C4: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 825EB9C8: 386A9AD4  addi r3, r10, -0x652c
	ctx.r[3].s64 = ctx.r[10].s64 + -25900;
	// 825EB9CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EB9D0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EB9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EB9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EB9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EB9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EB9E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EB9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EB9EC: 4BE7B435  bl 0x82466e20
	ctx.lr = 0x825EB9F0;
	sub_82466E20(ctx, base);
	// 825EB9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EB9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EB9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EB9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBA00 size=108
    let mut pc: u32 = 0x825EBA00;
    'dispatch: loop {
        match pc {
            0x825EBA00 => {
    //   block [0x825EBA00..0x825EBA6C)
	// 825EBA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBA0C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBA10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBA14: 38EB5988  addi r7, r11, 0x5988
	ctx.r[7].s64 = ctx.r[11].s64 + 22920;
	// 825EBA18: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EBA1C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 825EBA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBA24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBA28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBA2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBA30: 386A9B04  addi r3, r10, -0x64fc
	ctx.r[3].s64 = ctx.r[10].s64 + -25852;
	// 825EBA34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBA3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBA4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBA54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBA58: 4BE7B3C9  bl 0x82466e20
	ctx.lr = 0x825EBA5C;
	sub_82466E20(ctx, base);
	// 825EBA5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBA60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBA64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EBA70 size=24
    let mut pc: u32 = 0x825EBA70;
    'dispatch: loop {
        match pc {
            0x825EBA70 => {
    //   block [0x825EBA70..0x825EBA88)
	// 825EBA70: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBA74: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EBA78: 394ADBC8  addi r10, r10, -0x2438
	ctx.r[10].s64 = ctx.r[10].s64 + -9272;
	// 825EBA7C: 816B5A00  lwz r11, 0x5a00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23040 as u32) ) } as u64;
	// 825EBA80: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 825EBA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBA88 size=112
    let mut pc: u32 = 0x825EBA88;
    'dispatch: loop {
        match pc {
            0x825EBA88 => {
    //   block [0x825EBA88..0x825EBAF8)
	// 825EBA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBA94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBA98: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EBA9C: 392A0DCC  addi r9, r10, 0xdcc
	ctx.r[9].s64 = ctx.r[10].s64 + 3532;
	// 825EBAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBAA4: 390BDBC8  addi r8, r11, -0x2438
	ctx.r[8].s64 = ctx.r[11].s64 + -9272;
	// 825EBAA8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825EBAAC: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 825EBAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBAB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBAB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBAC0: 386A9B34  addi r3, r10, -0x64cc
	ctx.r[3].s64 = ctx.r[10].s64 + -25804;
	// 825EBAC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EBAC8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EBACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBADC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBAE4: 4BE7B33D  bl 0x82466e20
	ctx.lr = 0x825EBAE8;
	sub_82466E20(ctx, base);
	// 825EBAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBAF8 size=108
    let mut pc: u32 = 0x825EBAF8;
    'dispatch: loop {
        match pc {
            0x825EBAF8 => {
    //   block [0x825EBAF8..0x825EBB64)
	// 825EBAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBB04: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBB0C: 38EB5A04  addi r7, r11, 0x5a04
	ctx.r[7].s64 = ctx.r[11].s64 + 23044;
	// 825EBB10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EBB14: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 825EBB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBB1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBB20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBB28: 386A9B64  addi r3, r10, -0x649c
	ctx.r[3].s64 = ctx.r[10].s64 + -25756;
	// 825EBB2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBB34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBB4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBB50: 4BE7B2D1  bl 0x82466e20
	ctx.lr = 0x825EBB54;
	sub_82466E20(ctx, base);
	// 825EBB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBB68 size=108
    let mut pc: u32 = 0x825EBB68;
    'dispatch: loop {
        match pc {
            0x825EBB68 => {
    //   block [0x825EBB68..0x825EBBD4)
	// 825EBB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBB74: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBB7C: 38EB5A34  addi r7, r11, 0x5a34
	ctx.r[7].s64 = ctx.r[11].s64 + 23092;
	// 825EBB80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EBB84: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 825EBB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBB8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBB90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBB98: 386A9B94  addi r3, r10, -0x646c
	ctx.r[3].s64 = ctx.r[10].s64 + -25708;
	// 825EBB9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBBA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBBBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBBC0: 4BE7B261  bl 0x82466e20
	ctx.lr = 0x825EBBC4;
	sub_82466E20(ctx, base);
	// 825EBBC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBBC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBBCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBBD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EBBD8 size=24
    let mut pc: u32 = 0x825EBBD8;
    'dispatch: loop {
        match pc {
            0x825EBBD8 => {
    //   block [0x825EBBD8..0x825EBBF0)
	// 825EBBD8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBBDC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EBBE0: 394ADC10  addi r10, r10, -0x23f0
	ctx.r[10].s64 = ctx.r[10].s64 + -9200;
	// 825EBBE4: 816B5A64  lwz r11, 0x5a64(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(23140 as u32) ) } as u64;
	// 825EBBE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825EBBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBBF0 size=116
    let mut pc: u32 = 0x825EBBF0;
    'dispatch: loop {
        match pc {
            0x825EBBF0 => {
    //   block [0x825EBBF0..0x825EBC64)
	// 825EBBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBBFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EBC00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBC04: 390BDC10  addi r8, r11, -0x23f0
	ctx.r[8].s64 = ctx.r[11].s64 + -9200;
	// 825EBC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBC0C: 392A0E00  addi r9, r10, 0xe00
	ctx.r[9].s64 = ctx.r[10].s64 + 3584;
	// 825EBC10: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBC14: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EBC18: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EBC1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBC24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBC2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBC34: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EBC38: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 825EBC3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EBC40: 386B9BC4  addi r3, r11, -0x643c
	ctx.r[3].s64 = ctx.r[11].s64 + -25660;
	// 825EBC44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EBC48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBC50: 4BE7B1D1  bl 0x82466e20
	ctx.lr = 0x825EBC54;
	sub_82466E20(ctx, base);
	// 825EBC54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBC68 size=108
    let mut pc: u32 = 0x825EBC68;
    'dispatch: loop {
        match pc {
            0x825EBC68 => {
    //   block [0x825EBC68..0x825EBCD4)
	// 825EBC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBC74: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBC78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBC7C: 38EB5A68  addi r7, r11, 0x5a68
	ctx.r[7].s64 = ctx.r[11].s64 + 23144;
	// 825EBC80: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EBC84: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 825EBC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBC8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBC90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBC94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBC98: 386A9BF4  addi r3, r10, -0x640c
	ctx.r[3].s64 = ctx.r[10].s64 + -25612;
	// 825EBC9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBCA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBCA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBCAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBCB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBCB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBCB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBCBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBCC0: 4BE7B161  bl 0x82466e20
	ctx.lr = 0x825EBCC4;
	sub_82466E20(ctx, base);
	// 825EBCC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBCC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBCCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBCD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBCD8 size=112
    let mut pc: u32 = 0x825EBCD8;
    'dispatch: loop {
        match pc {
            0x825EBCD8 => {
    //   block [0x825EBCD8..0x825EBD48)
	// 825EBCD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBCDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBCE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBCE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBCE8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBCEC: 38AA9BC4  addi r5, r10, -0x643c
	ctx.r[5].s64 = ctx.r[10].s64 + -25660;
	// 825EBCF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBCF4: 390B5AF8  addi r8, r11, 0x5af8
	ctx.r[8].s64 = ctx.r[11].s64 + 23288;
	// 825EBCF8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825EBCFC: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 825EBD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBD04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBD08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBD10: 386A9C24  addi r3, r10, -0x63dc
	ctx.r[3].s64 = ctx.r[10].s64 + -25564;
	// 825EBD14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EBD18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBD1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBD24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBD34: 4BE7B0ED  bl 0x82466e20
	ctx.lr = 0x825EBD38;
	sub_82466E20(ctx, base);
	// 825EBD38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBD3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBD40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBD44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBD48 size=112
    let mut pc: u32 = 0x825EBD48;
    'dispatch: loop {
        match pc {
            0x825EBD48 => {
    //   block [0x825EBD48..0x825EBDB8)
	// 825EBD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBD4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBD54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBD58: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBD5C: 38AA9BC4  addi r5, r10, -0x643c
	ctx.r[5].s64 = ctx.r[10].s64 + -25660;
	// 825EBD60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBD64: 390B5C18  addi r8, r11, 0x5c18
	ctx.r[8].s64 = ctx.r[11].s64 + 23576;
	// 825EBD68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EBD6C: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 825EBD70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBD74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBD78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBD7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBD80: 386A9C54  addi r3, r10, -0x63ac
	ctx.r[3].s64 = ctx.r[10].s64 + -25516;
	// 825EBD84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EBD88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBD90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBD94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBD98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBDA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBDA4: 4BE7B07D  bl 0x82466e20
	ctx.lr = 0x825EBDA8;
	sub_82466E20(ctx, base);
	// 825EBDA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBDAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBDB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBDB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBDB8 size=108
    let mut pc: u32 = 0x825EBDB8;
    'dispatch: loop {
        match pc {
            0x825EBDB8 => {
    //   block [0x825EBDB8..0x825EBE24)
	// 825EBDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBDC4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBDC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBDCC: 38EB5C30  addi r7, r11, 0x5c30
	ctx.r[7].s64 = ctx.r[11].s64 + 23600;
	// 825EBDD0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EBDD4: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 825EBDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBDDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBDE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBDE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBDE8: 386A9C84  addi r3, r10, -0x637c
	ctx.r[3].s64 = ctx.r[10].s64 + -25468;
	// 825EBDEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBDF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBE0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBE10: 4BE7B011  bl 0x82466e20
	ctx.lr = 0x825EBE14;
	sub_82466E20(ctx, base);
	// 825EBE14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBE20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBE28 size=112
    let mut pc: u32 = 0x825EBE28;
    'dispatch: loop {
        match pc {
            0x825EBE28 => {
    //   block [0x825EBE28..0x825EBE98)
	// 825EBE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBE34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBE38: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBE3C: 38AA9BC4  addi r5, r10, -0x643c
	ctx.r[5].s64 = ctx.r[10].s64 + -25660;
	// 825EBE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBE44: 390B5CC0  addi r8, r11, 0x5cc0
	ctx.r[8].s64 = ctx.r[11].s64 + 23744;
	// 825EBE48: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 825EBE4C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 825EBE50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBE54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBE58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBE60: 386A9CB4  addi r3, r10, -0x634c
	ctx.r[3].s64 = ctx.r[10].s64 + -25420;
	// 825EBE64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EBE68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBE6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBE7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBE84: 4BE7AF9D  bl 0x82466e20
	ctx.lr = 0x825EBE88;
	sub_82466E20(ctx, base);
	// 825EBE88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBE8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBE90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBE94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBE98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBE98 size=108
    let mut pc: u32 = 0x825EBE98;
    'dispatch: loop {
        match pc {
            0x825EBE98 => {
    //   block [0x825EBE98..0x825EBF04)
	// 825EBE98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBE9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBEA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBEA4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBEA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBEAC: 38EB5DB0  addi r7, r11, 0x5db0
	ctx.r[7].s64 = ctx.r[11].s64 + 23984;
	// 825EBEB0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EBEB4: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 825EBEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBEBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBEC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBEC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBEC8: 386A9CE4  addi r3, r10, -0x631c
	ctx.r[3].s64 = ctx.r[10].s64 + -25372;
	// 825EBECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBEE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBEE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBEF0: 4BE7AF31  bl 0x82466e20
	ctx.lr = 0x825EBEF4;
	sub_82466E20(ctx, base);
	// 825EBEF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBEF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBEFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBF00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBF08 size=108
    let mut pc: u32 = 0x825EBF08;
    'dispatch: loop {
        match pc {
            0x825EBF08 => {
    //   block [0x825EBF08..0x825EBF74)
	// 825EBF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBF14: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBF18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBF1C: 38EB5DC8  addi r7, r11, 0x5dc8
	ctx.r[7].s64 = ctx.r[11].s64 + 24008;
	// 825EBF20: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EBF24: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 825EBF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBF2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBF30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EBF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EBF38: 386A9D14  addi r3, r10, -0x62ec
	ctx.r[3].s64 = ctx.r[10].s64 + -25324;
	// 825EBF3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EBF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EBF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBF4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBF5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EBF60: 4BE7AEC1  bl 0x82466e20
	ctx.lr = 0x825EBF64;
	sub_82466E20(ctx, base);
	// 825EBF64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBF70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBF78 size=116
    let mut pc: u32 = 0x825EBF78;
    'dispatch: loop {
        match pc {
            0x825EBF78 => {
    //   block [0x825EBF78..0x825EBFEC)
	// 825EBF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBF84: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EBF88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBF8C: 390B5E2C  addi r8, r11, 0x5e2c
	ctx.r[8].s64 = ctx.r[11].s64 + 24108;
	// 825EBF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EBF94: 392A0E2C  addi r9, r10, 0xe2c
	ctx.r[9].s64 = ctx.r[10].s64 + 3628;
	// 825EBF98: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EBF9C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825EBFA0: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EBFA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EBFA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EBFAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EBFB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EBFB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EBFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EBFBC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EBFC0: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 825EBFC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EBFC8: 386B9D44  addi r3, r11, -0x62bc
	ctx.r[3].s64 = ctx.r[11].s64 + -25276;
	// 825EBFCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EBFD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EBFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EBFD8: 4BE7AE49  bl 0x82466e20
	ctx.lr = 0x825EBFDC;
	sub_82466E20(ctx, base);
	// 825EBFDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EBFE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EBFE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EBFE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EBFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EBFF0 size=108
    let mut pc: u32 = 0x825EBFF0;
    'dispatch: loop {
        match pc {
            0x825EBFF0 => {
    //   block [0x825EBFF0..0x825EC05C)
	// 825EBFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EBFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EBFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EBFFC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC000: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC004: 38EB5E48  addi r7, r11, 0x5e48
	ctx.r[7].s64 = ctx.r[11].s64 + 24136;
	// 825EC008: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825EC00C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 825EC010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC014: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC018: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC01C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC020: 386A9D74  addi r3, r10, -0x628c
	ctx.r[3].s64 = ctx.r[10].s64 + -25228;
	// 825EC024: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC048: 4BE7ADD9  bl 0x82466e20
	ctx.lr = 0x825EC04C;
	sub_82466E20(ctx, base);
	// 825EC04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC060 size=108
    let mut pc: u32 = 0x825EC060;
    'dispatch: loop {
        match pc {
            0x825EC060 => {
    //   block [0x825EC060..0x825EC0CC)
	// 825EC060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC06C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC074: 38EB5E90  addi r7, r11, 0x5e90
	ctx.r[7].s64 = ctx.r[11].s64 + 24208;
	// 825EC078: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EC07C: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 825EC080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC084: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC088: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC08C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC090: 386A9DA4  addi r3, r10, -0x625c
	ctx.r[3].s64 = ctx.r[10].s64 + -25180;
	// 825EC094: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC09C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC0B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC0B8: 4BE7AD69  bl 0x82466e20
	ctx.lr = 0x825EC0BC;
	sub_82466E20(ctx, base);
	// 825EC0BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC0C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC0C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC0C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC0D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC0D0 size=108
    let mut pc: u32 = 0x825EC0D0;
    'dispatch: loop {
        match pc {
            0x825EC0D0 => {
    //   block [0x825EC0D0..0x825EC13C)
	// 825EC0D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC0D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC0D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC0DC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC0E4: 38EB5F20  addi r7, r11, 0x5f20
	ctx.r[7].s64 = ctx.r[11].s64 + 24352;
	// 825EC0E8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 825EC0EC: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 825EC0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC0F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC0F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC100: 386A9DD4  addi r3, r10, -0x622c
	ctx.r[3].s64 = ctx.r[10].s64 + -25132;
	// 825EC104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC10C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC128: 4BE7ACF9  bl 0x82466e20
	ctx.lr = 0x825EC12C;
	sub_82466E20(ctx, base);
	// 825EC12C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC140 size=100
    let mut pc: u32 = 0x825EC140;
    'dispatch: loop {
        match pc {
            0x825EC140 => {
    //   block [0x825EC140..0x825EC1A4)
	// 825EC140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC14C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC154: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC15C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC160: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 825EC164: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC174: 386A9E04  addi r3, r10, -0x61fc
	ctx.r[3].s64 = ctx.r[10].s64 + -25084;
	// 825EC178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC17C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC180: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EC184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC188: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EC18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC190: 4BE7AC91  bl 0x82466e20
	ctx.lr = 0x825EC194;
	sub_82466E20(ctx, base);
	// 825EC194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC1A8 size=112
    let mut pc: u32 = 0x825EC1A8;
    'dispatch: loop {
        match pc {
            0x825EC1A8 => {
    //   block [0x825EC1A8..0x825EC218)
	// 825EC1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC1B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC1B8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC1BC: 38AA9E04  addi r5, r10, -0x61fc
	ctx.r[5].s64 = ctx.r[10].s64 + -25084;
	// 825EC1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC1C4: 390B5FB0  addi r8, r11, 0x5fb0
	ctx.r[8].s64 = ctx.r[11].s64 + 24496;
	// 825EC1C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 825EC1CC: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 825EC1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC1D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC1E0: 386A9E34  addi r3, r10, -0x61cc
	ctx.r[3].s64 = ctx.r[10].s64 + -25036;
	// 825EC1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC204: 4BE7AC1D  bl 0x82466e20
	ctx.lr = 0x825EC208;
	sub_82466E20(ctx, base);
	// 825EC208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC218 size=108
    let mut pc: u32 = 0x825EC218;
    'dispatch: loop {
        match pc {
            0x825EC218 => {
    //   block [0x825EC218..0x825EC284)
	// 825EC218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC224: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC22C: 38EB6010  addi r7, r11, 0x6010
	ctx.r[7].s64 = ctx.r[11].s64 + 24592;
	// 825EC230: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EC234: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 825EC238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC23C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC240: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC248: 386A9E64  addi r3, r10, -0x619c
	ctx.r[3].s64 = ctx.r[10].s64 + -24988;
	// 825EC24C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC250: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC25C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC264: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC270: 4BE7ABB1  bl 0x82466e20
	ctx.lr = 0x825EC274;
	sub_82466E20(ctx, base);
	// 825EC274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC27C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC288 size=108
    let mut pc: u32 = 0x825EC288;
    'dispatch: loop {
        match pc {
            0x825EC288 => {
    //   block [0x825EC288..0x825EC2F4)
	// 825EC288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC294: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC298: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC29C: 38EB6040  addi r7, r11, 0x6040
	ctx.r[7].s64 = ctx.r[11].s64 + 24640;
	// 825EC2A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EC2A4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 825EC2A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC2AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC2B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC2B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC2B8: 386A9E94  addi r3, r10, -0x616c
	ctx.r[3].s64 = ctx.r[10].s64 + -24940;
	// 825EC2BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC2C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC2C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC2D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC2D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC2D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC2DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC2E0: 4BE7AB41  bl 0x82466e20
	ctx.lr = 0x825EC2E4;
	sub_82466E20(ctx, base);
	// 825EC2E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC2F8 size=108
    let mut pc: u32 = 0x825EC2F8;
    'dispatch: loop {
        match pc {
            0x825EC2F8 => {
    //   block [0x825EC2F8..0x825EC364)
	// 825EC2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC304: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC30C: 38EB60A0  addi r7, r11, 0x60a0
	ctx.r[7].s64 = ctx.r[11].s64 + 24736;
	// 825EC310: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EC314: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 825EC318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC31C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC328: 386A9EC4  addi r3, r10, -0x613c
	ctx.r[3].s64 = ctx.r[10].s64 + -24892;
	// 825EC32C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC350: 4BE7AAD1  bl 0x82466e20
	ctx.lr = 0x825EC354;
	sub_82466E20(ctx, base);
	// 825EC354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EC368 size=24
    let mut pc: u32 = 0x825EC368;
    'dispatch: loop {
        match pc {
            0x825EC368 => {
    //   block [0x825EC368..0x825EC380)
	// 825EC368: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC36C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EC370: 394ADC88  addi r10, r10, -0x2378
	ctx.r[10].s64 = ctx.r[10].s64 + -9080;
	// 825EC374: 816B5E44  lwz r11, 0x5e44(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24132 as u32) ) } as u64;
	// 825EC378: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 825EC37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC380 size=116
    let mut pc: u32 = 0x825EC380;
    'dispatch: loop {
        match pc {
            0x825EC380 => {
    //   block [0x825EC380..0x825EC3F4)
	// 825EC380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC38C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EC390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC394: 390BDC88  addi r8, r11, -0x2378
	ctx.r[8].s64 = ctx.r[11].s64 + -9080;
	// 825EC398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC39C: 392A0E60  addi r9, r10, 0xe60
	ctx.r[9].s64 = ctx.r[10].s64 + 3680;
	// 825EC3A0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC3A4: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 825EC3A8: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC3AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC3B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC3C4: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EC3C8: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 825EC3CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EC3D0: 386B9EF4  addi r3, r11, -0x610c
	ctx.r[3].s64 = ctx.r[11].s64 + -24844;
	// 825EC3D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EC3D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC3E0: 4BE7AA41  bl 0x82466e20
	ctx.lr = 0x825EC3E4;
	sub_82466E20(ctx, base);
	// 825EC3E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC3E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC3EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC3F8 size=112
    let mut pc: u32 = 0x825EC3F8;
    'dispatch: loop {
        match pc {
            0x825EC3F8 => {
    //   block [0x825EC3F8..0x825EC468)
	// 825EC3F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC3FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC404: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC408: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC40C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC414: 390B6100  addi r8, r11, 0x6100
	ctx.r[8].s64 = ctx.r[11].s64 + 24832;
	// 825EC418: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EC41C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 825EC420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC424: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC428: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC430: 386A9F24  addi r3, r10, -0x60dc
	ctx.r[3].s64 = ctx.r[10].s64 + -24796;
	// 825EC434: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC43C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC454: 4BE7A9CD  bl 0x82466e20
	ctx.lr = 0x825EC458;
	sub_82466E20(ctx, base);
	// 825EC458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC45C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC460: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC468 size=112
    let mut pc: u32 = 0x825EC468;
    'dispatch: loop {
        match pc {
            0x825EC468 => {
    //   block [0x825EC468..0x825EC4D8)
	// 825EC468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC474: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC478: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC47C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC484: 390B6148  addi r8, r11, 0x6148
	ctx.r[8].s64 = ctx.r[11].s64 + 24904;
	// 825EC488: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EC48C: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 825EC490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC494: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC49C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC4A0: 386A9F54  addi r3, r10, -0x60ac
	ctx.r[3].s64 = ctx.r[10].s64 + -24748;
	// 825EC4A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC4C4: 4BE7A95D  bl 0x82466e20
	ctx.lr = 0x825EC4C8;
	sub_82466E20(ctx, base);
	// 825EC4C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC4CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC4D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC4D8 size=112
    let mut pc: u32 = 0x825EC4D8;
    'dispatch: loop {
        match pc {
            0x825EC4D8 => {
    //   block [0x825EC4D8..0x825EC548)
	// 825EC4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC4DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC4E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC4E8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC4EC: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825EC4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC4F4: 390B6190  addi r8, r11, 0x6190
	ctx.r[8].s64 = ctx.r[11].s64 + 24976;
	// 825EC4F8: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 825EC4FC: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 825EC500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC504: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC508: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC510: 386A9F84  addi r3, r10, -0x607c
	ctx.r[3].s64 = ctx.r[10].s64 + -24700;
	// 825EC514: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC534: 4BE7A8ED  bl 0x82466e20
	ctx.lr = 0x825EC538;
	sub_82466E20(ctx, base);
	// 825EC538: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC53C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC548 size=108
    let mut pc: u32 = 0x825EC548;
    'dispatch: loop {
        match pc {
            0x825EC548 => {
    //   block [0x825EC548..0x825EC5B4)
	// 825EC548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC554: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC55C: 38EB6268  addi r7, r11, 0x6268
	ctx.r[7].s64 = ctx.r[11].s64 + 25192;
	// 825EC560: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EC564: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 825EC568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC56C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC578: 386A9FB4  addi r3, r10, -0x604c
	ctx.r[3].s64 = ctx.r[10].s64 + -24652;
	// 825EC57C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC59C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC5A0: 4BE7A881  bl 0x82466e20
	ctx.lr = 0x825EC5A4;
	sub_82466E20(ctx, base);
	// 825EC5A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC5B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC5B8 size=108
    let mut pc: u32 = 0x825EC5B8;
    'dispatch: loop {
        match pc {
            0x825EC5B8 => {
    //   block [0x825EC5B8..0x825EC624)
	// 825EC5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC5C4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC5CC: 38EB6298  addi r7, r11, 0x6298
	ctx.r[7].s64 = ctx.r[11].s64 + 25240;
	// 825EC5D0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825EC5D4: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 825EC5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC5DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC5E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC5E8: 386A9FE4  addi r3, r10, -0x601c
	ctx.r[3].s64 = ctx.r[10].s64 + -24604;
	// 825EC5EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC60C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC610: 4BE7A811  bl 0x82466e20
	ctx.lr = 0x825EC614;
	sub_82466E20(ctx, base);
	// 825EC614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC61C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC628 size=112
    let mut pc: u32 = 0x825EC628;
    'dispatch: loop {
        match pc {
            0x825EC628 => {
    //   block [0x825EC628..0x825EC698)
	// 825EC628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC634: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EC638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC63C: 392B0E94  addi r9, r11, 0xe94
	ctx.r[9].s64 = ctx.r[11].s64 + 3732;
	// 825EC640: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 825EC644: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825EC648: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC64C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 825EC650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC654: 396B6348  addi r11, r11, 0x6348
	ctx.r[11].s64 = ctx.r[11].s64 + 25416;
	// 825EC658: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825EC65C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC660: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825EC664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC668: 386AA014  addi r3, r10, -0x5fec
	ctx.r[3].s64 = ctx.r[10].s64 + -24556;
	// 825EC66C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EC670: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825EC674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC678: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825EC67C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC680: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EC684: 4BE7A79D  bl 0x82466e20
	ctx.lr = 0x825EC688;
	sub_82466E20(ctx, base);
	// 825EC688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC698 size=112
    let mut pc: u32 = 0x825EC698;
    'dispatch: loop {
        match pc {
            0x825EC698 => {
    //   block [0x825EC698..0x825EC708)
	// 825EC698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC6A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC6A8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC6AC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC6B4: 390B6498  addi r8, r11, 0x6498
	ctx.r[8].s64 = ctx.r[11].s64 + 25752;
	// 825EC6B8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825EC6BC: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 825EC6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC6C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC6C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC6D0: 386AA044  addi r3, r10, -0x5fbc
	ctx.r[3].s64 = ctx.r[10].s64 + -24508;
	// 825EC6D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC6E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC6F4: 4BE7A72D  bl 0x82466e20
	ctx.lr = 0x825EC6F8;
	sub_82466E20(ctx, base);
	// 825EC6F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC6FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC700: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC708 size=112
    let mut pc: u32 = 0x825EC708;
    'dispatch: loop {
        match pc {
            0x825EC708 => {
    //   block [0x825EC708..0x825EC778)
	// 825EC708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC714: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC718: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC71C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC724: 390B6540  addi r8, r11, 0x6540
	ctx.r[8].s64 = ctx.r[11].s64 + 25920;
	// 825EC728: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 825EC72C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 825EC730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC740: 386AA074  addi r3, r10, -0x5f8c
	ctx.r[3].s64 = ctx.r[10].s64 + -24460;
	// 825EC744: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC74C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC764: 4BE7A6BD  bl 0x82466e20
	ctx.lr = 0x825EC768;
	sub_82466E20(ctx, base);
	// 825EC768: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC778 size=112
    let mut pc: u32 = 0x825EC778;
    'dispatch: loop {
        match pc {
            0x825EC778 => {
    //   block [0x825EC778..0x825EC7E8)
	// 825EC778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC784: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC788: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC78C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EC790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC794: 390B65D0  addi r8, r11, 0x65d0
	ctx.r[8].s64 = ctx.r[11].s64 + 26064;
	// 825EC798: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825EC79C: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 825EC7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC7A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC7B0: 386AA0A4  addi r3, r10, -0x5f5c
	ctx.r[3].s64 = ctx.r[10].s64 + -24412;
	// 825EC7B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EC7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC7D4: 4BE7A64D  bl 0x82466e20
	ctx.lr = 0x825EC7D8;
	sub_82466E20(ctx, base);
	// 825EC7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC7E8 size=108
    let mut pc: u32 = 0x825EC7E8;
    'dispatch: loop {
        match pc {
            0x825EC7E8 => {
    //   block [0x825EC7E8..0x825EC854)
	// 825EC7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC7F4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC7FC: 38EB6648  addi r7, r11, 0x6648
	ctx.r[7].s64 = ctx.r[11].s64 + 26184;
	// 825EC800: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EC804: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 825EC808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC80C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC810: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC818: 386AA0D4  addi r3, r10, -0x5f2c
	ctx.r[3].s64 = ctx.r[10].s64 + -24364;
	// 825EC81C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC83C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC840: 4BE7A5E1  bl 0x82466e20
	ctx.lr = 0x825EC844;
	sub_82466E20(ctx, base);
	// 825EC844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC858 size=112
    let mut pc: u32 = 0x825EC858;
    'dispatch: loop {
        match pc {
            0x825EC858 => {
    //   block [0x825EC858..0x825EC8C8)
	// 825EC858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC864: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC868: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC86C: 392A0EF0  addi r9, r10, 0xef0
	ctx.r[9].s64 = ctx.r[10].s64 + 3824;
	// 825EC870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC874: 390B66C0  addi r8, r11, 0x66c0
	ctx.r[8].s64 = ctx.r[11].s64 + 26304;
	// 825EC878: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825EC87C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 825EC880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC884: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC890: 386AA104  addi r3, r10, -0x5efc
	ctx.r[3].s64 = ctx.r[10].s64 + -24316;
	// 825EC894: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EC898: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EC89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC8B4: 4BE7A56D  bl 0x82466e20
	ctx.lr = 0x825EC8B8;
	sub_82466E20(ctx, base);
	// 825EC8B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC8BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC8C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC8C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC8C8 size=100
    let mut pc: u32 = 0x825EC8C8;
    'dispatch: loop {
        match pc {
            0x825EC8C8 => {
    //   block [0x825EC8C8..0x825EC92C)
	// 825EC8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC8D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC8DC: 38AAA8E4  addi r5, r10, -0x571c
	ctx.r[5].s64 = ctx.r[10].s64 + -22300;
	// 825EC8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC8E8: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 825EC8EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC8FC: 386AA134  addi r3, r10, -0x5ecc
	ctx.r[3].s64 = ctx.r[10].s64 + -24268;
	// 825EC900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC904: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC908: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EC90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC910: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EC914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC918: 4BE7A509  bl 0x82466e20
	ctx.lr = 0x825EC91C;
	sub_82466E20(ctx, base);
	// 825EC91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC930 size=108
    let mut pc: u32 = 0x825EC930;
    'dispatch: loop {
        match pc {
            0x825EC930 => {
    //   block [0x825EC930..0x825EC99C)
	// 825EC930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC93C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC944: 38EB66F4  addi r7, r11, 0x66f4
	ctx.r[7].s64 = ctx.r[11].s64 + 26356;
	// 825EC948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EC94C: 388A2E80  addi r4, r10, 0x2e80
	ctx.r[4].s64 = ctx.r[10].s64 + 11904;
	// 825EC950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EC95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EC960: 386AA164  addi r3, r10, -0x5e9c
	ctx.r[3].s64 = ctx.r[10].s64 + -24220;
	// 825EC964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EC968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EC96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC988: 4BE7A499  bl 0x82466e20
	ctx.lr = 0x825EC98C;
	sub_82466E20(ctx, base);
	// 825EC98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EC990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EC994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EC998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EC9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EC9A0 size=112
    let mut pc: u32 = 0x825EC9A0;
    'dispatch: loop {
        match pc {
            0x825EC9A0 => {
    //   block [0x825EC9A0..0x825ECA10)
	// 825EC9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EC9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EC9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EC9AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC9B0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EC9B4: 392A0F68  addi r9, r10, 0xf68
	ctx.r[9].s64 = ctx.r[10].s64 + 3944;
	// 825EC9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EC9BC: 390B6728  addi r8, r11, 0x6728
	ctx.r[8].s64 = ctx.r[11].s64 + 26408;
	// 825EC9C0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EC9C4: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 825EC9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EC9CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EC9D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EC9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EC9D8: 386AA194  addi r3, r10, -0x5e6c
	ctx.r[3].s64 = ctx.r[10].s64 + -24172;
	// 825EC9DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EC9E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EC9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EC9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EC9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EC9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EC9F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EC9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EC9FC: 4BE7A425  bl 0x82466e20
	ctx.lr = 0x825ECA00;
	sub_82466E20(ctx, base);
	// 825ECA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECA10 size=112
    let mut pc: u32 = 0x825ECA10;
    'dispatch: loop {
        match pc {
            0x825ECA10 => {
    //   block [0x825ECA10..0x825ECA80)
	// 825ECA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECA1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECA20: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECA24: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECA28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECA2C: 390B67A0  addi r8, r11, 0x67a0
	ctx.r[8].s64 = ctx.r[11].s64 + 26528;
	// 825ECA30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825ECA34: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 825ECA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECA3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECA48: 386AA1C4  addi r3, r10, -0x5e3c
	ctx.r[3].s64 = ctx.r[10].s64 + -24124;
	// 825ECA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ECA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECA6C: 4BE7A3B5  bl 0x82466e20
	ctx.lr = 0x825ECA70;
	sub_82466E20(ctx, base);
	// 825ECA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECA80 size=116
    let mut pc: u32 = 0x825ECA80;
    'dispatch: loop {
        match pc {
            0x825ECA80 => {
    //   block [0x825ECA80..0x825ECAF4)
	// 825ECA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECA8C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825ECA90: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825ECA94: 390A67D0  addi r8, r10, 0x67d0
	ctx.r[8].s64 = ctx.r[10].s64 + 26576;
	// 825ECA98: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECA9C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ECAA0: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECAA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECAA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ECAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECAB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECAB4: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 825ECAB8: 396B0F7C  addi r11, r11, 0xf7c
	ctx.r[11].s64 = ctx.r[11].s64 + 3964;
	// 825ECABC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECAC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECAC4: 386AA1F4  addi r3, r10, -0x5e0c
	ctx.r[3].s64 = ctx.r[10].s64 + -24076;
	// 825ECAC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825ECACC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECAD0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825ECAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECAE0: 4BE7A341  bl 0x82466e20
	ctx.lr = 0x825ECAE4;
	sub_82466E20(ctx, base);
	// 825ECAE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECAE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECAEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECAF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECAF8 size=100
    let mut pc: u32 = 0x825ECAF8;
    'dispatch: loop {
        match pc {
            0x825ECAF8 => {
    //   block [0x825ECAF8..0x825ECB5C)
	// 825ECAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECB04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECB0C: 38AAA1F4  addi r5, r10, -0x5e0c
	ctx.r[5].s64 = ctx.r[10].s64 + -24076;
	// 825ECB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECB18: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 825ECB1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECB2C: 386AA224  addi r3, r10, -0x5ddc
	ctx.r[3].s64 = ctx.r[10].s64 + -24028;
	// 825ECB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECB34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECB38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825ECB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECB40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825ECB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECB48: 4BE7A2D9  bl 0x82466e20
	ctx.lr = 0x825ECB4C;
	sub_82466E20(ctx, base);
	// 825ECB4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECB50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECB54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECB58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ECB60 size=24
    let mut pc: u32 = 0x825ECB60;
    'dispatch: loop {
        match pc {
            0x825ECB60 => {
    //   block [0x825ECB60..0x825ECB78)
	// 825ECB60: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECB64: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825ECB68: 394ADDC0  addi r10, r10, -0x2240
	ctx.r[10].s64 = ctx.r[10].s64 + -8768;
	// 825ECB6C: 816B6724  lwz r11, 0x6724(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26404 as u32) ) } as u64;
	// 825ECB70: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825ECB74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECB78 size=116
    let mut pc: u32 = 0x825ECB78;
    'dispatch: loop {
        match pc {
            0x825ECB78 => {
    //   block [0x825ECB78..0x825ECBEC)
	// 825ECB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECB84: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ECB88: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECB8C: 392B0FB8  addi r9, r11, 0xfb8
	ctx.r[9].s64 = ctx.r[11].s64 + 4024;
	// 825ECB90: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825ECB94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECB98: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825ECB9C: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 825ECBA0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825ECBA4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 825ECBA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECBAC: 396BDDC0  addi r11, r11, -0x2240
	ctx.r[11].s64 = ctx.r[11].s64 + -8768;
	// 825ECBB0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825ECBB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECBB8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825ECBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECBC0: 386AA254  addi r3, r10, -0x5dac
	ctx.r[3].s64 = ctx.r[10].s64 + -23980;
	// 825ECBC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825ECBC8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825ECBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECBD0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825ECBD4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825ECBD8: 4BE7A249  bl 0x82466e20
	ctx.lr = 0x825ECBDC;
	sub_82466E20(ctx, base);
	// 825ECBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECBF0 size=116
    let mut pc: u32 = 0x825ECBF0;
    'dispatch: loop {
        match pc {
            0x825ECBF0 => {
    //   block [0x825ECBF0..0x825ECC64)
	// 825ECBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECBFC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ECC00: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECC04: 392B0FFC  addi r9, r11, 0xffc
	ctx.r[9].s64 = ctx.r[11].s64 + 4092;
	// 825ECC08: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECC0C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECC10: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 825ECC14: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 825ECC18: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECC1C: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 825ECC20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECC24: 396B6880  addi r11, r11, 0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + 26752;
	// 825ECC28: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825ECC2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECC30: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 825ECC34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECC38: 386AA284  addi r3, r10, -0x5d7c
	ctx.r[3].s64 = ctx.r[10].s64 + -23932;
	// 825ECC3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825ECC40: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 825ECC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECC48: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 825ECC4C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825ECC50: 4BE7A1D1  bl 0x82466e20
	ctx.lr = 0x825ECC54;
	sub_82466E20(ctx, base);
	// 825ECC54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECC58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECC5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECC68 size=108
    let mut pc: u32 = 0x825ECC68;
    'dispatch: loop {
        match pc {
            0x825ECC68 => {
    //   block [0x825ECC68..0x825ECCD4)
	// 825ECC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECC74: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECC78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECC7C: 38EB6928  addi r7, r11, 0x6928
	ctx.r[7].s64 = ctx.r[11].s64 + 26920;
	// 825ECC80: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825ECC84: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 825ECC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECC8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECC90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ECC94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECC98: 386AA2B4  addi r3, r10, -0x5d4c
	ctx.r[3].s64 = ctx.r[10].s64 + -23884;
	// 825ECC9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ECCA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECCA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECCAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECCB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECCB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECCB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECCBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ECCC0: 4BE7A161  bl 0x82466e20
	ctx.lr = 0x825ECCC4;
	sub_82466E20(ctx, base);
	// 825ECCC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECCC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECCCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECCD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECCD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ECCD8 size=24
    let mut pc: u32 = 0x825ECCD8;
    'dispatch: loop {
        match pc {
            0x825ECCD8 => {
    //   block [0x825ECCD8..0x825ECCF0)
	// 825ECCD8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECCDC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825ECCE0: 394ADE38  addi r10, r10, -0x21c8
	ctx.r[10].s64 = ctx.r[10].s64 + -8648;
	// 825ECCE4: 816B687C  lwz r11, 0x687c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26748 as u32) ) } as u64;
	// 825ECCE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825ECCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECCF0 size=116
    let mut pc: u32 = 0x825ECCF0;
    'dispatch: loop {
        match pc {
            0x825ECCF0 => {
    //   block [0x825ECCF0..0x825ECD64)
	// 825ECCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECCFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825ECD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECD04: 390BDE38  addi r8, r11, -0x21c8
	ctx.r[8].s64 = ctx.r[11].s64 + -8648;
	// 825ECD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECD0C: 392A1068  addi r9, r10, 0x1068
	ctx.r[9].s64 = ctx.r[10].s64 + 4200;
	// 825ECD10: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECD14: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 825ECD18: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECD1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECD24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECD34: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825ECD38: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 825ECD3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ECD40: 386BA2E4  addi r3, r11, -0x5d1c
	ctx.r[3].s64 = ctx.r[11].s64 + -23836;
	// 825ECD44: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825ECD48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECD4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECD50: 4BE7A0D1  bl 0x82466e20
	ctx.lr = 0x825ECD54;
	sub_82466E20(ctx, base);
	// 825ECD54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECD58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECD5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECD60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECD68 size=112
    let mut pc: u32 = 0x825ECD68;
    'dispatch: loop {
        match pc {
            0x825ECD68 => {
    //   block [0x825ECD68..0x825ECDD8)
	// 825ECD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECD70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECD74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECD78: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECD7C: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECD80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECD84: 390B6974  addi r8, r11, 0x6974
	ctx.r[8].s64 = ctx.r[11].s64 + 26996;
	// 825ECD88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825ECD8C: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 825ECD90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECD94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECD98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECDA0: 386AA314  addi r3, r10, -0x5cec
	ctx.r[3].s64 = ctx.r[10].s64 + -23788;
	// 825ECDA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ECDA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECDB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECDB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECDB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECDC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECDC4: 4BE7A05D  bl 0x82466e20
	ctx.lr = 0x825ECDC8;
	sub_82466E20(ctx, base);
	// 825ECDC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECDCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECDD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ECDD8 size=24
    let mut pc: u32 = 0x825ECDD8;
    'dispatch: loop {
        match pc {
            0x825ECDD8 => {
    //   block [0x825ECDD8..0x825ECDF0)
	// 825ECDD8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECDDC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825ECDE0: 394ADFB8  addi r10, r10, -0x2048
	ctx.r[10].s64 = ctx.r[10].s64 + -8264;
	// 825ECDE4: 816B69A4  lwz r11, 0x69a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27044 as u32) ) } as u64;
	// 825ECDE8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825ECDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECDF0 size=116
    let mut pc: u32 = 0x825ECDF0;
    'dispatch: loop {
        match pc {
            0x825ECDF0 => {
    //   block [0x825ECDF0..0x825ECE64)
	// 825ECDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECDFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825ECE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECE04: 390BDFB8  addi r8, r11, -0x2048
	ctx.r[8].s64 = ctx.r[11].s64 + -8264;
	// 825ECE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECE0C: 392A10A0  addi r9, r10, 0x10a0
	ctx.r[9].s64 = ctx.r[10].s64 + 4256;
	// 825ECE10: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECE14: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 825ECE18: 38AAA284  addi r5, r10, -0x5d7c
	ctx.r[5].s64 = ctx.r[10].s64 + -23932;
	// 825ECE1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECE24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECE34: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825ECE38: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 825ECE3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ECE40: 386BA344  addi r3, r11, -0x5cbc
	ctx.r[3].s64 = ctx.r[11].s64 + -23740;
	// 825ECE44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825ECE48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECE4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECE50: 4BE79FD1  bl 0x82466e20
	ctx.lr = 0x825ECE54;
	sub_82466E20(ctx, base);
	// 825ECE54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECE68 size=112
    let mut pc: u32 = 0x825ECE68;
    'dispatch: loop {
        match pc {
            0x825ECE68 => {
    //   block [0x825ECE68..0x825ECED8)
	// 825ECE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECE74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECE78: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECE7C: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825ECE80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECE84: 390B69A8  addi r8, r11, 0x69a8
	ctx.r[8].s64 = ctx.r[11].s64 + 27048;
	// 825ECE88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825ECE8C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 825ECE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECE94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECE98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECEA0: 386AA374  addi r3, r10, -0x5c8c
	ctx.r[3].s64 = ctx.r[10].s64 + -23692;
	// 825ECEA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ECEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECEC4: 4BE79F5D  bl 0x82466e20
	ctx.lr = 0x825ECEC8;
	sub_82466E20(ctx, base);
	// 825ECEC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECED8 size=100
    let mut pc: u32 = 0x825ECED8;
    'dispatch: loop {
        match pc {
            0x825ECED8 => {
    //   block [0x825ECED8..0x825ECF3C)
	// 825ECED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECEE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECEEC: 38AAA8E4  addi r5, r10, -0x571c
	ctx.r[5].s64 = ctx.r[10].s64 + -22300;
	// 825ECEF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECEF8: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 825ECEFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECF04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECF0C: 386AA3A4  addi r3, r10, -0x5c5c
	ctx.r[3].s64 = ctx.r[10].s64 + -23644;
	// 825ECF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECF14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECF18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825ECF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECF20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825ECF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECF28: 4BE79EF9  bl 0x82466e20
	ctx.lr = 0x825ECF2C;
	sub_82466E20(ctx, base);
	// 825ECF2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECF30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECF34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECF38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECF40 size=108
    let mut pc: u32 = 0x825ECF40;
    'dispatch: loop {
        match pc {
            0x825ECF40 => {
    //   block [0x825ECF40..0x825ECFAC)
	// 825ECF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECF4C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECF50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECF54: 38EB69C0  addi r7, r11, 0x69c0
	ctx.r[7].s64 = ctx.r[11].s64 + 27072;
	// 825ECF58: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825ECF5C: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 825ECF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECF64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECF68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ECF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECF70: 386AA3D4  addi r3, r10, -0x5c2c
	ctx.r[3].s64 = ctx.r[10].s64 + -23596;
	// 825ECF74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ECF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ECF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ECF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ECF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ECF94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ECF98: 4BE79E89  bl 0x82466e20
	ctx.lr = 0x825ECF9C;
	sub_82466E20(ctx, base);
	// 825ECF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ECFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ECFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ECFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ECFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ECFB0 size=112
    let mut pc: u32 = 0x825ECFB0;
    'dispatch: loop {
        match pc {
            0x825ECFB0 => {
    //   block [0x825ECFB0..0x825ED020)
	// 825ECFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ECFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ECFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ECFBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECFC0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ECFC4: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ECFC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ECFCC: 390B6A98  addi r8, r11, 0x6a98
	ctx.r[8].s64 = ctx.r[11].s64 + 27288;
	// 825ECFD0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825ECFD4: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 825ECFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ECFDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ECFE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ECFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ECFE8: 386AA404  addi r3, r10, -0x5bfc
	ctx.r[3].s64 = ctx.r[10].s64 + -23548;
	// 825ECFEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ECFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ECFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ECFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ECFFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED00C: 4BE79E15  bl 0x82466e20
	ctx.lr = 0x825ED010;
	sub_82466E20(ctx, base);
	// 825ED010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED014: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED018: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED020 size=108
    let mut pc: u32 = 0x825ED020;
    'dispatch: loop {
        match pc {
            0x825ED020 => {
    //   block [0x825ED020..0x825ED08C)
	// 825ED020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED02C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED034: 38EB6AC8  addi r7, r11, 0x6ac8
	ctx.r[7].s64 = ctx.r[11].s64 + 27336;
	// 825ED038: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825ED03C: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 825ED040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED044: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED050: 386AA434  addi r3, r10, -0x5bcc
	ctx.r[3].s64 = ctx.r[10].s64 + -23500;
	// 825ED054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED078: 4BE79DA9  bl 0x82466e20
	ctx.lr = 0x825ED07C;
	sub_82466E20(ctx, base);
	// 825ED07C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED090 size=112
    let mut pc: u32 = 0x825ED090;
    'dispatch: loop {
        match pc {
            0x825ED090 => {
    //   block [0x825ED090..0x825ED100)
	// 825ED090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED09C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED0A0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED0A4: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED0A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED0AC: 390B6AF8  addi r8, r11, 0x6af8
	ctx.r[8].s64 = ctx.r[11].s64 + 27384;
	// 825ED0B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825ED0B4: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 825ED0B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED0BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED0C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED0C8: 386AA464  addi r3, r10, -0x5b9c
	ctx.r[3].s64 = ctx.r[10].s64 + -23452;
	// 825ED0CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED0D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED0D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED0E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED0E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED0E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED0EC: 4BE79D35  bl 0x82466e20
	ctx.lr = 0x825ED0F0;
	sub_82466E20(ctx, base);
	// 825ED0F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED0F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED0F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED0FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED100 size=112
    let mut pc: u32 = 0x825ED100;
    'dispatch: loop {
        match pc {
            0x825ED100 => {
    //   block [0x825ED100..0x825ED170)
	// 825ED100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED10C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825ED110: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 825ED114: 38EA6B10  addi r7, r10, 0x6b10
	ctx.r[7].s64 = ctx.r[10].s64 + 27408;
	// 825ED118: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED11C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ED120: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 825ED124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED128: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED12C: 396B10B4  addi r11, r11, 0x10b4
	ctx.r[11].s64 = ctx.r[11].s64 + 4276;
	// 825ED130: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED134: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED138: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED13C: 386AA494  addi r3, r10, -0x5b6c
	ctx.r[3].s64 = ctx.r[10].s64 + -23404;
	// 825ED140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED144: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825ED148: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED14C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825ED150: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED154: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED158: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED15C: 4BE79CC5  bl 0x82466e20
	ctx.lr = 0x825ED160;
	sub_82466E20(ctx, base);
	// 825ED160: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED164: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED168: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED170 size=108
    let mut pc: u32 = 0x825ED170;
    'dispatch: loop {
        match pc {
            0x825ED170 => {
    //   block [0x825ED170..0x825ED1DC)
	// 825ED170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED17C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED184: 38EB6BE8  addi r7, r11, 0x6be8
	ctx.r[7].s64 = ctx.r[11].s64 + 27624;
	// 825ED188: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825ED18C: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 825ED190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED194: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED198: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED19C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED1A0: 386AA4C4  addi r3, r10, -0x5b3c
	ctx.r[3].s64 = ctx.r[10].s64 + -23356;
	// 825ED1A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED1B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED1B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED1BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED1C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED1C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED1C8: 4BE79C59  bl 0x82466e20
	ctx.lr = 0x825ED1CC;
	sub_82466E20(ctx, base);
	// 825ED1CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED1E0 size=108
    let mut pc: u32 = 0x825ED1E0;
    'dispatch: loop {
        match pc {
            0x825ED1E0 => {
    //   block [0x825ED1E0..0x825ED24C)
	// 825ED1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED1EC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED1F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED1F4: 38EB6C00  addi r7, r11, 0x6c00
	ctx.r[7].s64 = ctx.r[11].s64 + 27648;
	// 825ED1F8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 825ED1FC: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 825ED200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED204: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED208: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED20C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED210: 386AA4F4  addi r3, r10, -0x5b0c
	ctx.r[3].s64 = ctx.r[10].s64 + -23308;
	// 825ED214: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED22C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED234: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED238: 4BE79BE9  bl 0x82466e20
	ctx.lr = 0x825ED23C;
	sub_82466E20(ctx, base);
	// 825ED23C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED250 size=108
    let mut pc: u32 = 0x825ED250;
    'dispatch: loop {
        match pc {
            0x825ED250 => {
    //   block [0x825ED250..0x825ED2BC)
	// 825ED250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED25C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED264: 38EB6D08  addi r7, r11, 0x6d08
	ctx.r[7].s64 = ctx.r[11].s64 + 27912;
	// 825ED268: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825ED26C: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 825ED270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED274: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED278: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED27C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED280: 386AA524  addi r3, r10, -0x5adc
	ctx.r[3].s64 = ctx.r[10].s64 + -23260;
	// 825ED284: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED28C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED2A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED2A8: 4BE79B79  bl 0x82466e20
	ctx.lr = 0x825ED2AC;
	sub_82466E20(ctx, base);
	// 825ED2AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED2B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED2B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED2B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED2C0 size=112
    let mut pc: u32 = 0x825ED2C0;
    'dispatch: loop {
        match pc {
            0x825ED2C0 => {
    //   block [0x825ED2C0..0x825ED330)
	// 825ED2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED2C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED2CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED2D0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED2D4: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED2D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED2DC: 390B6D68  addi r8, r11, 0x6d68
	ctx.r[8].s64 = ctx.r[11].s64 + 28008;
	// 825ED2E0: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825ED2E4: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 825ED2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED2EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED2F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED2F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED2F8: 386AA554  addi r3, r10, -0x5aac
	ctx.r[3].s64 = ctx.r[10].s64 + -23212;
	// 825ED2FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED30C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED31C: 4BE79B05  bl 0x82466e20
	ctx.lr = 0x825ED320;
	sub_82466E20(ctx, base);
	// 825ED320: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED324: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED328: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED32C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED330 size=112
    let mut pc: u32 = 0x825ED330;
    'dispatch: loop {
        match pc {
            0x825ED330 => {
    //   block [0x825ED330..0x825ED3A0)
	// 825ED330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED33C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED340: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED344: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED34C: 390B6E88  addi r8, r11, 0x6e88
	ctx.r[8].s64 = ctx.r[11].s64 + 28296;
	// 825ED350: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825ED354: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 825ED358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED35C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED360: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED368: 386AA584  addi r3, r10, -0x5a7c
	ctx.r[3].s64 = ctx.r[10].s64 + -23164;
	// 825ED36C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED38C: 4BE79A95  bl 0x82466e20
	ctx.lr = 0x825ED390;
	sub_82466E20(ctx, base);
	// 825ED390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED394: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED398: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED3A0 size=116
    let mut pc: u32 = 0x825ED3A0;
    'dispatch: loop {
        match pc {
            0x825ED3A0 => {
    //   block [0x825ED3A0..0x825ED414)
	// 825ED3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED3AC: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825ED3B0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825ED3B4: 390A6EA0  addi r8, r10, 0x6ea0
	ctx.r[8].s64 = ctx.r[10].s64 + 28320;
	// 825ED3B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED3BC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ED3C0: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED3C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED3C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ED3CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED3D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED3D4: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 825ED3D8: 396B10E4  addi r11, r11, 0x10e4
	ctx.r[11].s64 = ctx.r[11].s64 + 4324;
	// 825ED3DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED3E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED3E4: 386AA5B4  addi r3, r10, -0x5a4c
	ctx.r[3].s64 = ctx.r[10].s64 + -23116;
	// 825ED3E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825ED3EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED3F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825ED3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED3F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED400: 4BE79A21  bl 0x82466e20
	ctx.lr = 0x825ED404;
	sub_82466E20(ctx, base);
	// 825ED404: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED40C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED418 size=108
    let mut pc: u32 = 0x825ED418;
    'dispatch: loop {
        match pc {
            0x825ED418 => {
    //   block [0x825ED418..0x825ED484)
	// 825ED418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED424: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED42C: 38EB6F00  addi r7, r11, 0x6f00
	ctx.r[7].s64 = ctx.r[11].s64 + 28416;
	// 825ED430: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825ED434: 388A3224  addi r4, r10, 0x3224
	ctx.r[4].s64 = ctx.r[10].s64 + 12836;
	// 825ED438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED43C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED440: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED448: 386AA5E4  addi r3, r10, -0x5a1c
	ctx.r[3].s64 = ctx.r[10].s64 + -23068;
	// 825ED44C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED45C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED46C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED470: 4BE799B1  bl 0x82466e20
	ctx.lr = 0x825ED474;
	sub_82466E20(ctx, base);
	// 825ED474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED47C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED488 size=108
    let mut pc: u32 = 0x825ED488;
    'dispatch: loop {
        match pc {
            0x825ED488 => {
    //   block [0x825ED488..0x825ED4F4)
	// 825ED488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED494: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED49C: 38EB6F48  addi r7, r11, 0x6f48
	ctx.r[7].s64 = ctx.r[11].s64 + 28488;
	// 825ED4A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825ED4A4: 388A3244  addi r4, r10, 0x3244
	ctx.r[4].s64 = ctx.r[10].s64 + 12868;
	// 825ED4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED4AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED4B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED4B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED4B8: 386AA614  addi r3, r10, -0x59ec
	ctx.r[3].s64 = ctx.r[10].s64 + -23020;
	// 825ED4BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED4C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED4C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED4CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED4D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED4D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED4D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED4DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED4E0: 4BE79941  bl 0x82466e20
	ctx.lr = 0x825ED4E4;
	sub_82466E20(ctx, base);
	// 825ED4E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED4F8 size=112
    let mut pc: u32 = 0x825ED4F8;
    'dispatch: loop {
        match pc {
            0x825ED4F8 => {
    //   block [0x825ED4F8..0x825ED568)
	// 825ED4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED504: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED508: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED50C: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED514: 390B6F90  addi r8, r11, 0x6f90
	ctx.r[8].s64 = ctx.r[11].s64 + 28560;
	// 825ED518: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 825ED51C: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 825ED520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED524: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED530: 386AA644  addi r3, r10, -0x59bc
	ctx.r[3].s64 = ctx.r[10].s64 + -22972;
	// 825ED534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED554: 4BE798CD  bl 0x82466e20
	ctx.lr = 0x825ED558;
	sub_82466E20(ctx, base);
	// 825ED558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED568 size=112
    let mut pc: u32 = 0x825ED568;
    'dispatch: loop {
        match pc {
            0x825ED568 => {
    //   block [0x825ED568..0x825ED5D8)
	// 825ED568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED574: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED578: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED57C: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED584: 390B7098  addi r8, r11, 0x7098
	ctx.r[8].s64 = ctx.r[11].s64 + 28824;
	// 825ED588: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 825ED58C: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 825ED590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED594: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED5A0: 386AA674  addi r3, r10, -0x598c
	ctx.r[3].s64 = ctx.r[10].s64 + -22924;
	// 825ED5A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED5B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED5C4: 4BE7985D  bl 0x82466e20
	ctx.lr = 0x825ED5C8;
	sub_82466E20(ctx, base);
	// 825ED5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED5D8 size=112
    let mut pc: u32 = 0x825ED5D8;
    'dispatch: loop {
        match pc {
            0x825ED5D8 => {
    //   block [0x825ED5D8..0x825ED648)
	// 825ED5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED5E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED5E8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED5EC: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED5F4: 390B71A0  addi r8, r11, 0x71a0
	ctx.r[8].s64 = ctx.r[11].s64 + 29088;
	// 825ED5F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825ED5FC: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 825ED600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED604: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED60C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED610: 386AA6A4  addi r3, r10, -0x595c
	ctx.r[3].s64 = ctx.r[10].s64 + -22876;
	// 825ED614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED634: 4BE797ED  bl 0x82466e20
	ctx.lr = 0x825ED638;
	sub_82466E20(ctx, base);
	// 825ED638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED648 size=112
    let mut pc: u32 = 0x825ED648;
    'dispatch: loop {
        match pc {
            0x825ED648 => {
    //   block [0x825ED648..0x825ED6B8)
	// 825ED648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED654: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED658: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED65C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825ED660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED664: 390B71B8  addi r8, r11, 0x71b8
	ctx.r[8].s64 = ctx.r[11].s64 + 29112;
	// 825ED668: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825ED66C: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 825ED670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED674: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED67C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED680: 386AA6D4  addi r3, r10, -0x592c
	ctx.r[3].s64 = ctx.r[10].s64 + -22828;
	// 825ED684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED6A4: 4BE7977D  bl 0x82466e20
	ctx.lr = 0x825ED6A8;
	sub_82466E20(ctx, base);
	// 825ED6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED6B8 size=108
    let mut pc: u32 = 0x825ED6B8;
    'dispatch: loop {
        match pc {
            0x825ED6B8 => {
    //   block [0x825ED6B8..0x825ED724)
	// 825ED6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED6C4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED6C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED6CC: 38EB71E8  addi r7, r11, 0x71e8
	ctx.r[7].s64 = ctx.r[11].s64 + 29160;
	// 825ED6D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825ED6D4: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 825ED6D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED6DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED6E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED6E8: 386AA704  addi r3, r10, -0x58fc
	ctx.r[3].s64 = ctx.r[10].s64 + -22780;
	// 825ED6EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED6F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED6F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED70C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED710: 4BE79711  bl 0x82466e20
	ctx.lr = 0x825ED714;
	sub_82466E20(ctx, base);
	// 825ED714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825ED728 size=24
    let mut pc: u32 = 0x825ED728;
    'dispatch: loop {
        match pc {
            0x825ED728 => {
    //   block [0x825ED728..0x825ED740)
	// 825ED728: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED72C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825ED730: 394AE0A8  addi r10, r10, -0x1f58
	ctx.r[10].s64 = ctx.r[10].s64 + -8024;
	// 825ED734: 816B7260  lwz r11, 0x7260(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(29280 as u32) ) } as u64;
	// 825ED738: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825ED73C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED740 size=116
    let mut pc: u32 = 0x825ED740;
    'dispatch: loop {
        match pc {
            0x825ED740 => {
    //   block [0x825ED740..0x825ED7B4)
	// 825ED740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED74C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825ED750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED754: 390BE0A8  addi r8, r11, -0x1f58
	ctx.r[8].s64 = ctx.r[11].s64 + -8024;
	// 825ED758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED75C: 392A1110  addi r9, r10, 0x1110
	ctx.r[9].s64 = ctx.r[10].s64 + 4368;
	// 825ED760: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED764: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 825ED768: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED76C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED774: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED784: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825ED788: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 825ED78C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ED790: 386BA734  addi r3, r11, -0x58cc
	ctx.r[3].s64 = ctx.r[11].s64 + -22732;
	// 825ED794: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825ED798: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED79C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED7A0: 4BE79681  bl 0x82466e20
	ctx.lr = 0x825ED7A4;
	sub_82466E20(ctx, base);
	// 825ED7A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED7B8 size=112
    let mut pc: u32 = 0x825ED7B8;
    'dispatch: loop {
        match pc {
            0x825ED7B8 => {
    //   block [0x825ED7B8..0x825ED828)
	// 825ED7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED7C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED7C8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED7CC: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED7D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED7D4: 390B7264  addi r8, r11, 0x7264
	ctx.r[8].s64 = ctx.r[11].s64 + 29284;
	// 825ED7D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825ED7DC: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 825ED7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED7E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED7E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED7F0: 386AA764  addi r3, r10, -0x589c
	ctx.r[3].s64 = ctx.r[10].s64 + -22684;
	// 825ED7F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED814: 4BE7960D  bl 0x82466e20
	ctx.lr = 0x825ED818;
	sub_82466E20(ctx, base);
	// 825ED818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED828 size=116
    let mut pc: u32 = 0x825ED828;
    'dispatch: loop {
        match pc {
            0x825ED828 => {
    //   block [0x825ED828..0x825ED89C)
	// 825ED828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED834: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825ED838: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825ED83C: 390A7298  addi r8, r10, 0x7298
	ctx.r[8].s64 = ctx.r[10].s64 + 29336;
	// 825ED840: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED844: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ED848: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED84C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED850: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ED854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED85C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 825ED860: 396B1124  addi r11, r11, 0x1124
	ctx.r[11].s64 = ctx.r[11].s64 + 4388;
	// 825ED864: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED868: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED86C: 386AA794  addi r3, r10, -0x586c
	ctx.r[3].s64 = ctx.r[10].s64 + -22636;
	// 825ED870: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825ED874: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED878: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825ED87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED888: 4BE79599  bl 0x82466e20
	ctx.lr = 0x825ED88C;
	sub_82466E20(ctx, base);
	// 825ED88C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED8A0 size=112
    let mut pc: u32 = 0x825ED8A0;
    'dispatch: loop {
        match pc {
            0x825ED8A0 => {
    //   block [0x825ED8A0..0x825ED910)
	// 825ED8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED8AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED8B0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED8B4: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED8BC: 390B7358  addi r8, r11, 0x7358
	ctx.r[8].s64 = ctx.r[11].s64 + 29528;
	// 825ED8C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825ED8C4: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 825ED8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED8CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED8D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED8D8: 386AA7C4  addi r3, r10, -0x583c
	ctx.r[3].s64 = ctx.r[10].s64 + -22588;
	// 825ED8DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825ED8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED8FC: 4BE79525  bl 0x82466e20
	ctx.lr = 0x825ED900;
	sub_82466E20(ctx, base);
	// 825ED900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED910 size=108
    let mut pc: u32 = 0x825ED910;
    'dispatch: loop {
        match pc {
            0x825ED910 => {
    //   block [0x825ED910..0x825ED97C)
	// 825ED910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED91C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825ED920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED924: 38EB7370  addi r7, r11, 0x7370
	ctx.r[7].s64 = ctx.r[11].s64 + 29552;
	// 825ED928: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 825ED92C: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 825ED930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825ED934: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED938: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825ED93C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED940: 386AA7F4  addi r3, r10, -0x580c
	ctx.r[3].s64 = ctx.r[10].s64 + -22540;
	// 825ED944: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825ED948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED94C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825ED954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825ED95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED964: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825ED968: 4BE794B9  bl 0x82466e20
	ctx.lr = 0x825ED96C;
	sub_82466E20(ctx, base);
	// 825ED96C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED980 size=116
    let mut pc: u32 = 0x825ED980;
    'dispatch: loop {
        match pc {
            0x825ED980 => {
    //   block [0x825ED980..0x825ED9F4)
	// 825ED980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825ED988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825ED98C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825ED990: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825ED994: 390A74A8  addi r8, r10, 0x74a8
	ctx.r[8].s64 = ctx.r[10].s64 + 29864;
	// 825ED998: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED99C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825ED9A0: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825ED9A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825ED9A8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825ED9AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825ED9B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825ED9B4: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 825ED9B8: 396B1148  addi r11, r11, 0x1148
	ctx.r[11].s64 = ctx.r[11].s64 + 4424;
	// 825ED9BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825ED9C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825ED9C4: 386AA824  addi r3, r10, -0x57dc
	ctx.r[3].s64 = ctx.r[10].s64 + -22492;
	// 825ED9C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825ED9CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825ED9D0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825ED9D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825ED9D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825ED9DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825ED9E0: 4BE79441  bl 0x82466e20
	ctx.lr = 0x825ED9E4;
	sub_82466E20(ctx, base);
	// 825ED9E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825ED9E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825ED9EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825ED9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825ED9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825ED9F8 size=112
    let mut pc: u32 = 0x825ED9F8;
    'dispatch: loop {
        match pc {
            0x825ED9F8 => {
    //   block [0x825ED9F8..0x825EDA68)
	// 825ED9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825ED9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDA04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDA08: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDA0C: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825EDA10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDA14: 390B7550  addi r8, r11, 0x7550
	ctx.r[8].s64 = ctx.r[11].s64 + 30032;
	// 825EDA18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EDA1C: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 825EDA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDA24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDA28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDA2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDA30: 386AA854  addi r3, r10, -0x57ac
	ctx.r[3].s64 = ctx.r[10].s64 + -22444;
	// 825EDA34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDA4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDA54: 4BE793CD  bl 0x82466e20
	ctx.lr = 0x825EDA58;
	sub_82466E20(ctx, base);
	// 825EDA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDA68 size=112
    let mut pc: u32 = 0x825EDA68;
    'dispatch: loop {
        match pc {
            0x825EDA68 => {
    //   block [0x825EDA68..0x825EDAD8)
	// 825EDA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDA74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDA78: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDA7C: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825EDA80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDA84: 390B7568  addi r8, r11, 0x7568
	ctx.r[8].s64 = ctx.r[11].s64 + 30056;
	// 825EDA88: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 825EDA8C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 825EDA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDA94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDAA0: 386AA884  addi r3, r10, -0x577c
	ctx.r[3].s64 = ctx.r[10].s64 + -22396;
	// 825EDAA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDAC4: 4BE7935D  bl 0x82466e20
	ctx.lr = 0x825EDAC8;
	sub_82466E20(ctx, base);
	// 825EDAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDAD8 size=112
    let mut pc: u32 = 0x825EDAD8;
    'dispatch: loop {
        match pc {
            0x825EDAD8 => {
    //   block [0x825EDAD8..0x825EDB48)
	// 825EDAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDAE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDAE8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDAEC: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825EDAF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDAF4: 390B7688  addi r8, r11, 0x7688
	ctx.r[8].s64 = ctx.r[11].s64 + 30344;
	// 825EDAF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EDAFC: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 825EDB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDB04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDB08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDB10: 386AA8B4  addi r3, r10, -0x574c
	ctx.r[3].s64 = ctx.r[10].s64 + -22348;
	// 825EDB14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDB34: 4BE792ED  bl 0x82466e20
	ctx.lr = 0x825EDB38;
	sub_82466E20(ctx, base);
	// 825EDB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDB48 size=116
    let mut pc: u32 = 0x825EDB48;
    'dispatch: loop {
        match pc {
            0x825EDB48 => {
    //   block [0x825EDB48..0x825EDBBC)
	// 825EDB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDB54: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDB58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDB5C: 390B76A0  addi r8, r11, 0x76a0
	ctx.r[8].s64 = ctx.r[11].s64 + 30368;
	// 825EDB60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDB64: 392A1180  addi r9, r10, 0x1180
	ctx.r[9].s64 = ctx.r[10].s64 + 4480;
	// 825EDB68: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDB6C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825EDB70: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDB74: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDB78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDB7C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDB80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDB88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDB8C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EDB90: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 825EDB94: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EDB98: 386BA8E4  addi r3, r11, -0x571c
	ctx.r[3].s64 = ctx.r[11].s64 + -22300;
	// 825EDB9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EDBA0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDBA8: 4BE79279  bl 0x82466e20
	ctx.lr = 0x825EDBAC;
	sub_82466E20(ctx, base);
	// 825EDBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDBC0 size=100
    let mut pc: u32 = 0x825EDBC0;
    'dispatch: loop {
        match pc {
            0x825EDBC0 => {
    //   block [0x825EDBC0..0x825EDC24)
	// 825EDBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDBCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDBD4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDBD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDBE0: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 825EDBE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDBE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDBF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDBF4: 386AA914  addi r3, r10, -0x56ec
	ctx.r[3].s64 = ctx.r[10].s64 + -22252;
	// 825EDBF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDBFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDC00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EDC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDC08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EDC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDC10: 4BE79211  bl 0x82466e20
	ctx.lr = 0x825EDC14;
	sub_82466E20(ctx, base);
	// 825EDC14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDC18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDC1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDC20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDC28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDC28 size=112
    let mut pc: u32 = 0x825EDC28;
    'dispatch: loop {
        match pc {
            0x825EDC28 => {
    //   block [0x825EDC28..0x825EDC98)
	// 825EDC28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDC2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDC30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDC38: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDC3C: 38AAA914  addi r5, r10, -0x56ec
	ctx.r[5].s64 = ctx.r[10].s64 + -22252;
	// 825EDC40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDC44: 390B76D0  addi r8, r11, 0x76d0
	ctx.r[8].s64 = ctx.r[11].s64 + 30416;
	// 825EDC48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EDC4C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 825EDC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDC54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDC58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDC5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDC60: 386AA944  addi r3, r10, -0x56bc
	ctx.r[3].s64 = ctx.r[10].s64 + -22204;
	// 825EDC64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDC68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDC70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDC74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDC78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDC80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDC84: 4BE7919D  bl 0x82466e20
	ctx.lr = 0x825EDC88;
	sub_82466E20(ctx, base);
	// 825EDC88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDC98 size=112
    let mut pc: u32 = 0x825EDC98;
    'dispatch: loop {
        match pc {
            0x825EDC98 => {
    //   block [0x825EDC98..0x825EDD08)
	// 825EDC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDCA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDCA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDCA8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDCAC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDCB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDCB4: 390B76E8  addi r8, r11, 0x76e8
	ctx.r[8].s64 = ctx.r[11].s64 + 30440;
	// 825EDCB8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825EDCBC: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 825EDCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDCC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDCC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDCCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDCD0: 386AA974  addi r3, r10, -0x568c
	ctx.r[3].s64 = ctx.r[10].s64 + -22156;
	// 825EDCD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDCD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDCDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDCE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDCE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDCE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDCEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDCF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDCF4: 4BE7912D  bl 0x82466e20
	ctx.lr = 0x825EDCF8;
	sub_82466E20(ctx, base);
	// 825EDCF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDD04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDD08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDD08 size=112
    let mut pc: u32 = 0x825EDD08;
    'dispatch: loop {
        match pc {
            0x825EDD08 => {
    //   block [0x825EDD08..0x825EDD78)
	// 825EDD08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDD0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDD10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDD14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDD18: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDD1C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDD20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDD24: 390B7790  addi r8, r11, 0x7790
	ctx.r[8].s64 = ctx.r[11].s64 + 30608;
	// 825EDD28: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EDD2C: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 825EDD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDD34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDD38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDD3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDD40: 386AA9A4  addi r3, r10, -0x565c
	ctx.r[3].s64 = ctx.r[10].s64 + -22108;
	// 825EDD44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDD48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDD4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDD50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDD58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDD60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDD64: 4BE790BD  bl 0x82466e20
	ctx.lr = 0x825EDD68;
	sub_82466E20(ctx, base);
	// 825EDD68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDD74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDD78 size=112
    let mut pc: u32 = 0x825EDD78;
    'dispatch: loop {
        match pc {
            0x825EDD78 => {
    //   block [0x825EDD78..0x825EDDE8)
	// 825EDD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDD84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDD88: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDD8C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDD94: 390B77D8  addi r8, r11, 0x77d8
	ctx.r[8].s64 = ctx.r[11].s64 + 30680;
	// 825EDD98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EDD9C: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 825EDDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDDA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDDA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDDB0: 386AA9D4  addi r3, r10, -0x562c
	ctx.r[3].s64 = ctx.r[10].s64 + -22060;
	// 825EDDB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDDD4: 4BE7904D  bl 0x82466e20
	ctx.lr = 0x825EDDD8;
	sub_82466E20(ctx, base);
	// 825EDDD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDDE8 size=112
    let mut pc: u32 = 0x825EDDE8;
    'dispatch: loop {
        match pc {
            0x825EDDE8 => {
    //   block [0x825EDDE8..0x825EDE58)
	// 825EDDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDDF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDDF8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDDFC: 38AAA3A4  addi r5, r10, -0x5c5c
	ctx.r[5].s64 = ctx.r[10].s64 + -23644;
	// 825EDE00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDE04: 390B7808  addi r8, r11, 0x7808
	ctx.r[8].s64 = ctx.r[11].s64 + 30728;
	// 825EDE08: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 825EDE0C: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 825EDE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDE14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDE18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDE20: 386AAA04  addi r3, r10, -0x55fc
	ctx.r[3].s64 = ctx.r[10].s64 + -22012;
	// 825EDE24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDE2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDE34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDE44: 4BE78FDD  bl 0x82466e20
	ctx.lr = 0x825EDE48;
	sub_82466E20(ctx, base);
	// 825EDE48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDE4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDE50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDE54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDE58 size=100
    let mut pc: u32 = 0x825EDE58;
    'dispatch: loop {
        match pc {
            0x825EDE58 => {
    //   block [0x825EDE58..0x825EDEBC)
	// 825EDE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDE64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDE6C: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EDE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDE74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDE78: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 825EDE7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDE8C: 386AAA34  addi r3, r10, -0x55cc
	ctx.r[3].s64 = ctx.r[10].s64 + -21964;
	// 825EDE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDE94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDE98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EDE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDEA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EDEA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDEA8: 4BE78F79  bl 0x82466e20
	ctx.lr = 0x825EDEAC;
	sub_82466E20(ctx, base);
	// 825EDEAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDEC0 size=108
    let mut pc: u32 = 0x825EDEC0;
    'dispatch: loop {
        match pc {
            0x825EDEC0 => {
    //   block [0x825EDEC0..0x825EDF2C)
	// 825EDEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDECC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDED4: 38EB7880  addi r7, r11, 0x7880
	ctx.r[7].s64 = ctx.r[11].s64 + 30848;
	// 825EDED8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EDEDC: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 825EDEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDEE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDEE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EDEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDEF0: 386AAA64  addi r3, r10, -0x559c
	ctx.r[3].s64 = ctx.r[10].s64 + -21916;
	// 825EDEF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EDEF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDF14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EDF18: 4BE78F09  bl 0x82466e20
	ctx.lr = 0x825EDF1C;
	sub_82466E20(ctx, base);
	// 825EDF1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDF30 size=112
    let mut pc: u32 = 0x825EDF30;
    'dispatch: loop {
        match pc {
            0x825EDF30 => {
    //   block [0x825EDF30..0x825EDFA0)
	// 825EDF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDF3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDF40: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDF44: 38AAAA34  addi r5, r10, -0x55cc
	ctx.r[5].s64 = ctx.r[10].s64 + -21964;
	// 825EDF48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDF4C: 390B78B0  addi r8, r11, 0x78b0
	ctx.r[8].s64 = ctx.r[11].s64 + 30896;
	// 825EDF50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EDF54: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 825EDF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDF5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDF60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EDF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDF68: 386AAA94  addi r3, r10, -0x556c
	ctx.r[3].s64 = ctx.r[10].s64 + -21868;
	// 825EDF6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EDF70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDF74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDF78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDF80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDF88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDF8C: 4BE78E95  bl 0x82466e20
	ctx.lr = 0x825EDF90;
	sub_82466E20(ctx, base);
	// 825EDF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EDF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EDF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EDF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EDFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EDFA0 size=108
    let mut pc: u32 = 0x825EDFA0;
    'dispatch: loop {
        match pc {
            0x825EDFA0 => {
    //   block [0x825EDFA0..0x825EE00C)
	// 825EDFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EDFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EDFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EDFAC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EDFB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EDFB4: 38EB78E0  addi r7, r11, 0x78e0
	ctx.r[7].s64 = ctx.r[11].s64 + 30944;
	// 825EDFB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EDFBC: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 825EDFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EDFC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EDFC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EDFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EDFD0: 386AAAC4  addi r3, r10, -0x553c
	ctx.r[3].s64 = ctx.r[10].s64 + -21820;
	// 825EDFD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EDFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EDFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EDFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EDFE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EDFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EDFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EDFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EDFF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EDFF8: 4BE78E29  bl 0x82466e20
	ctx.lr = 0x825EDFFC;
	sub_82466E20(ctx, base);
	// 825EDFFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE010 size=112
    let mut pc: u32 = 0x825EE010;
    'dispatch: loop {
        match pc {
            0x825EE010 => {
    //   block [0x825EE010..0x825EE080)
	// 825EE010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE01C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE020: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE024: 38AAAA34  addi r5, r10, -0x55cc
	ctx.r[5].s64 = ctx.r[10].s64 + -21964;
	// 825EE028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE02C: 390B7910  addi r8, r11, 0x7910
	ctx.r[8].s64 = ctx.r[11].s64 + 30992;
	// 825EE030: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EE034: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 825EE038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE03C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE040: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE048: 386AAAF4  addi r3, r10, -0x550c
	ctx.r[3].s64 = ctx.r[10].s64 + -21772;
	// 825EE04C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE06C: 4BE78DB5  bl 0x82466e20
	ctx.lr = 0x825EE070;
	sub_82466E20(ctx, base);
	// 825EE070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE080 size=108
    let mut pc: u32 = 0x825EE080;
    'dispatch: loop {
        match pc {
            0x825EE080 => {
    //   block [0x825EE080..0x825EE0EC)
	// 825EE080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE08C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE094: 38EB7958  addi r7, r11, 0x7958
	ctx.r[7].s64 = ctx.r[11].s64 + 31064;
	// 825EE098: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EE09C: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 825EE0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE0A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE0A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE0AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE0B0: 386AAB24  addi r3, r10, -0x54dc
	ctx.r[3].s64 = ctx.r[10].s64 + -21724;
	// 825EE0B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE0B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE0BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE0C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE0C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE0C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE0CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE0D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE0D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE0D8: 4BE78D49  bl 0x82466e20
	ctx.lr = 0x825EE0DC;
	sub_82466E20(ctx, base);
	// 825EE0DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE0E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE0E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE0E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE0F0 size=112
    let mut pc: u32 = 0x825EE0F0;
    'dispatch: loop {
        match pc {
            0x825EE0F0 => {
    //   block [0x825EE0F0..0x825EE160)
	// 825EE0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE0F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE0FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE100: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE104: 38AAAA34  addi r5, r10, -0x55cc
	ctx.r[5].s64 = ctx.r[10].s64 + -21964;
	// 825EE108: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE10C: 390B7988  addi r8, r11, 0x7988
	ctx.r[8].s64 = ctx.r[11].s64 + 31112;
	// 825EE110: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EE114: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 825EE118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE11C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE128: 386AAB54  addi r3, r10, -0x54ac
	ctx.r[3].s64 = ctx.r[10].s64 + -21676;
	// 825EE12C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE14C: 4BE78CD5  bl 0x82466e20
	ctx.lr = 0x825EE150;
	sub_82466E20(ctx, base);
	// 825EE150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE15C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE160 size=108
    let mut pc: u32 = 0x825EE160;
    'dispatch: loop {
        match pc {
            0x825EE160 => {
    //   block [0x825EE160..0x825EE1CC)
	// 825EE160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE16C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE174: 38EB79D0  addi r7, r11, 0x79d0
	ctx.r[7].s64 = ctx.r[11].s64 + 31184;
	// 825EE178: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EE17C: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 825EE180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE184: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE18C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE190: 386AAB84  addi r3, r10, -0x547c
	ctx.r[3].s64 = ctx.r[10].s64 + -21628;
	// 825EE194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE19C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE1A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE1AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE1B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE1B8: 4BE78C69  bl 0x82466e20
	ctx.lr = 0x825EE1BC;
	sub_82466E20(ctx, base);
	// 825EE1BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE1C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE1C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE1C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE1D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE1D0 size=112
    let mut pc: u32 = 0x825EE1D0;
    'dispatch: loop {
        match pc {
            0x825EE1D0 => {
    //   block [0x825EE1D0..0x825EE240)
	// 825EE1D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE1D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE1D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE1DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE1E0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE1E4: 38AAAA34  addi r5, r10, -0x55cc
	ctx.r[5].s64 = ctx.r[10].s64 + -21964;
	// 825EE1E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE1EC: 390B7A00  addi r8, r11, 0x7a00
	ctx.r[8].s64 = ctx.r[11].s64 + 31232;
	// 825EE1F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 825EE1F4: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 825EE1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE1FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE208: 386AABB4  addi r3, r10, -0x544c
	ctx.r[3].s64 = ctx.r[10].s64 + -21580;
	// 825EE20C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE22C: 4BE78BF5  bl 0x82466e20
	ctx.lr = 0x825EE230;
	sub_82466E20(ctx, base);
	// 825EE230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE240 size=108
    let mut pc: u32 = 0x825EE240;
    'dispatch: loop {
        match pc {
            0x825EE240 => {
    //   block [0x825EE240..0x825EE2AC)
	// 825EE240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE24C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE254: 38EB7A48  addi r7, r11, 0x7a48
	ctx.r[7].s64 = ctx.r[11].s64 + 31304;
	// 825EE258: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EE25C: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 825EE260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE264: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE270: 386AABE4  addi r3, r10, -0x541c
	ctx.r[3].s64 = ctx.r[10].s64 + -21532;
	// 825EE274: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE27C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE28C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE294: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE298: 4BE78B89  bl 0x82466e20
	ctx.lr = 0x825EE29C;
	sub_82466E20(ctx, base);
	// 825EE29C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE2A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE2A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE2A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE2B0 size=112
    let mut pc: u32 = 0x825EE2B0;
    'dispatch: loop {
        match pc {
            0x825EE2B0 => {
    //   block [0x825EE2B0..0x825EE320)
	// 825EE2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE2B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE2BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE2C0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE2C4: 392A11F0  addi r9, r10, 0x11f0
	ctx.r[9].s64 = ctx.r[10].s64 + 4592;
	// 825EE2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE2CC: 390B7AB0  addi r8, r11, 0x7ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 31408;
	// 825EE2D0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825EE2D4: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 825EE2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE2DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE2E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE2E8: 386AAC14  addi r3, r10, -0x53ec
	ctx.r[3].s64 = ctx.r[10].s64 + -21484;
	// 825EE2EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EE2F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EE2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE304: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE30C: 4BE78B15  bl 0x82466e20
	ctx.lr = 0x825EE310;
	sub_82466E20(ctx, base);
	// 825EE310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE31C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE320 size=108
    let mut pc: u32 = 0x825EE320;
    'dispatch: loop {
        match pc {
            0x825EE320 => {
    //   block [0x825EE320..0x825EE38C)
	// 825EE320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE32C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE334: 38EB7B70  addi r7, r11, 0x7b70
	ctx.r[7].s64 = ctx.r[11].s64 + 31600;
	// 825EE338: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825EE33C: 388A35EC  addi r4, r10, 0x35ec
	ctx.r[4].s64 = ctx.r[10].s64 + 13804;
	// 825EE340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE344: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE348: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE34C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE350: 386AAC44  addi r3, r10, -0x53bc
	ctx.r[3].s64 = ctx.r[10].s64 + -21436;
	// 825EE354: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE35C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE364: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE36C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE374: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE378: 4BE78AA9  bl 0x82466e20
	ctx.lr = 0x825EE37C;
	sub_82466E20(ctx, base);
	// 825EE37C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE388: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE390 size=108
    let mut pc: u32 = 0x825EE390;
    'dispatch: loop {
        match pc {
            0x825EE390 => {
    //   block [0x825EE390..0x825EE3FC)
	// 825EE390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE39C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE3A4: 38EB7BB8  addi r7, r11, 0x7bb8
	ctx.r[7].s64 = ctx.r[11].s64 + 31672;
	// 825EE3A8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EE3AC: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 825EE3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE3B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE3B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE3C0: 386AAC74  addi r3, r10, -0x538c
	ctx.r[3].s64 = ctx.r[10].s64 + -21388;
	// 825EE3C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE3E8: 4BE78A39  bl 0x82466e20
	ctx.lr = 0x825EE3EC;
	sub_82466E20(ctx, base);
	// 825EE3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE400 size=116
    let mut pc: u32 = 0x825EE400;
    'dispatch: loop {
        match pc {
            0x825EE400 => {
    //   block [0x825EE400..0x825EE474)
	// 825EE400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE40C: 3D408273  lis r10, -0x7d8d
	ctx.r[10].s64 = -2106392576;
	// 825EE410: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 825EE414: 390A7C30  addi r8, r10, 0x7c30
	ctx.r[8].s64 = ctx.r[10].s64 + 31792;
	// 825EE418: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE41C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 825EE420: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825EE424: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE428: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EE42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE434: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 825EE438: 396B1208  addi r11, r11, 0x1208
	ctx.r[11].s64 = ctx.r[11].s64 + 4616;
	// 825EE43C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE440: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE444: 386AACA4  addi r3, r10, -0x535c
	ctx.r[3].s64 = ctx.r[10].s64 + -21340;
	// 825EE448: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825EE44C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE450: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 825EE454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE460: 4BE789C1  bl 0x82466e20
	ctx.lr = 0x825EE464;
	sub_82466E20(ctx, base);
	// 825EE464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE46C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE478 size=100
    let mut pc: u32 = 0x825EE478;
    'dispatch: loop {
        match pc {
            0x825EE478 => {
    //   block [0x825EE478..0x825EE4DC)
	// 825EE478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE484: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE48C: 38AAA134  addi r5, r10, -0x5ecc
	ctx.r[5].s64 = ctx.r[10].s64 + -24268;
	// 825EE490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE498: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 825EE49C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE4A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE4AC: 386AACD4  addi r3, r10, -0x532c
	ctx.r[3].s64 = ctx.r[10].s64 + -21292;
	// 825EE4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE4B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE4B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EE4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE4C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EE4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE4C8: 4BE78959  bl 0x82466e20
	ctx.lr = 0x825EE4CC;
	sub_82466E20(ctx, base);
	// 825EE4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EE4E0 size=24
    let mut pc: u32 = 0x825EE4E0;
    'dispatch: loop {
        match pc {
            0x825EE4E0 => {
    //   block [0x825EE4E0..0x825EE4F8)
	// 825EE4E0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE4E4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EE4E8: 394AE1B0  addi r10, r10, -0x1e50
	ctx.r[10].s64 = ctx.r[10].s64 + -7760;
	// 825EE4EC: 816B7DC8  lwz r11, 0x7dc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32200 as u32) ) } as u64;
	// 825EE4F0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825EE4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE4F8 size=116
    let mut pc: u32 = 0x825EE4F8;
    'dispatch: loop {
        match pc {
            0x825EE4F8 => {
    //   block [0x825EE4F8..0x825EE56C)
	// 825EE4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE504: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE50C: 390BE1B0  addi r8, r11, -0x1e50
	ctx.r[8].s64 = ctx.r[11].s64 + -7760;
	// 825EE510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE514: 392A1280  addi r9, r10, 0x1280
	ctx.r[9].s64 = ctx.r[10].s64 + 4736;
	// 825EE518: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE51C: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 825EE520: 38AAACD4  addi r5, r10, -0x532c
	ctx.r[5].s64 = ctx.r[10].s64 + -21292;
	// 825EE524: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE52C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE53C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EE540: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 825EE544: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EE548: 386BAD04  addi r3, r11, -0x52fc
	ctx.r[3].s64 = ctx.r[11].s64 + -21244;
	// 825EE54C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825EE550: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE558: 4BE788C9  bl 0x82466e20
	ctx.lr = 0x825EE55C;
	sub_82466E20(ctx, base);
	// 825EE55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE570 size=112
    let mut pc: u32 = 0x825EE570;
    'dispatch: loop {
        match pc {
            0x825EE570 => {
    //   block [0x825EE570..0x825EE5E0)
	// 825EE570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE57C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE580: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE584: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EE588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE58C: 390B7DD0  addi r8, r11, 0x7dd0
	ctx.r[8].s64 = ctx.r[11].s64 + 32208;
	// 825EE590: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 825EE594: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 825EE598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE59C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE5A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE5A8: 386AAD34  addi r3, r10, -0x52cc
	ctx.r[3].s64 = ctx.r[10].s64 + -21196;
	// 825EE5AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE5B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE5B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE5B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE5C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE5C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE5C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE5CC: 4BE78855  bl 0x82466e20
	ctx.lr = 0x825EE5D0;
	sub_82466E20(ctx, base);
	// 825EE5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE5E0 size=108
    let mut pc: u32 = 0x825EE5E0;
    'dispatch: loop {
        match pc {
            0x825EE5E0 => {
    //   block [0x825EE5E0..0x825EE64C)
	// 825EE5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE5EC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE5F4: 38EB7EA8  addi r7, r11, 0x7ea8
	ctx.r[7].s64 = ctx.r[11].s64 + 32424;
	// 825EE5F8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EE5FC: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 825EE600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE604: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE60C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE610: 386AAD64  addi r3, r10, -0x529c
	ctx.r[3].s64 = ctx.r[10].s64 + -21148;
	// 825EE614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE638: 4BE787E9  bl 0x82466e20
	ctx.lr = 0x825EE63C;
	sub_82466E20(ctx, base);
	// 825EE63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE650 size=108
    let mut pc: u32 = 0x825EE650;
    'dispatch: loop {
        match pc {
            0x825EE650 => {
    //   block [0x825EE650..0x825EE6BC)
	// 825EE650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE65C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE664: 38EB7F20  addi r7, r11, 0x7f20
	ctx.r[7].s64 = ctx.r[11].s64 + 32544;
	// 825EE668: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 825EE66C: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 825EE670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE674: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE680: 386AAD94  addi r3, r10, -0x526c
	ctx.r[3].s64 = ctx.r[10].s64 + -21100;
	// 825EE684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE68C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE6A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE6A8: 4BE78779  bl 0x82466e20
	ctx.lr = 0x825EE6AC;
	sub_82466E20(ctx, base);
	// 825EE6AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE6B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE6B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE6C0 size=112
    let mut pc: u32 = 0x825EE6C0;
    'dispatch: loop {
        match pc {
            0x825EE6C0 => {
    //   block [0x825EE6C0..0x825EE730)
	// 825EE6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE6CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE6D0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE6D4: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EE6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE6DC: 390B7F68  addi r8, r11, 0x7f68
	ctx.r[8].s64 = ctx.r[11].s64 + 32616;
	// 825EE6E0: 39200013  li r9, 0x13
	ctx.r[9].s64 = 19;
	// 825EE6E4: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 825EE6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE6EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE6F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE6F8: 386AADC4  addi r3, r10, -0x523c
	ctx.r[3].s64 = ctx.r[10].s64 + -21052;
	// 825EE6FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE71C: 4BE78705  bl 0x82466e20
	ctx.lr = 0x825EE720;
	sub_82466E20(ctx, base);
	// 825EE720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE730 size=108
    let mut pc: u32 = 0x825EE730;
    'dispatch: loop {
        match pc {
            0x825EE730 => {
    //   block [0x825EE730..0x825EE79C)
	// 825EE730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE73C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE744: 38EB8130  addi r7, r11, -0x7ed0
	ctx.r[7].s64 = ctx.r[11].s64 + -32464;
	// 825EE748: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EE74C: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 825EE750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE754: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE760: 386AADF4  addi r3, r10, -0x520c
	ctx.r[3].s64 = ctx.r[10].s64 + -21004;
	// 825EE764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE76C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE788: 4BE78699  bl 0x82466e20
	ctx.lr = 0x825EE78C;
	sub_82466E20(ctx, base);
	// 825EE78C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EE7A0 size=24
    let mut pc: u32 = 0x825EE7A0;
    'dispatch: loop {
        match pc {
            0x825EE7A0 => {
    //   block [0x825EE7A0..0x825EE7B8)
	// 825EE7A0: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 825EE7A4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EE7A8: 394AE2D0  addi r10, r10, -0x1d30
	ctx.r[10].s64 = ctx.r[10].s64 + -7472;
	// 825EE7AC: 816B7DCC  lwz r11, 0x7dcc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32204 as u32) ) } as u64;
	// 825EE7B0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EE7B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE7B8 size=112
    let mut pc: u32 = 0x825EE7B8;
    'dispatch: loop {
        match pc {
            0x825EE7B8 => {
    //   block [0x825EE7B8..0x825EE828)
	// 825EE7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE7C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE7C8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE7CC: 392A12D8  addi r9, r10, 0x12d8
	ctx.r[9].s64 = ctx.r[10].s64 + 4824;
	// 825EE7D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE7D4: 390BE2D0  addi r8, r11, -0x1d30
	ctx.r[8].s64 = ctx.r[11].s64 + -7472;
	// 825EE7D8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825EE7DC: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 825EE7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE7E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE7E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE7F0: 386AAE24  addi r3, r10, -0x51dc
	ctx.r[3].s64 = ctx.r[10].s64 + -20956;
	// 825EE7F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EE7F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EE7FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE80C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE814: 4BE7860D  bl 0x82466e20
	ctx.lr = 0x825EE818;
	sub_82466E20(ctx, base);
	// 825EE818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE828 size=108
    let mut pc: u32 = 0x825EE828;
    'dispatch: loop {
        match pc {
            0x825EE828 => {
    //   block [0x825EE828..0x825EE894)
	// 825EE828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE834: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE83C: 38EB8148  addi r7, r11, -0x7eb8
	ctx.r[7].s64 = ctx.r[11].s64 + -32440;
	// 825EE840: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EE844: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 825EE848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE84C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE858: 386AAE54  addi r3, r10, -0x51ac
	ctx.r[3].s64 = ctx.r[10].s64 + -20908;
	// 825EE85C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE87C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE880: 4BE785A1  bl 0x82466e20
	ctx.lr = 0x825EE884;
	sub_82466E20(ctx, base);
	// 825EE884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE88C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE898 size=112
    let mut pc: u32 = 0x825EE898;
    'dispatch: loop {
        match pc {
            0x825EE898 => {
    //   block [0x825EE898..0x825EE908)
	// 825EE898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE89C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE8A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE8A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE8A8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE8AC: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EE8B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE8B4: 390B81A8  addi r8, r11, -0x7e58
	ctx.r[8].s64 = ctx.r[11].s64 + -32344;
	// 825EE8B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EE8BC: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 825EE8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE8C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE8C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EE8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE8D0: 386AAE84  addi r3, r10, -0x517c
	ctx.r[3].s64 = ctx.r[10].s64 + -20860;
	// 825EE8D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EE8D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE8E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE8E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE8F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE8F4: 4BE7852D  bl 0x82466e20
	ctx.lr = 0x825EE8F8;
	sub_82466E20(ctx, base);
	// 825EE8F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE908 size=108
    let mut pc: u32 = 0x825EE908;
    'dispatch: loop {
        match pc {
            0x825EE908 => {
    //   block [0x825EE908..0x825EE974)
	// 825EE908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE914: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE91C: 38EB81C8  addi r7, r11, -0x7e38
	ctx.r[7].s64 = ctx.r[11].s64 + -32312;
	// 825EE920: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 825EE924: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 825EE928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE92C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE938: 386AAEB4  addi r3, r10, -0x514c
	ctx.r[3].s64 = ctx.r[10].s64 + -20812;
	// 825EE93C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE95C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE960: 4BE784C1  bl 0x82466e20
	ctx.lr = 0x825EE964;
	sub_82466E20(ctx, base);
	// 825EE964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE96C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE978 size=108
    let mut pc: u32 = 0x825EE978;
    'dispatch: loop {
        match pc {
            0x825EE978 => {
    //   block [0x825EE978..0x825EE9E4)
	// 825EE978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE97C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE984: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE98C: 38EB8228  addi r7, r11, -0x7dd8
	ctx.r[7].s64 = ctx.r[11].s64 + -32216;
	// 825EE990: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EE994: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 825EE998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EE99C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EE9A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EE9A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EE9A8: 386AAEE4  addi r3, r10, -0x511c
	ctx.r[3].s64 = ctx.r[10].s64 + -20764;
	// 825EE9AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EE9B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EE9B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EE9B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EE9BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EE9C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EE9C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EE9C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EE9CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EE9D0: 4BE78451  bl 0x82466e20
	ctx.lr = 0x825EE9D4;
	sub_82466E20(ctx, base);
	// 825EE9D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EE9D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EE9DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EE9E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EE9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EE9E8 size=116
    let mut pc: u32 = 0x825EE9E8;
    'dispatch: loop {
        match pc {
            0x825EE9E8 => {
    //   block [0x825EE9E8..0x825EEA5C)
	// 825EE9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EE9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EE9F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EE9F4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EE9F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EE9FC: 390B8258  addi r8, r11, -0x7da8
	ctx.r[8].s64 = ctx.r[11].s64 + -32168;
	// 825EEA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEA04: 392A1304  addi r9, r10, 0x1304
	ctx.r[9].s64 = ctx.r[10].s64 + 4868;
	// 825EEA08: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEA0C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 825EEA10: 38AAAF74  addi r5, r10, -0x508c
	ctx.r[5].s64 = ctx.r[10].s64 + -20620;
	// 825EEA14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EEA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEA1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EEA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEA24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEA2C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EEA30: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 825EEA34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EEA38: 386BAF14  addi r3, r11, -0x50ec
	ctx.r[3].s64 = ctx.r[11].s64 + -20716;
	// 825EEA3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EEA40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEA48: 4BE783D9  bl 0x82466e20
	ctx.lr = 0x825EEA4C;
	sub_82466E20(ctx, base);
	// 825EEA4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEA58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEA60 size=96
    let mut pc: u32 = 0x825EEA60;
    'dispatch: loop {
        match pc {
            0x825EEA60 => {
    //   block [0x825EEA60..0x825EEAC0)
	// 825EEA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEA6C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEA74: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 825EEA78: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEA80: 386AAF44  addi r3, r10, -0x50bc
	ctx.r[3].s64 = ctx.r[10].s64 + -20668;
	// 825EEA84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEA8C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EEA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEAA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825EEAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEAA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825EEAAC: 4BE78375  bl 0x82466e20
	ctx.lr = 0x825EEAB0;
	sub_82466E20(ctx, base);
	// 825EEAB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEAB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEAB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEABC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEAC0 size=112
    let mut pc: u32 = 0x825EEAC0;
    'dispatch: loop {
        match pc {
            0x825EEAC0 => {
    //   block [0x825EEAC0..0x825EEB30)
	// 825EEAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEAD0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEAD4: 38AAAF44  addi r5, r10, -0x50bc
	ctx.r[5].s64 = ctx.r[10].s64 + -20668;
	// 825EEAD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEADC: 390B8270  addi r8, r11, -0x7d90
	ctx.r[8].s64 = ctx.r[11].s64 + -32144;
	// 825EEAE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 825EEAE4: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 825EEAE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEAEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEAF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EEAF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEAF8: 386AAF74  addi r3, r10, -0x508c
	ctx.r[3].s64 = ctx.r[10].s64 + -20620;
	// 825EEAFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EEB00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEB08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEB0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEB10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEB14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEB18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEB1C: 4BE78305  bl 0x82466e20
	ctx.lr = 0x825EEB20;
	sub_82466E20(ctx, base);
	// 825EEB20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEB30 size=112
    let mut pc: u32 = 0x825EEB30;
    'dispatch: loop {
        match pc {
            0x825EEB30 => {
    //   block [0x825EEB30..0x825EEBA0)
	// 825EEB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEB34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEB3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EEB40: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEB44: 392A1320  addi r9, r10, 0x1320
	ctx.r[9].s64 = ctx.r[10].s64 + 4896;
	// 825EEB48: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEB4C: 390B82A0  addi r8, r11, -0x7d60
	ctx.r[8].s64 = ctx.r[11].s64 + -32096;
	// 825EEB50: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 825EEB54: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 825EEB58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEB5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEB60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EEB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEB68: 386AAFA4  addi r3, r10, -0x505c
	ctx.r[3].s64 = ctx.r[10].s64 + -20572;
	// 825EEB6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EEB70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EEB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEB78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEB7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEB80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEB84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEB88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEB8C: 4BE78295  bl 0x82466e20
	ctx.lr = 0x825EEB90;
	sub_82466E20(ctx, base);
	// 825EEB90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEB94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEB98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEB9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEBA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEBA0 size=108
    let mut pc: u32 = 0x825EEBA0;
    'dispatch: loop {
        match pc {
            0x825EEBA0 => {
    //   block [0x825EEBA0..0x825EEC0C)
	// 825EEBA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEBA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEBA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEBAC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEBB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEBB4: 38EB8348  addi r7, r11, -0x7cb8
	ctx.r[7].s64 = ctx.r[11].s64 + -31928;
	// 825EEBB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EEBBC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 825EEBC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEBC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEBC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEBD0: 386AAFD4  addi r3, r10, -0x502c
	ctx.r[3].s64 = ctx.r[10].s64 + -20524;
	// 825EEBD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EEBD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEBDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEBE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEBE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEBE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEBEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEBF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEBF8: 4BE78229  bl 0x82466e20
	ctx.lr = 0x825EEBFC;
	sub_82466E20(ctx, base);
	// 825EEBFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEC10 size=108
    let mut pc: u32 = 0x825EEC10;
    'dispatch: loop {
        match pc {
            0x825EEC10 => {
    //   block [0x825EEC10..0x825EEC7C)
	// 825EEC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEC1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEC20: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEC24: 38EB8378  addi r7, r11, -0x7c88
	ctx.r[7].s64 = ctx.r[11].s64 + -31880;
	// 825EEC28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EEC2C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 825EEC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEC34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEC38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEC40: 386AB004  addi r3, r10, -0x4ffc
	ctx.r[3].s64 = ctx.r[10].s64 + -20476;
	// 825EEC44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EEC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEC64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEC68: 4BE781B9  bl 0x82466e20
	ctx.lr = 0x825EEC6C;
	sub_82466E20(ctx, base);
	// 825EEC6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEC70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEC74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEC78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EEC80 size=28
    let mut pc: u32 = 0x825EEC80;
    'dispatch: loop {
        match pc {
            0x825EEC80 => {
    //   block [0x825EEC80..0x825EEC9C)
	// 825EEC80: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEC84: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EEC88: 394AE300  addi r10, r10, -0x1d00
	ctx.r[10].s64 = ctx.r[10].s64 + -7424;
	// 825EEC8C: 816B83A8  lwz r11, -0x7c58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31832 as u32) ) } as u64;
	// 825EEC90: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825EEC94: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825EEC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EECA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EECA0 size=112
    let mut pc: u32 = 0x825EECA0;
    'dispatch: loop {
        match pc {
            0x825EECA0 => {
    //   block [0x825EECA0..0x825EED10)
	// 825EECA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EECA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EECA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EECAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EECB0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EECB4: 392A14B0  addi r9, r10, 0x14b0
	ctx.r[9].s64 = ctx.r[10].s64 + 5296;
	// 825EECB8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EECBC: 390BE300  addi r8, r11, -0x1d00
	ctx.r[8].s64 = ctx.r[11].s64 + -7424;
	// 825EECC0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825EECC4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 825EECC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EECCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EECD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EECD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EECD8: 386AB034  addi r3, r10, -0x4fcc
	ctx.r[3].s64 = ctx.r[10].s64 + -20428;
	// 825EECDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EECE0: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 825EECE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EECE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EECEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EECF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EECF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EECF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EECFC: 4BE78125  bl 0x82466e20
	ctx.lr = 0x825EED00;
	sub_82466E20(ctx, base);
	// 825EED00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EED04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EED08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EED0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EED10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EED10 size=108
    let mut pc: u32 = 0x825EED10;
    'dispatch: loop {
        match pc {
            0x825EED10 => {
    //   block [0x825EED10..0x825EED7C)
	// 825EED10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EED14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EED18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EED1C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EED20: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EED24: 38EB83B4  addi r7, r11, -0x7c4c
	ctx.r[7].s64 = ctx.r[11].s64 + -31820;
	// 825EED28: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EED2C: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 825EED30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EED34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EED38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EED3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EED40: 386AB064  addi r3, r10, -0x4f9c
	ctx.r[3].s64 = ctx.r[10].s64 + -20380;
	// 825EED44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EED48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EED4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EED50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EED54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EED58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EED5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EED60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EED64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EED68: 4BE780B9  bl 0x82466e20
	ctx.lr = 0x825EED6C;
	sub_82466E20(ctx, base);
	// 825EED6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EED70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EED74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EED78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EED80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EED80 size=108
    let mut pc: u32 = 0x825EED80;
    'dispatch: loop {
        match pc {
            0x825EED80 => {
    //   block [0x825EED80..0x825EEDEC)
	// 825EED80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EED84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EED88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EED8C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EED90: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EED94: 38EB83E4  addi r7, r11, -0x7c1c
	ctx.r[7].s64 = ctx.r[11].s64 + -31772;
	// 825EED98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EED9C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 825EEDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEDA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEDA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEDAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEDB0: 386AB094  addi r3, r10, -0x4f6c
	ctx.r[3].s64 = ctx.r[10].s64 + -20332;
	// 825EEDB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EEDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEDC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEDD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEDD8: 4BE78049  bl 0x82466e20
	ctx.lr = 0x825EEDDC;
	sub_82466E20(ctx, base);
	// 825EEDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEDE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEDF0 size=108
    let mut pc: u32 = 0x825EEDF0;
    'dispatch: loop {
        match pc {
            0x825EEDF0 => {
    //   block [0x825EEDF0..0x825EEE5C)
	// 825EEDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEDFC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEE00: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEE04: 38EB8414  addi r7, r11, -0x7bec
	ctx.r[7].s64 = ctx.r[11].s64 + -31724;
	// 825EEE08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EEE0C: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 825EEE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEE14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEE18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEE1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEE20: 386AB0C4  addi r3, r10, -0x4f3c
	ctx.r[3].s64 = ctx.r[10].s64 + -20284;
	// 825EEE24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EEE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEE34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEE44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEE48: 4BE77FD9  bl 0x82466e20
	ctx.lr = 0x825EEE4C;
	sub_82466E20(ctx, base);
	// 825EEE4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EEE60 size=24
    let mut pc: u32 = 0x825EEE60;
    'dispatch: loop {
        match pc {
            0x825EEE60 => {
    //   block [0x825EEE60..0x825EEE78)
	// 825EEE60: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEE64: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EEE68: 394AE3C0  addi r10, r10, -0x1c40
	ctx.r[10].s64 = ctx.r[10].s64 + -7232;
	// 825EEE6C: 816B842C  lwz r11, -0x7bd4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31700 as u32) ) } as u64;
	// 825EEE70: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825EEE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEE78 size=112
    let mut pc: u32 = 0x825EEE78;
    'dispatch: loop {
        match pc {
            0x825EEE78 => {
    //   block [0x825EEE78..0x825EEEE8)
	// 825EEE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEE84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EEE88: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEE8C: 392A1504  addi r9, r10, 0x1504
	ctx.r[9].s64 = ctx.r[10].s64 + 5380;
	// 825EEE90: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEE94: 390BE3C0  addi r8, r11, -0x1c40
	ctx.r[8].s64 = ctx.r[11].s64 + -7232;
	// 825EEE98: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 825EEE9C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 825EEEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEEA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEEA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EEEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEEB0: 386AB0F4  addi r3, r10, -0x4f0c
	ctx.r[3].s64 = ctx.r[10].s64 + -20236;
	// 825EEEB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EEEB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EEEBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEEC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEED4: 4BE77F4D  bl 0x82466e20
	ctx.lr = 0x825EEED8;
	sub_82466E20(ctx, base);
	// 825EEED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEEE8 size=112
    let mut pc: u32 = 0x825EEEE8;
    'dispatch: loop {
        match pc {
            0x825EEEE8 => {
    //   block [0x825EEEE8..0x825EEF58)
	// 825EEEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEEF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EEEF8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEEFC: 392A1540  addi r9, r10, 0x1540
	ctx.r[9].s64 = ctx.r[10].s64 + 5440;
	// 825EEF00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EEF04: 390B8438  addi r8, r11, -0x7bc8
	ctx.r[8].s64 = ctx.r[11].s64 + -31688;
	// 825EEF08: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 825EEF0C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 825EEF10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEF14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEF18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EEF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEF20: 386AB124  addi r3, r10, -0x4edc
	ctx.r[3].s64 = ctx.r[10].s64 + -20188;
	// 825EEF24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EEF28: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 825EEF2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEF30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEF34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEF38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEF3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEF40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEF44: 4BE77EDD  bl 0x82466e20
	ctx.lr = 0x825EEF48;
	sub_82466E20(ctx, base);
	// 825EEF48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEF54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEF58 size=108
    let mut pc: u32 = 0x825EEF58;
    'dispatch: loop {
        match pc {
            0x825EEF58 => {
    //   block [0x825EEF58..0x825EEFC4)
	// 825EEF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEF64: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEF68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEF6C: 38EB8480  addi r7, r11, -0x7b80
	ctx.r[7].s64 = ctx.r[11].s64 + -31616;
	// 825EEF70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EEF74: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 825EEF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEF7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEF80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEF88: 386AB154  addi r3, r10, -0x4eac
	ctx.r[3].s64 = ctx.r[10].s64 + -20140;
	// 825EEF8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EEF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EEF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EEF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EEF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EEFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EEFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EEFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EEFAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EEFB0: 4BE77E71  bl 0x82466e20
	ctx.lr = 0x825EEFB4;
	sub_82466E20(ctx, base);
	// 825EEFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EEFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EEFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EEFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EEFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EEFC8 size=108
    let mut pc: u32 = 0x825EEFC8;
    'dispatch: loop {
        match pc {
            0x825EEFC8 => {
    //   block [0x825EEFC8..0x825EF034)
	// 825EEFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EEFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EEFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EEFD4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EEFD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EEFDC: 38EB84B0  addi r7, r11, -0x7b50
	ctx.r[7].s64 = ctx.r[11].s64 + -31568;
	// 825EEFE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EEFE4: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 825EEFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EEFEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EEFF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EEFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EEFF8: 386AB184  addi r3, r10, -0x4e7c
	ctx.r[3].s64 = ctx.r[10].s64 + -20092;
	// 825EEFFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF01C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF020: 4BE77E01  bl 0x82466e20
	ctx.lr = 0x825EF024;
	sub_82466E20(ctx, base);
	// 825EF024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF038 size=112
    let mut pc: u32 = 0x825EF038;
    'dispatch: loop {
        match pc {
            0x825EF038 => {
    //   block [0x825EF038..0x825EF0A8)
	// 825EF038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF044: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF048: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF04C: 392A1578  addi r9, r10, 0x1578
	ctx.r[9].s64 = ctx.r[10].s64 + 5496;
	// 825EF050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF054: 390B84E8  addi r8, r11, -0x7b18
	ctx.r[8].s64 = ctx.r[11].s64 + -31512;
	// 825EF058: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 825EF05C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 825EF060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF064: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EF06C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF070: 386AB1B4  addi r3, r10, -0x4e4c
	ctx.r[3].s64 = ctx.r[10].s64 + -20044;
	// 825EF074: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EF078: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EF07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF094: 4BE77D8D  bl 0x82466e20
	ctx.lr = 0x825EF098;
	sub_82466E20(ctx, base);
	// 825EF098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF0A8 size=108
    let mut pc: u32 = 0x825EF0A8;
    'dispatch: loop {
        match pc {
            0x825EF0A8 => {
    //   block [0x825EF0A8..0x825EF114)
	// 825EF0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF0B4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF0B8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EF0BC: 38EB8548  addi r7, r11, -0x7ab8
	ctx.r[7].s64 = ctx.r[11].s64 + -31416;
	// 825EF0C0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 825EF0C4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 825EF0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF0CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF0D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF0D8: 386AB1E4  addi r3, r10, -0x4e1c
	ctx.r[3].s64 = ctx.r[10].s64 + -19996;
	// 825EF0DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF0FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF100: 4BE77D21  bl 0x82466e20
	ctx.lr = 0x825EF104;
	sub_82466E20(ctx, base);
	// 825EF104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF118 size=108
    let mut pc: u32 = 0x825EF118;
    'dispatch: loop {
        match pc {
            0x825EF118 => {
    //   block [0x825EF118..0x825EF184)
	// 825EF118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF124: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF128: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825EF12C: 38EB8650  addi r7, r11, -0x79b0
	ctx.r[7].s64 = ctx.r[11].s64 + -31152;
	// 825EF130: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EF134: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 825EF138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF13C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF148: 386AB214  addi r3, r10, -0x4dec
	ctx.r[3].s64 = ctx.r[10].s64 + -19948;
	// 825EF14C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF16C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF170: 4BE77CB1  bl 0x82466e20
	ctx.lr = 0x825EF174;
	sub_82466E20(ctx, base);
	// 825EF174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF188 size=108
    let mut pc: u32 = 0x825EF188;
    'dispatch: loop {
        match pc {
            0x825EF188 => {
    //   block [0x825EF188..0x825EF1F4)
	// 825EF188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF194: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF19C: 38EB8668  addi r7, r11, -0x7998
	ctx.r[7].s64 = ctx.r[11].s64 + -31128;
	// 825EF1A0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 825EF1A4: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 825EF1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF1AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF1B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF1B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF1B8: 386AB244  addi r3, r10, -0x4dbc
	ctx.r[3].s64 = ctx.r[10].s64 + -19900;
	// 825EF1BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF1C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF1C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF1D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF1D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF1D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF1DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF1E0: 4BE77C41  bl 0x82466e20
	ctx.lr = 0x825EF1E4;
	sub_82466E20(ctx, base);
	// 825EF1E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF1E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF1EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EF1F8 size=24
    let mut pc: u32 = 0x825EF1F8;
    'dispatch: loop {
        match pc {
            0x825EF1F8 => {
    //   block [0x825EF1F8..0x825EF210)
	// 825EF1F8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF1FC: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EF200: 394AE498  addi r10, r10, -0x1b68
	ctx.r[10].s64 = ctx.r[10].s64 + -7016;
	// 825EF204: 816B84E4  lwz r11, -0x7b1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31516 as u32) ) } as u64;
	// 825EF208: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EF20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF210 size=108
    let mut pc: u32 = 0x825EF210;
    'dispatch: loop {
        match pc {
            0x825EF210 => {
    //   block [0x825EF210..0x825EF27C)
	// 825EF210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF21C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF220: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF224: 38EBE498  addi r7, r11, -0x1b68
	ctx.r[7].s64 = ctx.r[11].s64 + -7016;
	// 825EF228: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF22C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 825EF230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF23C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF240: 386AB274  addi r3, r10, -0x4d8c
	ctx.r[3].s64 = ctx.r[10].s64 + -19852;
	// 825EF244: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF268: 4BE77BB9  bl 0x82466e20
	ctx.lr = 0x825EF26C;
	sub_82466E20(ctx, base);
	// 825EF26C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EF280 size=24
    let mut pc: u32 = 0x825EF280;
    'dispatch: loop {
        match pc {
            0x825EF280 => {
    //   block [0x825EF280..0x825EF298)
	// 825EF280: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF284: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EF288: 394AE4C8  addi r10, r10, -0x1b38
	ctx.r[10].s64 = ctx.r[10].s64 + -6968;
	// 825EF28C: 816B84E4  lwz r11, -0x7b1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31516 as u32) ) } as u64;
	// 825EF290: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EF294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF298 size=108
    let mut pc: u32 = 0x825EF298;
    'dispatch: loop {
        match pc {
            0x825EF298 => {
    //   block [0x825EF298..0x825EF304)
	// 825EF298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF2A4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF2AC: 38EBE4C8  addi r7, r11, -0x1b38
	ctx.r[7].s64 = ctx.r[11].s64 + -6968;
	// 825EF2B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF2B4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 825EF2B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF2BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF2C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF2C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF2C8: 386AB2A4  addi r3, r10, -0x4d5c
	ctx.r[3].s64 = ctx.r[10].s64 + -19804;
	// 825EF2CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF2D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF2D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF2DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF2E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF2E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF2E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF2EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF2F0: 4BE77B31  bl 0x82466e20
	ctx.lr = 0x825EF2F4;
	sub_82466E20(ctx, base);
	// 825EF2F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF308 size=108
    let mut pc: u32 = 0x825EF308;
    'dispatch: loop {
        match pc {
            0x825EF308 => {
    //   block [0x825EF308..0x825EF374)
	// 825EF308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF314: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF31C: 38EB86E0  addi r7, r11, -0x7920
	ctx.r[7].s64 = ctx.r[11].s64 + -31008;
	// 825EF320: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EF324: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 825EF328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF32C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF338: 386AB2D4  addi r3, r10, -0x4d2c
	ctx.r[3].s64 = ctx.r[10].s64 + -19756;
	// 825EF33C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF34C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF35C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF360: 4BE77AC1  bl 0x82466e20
	ctx.lr = 0x825EF364;
	sub_82466E20(ctx, base);
	// 825EF364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EF378 size=24
    let mut pc: u32 = 0x825EF378;
    'dispatch: loop {
        match pc {
            0x825EF378 => {
    //   block [0x825EF378..0x825EF390)
	// 825EF378: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF37C: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EF380: 394AE4F8  addi r10, r10, -0x1b08
	ctx.r[10].s64 = ctx.r[10].s64 + -6920;
	// 825EF384: 816B84E4  lwz r11, -0x7b1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-31516 as u32) ) } as u64;
	// 825EF388: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EF38C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF390 size=108
    let mut pc: u32 = 0x825EF390;
    'dispatch: loop {
        match pc {
            0x825EF390 => {
    //   block [0x825EF390..0x825EF3FC)
	// 825EF390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF39C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF3A4: 38EBE4F8  addi r7, r11, -0x1b08
	ctx.r[7].s64 = ctx.r[11].s64 + -6920;
	// 825EF3A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF3AC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 825EF3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF3B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF3B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF3C0: 386AB304  addi r3, r10, -0x4cfc
	ctx.r[3].s64 = ctx.r[10].s64 + -19708;
	// 825EF3C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF3C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF3E8: 4BE77A39  bl 0x82466e20
	ctx.lr = 0x825EF3EC;
	sub_82466E20(ctx, base);
	// 825EF3EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF3F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF400 size=112
    let mut pc: u32 = 0x825EF400;
    'dispatch: loop {
        match pc {
            0x825EF400 => {
    //   block [0x825EF400..0x825EF470)
	// 825EF400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF40C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF410: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF414: 392A15BC  addi r9, r10, 0x15bc
	ctx.r[9].s64 = ctx.r[10].s64 + 5564;
	// 825EF418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF41C: 390B86F8  addi r8, r11, -0x7908
	ctx.r[8].s64 = ctx.r[11].s64 + -30984;
	// 825EF420: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 825EF424: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 825EF428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF42C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EF434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF438: 386AB334  addi r3, r10, -0x4ccc
	ctx.r[3].s64 = ctx.r[10].s64 + -19660;
	// 825EF43C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EF440: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EF444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF45C: 4BE779C5  bl 0x82466e20
	ctx.lr = 0x825EF460;
	sub_82466E20(ctx, base);
	// 825EF460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF470 size=108
    let mut pc: u32 = 0x825EF470;
    'dispatch: loop {
        match pc {
            0x825EF470 => {
    //   block [0x825EF470..0x825EF4DC)
	// 825EF470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF47C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF484: 38EB8728  addi r7, r11, -0x78d8
	ctx.r[7].s64 = ctx.r[11].s64 + -30936;
	// 825EF488: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF48C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 825EF490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF494: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF4A0: 386AB364  addi r3, r10, -0x4c9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19612;
	// 825EF4A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF4A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF4AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF4B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF4B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF4C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF4C8: 4BE77959  bl 0x82466e20
	ctx.lr = 0x825EF4CC;
	sub_82466E20(ctx, base);
	// 825EF4CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF4D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF4D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF4D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF4E0 size=108
    let mut pc: u32 = 0x825EF4E0;
    'dispatch: loop {
        match pc {
            0x825EF4E0 => {
    //   block [0x825EF4E0..0x825EF54C)
	// 825EF4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF4EC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF4F4: 38EB8758  addi r7, r11, -0x78a8
	ctx.r[7].s64 = ctx.r[11].s64 + -30888;
	// 825EF4F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EF4FC: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 825EF500: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF504: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF508: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF510: 386AB394  addi r3, r10, -0x4c6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19564;
	// 825EF514: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF518: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF520: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF528: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF530: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF534: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF538: 4BE778E9  bl 0x82466e20
	ctx.lr = 0x825EF53C;
	sub_82466E20(ctx, base);
	// 825EF53C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF550 size=108
    let mut pc: u32 = 0x825EF550;
    'dispatch: loop {
        match pc {
            0x825EF550 => {
    //   block [0x825EF550..0x825EF5BC)
	// 825EF550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF55C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF564: 38EB8770  addi r7, r11, -0x7890
	ctx.r[7].s64 = ctx.r[11].s64 + -30864;
	// 825EF568: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF56C: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 825EF570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF574: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF578: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF57C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF580: 386AB3C4  addi r3, r10, -0x4c3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19516;
	// 825EF584: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF594: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF5A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF5A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF5A8: 4BE77879  bl 0x82466e20
	ctx.lr = 0x825EF5AC;
	sub_82466E20(ctx, base);
	// 825EF5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF5C0 size=112
    let mut pc: u32 = 0x825EF5C0;
    'dispatch: loop {
        match pc {
            0x825EF5C0 => {
    //   block [0x825EF5C0..0x825EF630)
	// 825EF5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF5CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF5D0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF5D4: 38AAB424  addi r5, r10, -0x4bdc
	ctx.r[5].s64 = ctx.r[10].s64 + -19420;
	// 825EF5D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF5DC: 390B87A0  addi r8, r11, -0x7860
	ctx.r[8].s64 = ctx.r[11].s64 + -30816;
	// 825EF5E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825EF5E4: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 825EF5E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF5EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF5F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EF5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF5F8: 386AB3F4  addi r3, r10, -0x4c0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19468;
	// 825EF5FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EF600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF60C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF61C: 4BE77805  bl 0x82466e20
	ctx.lr = 0x825EF620;
	sub_82466E20(ctx, base);
	// 825EF620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF630 size=108
    let mut pc: u32 = 0x825EF630;
    'dispatch: loop {
        match pc {
            0x825EF630 => {
    //   block [0x825EF630..0x825EF69C)
	// 825EF630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF63C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF644: 38EB87B8  addi r7, r11, -0x7848
	ctx.r[7].s64 = ctx.r[11].s64 + -30792;
	// 825EF648: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF64C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 825EF650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF654: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF660: 386AB424  addi r3, r10, -0x4bdc
	ctx.r[3].s64 = ctx.r[10].s64 + -19420;
	// 825EF664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF688: 4BE77799  bl 0x82466e20
	ctx.lr = 0x825EF68C;
	sub_82466E20(ctx, base);
	// 825EF68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF6A0 size=108
    let mut pc: u32 = 0x825EF6A0;
    'dispatch: loop {
        match pc {
            0x825EF6A0 => {
    //   block [0x825EF6A0..0x825EF70C)
	// 825EF6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF6AC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF6B4: 38EB87E8  addi r7, r11, -0x7818
	ctx.r[7].s64 = ctx.r[11].s64 + -30744;
	// 825EF6B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EF6BC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 825EF6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF6C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF6C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF6D0: 386AB454  addi r3, r10, -0x4bac
	ctx.r[3].s64 = ctx.r[10].s64 + -19372;
	// 825EF6D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF6F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF6F8: 4BE77729  bl 0x82466e20
	ctx.lr = 0x825EF6FC;
	sub_82466E20(ctx, base);
	// 825EF6FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF710 size=108
    let mut pc: u32 = 0x825EF710;
    'dispatch: loop {
        match pc {
            0x825EF710 => {
    //   block [0x825EF710..0x825EF77C)
	// 825EF710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF71C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF724: 38EB8800  addi r7, r11, -0x7800
	ctx.r[7].s64 = ctx.r[11].s64 + -30720;
	// 825EF728: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF72C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 825EF730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF738: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF740: 386AB484  addi r3, r10, -0x4b7c
	ctx.r[3].s64 = ctx.r[10].s64 + -19324;
	// 825EF744: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF768: 4BE776B9  bl 0x82466e20
	ctx.lr = 0x825EF76C;
	sub_82466E20(ctx, base);
	// 825EF76C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF780 size=108
    let mut pc: u32 = 0x825EF780;
    'dispatch: loop {
        match pc {
            0x825EF780 => {
    //   block [0x825EF780..0x825EF7EC)
	// 825EF780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF78C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF794: 38EB8830  addi r7, r11, -0x77d0
	ctx.r[7].s64 = ctx.r[11].s64 + -30672;
	// 825EF798: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 825EF79C: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 825EF7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF7A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF7A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF7B0: 386AB4B4  addi r3, r10, -0x4b4c
	ctx.r[3].s64 = ctx.r[10].s64 + -19276;
	// 825EF7B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF7BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF7D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF7D8: 4BE77649  bl 0x82466e20
	ctx.lr = 0x825EF7DC;
	sub_82466E20(ctx, base);
	// 825EF7DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF7F0 size=108
    let mut pc: u32 = 0x825EF7F0;
    'dispatch: loop {
        match pc {
            0x825EF7F0 => {
    //   block [0x825EF7F0..0x825EF85C)
	// 825EF7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF7FC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF804: 38EB88D8  addi r7, r11, -0x7728
	ctx.r[7].s64 = ctx.r[11].s64 + -30504;
	// 825EF808: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF80C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 825EF810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF814: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF818: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF820: 386AB4E4  addi r3, r10, -0x4b1c
	ctx.r[3].s64 = ctx.r[10].s64 + -19228;
	// 825EF824: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF848: 4BE775D9  bl 0x82466e20
	ctx.lr = 0x825EF84C;
	sub_82466E20(ctx, base);
	// 825EF84C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF860 size=108
    let mut pc: u32 = 0x825EF860;
    'dispatch: loop {
        match pc {
            0x825EF860 => {
    //   block [0x825EF860..0x825EF8CC)
	// 825EF860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF86C: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF874: 38EB8908  addi r7, r11, -0x76f8
	ctx.r[7].s64 = ctx.r[11].s64 + -30456;
	// 825EF878: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 825EF87C: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 825EF880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF884: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF888: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF890: 386AB514  addi r3, r10, -0x4aec
	ctx.r[3].s64 = ctx.r[10].s64 + -19180;
	// 825EF894: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF8AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF8B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF8B8: 4BE77569  bl 0x82466e20
	ctx.lr = 0x825EF8BC;
	sub_82466E20(ctx, base);
	// 825EF8BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF8D0 size=108
    let mut pc: u32 = 0x825EF8D0;
    'dispatch: loop {
        match pc {
            0x825EF8D0 => {
    //   block [0x825EF8D0..0x825EF93C)
	// 825EF8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF8DC: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF8E4: 38EB8920  addi r7, r11, -0x76e0
	ctx.r[7].s64 = ctx.r[11].s64 + -30432;
	// 825EF8E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EF8EC: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 825EF8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF8F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF8F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EF8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF900: 386AB544  addi r3, r10, -0x4abc
	ctx.r[3].s64 = ctx.r[10].s64 + -19132;
	// 825EF904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EF908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EF928: 4BE774F9  bl 0x82466e20
	ctx.lr = 0x825EF92C;
	sub_82466E20(ctx, base);
	// 825EF92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF940 size=112
    let mut pc: u32 = 0x825EF940;
    'dispatch: loop {
        match pc {
            0x825EF940 => {
    //   block [0x825EF940..0x825EF9B0)
	// 825EF940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF94C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF950: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF954: 38AAB394  addi r5, r10, -0x4c6c
	ctx.r[5].s64 = ctx.r[10].s64 + -19564;
	// 825EF958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF95C: 390B8950  addi r8, r11, -0x76b0
	ctx.r[8].s64 = ctx.r[11].s64 + -30384;
	// 825EF960: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 825EF964: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 825EF968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF96C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF970: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EF974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EF978: 386AB574  addi r3, r10, -0x4a8c
	ctx.r[3].s64 = ctx.r[10].s64 + -19084;
	// 825EF97C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 825EF980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EF984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EF988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EF98C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EF990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EF994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EF998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EF99C: 4BE77485  bl 0x82466e20
	ctx.lr = 0x825EF9A0;
	sub_82466E20(ctx, base);
	// 825EF9A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EF9A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EF9A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EF9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EF9B0 size=24
    let mut pc: u32 = 0x825EF9B0;
    'dispatch: loop {
        match pc {
            0x825EF9B0 => {
    //   block [0x825EF9B0..0x825EF9C8)
	// 825EF9B0: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF9B4: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EF9B8: 394AE528  addi r10, r10, -0x1ad8
	ctx.r[10].s64 = ctx.r[10].s64 + -6872;
	// 825EF9BC: 816B89F8  lwz r11, -0x7608(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30216 as u32) ) } as u64;
	// 825EF9C0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 825EF9C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EF9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EF9C8 size=112
    let mut pc: u32 = 0x825EF9C8;
    'dispatch: loop {
        match pc {
            0x825EF9C8 => {
    //   block [0x825EF9C8..0x825EFA38)
	// 825EF9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EF9CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EF9D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EF9D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF9D8: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EF9DC: 392A15E8  addi r9, r10, 0x15e8
	ctx.r[9].s64 = ctx.r[10].s64 + 5608;
	// 825EF9E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EF9E4: 390BE528  addi r8, r11, -0x1ad8
	ctx.r[8].s64 = ctx.r[11].s64 + -6872;
	// 825EF9E8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 825EF9EC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 825EF9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EF9F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EF9F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EF9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFA00: 386AB5A4  addi r3, r10, -0x4a5c
	ctx.r[3].s64 = ctx.r[10].s64 + -19036;
	// 825EFA04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EFA08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EFA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFA10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFA18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFA1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFA20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFA24: 4BE773FD  bl 0x82466e20
	ctx.lr = 0x825EFA28;
	sub_82466E20(ctx, base);
	// 825EFA28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFA2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFA30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFA34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFA38 size=108
    let mut pc: u32 = 0x825EFA38;
    'dispatch: loop {
        match pc {
            0x825EFA38 => {
    //   block [0x825EFA38..0x825EFAA4)
	// 825EFA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFA44: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFA4C: 38EB8A00  addi r7, r11, -0x7600
	ctx.r[7].s64 = ctx.r[11].s64 + -30208;
	// 825EFA50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 825EFA54: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 825EFA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFA5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFA60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 825EFA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 825EFA68: 386AB5D4  addi r3, r10, -0x4a2c
	ctx.r[3].s64 = ctx.r[10].s64 + -18988;
	// 825EFA6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 825EFA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825EFA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 825EFA90: 4BE77391  bl 0x82466e20
	ctx.lr = 0x825EFA94;
	sub_82466E20(ctx, base);
	// 825EFA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825EFAA8 size=116
    let mut pc: u32 = 0x825EFAA8;
    'dispatch: loop {
        match pc {
            0x825EFAA8 => {
    //   block [0x825EFAA8..0x825EFB1C)
	// 825EFAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825EFAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825EFAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825EFAB4: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFAB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFABC: 390B8A30  addi r8, r11, -0x75d0
	ctx.r[8].s64 = ctx.r[11].s64 + -30160;
	// 825EFAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825EFAC4: 392A162C  addi r9, r10, 0x162c
	ctx.r[9].s64 = ctx.r[10].s64 + 5676;
	// 825EFAC8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 825EFACC: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 825EFAD0: 38AAB394  addi r5, r10, -0x4c6c
	ctx.r[5].s64 = ctx.r[10].s64 + -19564;
	// 825EFAD4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 825EFAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825EFADC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 825EFAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825EFAE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825EFAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 825EFAEC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 825EFAF0: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 825EFAF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 825EFAF8: 386BB604  addi r3, r11, -0x49fc
	ctx.r[3].s64 = ctx.r[11].s64 + -18940;
	// 825EFAFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 825EFB00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 825EFB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 825EFB08: 4BE77319  bl 0x82466e20
	ctx.lr = 0x825EFB0C;
	sub_82466E20(ctx, base);
	// 825EFB0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825EFB10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825EFB14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825EFB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825EFB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825EFB20 size=24
    let mut pc: u32 = 0x825EFB20;
    'dispatch: loop {
        match pc {
            0x825EFB20 => {
    //   block [0x825EFB20..0x825EFB38)
	// 825EFB20: 3D608274  lis r11, -0x7d8c
	ctx.r[11].s64 = -2106327040;
	// 825EFB24: 3D408274  lis r10, -0x7d8c
	ctx.r[10].s64 = -2106327040;
	// 825EFB28: 394AE5A0  addi r10, r10, -0x1a60
	ctx.r[10].s64 = ctx.r[10].s64 + -6752;
	// 825EFB2C: 816B8AF0  lwz r11, -0x7510(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29968 as u32) ) } as u64;
	// 825EFB30: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 825EFB34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


